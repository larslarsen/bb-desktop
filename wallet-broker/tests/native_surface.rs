use bitbook_wallet_broker::native::{
    ActionOrigin, CustodyPort, FileDialogPort, NativeAction, NativeController,
    NativeError, NativeSurfacePort, PasswordPrompt, RestoreMetadata, SurfaceResult,
};
use bitbook_wallet_broker::vault::{SecretBytes, WipeEvent, WipeObserver};

const ACCOUNT: &str = "00112233445566778899aabbccddeeff";
const BACKUP_PATH: &str = "target/wal004-scratch/selected-backup.vault";

#[derive(Default)]
struct Surface {
    prompt: Option<PasswordPrompt>,
    passphrase: Option<SecretBytes>,
    restore: Option<RestoreMetadata>,
    confirmed: bool,
    results: Vec<SurfaceResult>,
}

impl NativeSurfacePort for Surface {
    fn prompt_password(&mut self, prompt: PasswordPrompt) -> Result<Option<SecretBytes>, NativeError> {
        self.prompt = Some(prompt);
        Ok(self.passphrase.take())
    }

    fn confirm_restore(&mut self, metadata: RestoreMetadata) -> Result<bool, NativeError> {
        self.restore = Some(metadata);
        Ok(self.confirmed)
    }

    fn show_result(&mut self, result: SurfaceResult) {
        self.results.push(result);
    }
}

#[derive(Default)]
struct Dialog {
    save_calls: usize,
    open_calls: usize,
    selected: Option<String>,
}

impl FileDialogPort for Dialog {
    fn choose_new_backup(&mut self) -> Result<Option<String>, NativeError> {
        self.save_calls += 1;
        Ok(self.selected.clone())
    }

    fn choose_existing_backup(&mut self) -> Result<Option<String>, NativeError> {
        self.open_calls += 1;
        Ok(self.selected.clone())
    }
}

#[derive(Default)]
struct Custody {
    calls: Vec<String>,
    passphrase_lengths: Vec<usize>,
    restored: bool,
    fail_locked: bool,
}

impl CustodyPort for Custody {
    fn unlock(&mut self, account_id: &str, passphrase: &mut SecretBytes) -> Result<(), NativeError> {
        self.calls.push(format!("unlock:{account_id}"));
        self.passphrase_lengths.push(passphrase.len());
        if self.fail_locked { Err(NativeError::locked()) } else { Ok(()) }
    }

    fn export_encrypted(&mut self, account_id: &str, path: &str) -> Result<(), NativeError> {
        self.calls.push(format!("export:{account_id}:{path}"));
        Ok(())
    }

    fn inspect_restore(
        &mut self,
        path: &str,
        passphrase: &mut SecretBytes,
    ) -> Result<RestoreMetadata, NativeError> {
        self.calls.push(format!("inspect:{path}"));
        self.passphrase_lengths.push(passphrase.len());
        Ok(RestoreMetadata {
            account_id: ACCOUNT.to_owned(),
            asset: "ZEC".to_owned(),
            network: "zec-testnet".to_owned(),
            epoch: 9,
        })
    }

    fn commit_restore(&mut self, path: &str, expected: &RestoreMetadata) -> Result<(), NativeError> {
        self.calls.push(format!("commit:{path}:{}", expected.epoch));
        self.restored = true;
        Ok(())
    }
}

#[derive(Default)]
struct Wipes(Vec<WipeEvent>);

impl WipeObserver for Wipes {
    fn observe(&mut self, event: WipeEvent) {
        self.0.push(event);
    }
}

fn controller() -> NativeController<Custody, Wipes> {
    NativeController::new(Custody::default(), Wipes::default())
}

#[test]
fn unlock_is_accepted_only_from_native_surface_origin() {
    for origin in [ActionOrigin::Electron, ActionOrigin::BrokerProtocol, ActionOrigin::Http] {
        let mut controller = controller();
        let mut surface = Surface {
            passphrase: Some(SecretBytes::new(b"synthetic-passphrase".to_vec()).unwrap()),
            ..Surface::default()
        };
        let mut dialog = Dialog::default();
        assert_eq!(
            controller
                .execute(origin, NativeAction::Unlock { account_id: ACCOUNT.to_owned() }, &mut surface, &mut dialog)
                .unwrap_err()
                .code(),
            "UNAUTH"
        );
        assert!(controller.custody().calls.is_empty());
    }
}

#[test]
fn export_and_restore_are_also_rejected_from_every_nonnative_origin() {
    for origin in [ActionOrigin::Electron, ActionOrigin::BrokerProtocol, ActionOrigin::Http] {
        for action in [
            NativeAction::Export { account_id: ACCOUNT.to_owned() },
            NativeAction::Restore,
        ] {
            let mut controller = controller();
            let mut surface = Surface {
                passphrase: Some(SecretBytes::new(b"synthetic-restore-passphrase".to_vec()).unwrap()),
                confirmed: true,
                ..Surface::default()
            };
            let mut dialog = Dialog { selected: Some(BACKUP_PATH.to_owned()), ..Dialog::default() };
            assert_eq!(
                controller.execute(origin, action, &mut surface, &mut dialog).unwrap_err().code(),
                "UNAUTH"
            );
            assert!(controller.custody().calls.is_empty());
            assert_eq!(dialog.save_calls + dialog.open_calls, 0);
        }
    }
}

