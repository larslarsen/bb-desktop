use core::fmt;
use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

use crate::vault::{MAX_PASSPHRASE_BYTES, SecretBytes, WipeObserver, valid_account_id};

fn consumed_confirmation_nonces() -> &'static Mutex<BTreeSet<Vec<u8>>> {
    static CONSUMED: OnceLock<Mutex<BTreeSet<Vec<u8>>>> = OnceLock::new();
    CONSUMED.get_or_init(|| Mutex::new(BTreeSet::new()))
}

/// The exact, already-rendered Zcash review accepted by the native confirmation surface.
/// This is crate-private so no protocol or Electron caller can construct a confirmation.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ZecNativeReview {
    pub(crate) handle: String,
    pub(crate) session_id: String,
    pub(crate) account_id: String,
    pub(crate) network: String,
    pub(crate) request_id: String,
    pub(crate) intent_hash: String,
    pub(crate) receiver: String,
    pub(crate) amount_zat: String,
    pub(crate) fee_zat: String,
    pub(crate) fee_bound_zat: String,
    pub(crate) memo_sha256: String,
    pub(crate) expires_at: String,
    pub(crate) transaction_version: u32,
    pub(crate) consensus_branch: u32,
    pub(crate) spend_pool: String,
    pub(crate) output_pool: String,
    pub(crate) review_hash: String,
}

/// A one-shot process-local authority. It deliberately has no public constructor, Clone,
/// Debug, Display, or serialization implementation.
pub(crate) struct ZecConfirmationCapability {
    handle: String,
    session_id: String,
    account_id: String,
    network: String,
    request_id: String,
    intent_hash: String,
    review_hash: String,
    nonce: SecretBytes,
    synthetic_test_surface: bool,
}

impl ZecConfirmationCapability {
    fn mint(review: ZecNativeReview, synthetic_test_surface: bool) -> Result<Self, NativeError> {
        let mut nonce = vec![0; 32];
        getrandom::fill(&mut nonce).map_err(|_| NativeError::locked())?;
        Ok(Self {
            handle: review.handle,
            session_id: review.session_id,
            account_id: review.account_id,
            network: review.network,
            request_id: review.request_id,
            intent_hash: review.intent_hash,
            review_hash: review.review_hash,
            nonce: SecretBytes::new(nonce).map_err(|_| NativeError::locked())?,
            synthetic_test_surface,
        })
    }

    pub(crate) fn consume(self) -> Result<Self, NativeError> {
        let fingerprint = self.nonce.expose(|bytes| bytes.to_vec());
        let mut consumed = consumed_confirmation_nonces()
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if !consumed.insert(fingerprint) {
            return Err(NativeError::unauth());
        }
        Ok(self)
    }

    pub(crate) fn matches_review(&self, review: &ZecNativeReview) -> bool {
        self.handle == review.handle
            && self.session_id == review.session_id
            && self.account_id == review.account_id
            && self.network == review.network
            && self.request_id == review.request_id
            && self.intent_hash == review.intent_hash
            && self.review_hash == review.review_hash
            && self.nonce.len() == 32
    }

    pub(crate) fn handle(&self) -> &str {
        &self.handle
    }

    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    pub(crate) fn account_id(&self) -> &str {
        &self.account_id
    }

    pub(crate) fn network(&self) -> &str {
        &self.network
    }

    pub(crate) fn request_id(&self) -> &str {
        &self.request_id
    }

    pub(crate) fn intent_hash(&self) -> &str {
        &self.intent_hash
    }

    pub(crate) fn review_hash(&self) -> &str {
        &self.review_hash
    }

    pub(crate) fn nonce_len(&self) -> usize {
        self.nonce.len()
    }

    pub(crate) fn came_from_synthetic_test_surface(&self) -> bool {
        self.synthetic_test_surface
    }
}

pub(crate) trait ZecNativeReviewSurfacePort {
    fn confirm_zec_review(&mut self, review: &ZecNativeReview) -> Result<bool, NativeError>;
}

/// The sole production minting adapter. The capability is produced only after a native surface
/// reports that it displayed and accepted this exact frozen review.
pub(crate) fn confirm_zec_review_native(
    surface: &mut dyn ZecNativeReviewSurfacePort,
    review: ZecNativeReview,
) -> Result<ZecConfirmationCapability, NativeError> {
    if !surface.confirm_zec_review(&review)? {
        return Err(NativeError::unauth());
    }
    ZecConfirmationCapability::mint(review, false)
}

/// Unmistakable test-only native-surface adapter; it does not accept an ActionOrigin or method.
#[doc(hidden)]
pub(crate) fn confirm_zec_review_synthetic_test_surface(
    review: ZecNativeReview,
) -> Result<ZecConfirmationCapability, NativeError> {
    ZecConfirmationCapability::mint(review, true)
}

pub(crate) fn confirm_zec_review_from_origin(
    origin: ActionOrigin,
    surface: &mut dyn ZecNativeReviewSurfacePort,
    review: ZecNativeReview,
) -> Result<ZecConfirmationCapability, NativeError> {
    if origin != ActionOrigin::NativeSurface {
        return Err(NativeError::unauth());
    }
    confirm_zec_review_native(surface, review)
}

pub(crate) fn confirm_zec_review_from_method(_method: &str) -> Result<(), NativeError> {
    Err(NativeError::unauth())
}

pub(crate) fn replay_consumed_confirmation(marker: &[u8]) -> Result<(), NativeError> {
    let consumed = consumed_confirmation_nonces()
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    if consumed.iter().any(|value| value == marker) || !marker.is_empty() {
        return Err(NativeError::unauth());
    }
    Err(NativeError::unauth())
}

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

    pub(crate) fn schema() -> Self {
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

pub trait XmrInstallationSelectionPort {
    fn choose_wallet_rpc(&mut self) -> Result<Option<String>, NativeError>;
    fn validate_selected_path(&mut self, path: &str) -> Result<(), NativeError>;
    fn verify_selected_executable(&mut self, path: &str) -> Result<(), NativeError>;
    fn persist_selection(&mut self, path: &str) -> Result<(), NativeError>;
    fn process_side_effect_for_test(&mut self);
}

pub struct XmrSelectionController;

impl XmrSelectionController {
    pub fn select(
        origin: ActionOrigin,
        port: &mut dyn XmrInstallationSelectionPort,
    ) -> Result<(), NativeError> {
        if origin != ActionOrigin::NativeSurface {
            return Err(NativeError::unauth());
        }
        let Some(path) = port.choose_wallet_rpc()? else {
            return Ok(());
        };
        port.validate_selected_path(&path)?;
        port.verify_selected_executable(&path)?;
        port.persist_selection(&path)
    }
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
