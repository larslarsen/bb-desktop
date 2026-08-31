//! Test-only, upstream-only oracle for the frozen BBD-WAL-006 compact-block fixture.
//!
//! This target deliberately imports no BitBook ZEC adapter. It is independently invocable and
//! may write only to the fixed Cargo target subdirectory named below. Phase A authors this source;
//! a separately authorized Phase B invocation reviews and freezes its output.

use std::{
    cell::RefCell,
    fs::{self, DirBuilder, OpenOptions},
    io::Write,
    os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    rc::Rc,
};

use serde::Serialize;
use sha2::{Digest, Sha256};
use zcash_client_backend::{
    data_api::testing::{AddressType, FakeCompactOutput, IronwoodFvk, TestBuilder, TestCache},
    proto::compact_formats::{
        ChainMetadata, CompactBlock, CompactOrchardAction, CompactSaplingOutput,
        CompactSaplingSpend, CompactTx, CompactTxIn, TxOut,
    },
};
use zcash_client_sqlite::testing::{BlockCache, db::TestDbFactory};
use zcash_keys::keys::UnifiedAddressRequest;
use zcash_primitives::block::BlockHash;
use zcash_protocol::{consensus::BlockHeight, local_consensus::LocalNetwork, value::Zatoshis};

const FORMAT: &str = "bitbook-zec-compact-fixture";
const FORMAT_VERSION: u32 = 1;
const BIRTHDAY_HEIGHT: u32 = 100;
const CHECKPOINT_HEIGHT: u32 = 99;
const NU6_3_HEIGHT: u32 = 102;
const CONFIRMATION_HEIGHT: u32 = 106;
const ORCHARD_VALUE_ZAT: u64 = 40_000_000;
const IRONWOOD_VALUE_ZAT: u64 = 150_000_000;
const REORG_VICTIM_IRONWOOD_VALUE_ZAT: u64 = 30_000_000;
const REORG_REPLACEMENT_IRONWOOD_VALUE_ZAT: u64 = 120_000_000;
const PREVIOUS_HASH: [u8; 32] = [0x42; 32];

struct RecordingCache {
    inner: BlockCache,
    recorded: Rc<RefCell<Vec<CompactBlock>>>,
}

impl RecordingCache {
    fn new(recorded: Rc<RefCell<Vec<CompactBlock>>>) -> Self {
        Self {
            inner: BlockCache::new(),
            recorded,
        }
    }
}

impl TestCache for RecordingCache {
    type BsError = <BlockCache as TestCache>::BsError;
    type BlockSource = <BlockCache as TestCache>::BlockSource;
    type InsertResult = <BlockCache as TestCache>::InsertResult;

    fn block_source(&self) -> &Self::BlockSource {
        self.inner.block_source()
    }

    fn insert(&mut self, block: &CompactBlock) -> Self::InsertResult {
        if self
            .recorded
            .borrow()
            .iter()
            .any(|recorded| recorded.height == block.height)
        {
            let duplicate_height =
                u32::try_from(block.height).expect("duplicate compact-block height exceeded u32");
            let preceding_height = duplicate_height
                .checked_sub(1)
                .expect("duplicate compact-block height had no predecessor");
            self.inner
                .truncate_to_height(BlockHeight::from_u32(preceding_height));
        }
        self.recorded.borrow_mut().push(block.clone());
        self.inner.insert(block)
    }

    fn truncate_to_height(&mut self, height: BlockHeight) {
        self.recorded
            .borrow_mut()
            .retain(|block| block.height <= u64::from(u32::from(height)));
        self.inner.truncate_to_height(height);
    }
}

#[derive(Debug, Serialize)]
struct FixtureManifest {
    format: &'static str,
    version: u32,
    generator: GeneratorCompatibility,
    network: NetworkManifest,
    expected: ExpectedPublicValues,
    files: Vec<FileManifest>,
    scenarios: ScenarioManifest,
}

#[derive(Debug, Serialize)]
struct GeneratorCompatibility {
    zcash_client_backend: &'static str,
    zcash_client_sqlite: &'static str,
    pczt: &'static str,
    zcash_primitives: &'static str,
    zcash_protocol: &'static str,
    zcash_keys: &'static str,
}

