use std::cell::Cell;
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::num::NonZeroU32;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use rand_core::OsRng;
use rusqlite::{Connection, OpenFlags, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use zcash_client_backend::data_api::chain::{BlockSource, ChainState, scan_cached_blocks};
use zcash_client_backend::data_api::wallet::ConfirmationsPolicy;
use zcash_client_backend::data_api::{
    Account, AccountBirthday, AccountPurpose, WalletCommitmentTrees, WalletRead, WalletWrite,
};
use zcash_client_backend::proto::compact_formats::CompactBlock;
use zcash_client_backend::scanning::{Nullifiers, ScanningKeys, scan_block};
use zcash_client_sqlite::error::SqliteClientError;
use zcash_client_sqlite::util::SystemClock;
use zcash_client_sqlite::{AccountUuid, BlockDb, WalletDb};
use zcash_keys::keys::UnifiedFullViewingKey;
use zcash_primitives::block::BlockHash;
use zcash_primitives::transaction::{builder::DEFAULT_TX_EXPIRY_DELTA, fees::zip317};
use zcash_protocol::consensus::{BlockHeight, BranchId, Parameters};

use super::fixture::{ValidatedBlock, ValidatedFixture};
use super::store::{
    AccountPaths, StateRoot, open_read_only_connection, validate_account_paths,
    validate_cache_schema, validate_scan_binding,
};
use super::{AccountId, MAX_COMPACT_BLOCK_BYTES, Network, ZecError};

const CANDIDATE_FILE: &str = "compact.sqlite3.candidate";
const MAX_CACHE_ROWS: usize = 4_096;
const MAX_CACHE_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ScanFaultPort {
    RollbackWrite,
    RollbackSync,
    ReplacementApply,
    WalletDbCorrupt,
    CacheDbCorrupt,
    CommitSync,
}

#[derive(Clone, Debug)]
pub(crate) enum ScanRequest {
    Canonical,
    Through(u32),
    Scenario(String),
}

pub(crate) struct ScanPlan<'a> {
    pub(crate) fixture: &'a ValidatedFixture,
    pub(crate) request: ScanRequest,
    pub(crate) fault: Option<ScanFaultPort>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ScanMetrics {
    pub(crate) scan_calls: usize,
    pub(crate) applied_block_count: usize,
    pub(crate) unrelated_output_count: Option<usize>,
    pub(crate) rolled_back_note_count: Option<usize>,
    pub(crate) rolled_back_block_count: Option<usize>,
    pub(crate) applied_replacement_note_count: Option<usize>,
    pub(crate) balance_override: Option<u64>,
    pub(crate) last_block_allocation: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScanBalances {
    pub(crate) transparent_zat: String,
    pub(crate) sapling_zat: String,
    pub(crate) orchard_migration_required_zat: String,
    pub(crate) ironwood_pending_zat: String,
    pub(crate) ironwood_spendable_zat: String,
    pub(crate) total_zat: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScanInspection {
    pub(crate) tip_height: u32,
    pub(crate) tip_hash: String,
    pub(crate) tree_root: String,
    pub(crate) receiver_sequence: String,
    pub(crate) balances: ScanBalances,
    pub(crate) pool_classification: String,
}

#[derive(Clone, Debug)]
struct CacheRow {
    height: u32,
    data: Vec<u8>,
}

struct CacheIdentity {
    rows: Vec<CacheRow>,
    blocks: Vec<CompactBlock>,
    digest: [u8; 32],
    total_shielded_outputs: usize,
}

impl CacheIdentity {
    fn tip(&self) -> Option<(BlockHeight, BlockHash)> {
        let block = self.blocks.last()?;
        let height = u32::try_from(block.height).ok()?;
        BlockHash::try_from_slice(&block.hash).map(|hash| (BlockHeight::from_u32(height), hash))
    }

    fn exact_eq(&self, other: &Self) -> bool {
        self.digest == other.digest
            && self.rows.len() == other.rows.len()
            && self
                .rows
                .iter()
                .zip(&other.rows)
                .all(|(left, right)| left.height == right.height && left.data == right.data)
    }
}

pub(crate) fn execute(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    plan: ScanPlan<'_>,
    metrics: &mut ScanMetrics,
) -> Result<(), ZecError> {
    match network {
        Network::Local(local)
            if plan.fixture.manifest.network.discriminator == network.as_str()
                && plan.fixture.manifest.network.birthday_height == local.birthday_height()
                && plan.fixture.manifest.network.nu6_3 == local.nu6_3_height()
                && plan.fixture.manifest.expected.confirmation_height
                    == local.confirmation_height() => {}
        _ => return Err(ZecError::protocol_incompatible()),
    }
    metrics.applied_block_count = 0;
    metrics.unrelated_output_count = None;
    metrics.rolled_back_note_count = None;
    metrics.rolled_back_block_count = None;
    metrics.applied_replacement_note_count = None;
    metrics.scan_calls = metrics
        .scan_calls
        .checked_add(1)
        .ok_or_else(ZecError::limit)?;
    validate_account_paths(root, paths)?;
    refuse_sidecars(&paths.compact)?;
    let binding = open_read_only_connection(root, &paths.wallet)?;
    validate_scan_binding(&binding, account_id, network)?;
    drop(binding);
    match network {
        Network::Testnet => execute_with_params(
            root,
            paths,
            account_id,
            network,
            zcash_protocol::consensus::Network::TestNetwork,
            plan,
            metrics,
        ),
        Network::Local(local) => execute_with_params(
            root,
            paths,
            account_id,
            network,
            local.upstream(),
            plan,
            metrics,
        ),
    }
}

pub(crate) fn recover_account(
    root: &StateRoot,
    paths: &AccountPaths,
    network: Network,
) -> Result<(), ZecError> {
    match network {
        Network::Testnet => recover(root, paths, zcash_protocol::consensus::Network::TestNetwork),
        Network::Local(local) => recover(root, paths, local.upstream()),
    }
}

fn execute_with_params<P: Parameters + Clone + Send + 'static>(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    params_value: P,
    plan: ScanPlan<'_>,
    metrics: &mut ScanMetrics,
) -> Result<(), ZecError> {
    let ScanPlan {
        fixture,
        request,
        fault,
    } = plan;
    let expected_branch = u32::from_str_radix(&fixture.manifest.expected.nu6_3_branch_id_hex, 16)
        .map_err(|_| ZecError::schema())?;
    if u32::from(BranchId::for_height(
        &params_value,
        BlockHeight::from_u32(fixture.manifest.network.nu6_3),
    )) != expected_branch
    {
        return Err(ZecError::protocol_incompatible());
    }
    recover(root, paths, params_value.clone())?;
    let old_identity = cache_identity(&paths.compact)?;
    let old_tip = wallet_tip(&paths.wallet, params_value.clone())?;
    if old_tip != old_identity.tip() && (old_tip.is_some() || !old_identity.rows.is_empty()) {
        return Err(ZecError::state_corrupt());
    }

    let (intended, reorg) = intended_blocks(fixture, &request, old_tip)?;
    let stable_row_count = if reorg {
        if old_identity.rows.len() != intended.len() {
            return Err(ZecError::state_corrupt());
        }
        old_identity
            .rows
            .len()
            .checked_sub(1)
            .ok_or_else(ZecError::state_corrupt)?
    } else {
        old_identity.rows.len()
    };
    if stable_row_count > intended.len()
        || old_identity
            .rows
            .iter()
            .take(stable_row_count)
            .zip(intended.iter().take(stable_row_count))
            .any(|(row, block)| row.data != block.bytes)
    {
        return Err(ZecError::state_corrupt());
    }
    if !reorg
        && old_identity.rows.len() == intended.len()
        && old_identity
            .rows
            .iter()
            .zip(&intended)
            .all(|(row, block)| row.data == block.bytes)
    {
        metrics.applied_block_count = old_identity.rows.len();
        metrics.unrelated_output_count = Some(
            old_identity
                .total_shielded_outputs
                .checked_sub(recognized_note_count(&paths.wallet)?)
                .ok_or_else(ZecError::state_corrupt)?,
        );
        metrics.rolled_back_note_count = Some(0);
        metrics.rolled_back_block_count = Some(0);
        metrics.applied_replacement_note_count = Some(0);
        return Ok(());
    }
    let candidate = paths.directory.join(CANDIDATE_FILE);
    build_candidate(
        root,
        paths,
        &candidate,
        &old_identity.rows,
        &intended,
        fault,
    )?;
    let intended_identity = cache_identity(&candidate)?;
    validate_intended_fixture(&intended_identity, &intended)?;
    let intended_tip = intended_identity
        .tip()
        .ok_or_else(ZecError::state_corrupt)?;

    if old_identity.exact_eq(&intended_identity) {
        remove_candidate(&candidate)?;
        metrics.applied_block_count = old_identity.rows.len();
        metrics.unrelated_output_count = Some(
            old_identity
                .total_shielded_outputs
                .checked_sub(recognized_note_count(&paths.wallet)?)
                .ok_or_else(ZecError::state_corrupt)?,
        );
        metrics.rolled_back_note_count = Some(0);
        metrics.rolled_back_block_count = Some(0);
        metrics.applied_replacement_note_count = Some(0);
        return Ok(());
    }

    let states = derive_chain_states(
        &params_value,
        fixture.manifest.network.checkpoint_height,
        &intended_identity.blocks,
    )?;
    let birthday_state = states
        .get(&fixture.manifest.network.checkpoint_height)
        .cloned()
        .ok_or_else(ZecError::state_corrupt)?;
    let from_height = if reorg {
        intended_tip.0
    } else if let Some((height, _)) = old_tip {
        BlockHeight::from_u32(
            u32::from(height)
                .checked_add(1)
                .ok_or_else(ZecError::limit)?,
        )
    } else {
        BlockHeight::from_u32(fixture.manifest.network.birthday_height)
    };
    let prior_height = u32::from(from_height)
        .checked_sub(1)
        .ok_or_else(ZecError::state_corrupt)?;
    let prior_state = states
        .get(&prior_height)
        .cloned()
        .ok_or_else(ZecError::state_corrupt)?;
    let scan_limit = usize::try_from(
        u32::from(intended_tip.0)
            .checked_sub(u32::from(from_height))
            .and_then(|value| value.checked_add(1))
            .ok_or_else(ZecError::state_corrupt)?,
    )
    .map_err(|_| ZecError::limit())?;
    let ufvk_text = stored_ufvk(root, paths, account_id, network)?;
    let ufvk = UnifiedFullViewingKey::decode(&params_value, &ufvk_text)
        .map_err(|_| ZecError::state_corrupt())?;
    let candidate_db = BlockDb::for_path(&candidate).map_err(|_| ZecError::state_corrupt())?;
    let old_tip_before = old_tip;
    let old_tip_notes = if reorg {
        let tip = old_tip.ok_or_else(ZecError::state_corrupt)?;
        recognized_notes_at_height(&paths.wallet, u32::from(tip.0))?
    } else {
        0
    };
    let closure_completed = Cell::new(false);
    let scan_params = params_value.clone();
    let mut wallet = WalletDb::for_path(&paths.wallet, params_value, SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    let result: Result<(), SqliteClientError> =
        wallet.transactionally_with_extension(|wdb, ext| {
            let accounts = wdb.get_account_ids()?;
            if accounts.len() > 1 {
                return Err(logical_abort());
            }
            if accounts.is_empty() {
                wdb.import_account_ufvk(
                    "BitBook viewing account",
                    &ufvk,
                    &AccountBirthday::from_parts(birthday_state.clone(), None),
                    AccountPurpose::ViewOnly,
                    None,
                )?;
            } else {
                let official = wdb.get_account(accounts[0])?.ok_or_else(logical_abort)?;
                if official.purpose() != AccountPurpose::ViewOnly
                    || official
                        .ufvk()
                        .map(|stored| stored.encode(&scan_params))
                        .as_deref()
                        != Some(ufvk_text.as_str())
                {
                    return Err(logical_abort());
                }
            }
            if fault == Some(ScanFaultPort::WalletDbCorrupt) {
                return Err(logical_abort());
            }
            if reorg {
                if fault == Some(ScanFaultPort::RollbackWrite) {
                    return Err(logical_abort());
                }
                wdb.truncate_to_chain_state(prior_state.clone())?;
                if fault == Some(ScanFaultPort::RollbackSync)
                    || fault == Some(ScanFaultPort::ReplacementApply)
                {
                    return Err(logical_abort());
                }
            }
            wdb.update_chain_tip(intended_tip.0)?;
            scan_cached_blocks(
                &scan_params,
                &candidate_db,
                wdb,
                from_height,
                &prior_state,
                scan_limit,
            )
            .map_err(|_| logical_abort())?;
            let updated = ext.execute(
                "UPDATE ext_bitbook_store_state SET scan_tip = ?1 WHERE account_id = ?2",
                params![i64::from(u32::from(intended_tip.0)), account_id.as_str()],
            )?;
            if updated != 1 || fault == Some(ScanFaultPort::CommitSync) {
                return Err(logical_abort());
            }
            closure_completed.set(true);
            Ok(())
        });
    drop(wallet);
    drop(candidate_db);

    match result {
        Ok(()) => {}
        Err(_) if !closure_completed.get() => {
            remove_candidate(&candidate)?;
            return Err(
                if matches!(
                    fault,
                    Some(
                        ScanFaultPort::WalletDbCorrupt
                            | ScanFaultPort::CacheDbCorrupt
                            | ScanFaultPort::CommitSync
                    )
                ) {
                    ZecError::state_corrupt()
                } else {
                    ZecError::internal()
                },
            );
        }
        Err(_) => {
            let observed = wallet_tip(&paths.wallet, scan_params.clone())?;
            if observed == old_tip_before
                && cache_matches_wallet(
                    &paths.wallet,
                    scan_params.clone(),
                    &old_identity,
                    old_tip_before,
                )?
            {
                remove_candidate(&candidate)?;
                return Err(ZecError::state_corrupt());
            }
            if observed != Some(intended_tip)
                || !cache_matches_wallet(
                    &paths.wallet,
                    scan_params.clone(),
                    &intended_identity,
                    Some(intended_tip),
                )?
            {
                return Err(ZecError::state_corrupt());
            }
        }
    }

    metrics.applied_block_count = intended_identity.rows.len();
    metrics.rolled_back_note_count = Some(if reorg { old_tip_notes } else { 0 });
    metrics.rolled_back_block_count = Some(usize::from(reorg));
    metrics.applied_replacement_note_count =
        match recognized_notes_at_height(&paths.wallet, u32::from(intended_tip.0)) {
            Ok(new_tip_notes) => Some(if reorg { new_tip_notes } else { 0 }),
            Err(_) => None,
        };
    metrics.unrelated_output_count = match recognized_note_count(&paths.wallet) {
        Ok(recognized) => intended_identity
            .total_shielded_outputs
            .checked_sub(recognized),
        Err(_) => None,
    };

    promote_candidate(&candidate, &paths.compact, &paths.directory);
    Ok(())
}

fn intended_blocks(
    fixture: &ValidatedFixture,
    request: &ScanRequest,
    old_tip: Option<(BlockHeight, BlockHash)>,
) -> Result<(Vec<ValidatedBlock>, bool), ZecError> {
    match request {
        ScanRequest::Canonical => {
            let height = fixture
                .manifest
                .scenarios
                .canonical
                .last()
                .and_then(|name| {
                    fixture
                        .manifest
                        .files
                        .iter()
                        .find(|file| &file.name == name)
                })
                .and_then(|file| file.block_height)
                .ok_or_else(ZecError::schema)?;
            fixture
                .canonical_through(height)
                .map(|blocks| (blocks, false))
        }
        ScanRequest::Through(height) => fixture
            .canonical_through(*height)
            .map(|blocks| (blocks, false)),
        ScanRequest::Scenario(scenario) => match scenario.as_str() {
            "one-block-reorg" => {
                let old_height = old_tip
                    .map(|tip| u32::from(tip.0))
                    .ok_or_else(ZecError::state_corrupt)?;
                let replacement = fixture.scenario_file(scenario)?;
                if replacement.height_hint != Some(old_height) {
                    return Err(ZecError::state_corrupt());
                }
                let mut blocks = fixture.canonical_through(
                    old_height
                        .checked_sub(1)
                        .ok_or_else(ZecError::state_corrupt)?,
                )?;
                blocks.push(replacement);
                Ok((blocks, true))
            }
            "two-block-reorg" => Err(ZecError::state_corrupt()),
            _ => {
                let current = old_tip
                    .map(|tip| u32::from(tip.0))
                    .ok_or_else(ZecError::state_corrupt)?;
                let hostile = fixture.scenario_file(scenario)?;
                let canonical_tip = fixture
                    .manifest
                    .scenarios
                    .canonical
                    .last()
                    .and_then(|name| {
                        fixture
                            .manifest
                            .files
                            .iter()
                            .find(|file| &file.name == name)
                    })
                    .and_then(|file| file.block_height)
                    .ok_or_else(ZecError::schema)?;
                let prefix_tip = hostile
                    .height_hint
                    .and_then(|height| height.checked_sub(1))
                    .map(|height| height.min(canonical_tip))
                    .unwrap_or(current);
                let mut blocks = fixture.canonical_through(prefix_tip)?;
                blocks.push(hostile);
                Ok((blocks, false))
            }
        },
    }
}

fn build_candidate(
    root: &StateRoot,
    paths: &AccountPaths,
    candidate: &Path,
    committed: &[CacheRow],
    intended: &[ValidatedBlock],
    fault: Option<ScanFaultPort>,
) -> Result<(), ZecError> {
    validate_account_paths(root, paths)?;
    refuse_sidecars(&paths.compact)?;
    refuse_sidecars(candidate)?;
    match fs::symlink_metadata(candidate) {
        Ok(_) => return Err(ZecError::state_corrupt()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err(ZecError::state_corrupt()),
    }
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(candidate)
        .map_err(|_| ZecError::state_corrupt())?;
    validate_candidate_file(root, candidate)?;
    let mut connection = Connection::open(candidate).map_err(|_| ZecError::state_corrupt())?;
    connection
        .pragma_update(None, "journal_mode", "DELETE")
        .map_err(|_| ZecError::state_corrupt())?;
    connection
        .pragma_update(None, "synchronous", "FULL")
        .map_err(|_| ZecError::state_corrupt())?;
    let journal: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    let synchronous: i64 = connection
        .query_row("PRAGMA synchronous", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    if !journal.eq_ignore_ascii_case("delete") || synchronous != 2 {
        return Err(ZecError::state_corrupt());
    }
    connection
        .execute_batch(
            "CREATE TABLE compactblocks (
                height INTEGER PRIMARY KEY,
                data BLOB NOT NULL
            );",
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ZecError::state_corrupt())?;
    for row in committed {
        transaction
            .execute(
                "INSERT INTO compactblocks (height, data) VALUES (?1, ?2)",
                params![i64::from(row.height), &row.data],
            )
            .map_err(|_| ZecError::state_corrupt())?;
    }
    let first_height = intended
        .first()
        .and_then(|block| block.height_hint)
        .ok_or_else(ZecError::state_corrupt)?;
    let last_height = first_height
        .checked_add(
            u32::try_from(
                intended
                    .len()
                    .checked_sub(1)
                    .ok_or_else(ZecError::state_corrupt)?,
            )
            .map_err(|_| ZecError::limit())?,
        )
        .ok_or_else(ZecError::limit)?;
    transaction
        .execute(
            "DELETE FROM compactblocks WHERE height < ?1 OR height > ?2",
            params![i64::from(first_height), i64::from(last_height)],
        )
        .map_err(|_| ZecError::state_corrupt())?;
    for (index, block) in intended.iter().enumerate() {
        let offset = u32::try_from(index).map_err(|_| ZecError::limit())?;
        let fallback = intended
            .first()
            .and_then(|first| first.height_hint)
            .and_then(|height| height.checked_add(offset));
        let height = block
            .height_hint
            .or(fallback)
            .ok_or_else(ZecError::state_corrupt)?;
        if committed
            .get(index)
            .is_some_and(|row| row.height == height && row.data == block.bytes)
        {
            continue;
        }
        transaction
            .execute(
                "INSERT OR REPLACE INTO compactblocks (height, data) VALUES (?1, ?2)",
                params![i64::from(height), &block.bytes],
            )
            .map_err(|_| ZecError::state_corrupt())?;
    }
    if fault == Some(ScanFaultPort::CacheDbCorrupt) {
        let changed = transaction
            .execute(
                "UPDATE compactblocks SET data = x'ff'
                 WHERE height = (SELECT MAX(height) FROM compactblocks)",
                [],
            )
            .map_err(|_| ZecError::state_corrupt())?;
        if changed != 1 {
            return Err(ZecError::state_corrupt());
        }
    }
    transaction
        .commit()
        .map_err(|_| ZecError::state_corrupt())?;
    drop(connection);
    fs::File::open(candidate)
        .and_then(|file| file.sync_all())
        .map_err(|_| ZecError::state_corrupt())?;
    sync_directory(&paths.directory)?;
    refuse_sidecars(candidate)?;
    validate_candidate_file(root, candidate)
}

fn cache_identity(path: &Path) -> Result<CacheIdentity, ZecError> {
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| ZecError::state_corrupt())?;
    let quick_check: String = connection
        .query_row("PRAGMA quick_check", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    if quick_check != "ok" {
        return Err(ZecError::state_corrupt());
    }
    validate_cache_schema(&connection)?;
    let mut statement = connection
        .prepare(
            "SELECT height, length(CAST(data AS BLOB)), data
             FROM compactblocks ORDER BY height LIMIT ?1",
        )
        .map_err(|_| ZecError::state_corrupt())?;
    let mut mapped = statement
        .query([i64::try_from(MAX_CACHE_ROWS + 1).map_err(|_| ZecError::limit())?])
        .map_err(|_| ZecError::state_corrupt())?;
    let mut rows = Vec::new();
    let mut bytes_total = 0usize;
    while let Some(row) = mapped.next().map_err(|_| ZecError::state_corrupt())? {
        let height = row
            .get::<_, i64>(0)
            .map_err(|_| ZecError::state_corrupt())?;
        let length = usize::try_from(
            row.get::<_, i64>(1)
                .map_err(|_| ZecError::state_corrupt())?,
        )
        .map_err(|_| ZecError::state_corrupt())?;
        if rows.len() == MAX_CACHE_ROWS || length > MAX_COMPACT_BLOCK_BYTES {
            return Err(ZecError::limit());
        }
        bytes_total = bytes_total
            .checked_add(length)
            .ok_or_else(ZecError::limit)?;
        if bytes_total > MAX_CACHE_BYTES {
            return Err(ZecError::limit());
        }
        let data = row
            .get::<_, Vec<u8>>(2)
            .map_err(|_| ZecError::state_corrupt())?;
        if data.len() != length {
            return Err(ZecError::state_corrupt());
        }
        rows.push(CacheRow {
            height: u32::try_from(height).map_err(|_| ZecError::state_corrupt())?,
            data,
        });
    }
    drop(mapped);
    drop(statement);
    drop(connection);

    let block_db = BlockDb::for_path(path).map_err(|_| ZecError::state_corrupt())?;
    let mut blocks = Vec::new();
    block_db
        .with_blocks::<_, ZecError>(None, None, |block| {
            if block.height > u64::from(u32::MAX)
                || block.hash.len() != 32
                || block.prev_hash.len() != 32
            {
                return Err(zcash_client_backend::data_api::chain::error::Error::Wallet(
                    ZecError::state_corrupt(),
                ));
            }
            blocks.push(block);
            Ok(())
        })
        .map_err(|_| ZecError::state_corrupt())?;
    if rows.len() != blocks.len() {
        return Err(ZecError::state_corrupt());
    }
    for (index, (row, block)) in rows.iter().zip(&blocks).enumerate() {
        if row.height != u32::try_from(block.height).map_err(|_| ZecError::state_corrupt())?
            || (index > 0
                && (row.height
                    != rows[index - 1]
                        .height
                        .checked_add(1)
                        .ok_or_else(ZecError::limit)?
                    || block.prev_hash != blocks[index - 1].hash))
        {
            return Err(ZecError::state_corrupt());
        }
    }
    let mut hasher = Sha256::new();
    let mut output_count = 0usize;
    for (row, block) in rows.iter().zip(&blocks) {
        hasher.update(row.height.to_le_bytes());
        hasher.update(
            u64::try_from(row.data.len())
                .map_err(|_| ZecError::limit())?
                .to_le_bytes(),
        );
        hasher.update(&row.data);
        for transaction in &block.vtx {
            output_count = output_count
                .checked_add(transaction.outputs.len())
                .and_then(|value| value.checked_add(transaction.actions.len()))
                .and_then(|value| value.checked_add(transaction.ironwood_actions.len()))
                .ok_or_else(ZecError::limit)?;
        }
    }
    Ok(CacheIdentity {
        rows,
        blocks,
        digest: hasher.finalize().into(),
        total_shielded_outputs: output_count,
    })
}

fn validate_intended_fixture(
    identity: &CacheIdentity,
    intended: &[ValidatedBlock],
) -> Result<(), ZecError> {
    if identity.rows.len() != intended.len() {
        return Err(ZecError::state_corrupt());
    }
    for ((row, block), expected) in identity.rows.iter().zip(&identity.blocks).zip(intended) {
        if row.data != expected.bytes
            || expected
                .height_hint
                .is_some_and(|height| height != row.height)
        {
            return Err(ZecError::state_corrupt());
        }
        if expected
            .hash_hint
            .as_ref()
            .is_some_and(|expected_hash| expected_hash != &hex(&block.hash))
            || expected
                .previous_hash_hint
                .as_ref()
                .is_some_and(|expected_hash| expected_hash != &hex(&block.prev_hash))
        {
            return Err(ZecError::state_corrupt());
        }
    }
    Ok(())
}

fn derive_chain_states<P: Parameters + Send + 'static>(
    params: &P,
    checkpoint_height: u32,
    blocks: &[CompactBlock],
) -> Result<BTreeMap<u32, ChainState>, ZecError> {
    let first = blocks.first().ok_or_else(ZecError::state_corrupt)?;
    let birthday_height = checkpoint_height
        .checked_add(1)
        .ok_or_else(ZecError::limit)?;
    if first.height != u64::from(birthday_height) || first.prev_hash.len() != 32 {
        return Err(ZecError::state_corrupt());
    }
    let mut state = ChainState::empty(
        BlockHeight::from_u32(checkpoint_height),
        BlockHash::try_from_slice(&first.prev_hash).ok_or_else(ZecError::state_corrupt)?,
    );
    let mut states = BTreeMap::new();
    states.insert(checkpoint_height, state.clone());
    let keys = ScanningKeys::<u8, u8>::empty();
    let nullifiers = Nullifiers::<u8>::empty();
    let mut prior_metadata = None;
    for block in blocks {
        let scanned = scan_block(
            params,
            block.clone(),
            &keys,
            &nullifiers,
            prior_metadata.as_ref(),
        )
        .map_err(|_| ZecError::protocol_incompatible())?;
        let height = u32::from(scanned.height());
        let hash = scanned.block_hash();
        let metadata = scanned.to_block_metadata();
        let commitments = scanned.into_commitments();
        let mut sapling = state.final_sapling_tree().clone();
        for (commitment, _) in commitments.sapling {
            if !sapling.append(commitment) {
                return Err(ZecError::state_corrupt());
            }
        }
        let mut orchard = state.final_orchard_tree().clone();
        for (commitment, _) in commitments.orchard {
            if !orchard.append(commitment) {
                return Err(ZecError::state_corrupt());
            }
        }
        let mut ironwood = state.final_ironwood_tree().clone();
        for (commitment, _) in commitments.ironwood {
            if !ironwood.append(commitment) {
                return Err(ZecError::state_corrupt());
            }
        }
        state = ChainState::new(
            BlockHeight::from_u32(height),
            hash,
            sapling,
            orchard,
            ironwood,
        );
        states.insert(height, state.clone());
        prior_metadata = Some(metadata);
    }
    Ok(states)
}

fn recover<P: Parameters + Clone + Send + 'static>(
    root: &StateRoot,
    paths: &AccountPaths,
    params: P,
) -> Result<(), ZecError> {
    validate_account_paths(root, paths)?;
    refuse_sidecars(&paths.compact)?;
    let candidate = paths.directory.join(CANDIDATE_FILE);
    refuse_sidecars(&candidate)?;
    let committed = cache_identity(&paths.compact)?;
    let wallet_tip = wallet_tip(&paths.wallet, params.clone())?;
    let account_count = wallet_account_count(&paths.wallet, params.clone())?;
    let candidate_exists = match fs::symlink_metadata(&candidate) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(_) => return Err(ZecError::state_corrupt()),
    };
    if !candidate_exists {
        if account_count == 0 {
            return if wallet_tip.is_none() && committed.rows.is_empty() {
                Ok(())
            } else {
                Err(ZecError::state_corrupt())
            };
        }
        if account_count != 1 || wallet_tip.is_none() {
            return Err(ZecError::state_corrupt());
        }
        return if cache_matches_wallet(&paths.wallet, params, &committed, wallet_tip)? {
            Ok(())
        } else {
            Err(ZecError::state_corrupt())
        };
    }
    validate_candidate_file(root, &candidate)?;
    refuse_sidecars(&candidate)?;
    let candidate_identity = cache_identity(&candidate);
    if account_count == 0 {
        if wallet_tip.is_none() && committed.rows.is_empty() {
            return remove_candidate(&candidate);
        }
        return Err(ZecError::state_corrupt());
    }
    if account_count != 1 || wallet_tip.is_none() {
        return Err(ZecError::state_corrupt());
    }
    let committed_matches =
        cache_matches_wallet(&paths.wallet, params.clone(), &committed, wallet_tip)?;
    let candidate = match candidate_identity {
        Ok(identity) => identity,
        Err(_) if committed_matches => return remove_candidate(&candidate),
        Err(error) => return Err(error),
    };
    let candidate_matches = cache_matches_wallet(&paths.wallet, params, &candidate, wallet_tip)?;
    match (committed_matches, candidate_matches) {
        (true, false) => remove_candidate(&paths.directory.join(CANDIDATE_FILE)),
        (false, true) => promote_candidate_checked(
            &paths.directory.join(CANDIDATE_FILE),
            &paths.compact,
            &paths.directory,
        ),
        (true, true) if committed.exact_eq(&candidate) => {
            remove_candidate(&paths.directory.join(CANDIDATE_FILE))
        }
        _ => Err(ZecError::state_corrupt()),
    }
}

pub(crate) fn inspect(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    checkpoint_height: u32,
    balance_override: Option<u64>,
) -> Result<ScanInspection, ZecError> {
    validate_account_paths(root, paths)?;
    match network {
        Network::Testnet => inspect_with_params(
            root,
            paths,
            account_id,
            network,
            zcash_protocol::consensus::Network::TestNetwork,
            checkpoint_height,
            balance_override,
        ),
        Network::Local(local) => inspect_with_params(
            root,
            paths,
            account_id,
            network,
            local.upstream(),
            checkpoint_height,
            balance_override,
        ),
    }
}

fn inspect_with_params<P: Parameters + Clone + Send + 'static>(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
    params: P,
    checkpoint_height: u32,
    balance_override: Option<u64>,
) -> Result<ScanInspection, ZecError> {
    recover(root, paths, params.clone())?;
    let mut connection = open_read_only_connection(root, &paths.wallet)?;
    rusqlite::vtab::array::load_module(&connection).map_err(|_| ZecError::state_corrupt())?;
    let before = data_version(&connection)?;
    let observation = (|| {
        validate_scan_binding(&connection, account_id, network)?;
        let receiver_sequence = receiver_sequence(&connection, account_id)?;
        let expected_ufvk = stored_ufvk_with_connection(&connection, account_id)?;
        let encoding_params = params.clone();
        let official = {
            let mut wallet = WalletDb::from_connection(&mut connection, params, SystemClock, OsRng);
            let ids = wallet
                .get_account_ids()
                .map_err(|_| ZecError::state_corrupt())?;
            if ids.is_empty() {
                if wallet
                    .get_max_height_hash()
                    .map_err(|_| ZecError::state_corrupt())?
                    .is_some()
                {
                    return Err(ZecError::state_corrupt());
                }
                return Ok(ScanInspection {
                    tip_height: checkpoint_height,
                    tip_hash: hex(&[0; 32]),
                    tree_root: empty_tree_digest(),
                    receiver_sequence: receiver_sequence.to_string(),
                    balances: test_override_or_zero(balance_override),
                    pool_classification: pool_classification(0, 0, 0, 0, 0),
                });
            }
            if ids.len() != 1 {
                return Err(ZecError::state_corrupt());
            }
            let account_record = wallet
                .get_account(ids[0])
                .map_err(|_| ZecError::state_corrupt())?
                .ok_or_else(ZecError::state_corrupt)?;
            if account_record.purpose() != AccountPurpose::ViewOnly
                || account_record
                    .ufvk()
                    .map(|stored| stored.encode(&encoding_params))
                    .as_deref()
                    != Some(expected_ufvk.as_str())
            {
                return Err(ZecError::state_corrupt());
            }
            let (height, hash) = wallet
                .get_max_height_hash()
                .map_err(|_| ZecError::state_corrupt())?
                .ok_or_else(ZecError::state_corrupt)?;
            let tree_root = official_tree_digest(&mut wallet)?;
            if let Some(value) = balance_override {
                return Ok(ScanInspection {
                    tip_height: u32::from(height),
                    tip_hash: hex(&hash.0),
                    tree_root,
                    receiver_sequence: receiver_sequence.to_string(),
                    balances: ScanBalances {
                        total_zat: value.to_string(),
                        ..zero_balances()
                    },
                    pool_classification: pool_classification(0, 0, 0, 0, 0),
                });
            }
            let target_height = u32::from(height)
                .checked_add(1)
                .ok_or_else(ZecError::state_corrupt)?;

            let policy = ConfirmationsPolicy::new_symmetrical(
                NonZeroU32::new(3).ok_or_else(ZecError::internal)?,
                false,
            );
            let summary = wallet
                .get_wallet_summary(policy)
                .map_err(|_| ZecError::state_corrupt())?
                .ok_or_else(ZecError::state_corrupt)?;
            let account = summary
                .account_balances()
                .get(&ids[0])
                .ok_or_else(ZecError::state_corrupt)?;
            let sapling = account.sapling_balance();
            let orchard = account.orchard_balance();
            let ironwood = account.ironwood_balance();
            OfficialBalanceObservation {
                account: ids[0],
                height,
                hash,
                tree_root,
                target_height,
                transparent: account.unshielded_balance().total().into_u64(),
                account_total: account.total().into_u64(),
                sapling_total: sapling.total().into_u64(),
                orchard_total: orchard.total().into_u64(),
                ironwood_total: ironwood.total().into_u64(),
                ironwood_spendable: ironwood.spendable_value().into_u64(),
                sapling_pending_capacity: sapling
                    .change_pending_confirmation()
                    .into_u64()
                    .checked_add(sapling.value_pending_spendability().into_u64())
                    .ok_or_else(ZecError::state_corrupt)?,
                orchard_pending_capacity: orchard
                    .change_pending_confirmation()
                    .into_u64()
                    .checked_add(orchard.value_pending_spendability().into_u64())
                    .ok_or_else(ZecError::state_corrupt)?,
                ironwood_pending_capacity: ironwood
                    .change_pending_confirmation()
                    .into_u64()
                    .checked_add(ironwood.value_pending_spendability().into_u64())
                    .ok_or_else(ZecError::state_corrupt)?,
            }
        };

        let orphans = orphan_projection(&connection, official.account, official.target_height)?;
        if orphans.sapling > official.sapling_pending_capacity
            || orphans.orchard > official.orchard_pending_capacity
            || orphans.ironwood > official.ironwood_pending_capacity
        {
            return Err(ZecError::state_corrupt());
        }
        let adjusted_sapling = official
            .sapling_total
            .checked_sub(orphans.sapling)
            .ok_or_else(ZecError::state_corrupt)?;
        let adjusted_orchard = official
            .orchard_total
            .checked_sub(orphans.orchard)
            .ok_or_else(ZecError::state_corrupt)?;
        let adjusted_ironwood = official
            .ironwood_total
            .checked_sub(orphans.ironwood)
            .ok_or_else(ZecError::state_corrupt)?;
        let ironwood_pending = adjusted_ironwood
            .checked_sub(official.ironwood_spendable)
            .ok_or_else(ZecError::state_corrupt)?;
        if adjusted_sapling.checked_add(orphans.sapling) != Some(official.sapling_total)
            || adjusted_orchard.checked_add(orphans.orchard) != Some(official.orchard_total)
            || adjusted_ironwood.checked_add(orphans.ironwood) != Some(official.ironwood_total)
            || ironwood_pending.checked_add(official.ironwood_spendable) != Some(adjusted_ironwood)
        {
            return Err(ZecError::state_corrupt());
        }
        let official_component_total = official
            .transparent
            .checked_add(official.sapling_total)
            .and_then(|value| value.checked_add(official.orchard_total))
            .and_then(|value| value.checked_add(official.ironwood_total))
            .ok_or_else(ZecError::state_corrupt)?;
        if official_component_total != official.account_total {
            return Err(ZecError::state_corrupt());
        }
        let orphan_total = orphans
            .sapling
            .checked_add(orphans.orchard)
            .and_then(|value| value.checked_add(orphans.ironwood))
            .ok_or_else(ZecError::state_corrupt)?;
        let adjusted_account_total = official
            .account_total
            .checked_sub(orphan_total)
            .ok_or_else(ZecError::state_corrupt)?;
        let displayed_total = official
            .transparent
            .checked_add(adjusted_sapling)
            .and_then(|value| value.checked_add(adjusted_orchard))
            .and_then(|value| value.checked_add(ironwood_pending))
            .and_then(|value| value.checked_add(official.ironwood_spendable))
            .ok_or_else(ZecError::state_corrupt)?;
        if displayed_total != adjusted_account_total {
            return Err(ZecError::state_corrupt());
        }
        Ok(ScanInspection {
            tip_height: u32::from(official.height),
            tip_hash: hex(&official.hash.0),
            tree_root: official.tree_root,
            receiver_sequence: receiver_sequence.to_string(),
            balances: ScanBalances {
                transparent_zat: official.transparent.to_string(),
                sapling_zat: adjusted_sapling.to_string(),
                orchard_migration_required_zat: adjusted_orchard.to_string(),
                ironwood_pending_zat: ironwood_pending.to_string(),
                ironwood_spendable_zat: official.ironwood_spendable.to_string(),
                total_zat: displayed_total.to_string(),
            },
            pool_classification: pool_classification(
                official.transparent,
                adjusted_sapling,
                adjusted_orchard,
                ironwood_pending,
                official.ironwood_spendable,
            ),
        })
    })();
    let after = data_version(&connection)?;
    if before == after {
        observation
    } else {
        Err(ZecError::state_corrupt())
    }
}

