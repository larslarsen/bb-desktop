use core::fmt;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;
use zcash_keys::address::Address;
use zcash_protocol::consensus::Parameters;

use crate::vault::{SecretBytes, WipeEvent, WipeObserver};

use super::store::{AddressAccount, PreparedBuild};
use super::{
    AccountId, MAX_DIAGNOSTIC_BYTES, MAX_MEMO_BYTES, MAX_PREPARED_HANDLES, Network, ZecError,
};

const HANDLE_RETRIES: usize = 64;
const CONSTANT_MISS_SHAPE: &str = "zec-prepared-lookup-miss-v1";
const PREPARE_WIPE_LOG_CAPACITY: usize = MAX_PREPARED_HANDLES + 1;

#[derive(Clone, Eq, PartialEq)]
pub struct PrepareZecV1 {
    pub(crate) account_id: String,
    pub(crate) network: String,
    pub(crate) request_id: String,
    pub(crate) intent_hash: String,
    pub(crate) receiver: String,
    pub(crate) amount_zat: String,
    pub(crate) fee_bound_zat: String,
    pub(crate) memo: String,
    pub(crate) expires_at: String,
}

impl PrepareZecV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_id: &str,
        network: &str,
        request_id: &str,
        intent_hash: &str,
        receiver: &str,
        amount_zat: &str,
        fee_bound_zat: &str,
        memo: &str,
        expires_at: &str,
    ) -> Result<Self, ZecError> {
        let value = Self {
            account_id: account_id.to_owned(),
            network: network.to_owned(),
            request_id: request_id.to_owned(),
            intent_hash: intent_hash.to_owned(),
            receiver: receiver.to_owned(),
            amount_zat: amount_zat.to_owned(),
            fee_bound_zat: fee_bound_zat.to_owned(),
            memo: memo.to_owned(),
            expires_at: expires_at.to_owned(),
        };
        validate_closed_shape(&value)?;
        Ok(value)
    }
}

impl fmt::Debug for PrepareZecV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PrepareZecV1([REDACTED])")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct PreparedZecV1 {
    pub handle: String,
    pub account_id: String,
    pub network: String,
    pub request_id: String,
    pub intent_hash: String,
    pub receiver: String,
    pub amount_zat: String,
    pub fee_zat: String,
    pub fee_bound_zat: String,
    pub expires_at: String,
    pub tx_version: String,
    pub consensus_branch: String,
    pub spend_pool: String,
    pub output_pool: String,
    pub signed: bool,
    pub extractable: bool,
    pub broadcastable: bool,
}

impl PreparedZecV1 {
    pub fn field_names(&self) -> [&'static str; 17] {
        [
            "handle",
            "account_id",
            "network",
            "request_id",
            "intent_hash",
            "receiver",
            "amount_zat",
            "fee_zat",
            "fee_bound_zat",
            "expires_at",
            "tx_version",
            "consensus_branch",
            "spend_pool",
            "output_pool",
            "signed",
            "extractable",
            "broadcastable",
        ]
    }

    pub fn sanitized_json_for_test(&self) -> String {
        serde_json::json!({
            "handle": self.handle,
            "account_id": self.account_id,
            "network": self.network,
            "request_id": self.request_id,
            "intent_hash": self.intent_hash,
            "receiver": self.receiver,
            "amount_zat": self.amount_zat,
            "fee_zat": self.fee_zat,
            "fee_bound_zat": self.fee_bound_zat,
            "expires_at": self.expires_at,
            "tx_version": self.tx_version,
            "consensus_branch": self.consensus_branch,
            "spend_pool": self.spend_pool,
            "output_pool": self.output_pool,
            "signed": self.signed,
            "extractable": self.extractable,
            "broadcastable": self.broadcastable,
        })
        .to_string()
    }
}

impl fmt::Debug for PreparedZecV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedZecV1([REDACTED])")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct HandleBinding {
    pub(crate) account_id: String,
    pub(crate) session_id: String,
    pub(crate) request_id: String,
    pub(crate) intent_hash: String,
}