#[derive(Debug, Serialize)]
struct NetworkManifest {
    discriminator: &'static str,
    birthday_height: u32,
    checkpoint_height: u32,
    overwinter: u32,
    sapling: u32,
    blossom: u32,
    heartwood: u32,
    canopy: u32,
    nu5: u32,
    nu6: u32,
    nu6_1: u32,
    nu6_2: u32,
    nu6_3: u32,
}

#[derive(Debug, Serialize)]
struct ExpectedPublicValues {
    orchard_only_receiver: String,
    orchard_migration_required_zat: u64,
    ironwood_spendable_zat: u64,
    reorg_victim_ironwood_pending_zat: u64,
    reorg_replacement_ironwood_pending_zat: u64,
    confirmation_height: u32,
    nu6_3_branch_id_hex: &'static str,
    prepared_transaction_version: u32,
}

#[derive(Debug, Serialize)]
struct FileManifest {
    name: String,
    byte_length: u64,
    sha256: String,
    block_height: Option<u64>,
    block_hash: Option<String>,
    previous_hash: Option<String>,
    scenario_labels: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ScenarioManifest {
    canonical: Vec<String>,
    replay: Vec<String>,
    discontinuity: String,
    height_gap: String,
    one_block_reorg: String,
    truncation: String,
    malformed: String,
    corruption: String,
    impossible_tree_state: String,
}

#[derive(Debug)]
struct GeneratedFixture {
    manifest_bytes: Vec<u8>,
    files: Vec<(String, Vec<u8>)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AncestorKind {
    Directory,
    Symlink,
    NonDirectory,
}

fn local_network() -> LocalNetwork {
    LocalNetwork {
        overwinter: Some(BlockHeight::from_u32(1)),
        sapling: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        blossom: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        heartwood: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        canopy: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        nu5: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        nu6: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        nu6_1: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        nu6_2: Some(BlockHeight::from_u32(BIRTHDAY_HEIGHT)),
        nu6_3: Some(BlockHeight::from_u32(NU6_3_HEIGHT)),
    }
}

fn generated_fixture() -> Result<GeneratedFixture, String> {
    let recorded = Rc::new(RefCell::new(Vec::new()));
    let cache = RecordingCache::new(Rc::clone(&recorded));
    let network = local_network();
    let mut state = TestBuilder::new()
        .with_network(network)
        .with_block_cache(cache)
        .with_data_store_factory(TestDbFactory::default())
        .with_account_from_sapling_activation(BlockHash(PREVIOUS_HASH))
        .build();

    // TestBuilder's reviewed upstream implementation derives this account from the fixed
    // synthetic seed [0u8; 32]. No mnemonic, user secret, or mainnet key is involved.
    let account_ufvk = state
        .test_account()
        .ok_or("upstream test builder omitted the configured account")?
        .usk()
        .to_unified_full_viewing_key();
    let account_orchard = account_ufvk
        .orchard()
        .ok_or("fixed account omitted its Orchard full viewing key")?
        .clone();
    let (receiver, _) = account_ufvk
        .default_address(UnifiedAddressRequest::ORCHARD)
        .map_err(|error| format!("upstream Orchard address derivation failed: {error:?}"))?;
    let receiver = receiver.encode(&network);

    // A sibling account provides an independently derived shielded output that the fixture wallet
    // must ignore. It is created before scanning/generation, as required by the upstream harness.
    let (_, unrelated_usk) = state.create_account_from_test_seed("unrelated-fixture-account");
    let unrelated_orchard = unrelated_usk
        .to_unified_full_viewing_key()
        .orchard()
        .ok_or("sibling account omitted its Orchard full viewing key")?
        .clone();

    let orchard_value = Zatoshis::from_u64(ORCHARD_VALUE_ZAT)
        .map_err(|error| format!("fixed Orchard amount is invalid: {error:?}"))?;
    let ironwood_value = Zatoshis::from_u64(IRONWOOD_VALUE_ZAT)
        .map_err(|error| format!("fixed Ironwood amount is invalid: {error:?}"))?;
    let reorg_victim_value = Zatoshis::from_u64(REORG_VICTIM_IRONWOOD_VALUE_ZAT)
        .map_err(|error| format!("fixed reorg victim amount is invalid: {error:?}"))?;
    let reorg_replacement_value = Zatoshis::from_u64(REORG_REPLACEMENT_IRONWOOD_VALUE_ZAT)
        .map_err(|error| format!("fixed reorg amount is invalid: {error:?}"))?;

    let (height, _, _) = state.generate_next_block(
        &account_orchard,
        AddressType::DefaultExternal,
        orchard_value,
    );
    require_height(height, 100)?;
    require_height(state.generate_empty_block().0, 101)?;
    require_height(state.generate_empty_block().0, 102)?;
    let (height, _, _) = state.generate_next_block(
        &unrelated_orchard,
        AddressType::DefaultExternal,
        orchard_value,
    );
    require_height(height, 103)?;
    let (height, _, _) = state.generate_next_block(
        &IronwoodFvk(account_orchard.clone()),
        AddressType::DefaultExternal,
        ironwood_value,
    );
    require_height(height, 104)?;
    require_height(state.generate_empty_block().0, 105)?;
    require_height(state.generate_empty_block().0, CONFIRMATION_HEIGHT)?;
    let (height, _, _) = state.generate_next_block(
        &IronwoodFvk(account_orchard.clone()),
        AddressType::DefaultExternal,
        reorg_victim_value,
    );
    require_height(height, 107)?;

    let canonical = recorded.borrow().clone();
    if canonical.len() != 8 {
        return Err(format!(
            "upstream harness generated {} canonical blocks, expected 8",
            canonical.len()
        ));
    }

    let replacement_parent = canonical
        .iter()
        .find(|block| block.height == u64::from(CONFIRMATION_HEIGHT))
        .ok_or("upstream harness omitted the reorg replacement parent")?;
    let replacement_parent_metadata = replacement_parent
        .chain_metadata
        .as_ref()
        .ok_or("upstream harness omitted the reorg replacement parent metadata")?;
    let replacement_parent_hash = BlockHash::try_from_slice(&replacement_parent.hash)
        .ok_or("upstream harness produced an invalid reorg replacement parent hash")?;
    let replacement_output = FakeCompactOutput::new(
        IronwoodFvk(account_orchard),
        AddressType::DefaultExternal,
        reorg_replacement_value,
    );
    state.generate_block_at(
        BlockHeight::from_u32(CONFIRMATION_HEIGHT + 1),
        replacement_parent_hash,
        &[replacement_output],
        replacement_parent_metadata.sapling_commitment_tree_size,
        replacement_parent_metadata.orchard_commitment_tree_size,
        replacement_parent_metadata.ironwood_commitment_tree_size,
        false,
    );
    let replacement = recorded
        .borrow()
        .last()
        .cloned()
        .ok_or("upstream harness omitted the reorg replacement block")?;
    if replacement.height != u64::from(CONFIRMATION_HEIGHT + 1) {
        return Err(format!(
            "upstream harness generated replacement height {}, expected {}",
            replacement.height,
            CONFIRMATION_HEIGHT + 1
        ));
    }

    assemble_fixture(receiver, canonical, replacement)
}

fn require_height(actual: BlockHeight, expected: u32) -> Result<(), String> {
    let actual = u32::from(actual);
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "upstream harness generated height {actual}, expected {expected}"
        ))
    }
}

