use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use bitbook_wallet_broker::hygiene::{
    DiagnosticEvent, SecretOperation, run_secret_operation,
};
use bitbook_wallet_broker::vault::{
    EntropyPort, SecretBytes, VaultError, VaultMetadata, WipeEvent, WipeObserver,
    Asset, Network, seal_vault,
};

const SECRET: &[u8] = b"CANARY_WAL004_SECRET_BYTES_7f3a";
const PASSPHRASE: &[u8] = b"CANARY_WAL004_PASSPHRASE_4c2d";

#[derive(Clone, Default)]
struct SharedWipes(Rc<RefCell<Vec<WipeEvent>>>);

impl WipeObserver for SharedWipes {
    fn observe(&mut self, event: WipeEvent) {
        self.0.borrow_mut().push(event);
    }
}

struct Entropy;

impl EntropyPort for Entropy {
    fn fill(&mut self, label: &'static str, output: &mut [u8]) -> Result<(), VaultError> {
        match label {
            "vault-salt" => output.fill(0x51),
            "vault-nonce" => output.fill(0x27),
            _ => return Err(VaultError::entropy()),
        }
        Ok(())
    }
}

fn assert_all_zero(log: &SharedWipes, label: &str, length: usize) {
    assert!(log.0.borrow().iter().any(|event| {
        event.label == label && event.length == length && event.all_zero
    }), "missing post-zeroize observation for {label}");
}

#[test]
fn debug_display_errors_logs_snapshots_and_evidence_omit_secret_canaries() {
    let secret = SecretBytes::new(SECRET.to_vec()).unwrap();
    let error = VaultError::locked();
    let diagnostic = DiagnosticEvent::new("vault.open", "00112233445566778899aabbccddeeff", error.code()).unwrap();
    let observable = format!(
        "debug={secret:?};display={};error={error:?};diagnostic={diagnostic:?};snapshot={:?};evidence={:?}",
        secret,
        diagnostic.snapshot(),
        diagnostic.evidence_fields(),
    );
    assert!(!observable.contains("CANARY"));
    assert!(!observable.contains("7f3a"));
    assert!(!observable.contains("4c2d"));
    assert_eq!(diagnostic.field_names(), ["operation", "account_id", "code"]);
}

#[test]
fn diagnostic_operations_and_codes_are_closed_to_exact_reviewed_values() {
    const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
    for operation in [
        "vault.seal",
        "vault.open",
        "vault.store",
        "vault.export",
        "vault.restore",
        "session.lock",
        "native.unlock",
        "native.export",
        "native.restore",
    ] {
        DiagnosticEvent::new(operation, ACCOUNT, "LOCKED").unwrap();
    }
    for code in [
        "ENTROPY",
        "LOCKED",
        "SCHEMA",
        "LIMIT",
        "WRONG_NETWORK",
        "ACCOUNT_BUSY",
        "NOT_FOUND",
        "ALREADY_EXISTS",
        "UNAVAILABLE",
        "REPLAY",
        "STATE_CORRUPT",
        "TIMEOUT",
        "UNAUTH",
    ] {
        DiagnosticEvent::new("vault.open", ACCOUNT, code).unwrap();
    }

    assert_eq!(
        DiagnosticEvent::new("vault.unknown", ACCOUNT, "LOCKED")
            .unwrap_err()
            .code(),
        "SCHEMA"
    );
    assert_eq!(
        DiagnosticEvent::new("vault.open", ACCOUNT, "UNKNOWN")
            .unwrap_err()
            .code(),
        "SCHEMA"
    );
}

#[test]
fn diagnostic_fields_reject_malformed_accounts_and_secret_canaries() {
    const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
    for account in [
        "",
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeefg",
        "00112233445566778899aabbccddee",
        "../112233445566778899aabbccddeeff",
    ] {
        assert_eq!(
            DiagnosticEvent::new("vault.open", account, "LOCKED")
                .unwrap_err()
                .code(),
            "SCHEMA"
        );
    }

    for result in [
        DiagnosticEvent::new("CANARY_WAL004_SECRET_OPERATION", ACCOUNT, "LOCKED"),
        DiagnosticEvent::new(
            "vault.open",
            "CANARY_WAL004_SECRET_ACCOUNT_ID",
            "LOCKED",
        ),
        DiagnosticEvent::new("vault.open", ACCOUNT, "CANARY_WAL004_SECRET_CODE"),
    ] {
        assert_eq!(result.unwrap_err().code(), "SCHEMA");
    }
}

