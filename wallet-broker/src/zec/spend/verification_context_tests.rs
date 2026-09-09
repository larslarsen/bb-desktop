use std::io::Cursor;

use orchard::circuit::VerifyingKey;
use orchard::keys::{FullViewingKey, Scope};
use pczt::orchard::MemoPlaintext;
use pczt::roles::prover::Prover;
use pczt::roles::signer::{Signer, extract_orchard_spend_auth_signatures};
use pczt::roles::spend_finalizer::SpendFinalizer;
use pczt::roles::tx_extractor::TransactionExtractor;
use zcash_keys::address::UnifiedAddress;
use zcash_keys::keys::UnifiedSpendingKey;
use zcash_primitives::transaction::builder::cached_orchard_proving_key;
use zcash_primitives::transaction::components::orchard::{
    bundle_version_for_branch, write_v6_bundle,
};
use zcash_primitives::transaction::{Authorized, Transaction, TxVersion};
use zcash_protocol::consensus::BranchId;
use zeroize::Zeroizing;

use crate::vault::{SecretBytes, WipeEvent, WipeObserver};
use crate::zec::prepare::{PcztInspection, sha256_hex};
use crate::zec::scan::ScanRequest;
use crate::zec::store::AddressAccount;
use crate::zec::test_support::verification_context_test_root;
use crate::zec::{AccountId, LocalNetwork, Network};

use super::ShieldedVerificationContext;

const FIXTURE_DIR: &str = "tests/fixtures/zec";
const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const REQUEST_ID: &str = "00112233445566778899aabbccddeeff";
const INTENT_HASH: &str = "ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0";
const AMOUNT_ZAT: u64 = 100_000_000;
const MEMO: &str = "coffee";

struct IgnoreWipes;

impl WipeObserver for IgnoreWipes {
    fn observe(&mut self, _event: WipeEvent) {}
}

struct SignedFixture {
    encoded: Zeroizing<Vec<u8>>,
    pczt_sighash: [u8; 32],
    inspection: PcztInspection,
    orchard_fvk: FullViewingKey,
    network: LocalNetwork,
    authority: super::TrustedVerificationAuthority,
}

