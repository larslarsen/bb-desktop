use std::collections::BTreeSet;
use std::os::unix::fs::PermissionsExt;

use bitbook_wallet_broker::vault::SecretBytes;
use bitbook_wallet_broker::zec::test_support::{
    SecretCanary, SecretClass, StoreEntryKind, TestAccount, TestStateRoot,
};
use bitbook_wallet_broker::zec::{AccountId, LocalNetwork, Network, StoreFault};
use sha2::{Digest, Sha256};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const OTHER_ACCOUNT: &str = "ffeeddccbbaa99887766554433221100";
const SYNTHETIC_SEED: [u8; 32] = [0; 32];
const SEED_CANARY: &[u8] = b"CANARY_WAL006_SEED_BYTES_NEVER_IN_SQLITE";
const MNEMONIC_CANARY: &[u8] = b"INVALID_SYNTHETIC_MNEMONIC_@@@_NOT_BIP39_WAL006";
const USK_CANARY: &[u8] = b"CANARY_WAL006_UNIFIED_SPENDING_KEY_NEVER_IN_SQLITE";
const DERIVED_SPEND_MATERIAL_CANARY: &[u8] = b"CANARY_WAL006_DERIVED_SPEND_6d1f";
const VAULT_PLAINTEXT_CANARY: &[u8] = b"CANARY_WAL006_VAULT_PLAINTEXT_6d20";
const PASSPHRASE_CANARY: &[u8] = b"CANARY_WAL006_PASSPHRASE_NEVER_IN_SQLITE";
const PCZT_CANARY: &[u8] = b"CANARY_WAL006_PCZT_NEVER_IN_SQLITE";
const AUTHORIZATION_SESSION_CANARY: &[u8] = b"CANARY_WAL006_AUTHORIZATION_SESSION_NEVER_IN_SQLITE";

fn network() -> Network {
    Network::Local(LocalNetwork::new(100, 102, 106).unwrap())
}

fn bootstrap(label: &str) -> TestAccount {
    TestAccount::bootstrap(
        TestStateRoot::fresh(label),
        AccountId::parse(ACCOUNT).unwrap(),
        network(),
        SecretBytes::new(SYNTHETIC_SEED.to_vec()).unwrap(),
    )
    .unwrap()
}

fn sha256_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(digest.len() * 2);
    for byte in digest {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}

