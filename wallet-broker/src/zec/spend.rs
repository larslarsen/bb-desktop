use std::collections::BTreeSet;
use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex, MutexGuard};

use orchard::keys::FullViewingKey;
use pczt::roles::prover::Prover;
use pczt::roles::signer::{Signer, SpendAuthSignature, extract_orchard_spend_auth_signatures};
use pczt::roles::spend_finalizer::SpendFinalizer;
use pczt::roles::tx_extractor::TransactionExtractor;
use pczt::roles::verifier::Verifier;
use zcash_client_backend::data_api::wallet::{SignerView, redact_pczt_for_signer};
use zcash_keys::keys::UnifiedSpendingKey;
use zcash_primitives::transaction::sighash::{SignableInput, signature_hash};
use zcash_primitives::transaction::txid::TxIdDigester;
use zcash_primitives::transaction::{
    Authorization, Authorized, Transaction, TransactionData, TxVersion,
};
use zcash_protocol::consensus::{BranchId, Parameters};
use zcash_protocol::value::Zatoshis;
use zeroize::{Zeroize, Zeroizing};

use crate::vault::{SecretBytes, WipeEvent, WipeObserver};

use super::hardware::PRODUCTION_REVIEWED_PROFILES;
use super::prepare::{
    PcztInspection, PreparedReview, PreparedSigningArtifact, PreparedZecV1, sha256_hex,
};
use super::{Network, ZecError};

mod effects;
pub(crate) use effects::TrustedVerificationAuthority;

pub(crate) const SIGN_VERIFY_OPERATIONS: [&str; 6] = [
    "account.bootstrap",
    "receiver.fresh",
    "fixture.scan",
    "pczt.prepare",
    "intent.confirm.native",
    "pczt.sign_verify",
];

const FORBIDDEN_OPERATIONS: [&str; 9] = [
    "tx.broadcast",
    "intent.broadcast",
    "network.submit",
    "http.request",
    "grpc.submit",
    "xmr.sign",
    "mainnet.enable",
    "hardware.enumerate",
    "hardware.transport",
];

#[derive(Clone, Default)]
pub(crate) struct PipelineCalls {
    pub(crate) signer_calls: usize,
    pub(crate) prover_calls: usize,
    pub(crate) finalizer_calls: usize,
    pub(crate) extractor_calls: usize,
    pub(crate) independent_decoder_calls: usize,
    pub(crate) verifier_calls: usize,
    pub(crate) authoritative_pczt_accesses: usize,
    pub(crate) external_contributions_applied: usize,
    pub(crate) external_signatures_verified: usize,
    triggered_fault: Option<PipelineFault>,
}

impl PipelineCalls {
    fn record_triggered_fault(&mut self, fault: PipelineFault) {
        self.triggered_fault = Some(fault);
    }

