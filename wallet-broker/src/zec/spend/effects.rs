use orchard::Address;
use orchard::bundle::Authorization as OrchardAuthorization;
use orchard::keys::{FullViewingKey, Scope};
use orchard::note::Note;
use zcash_keys::address::UnifiedAddress;
use zcash_keys::keys::UnifiedFullViewingKey;
use zcash_protocol::consensus::Parameters;
use zcash_protocol::value::{ZatBalance, Zatoshis};
use zeroize::Zeroizing;

use crate::zec::prepare::{PcztInspection, sha256_hex};
use crate::zec::{Network, ZecError};

use super::RetainedSpendBinding;

pub(crate) struct TrustedVerificationAuthority {
    network: Network,
    fvk: FullViewingKey,
    retained_spend: RetainedSpendBinding,
    pre_sign_sighash: [u8; 32],
    resolved_anchor: [u8; 32],
}

pub(super) struct PendingTrustedContext {
    network: Network,
    fvk: FullViewingKey,
    retained_spend: RetainedSpendBinding,
    resolved_anchor: [u8; 32],
}

pub(super) struct RecoveredEffects {
    pub(super) payment_receiver_bytes: Vec<u8>,
    pub(super) payment_receiver: String,
    pub(super) payment_amount: Zatoshis,
    pub(super) memo_sha256: String,
    pub(super) fee: Zatoshis,
    pub(super) real_spends: usize,
    pub(super) external_outputs: usize,
    pub(super) internal_change_outputs: usize,
}

pub(super) struct DecodedPoolShape {
    pub(super) has_transparent: bool,
    pub(super) has_sapling: bool,
    pub(super) has_orchard: bool,
    pub(super) version: u32,
    pub(super) branch: u32,
}

impl PendingTrustedContext {
    pub(super) fn new(
        network: Network,
        fvk: FullViewingKey,
        retained_spend: RetainedSpendBinding,
        resolved_anchor: [u8; 32],
    ) -> Self {
        Self {
            network,
            fvk,
            retained_spend,
            resolved_anchor,
        }
    }

    pub(super) fn action_index(&self) -> usize {
        self.retained_spend.action_index
    }

    pub(super) fn retained_spend(&self) -> &RetainedSpendBinding {
        &self.retained_spend
    }

    pub(super) fn with_pre_sign_hash(
        self,
        pre_sign_sighash: [u8; 32],
    ) -> TrustedVerificationAuthority {
        TrustedVerificationAuthority {
            network: self.network,
            fvk: self.fvk,
            retained_spend: self.retained_spend,
            pre_sign_sighash,
            resolved_anchor: self.resolved_anchor,
        }
    }
}

impl TrustedVerificationAuthority {
    pub(super) fn network(&self) -> Network {
        self.network
    }

    pub(super) fn fvk(&self) -> &FullViewingKey {
        &self.fvk
    }

    pub(super) fn retained_spend(&self) -> &RetainedSpendBinding {
        &self.retained_spend
    }

    pub(super) fn pre_sign_sighash(&self) -> [u8; 32] {
        self.pre_sign_sighash
    }

    pub(super) fn resolved_anchor(&self) -> [u8; 32] {
        self.resolved_anchor
    }
}

pub(super) fn orchard_fvk_from_stored_ufvk(
    stored_ufvk: &str,
    network: Network,
) -> Result<FullViewingKey, ZecError> {
    match network {
        Network::Testnet => orchard_fvk_for(
            &zcash_protocol::consensus::Network::TestNetwork,
            stored_ufvk,
        ),
        Network::Local(local) => orchard_fvk_for(&local.upstream(), stored_ufvk),
    }
}

fn orchard_fvk_for<P: Parameters>(
    params: &P,
    stored_ufvk: &str,
) -> Result<FullViewingKey, ZecError> {
    let ufvk =
        UnifiedFullViewingKey::decode(params, stored_ufvk).map_err(|_| ZecError::locked())?;
    ufvk.orchard().cloned().ok_or_else(ZecError::locked)
}