fn assemble_fixture(
    receiver: String,
    canonical: Vec<CompactBlock>,
    replacement: CompactBlock,
) -> Result<GeneratedFixture, String> {
    let mut files = Vec::new();
    let mut entries = Vec::new();
    let mut canonical_names = Vec::new();

    for block in &canonical {
        let name = format!("blocks/canonical-{:06}.compact", block.height);
        canonical_names.push(name.clone());
        add_block_file(
            &mut files,
            &mut entries,
            name,
            block,
            vec![canonical_label(block.height)],
        );
    }

    let canonical_tip = canonical
        .last()
        .ok_or("canonical fixture unexpectedly contained no blocks")?;

    let reorg_name = "blocks/reorg-replacement-000107.compact".to_owned();
    add_block_file(
        &mut files,
        &mut entries,
        reorg_name.clone(),
        &replacement,
        vec!["one-block-reorg", "replacement-ironwood"],
    );

    let mut discontinuity = canonical_tip.clone();
    discontinuity.prev_hash = vec![0x99; 32];
    let discontinuity_name = "blocks/discontinuity-wrong-prev-000107.compact".to_owned();
    add_block_file(
        &mut files,
        &mut entries,
        discontinuity_name.clone(),
        &discontinuity,
        vec!["discontinuity", "wrong-previous-hash"],
    );

    let mut height_gap = canonical_tip.clone();
    height_gap.height = 109;
    let height_gap_name = "blocks/discontinuity-height-gap-000109.compact".to_owned();
    add_block_file(
        &mut files,
        &mut entries,
        height_gap_name.clone(),
        &height_gap,
        vec!["discontinuity", "height-gap"],
    );

    let mut impossible_tree = canonical_tip.clone();
    impossible_tree.chain_metadata = Some(ChainMetadata {
        sapling_commitment_tree_size: 0,
        orchard_commitment_tree_size: 0,
        ironwood_commitment_tree_size: 0,
    });
    let impossible_tree_name = "blocks/impossible-tree-state-000107.compact".to_owned();
    add_block_file(
        &mut files,
        &mut entries,
        impossible_tree_name.clone(),
        &impossible_tree,
        vec!["corruption", "impossible-tree-state"],
    );

    let canonical_tip_bytes = encode_compact_block(canonical_tip);
    let truncated_name = "blocks/truncated-000107.compact".to_owned();
    let truncated = canonical_tip_bytes
        .get(..canonical_tip_bytes.len().saturating_sub(1))
        .ok_or("canonical tip could not be truncated")?
        .to_vec();
    add_raw_file(
        &mut files,
        &mut entries,
        truncated_name.clone(),
        truncated,
        vec!["truncation"],
    );

    let malformed_name = "blocks/malformed.compact".to_owned();
    add_raw_file(
        &mut files,
        &mut entries,
        malformed_name.clone(),
        vec![0x80],
        vec!["malformed"],
    );

    let corruption_name = "blocks/corrupt-wire-type-000107.compact".to_owned();
    let mut corrupt = canonical_tip_bytes;
    if let Some(first) = corrupt.first_mut() {
        *first = 0x0f;
    } else {
        return Err("canonical tip encoded to no bytes".to_owned());
    }
    add_raw_file(
        &mut files,
        &mut entries,
        corruption_name.clone(),
        corrupt,
        vec!["corruption", "invalid-protobuf-wire-type"],
    );

    let manifest = FixtureManifest {
        format: FORMAT,
        version: FORMAT_VERSION,
        generator: GeneratorCompatibility {
            zcash_client_backend: "0.24.0",
            zcash_client_sqlite: "0.22.0",
            pczt: "0.9.3",
            zcash_primitives: "0.30.1",
            zcash_protocol: "0.10.5",
            zcash_keys: "0.16.1",
        },
        network: NetworkManifest {
            discriminator: "zec-local",
            birthday_height: BIRTHDAY_HEIGHT,
            checkpoint_height: CHECKPOINT_HEIGHT,
            overwinter: 1,
            sapling: BIRTHDAY_HEIGHT,
            blossom: BIRTHDAY_HEIGHT,
            heartwood: BIRTHDAY_HEIGHT,
            canopy: BIRTHDAY_HEIGHT,
            nu5: BIRTHDAY_HEIGHT,
            nu6: BIRTHDAY_HEIGHT,
            nu6_1: BIRTHDAY_HEIGHT,
            nu6_2: BIRTHDAY_HEIGHT,
            nu6_3: NU6_3_HEIGHT,
        },
        expected: ExpectedPublicValues {
            orchard_only_receiver: receiver,
            orchard_migration_required_zat: ORCHARD_VALUE_ZAT,
            ironwood_spendable_zat: IRONWOOD_VALUE_ZAT,
            reorg_victim_ironwood_pending_zat: REORG_VICTIM_IRONWOOD_VALUE_ZAT,
            reorg_replacement_ironwood_pending_zat: REORG_REPLACEMENT_IRONWOOD_VALUE_ZAT,
            confirmation_height: CONFIRMATION_HEIGHT,
            nu6_3_branch_id_hex: "37a5165b",
            prepared_transaction_version: 6,
        },
        files: entries,
        scenarios: ScenarioManifest {
            canonical: canonical_names.clone(),
            replay: canonical_names,
            discontinuity: discontinuity_name,
            height_gap: height_gap_name,
            one_block_reorg: reorg_name,
            truncation: truncated_name,
            malformed: malformed_name,
            corruption: corruption_name,
            impossible_tree_state: impossible_tree_name,
        },
    };
    let mut manifest_bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("fixture manifest serialization failed: {error}"))?;
    manifest_bytes.push(b'\n');

    Ok(GeneratedFixture {
        manifest_bytes,
        files,
    })
}

