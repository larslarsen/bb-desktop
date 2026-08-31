use core::fmt;

use crate::vault::{MAX_PASSPHRASE_BYTES, SecretBytes, WipeObserver, valid_account_id};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionOrigin {
    NativeSurface,
    Electron,
    BrokerProtocol,
    Http,
}

#[derive(Debug)]
pub enum NativeAction {
    Unlock { account_id: String },
    Export { account_id: String },
    Restore,
    UnlockCancelled { pending_passphrase: SecretBytes },
    WindowClosed { pending_passphrase: SecretBytes },
}

impl NativeAction {
    pub fn from_method(_method: &str) -> Result<Self, NativeError> {
        Err(NativeError::schema())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordPrompt {
    pub masked: bool,
    pub copy_enabled: bool,
    pub paste_to_other_surface: bool,
    pub accessibility_value_exposed: bool,
    pub maximum_utf8_bytes: usize,
}

impl Default for PasswordPrompt {
    fn default() -> Self {
        Self {
            masked: true,
            copy_enabled: false,
            paste_to_other_surface: false,
            accessibility_value_exposed: false,
            maximum_utf8_bytes: 1_024,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestoreMetadata {
    pub account_id: String,
    pub asset: String,
    pub network: String,
    pub epoch: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceResult {
    Success,
    Cancelled,
    Error(String),
}

#[derive(Clone, Eq, PartialEq)]
pub struct NativeError {
    code: &'static str,
    message: &'static str,
}

impl NativeError {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }

    pub fn locked() -> Self {
        Self::new("LOCKED", "Wallet locked")
    }

    fn schema() -> Self {
        Self::new("SCHEMA", "Wallet request is invalid")
    }

    fn unauth() -> Self {
        Self::new("UNAUTH", "Wallet action requires the native surface")
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn public_message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Debug for NativeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeError")
            .field("code", &self.code)
            .finish()
    }
}

impl fmt::Display for NativeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for NativeError {}

pub trait NativeSurfacePort {
    fn prompt_password(
        &mut self,
        prompt: PasswordPrompt,
    ) -> Result<Option<SecretBytes>, NativeError>;
    fn confirm_restore(&mut self, metadata: RestoreMetadata) -> Result<bool, NativeError>;
    fn show_result(&mut self, result: SurfaceResult);
}

pub trait FileDialogPort {
    fn choose_new_backup(&mut self) -> Result<Option<String>, NativeError>;
    fn choose_existing_backup(&mut self) -> Result<Option<String>, NativeError>;
}

pub trait CustodyPort {
    fn unlock(&mut self, account_id: &str, passphrase: &mut SecretBytes)
    -> Result<(), NativeError>;
    fn export_encrypted(&mut self, account_id: &str, path: &str) -> Result<(), NativeError>;
    fn inspect_restore(
        &mut self,
        path: &str,
        passphrase: &mut SecretBytes,
    ) -> Result<RestoreMetadata, NativeError>;
    fn commit_restore(&mut self, path: &str, expected: &RestoreMetadata)
    -> Result<(), NativeError>;
}

pub struct NativeController<C: CustodyPort, W: WipeObserver> {
    custody: C,
    wipe_observer: W,
}

impl<C: CustodyPort, W: WipeObserver> NativeController<C, W> {
    pub fn new(custody: C, wipe_observer: W) -> Self {
        Self {
            custody,
            wipe_observer,
        }
    }

    pub fn custody(&self) -> &C {
        &self.custody
    }

    pub fn wipe_observer(&self) -> &W {
        &self.wipe_observer
    }

    pub fn execute(
        &mut self,
        origin: ActionOrigin,
        action: NativeAction,
        surface: &mut dyn NativeSurfacePort,
        dialog: &mut dyn FileDialogPort,
    ) -> Result<(), NativeError> {
        if origin != ActionOrigin::NativeSurface {
            return Err(NativeError::unauth());
        }
        match action {
            NativeAction::Unlock { account_id } => self.unlock(&account_id, surface),
            NativeAction::Export { account_id } => self.export(&account_id, surface, dialog),
            NativeAction::Restore => self.restore(surface, dialog),
            NativeAction::UnlockCancelled {
                mut pending_passphrase,
            }
            | NativeAction::WindowClosed {
                mut pending_passphrase,
            } => {
                pending_passphrase.wipe_with("native-passphrase", &mut self.wipe_observer);
                surface.show_result(SurfaceResult::Cancelled);
                Ok(())
            }
        }
    }

    fn unlock(
        &mut self,
        account_id: &str,
        surface: &mut dyn NativeSurfacePort,
    ) -> Result<(), NativeError> {
        if !valid_account_id(account_id) {
            return Err(NativeError::schema());
        }
        let Some(mut passphrase) = surface.prompt_password(PasswordPrompt::default())? else {
            surface.show_result(SurfaceResult::Cancelled);
            return Ok(());
        };
        if !valid_native_passphrase(&passphrase) {
            passphrase.wipe_with("native-passphrase", &mut self.wipe_observer);
            surface.show_result(SurfaceResult::Error("Wallet locked".to_owned()));
            return Err(NativeError::locked());
        }
        let result = self.custody.unlock(account_id, &mut passphrase);
        passphrase.wipe_with("native-passphrase", &mut self.wipe_observer);
        match result {
            Ok(()) => {
                surface.show_result(SurfaceResult::Success);
                Ok(())
            }
            Err(error) => {
                surface.show_result(SurfaceResult::Error(error.public_message().to_owned()));
                Err(error)
            }
        }
    }

    fn export(
        &mut self,
        account_id: &str,
        surface: &mut dyn NativeSurfacePort,
        dialog: &mut dyn FileDialogPort,
    ) -> Result<(), NativeError> {
        if !valid_account_id(account_id) {
            return Err(NativeError::schema());
        }
        let Some(path) = dialog.choose_new_backup()? else {
            surface.show_result(SurfaceResult::Cancelled);
            return Ok(());
        };
        let result = self.custody.export_encrypted(account_id, &path);
        self.finish(result, surface)
    }

    fn restore(
        &mut self,
        surface: &mut dyn NativeSurfacePort,
        dialog: &mut dyn FileDialogPort,
    ) -> Result<(), NativeError> {
        let Some(path) = dialog.choose_existing_backup()? else {
            surface.show_result(SurfaceResult::Cancelled);
            return Ok(());
        };
        let Some(mut passphrase) = surface.prompt_password(PasswordPrompt::default())? else {
            surface.show_result(SurfaceResult::Cancelled);
            return Ok(());
        };
        if !valid_native_passphrase(&passphrase) {
            passphrase.wipe_with("native-passphrase", &mut self.wipe_observer);
            surface.show_result(SurfaceResult::Error("Wallet locked".to_owned()));
            return Err(NativeError::locked());
        }
        let inspected = self.custody.inspect_restore(&path, &mut passphrase);
        passphrase.wipe_with("native-passphrase", &mut self.wipe_observer);
        let metadata = match inspected {
            Ok(metadata) => metadata,
            Err(error) => {
                surface.show_result(SurfaceResult::Error(error.public_message().to_owned()));
                return Err(error);
            }
        };
        if !surface.confirm_restore(metadata.clone())? {
            surface.show_result(SurfaceResult::Cancelled);
            return Ok(());
        }
        let result = self.custody.commit_restore(&path, &metadata);
        self.finish(result, surface)
    }

    fn finish(
        &self,
        result: Result<(), NativeError>,
        surface: &mut dyn NativeSurfacePort,
    ) -> Result<(), NativeError> {
        match result {
            Ok(()) => {
                surface.show_result(SurfaceResult::Success);
                Ok(())
            }
            Err(error) => {
                surface.show_result(SurfaceResult::Error(error.public_message().to_owned()));
                Err(error)
            }
        }
    }
}

fn valid_native_passphrase(passphrase: &SecretBytes) -> bool {
    !passphrase.is_empty()
        && passphrase.len() <= MAX_PASSPHRASE_BYTES
        && passphrase.expose(|bytes| core::str::from_utf8(bytes).is_ok())
}