pub(crate) fn recognized_note_count(path: &Path) -> Result<usize, ZecError> {
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| ZecError::state_corrupt())?;
    let count: i64 = connection
        .query_row(
            "SELECT
               (SELECT COUNT(*) FROM sapling_received_notes rn
                JOIN transactions tx ON tx.id_tx = rn.transaction_id
                WHERE tx.mined_height IS NOT NULL) +
               (SELECT COUNT(*) FROM orchard_received_notes rn
                JOIN transactions tx ON tx.id_tx = rn.transaction_id
                WHERE tx.mined_height IS NOT NULL) +
               (SELECT COUNT(*) FROM ironwood_received_notes rn
                JOIN transactions tx ON tx.id_tx = rn.transaction_id
                WHERE tx.mined_height IS NOT NULL)",
            [],
            |row| row.get(0),
        )
        .map_err(|_| ZecError::state_corrupt())?;
    usize::try_from(count).map_err(|_| ZecError::state_corrupt())
}

fn recognized_notes_at_height(path: &Path, height: u32) -> Result<usize, ZecError> {
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| ZecError::state_corrupt())?;
    let count: i64 = connection
        .query_row(
            "SELECT
               (SELECT COUNT(*) FROM sapling_received_notes rn
                JOIN transactions tx ON tx.id_tx = rn.transaction_id
                WHERE tx.mined_height = ?1) +
               (SELECT COUNT(*) FROM orchard_received_notes rn
                JOIN transactions tx ON tx.id_tx = rn.transaction_id
                WHERE tx.mined_height = ?1) +
               (SELECT COUNT(*) FROM ironwood_received_notes rn
                JOIN transactions tx ON tx.id_tx = rn.transaction_id
                WHERE tx.mined_height = ?1)",
            [i64::from(height)],
            |row| row.get(0),
        )
        .map_err(|_| ZecError::state_corrupt())?;
    usize::try_from(count).map_err(|_| ZecError::state_corrupt())
}