fn canonical_label(height: u64) -> &'static str {
    match height {
        100 => "pre-nu6.3-older-orchard",
        101 => "pre-nu6.3",
        102 => "nu6.3-activation",
        103 => "post-nu6.3-unrelated-output",
        104 => "post-nu6.3-incoming-ironwood-pending",
        105 => "post-nu6.3-confirmation-depth",
        106 => "post-nu6.3-incoming-ironwood-confirmed",
        107 => "one-block-reorg-recognized-victim",
        _ => "unexpected-height",
    }
}

fn add_block_file(
    files: &mut Vec<(String, Vec<u8>)>,
    entries: &mut Vec<FileManifest>,
    name: String,
    block: &CompactBlock,
    scenario_labels: Vec<&'static str>,
) {
    let bytes = encode_compact_block(block);
    entries.push(FileManifest {
        name: name.clone(),
        byte_length: bytes.len() as u64,
        sha256: sha256_hex(&bytes),
        block_height: Some(block.height),
        block_hash: Some(hex(&block.hash)),
        previous_hash: Some(hex(&block.prev_hash)),
        scenario_labels,
    });
    files.push((name, bytes));
}

fn add_raw_file(
    files: &mut Vec<(String, Vec<u8>)>,
    entries: &mut Vec<FileManifest>,
    name: String,
    bytes: Vec<u8>,
    scenario_labels: Vec<&'static str>,
) {
    entries.push(FileManifest {
        name: name.clone(),
        byte_length: bytes.len() as u64,
        sha256: sha256_hex(&bytes),
        block_height: None,
        block_hash: None,
        previous_hash: None,
        scenario_labels,
    });
    files.push((name, bytes));
}

