use std::collections::BTreeSet;
use std::sync::{Arc, Barrier};
use std::thread;

use bitbook_wallet_broker::vault::SecretBytes;
use bitbook_wallet_broker::zec::test_support::{
    AddressFault, FrozenFixture, RecordingWipes, TestAccount, TestStateRoot, decode_unified_address,
};
use bitbook_wallet_broker::zec::{
    AccountId, FreshReceiverV1, LocalNetwork, MAX_DIVERSIFIER_INDEX, MAX_ISSUANCE_SEQUENCE,
    Network, ZecError,
};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const OTHER_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const FIXTURE_DIR: &str = "tests/fixtures/zec";
const SYNTHETIC_SEED: [u8; 32] = [0; 32];

fn local_network() -> LocalNetwork {
    LocalNetwork::new(100, 102, 106).expect("reviewed pre/at/post NU6.3 schedule")
}

fn account(label: &str) -> TestAccount {
    TestAccount::bootstrap(
        TestStateRoot::fresh(label),
        AccountId::parse(ACCOUNT).unwrap(),
        Network::Local(local_network()),
        SecretBytes::new(SYNTHETIC_SEED.to_vec()).unwrap(),
    )
    .unwrap()
}

fn fixture() -> FrozenFixture {
    FrozenFixture::open(FIXTURE_DIR).expect("reviewer-frozen WAL-006 fixture")
}

fn assert_orchard_protocol_only(receiver: &FreshReceiverV1) {
    let decoded = decode_unified_address(&receiver.receiver).unwrap();
    assert_eq!(decoded.network, receiver.network);
    assert_eq!(decoded.receivers.len(), 1);
    assert!(decoded.receivers[0].is_orchard_protocol());
    assert!(!decoded.receivers[0].is_p2pkh());
    assert!(!decoded.receivers[0].is_p2sh());
    assert!(!decoded.receivers[0].is_sapling());
    assert!(!decoded.receivers[0].is_tex());
    assert!(!decoded.receivers[0].is_unknown());
}

#[test]
fn fresh_receiver_decodes_to_exactly_one_orchard_protocol_receiver() {
    let mut wallet = account("address-composition");
    let receiver = wallet.fresh_receiver(1_700_000_000).unwrap();
    assert_eq!(receiver.account_id.as_str(), ACCOUNT);
    assert_eq!(receiver.network.as_str(), "zec-local");
    assert_eq!(receiver.diversifier_index, "0");
    assert_eq!(receiver.issued_at_sequence, "1");
    assert_eq!(
        receiver.receiver,
        fixture().manifest().expected.orchard_only_receiver
    );
    assert_orchard_protocol_only(&receiver);
}

#[test]
fn receiver_issuance_is_monotonic_durable_and_viewing_only_after_reopen() {
    let mut wallet = account("address-reopen");
    let first = wallet.fresh_receiver(10).unwrap();
    let second = wallet.fresh_receiver(11).unwrap();
    assert_ne!(first.receiver, second.receiver);
    assert_eq!(
        (
            first.diversifier_index.as_str(),
            first.issued_at_sequence.as_str()
        ),
        ("0", "1")
    );
    assert_eq!(
        (
            second.diversifier_index.as_str(),
            second.issued_at_sequence.as_str()
        ),
        ("1", "2")
    );

    let root = wallet.close().unwrap();
    let mut reopened = TestAccount::open_viewing(root, AccountId::parse(ACCOUNT).unwrap()).unwrap();
    assert!(!reopened.has_spending_authority());
    let third = reopened.fresh_receiver(12).unwrap();
    assert_eq!(
        (
            third.diversifier_index.as_str(),
            third.issued_at_sequence.as_str()
        ),
        ("2", "3")
    );
    assert_orchard_protocol_only(&third);
}

#[test]
fn two_concurrent_issuers_serialize_one_account_without_duplicates() {
    let wallet = Arc::new(account("address-concurrent"));
    let barrier = Arc::new(Barrier::new(3));
    let mut joins = Vec::new();
    for now in [20, 21] {
        let wallet = Arc::clone(&wallet);
        let barrier = Arc::clone(&barrier);
        joins.push(thread::spawn(move || {
            barrier.wait();
            wallet.fresh_receiver_concurrent(now).unwrap()
        }));
    }
    barrier.wait();
    let issued: Vec<FreshReceiverV1> = joins.into_iter().map(|join| join.join().unwrap()).collect();
    let indices: BTreeSet<&str> = issued
        .iter()
        .map(|item| item.diversifier_index.as_str())
        .collect();
    let sequences: BTreeSet<&str> = issued
        .iter()
        .map(|item| item.issued_at_sequence.as_str())
        .collect();
    let receivers: BTreeSet<&str> = issued.iter().map(|item| item.receiver.as_str()).collect();
    assert_eq!(indices, BTreeSet::from(["0", "1"]));
    assert_eq!(sequences, BTreeSet::from(["1", "2"]));
    assert_eq!(receivers.len(), 2);
    for receiver in &issued {
        assert_orchard_protocol_only(receiver);
    }
}