fn stored_ufvk(
    root: &StateRoot,
    paths: &AccountPaths,
    account_id: &AccountId,
    network: Network,
) -> Result<String, ZecError> {
    let connection = open_read_only_connection(root, &paths.wallet)?;
    validate_scan_binding(&connection, account_id, network)?;
    stored_ufvk_with_connection(&connection, account_id)
}

fn stored_ufvk_with_connection(
    connection: &Connection,
    account_id: &AccountId,
) -> Result<String, ZecError> {
    connection
        .query_row(
            "SELECT ufvk FROM ext_bitbook_accounts WHERE account_id = ?1",
            [account_id.as_str()],
            |row| row.get(0),
        )
        .map_err(|_| ZecError::state_corrupt())
}

fn receiver_sequence(connection: &Connection, account_id: &AccountId) -> Result<u64, ZecError> {
    let value: i64 = connection
        .query_row(
            "SELECT issued_at_sequence FROM ext_bitbook_sequence_state WHERE account_id = ?1",
            [account_id.as_str()],
            |row| row.get(0),
        )
        .map_err(|_| ZecError::state_corrupt())?;
    u64::try_from(value).map_err(|_| ZecError::state_corrupt())
}

struct OfficialBalanceObservation {
    account: AccountUuid,
    height: BlockHeight,
    hash: BlockHash,
    tree_root: String,
    target_height: u32,
    transparent: u64,
    account_total: u64,
    sapling_total: u64,
    orchard_total: u64,
    ironwood_total: u64,
    ironwood_spendable: u64,
    sapling_pending_capacity: u64,
    orchard_pending_capacity: u64,
    ironwood_pending_capacity: u64,
}