fn required_output_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("wal006-fixture-build")
}

fn write_or_verify_fixture(output_dir: &Path) -> Result<(), String> {
    let required = required_output_dir();
    if output_dir != required {
        return Err("refusing fixture output outside fixed wal006 target".to_owned());
    }
    validate_existing_ancestors(output_dir)?;
    preflight_fixed_output(output_dir)?;

    let generated = generated_fixture()?;
    if output_dir.exists() {
        verify_existing_fixture(output_dir, &generated)
    } else {
        DirBuilder::new()
            .mode(0o700)
            .create(output_dir)
            .map_err(|error| format!("could not create fixed fixture directory: {error}"))?;
        DirBuilder::new()
            .mode(0o700)
            .create(output_dir.join("blocks"))
            .map_err(|error| format!("could not create fixture blocks directory: {error}"))?;
        for (name, bytes) in &generated.files {
            write_new_file(&output_dir.join(name), bytes)?;
        }
        write_new_file(&output_dir.join("manifest.json"), &generated.manifest_bytes)
    }
}

fn preflight_fixed_output(output_dir: &Path) -> Result<(), String> {
    match fs::symlink_metadata(output_dir) {
        Ok(_) => {
            require_secure_directory(output_dir)?;
            require_secure_directory(&output_dir.join("blocks"))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("could not inspect fixed fixture output: {error}")),
    }
}

fn validate_existing_ancestors(output_dir: &Path) -> Result<(), String> {
    let parent = output_dir
        .parent()
        .ok_or("fixed fixture output has no parent directory")?;
    let mut current = PathBuf::new();
    for component in parent.components() {
        current.push(component.as_os_str());
        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            format!(
                "could not inspect fixed fixture output ancestor {}: {error}",
                current.display()
            )
        })?;
        let kind = if metadata.file_type().is_symlink() {
            AncestorKind::Symlink
        } else if metadata.is_dir() {
            AncestorKind::Directory
        } else {
            AncestorKind::NonDirectory
        };
        require_directory_kind(kind)?;
    }
    Ok(())
}

fn require_directory_kind(kind: AncestorKind) -> Result<(), String> {
    match kind {
        AncestorKind::Directory => Ok(()),
        AncestorKind::Symlink => {
            Err("fixed fixture output ancestor must not be a symlink".to_owned())
        }
        AncestorKind::NonDirectory => {
            Err("fixed fixture output ancestor must be a directory".to_owned())
        }
    }
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|error| format!("could not create fixture file {}: {error}", path.display()))?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| {
            format!(
                "could not durably write fixture file {}: {error}",
                path.display()
            )
        })
}