    pub(crate) fn triggered_fault(&self) -> Option<PipelineFault> {
        self.triggered_fault
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PipelineFault {
    Signer,
    Prover,
    Finalizer,
    Extractor,
    Verifier,
    Cleanup,
    PanicAfterVerification,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VerifyMutation {
    Receiver,
    Amount,
    Network,
    Fee,
    FeeBound,
    Memo,
    RequestId,
    IntentHash,
    Pool,
    Version,
    ConsensusBranch,
    Change,
    ExtractedTransactionIdBinding,
    Proof,
    Signature,
    MalformedState,
    MalformedSchema,
}

pub(crate) struct VerifiedEffects {
    pub(crate) network: String,
    pub(crate) mainnet: bool,
    pub(crate) transaction_version: u32,
    pub(crate) consensus_branch: u32,
    pub(crate) external_receiver_bytes: Vec<u8>,
    pub(crate) external_receiver: String,
    pub(crate) external_amount_zat: String,
    pub(crate) fee_zat: String,
    pub(crate) fee_bound_zat: String,
    pub(crate) fee_zat_u64: u64,
    pub(crate) fee_bound_zat_u64: u64,
    pub(crate) memo_sha256: String,
    pub(crate) request_id_binding: String,
    pub(crate) intent_hash_binding: String,
    pub(crate) ironwood_real_spends: usize,
    pub(crate) ironwood_external_outputs: usize,
    pub(crate) ironwood_internal_change_outputs: usize,
    pub(crate) transparent_effects: usize,
    pub(crate) sapling_effects: usize,
    pub(crate) orchard_effects: usize,
    pub(crate) proof_present: bool,
    pub(crate) proof_valid: bool,
    pub(crate) spend_authorization_present: bool,
    pub(crate) spend_authorization_valid: bool,
    pub(crate) binding_signature_present: bool,
    pub(crate) binding_signature_valid: bool,
    pub(crate) derived_transaction_id: String,
}

pub(crate) struct SignerViewArtifact {
    pub(crate) pczt: pczt::Pczt,
    pub(crate) randomized_key: [u8; 32],
    pub(crate) action_index: usize,
}

#[derive(Clone, Copy)]
struct RetainedSpendBinding {
    action_index: usize,
    randomized_key: [u8; 32],
    nullifier: [u8; 32],
    value_zat: u64,
}

enum SpendValueInspectionError {
    ActionCount,
    MissingValue,
}

#[derive(Clone)]
pub(crate) struct TaggedContribution {
    pub(crate) signature: SpendAuthSignature,
    pub(crate) randomized_key: [u8; 32],
}

pub(crate) struct AccountAuthorizationGate {
    held: Arc<Mutex<BTreeSet<String>>>,
}

pub(crate) struct AccountAuthorizationLease {
    held: Arc<Mutex<BTreeSet<String>>>,
    account_id: String,
}

impl AccountAuthorizationGate {
    pub(crate) fn new() -> Self {
        Self {
            held: Arc::new(Mutex::new(BTreeSet::new())),
        }
    }

    pub(crate) fn acquire(&self, account_id: &str) -> Result<AccountAuthorizationLease, ZecError> {
        if !mutex_lock(&self.held).insert(account_id.to_owned()) {
            return Err(ZecError::account_busy());
        }
        Ok(AccountAuthorizationLease {
            held: self.held.clone(),
            account_id: account_id.to_owned(),
        })
    }

    pub(crate) fn lock_count(&self, account_id: &str) -> usize {
        usize::from(mutex_lock(&self.held).contains(account_id))
    }
}

impl Drop for AccountAuthorizationLease {
    fn drop(&mut self) {
        mutex_lock(&self.held).remove(&self.account_id);
    }
}

pub(crate) fn invoke_operation(operation: &str) -> Result<(), ZecError> {
    if SIGN_VERIFY_OPERATIONS.contains(&operation) {
        return Ok(());
    }
    if FORBIDDEN_OPERATIONS.contains(&operation) {
        return Err(ZecError::capability_missing());
    }
    Err(ZecError::capability_missing())
}

pub(crate) fn production_hardware_denied() -> Result<(), ZecError> {
    let _ = PRODUCTION_REVIEWED_PROFILES.len();
    Err(ZecError::capability_missing())
}

pub(crate) fn build_signer_view(
    raw: &SecretBytes,
    calls: &mut PipelineCalls,
) -> Result<SignerViewArtifact, ZecError> {
    calls.authoritative_pczt_accesses = calls.authoritative_pczt_accesses.saturating_add(1);
    let pczt = raw
        .expose(pczt::Pczt::parse)
        .map_err(|_| ZecError::state_corrupt())?;
    validate_unsigned_shape(&pczt)?;
    let unsigned = unsigned_ironwood_action(&pczt)?;
    let randomized_key = *pczt.ironwood().actions()[unsigned].spend().rk();
    Ok(SignerViewArtifact {
        pczt: redact_pczt_for_signer(&pczt, SignerView::Compact),
        randomized_key,
        action_index: unsigned,
    })
}

pub(crate) fn synthetic_sign_view(
    view: &SignerViewArtifact,
    network: Network,
) -> Result<TaggedContribution, ZecError> {
    match network {
        Network::Testnet => {
            synthetic_sign_view_for(view, &zcash_protocol::consensus::Network::TestNetwork)
        }
        Network::Local(local) => synthetic_sign_view_for(view, &local.upstream()),
    }
}

fn synthetic_sign_view_for<P: Parameters>(
    view: &SignerViewArtifact,
    params: &P,
) -> Result<TaggedContribution, ZecError> {
    let index = unsigned_ironwood_action(&view.pczt)?;
    if index != view.action_index
        || view.pczt.ironwood().actions()[index].spend().rk() != &view.randomized_key
    {
        return Err(ZecError::signature_invalid());
    }
    let signed = {
        let usk = UnifiedSpendingKey::from_seed(params, &[0; 32], Default::default())
            .map_err(|_| ZecError::internal())?;
        let mut signer =
            Signer::new(view.pczt.clone()).map_err(|_| ZecError::signature_invalid())?;
        signer
            .sign_ironwood(index, &usk.orchard().into())
            .map_err(|_| ZecError::signature_invalid())?;
        signer.finish()
    };
    let mut signatures = extract_orchard_spend_auth_signatures(&signed)
        .into_iter()
        .filter(|signature| signature.action_index() == index);
    let signature = signatures.next().ok_or_else(ZecError::signature_invalid)?;
    if signatures.next().is_some() {
        return Err(ZecError::signature_invalid());
    }
    Ok(TaggedContribution {
        signature,
        randomized_key: view.randomized_key,
    })
}

struct ObservedSigningArtifact<'a> {
    raw: SecretBytes,
    public: PreparedZecV1,
    inspection: PcztInspection,
    observer: &'a mut dyn WipeObserver,
}

impl<'a> ObservedSigningArtifact<'a> {
    fn from_prepared(
        artifact: PreparedSigningArtifact,
        observer: &'a mut dyn WipeObserver,
    ) -> Self {
        Self {
            raw: artifact.raw,
            public: artifact.public,
            inspection: artifact.inspection,
            observer,
        }
    }
}

impl Drop for ObservedSigningArtifact<'_> {
    fn drop(&mut self) {
        self.raw.wipe_with("zec-authoritative-pczt", self.observer);
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "reviewed authority/verification/fault inputs deliberately remain explicit"
)]
pub(crate) fn authorize_software(
    artifact: PreparedSigningArtifact,
    seed: &SecretBytes,
    stored_ufvk: &str,
    network: Network,
    calls: &mut PipelineCalls,
    fault: Option<PipelineFault>,
    mutation: Option<VerifyMutation>,
    secrets: &mut dyn WipeObserver,
) -> Result<VerifiedEffects, ZecError> {
    let mut artifact = ObservedSigningArtifact::from_prepared(artifact, secrets);
    match network {
        Network::Testnet => authorize_software_for(
            &mut artifact,
            seed,
            stored_ufvk,
            network,
            &zcash_protocol::consensus::Network::TestNetwork,
            calls,
            fault,
            mutation,
        ),
        Network::Local(local) => authorize_software_for(
            &mut artifact,
            seed,
            stored_ufvk,
            network,
            &local.upstream(),
            calls,
            fault,
            mutation,
        ),
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "reviewed authority/verification/fault inputs deliberately remain explicit"
)]
fn authorize_software_for<P: Parameters>(
    artifact: &mut ObservedSigningArtifact<'_>,
    seed: &SecretBytes,
    stored_ufvk: &str,
    network: Network,
    params: &P,
    calls: &mut PipelineCalls,
    fault: Option<PipelineFault>,
    mutation: Option<VerifyMutation>,
) -> Result<VerifiedEffects, ZecError> {
    let (signed_pczt, authority) = {
        let (signed, pending, pre_sign_sighash) = {
            let usk = seed.expose(|bytes| {
                if bytes.len() != 32 {
                    return Err(ZecError::locked());
                }
                UnifiedSpendingKey::from_seed(params, bytes, Default::default())
                    .map_err(|_| ZecError::locked())
            })?;
            if usk.to_unified_full_viewing_key().encode(params) != stored_ufvk {
                return Err(ZecError::locked());
            }
            let fvk = FullViewingKey::from(usk.orchard());
            calls.authoritative_pczt_accesses = calls.authoritative_pczt_accesses.saturating_add(1);
            let pczt = artifact
                .raw
                .expose(pczt::Pczt::parse)
                .map_err(|_| ZecError::state_corrupt())?;
            validate_prepared(&pczt, &artifact.inspection)?;
            let (pczt, pending) = take_trusted_context(pczt, fvk, network)?;
            calls.signer_calls = calls.signer_calls.saturating_add(1);
            let mut signer = Signer::new(pczt).map_err(|_| ZecError::signature_invalid())?;
            let pre_sign_sighash = signer.shielded_sighash();
            let signed = signer
                .sign_ironwood(pending.action_index(), &usk.orchard().into())
                .map(|_| signer.finish())
                .map_err(|_| ZecError::signature_invalid())?;
            (signed, pending, pre_sign_sighash)
        };
        // Inner scope ends USK ownership; this is not a guaranteed upstream memory erasure.
        if fault == Some(PipelineFault::Signer) {
            calls.record_triggered_fault(PipelineFault::Signer);
            return Err(ZecError::internal());
        }
        (signed, pending.with_pre_sign_hash(pre_sign_sighash))
    };
    finish_pipeline(signed_pczt, artifact, authority, calls, fault, mutation)
}

pub(crate) fn authorize_external(
    artifact: PreparedSigningArtifact,
    contribution: TaggedContribution,
    stored_ufvk: &str,
    network: Network,
    calls: &mut PipelineCalls,
    fault: Option<PipelineFault>,
    secrets: &mut dyn WipeObserver,
) -> Result<VerifiedEffects, ZecError> {
    let mut artifact = ObservedSigningArtifact::from_prepared(artifact, secrets);
    calls.authoritative_pczt_accesses = calls.authoritative_pczt_accesses.saturating_add(1);
    let pczt = artifact
        .raw
        .expose(pczt::Pczt::parse)
        .map_err(|_| ZecError::state_corrupt())?;
    validate_prepared(&pczt, &artifact.inspection)?;
    let fvk = effects::orchard_fvk_from_stored_ufvk(stored_ufvk, network)?;
    let (pczt, pending) = take_trusted_context(pczt, fvk, network)?;
    validate_external_binding(&contribution, pending.retained_spend())?;
    calls.signer_calls = calls.signer_calls.saturating_add(1);
    let mut signer = Signer::new(pczt).map_err(|_| ZecError::signature_invalid())?;
    let pre_sign_sighash = signer.shielded_sighash();
    signer
        .apply_orchard_spend_auth_signature(&contribution.signature)
        .map_err(|_| ZecError::signature_invalid())?;
    calls.external_signatures_verified = calls.external_signatures_verified.saturating_add(1);
    calls.external_contributions_applied = calls.external_contributions_applied.saturating_add(1);
    if fault == Some(PipelineFault::Signer) {
        calls.record_triggered_fault(PipelineFault::Signer);
        return Err(ZecError::internal());
    }
    finish_pipeline(
        signer.finish(),
        &mut artifact,
        pending.with_pre_sign_hash(pre_sign_sighash),
        calls,
        fault,
        None,
    )
}

fn validate_external_binding(
    contribution: &TaggedContribution,
    binding: &RetainedSpendBinding,
) -> Result<(), ZecError> {
    if contribution.signature.value_pool() != orchard::ValuePool::Ironwood
        || contribution.signature.action_index() != binding.action_index
        || contribution.randomized_key != binding.randomized_key
        || contribution.signature.signature() == &[0; 64]
    {
        return Err(ZecError::signature_invalid());
    }
    Ok(())
}

fn finish_pipeline(
    signed_pczt: pczt::Pczt,
    artifact: &mut ObservedSigningArtifact<'_>,
    authority: TrustedVerificationAuthority,
    calls: &mut PipelineCalls,
    fault: Option<PipelineFault>,
    mutation: Option<VerifyMutation>,
) -> Result<VerifiedEffects, ZecError> {
    validate_signed_shape(&signed_pczt, &artifact.inspection)?;
    calls.prover_calls = calls.prover_calls.saturating_add(1);
    let branch = BranchId::try_from(*signed_pczt.global().consensus_branch_id())
        .map_err(|_| ZecError::state_corrupt())?;
    let pool = extract_orchard_spend_auth_signatures(&signed_pczt)
        .first()
        .map(SpendAuthSignature::value_pool)
        .ok_or_else(ZecError::signature_invalid)?;
    let bundle_version =
        zcash_primitives::transaction::components::orchard::bundle_version_for_branch(branch, pool)
            .ok_or_else(ZecError::state_corrupt)?;
    let proving_key = zcash_primitives::transaction::builder::cached_orchard_proving_key(
        bundle_version.circuit_version(),
    );
    let proven = Prover::new(signed_pczt)
        .create_ironwood_proof(proving_key)
        .map_err(|_| ZecError::internal())?
        .finish();
    if fault == Some(PipelineFault::Prover) {
        calls.record_triggered_fault(PipelineFault::Prover);
        return Err(ZecError::internal());
    }

    calls.finalizer_calls = calls.finalizer_calls.saturating_add(1);
    let finalized = SpendFinalizer::new(proven)
        .finalize_spends()
        .map_err(|_| ZecError::internal())?;
    let (finalized, clear) =
        inspect_final_pczt(finalized, &artifact.inspection, authority.retained_spend())?;
    if fault == Some(PipelineFault::Finalizer) {
        calls.record_triggered_fault(PipelineFault::Finalizer);
        return Err(ZecError::internal());
    }

    calls.extractor_calls = calls.extractor_calls.saturating_add(1);
    let transaction = TransactionExtractor::new(finalized)
        .extract()
        .map_err(|_| ZecError::signature_invalid())?;
    let mut encoded_transaction = ExtractedTransaction::new(&mut *artifact.observer);
    transaction
        .write(&mut encoded_transaction)
        .map_err(|_| ZecError::internal())?;
    drop(transaction);
    if fault == Some(PipelineFault::Extractor) {
        calls.record_triggered_fault(PipelineFault::Extractor);
        return Err(ZecError::internal());
    }

    public_matches_inspection(&artifact.public, &artifact.inspection)?;
    let mut expected = artifact.inspection.clone();
    let mut expected_fee_bound = artifact.public.fee_bound_zat.clone();
    let mut expected_txid = None;
    apply_expected_mutation(
        &mut expected,
        &mut expected_fee_bound,
        &mut expected_txid,
        mutation,
    );
    if mutation == Some(VerifyMutation::Proof) {
        flip_unique_bytes(encoded_transaction.as_mut_slice(), clear.proof.as_slice())?;
    }
    if mutation == Some(VerifyMutation::Signature) {
        flip_unique_bytes(
            encoded_transaction.as_mut_slice(),
            clear.spend_auth_sig.as_slice(),
        )?;
    }
    if mutation == Some(VerifyMutation::MalformedState) {
        encoded_transaction.wipe_tail_and_truncate();
    }

    calls.independent_decoder_calls = calls.independent_decoder_calls.saturating_add(1);
    let decoded = decode_extracted_transaction(encoded_transaction.as_slice())?;
    calls.verifier_calls = calls.verifier_calls.saturating_add(1);
    if fault == Some(PipelineFault::Verifier) {
        calls.record_triggered_fault(PipelineFault::Verifier);
        return Err(ZecError::internal());
    }
    let effects = independently_verify(
        decoded,
        &artifact.inspection,
        &expected,
        &expected_fee_bound,
        expected_txid.as_deref(),
        &authority,
    )?;
    if fault == Some(PipelineFault::PanicAfterVerification) {
        panic!("INTERNAL");
    }
    drop(encoded_transaction);
    if fault == Some(PipelineFault::Cleanup) {
        calls.record_triggered_fault(PipelineFault::Cleanup);
        return Err(ZecError::internal());
    }
    Ok(effects)
}

fn apply_expected_mutation(
    expected: &mut PcztInspection,
    fee_bound: &mut String,
    expected_txid: &mut Option<String>,
    mutation: Option<VerifyMutation>,
) {
    match mutation {
        Some(VerifyMutation::Receiver) => {
            expected.destination.push('x');
            if let Some(byte) = expected.destination_receiver_bytes.first_mut() {
                *byte ^= 0x01;
            }
        }
        Some(VerifyMutation::Amount) => expected.amount_zat = "1".to_owned(),
        Some(VerifyMutation::Network) => expected.network = "zec-testnet".to_owned(),
        Some(VerifyMutation::Fee) => expected.fee_zat = "1".to_owned(),
        Some(VerifyMutation::FeeBound) => *fee_bound = "1".to_owned(),
        Some(VerifyMutation::Memo) => expected.memo_sha256 = "ff".repeat(32),
        Some(VerifyMutation::RequestId) => expected.request_id_binding = "11".repeat(16),
        Some(VerifyMutation::IntentHash) => expected.intent_hash_binding = "ee".repeat(32),
        Some(VerifyMutation::Pool) => expected.spend_pool = "orchard".to_owned(),
        Some(VerifyMutation::Version) => expected.transaction_version = 5,
        Some(VerifyMutation::ConsensusBranch) => expected.consensus_branch = 0,
        Some(VerifyMutation::Change) => expected.ironwood_outputs = 1,
        Some(VerifyMutation::ExtractedTransactionIdBinding) => {
            *expected_txid = Some("00".repeat(32));
        }
        Some(VerifyMutation::MalformedSchema) => expected.network.clear(),
        _ => {}
    }
}

struct ShieldedVerificationAuth;

impl Authorization for ShieldedVerificationAuth {
    type TransparentAuth = <pczt::EffectsOnly as Authorization>::TransparentAuth;
    type SaplingAuth = <Authorized as Authorization>::SaplingAuth;
    type OrchardAuth = <Authorized as Authorization>::OrchardAuth;
}

struct ShieldedVerificationContext {
    data: TransactionData<ShieldedVerificationAuth>,
    txid: zcash_protocol::TxId,
}

impl ShieldedVerificationContext {
    fn new(transaction: Transaction) -> Result<Self, ZecError> {
        let txid = transaction.txid();
        let data = transaction
            .into_data()
            .try_map_bundles::<ShieldedVerificationAuth, ZecError>(
                |bundle| match bundle {
                    Some(_) => Err(ZecError::intent_mismatch()),
                    None => Ok(None),
                },
                Ok,
                Ok,
            )?;
        Ok(Self { data, txid })
    }