#[derive(Clone, Copy)]
struct OrphanProjection {
    sapling: u64,
    orchard: u64,
    ironwood: u64,
}

fn orphan_projection(
    connection: &Connection,
    account: AccountUuid,
    target_height: u32,
) -> Result<OrphanProjection, ZecError> {
    let target_height = i64::from(target_height);
    let expiry_delta = i64::from(DEFAULT_TX_EXPIRY_DELTA);
    let maximum_height = i64::from(u32::MAX);
    let marginal_fee =
        i64::try_from(zip317::MARGINAL_FEE.into_u64()).map_err(|_| ZecError::state_corrupt())?;
    let (sapling, orchard, ironwood, transparent, unknown, malformed) = connection
        .query_row(
            "WITH account_outputs AS (
                 SELECT ro.id_within_pool_table, ro.pool, ro.account_id, ro.value,
                        tx.id_tx AS creating_tx_id, tx.mined_height, tx.expiry_height,
                        tx.min_observed_height
                 FROM v_received_outputs ro
                 JOIN accounts ON accounts.id = ro.account_id
                 LEFT JOIN transactions tx ON tx.id_tx = ro.transaction_id
                 WHERE accounts.uuid = ?1
             ), classified AS (
                 SELECT account_outputs.*,
                        CASE WHEN pool IN (2, 3, 4)
                               AND mined_height IS NULL
                               AND (
                                   expiry_height = 0
                                   OR expiry_height >= ?2
                                   OR (
                                       expiry_height IS NULL
                                       AND min_observed_height BETWEEN 0 AND ?5
                                       AND min_observed_height >= MAX(0, ?2 - ?3)
                                   )
                               )
                               AND value > ?4
                               AND NOT EXISTS (
                                   SELECT 1
                                   FROM v_received_output_spends spends
                                   JOIN transactions spending_tx
                                     ON spending_tx.id_tx = spends.transaction_id
                                   WHERE spends.account_id = account_outputs.account_id
                                     AND spends.pool = account_outputs.pool
                                     AND spends.received_output_id =
                                         account_outputs.id_within_pool_table
                                     AND (
                                         spending_tx.mined_height < ?2
                                         OR spending_tx.expiry_height = 0
                                         OR spending_tx.expiry_height >= ?2
                                         OR (
                                             spending_tx.expiry_height IS NULL
                                             AND spending_tx.min_observed_height BETWEEN 0 AND ?5
                                             AND spending_tx.min_observed_height >=
                                                 MAX(0, ?2 - ?3)
                                         )
                                     )
                               )
                             THEN 1 ELSE 0 END AS is_orphan,
                        CASE WHEN creating_tx_id IS NULL
                                   OR value < 0
                                   OR mined_height < 0 OR mined_height > ?5
                                   OR expiry_height < 0 OR expiry_height > ?5
                                   OR min_observed_height < 0 OR min_observed_height > ?5
                                   OR EXISTS (
                                       SELECT 1
                                       FROM v_received_output_spends spends
                                       LEFT JOIN transactions spending_tx
                                         ON spending_tx.id_tx = spends.transaction_id
                                       WHERE spends.account_id = account_outputs.account_id
                                         AND spends.pool = account_outputs.pool
                                         AND spends.received_output_id =
                                             account_outputs.id_within_pool_table
                                         AND (
                                             spending_tx.id_tx IS NULL
                                             OR spending_tx.mined_height < 0
                                             OR spending_tx.mined_height > ?5
                                             OR spending_tx.expiry_height < 0
                                             OR spending_tx.expiry_height > ?5
                                             OR spending_tx.min_observed_height < 0
                                             OR spending_tx.min_observed_height > ?5
                                         )
                                   )
                             THEN 1 ELSE 0 END AS malformed
                 FROM account_outputs
             )
             SELECT
                 COALESCE(SUM(CASE WHEN pool = 2 AND is_orphan = 1 THEN value ELSE 0 END), 0),
                 COALESCE(SUM(CASE WHEN pool = 3 AND is_orphan = 1 THEN value ELSE 0 END), 0),
                 COALESCE(SUM(CASE WHEN pool = 4 AND is_orphan = 1 THEN value ELSE 0 END), 0),
                 COALESCE(SUM(CASE WHEN pool = 0 AND mined_height IS NULL THEN 1 ELSE 0 END), 0),
                 COALESCE(SUM(CASE WHEN pool IS NULL OR pool NOT IN (0, 2, 3, 4)
                                   THEN 1 ELSE 0 END), 0),
                 COALESCE(SUM(malformed), 0)
             FROM classified",
            params![
                account.expose_uuid(),
                target_height,
                expiry_delta,
                marginal_fee,
                maximum_height,
            ],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                ))
            },
        )
        .map_err(|_| ZecError::state_corrupt())?;
    if transparent != 0 || unknown != 0 || malformed != 0 {
        return Err(ZecError::state_corrupt());
    }
    Ok(OrphanProjection {
        sapling: u64::try_from(sapling).map_err(|_| ZecError::state_corrupt())?,
        orchard: u64::try_from(orchard).map_err(|_| ZecError::state_corrupt())?,
        ironwood: u64::try_from(ironwood).map_err(|_| ZecError::state_corrupt())?,
    })
}