fn verify_existing_fixture(output_dir: &Path, generated: &GeneratedFixture) -> Result<(), String> {
    require_secure_directory(output_dir)?;
    require_secure_directory(&output_dir.join("blocks"))?;

    let mut expected_root = vec!["blocks".to_owned(), "manifest.json".to_owned()];
    expected_root.sort();
    if directory_names(output_dir)? != expected_root {
        return Err("fixed fixture directory contains an unexpected entry".to_owned());
    }

    let mut expected_blocks = generated
        .files
        .iter()
        .map(|(name, _)| {
            Path::new(name)
                .file_name()
                .and_then(|value| value.to_str())
                .map(str::to_owned)
                .ok_or_else(|| format!("fixture generator produced an invalid name: {name}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    expected_blocks.sort();
    if directory_names(&output_dir.join("blocks"))? != expected_blocks {
        return Err("fixture blocks directory contains an unexpected entry".to_owned());
    }

    verify_file(&output_dir.join("manifest.json"), &generated.manifest_bytes)?;
    for (name, bytes) in &generated.files {
        verify_file(&output_dir.join(name), bytes)?;
    }
    Ok(())
}

fn require_secure_directory(path: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        format!(
            "could not inspect fixture directory {}: {error}",
            path.display()
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "fixture directory is not a real directory: {}",
            path.display()
        ));
    }
    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(format!(
            "fixture directory grants group/other access: {}",
            path.display()
        ));
    }
    Ok(())
}

fn directory_names(path: &Path) -> Result<Vec<String>, String> {
    let mut names = fs::read_dir(path)
        .map_err(|error| format!("could not enumerate fixture directory: {error}"))?
        .map(|entry| {
            entry
                .map_err(|error| format!("could not inspect fixture entry: {error}"))?
                .file_name()
                .into_string()
                .map_err(|_| "fixture entry name is not UTF-8".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    Ok(names)
}

fn verify_file(path: &Path, expected: &[u8]) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("could not inspect fixture file {}: {error}", path.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!(
            "fixture entry is not a regular file: {}",
            path.display()
        ));
    }
    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(format!(
            "fixture file grants group/other access: {}",
            path.display()
        ));
    }
    let actual = fs::read(path)
        .map_err(|error| format!("could not read fixture file {}: {error}", path.display()))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "existing fixture file differs from deterministic oracle: {}",
            path.display()
        ))
    }
}

// The fixed direct dependency set intentionally contains no standalone protobuf codec. These
// functions encode only the reviewed public compact-format structs using canonical protobuf wire
// rules. Every cryptographic field and compact note is still constructed by official upstream
// Zcash APIs above; this small encoder is deterministic packaging, not a wallet oracle.
fn encode_compact_block(block: &CompactBlock) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_u64(&mut encoded, 2, block.height);
    put_bytes(&mut encoded, 3, &block.hash);
    put_bytes(&mut encoded, 4, &block.prev_hash);
    put_u32(&mut encoded, 5, block.time);
    put_bytes(&mut encoded, 6, &block.header);
    for transaction in &block.vtx {
        put_message(&mut encoded, 7, &encode_compact_tx(transaction));
    }
    if let Some(metadata) = &block.chain_metadata {
        put_message(&mut encoded, 8, &encode_chain_metadata(metadata));
    }
    encoded
}

fn encode_chain_metadata(metadata: &ChainMetadata) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_u32(&mut encoded, 1, metadata.sapling_commitment_tree_size);
    put_u32(&mut encoded, 2, metadata.orchard_commitment_tree_size);
    put_u32(&mut encoded, 3, metadata.ironwood_commitment_tree_size);
    encoded
}

fn encode_compact_tx(transaction: &CompactTx) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_u64(&mut encoded, 1, transaction.index);
    put_bytes(&mut encoded, 2, &transaction.txid);
    put_u32(&mut encoded, 3, transaction.fee);
    for spend in &transaction.spends {
        put_message(&mut encoded, 4, &encode_sapling_spend(spend));
    }
    for output in &transaction.outputs {
        put_message(&mut encoded, 5, &encode_sapling_output(output));
    }
    for action in &transaction.actions {
        put_message(&mut encoded, 6, &encode_orchard_action(action));
    }
    for action in &transaction.ironwood_actions {
        put_message(&mut encoded, 9, &encode_orchard_action(action));
    }
    for input in &transaction.vin {
        put_message(&mut encoded, 7, &encode_transparent_input(input));
    }
    for output in &transaction.vout {
        put_message(&mut encoded, 8, &encode_transparent_output(output));
    }
    encoded
}