    fn data(&self) -> &TransactionData<ShieldedVerificationAuth> {
        &self.data
    }

    fn transaction_id(&self) -> zcash_protocol::TxId {
        self.txid
    }

    fn shielded_sighash(&self) -> [u8; 32] {
        let txid_parts = self.data.digest(TxIdDigester);
        let sighash = signature_hash(&self.data, &SignableInput::Shielded, &txid_parts);
        *sighash.as_ref()
    }
}

fn independently_verify(
    transaction: Transaction,
    prepared: &PcztInspection,
    expected: &PcztInspection,
    fee_bound: &str,
    expected_txid: Option<&str>,
    authority: &TrustedVerificationAuthority,
) -> Result<VerifiedEffects, ZecError> {
    if expected.network.is_empty()
        || (!matches!(expected.network.as_str(), "zec-local" | "zec-testnet")
            && expected.network != "zec-mainnet")
    {
        return Err(ZecError::schema());
    }
    if expected.network == "zec-mainnet" {
        return Err(ZecError::network_disabled());
    }
    let context = ShieldedVerificationContext::new(transaction)?;
    let data = context.data();
    let ironwood = data
        .ironwood_bundle()
        .ok_or_else(ZecError::intent_mismatch)?;
    let proof_present = !ironwood.authorization().proof().as_ref().is_empty();
    let spend_authorization_present = !ironwood.actions().is_empty();
    let binding_signature_present = true;

    let sighash_bytes = context.shielded_sighash();

    let mut spend_authorization_valid = true;
    for action in ironwood.actions() {
        if action
            .rk()
            .verify(&sighash_bytes, action.authorization())
            .is_err()
        {
            spend_authorization_valid = false;
        }
    }
    let binding_signature_valid = ironwood
        .binding_validating_key()
        .verify(&sighash_bytes, ironwood.authorization().binding_signature())
        .is_ok();

    let circuit_version = ironwood.bundle_version().circuit_version();
    let vk = orchard::circuit::VerifyingKey::build(circuit_version);
    let proof_valid = ironwood.verify_proof(&vk).is_ok();
    if !proof_present
        || !proof_valid
        || !spend_authorization_present
        || !spend_authorization_valid
        || !binding_signature_present
        || !binding_signature_valid
    {
        return Err(ZecError::signature_invalid());
    }

    let derived_transaction_id = context.transaction_id().to_string();
    let fee_bound_zat = fee_bound
        .parse::<u64>()
        .map_err(|_| ZecError::schema())
        .and_then(|value| Zatoshis::from_u64(value).map_err(|_| ZecError::schema()))?;
    if data.version() != TxVersion::V6
        || data.consensus_branch_id() != BranchId::Nu6_3
        || !ironwood.flags().spends_enabled()
        || !ironwood.flags().outputs_enabled()
        || ironwood.actions().len() != 2
    {
        return Err(ZecError::intent_mismatch());
    }
    if sighash_bytes != authority.pre_sign_sighash()
        || ironwood.anchor().to_bytes() != authority.resolved_anchor()
    {
        return Err(ZecError::intent_mismatch());
    }
    let retained = authority.retained_spend();
    let decoded_action = ironwood
        .actions()
        .get(retained.action_index)
        .ok_or_else(ZecError::intent_mismatch)?;
    if decoded_action.nullifier().to_bytes() != retained.nullifier
        || <[u8; 32]>::from(decoded_action.rk()) != retained.randomized_key
    {
        return Err(ZecError::intent_mismatch());
    }
    let has_transparent = data.transparent_bundle().is_some();
    let has_sapling = data.sapling_bundle().is_some();
    let has_orchard = data.orchard_bundle().is_some();
    let decoded_version = 6;
    let decoded_branch = u32::from(data.consensus_branch_id());
    let recovered = effects::recover_decoded_effects(ironwood, authority)?;
    effects::recovered_matches_expectation(
        &recovered,
        prepared,
        expected,
        fee_bound_zat,
        authority,
        effects::DecodedPoolShape {
            has_transparent,
            has_sapling,
            has_orchard,
            version: decoded_version,
            branch: decoded_branch,
        },
    )?;
    if expected_txid.is_some_and(|value| value != derived_transaction_id) {
        return Err(ZecError::intent_mismatch());
    }
    let fee_zat_u64 = recovered.fee.into_u64();
    let fee_bound_zat_u64 = fee_bound_zat.into_u64();
    Ok(VerifiedEffects {
        network: prepared.network.clone(),
        mainnet: false,
        transaction_version: decoded_version,
        consensus_branch: decoded_branch,
        external_receiver_bytes: recovered.payment_receiver_bytes,
        external_receiver: recovered.payment_receiver,
        external_amount_zat: recovered.payment_amount.into_u64().to_string(),
        fee_zat: recovered.fee.into_u64().to_string(),
        fee_bound_zat: artifact_fee_bound(fee_bound, fee_zat_u64),
        fee_zat_u64,
        fee_bound_zat_u64,
        memo_sha256: recovered.memo_sha256,
        request_id_binding: prepared.request_id_binding.clone(),
        intent_hash_binding: prepared.intent_hash_binding.clone(),
        ironwood_real_spends: recovered.real_spends,
        ironwood_external_outputs: recovered.external_outputs,
        ironwood_internal_change_outputs: recovered.internal_change_outputs,
        transparent_effects: usize::from(has_transparent),
        sapling_effects: usize::from(has_sapling),
        orchard_effects: usize::from(has_orchard),
        proof_present,
        proof_valid,
        spend_authorization_present,
        spend_authorization_valid,
        binding_signature_present,
        binding_signature_valid,
        derived_transaction_id,
    })
}

fn artifact_fee_bound(fee_bound: &str, fee_zat_u64: u64) -> String {
    let _ = fee_zat_u64;
    fee_bound.to_owned()
}

struct ClearEffects {
    proof: Vec<u8>,
    spend_auth_sig: Vec<u8>,
}

fn validate_unsigned_shape(pczt: &pczt::Pczt) -> Result<(), ZecError> {
    if *pczt.global().tx_version() != zcash_protocol::constants::V6_TX_VERSION
        || *pczt.global().consensus_branch_id() != 0x37a5_165b
        || !pczt.transparent().inputs().is_empty()
        || !pczt.transparent().outputs().is_empty()
        || !pczt.sapling().spends().is_empty()
        || !pczt.sapling().outputs().is_empty()
        || !pczt.orchard().actions().is_empty()
        || pczt.ironwood().actions().len() != 2
    {
        return Err(ZecError::intent_mismatch());
    }
    unsigned_ironwood_action(pczt).map(|_| ())
}

fn validate_prepared(pczt: &pczt::Pczt, inspection: &PcztInspection) -> Result<(), ZecError> {
    validate_unsigned_shape(pczt)?;
    if inspection.network != "zec-local"
        || inspection.consensus_branch != 0x37a5_165b
        || inspection.transaction_version != 6
        || inspection.spend_pool != "ironwood"
        || inspection.ironwood_inputs != 1
        || inspection.ironwood_outputs != 2
        || inspection.has_transparent_bundle
        || inspection.has_sapling_bundle
        || inspection.has_orchard_output_bundle
    {
        return Err(ZecError::intent_mismatch());
    }
    let expected_amount = inspection
        .amount_zat
        .parse::<u64>()
        .map_err(|_| ZecError::state_corrupt())?;
    let mut matching_outputs = 0;
    for action in pczt.ironwood().actions() {
        if action.output().value() == &Some(expected_amount)
            && action.output().user_address().as_deref() == Some(&inspection.destination)
        {
            matching_outputs += 1;
            if action
                .output()
                .recipient()
                .as_ref()
                .map(|value| value.as_slice())
                != Some(inspection.destination_receiver_bytes.as_slice())
            {
                return Err(ZecError::intent_mismatch());
            }
            let pczt::orchard::EncCiphertext::MemoPlaintext(memo) =
                action.output().enc_ciphertext()
            else {
                return Err(ZecError::intent_mismatch());
            };
            if sha256_hex(memo.as_stripped_bytes()) != inspection.memo_sha256 {
                return Err(ZecError::intent_mismatch());
            }
        }
    }
    if matching_outputs != 1 {
        return Err(ZecError::intent_mismatch());
    }
    Ok(())
}

fn validate_signed_shape(pczt: &pczt::Pczt, inspection: &PcztInspection) -> Result<(), ZecError> {
    if *pczt.global().tx_version() != inspection.transaction_version
        || *pczt.global().consensus_branch_id() != inspection.consensus_branch
        || pczt.ironwood().actions().len() != 2
        || pczt
            .ironwood()
            .actions()
            .iter()
            .any(|action| action.spend().spend_auth_sig().is_none())
    {
        return Err(ZecError::signature_invalid());
    }
    Ok(())
}

fn unsigned_ironwood_action(pczt: &pczt::Pczt) -> Result<usize, ZecError> {
    let mut indices = pczt
        .ironwood()
        .actions()
        .iter()
        .enumerate()
        .filter_map(|(index, action)| action.spend().spend_auth_sig().is_none().then_some(index));
    let index = indices.next().ok_or_else(ZecError::signature_invalid)?;
    if indices.next().is_some() {
        return Err(ZecError::signature_invalid());
    }
    Ok(index)
}

fn take_trusted_context(
    pczt: pczt::Pczt,
    account_fvk: FullViewingKey,
    network: Network,
) -> Result<(pczt::Pczt, effects::PendingTrustedContext), ZecError> {
    let action_index = unsigned_ironwood_action(&pczt)?;
    if pczt.ironwood().actions().len() != 2 || action_index >= 2 {
        return Err(ZecError::intent_mismatch());
    }
    let mut captured = None;
    let verifier = Verifier::new(pczt)
        .with_ironwood(|bundle| {
            let value =
                effects::validate_retained_action_cryptography(bundle, action_index, &account_fvk)
                    .map_err(|_| {
                        pczt::roles::verifier::OrchardError::Custom(
                            SpendValueInspectionError::MissingValue,
                        )
                    })?;
            captured = Some(value);
            Ok(())
        })
        .map_err(|_| ZecError::intent_mismatch())?;
    let (binding, resolved_anchor) = captured.ok_or_else(ZecError::intent_mismatch)?;
    Ok((
        verifier.finish(),
        effects::PendingTrustedContext::new(network, account_fvk, binding, resolved_anchor),
    ))
}

#[cfg(test)]
pub(crate) fn capture_trusted_verification_authority(
    pczt: &pczt::Pczt,
    account_fvk: FullViewingKey,
    network: Network,
    pre_sign_sighash: [u8; 32],
) -> Result<TrustedVerificationAuthority, ZecError> {
    let (_, pending) = take_trusted_context(pczt.clone(), account_fvk, network)?;
    Ok(pending.with_pre_sign_hash(pre_sign_sighash))
}

fn decode_extracted_transaction(bytes: &[u8]) -> Result<Transaction, ZecError> {
    let mut cursor = Cursor::new(bytes);
    let transaction =
        Transaction::read(&mut cursor, BranchId::Nu6_3).map_err(|_| ZecError::state_corrupt())?;
    let consumed = usize::try_from(cursor.position()).map_err(|_| ZecError::state_corrupt())?;
    if consumed != bytes.len() {
        return Err(ZecError::state_corrupt());
    }
    Ok(transaction)
}

fn public_matches_inspection(
    public: &PreparedZecV1,
    inspection: &PcztInspection,
) -> Result<(), ZecError> {
    if public.network != inspection.network
        || public.request_id != inspection.request_id_binding
        || public.intent_hash != inspection.intent_hash_binding
        || public.receiver != inspection.destination
        || public.amount_zat != inspection.amount_zat
        || public.fee_zat != inspection.fee_zat
        || public.spend_pool != inspection.spend_pool
        || public.output_pool != "ironwood"
        || public.tx_version != inspection.transaction_version.to_string()
        || public.consensus_branch != format!("{:x}", inspection.consensus_branch)
    {
        return Err(ZecError::intent_mismatch());
    }
    Ok(())
}

fn inspect_ironwood_spend_values(pczt: pczt::Pczt) -> Result<(pczt::Pczt, [u64; 2]), ZecError> {
    let mut values = [0u64; 2];
    let verifier = Verifier::new(pczt)
        .with_ironwood(|bundle| {
            let actions = bundle.actions();
            if actions.len() != 2 {
                return Err(pczt::roles::verifier::OrchardError::Custom(
                    SpendValueInspectionError::ActionCount,
                ));
            }
            for (index, action) in actions.iter().enumerate() {
                let Some(value) = *action.spend().value() else {
                    return Err(pczt::roles::verifier::OrchardError::Custom(
                        SpendValueInspectionError::MissingValue,
                    ));
                };
                values[index] = value.inner();
            }
            Ok(())
        })
        .map_err(|_| ZecError::intent_mismatch())?;
    Ok((verifier.finish(), values))
}

fn inspect_final_pczt(
    pczt: pczt::Pczt,
    inspection: &PcztInspection,
    binding: &RetainedSpendBinding,
) -> Result<(pczt::Pczt, ClearEffects), ZecError> {
    let expected_amount = inspection
        .amount_zat
        .parse::<u64>()
        .map_err(|_| ZecError::state_corrupt())?;
    let expected_fee = inspection
        .fee_zat
        .parse::<u64>()
        .map_err(|_| ZecError::state_corrupt())?;
    let spend_auth_sig = {
        let actions = pczt.ironwood().actions();
        if actions.len() != 2 || binding.action_index >= 2 {
            return Err(ZecError::intent_mismatch());
        }
        let retained = &actions[binding.action_index];
        let other = &actions[1 - binding.action_index];
        if retained.spend().rk() != &binding.randomized_key
            || retained.spend().nullifier() != &binding.nullifier
        {
            return Err(ZecError::intent_mismatch());
        }
        let spend_auth_sig = retained
            .spend()
            .spend_auth_sig()
            .ok_or_else(ZecError::signature_invalid)?
            .to_vec();
        if other.spend().spend_auth_sig().is_none() {
            return Err(ZecError::signature_invalid());
        }
        spend_auth_sig
    };
    let mut external_outputs = 0;
    let mut change_outputs = 0;
    for action in pczt.ironwood().actions() {
        if action.output().value() == &Some(expected_amount)
            && action.output().user_address().as_deref() == Some(&inspection.destination)
        {
            external_outputs += 1;
            if action
                .output()
                .recipient()
                .map(|value| value.to_vec())
                .as_ref()
                != Some(&inspection.destination_receiver_bytes)
            {
                return Err(ZecError::intent_mismatch());
            }
            if let pczt::orchard::EncCiphertext::MemoPlaintext(memo) =
                action.output().enc_ciphertext()
                && sha256_hex(memo.as_stripped_bytes()) != inspection.memo_sha256
            {
                return Err(ZecError::intent_mismatch());
            }
        } else if action.output().value().is_some_and(|value| value > 0)
            && action.output().user_address().is_none()
        {
            change_outputs += 1;
        }
    }
    let proof = pczt
        .ironwood()
        .zkproof()
        .clone()
        .ok_or_else(ZecError::intent_mismatch)?;
    if external_outputs != 1
        || change_outputs != 1
        || pczt.ironwood().value_sum() != &(expected_fee, false)
        || proof.is_empty()
    {
        return Err(ZecError::intent_mismatch());
    }
    let (pczt, values) = inspect_ironwood_spend_values(pczt)?;
    let other_index = 1 - binding.action_index;
    if values[binding.action_index] != binding.value_zat
        || binding.value_zat == 0
        || values[other_index] != 0
    {
        return Err(ZecError::intent_mismatch());
    }
    Ok((
        pczt,
        ClearEffects {
            proof,
            spend_auth_sig,
        },
    ))
}

fn flip_unique_bytes(haystack: &mut [u8], needle: &[u8]) -> Result<(), ZecError> {
    if needle.is_empty() {
        return Err(ZecError::state_corrupt());
    }
    let Some(index) = haystack
        .windows(needle.len())
        .position(|window| window == needle)
    else {
        return Err(ZecError::state_corrupt());
    };
    haystack[index] ^= 0xff;
    Ok(())
}

struct ExtractedTransaction<'a> {
    bytes: Zeroizing<Vec<u8>>,
    observer: &'a mut dyn WipeObserver,
}