fn signed_fixture(label: &str) -> SignedFixture {
    let frozen = crate::zec::fixture::FrozenFixture::open(FIXTURE_DIR)
        .expect("open compact fixture");
    let validated = frozen
        .validate_complete()
        .expect("validate compact fixture");
    let network = LocalNetwork::new(
        validated.manifest.network.birthday_height,
        validated.manifest.network.nu6_3,
        validated.manifest.expected.confirmation_height,
    )
    .expect("local network from fixture manifest");
    let params = network.upstream();
    let seed = SecretBytes::new(vec![0; 32]).expect("synthetic zero seed");
    let account_id = AccountId::parse(ACCOUNT).expect("fixture account id");
    let mut observer = IgnoreWipes;
    let account = AddressAccount::bootstrap(
        verification_context_test_root(label),
        account_id,
        Network::Local(network),
        seed,
        &mut observer,
    )
    .expect("bootstrap fixture account");
    account
        .scan_fixture(&validated, ScanRequest::Canonical)
        .expect("scan canonical fixture");
    let prepared = account
        .build_prepared_pczt(
            frozen.orchard_only_receiver(),
            AMOUNT_ZAT,
            MEMO,
            REQUEST_ID,
            INTENT_HASH,
        )
        .expect("build prepared pczt");
    assert!(!prepared.inspection.request_id_binding.is_empty());
    assert!(!prepared.inspection.intent_hash_binding.is_empty());
    assert_eq!(prepared.inspection.request_id_binding, REQUEST_ID);
    assert_eq!(prepared.inspection.intent_hash_binding, INTENT_HASH);
    let inspection = prepared.inspection.clone();

    let pczt = prepared
        .raw
        .expose(pczt::Pczt::parse)
        .expect("parse prepared pczt");
    assert_eq!(
        *pczt.global().tx_version(),
        zcash_protocol::constants::V6_TX_VERSION
    );
    assert!(pczt.transparent().inputs().is_empty());
    assert!(pczt.transparent().outputs().is_empty());
    assert!(pczt.sapling().spends().is_empty());
    assert!(pczt.sapling().outputs().is_empty());
    assert!(pczt.orchard().actions().is_empty());
    assert_eq!(pczt.ironwood().actions().len(), 2);
    let mut unsigned = pczt.ironwood().actions().iter().enumerate().filter_map(
        |(index, action)| action.spend().spend_auth_sig().is_none().then_some(index),
    );
    let index = unsigned
        .next()
        .expect("exactly one unsigned real Ironwood action");
    assert!(
        unsigned.next().is_none(),
        "exactly one unsigned real Ironwood action"
    );
    assert!(
        pczt.ironwood().actions()[index]
            .spend()
            .witness()
            .is_some(),
        "unsigned Ironwood action must be a real spend"
    );

    let usk = UnifiedSpendingKey::from_seed(&params, &[0; 32], Default::default())
        .expect("derive fixture unified spending key");
    let orchard_fvk = FullViewingKey::from(usk.orchard());
    let mut signer = Signer::new(pczt.clone()).expect("instantiate pczt signer");
    let pczt_sighash = signer.shielded_sighash();
    let authority = super::capture_trusted_verification_authority(
        &pczt,
        orchard_fvk.clone(),
        Network::Local(network),
        pczt_sighash,
    )
    .expect("capture trusted verification authority");
    signer
        .sign_ironwood(index, &usk.orchard().into())
        .expect("sign real ironwood action");
    let signed = signer.finish();
    drop(usk);

    let branch = BranchId::try_from(*signed.global().consensus_branch_id())
        .expect("pczt consensus branch");
    let pool = extract_orchard_spend_auth_signatures(&signed)
        .into_iter()
        .find(|signature| signature.action_index() == index)
        .map(|signature| signature.value_pool())
        .expect("signed ironwood action pool");
    let bundle_version =
        bundle_version_for_branch(branch, pool).expect("ironwood bundle version for branch");
    let proving_key = cached_orchard_proving_key(bundle_version.circuit_version());
    let proven = Prover::new(signed)
        .create_ironwood_proof(proving_key)
        .expect("create ironwood proof")
        .finish();
    let finalized = SpendFinalizer::new(proven)
        .finalize_spends()
        .expect("finalize spends");
    let transaction = TransactionExtractor::new(finalized)
        .extract()
        .expect("extract transaction");
    let mut encoded = Zeroizing::new(Vec::new());
    transaction
        .write(&mut *encoded)
        .expect("serialize extracted transaction");
    drop(transaction);
    SignedFixture {
        encoded,
        pczt_sighash,
        inspection,
        orchard_fvk,
        network,
        authority,
    }
}

fn decode_complete(bytes: &[u8], branch: BranchId) -> Transaction {
    let mut cursor = Cursor::new(bytes);
    let transaction =
        Transaction::read(&mut cursor, branch).expect("independently decode transaction");
    assert_eq!(
        usize::try_from(cursor.position()).expect("cursor position"),
        bytes.len(),
        "transaction decode must consume every input byte"
    );
    transaction
}

fn ironwood_bundle_bytes(transaction: &Transaction) -> Zeroizing<Vec<u8>> {
    let mut bytes = Zeroizing::new(Vec::new());
    write_v6_bundle(transaction.ironwood_bundle(), &mut *bytes)
        .expect("write v6 ironwood bundle");
    bytes
}

fn unique_occurrence(haystack: &[u8], needle: &[u8]) -> usize {
    assert!(!needle.is_empty(), "needle must be nonempty");
    let mut found = None;
    let mut count = 0usize;
    if haystack.len() >= needle.len() {
        for index in 0..=haystack.len() - needle.len() {
            if &haystack[index..index + needle.len()] == needle {
                count += 1;
                found = Some(index);
            }
        }
    }
    assert_eq!(count, 1, "needle must occur exactly once");
    found.expect("counted unique occurrence")
}

fn flip_unique(haystack: &mut [u8], needle: &[u8]) {
    let index = unique_occurrence(haystack, needle);
    haystack[index] ^= 0xff;
}

fn require_context(transaction: Transaction) -> ShieldedVerificationContext {
    match ShieldedVerificationContext::new(transaction) {
        Ok(context) => context,
        Err(_) => panic!("constructor rejected a valid shielded transaction"),
    }
}