impl HandleBinding {
    pub fn new(
        account_id: &str,
        session_id: impl AsRef<str>,
        request_id: &str,
        intent_hash: &str,
    ) -> Result<Self, ZecError> {
        let session_id = session_id.as_ref();
        AccountId::parse(account_id)?;
        validate_lower_hex(session_id, 32)?;
        validate_lower_hex(request_id, 32)?;
        validate_lower_hex(intent_hash, 64)?;
        Ok(Self {
            account_id: account_id.to_owned(),
            session_id: session_id.to_owned(),
            request_id: request_id.to_owned(),
            intent_hash: intent_hash.to_owned(),
        })
    }
}

impl fmt::Debug for HandleBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("HandleBinding([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandleInvalidation {
    Lock,
    Timeout,
    Cancel,
    Expiry,
    AccountReplacement,
    DatabaseRollback,
    OperationError,
    PanicUnwind,
    BrokerExit,
}

impl HandleInvalidation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lock => "lock",
            Self::Timeout => "timeout",
            Self::Cancel => "cancel",
            Self::Expiry => "expiry",
            Self::AccountReplacement => "account-replacement",
            Self::DatabaseRollback => "database-rollback",
            Self::OperationError => "operation-error",
            Self::PanicUnwind => "panic-unwind",
            Self::BrokerExit => "broker-exit",
        }
    }

    fn destroys_derived(self) -> bool {
        matches!(
            self,
            Self::Lock
                | Self::AccountReplacement
                | Self::DatabaseRollback
                | Self::OperationError
                | Self::PanicUnwind
                | Self::BrokerExit
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PcztInspection {
    pub network: String,
    pub consensus_branch: u32,
    pub transaction_version: u32,
    pub destination: String,
    pub amount_zat: String,
    pub memo_sha256: String,
    pub fee_zat: String,
    pub ironwood_inputs: usize,
    pub ironwood_outputs: usize,
    pub has_transparent_bundle: bool,
    pub has_sapling_bundle: bool,
    pub has_orchard_output_bundle: bool,
    pub has_signatures: bool,
    pub has_proofs: bool,
    pub finalized: bool,
    pub extractable: bool,
    pub spend_pool: String,
    pub legacy_input_value_zat: String,
    pub intent_hash_binding: String,
    pub request_id_binding: String,
}

struct PreparedArtifact {
    raw: Option<SecretBytes>,
    binding: HandleBinding,
    expires_at: String,
    public: PreparedZecV1,
    inspection: Option<PcztInspection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WipeRecord {
    pub label: &'static str,
    pub length: usize,
    pub all_zero: bool,
    pub exit: String,
}

#[derive(Clone)]
pub(crate) struct PrepareWipeLog {
    pub records: Arc<Mutex<Vec<WipeRecord>>>,
}

impl PrepareWipeLog {
    pub(crate) fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

struct PrepareObserver {
    log: PrepareWipeLog,
    exit: String,
}

impl WipeObserver for PrepareObserver {
    fn observe(&mut self, event: WipeEvent) {
        let mut records = mutex_lock(&self.log.records);
        while records.len() >= PREPARE_WIPE_LOG_CAPACITY {
            records.remove(0);
        }
        records.push(WipeRecord {
            label: event.label,
            length: event.length,
            all_zero: event.all_zero,
            exit: self.exit.clone(),
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PoolInventoryData {
    pub transparent: u64,
    pub sapling: u64,
    pub orchard: u64,
    pub ironwood_spendable: u64,
    pub ironwood_unconfirmed: u64,
    pub ironwood_locked: u64,
}

impl PoolInventoryData {
    fn outcome(&self, required: u64) -> Result<(), ZecError> {
        if self.ironwood_spendable >= required {
            return Ok(());
        }
        if self.orchard >= required
            || self
                .orchard
                .checked_add(self.ironwood_spendable)
                .is_some_and(|value| value >= required)
        {
            return Err(ZecError::migration_required());
        }
        if self.transparent >= required || self.sapling >= required {
            return Err(ZecError::capability_missing());
        }
        let _ineligible_ironwood = self
            .ironwood_unconfirmed
            .checked_add(self.ironwood_locked)
            .ok_or_else(ZecError::limit)?;
        Err(ZecError::insufficient_funds())
    }
}

struct PrepareInner {
    session_id: Option<String>,
    derived: Option<SecretBytes>,
    handles: BTreeMap<String, PreparedArtifact>,
    inventory_override: Option<PoolInventoryData>,
    spend_accesses: usize,
    fee_rule_calls: usize,
    caller_fee_calls: usize,
    lookup_shape: String,
    lookup_returned_bytes: usize,
    canary_commitments: Vec<(String, usize, String)>,
    last_now: Option<String>,
    panic_after_access: bool,
    wipe_log: PrepareWipeLog,
}

pub(crate) struct PrepareState {
    inner: Mutex<PrepareInner>,
    viewing_only: bool,
}

impl PrepareState {
    pub(crate) fn new() -> Self {
        Self {
            inner: Mutex::new(PrepareInner {
                session_id: None,
                derived: None,
                handles: BTreeMap::new(),
                inventory_override: None,
                spend_accesses: 0,
                fee_rule_calls: 0,
                caller_fee_calls: 0,
                lookup_shape: CONSTANT_MISS_SHAPE.to_owned(),
                lookup_returned_bytes: 0,
                canary_commitments: Vec::new(),
                last_now: None,
                panic_after_access: false,
                wipe_log: PrepareWipeLog::new(),
            }),
            viewing_only: false,
        }
    }

    pub(crate) fn viewing_only() -> Self {
        let mut state = Self::new();
        state.viewing_only = true;
        state
    }

    pub(crate) fn unlock(&self, mut seed: SecretBytes) -> Result<(), ZecError> {
        if self.viewing_only {
            return Err(ZecError::watch_only());
        }
        let digest = seed.expose(|bytes| Sha256::digest(bytes).to_vec());
        let mut inner = mutex_lock(&self.inner);
        if inner.derived.is_some() || !inner.handles.is_empty() {
            invalidate_inner(&mut inner, HandleInvalidation::AccountReplacement);
        }
        let mut observer = PrepareObserver {
            log: inner.wipe_log.clone(),
            exit: "unlock".to_owned(),
        };
        seed.wipe_with("zec-unlock-seed", &mut observer);
        inner.derived = Some(SecretBytes::new(digest).map_err(|_| ZecError::internal())?);
        inner.session_id = Some(random_hex(16)?);
        Ok(())
    }

    pub(crate) fn session_id(&self) -> String {
        mutex_lock(&self.inner)
            .session_id
            .clone()
            .unwrap_or_default()
    }

    pub(crate) fn prepare(
        &self,
        account: &AddressAccount,
        input: PrepareZecV1,
        binding: Option<&HandleBinding>,
        now: &str,
    ) -> Result<PreparedZecV1, ZecError> {
        validate_for_account(account, &input, now)?;
        let amount = parse_canonical_positive_u64(&input.amount_zat)?;
        let fee_bound = parse_canonical_positive_u64(&input.fee_bound_zat)?;
        let mut inner = mutex_lock(&self.inner);
        let session = inner.session_id.clone().ok_or_else(|| {
            if self.viewing_only {
                ZecError::watch_only()
            } else {
                ZecError::locked()
            }
        })?;
        if let Some(binding) = binding
            && (binding.account_id != input.account_id
                || binding.account_id != account.account_id().as_str()
                || binding.session_id != session
                || binding.request_id != input.request_id
                || binding.intent_hash != input.intent_hash)
        {
            return Err(ZecError::locked());
        }
        if inner.derived.is_none() {
            return Err(ZecError::locked());
        }
        inner.last_now = Some(now.to_owned());
        if inner.handles.len() >= MAX_PREPARED_HANDLES {
            return Err(ZecError::limit());
        }
        inner.spend_accesses = inner.spend_accesses.saturating_add(1);
        inner.fee_rule_calls = inner.fee_rule_calls.saturating_add(1);
        let mut pending_raw = None;
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if core::mem::take(&mut inner.panic_after_access) {
                panic!("synthetic WAL-006 prepare panic");
            }
            let inventory = match &inner.inventory_override {
                Some(inventory) => inventory.clone(),
                None => {
                    let balances = account.inspect_scan()?.balances;
                    PoolInventoryData {
                        transparent: parse_inventory_value(&balances.transparent_zat)?,
                        sapling: parse_inventory_value(&balances.sapling_zat)?,
                        orchard: parse_inventory_value(&balances.orchard_migration_required_zat)?,
                        ironwood_spendable: parse_inventory_value(
                            &balances.ironwood_spendable_zat,
                        )?,
                        ironwood_unconfirmed: parse_inventory_value(
                            &balances.ironwood_pending_zat,
                        )?,
                        ironwood_locked: 0,
                    }
                }
            };
            let required = amount.checked_add(10_000).ok_or_else(ZecError::limit)?;
            inventory.outcome(required)?;

            let PreparedBuild {
                raw,
                fee_zat,
                inspection,
            } = account.build_prepared_pczt(
                &input.receiver,
                amount,
                &input.memo,
                &input.request_id,
                &input.intent_hash,
            )?;
            pending_raw = Some(raw);
            if fee_zat > fee_bound {
                return Err(ZecError::fee_bound());
            }
            let handle = unique_handle(&inner.handles)?;
            let prepared = public_value(&handle, &input, fee_zat);
            let artifact = PreparedArtifact {
                raw: None,
                binding: HandleBinding {
                    account_id: input.account_id.clone(),
                    session_id: session,
                    request_id: input.request_id.clone(),
                    intent_hash: input.intent_hash.clone(),
                },
                expires_at: input.expires_at.clone(),
                public: prepared.clone(),
                inspection: Some(inspection),
            };
            inner.handles.insert(handle.clone(), artifact);
            inner
                .handles
                .get_mut(&handle)
                .ok_or_else(ZecError::internal)?
                .raw = pending_raw.take();
            Ok(prepared)
        }));
        match outcome {
            Ok(Ok(prepared)) => Ok(prepared),
            Ok(Err(error)) => {
                wipe_pending_raw(
                    &mut inner,
                    &mut pending_raw,
                    HandleInvalidation::OperationError,
                );
                invalidate_inner(&mut inner, HandleInvalidation::OperationError);
                Err(error)
            }
            Err(payload) => {
                wipe_pending_raw(
                    &mut inner,
                    &mut pending_raw,
                    HandleInvalidation::PanicUnwind,
                );
                invalidate_inner(&mut inner, HandleInvalidation::PanicUnwind);
                std::panic::resume_unwind(payload)
            }
        }
    }

    pub(crate) fn arm_panic_after_access(&self) {
        mutex_lock(&self.inner).panic_after_access = true;
    }

    pub(crate) fn lookup(
        &self,
        handle: &str,
        binding: &HandleBinding,
    ) -> Result<PreparedZecV1, ZecError> {
        let mut inner = mutex_lock(&self.inner);
        inner.lookup_shape = CONSTANT_MISS_SHAPE.to_owned();
        inner.lookup_returned_bytes = 0;
        let Some(session) = &inner.session_id else {
            return Err(ZecError::locked());
        };
        let Some(artifact) = inner.handles.get(handle) else {
            return Err(ZecError::locked());
        };
        let expired = inner
            .last_now
            .as_deref()
            .and_then(|now| parse_timestamp(now).ok())
            .zip(parse_timestamp(&artifact.expires_at).ok())
            .is_some_and(|(now, expiry)| now >= expiry);
        if expired {
            invalidate_inner(&mut inner, HandleInvalidation::Expiry);
            return Err(ZecError::locked());
        }
        if &artifact.binding != binding || artifact.binding.session_id.as_str() != session.as_str()
        {
            return Err(ZecError::locked());
        }
        Ok(artifact.public.clone())
    }

    pub(crate) fn invalidate(&self, edge: HandleInvalidation) {
        invalidate_inner(&mut mutex_lock(&self.inner), edge);
    }

    pub(crate) fn attach_wipe_log(&self, log: PrepareWipeLog) {
        mutex_lock(&self.inner).wipe_log = log;
    }

    pub(crate) fn handle_count(&self) -> usize {
        mutex_lock(&self.inner).handles.len()
    }

    pub(crate) fn contains(&self, handle: &str) -> bool {
        mutex_lock(&self.inner).handles.contains_key(handle)
    }

    pub(crate) fn raw_len(&self, handle: &str) -> Result<usize, ZecError> {
        mutex_lock(&self.inner)
            .handles
            .get(handle)
            .and_then(|artifact| artifact.raw.as_ref())
            .map(SecretBytes::len)
            .ok_or_else(ZecError::locked)
    }

    pub(crate) fn derived_len(&self) -> usize {
        mutex_lock(&self.inner)
            .derived
            .as_ref()
            .map_or(0, SecretBytes::len)
    }

    pub(crate) fn inspection(&self, handle: &str) -> Result<PcztInspection, ZecError> {
        mutex_lock(&self.inner)
            .handles
            .get(handle)
            .and_then(|artifact| artifact.inspection.clone())
            .ok_or_else(ZecError::locked)
    }

    pub(crate) fn replace_inventory(&self, inventory: PoolInventoryData) {
        mutex_lock(&self.inner).inventory_override = Some(inventory);
    }

    pub(crate) fn fill_reserved(&self, count: usize) -> Result<(), ZecError> {
        if count > MAX_PREPARED_HANDLES {
            return Err(ZecError::limit());
        }
        let mut inner = mutex_lock(&self.inner);
        while inner.handles.len() < count {
            let handle = unique_handle(&inner.handles)?;
            inner.handles.insert(
                handle.clone(),
                PreparedArtifact {
                    raw: None,
                    binding: HandleBinding {
                        account_id: String::new(),
                        session_id: String::new(),
                        request_id: String::new(),
                        intent_hash: String::new(),
                    },
                    expires_at: String::new(),
                    public: empty_public(handle),
                    inspection: None,
                },
            );
        }
        Ok(())
    }

    pub(crate) fn reset_spend_access(&self) {
        mutex_lock(&self.inner).spend_accesses = 0;
    }

    pub(crate) fn spend_accesses(&self) -> usize {
        mutex_lock(&self.inner).spend_accesses
    }

    pub(crate) fn fee_rule_calls(&self) -> usize {
        mutex_lock(&self.inner).fee_rule_calls
    }

    pub(crate) fn caller_fee_calls(&self) -> usize {
        mutex_lock(&self.inner).caller_fee_calls
    }

    pub(crate) fn reset_lookup(&self) {
        let mut inner = mutex_lock(&self.inner);
        inner.lookup_shape = CONSTANT_MISS_SHAPE.to_owned();
        inner.lookup_returned_bytes = 0;
    }

    pub(crate) fn lookup_observation(&self) -> (String, usize) {
        let inner = mutex_lock(&self.inner);
        (inner.lookup_shape.clone(), inner.lookup_returned_bytes)
    }

    pub(crate) fn install_canary_commitments(&self, commitments: Vec<(String, usize, String)>) {
        mutex_lock(&self.inner).canary_commitments = commitments;
    }

    pub(crate) fn canary_commitment_count(&self) -> usize {
        mutex_lock(&self.inner).canary_commitments.len()
    }
}

impl Drop for PrepareState {
    fn drop(&mut self) {
        invalidate_inner(
            self.inner.get_mut().unwrap_or_else(|e| e.into_inner()),
            HandleInvalidation::BrokerExit,
        );
    }
}

fn public_value(handle: &str, input: &PrepareZecV1, fee_zat: u64) -> PreparedZecV1 {
    PreparedZecV1 {
        handle: handle.to_owned(),
        account_id: input.account_id.clone(),
        network: input.network.clone(),
        request_id: input.request_id.clone(),
        intent_hash: input.intent_hash.clone(),
        receiver: input.receiver.clone(),
        amount_zat: input.amount_zat.clone(),
        fee_zat: fee_zat.to_string(),
        fee_bound_zat: input.fee_bound_zat.clone(),
        expires_at: input.expires_at.clone(),
        tx_version: "6".to_owned(),
        consensus_branch: "37a5165b".to_owned(),
        spend_pool: "ironwood".to_owned(),
        output_pool: "ironwood".to_owned(),
        signed: false,
        extractable: false,
        broadcastable: false,
    }
}

fn empty_public(handle: String) -> PreparedZecV1 {
    PreparedZecV1 {
        handle,
        account_id: String::new(),
        network: String::new(),
        request_id: String::new(),
        intent_hash: String::new(),
        receiver: String::new(),
        amount_zat: String::new(),
        fee_zat: String::new(),
        fee_bound_zat: String::new(),
        expires_at: String::new(),
        tx_version: String::new(),
        consensus_branch: String::new(),
        spend_pool: String::new(),
        output_pool: String::new(),
        signed: false,
        extractable: false,
        broadcastable: false,
    }
}

fn wipe_pending_raw(
    inner: &mut PrepareInner,
    pending_raw: &mut Option<SecretBytes>,
    edge: HandleInvalidation,
) {
    if let Some(mut raw) = pending_raw.take() {
        let mut observer = PrepareObserver {
            log: inner.wipe_log.clone(),
            exit: edge.as_str().to_owned(),
        };
        raw.wipe_with("zec-prepared-pczt", &mut observer);
    }
}

fn invalidate_inner(inner: &mut PrepareInner, edge: HandleInvalidation) {
    let mut observer = PrepareObserver {
        log: inner.wipe_log.clone(),
        exit: edge.as_str().to_owned(),
    };
    for (_, mut artifact) in core::mem::take(&mut inner.handles) {
        if let Some(mut raw) = artifact.raw.take() {
            raw.wipe_with("zec-prepared-pczt", &mut observer);
        }
    }
    if edge.destroys_derived() {
        if let Some(mut derived) = inner.derived.take() {
            derived.wipe_with("zec-derived-spend", &mut observer);
        }
        inner.session_id = None;
    }
}

fn validate_closed_shape(input: &PrepareZecV1) -> Result<(), ZecError> {
    AccountId::parse(&input.account_id)?;
    if !matches!(input.network.as_str(), "zec-testnet" | "zec-local") {
        return if input.network == "zec-mainnet" {
            Err(ZecError::network_disabled())
        } else {
            Err(ZecError::schema())
        };
    }
    validate_lower_hex(&input.request_id, 32)?;
    validate_lower_hex(&input.intent_hash, 64)?;
    parse_canonical_positive_u64(&input.amount_zat)?;
    parse_canonical_positive_u64(&input.fee_bound_zat)?;
    validate_memo(&input.memo)?;
    parse_timestamp(&input.expires_at)?;
    Ok(())
}

fn validate_for_account(
    account: &AddressAccount,
    input: &PrepareZecV1,
    now: &str,
) -> Result<(), ZecError> {
    validate_closed_shape(input)?;
    if input.account_id != account.account_id().as_str() {
        return Err(ZecError::schema());
    }
    if input.network != account.network().as_str() {
        return Err(ZecError::schema());
    }
    match account.network() {
        Network::Testnet => validate_receiver_for(
            &zcash_protocol::consensus::Network::TestNetwork,
            &input.receiver,
        )?,
        Network::Local(local) => validate_receiver_for(&local.upstream(), &input.receiver)?,
    }
    let now = parse_timestamp(now)?;
    if now >= parse_timestamp(&input.expires_at)? {
        return Err(ZecError::expired());
    }
    Ok(())
}

fn validate_receiver_for<P: Parameters>(params: &P, encoded: &str) -> Result<(), ZecError> {
    let Address::Unified(address) =
        Address::decode(params, encoded).ok_or_else(ZecError::schema)?
    else {
        return Err(ZecError::schema());
    };
    if address.has_transparent() {
        return Err(ZecError::transparent_downgrade());
    }
    if !address.has_orchard() || address.has_sapling() || !address.unknown().is_empty() {
        return Err(ZecError::protocol_incompatible());
    }
    Ok(())
}

pub(crate) fn parse_canonical_positive_u64(value: &str) -> Result<u64, ZecError> {
    if value.is_empty()
        || value == "0"
        || value.starts_with('0')
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(ZecError::schema());
    }
    value.parse::<u64>().map_err(|_| ZecError::schema())
}

fn parse_inventory_value(value: &str) -> Result<u64, ZecError> {
    if value == "0" {
        Ok(0)
    } else {
        parse_canonical_positive_u64(value).map_err(|_| ZecError::state_corrupt())
    }
}

fn validate_lower_hex(value: &str, length: usize) -> Result<(), ZecError> {
    if value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(ZecError::schema())
    }
}

fn validate_memo(value: &str) -> Result<(), ZecError> {
    if value.len() > MAX_MEMO_BYTES || value.nfc().ne(value.chars()) {
        return Err(ZecError::schema());
    }
    if value.chars().any(forbidden_memo_char) {
        return Err(ZecError::schema());
    }
    Ok(())
}

fn forbidden_memo_char(value: char) -> bool {
    let code = u32::from(value);
    code <= 0x1f
        || (0x7f..=0x9f).contains(&code)
        || (0xfdd0..=0xfdef).contains(&code)
        || code & 0xffff == 0xfffe
        || code & 0xffff == 0xffff
        || (0x202a..=0x202e).contains(&code)
        || (0x2066..=0x206f).contains(&code)
        || (0x200b..=0x200f).contains(&code)
        || code == 0x061c
        || code == 0x2060
        || code == 0xfeff
        || (0xfff9..=0xfffb).contains(&code)
        || (0xe0001..=0xe007f).contains(&code)
}

fn parse_timestamp(value: &str) -> Result<u64, ZecError> {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
        || bytes.iter().enumerate().any(|(index, byte)| {
            !matches!(index, 4 | 7 | 10 | 13 | 16 | 19) && !byte.is_ascii_digit()
        })
    {
        return Err(ZecError::schema());
    }
    let number = |range: core::ops::Range<usize>| -> Result<u32, ZecError> {
        core::str::from_utf8(&bytes[range])
            .map_err(|_| ZecError::schema())?
            .parse::<u32>()
            .map_err(|_| ZecError::schema())
    };
    let year = number(0..4)?;
    let month = number(5..7)?;
    let day = number(8..10)?;
    let hour = number(11..13)?;
    let minute = number(14..16)?;
    let second = number(17..19)?;
    if !(2020..=2100).contains(&year)
        || !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return Err(ZecError::schema());
    }
    Ok(
        (((((u64::from(year) * 13 + u64::from(month)) * 32 + u64::from(day)) * 24
            + u64::from(hour))
            * 60
            + u64::from(minute))
            * 60)
            + u64::from(second),
    )
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
        2 => 28,
        _ => 31,
    }
}

fn random_hex(bytes: usize) -> Result<String, ZecError> {
    let mut random = vec![0u8; bytes];
    getrandom::fill(&mut random).map_err(|_| ZecError::internal())?;
    Ok(hex(&random))
}

fn unique_handle(handles: &BTreeMap<String, PreparedArtifact>) -> Result<String, ZecError> {
    for _ in 0..HANDLE_RETRIES {
        let handle = random_hex(16)?;
        if !handles.contains_key(&handle) {
            return Ok(handle);
        }
    }
    Err(ZecError::internal())
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

pub(crate) fn normalize_diagnostic(value: &str) -> Result<&'static str, ZecError> {
    if value.len() > MAX_DIAGNOSTIC_BYTES {
        Err(ZecError::limit())
    } else {
        Ok("[REDACTED]")
    }
}

fn mutex_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|error| error.into_inner())
}