#[test]
fn coupled_receiver_state_write_failure_returns_nothing_and_advances_neither_record() {
    for fault in [
        AddressFault::ReceiverRowWrite,
        AddressFault::SequenceRowWrite,
        AddressFault::CommitSync,
    ] {
        let mut wallet = account("address-write-fault");
        wallet.arm_address_fault(fault);
        let error = wallet.fresh_receiver(30).unwrap_err();
        assert!(matches!(error.code(), "STATE_CORRUPT" | "INTERNAL"));
        assert_eq!(wallet.inspect_receiver_state().last_diversifier_index, None);
        assert_eq!(wallet.inspect_receiver_state().issued_at_sequence, "0");
        wallet.clear_address_fault();
        let issued = wallet.fresh_receiver(31).unwrap();
        assert_eq!(
            (
                issued.diversifier_index.as_str(),
                issued.issued_at_sequence.as_str()
            ),
            ("0", "1")
        );
    }
}

#[test]
fn account_network_and_mainnet_validation_precede_database_or_derivation() {
    for (length, accepted) in [(31, false), (32, true), (33, false)] {
        let candidate = "a".repeat(length);
        assert_eq!(
            AccountId::parse(&candidate).is_ok(),
            accepted,
            "length {length}"
        );
    }
    for invalid in [
        "",
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeefg",
        "../112233445566778899aabbccddeeff",
    ] {
        assert_eq!(AccountId::parse(invalid).unwrap_err().code(), "SCHEMA");
    }
    let root = TestStateRoot::fresh("address-mainnet");
    let error = TestAccount::bootstrap_product_network(
        root.clone(),
        ACCOUNT,
        "zec-mainnet",
        SecretBytes::new(SYNTHETIC_SEED.to_vec()).unwrap(),
    )
    .unwrap_err();
    assert_eq!(error.code(), "NETWORK_DISABLED");
    assert!(root.operations().is_empty());

    let mut wallet = account("address-binding");
    assert_eq!(
        wallet
            .fresh_receiver_for(OTHER_ACCOUNT, 40)
            .unwrap_err()
            .code(),
        "SCHEMA"
    );
    assert_eq!(wallet.inspect_receiver_state().issued_at_sequence, "0");
}

#[test]
fn receiver_limits_cover_immediate_below_at_and_above_without_wrap() {
    let mut below = account("address-limit-below");
    below
        .set_receiver_state_for_test(MAX_DIVERSIFIER_INDEX - 1, MAX_ISSUANCE_SEQUENCE - 1)
        .unwrap();
    let last = below.fresh_receiver(50).unwrap();
    assert_eq!(last.diversifier_index, MAX_DIVERSIFIER_INDEX.to_string());
    assert_eq!(last.issued_at_sequence, MAX_ISSUANCE_SEQUENCE.to_string());

    let mut at = account("address-limit-at");
    at.set_receiver_state_for_test(MAX_DIVERSIFIER_INDEX, MAX_ISSUANCE_SEQUENCE)
        .unwrap();
    assert_eq!(at.fresh_receiver(51).unwrap_err().code(), "LIMIT");
    assert_eq!(
        at.inspect_receiver_state().issued_at_sequence,
        MAX_ISSUANCE_SEQUENCE.to_string()
    );

    for (index, sequence) in [
        (u64::MAX, MAX_ISSUANCE_SEQUENCE),
        (MAX_DIVERSIFIER_INDEX, u64::MAX),
    ] {
        let mut above = account("address-limit-above");
        assert_eq!(
            above
                .set_receiver_state_for_test(index, sequence)
                .unwrap_err()
                .code(),
            "LIMIT"
        );
    }
}

#[test]
fn seed_is_wiped_on_success_error_cancellation_replacement_unwind_and_drop() {
    let wipes = RecordingWipes::shared();
    for exit in [
        "success",
        "error",
        "cancellation",
        "replacement",
        "unwind",
        "drop",
    ] {
        let result = TestAccount::exercise_seed_exit(
            TestStateRoot::fresh("address-seed-wipe"),
            AccountId::parse(ACCOUNT).unwrap(),
            Network::Local(local_network()),
            SecretBytes::new(SYNTHETIC_SEED.to_vec()).unwrap(),
            exit,
            wipes.clone(),
        );
        if exit == "success" || exit == "drop" {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
        }
        assert!(wipes.contains_post_wipe("zec-seed", SYNTHETIC_SEED.len(), exit));
    }
}

#[test]
fn unsupported_receiver_composition_never_falls_back() {
    let mut wallet = account("address-no-fallback");
    for composition in [
        "p2pkh",
        "p2sh",
        "sapling",
        "orchard+p2pkh",
        "orchard+unknown",
    ] {
        let error: ZecError = wallet
            .request_receiver_composition_for_test(composition)
            .unwrap_err();
        assert!(matches!(
            error.code(),
            "TRANSPARENT_DOWNGRADE" | "PROTOCOL_INCOMPATIBLE"
        ));
        assert_eq!(wallet.inspect_receiver_state().issued_at_sequence, "0");
    }
}