fn reject_present_transparent(transaction: Transaction) {
    match ShieldedVerificationContext::new(transaction) {
        Ok(_) => panic!("constructor accepted a present transparent bundle"),
        Err(error) => assert_eq!(error.code(), "INTENT_MISMATCH"),
    }
}

#[test]
fn preserves_decoded_v6_ironwood_and_matches_pczt_sighash() {
    let fixture = signed_fixture("wal009-sighash");
    let decoded = decode_complete(fixture.encoded.as_slice(), BranchId::Nu6_3);
    let txid = decoded.txid();
    let version = decoded.version();
    let branch = decoded.consensus_branch_id();
    let lock_time = decoded.lock_time();
    let expiry_height = decoded.expiry_height();
    assert_eq!(version, TxVersion::V6);
    assert_eq!(branch, BranchId::Nu6_3);
    assert!(decoded.transparent_bundle().is_none());
    assert!(decoded.sprout_bundle().is_none());
    assert!(decoded.sapling_bundle().is_none());
    assert!(decoded.orchard_bundle().is_none());
    {
        let ironwood = decoded
            .ironwood_bundle()
            .expect("nonempty ironwood data");
        assert_eq!(
            ironwood.bundle_version(),
            orchard::bundle::BundleVersion::ironwood_v3()
        );
        assert_eq!(ironwood.actions().len(), 2);
    }
    let before_bundle = ironwood_bundle_bytes(&decoded);

    let context = require_context(decoded);
    let data = context.data();
    assert_eq!(data.version(), version);
    assert_eq!(data.consensus_branch_id(), branch);
    assert_eq!(data.lock_time(), lock_time);
    assert_eq!(data.expiry_height(), expiry_height);
    assert!(data.transparent_bundle().is_none());
    assert!(data.sprout_bundle().is_none());
    assert!(data.sapling_bundle().is_none());
    assert!(data.orchard_bundle().is_none());
    let converted = data
        .ironwood_bundle()
        .expect("converted nonempty ironwood data");
    assert_eq!(
        converted.bundle_version(),
        orchard::bundle::BundleVersion::ironwood_v3()
    );
    assert_eq!(converted.actions().len(), 2);
    let mut after_bundle = Zeroizing::new(Vec::new());
    write_v6_bundle(data.ironwood_bundle(), &mut *after_bundle)
        .expect("write converted v6 ironwood bundle");
    assert!(
        &*before_bundle == &*after_bundle,
        "converted Ironwood encoding must equal the original complete v6 encoding"
    );
    assert_eq!(context.transaction_id(), txid);
    assert_eq!(context.shielded_sighash(), fixture.pczt_sighash);
}

#[test]
fn rejects_every_present_transparent_bundle() {
    let fixture = signed_fixture("wal009-transparent");
    let canopy = decode_complete(
        &zcash_primitives::transaction::tests::data::tx_read_write::TX_READ_WRITE,
        BranchId::Canopy,
    );
    let source = canopy
        .transparent_bundle()
        .cloned()
        .expect("TX_READ_WRITE transparent bundle");
    assert!(!source.vin.is_empty());
    assert!(!source.vout.is_empty());

    let both = source.clone();
    let mut inputs_only = source.clone();
    inputs_only.vout.clear();
    let mut outputs_only = source.clone();
    outputs_only.vin.clear();
    let mut empty_some = source;
    empty_some.vin.clear();
    empty_some.vout.clear();

    let cases = [
        ("both inputs and outputs", both, true, true),
        ("inputs only", inputs_only, true, false),
        ("outputs only", outputs_only, false, true),
        ("empty Some", empty_some, false, false),
    ];
    for (label, bundle, expect_inputs, expect_outputs) in cases {
        let local = decode_complete(fixture.encoded.as_slice(), BranchId::Nu6_3);
        assert!(local.transparent_bundle().is_none());
        assert!(local.ironwood_bundle().is_some());
        let frozen = local
            .into_data()
            .map_bundles::<Authorized>(
                |_| Some(bundle),
                |sapling| sapling,
                |orchard| orchard,
            )
            .freeze()
            .expect("freeze transplanted local fixture");
        let transparent = frozen
            .transparent_bundle()
            .unwrap_or_else(|| panic!("{label}: transplanted bundle must remain Some"));
        assert_eq!(
            !transparent.vin.is_empty(),
            expect_inputs,
            "{label}: input presence"
        );
        assert_eq!(
            !transparent.vout.is_empty(),
            expect_outputs,
            "{label}: output presence"
        );
        assert!(frozen.ironwood_bundle().is_some(), "{label}: ironwood retained");
        reject_present_transparent(frozen);
    }
}

