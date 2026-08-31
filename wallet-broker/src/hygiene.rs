use core::fmt;

use crate::vault::{SecretBytes, VaultError, WipeObserver, valid_account_id};

const DIAGNOSTIC_OPERATIONS: [&str; 9] = [
    "vault.seal",
    "vault.open",
    "vault.store",
    "vault.export",
    "vault.restore",
    "session.lock",
    "native.unlock",
    "native.export",
    "native.restore",
];

const DIAGNOSTIC_CODES: [&str; 13] = [
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
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecretOperation {
    Unlock,
    Decrypt,
    NativePromptCancel,
    AccountReplace,
    Restore,
}

impl SecretOperation {
    fn label(self) -> &'static str {
        match self {
            Self::Unlock => "unlock-secret",
            Self::Decrypt => "decrypt-plaintext",
            Self::NativePromptCancel => "native-passphrase",
            Self::AccountReplace => "replacement-secret",
            Self::Restore => "restore-secret",
        }
    }
}

pub fn run_secret_operation<T>(
    operation: SecretOperation,
    secret: SecretBytes,
    observer: &mut dyn WipeObserver,
    action: impl FnOnce(&mut SecretBytes) -> Result<T, VaultError>,
) -> Result<T, VaultError> {
    struct Guard<'a> {
        secret: SecretBytes,
        label: &'static str,
        observer: &'a mut dyn WipeObserver,
    }

    impl Drop for Guard<'_> {
        fn drop(&mut self) {
            self.secret.wipe_with(self.label, self.observer);
        }
    }

    let mut guard = Guard {
        secret,
        label: operation.label(),
        observer,
    };
    action(&mut guard.secret)
}

#[derive(Clone, Eq, PartialEq)]
pub struct DiagnosticEvent {
    operation: String,
    account_id: String,
    code: String,
}

impl DiagnosticEvent {
    pub fn new(operation: &str, account_id: &str, code: &str) -> Result<Self, VaultError> {
        if !DIAGNOSTIC_OPERATIONS.contains(&operation)
            || !valid_account_id(account_id)
            || !DIAGNOSTIC_CODES.contains(&code)
        {
            return Err(VaultError::schema());
        }
        Ok(Self {
            operation: operation.to_owned(),
            account_id: account_id.to_owned(),
            code: code.to_owned(),
        })
    }

    pub fn snapshot(&self) -> DiagnosticSnapshot<'_> {
        DiagnosticSnapshot {
            operation: &self.operation,
            account_id: &self.account_id,
            code: &self.code,
        }
    }

    pub fn evidence_fields(&self) -> [&'static str; 3] {
        self.field_names()
    }

    pub fn field_names(&self) -> [&'static str; 3] {
        ["operation", "account_id", "code"]
    }
}

impl fmt::Debug for DiagnosticEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DiagnosticEvent")
            .field("operation", &self.operation)
            .field("account_id", &self.account_id)
            .field("code", &self.code)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticSnapshot<'a> {
    pub operation: &'a str,
    pub account_id: &'a str,
    pub code: &'a str,
}