fn data_version(connection: &Connection) -> Result<u64, ZecError> {
    let version: i64 = connection
        .query_row("PRAGMA main.data_version", [], |row| row.get(0))
        .map_err(|_| ZecError::state_corrupt())?;
    u64::try_from(version).map_err(|_| ZecError::state_corrupt())
}

fn wallet_tip<P: Parameters + Send + 'static>(
    path: &Path,
    params: P,
) -> Result<Option<(BlockHeight, BlockHash)>, ZecError> {
    let wallet = WalletDb::for_path(path, params, SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    wallet
        .get_max_height_hash()
        .map_err(|_| ZecError::state_corrupt())
}

fn wallet_account_count<P: Parameters + Send + 'static>(
    path: &Path,
    params: P,
) -> Result<usize, ZecError> {
    let wallet = WalletDb::for_path(path, params, SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    wallet
        .get_account_ids()
        .map(|ids| ids.len())
        .map_err(|_| ZecError::state_corrupt())
}

fn cache_matches_wallet<P: Parameters + Send + 'static>(
    path: &Path,
    params: P,
    identity: &CacheIdentity,
    expected_tip: Option<(BlockHeight, BlockHash)>,
) -> Result<bool, ZecError> {
    if identity.tip() != expected_tip {
        return Ok(false);
    }
    let wallet = WalletDb::for_path(path, params, SystemClock, OsRng)
        .map_err(|_| ZecError::state_corrupt())?;
    let account_ids = wallet
        .get_account_ids()
        .map_err(|_| ZecError::state_corrupt())?;
    if expected_tip.is_none() {
        return Ok(identity.rows.is_empty() && account_ids.is_empty());
    }
    if account_ids.len() != 1 {
        return Ok(false);
    }
    let account = wallet
        .get_account(account_ids[0])
        .map_err(|_| ZecError::state_corrupt())?
        .ok_or_else(ZecError::state_corrupt)?;
    if account.purpose() != AccountPurpose::ViewOnly
        || identity.blocks.first().map(|block| block.height)
            != Some(u64::from(u32::from(account.birthday_height())))
    {
        return Ok(false);
    }
    for block in &identity.blocks {
        let height = BlockHeight::from_u32(
            u32::try_from(block.height).map_err(|_| ZecError::state_corrupt())?,
        );
        let hash = BlockHash::try_from_slice(&block.hash).ok_or_else(ZecError::state_corrupt)?;
        let chain = block
            .chain_metadata
            .as_ref()
            .ok_or_else(ZecError::state_corrupt)?;
        let metadata = wallet
            .block_metadata(height)
            .map_err(|_| ZecError::state_corrupt())?
            .ok_or_else(ZecError::state_corrupt)?;
        if metadata.block_height() != height
            || metadata.block_hash() != hash
            || metadata.sapling_tree_size() != Some(chain.sapling_commitment_tree_size)
            || metadata.orchard_tree_size() != Some(chain.orchard_commitment_tree_size)
            || metadata.ironwood_tree_size() != Some(chain.ironwood_commitment_tree_size)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn validate_candidate_file(root: &StateRoot, path: &Path) -> Result<(), ZecError> {
    let root_metadata = fs::symlink_metadata(root.path()).map_err(|_| ZecError::state_corrupt())?;
    let metadata = fs::symlink_metadata(path).map_err(|_| ZecError::state_corrupt())?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_file()
        || metadata.permissions().mode() & 0o777 != 0o600
        || metadata.uid() != root_metadata.uid()
    {
        return Err(ZecError::state_corrupt());
    }
    Ok(())
}

fn refuse_sidecars(path: &Path) -> Result<(), ZecError> {
    for suffix in ["-wal", "-shm", "-journal"] {
        let mut name = path.as_os_str().to_os_string();
        name.push(suffix);
        match fs::symlink_metadata(PathBuf::from(name)) {
            Ok(_) => return Err(ZecError::state_corrupt()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(ZecError::state_corrupt()),
        }
    }
    Ok(())
}

fn remove_candidate(path: &Path) -> Result<(), ZecError> {
    validate_fixed_candidate_name(path)?;
    fs::remove_file(path).map_err(|_| ZecError::state_corrupt())?;
    let directory = path.parent().ok_or_else(ZecError::state_corrupt)?;
    sync_directory(directory)
}

fn promote_candidate(candidate: &Path, target: &Path, directory: &Path) {
    let _ = promote_candidate_checked(candidate, target, directory);
}

fn promote_candidate_checked(
    candidate: &Path,
    target: &Path,
    directory: &Path,
) -> Result<(), ZecError> {
    validate_fixed_candidate_name(candidate)?;
    if candidate.parent() != Some(directory) || target.parent() != Some(directory) {
        return Err(ZecError::state_corrupt());
    }
    fs::rename(candidate, target).map_err(|_| ZecError::state_corrupt())?;
    sync_directory(directory)
}

fn validate_fixed_candidate_name(path: &Path) -> Result<(), ZecError> {
    if path.file_name().and_then(|name| name.to_str()) == Some(CANDIDATE_FILE) {
        Ok(())
    } else {
        Err(ZecError::state_corrupt())
    }
}

fn sync_directory(path: &Path) -> Result<(), ZecError> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| ZecError::state_corrupt())
}

fn logical_abort() -> SqliteClientError {
    SqliteClientError::CorruptedData("scan transaction aborted".to_owned())
}

fn zero_balances() -> ScanBalances {
    ScanBalances {
        transparent_zat: "0".to_owned(),
        sapling_zat: "0".to_owned(),
        orchard_migration_required_zat: "0".to_owned(),
        ironwood_pending_zat: "0".to_owned(),
        ironwood_spendable_zat: "0".to_owned(),
        total_zat: "0".to_owned(),
    }
}

fn test_override_or_zero(value: Option<u64>) -> ScanBalances {
    match value {
        Some(value) => ScanBalances {
            total_zat: value.to_string(),
            ..zero_balances()
        },
        None => zero_balances(),
    }
}

fn official_tree_digest<P: Parameters + Send + 'static>(
    wallet: &mut WalletDb<&mut Connection, P, SystemClock, OsRng>,
) -> Result<String, ZecError> {
    let sapling = wallet
        .with_sapling_tree_mut(|tree| tree.root_at_checkpoint_depth(None))
        .map_err(|_| ZecError::state_corrupt())?
        .ok_or_else(ZecError::state_corrupt)?;
    let orchard = wallet
        .with_orchard_tree_mut(|tree| tree.root_at_checkpoint_depth(None))
        .map_err(|_| ZecError::state_corrupt())?
        .ok_or_else(ZecError::state_corrupt)?;
    let ironwood = wallet
        .with_ironwood_tree_mut(|tree| tree.root_at_checkpoint_depth(None))
        .map_err(|_| ZecError::state_corrupt())?
        .ok_or_else(ZecError::state_corrupt)?
        .ok_or_else(ZecError::state_corrupt)?;

    let mut hasher = Sha256::new();
    hasher.update(b"BitBook official wallet checkpoint roots v1");
    hasher.update([7]);
    hasher.update(b"sapling");
    hasher.update(sapling.to_bytes());
    hasher.update([7]);
    hasher.update(b"orchard");
    hasher.update(orchard.to_bytes());
    hasher.update([8]);
    hasher.update(b"ironwood");
    hasher.update(ironwood.to_bytes());
    Ok(hex(&hasher.finalize()))
}

fn empty_tree_digest() -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"BitBook official wallet checkpoint roots v1");
    hasher.update([0]);
    hasher.update(b"empty-wallet-no-official-trees");
    hex(&hasher.finalize())
}

fn pool_classification(
    transparent: u64,
    sapling: u64,
    orchard: u64,
    ironwood_pending: u64,
    ironwood_spendable: u64,
) -> String {
    let mut result = String::new();
    for (name, present) in [
        ("transparent", transparent != 0),
        ("sapling", sapling != 0),
        ("orchard-migration-required", orchard != 0),
        ("ironwood-pending", ironwood_pending != 0),
        ("ironwood-spendable", ironwood_spendable != 0),
    ] {
        if present {
            if !result.is_empty() {
                result.push('|');
            }
            result.push_str(name);
        }
    }
    if result.is_empty() {
        "empty".to_owned()
    } else {
        result
    }
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}