#[test]
fn uses_real_spend_and_binding_signatures() {
    let fixture = signed_fixture("wal009-signatures");
    let decoded = decode_complete(fixture.encoded.as_slice(), BranchId::Nu6_3);
    let (spend_sig, binding_sig) = {
        let ironwood = decoded
            .ironwood_bundle()
            .expect("nonempty ironwood data");
        assert_eq!(ironwood.actions().len(), 2);
        (
            <[u8; 64]>::from(ironwood.actions().first().authorization()),
            <[u8; 64]>::from(ironwood.authorization().binding_signature()),
        )
    };
    unique_occurrence(fixture.encoded.as_slice(), &spend_sig);
    unique_occurrence(fixture.encoded.as_slice(), &binding_sig);

    let context = require_context(decoded);
    let message = context.shielded_sighash();
    let ironwood = context
        .data()
        .ironwood_bundle()
        .expect("converted nonempty ironwood data");
    assert_eq!(ironwood.actions().len(), 2);
    let mut spend_ok = 0usize;
    for action in ironwood.actions() {
        assert!(
            action
                .rk()
                .verify(&message, action.authorization())
                .is_ok()
        );
        spend_ok += 1;
    }
    assert_eq!(spend_ok, 2);
    assert!(
        ironwood
            .binding_validating_key()
            .verify(&message, ironwood.authorization().binding_signature())
            .is_ok()
    );
    assert_eq!(message, fixture.pczt_sighash);

    let mut altered = message;
    altered[0] ^= 0xff;
    assert_ne!(altered, message);
    let mut spend_rejected = 0usize;
    for action in ironwood.actions() {
        assert!(
            action
                .rk()
                .verify(&altered, action.authorization())
                .is_err()
        );
        spend_rejected += 1;
    }
    assert_eq!(spend_rejected, 2);
    assert!(
        ironwood
            .binding_validating_key()
            .verify(&altered, ironwood.authorization().binding_signature())
            .is_err()
    );

    let mut spend_mutated = fixture.encoded.clone();
    flip_unique(spend_mutated.as_mut_slice(), &spend_sig);
    let spend_decoded = decode_complete(spend_mutated.as_slice(), BranchId::Nu6_3);
    let spend_context = require_context(spend_decoded);
    let spend_message = spend_context.shielded_sighash();
    assert!(
        spend_message == fixture.pczt_sighash,
        "spend-mutated context message must remain the independently captured PCZT sighash"
    );
    let spend_ironwood = spend_context
        .data()
        .ironwood_bundle()
        .expect("spend-mutated ironwood data");
    assert_eq!(spend_ironwood.actions().len(), 2);
    let mut mutated_sig = spend_sig;
    mutated_sig[0] ^= 0xff;
    let mut matching_failed = 0usize;
    let mut untouched_ok = 0usize;
    for action in spend_ironwood.actions() {
        if <[u8; 64]>::from(action.authorization()) == mutated_sig {
            assert!(
                action
                    .rk()
                    .verify(&spend_message, action.authorization())
                    .is_err()
            );
            matching_failed += 1;
        } else {
            assert!(
                action
                    .rk()
                    .verify(&spend_message, action.authorization())
                    .is_ok()
            );
            untouched_ok += 1;
        }
    }
    assert_eq!(matching_failed, 1);
    assert_eq!(untouched_ok, 1);
    assert!(
        spend_ironwood
            .binding_validating_key()
            .verify(
                &spend_message,
                spend_ironwood.authorization().binding_signature()
            )
            .is_ok()
    );

    let mut binding_mutated = fixture.encoded.clone();
    flip_unique(binding_mutated.as_mut_slice(), &binding_sig);
    let binding_decoded = decode_complete(binding_mutated.as_slice(), BranchId::Nu6_3);
    let binding_context = require_context(binding_decoded);
    let binding_message = binding_context.shielded_sighash();
    assert!(
        binding_message == fixture.pczt_sighash,
        "binding-mutated context message must remain the independently captured PCZT sighash"
    );
    let binding_ironwood = binding_context
        .data()
        .ironwood_bundle()
        .expect("binding-mutated ironwood data");
    assert_eq!(binding_ironwood.actions().len(), 2);
    let mut untouched_spend_ok = 0usize;
    for action in binding_ironwood.actions() {
        assert!(
            action
                .rk()
                .verify(&binding_message, action.authorization())
                .is_ok()
        );
        untouched_spend_ok += 1;
    }
    assert_eq!(untouched_spend_ok, 2);
    assert!(
        binding_ironwood
            .binding_validating_key()
            .verify(
                &binding_message,
                binding_ironwood.authorization().binding_signature()
            )
            .is_err()
    );
}