#[test]
fn successful_operation_reports_actual_post_wipe_zeroes() {
    let log = SharedWipes::default();
    let mut observer = log.clone();
    let result = run_secret_operation(
        SecretOperation::Unlock,
        SecretBytes::new(SECRET.to_vec()).unwrap(),
        &mut observer,
        |_secret| Ok::<_, VaultError>(()),
    );
    assert!(result.is_ok());
    assert_all_zero(&log, "unlock-secret", SECRET.len());
}

#[test]
fn error_cancel_and_replacement_paths_each_zeroize_before_reporting() {
    for (operation, label) in [
        (SecretOperation::Decrypt, "decrypt-plaintext"),
        (SecretOperation::NativePromptCancel, "native-passphrase"),
        (SecretOperation::AccountReplace, "replacement-secret"),
        (SecretOperation::Restore, "restore-secret"),
    ] {
        let log = SharedWipes::default();
        let mut observer = log.clone();
        let result = run_secret_operation(
            operation,
            SecretBytes::new(SECRET.to_vec()).unwrap(),
            &mut observer,
            |_secret| Err::<(), _>(VaultError::locked()),
        );
        assert!(result.is_err());
        assert_all_zero(&log, label, SECRET.len());
    }
}

#[test]
fn panic_unwind_zeroizes_secret_before_control_returns() {
    let log = SharedWipes::default();
    let mut observer = log.clone();
    let unwind = catch_unwind(AssertUnwindSafe(|| {
        let _ = run_secret_operation(
            SecretOperation::Decrypt,
            SecretBytes::new(SECRET.to_vec()).unwrap(),
            &mut observer,
            |_secret| -> Result<(), VaultError> { panic!("synthetic failure without secret") },
        );
    }));
    assert!(unwind.is_err());
    assert_all_zero(&log, "decrypt-plaintext", SECRET.len());
}

#[test]
fn observed_secret_drop_reports_post_wipe_state_not_predeclared_success() {
    let log = SharedWipes::default();
    {
        let _secret = SecretBytes::new_observed(
            SECRET.to_vec(),
            "drop-secret",
            Box::new(log.clone()),
        )
        .unwrap();
    }
    assert_all_zero(&log, "drop-secret", SECRET.len());
}

#[test]
fn replacing_secret_wipes_old_region_before_new_region_is_installed() {
    let log = SharedWipes::default();
    let mut observer = log.clone();
    let mut secret = SecretBytes::new(SECRET.to_vec()).unwrap();
    secret
        .replace(
            b"CANARY_WAL004_REPLACEMENT_BYTES".to_vec(),
            "replacement-old",
            &mut observer,
        )
        .unwrap();
    assert_all_zero(&log, "replacement-old", SECRET.len());
    secret.expose(|bytes| assert_eq!(bytes, b"CANARY_WAL004_REPLACEMENT_BYTES"));
}

#[test]
fn seal_success_wipes_passphrase_argon_hkdf_and_plaintext_regions() {
    let log = SharedWipes::default();
    let mut observer = log.clone();
    let metadata = VaultMetadata::new(
        [0x11; 16],
        Asset::Zec,
        Network::ZecTestnet,
        3,
    )
    .unwrap();
    let mut passphrase = SecretBytes::new(PASSPHRASE.to_vec()).unwrap();
    let mut plaintext = SecretBytes::new(SECRET.to_vec()).unwrap();
    let envelope = seal_vault(
        &metadata,
        &mut passphrase,
        &mut plaintext,
        &mut Entropy,
        &mut observer,
    )
    .unwrap();
    for (label, length) in [
        ("passphrase", PASSPHRASE.len()),
        ("argon2-output", 32),
        ("hkdf-output", 32),
        ("plaintext", SECRET.len()),
    ] {
        assert_all_zero(&log, label, length);
    }
    assert!(!envelope.as_bytes().windows(SECRET.len()).any(|part| part == SECRET));
    assert!(!envelope.as_bytes().windows(PASSPHRASE.len()).any(|part| part == PASSPHRASE));
}

#[test]
fn wipe_observer_exposes_only_label_length_and_boolean() {
    assert_eq!(WipeEvent::field_names(), ["label", "length", "all_zero"]);
    let event = WipeEvent { label: "synthetic", length: 31, all_zero: true };
    assert_eq!(format!("{event:?}"), "WipeEvent { label: \"synthetic\", length: 31, all_zero: true }");
    assert!(!format!("{event:?}").contains("CANARY"));
}