impl<'a> ExtractedTransaction<'a> {
    fn new(observer: &'a mut dyn WipeObserver) -> Self {
        Self {
            bytes: Zeroizing::new(Vec::new()),
            observer,
        }
    }

    fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }

    fn wipe_tail_and_truncate(&mut self) {
        let keep = self.bytes.len() / 2;
        self.bytes[keep..].zeroize();
        self.bytes.truncate(keep);
    }
}

impl Write for ExtractedTransaction<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.bytes.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.bytes.flush()
    }
}

impl Drop for ExtractedTransaction<'_> {
    fn drop(&mut self) {
        let length = self.bytes.len();
        self.bytes.as_mut_slice().zeroize();
        let all_zero = self.bytes.iter().all(|byte| *byte == 0);
        self.observer.observe(WipeEvent {
            label: "zec-extracted-transaction",
            length,
            all_zero,
        });
    }
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

#[allow(dead_code)]
#[allow(
    clippy::too_many_arguments,
    reason = "reviewed authority/verification/fault inputs deliberately remain explicit"
)]
pub(crate) fn review_matches_capability(
    review: &PreparedReview,
    handle: &str,
    session_id: &str,
    account_id: &str,
    network: &str,
    request_id: &str,
    intent_hash: &str,
    review_hash: &str,
) -> Result<(), ZecError> {
    if handle != review.handle
        || session_id != review.session_id
        || account_id != review.public.account_id
        || network != review.public.network
        || request_id != review.public.request_id
        || intent_hash != review.public.intent_hash
        || review_hash != review.review_hash
    {
        return Err(ZecError::intent_mismatch());
    }
    Ok(())
}

fn mutex_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|error| error.into_inner())
}

#[cfg(test)]
mod verification_context_tests;

#[cfg(test)]
mod external_binding_tests;

#[cfg(test)]
mod cleanup_lifecycle_tests;