#[test]
fn keeps_proof_verification_independent_of_sighash() {
    let fixture = signed_fixture("wal009-proof");
    let decoded = decode_complete(fixture.encoded.as_slice(), BranchId::Nu6_3);
    let (proof, vk) = {
        let ironwood = decoded
            .ironwood_bundle()
            .expect("nonempty ironwood data");
        assert_eq!(ironwood.actions().len(), 2);
        let proof = ironwood.authorization().proof().as_ref();
        assert!(!proof.is_empty());
        unique_occurrence(fixture.encoded.as_slice(), proof);
        let vk = VerifyingKey::build(ironwood.bundle_version().circuit_version());
        assert!(ironwood.verify_proof(&vk).is_ok());
        (proof.to_vec(), vk)
    };

    let context = require_context(decoded);
    assert_eq!(context.shielded_sighash(), fixture.pczt_sighash);

    let mut proof_mutated = fixture.encoded.clone();
    flip_unique(proof_mutated.as_mut_slice(), &proof);
    let mutated = decode_complete(proof_mutated.as_slice(), BranchId::Nu6_3);
    let mutated_context = require_context(mutated);
    assert_eq!(mutated_context.shielded_sighash(), fixture.pczt_sighash);
    let mutated_ironwood = mutated_context
        .data()
        .ironwood_bundle()
        .expect("proof-mutated ironwood data");
    assert_eq!(mutated_ironwood.actions().len(), 2);
    let message = mutated_context.shielded_sighash();
    let mut spend_ok = 0usize;
    for action in mutated_ironwood.actions() {
        assert!(
            action
                .rk()
                .verify(&message, action.authorization())
                .is_ok()
        );
        spend_ok += 1;
    }
    assert_eq!(spend_ok, 2);
    assert!(
        mutated_ironwood
            .binding_validating_key()
            .verify(
                &message,
                mutated_ironwood.authorization().binding_signature()
            )
            .is_ok()
    );
    assert!(mutated_ironwood.verify_proof(&vk).is_err());
}