pub(super) fn validate_retained_action_cryptography(
    bundle: &orchard::pczt::Bundle,
    real_index: usize,
    account_fvk: &FullViewingKey,
) -> Result<(RetainedSpendBinding, [u8; 32]), ZecError> {
    let actions = bundle.actions();
    if actions.len() != 2 || real_index >= 2 {
        return Err(ZecError::intent_mismatch());
    }
    bundle
        .verify_cross_address_restriction()
        .map_err(|_| ZecError::intent_mismatch())?;
    for action in actions {
        action
            .verify_cv_net()
            .map_err(|_| ZecError::intent_mismatch())?;
        action
            .output()
            .verify_note_commitment(action.spend())
            .map_err(|_| ZecError::intent_mismatch())?;
    }
    let real = &actions[real_index];
    real.spend()
        .verify_nullifier(Some(account_fvk))
        .map_err(|_| ZecError::intent_mismatch())?;
    real.spend()
        .verify_rk(Some(account_fvk))
        .map_err(|_| ZecError::intent_mismatch())?;
    let Some(value) = *real.spend().value() else {
        return Err(ZecError::intent_mismatch());
    };
    let Some(other_value) = *actions[1 - real_index].spend().value() else {
        return Err(ZecError::intent_mismatch());
    };
    let value_zat = value.inner();
    let other_value = other_value.inner();
    if value_zat == 0 || other_value != 0 {
        return Err(ZecError::intent_mismatch());
    }
    Ok((
        RetainedSpendBinding {
            action_index: real_index,
            randomized_key: <[u8; 32]>::from(real.spend().rk()),
            nullifier: real.spend().nullifier().to_bytes(),
            value_zat,
        },
        bundle.anchor().to_bytes(),
    ))
}

pub(super) fn recover_decoded_effects<T: OrchardAuthorization>(
    ironwood: &orchard::Bundle<T, ZatBalance>,
    authority: &TrustedVerificationAuthority,
) -> Result<RecoveredEffects, ZecError> {
    let fee =
        Zatoshis::try_from(*ironwood.value_balance()).map_err(|_| ZecError::intent_mismatch())?;
    let external_ovk = authority.fvk().to_ovk(Scope::External);
    let internal_ovk = authority.fvk().to_ovk(Scope::Internal);
    let internal_ivk = authority.fvk().to_ivk(Scope::Internal);
    let action_count = ironwood.actions().len();
    let mut payment = None;
    let mut change = None;
    let mut external_outputs = 0usize;
    let mut internal_change_outputs = 0usize;
    for index in 0..action_count {
        let external_outgoing = ironwood.recover_output_with_ovk(index, &external_ovk);
        let internal_outgoing = ironwood.recover_output_with_ovk(index, &internal_ovk);
        let internal_incoming = ironwood.decrypt_output_with_key(index, &internal_ivk);
        if let (
            Some((left_note, left_address, left_memo)),
            Some((right_note, right_address, right_memo)),
        ) = (external_outgoing.as_ref(), internal_outgoing.as_ref())
            && !same_recovered_note(
                left_note,
                left_address,
                left_memo,
                right_note,
                right_address,
                right_memo,
            )
        {
            return Err(ZecError::intent_mismatch());
        }
        let outgoing = external_outgoing.or(internal_outgoing);
        if let (
            Some((left_note, left_address, left_memo)),
            Some((right_note, right_address, right_memo)),
        ) = (outgoing.as_ref(), internal_incoming.as_ref())
            && !same_recovered_note(
                left_note,
                left_address,
                left_memo,
                right_note,
                right_address,
                right_memo,
            )
        {
            return Err(ZecError::intent_mismatch());
        }
        if let Some((note, address, memo)) = internal_incoming {
            if change.is_some() {
                return Err(ZecError::intent_mismatch());
            }
            let value = note.value().inner();
            if value == 0 || authority.fvk().scope_for_address(&address) != Some(Scope::Internal) {
                return Err(ZecError::intent_mismatch());
            }
            internal_change_outputs = internal_change_outputs
                .checked_add(1)
                .ok_or_else(ZecError::intent_mismatch)?;
            change = Some((address, value, Zeroizing::new(memo)));
        } else if let Some((note, address, memo)) = outgoing {
            if payment.is_some() {
                return Err(ZecError::intent_mismatch());
            }
            let value = note.value().inner();
            if value == 0 || authority.fvk().scope_for_address(&address) == Some(Scope::Internal) {
                return Err(ZecError::intent_mismatch());
            }
            external_outputs = external_outputs
                .checked_add(1)
                .ok_or_else(ZecError::intent_mismatch)?;
            payment = Some((address, value, Zeroizing::new(memo)));
        } else {
            return Err(ZecError::intent_mismatch());
        }
    }
    if external_outputs != 1 || internal_change_outputs != 1 {
        return Err(ZecError::intent_mismatch());
    }
    let (payment_address, payment_value, payment_memo) =
        payment.ok_or_else(ZecError::intent_mismatch)?;
    let (_change_address, change_value, _change_memo) =
        change.ok_or_else(ZecError::intent_mismatch)?;
    let payment_amount =
        Zatoshis::from_u64(payment_value).map_err(|_| ZecError::intent_mismatch())?;
    let change_amount =
        Zatoshis::from_u64(change_value).map_err(|_| ZecError::intent_mismatch())?;
    let conserved = (payment_amount + change_amount)
        .and_then(|sum| sum + fee)
        .ok_or_else(ZecError::intent_mismatch)?;
    let retained_input = Zatoshis::from_u64(authority.retained_spend().value_zat)
        .map_err(|_| ZecError::intent_mismatch())?;
    if retained_input != conserved {
        return Err(ZecError::intent_mismatch());
    }
    let memo_sha256 = sha256_hex(stripped_memo_bytes(&payment_memo));
    Ok(RecoveredEffects {
        payment_receiver_bytes: payment_address.to_raw_address_bytes().to_vec(),
        payment_receiver: encode_orchard_unified_address(authority.network(), payment_address)?,
        payment_amount,
        memo_sha256,
        fee,
        real_spends: usize::from(authority.retained_spend().value_zat > 0),
        external_outputs,
        internal_change_outputs,
    })
}