fn encode_sapling_spend(spend: &CompactSaplingSpend) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_bytes(&mut encoded, 1, &spend.nf);
    encoded
}

fn encode_sapling_output(output: &CompactSaplingOutput) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_bytes(&mut encoded, 1, &output.cmu);
    put_bytes(&mut encoded, 2, &output.ephemeral_key);
    put_bytes(&mut encoded, 3, &output.ciphertext);
    encoded
}

fn encode_orchard_action(action: &CompactOrchardAction) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_bytes(&mut encoded, 1, &action.nullifier);
    put_bytes(&mut encoded, 2, &action.cmx);
    put_bytes(&mut encoded, 3, &action.ephemeral_key);
    put_bytes(&mut encoded, 4, &action.ciphertext);
    encoded
}

fn encode_transparent_input(input: &CompactTxIn) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_bytes(&mut encoded, 1, &input.prevout_txid);
    put_u32(&mut encoded, 2, input.prevout_index);
    encoded
}

fn encode_transparent_output(output: &TxOut) -> Vec<u8> {
    let mut encoded = Vec::new();
    put_u64(&mut encoded, 1, output.value);
    put_bytes(&mut encoded, 2, &output.script_pub_key);
    encoded
}

fn put_u32(encoded: &mut Vec<u8>, tag: u64, value: u32) {
    put_u64(encoded, tag, u64::from(value));
}

fn put_u64(encoded: &mut Vec<u8>, tag: u64, value: u64) {
    if value != 0 {
        put_varint(encoded, tag << 3);
        put_varint(encoded, value);
    }
}

fn put_bytes(encoded: &mut Vec<u8>, tag: u64, value: &[u8]) {
    if !value.is_empty() {
        put_message(encoded, tag, value);
    }
}

fn put_message(encoded: &mut Vec<u8>, tag: u64, value: &[u8]) {
    put_varint(encoded, (tag << 3) | 2);
    put_varint(encoded, value.len() as u64);
    encoded.extend_from_slice(value);
}

fn put_varint(encoded: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        encoded.push((value as u8) | 0x80);
        value >>= 7;
    }
    encoded.push(value as u8);
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
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

#[test]
fn upstream_fixture_oracle_is_deterministic_and_closed() {
    let first = generated_fixture().expect("first upstream-only fixture construction succeeds");
    let second = generated_fixture().expect("repeat upstream-only fixture construction succeeds");

    assert_eq!(first.manifest_bytes, second.manifest_bytes);
    assert_eq!(first.files, second.files);
    assert_eq!(first.files.len(), 15);
    assert!(
        first
            .files
            .iter()
            .all(|(name, bytes)| name.starts_with("blocks/") && !bytes.is_empty())
    );
    let manifest_text = std::str::from_utf8(&first.manifest_bytes).unwrap();
    for forbidden in [
        "seed", "mnemonic", "mainnet", "endpoint", "http://", "https://",
    ] {
        assert!(!manifest_text.contains(forbidden));
    }
}

#[test]
fn fixture_builder_refuses_every_other_output_location() {
    let other = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("not-wal006-fixture-build");
    let error = write_or_verify_fixture(&other).expect_err("non-fixed output path must be refused");
    assert_eq!(error, "refusing fixture output outside fixed wal006 target");
    assert!(!other.exists());
}

#[test]
fn fixture_output_ancestor_policy_rejects_symlink_and_nondirectory_without_io() {
    assert_eq!(require_directory_kind(AncestorKind::Directory), Ok(()));
    assert_eq!(
        require_directory_kind(AncestorKind::Symlink).unwrap_err(),
        "fixed fixture output ancestor must not be a symlink"
    );
    assert_eq!(
        require_directory_kind(AncestorKind::NonDirectory).unwrap_err(),
        "fixed fixture output ancestor must be a directory"
    );
}

#[test]
fn fixture_builder_writes_or_verifies_only_the_fixed_target_path() {
    write_or_verify_fixture(&required_output_dir())
        .expect("fixed target fixture is created once or verified byte-for-byte");
}