#[test]
fn decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes() {
    let fixture = signed_fixture("wal009-decoded-effects");
    let decoded = decode_complete(fixture.encoded.as_slice(), BranchId::Nu6_3);
    assert!(decoded.transparent_bundle().is_none());
    assert!(decoded.sapling_bundle().is_none());
    assert!(decoded.orchard_bundle().is_none());
    let recovered_txid = decoded.txid().to_string();

    let external_ovk = fixture.orchard_fvk.to_ovk(Scope::External);
    let internal_ovk = fixture.orchard_fvk.to_ovk(Scope::Internal);
    let internal_ivk = fixture.orchard_fvk.to_ivk(Scope::Internal);
    let ironwood = decoded
        .ironwood_bundle()
        .expect("nonempty ironwood data");
    let action_count = ironwood.actions().len();
    let value_balance = i64::from(*ironwood.value_balance());
    assert!(value_balance >= 0, "decoded fee must be nonnegative");
    let recovered_fee = u64::try_from(value_balance).expect("nonnegative ironwood value balance");
    assert_eq!(recovered_fee, 10_000);

    let mut accounted = 0usize;
    let mut payment = None;
    let mut change = None;
    for index in 0..action_count {
        let external_outgoing = ironwood.recover_output_with_ovk(index, &external_ovk);
        let internal_outgoing = ironwood.recover_output_with_ovk(index, &internal_ovk);
        let internal_incoming = ironwood.decrypt_output_with_key(index, &internal_ivk);
        if let (
            Some((left_note, left_address, left_memo)),
            Some((right_note, right_address, right_memo)),
        ) = (external_outgoing.as_ref(), internal_outgoing.as_ref())
        {
            assert!(left_note.value() == right_note.value());
            assert!(left_address == right_address);
            assert!(left_memo == right_memo);
        }
        let outgoing = external_outgoing.or(internal_outgoing);
        if let (
            Some((left_note, left_address, left_memo)),
            Some((right_note, right_address, right_memo)),
        ) = (outgoing.as_ref(), internal_incoming.as_ref())
        {
            assert!(left_note.value() == right_note.value());
            assert!(left_address == right_address);
            assert!(left_memo == right_memo);
        }
        assert!(
            outgoing.is_some() || internal_incoming.is_some(),
            "each action must be recovered once"
        );
        accounted += 1;
        if let Some((note, address, memo)) = internal_incoming {
            assert!(change.is_none(), "ambiguous internal change");
            let value = note.value().inner();
            assert!(value > 0, "internal change must be positive");
            assert!(fixture.orchard_fvk.scope_for_address(&address) == Some(Scope::Internal));
            change = Some((address, value, memo));
        } else if let Some((note, address, memo)) = outgoing {
            assert!(payment.is_none(), "ambiguous external payment");
            assert!(fixture.orchard_fvk.scope_for_address(&address) != Some(Scope::Internal));
            payment = Some((address, note.value().inner(), memo));
        }
    }
    assert_eq!(accounted, 2);
    assert_eq!(accounted, action_count);
    let (payment_address, recovered_amount, payment_memo) =
        payment.expect("one external payment");
    let (change_address, change_value, _change_memo) = change.expect("one internal change");
    assert_eq!(recovered_amount, AMOUNT_ZAT);
    assert!(change_value > 0);
    assert!(payment_address != change_address);

    let stripped = MemoPlaintext::from_memo(payment_memo);
    assert!(stripped.as_stripped_bytes() == MEMO.as_bytes());
    let recovered_memo_sha256 = sha256_hex(stripped.as_stripped_bytes());
    let recovered_receiver_bytes = payment_address.to_raw_address_bytes().to_vec();
    let recovered_receiver = UnifiedAddress::from_receivers(Some(payment_address), None, None)
        .expect("orchard-only payment unified address")
        .encode(&fixture.network.upstream());

    assert!(fixture.inspection.destination_receiver_bytes == recovered_receiver_bytes);
    assert!(fixture.inspection.destination == recovered_receiver);
    assert_eq!(fixture.inspection.amount_zat, recovered_amount.to_string());
    assert_eq!(fixture.inspection.fee_zat, recovered_fee.to_string());
    assert_eq!(fixture.inspection.memo_sha256, recovered_memo_sha256);

    let effects = match super::independently_verify(
        decode_complete(fixture.encoded.as_slice(), BranchId::Nu6_3),
        &fixture.inspection,
        &fixture.inspection,
        "12000",
        None,
        &fixture.authority,
    ) {
        Ok(effects) => effects,
        Err(error) => panic!("positive control {}", error.code()),
    };
    assert!(effects.external_receiver_bytes == recovered_receiver_bytes);
    assert!(effects.external_receiver == recovered_receiver);
    assert_eq!(effects.external_amount_zat, recovered_amount.to_string());
    assert_eq!(effects.fee_zat, recovered_fee.to_string());
    assert_eq!(effects.memo_sha256, recovered_memo_sha256);
    assert_eq!(effects.ironwood_external_outputs, 1);
    assert_eq!(effects.ironwood_internal_change_outputs, 1);
    assert_eq!(effects.derived_transaction_id, recovered_txid);

    let payment_raw = payment_address.to_raw_address_bytes();
    let change_raw = change_address.to_raw_address_bytes();
    let other_address = (0u32..256)
        .map(|index| fixture.orchard_fvk.address_at(index, Scope::External))
        .find(|address| {
            let raw = address.to_raw_address_bytes();
            raw != payment_raw && raw != change_raw
        })
        .expect("bounded diversifier search found another orchard address");
    assert!(other_address != payment_address);
    assert!(other_address != change_address);
    let other_receiver = UnifiedAddress::from_receivers(Some(other_address), None, None)
        .expect("orchard-only mutated unified address")
        .encode(&fixture.network.upstream());
    let other_receiver_bytes = other_address.to_raw_address_bytes().to_vec();

    let bytes = fixture.encoded.as_slice();
    let original = &fixture.inspection;
    let run = |inspection: PcztInspection| {
        let actual = inspection.clone();
        let expected = inspection;
        assert!(actual == expected, "actual and expected metadata disagree");
        match super::independently_verify(
            decode_complete(bytes, BranchId::Nu6_3),
            &actual,
            &expected,
            "12000",
            None,
            &fixture.authority,
        ) {
            Ok(_) => None,
            Err(error) => Some(error.code()),
        }
    };

    let mut receiver_inspection = original.clone();
    receiver_inspection.destination = other_receiver;
    receiver_inspection.destination_receiver_bytes = other_receiver_bytes;
    assert!(receiver_inspection.destination_receiver_bytes != recovered_receiver_bytes);
    assert!(receiver_inspection.destination != original.destination);
    let receiver = run(receiver_inspection);

    let mut amount_inspection = original.clone();
    amount_inspection.amount_zat = "100000001".to_owned();
    assert!(amount_inspection.amount_zat != recovered_amount.to_string());
    let amount = run(amount_inspection);

    let mut memo_inspection = original.clone();
    memo_inspection.memo_sha256 = sha256_hex(b"tea");
    assert!(memo_inspection.memo_sha256 != recovered_memo_sha256);
    let memo = run(memo_inspection);

    let mut fee_inspection = original.clone();
    fee_inspection.fee_zat = "10001".to_owned();
    assert!(fee_inspection.fee_zat != recovered_fee.to_string());
    let fee = run(fee_inspection);

    let mut inputs_inspection = original.clone();
    inputs_inspection.ironwood_inputs = 0;
    assert!(inputs_inspection.ironwood_inputs != original.ironwood_inputs);
    let inputs = run(inputs_inspection);

    let mut outputs_inspection = original.clone();
    outputs_inspection.ironwood_outputs = 1;
    assert!(outputs_inspection.ironwood_outputs != original.ironwood_outputs);
    let outputs = run(outputs_inspection);

    let mut pool_inspection = original.clone();
    pool_inspection.spend_pool = "orchard".to_owned();
    assert!(pool_inspection.spend_pool != original.spend_pool);
    let pool = run(pool_inspection);

    let mut transparent_inspection = original.clone();
    transparent_inspection.has_transparent_bundle = true;
    assert!(transparent_inspection.has_transparent_bundle != original.has_transparent_bundle);
    let transparent = run(transparent_inspection);

    let mut sapling_inspection = original.clone();
    sapling_inspection.has_sapling_bundle = true;
    assert!(sapling_inspection.has_sapling_bundle != original.has_sapling_bundle);
    let sapling = run(sapling_inspection);

    let mut orchard_output_inspection = original.clone();
    orchard_output_inspection.has_orchard_output_bundle = true;
    assert!(
        orchard_output_inspection.has_orchard_output_bundle != original.has_orchard_output_bundle
    );
    let orchard_output = run(orchard_output_inspection);

    let outcomes = [
        ("receiver", receiver),
        ("amount", amount),
        ("memo", memo),
        ("fee", fee),
        ("real Ironwood input count", inputs),
        ("Ironwood output count", outputs),
        ("spend pool", pool),
        ("has_transparent_bundle", transparent),
        ("has_sapling_bundle", sapling),
        ("has_orchard_output_bundle", orchard_output),
    ];
    let unexpected: Vec<String> = outcomes
        .iter()
        .filter(|(_, code)| *code != Some("INTENT_MISMATCH"))
        .map(|(label, code)| match code {
            Some(code) => format!("{label}:{code}"),
            None => format!("{label}:ok"),
        })
        .collect();
    assert!(unexpected.is_empty(), "{}", unexpected.join(","));
}