pub(super) fn recovered_matches_expectation(
    recovered: &RecoveredEffects,
    prepared: &PcztInspection,
    expected: &PcztInspection,
    fee_bound: Zatoshis,
    authority: &TrustedVerificationAuthority,
    decoded: DecodedPoolShape,
) -> Result<(), ZecError> {
    if expected.network != prepared.network
        || expected.network != authority.network().as_str()
        || expected.request_id_binding != prepared.request_id_binding
        || expected.intent_hash_binding != prepared.intent_hash_binding
    {
        return Err(ZecError::intent_mismatch());
    }
    let expected_amount = parse_zatoshis(&expected.amount_zat)?;
    let expected_fee = parse_zatoshis(&expected.fee_zat)?;
    let output_count = recovered
        .external_outputs
        .checked_add(recovered.internal_change_outputs)
        .ok_or_else(ZecError::intent_mismatch)?;
    if expected.destination_receiver_bytes != recovered.payment_receiver_bytes
        || expected.destination != recovered.payment_receiver
        || expected_amount != recovered.payment_amount
        || expected.memo_sha256 != recovered.memo_sha256
        || expected_fee != recovered.fee
        || recovered.fee > fee_bound
        || expected.ironwood_inputs != recovered.real_spends
        || expected.ironwood_outputs != output_count
        || expected.spend_pool != "ironwood"
        || expected.has_transparent_bundle != decoded.has_transparent
        || expected.has_sapling_bundle != decoded.has_sapling
        || expected.has_orchard_output_bundle != decoded.has_orchard
        || decoded.has_transparent
        || decoded.has_sapling
        || decoded.has_orchard
        || expected.transaction_version != decoded.version
        || expected.consensus_branch != decoded.branch
    {
        return Err(ZecError::intent_mismatch());
    }
    Ok(())
}

pub(super) fn parse_zatoshis(value: &str) -> Result<Zatoshis, ZecError> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| ZecError::state_corrupt())?;
    Zatoshis::from_u64(parsed).map_err(|_| ZecError::intent_mismatch())
}

fn same_recovered_note(
    left_note: &Note,
    left_address: &Address,
    left_memo: &[u8; 512],
    right_note: &Note,
    right_address: &Address,
    right_memo: &[u8; 512],
) -> bool {
    left_note == right_note
        && left_address == right_address
        && left_note.value() == right_note.value()
        && left_memo == right_memo
}

fn stripped_memo_bytes(memo: &[u8; 512]) -> &[u8] {
    let len = memo
        .iter()
        .rposition(|byte| *byte != 0)
        .map_or(0, |index| index + 1);
    &memo[..len]
}

fn encode_orchard_unified_address(network: Network, address: Address) -> Result<String, ZecError> {
    let unified = UnifiedAddress::from_receivers(Some(address), None, None)
        .ok_or_else(ZecError::intent_mismatch)?;
    match network {
        Network::Testnet => Ok(unified.encode(&zcash_protocol::consensus::Network::TestNetwork)),
        Network::Local(local) => Ok(unified.encode(&local.upstream())),
    }
}