#[test]
fn sqlite_paths_are_closed_account_network_derived_and_linux_private() {
    let wallet = bootstrap("store-paths");
    let paths = wallet.inspect_paths();
    assert_eq!(paths.relative_account_dir, format!("zec-local/{ACCOUNT}"));
    assert_eq!(paths.wallet_db_file, "wallet.sqlite3");
    assert_eq!(paths.compact_cache_file, "compact.sqlite3");
    assert!(!paths.relative_account_dir.starts_with('/'));
    assert!(!paths.relative_account_dir.contains(".."));
    assert_eq!(
        std::fs::metadata(paths.absolute_account_dir())
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    for file in [paths.absolute_wallet_db(), paths.absolute_compact_cache()] {
        let metadata = std::fs::symlink_metadata(file).unwrap();
        assert!(metadata.file_type().is_file());
        assert!(!metadata.file_type().is_symlink());
        assert_eq!(metadata.permissions().mode() & 0o777, 0o600);
    }
}

#[test]
fn initialization_and_reopen_bind_exact_account_network_and_schema() {
    let wallet = bootstrap("store-binding");
    let before = wallet.inspect_store().unwrap();
    assert_eq!(before.account_id, ACCOUNT);
    assert_eq!(before.network, "zec-local");
    assert_eq!(before.schema_version, "1");
    let root = wallet.close().unwrap();

    let reopened =
        TestAccount::open_viewing(root.clone(), AccountId::parse(ACCOUNT).unwrap()).unwrap();
    assert_eq!(reopened.inspect_store().unwrap(), before);
    assert_eq!(
        TestAccount::open_viewing(root.clone(), AccountId::parse(OTHER_ACCOUNT).unwrap())
            .unwrap_err()
            .code(),
        "STATE_CORRUPT"
    );
    assert_eq!(
        TestAccount::open_viewing_with_network(
            root,
            AccountId::parse(ACCOUNT).unwrap(),
            Network::Testnet
        )
        .unwrap_err()
        .code(),
        "STATE_CORRUPT"
    );
}

#[test]
fn sqlite_schema_and_rows_contain_viewing_state_but_no_spend_secrets() {
    let mut wallet = bootstrap("store-secret-inspection");
    let canaries = [
        SecretCanary::new(SecretClass::Seed, SEED_CANARY),
        SecretCanary::new(SecretClass::Mnemonic, MNEMONIC_CANARY),
        SecretCanary::new(SecretClass::UnifiedSpendingKey, USK_CANARY),
        SecretCanary::new(
            SecretClass::DerivedSpendingMaterial,
            DERIVED_SPEND_MATERIAL_CANARY,
        ),
        SecretCanary::new(SecretClass::VaultPlaintext, VAULT_PLAINTEXT_CANARY),
        SecretCanary::new(SecretClass::Passphrase, PASSPHRASE_CANARY),
        SecretCanary::new(SecretClass::RawPczt, PCZT_CANARY),
        SecretCanary::new(
            SecretClass::AuthorizationSession,
            AUTHORIZATION_SESSION_CANARY,
        ),
    ];
    let receipt = wallet
        .install_nonpersistent_canaries_for_test(&canaries)
        .unwrap();
    assert!(
        canaries
            .iter()
            .all(|canary| canary.bytes().iter().any(|byte| *byte != 0))
    );
    assert!(receipt.is_closed());
    assert_eq!(
        receipt.class_names(),
        [
            "seed",
            "mnemonic",
            "unified-spending-key",
            "derived-spending-material",
            "vault-plaintext",
            "passphrase",
            "raw-pczt",
            "authorization-session",
        ]
    );
    assert_eq!(receipt.commitments().len(), canaries.len());
    let mut commitment_hashes = BTreeSet::new();
    for (commitment, canary) in receipt.commitments().iter().zip(&canaries) {
        assert_eq!(commitment.class, canary.class().as_str());
        assert_eq!(commitment.byte_length, canary.bytes().len());
        assert_eq!(commitment.sha256, sha256_hex(canary.bytes()));
        assert!(commitment_hashes.insert(commitment.sha256.as_str()));
    }

    let inspection = wallet.inspect_sqlite_for_test().unwrap();
    assert!(inspection.tables.contains(&"accounts".to_owned()));
    assert!(inspection.tables.contains(&"addresses".to_owned()));
    assert!(inspection.tables.contains(&"scan_queue".to_owned()));
    assert!(inspection.columns.iter().any(|column| column == "ufvk"));
    for forbidden in [
        "seed",
        "mnemonic",
        "usk",
        "spending_key",
        "passphrase",
        "vault_plaintext",
        "authorization_session",
        "raw_pczt",
        "prepared_pczt",
    ] {
        assert!(
            !inspection
                .columns
                .iter()
                .any(|column| column.eq_ignore_ascii_case(forbidden))
        );
    }
    assert!(inspection.decoded_row_count() > 0);
    assert!(inspection.decoded_value_kinds().contains(&"text"));
    assert!(inspection.decoded_value_kinds().contains(&"blob"));
    for canary in [
        SEED_CANARY,
        MNEMONIC_CANARY,
        USK_CANARY,
        DERIVED_SPEND_MATERIAL_CANARY,
        VAULT_PLAINTEXT_CANARY,
        PASSPHRASE_CANARY,
        PCZT_CANARY,
        AUTHORIZATION_SESSION_CANARY,
    ] {
        assert!(!inspection.contains_decoded_row_bytes(canary));
    }

    // The all-zero seed is the single reviewed synthetic seed shared with the upstream fixture.
    // It is not a raw-page canary because ordinary SQLite pages contain long zero runs; decoded
    // row/column inspection above proves there is no spending-secret persistence slot.
    let bytes = std::fs::read(wallet.inspect_paths().absolute_wallet_db()).unwrap();
    for canary in [
        SEED_CANARY,
        MNEMONIC_CANARY,
        USK_CANARY,
        DERIVED_SPEND_MATERIAL_CANARY,
        VAULT_PLAINTEXT_CANARY,
        PASSPHRASE_CANARY,
        PCZT_CANARY,
        AUTHORIZATION_SESSION_CANARY,
    ] {
        assert!(!bytes.windows(canary.len()).any(|window| window == canary));
    }
    assert!(
        !wallet
            .open_viewing_context()
            .unwrap()
            .has_spending_authority()
    );
}

#[test]
fn schema_migration_is_atomic_across_write_sync_and_commit_failures() {
    for fault in [
        StoreFault::MigrationWrite,
        StoreFault::MigrationSync,
        StoreFault::MigrationCommit,
    ] {
        let mut wallet = bootstrap("store-migration-fault");
        wallet.install_previous_schema_for_test().unwrap();
        let before = std::fs::read(wallet.inspect_paths().absolute_wallet_db()).unwrap();
        wallet.arm_store_fault(fault);
        assert_eq!(
            wallet.reopen_and_migrate().unwrap_err().code(),
            "STATE_CORRUPT"
        );
        let after = std::fs::read(wallet.inspect_paths().absolute_wallet_db()).unwrap();
        assert_eq!(after, before);
        assert_eq!(wallet.inspect_store().unwrap().schema_version, "0");
    }
}

#[test]
fn symlink_nonregular_and_wrong_mode_state_are_rejected_without_replacement() {
    for kind in [
        StoreEntryKind::Symlink,
        StoreEntryKind::Directory,
        StoreEntryKind::Fifo,
        StoreEntryKind::BlockDevice,
        StoreEntryKind::CharacterDevice,
        StoreEntryKind::RegularWrongMode,
        StoreEntryKind::RegularWrongOwner,
    ] {
        let root = TestStateRoot::with_hostile_wallet_entry("store-hostile-entry", ACCOUNT, kind);
        let marker = root.entry_marker().to_vec();
        let error = TestAccount::open_viewing(root.clone(), AccountId::parse(ACCOUNT).unwrap())
            .unwrap_err();
        assert_eq!(error.code(), "STATE_CORRUPT");
        assert_eq!(root.entry_marker(), marker.as_slice());
        assert!(
            !root
                .operations()
                .iter()
                .any(|operation| operation == "replace")
        );
    }
}

#[test]
fn corrupt_wrong_schema_and_truncated_sqlite_fail_closed_without_empty_recreation() {
    for mutation in [
        "truncated-header",
        "invalid-page-size",
        "unknown-schema",
        "wrong-network",
        "wrong-account",
    ] {
        let mut wallet = bootstrap("store-corrupt");
        wallet.mutate_sqlite_for_test(mutation).unwrap();
        let before = std::fs::read(wallet.inspect_paths().absolute_wallet_db()).unwrap();
        let root = wallet.close_without_validation();
        let error = TestAccount::open_viewing(root.clone(), AccountId::parse(ACCOUNT).unwrap())
            .unwrap_err();
        assert_eq!(error.code(), "STATE_CORRUPT", "mutation {mutation}");
        assert_eq!(std::fs::read(root.wallet_db_path(ACCOUNT)).unwrap(), before);
        assert!(
            !root
                .operations()
                .iter()
                .any(|operation| operation == "initialize-empty")
        );
    }
}

#[test]
fn failed_write_file_sync_and_directory_sync_never_report_durable_state() {
    for fault in [
        StoreFault::Write,
        StoreFault::FileSync,
        StoreFault::DirectorySync,
    ] {
        let mut wallet = bootstrap("store-durability-fault");
        let before = wallet.inspect_store().unwrap();
        wallet.arm_store_fault(fault);
        assert!(matches!(
            wallet.persist_checkpoint_for_test(103).unwrap_err().code(),
            "STATE_CORRUPT" | "INTERNAL"
        ));
        let after = wallet.inspect_store().unwrap();
        assert_eq!(after.scan_tip, before.scan_tip);
        assert_eq!(after.receiver_sequence, before.receiver_sequence);
    }
}

#[test]
fn store_limits_cover_immediate_below_at_and_above_before_allocation() {
    let mut wallet = bootstrap("store-limits");
    let limit = bitbook_wallet_broker::zec::MAX_FIXTURE_MANIFEST_BYTES;
    for (length, accepted) in [(limit - 1, true), (limit, true), (limit + 1, false)] {
        wallet.reset_allocation_observer();
        let result = wallet.read_manifest_sized_for_test(length);
        assert_eq!(result.is_ok(), accepted);
        if accepted {
            assert_eq!(wallet.observed_allocation_bytes(), Some(length));
        } else {
            assert_eq!(result.unwrap_err().code(), "LIMIT");
            assert_eq!(wallet.observed_allocation_bytes(), None);
        }
    }
}