#[test]
fn password_prompt_is_masked_noncopyable_and_bounded() {
    let mut controller = controller();
    let mut surface = Surface {
        passphrase: Some(SecretBytes::new(b"synthetic-passphrase".to_vec()).unwrap()),
        ..Surface::default()
    };
    let mut dialog = Dialog::default();
    controller
        .execute(
            ActionOrigin::NativeSurface,
            NativeAction::Unlock { account_id: ACCOUNT.to_owned() },
            &mut surface,
            &mut dialog,
        )
        .unwrap();
    let prompt = surface.prompt.unwrap();
    assert!(prompt.masked);
    assert!(!prompt.copy_enabled);
    assert!(!prompt.paste_to_other_surface);
    assert!(!prompt.accessibility_value_exposed);
    assert_eq!(prompt.maximum_utf8_bytes, 1_024);
    assert_eq!(controller.custody().passphrase_lengths, vec![20]);
}

#[test]
fn cancel_or_window_close_wipes_passphrase_and_performs_no_partial_action() {
    for closed in [false, true] {
        let mut controller = controller();
        let passphrase = SecretBytes::new(b"CANARY_WAL004_CANCELLED_PASSWORD".to_vec()).unwrap();
        let action = if closed {
            NativeAction::WindowClosed { pending_passphrase: passphrase }
        } else {
            NativeAction::UnlockCancelled { pending_passphrase: passphrase }
        };
        let mut surface = Surface::default();
        let mut dialog = Dialog::default();
        controller.execute(ActionOrigin::NativeSurface, action, &mut surface, &mut dialog).unwrap();
        assert!(controller.custody().calls.is_empty());
        assert!(controller.wipe_observer().0.iter().any(|event| {
            event.label == "native-passphrase" && event.length > 0 && event.all_zero
        }));
    }
}

#[test]
fn export_exchanges_only_a_path_with_dialog_and_ciphertext_stays_in_core() {
    let mut controller = controller();
    let mut surface = Surface::default();
    let mut dialog = Dialog { selected: Some(BACKUP_PATH.to_owned()), ..Dialog::default() };
    controller
        .execute(
            ActionOrigin::NativeSurface,
            NativeAction::Export { account_id: ACCOUNT.to_owned() },
            &mut surface,
            &mut dialog,
        )
        .unwrap();
    assert_eq!(dialog.save_calls, 1);
    assert_eq!(dialog.open_calls, 0);
    assert_eq!(controller.custody().calls, vec![format!("export:{ACCOUNT}:{BACKUP_PATH}")]);
    assert!(surface.restore.is_none());
    assert_eq!(surface.results, vec![SurfaceResult::Success]);
}

#[test]
fn restore_authenticates_before_metadata_and_requires_explicit_confirmation() {
    let mut controller = controller();
    let mut surface = Surface {
        passphrase: Some(SecretBytes::new(b"synthetic-restore-passphrase".to_vec()).unwrap()),
        confirmed: true,
        ..Surface::default()
    };
    let mut dialog = Dialog { selected: Some(BACKUP_PATH.to_owned()), ..Dialog::default() };
    controller
        .execute(ActionOrigin::NativeSurface, NativeAction::Restore, &mut surface, &mut dialog)
        .unwrap();
    assert_eq!(controller.custody().calls, vec![
        format!("inspect:{BACKUP_PATH}"),
        format!("commit:{BACKUP_PATH}:9"),
    ]);
    assert_eq!(controller.custody().passphrase_lengths, vec![28]);
    assert!(surface.prompt.is_some());
    assert!(controller.custody().restored);
    assert_eq!(surface.restore.unwrap().epoch, 9);
}

#[test]
fn restore_cancel_never_commits_or_changes_active_state() {
    let mut controller = controller();
    let mut surface = Surface {
        passphrase: Some(SecretBytes::new(b"synthetic-restore-passphrase".to_vec()).unwrap()),
        confirmed: false,
        ..Surface::default()
    };
    let mut dialog = Dialog { selected: Some(BACKUP_PATH.to_owned()), ..Dialog::default() };
    controller
        .execute(ActionOrigin::NativeSurface, NativeAction::Restore, &mut surface, &mut dialog)
        .unwrap();
    assert_eq!(controller.custody().calls, vec![format!("inspect:{BACKUP_PATH}")]);
    assert!(!controller.custody().restored);
    assert_eq!(surface.results, vec![SurfaceResult::Cancelled]);
}

#[test]
fn native_error_text_is_closed_and_secret_free() {
    let mut controller = NativeController::new(
        Custody { fail_locked: true, ..Custody::default() },
        Wipes::default(),
    );
    let mut surface = Surface {
        passphrase: Some(SecretBytes::new(b"CANARY_WAL004_NATIVE_PASSWORD".to_vec()).unwrap()),
        ..Surface::default()
    };
    let mut dialog = Dialog::default();
    assert!(controller
        .execute(
            ActionOrigin::NativeSurface,
            NativeAction::Unlock { account_id: ACCOUNT.to_owned() },
            &mut surface,
            &mut dialog,
        )
        .is_err());
    assert_eq!(surface.results, vec![SurfaceResult::Error("Wallet locked".to_owned())]);
    assert!(!format!("{:?}", surface.results).contains("CANARY"));
}

#[test]
fn generic_unlock_backup_and_future_payment_confirmation_methods_are_absent() {
    for method in [
        "wallet.invoke",
        "account.unlock",
        "account.exportBackup",
        "account.restoreBackup",
        "intent.confirm",
        "signer.sign",
        "tx.broadcast",
    ] {
        assert_eq!(NativeAction::from_method(method).unwrap_err().code(), "SCHEMA");
    }
}
