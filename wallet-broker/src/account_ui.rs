use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use eframe::egui;
use zeroize::{Zeroize, Zeroizing};

use crate::accounts::{AccountManager, AccountSummary, PreparedRestore};
use crate::native::ActionOrigin;
use crate::session::MonotonicClock;
use crate::vault::{EntropyPort, SecretBytes, WipeObserver};

const MAX_PASSPHRASE_BYTES: usize = 1_024;
const TITLE: &str = "BitBook accounts";
const UNAVAILABLE: &str = "UNAVAILABLE";

pub trait AccountUiPort {
    type Restore;

    fn list(&mut self) -> Result<Vec<AccountSummary>, &'static str>;
    fn create(&mut self, passphrase: SecretBytes) -> Result<AccountSummary, &'static str>;
    fn unlock(&mut self, id: &str, passphrase: SecretBytes) -> Result<(), &'static str>;
    fn lock(&mut self, id: &str) -> Result<(), &'static str>;
    fn lock_all(&mut self);
    fn export(&mut self, id: &str, path: &Path) -> Result<(), &'static str>;
    fn prepare_restore(
        &mut self,
        path: &Path,
        passphrase: SecretBytes,
    ) -> Result<Self::Restore, &'static str>;
    fn restore_summary(&self, prepared: &Self::Restore) -> AccountSummary;
    fn confirm_restore(&mut self, prepared: Self::Restore) -> Result<AccountSummary, &'static str>;
}

pub struct SharedAccountPort<C: MonotonicClock, E: EntropyPort, W: WipeObserver> {
    manager: Arc<Mutex<AccountManager<C, E, W>>>,
}

impl<C: MonotonicClock, E: EntropyPort, W: WipeObserver> SharedAccountPort<C, E, W> {
    pub fn new(manager: Arc<Mutex<AccountManager<C, E, W>>>) -> Self {
        Self { manager }
    }

    fn with_manager<R>(
        &self,
        action: impl FnOnce(&mut AccountManager<C, E, W>) -> Result<R, crate::accounts::AccountError>,
    ) -> Result<R, &'static str> {
        match self.manager.lock() {
            Ok(mut manager) => action(&mut manager).map_err(|error| error.code()),
            Err(poisoned) => {
                poisoned.into_inner().lock_all();
                Err(UNAVAILABLE)
            }
        }
    }
}

impl<C: MonotonicClock + Send, E: EntropyPort + Send, W: WipeObserver + Send> AccountUiPort
    for SharedAccountPort<C, E, W>
{
    type Restore = PreparedRestore;

    fn list(&mut self) -> Result<Vec<AccountSummary>, &'static str> {
        self.with_manager(|manager| {
            manager.tick()?;
            manager.list()
        })
    }

    fn create(&mut self, passphrase: SecretBytes) -> Result<AccountSummary, &'static str> {
        self.with_manager(|manager| {
            manager.create_software(ActionOrigin::NativeSurface, passphrase)
        })
    }

    fn unlock(&mut self, id: &str, passphrase: SecretBytes) -> Result<(), &'static str> {
        self.with_manager(|manager| manager.unlock(ActionOrigin::NativeSurface, id, passphrase))
    }

    fn lock(&mut self, id: &str) -> Result<(), &'static str> {
        self.with_manager(|manager| manager.lock(id))
    }

    fn lock_all(&mut self) {
        match self.manager.lock() {
            Ok(mut manager) => manager.lock_all(),
            Err(poisoned) => poisoned.into_inner().lock_all(),
        }
    }

    fn export(&mut self, id: &str, path: &Path) -> Result<(), &'static str> {
        self.with_manager(|manager| manager.export_encrypted(ActionOrigin::NativeSurface, id, path))
    }

    fn prepare_restore(
        &mut self,
        path: &Path,
        passphrase: SecretBytes,
    ) -> Result<Self::Restore, &'static str> {
        self.with_manager(|manager| {
            manager.prepare_restore(ActionOrigin::NativeSurface, path, passphrase)
        })
    }

    fn restore_summary(&self, prepared: &Self::Restore) -> AccountSummary {
        prepared.summary().clone()
    }

    fn confirm_restore(&mut self, prepared: Self::Restore) -> Result<AccountSummary, &'static str> {
        match self.manager.lock() {
            Ok(mut manager) => manager
                .confirm_restore(ActionOrigin::NativeSurface, prepared, true)
                .map_err(|error| error.code())?
                .ok_or(UNAVAILABLE),
            Err(poisoned) => {
                poisoned.into_inner().lock_all();
                Err(UNAVAILABLE)
            }
        }
    }
}

pub trait AccountDialogs {
    fn export_path(&mut self) -> Option<PathBuf>;
    fn restore_path(&mut self) -> Option<PathBuf>;
}

#[derive(Default)]
pub struct NativeAccountDialogs;

impl AccountDialogs for NativeAccountDialogs {
    fn export_path(&mut self) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("BitBook encrypted vault", &["vault"])
            .save_file()
    }

    fn restore_path(&mut self) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("BitBook encrypted vault", &["vault"])
            .pick_file()
    }
}

struct WindowControlInner {
    visible: AtomicBool,
    quit: AtomicBool,
    context: Mutex<Option<egui::Context>>,
}

#[derive(Clone)]
pub struct AccountWindowControl {
    inner: Arc<WindowControlInner>,
}

impl AccountWindowControl {
    pub fn request_open(&self) -> bool {
        if self.inner.quit.load(Ordering::Acquire) {
            return false;
        }
        let context = match self.inner.context.lock() {
            Ok(context) => context,
            Err(_) => return false,
        };
        let Some(context) = context.as_ref() else {
            return false;
        };
        if self.inner.quit.load(Ordering::Acquire) {
            return false;
        }
        self.inner.visible.store(true, Ordering::Release);
        context.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        context.send_viewport_cmd(egui::ViewportCommand::Focus);
        context.request_repaint();
        true
    }

    pub fn request_quit(&self) {
        self.inner.quit.store(true, Ordering::Release);
        if let Ok(context) = self.inner.context.lock()
            && let Some(context) = context.as_ref()
        {
            context.request_repaint();
        }
    }

    fn register_context(&self, context: &egui::Context) {
        if let Ok(mut registered) = self.inner.context.lock()
            && registered.is_none()
        {
            *registered = Some(context.clone());
        }
    }

    fn visible(&self) -> bool {
        self.inner.visible.load(Ordering::Acquire)
    }
}

struct MaskedInput {
    text: Zeroizing<String>,
    selected: bool,
}

impl MaskedInput {
    fn new() -> Self {
        Self {
            text: Zeroizing::new(String::with_capacity(MAX_PASSPHRASE_BYTES)),
            selected: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn matches(&self, other: &Self) -> bool {
        self.text.as_bytes() == other.text.as_bytes()
    }

    fn clear(&mut self) {
        self.text.zeroize();
        self.selected = false;
    }

    fn replace_or_append(&mut self, incoming: &mut String) {
        let retained = if self.selected { 0 } else { self.text.len() };
        let Some(new_length) = retained.checked_add(incoming.len()) else {
            incoming.zeroize();
            return;
        };
        if new_length > MAX_PASSPHRASE_BYTES {
            incoming.zeroize();
            return;
        }
        if self.selected {
            self.clear();
        }
        self.text.push_str(incoming);
        self.selected = false;
        incoming.zeroize();
    }

    fn backspace(&mut self) {
        if self.selected {
            self.clear();
        } else {
            self.text.pop();
        }
    }

    fn delete_selected(&mut self) {
        if self.selected {
            self.clear();
        }
    }

    fn delete_surrounding(&mut self, before_chars: usize) {
        if self.selected {
            self.clear();
            return;
        }
        for _ in 0..before_chars {
            self.text.pop();
        }
    }

    fn take_secret(&mut self) -> Option<SecretBytes> {
        if self.text.is_empty() || self.text.len() > MAX_PASSPHRASE_BYTES {
            self.clear();
            return None;
        }

        let mut secret = vec![0; self.text.len()].into_boxed_slice();
        secret.copy_from_slice(self.text.as_bytes());

        let unused_capacity = self.text.capacity() - self.text.len();
        self.text.extend(std::iter::repeat_n('\0', unused_capacity));
        self.text.zeroize();
        self.selected = false;
        SecretBytes::new(secret.into_vec()).ok()
    }

    fn ui(&mut self, ui: &mut egui::Ui, id_salt: &'static str) {
        let desired = egui::vec2(
            ui.available_width().max(ui.spacing().interact_size.x),
            ui.spacing().interact_size.y.max(28.0),
        );
        let (_, rect) = ui.allocate_space(desired);
        let id = ui.make_persistent_id(("masked-account-passphrase", id_salt));
        let response = ui.interact(rect, id, egui::Sense::click());
        if response.clicked() {
            self.selected = false;
            response.request_focus();
        }
        let focused = ui.memory(|memory| memory.has_focus(id));
        if focused {
            ui.ctx().input_mut(|input| {
                consume_secret_events(&mut input.events, Some(self));
                consume_secret_events(&mut input.raw.events, None);
            });
        }

        let visuals = *ui.style().interact(&response);
        let field_stroke = if focused {
            ui.visuals().selection.stroke
        } else {
            visuals.bg_stroke
        };
        ui.painter().rect(
            rect,
            visuals.corner_radius,
            visuals.bg_fill,
            field_stroke,
            egui::epaint::StrokeKind::Inside,
        );
        let bullets: String = std::iter::repeat_n('•', self.text.chars().count()).collect();
        let font = egui::TextStyle::Body.resolve(ui.style());
        let text_rect = ui.painter_at(rect).text(
            rect.left_center() + egui::vec2(6.0, 0.0),
            egui::Align2::LEFT_CENTER,
            bullets,
            font,
            visuals.text_color(),
        );

        if focused {
            let cursor_x = text_rect
                .right()
                .clamp(rect.left() + 5.0, rect.right() - 4.0);
            let cursor_rect = egui::Rect::from_min_max(
                egui::pos2(cursor_x, rect.top() + 4.0),
                egui::pos2(cursor_x + 1.0, rect.bottom() - 4.0),
            );
            ui.painter().line_segment(
                [cursor_rect.center_top(), cursor_rect.center_bottom()],
                visuals.fg_stroke,
            );
            let to_global = ui
                .ctx()
                .layer_transform_to_global(ui.layer_id())
                .unwrap_or_default();
            ui.output_mut(|output| {
                output.ime = Some(egui::output::IMEOutput {
                    purpose: egui::IMEPurpose::Password,
                    rect: to_global * rect,
                    cursor_rect: to_global * cursor_rect,
                    should_interrupt_composition: false,
                });
            });
        }
    }
}

fn consume_secret_events(events: &mut Vec<egui::Event>, mut input: Option<&mut MaskedInput>) {
    let mut retained = Vec::with_capacity(events.len());
    for event in std::mem::take(events) {
        match event {
            egui::Event::Text(mut text) | egui::Event::Paste(mut text) => {
                if let Some(input) = input.as_deref_mut() {
                    input.replace_or_append(&mut text);
                }
                text.zeroize();
            }
            egui::Event::Ime(egui::ImeEvent::Commit(mut text)) => {
                if let Some(input) = input.as_deref_mut() {
                    input.replace_or_append(&mut text);
                }
                text.zeroize();
            }
            egui::Event::Ime(egui::ImeEvent::Preedit { mut text, .. }) => {
                text.zeroize();
            }
            egui::Event::Ime(egui::ImeEvent::DeleteSurrounding { before_chars, .. }) => {
                if let Some(input) = input.as_deref_mut() {
                    input.delete_surrounding(before_chars);
                }
            }
            egui::Event::Copy | egui::Event::Cut => {}
            egui::Event::Key {
                key: egui::Key::Backspace,
                pressed: true,
                ..
            } => {
                if let Some(input) = input.as_deref_mut() {
                    input.backspace();
                }
            }
            egui::Event::Key {
                key: egui::Key::Delete,
                pressed: true,
                ..
            } => {
                if let Some(input) = input.as_deref_mut() {
                    input.delete_selected();
                }
            }
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if (modifiers.command || modifiers.ctrl) && key == egui::Key::A => {
                if let Some(input) = input.as_deref_mut() {
                    input.selected = true;
                }
            }
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if (modifiers.command || modifiers.ctrl)
                && matches!(key, egui::Key::C | egui::Key::X | egui::Key::Z) => {}
            event => retained.push(event),
        }
    }
    *events = retained;
}

fn scrub_secret_events(context: &egui::Context) {
    context.input_mut(|input| {
        consume_secret_events(&mut input.events, None);
        consume_secret_events(&mut input.raw.events, None);
    });
}

enum Scene<R> {
    List,
    Create {
        passphrase: MaskedInput,
        confirmation: MaskedInput,
    },
    Unlock {
        account_id: String,
        passphrase: MaskedInput,
    },
    RestorePassword {
        path: PathBuf,
        passphrase: MaskedInput,
    },
    RestoreConfirm {
        prepared: R,
        summary: AccountSummary,
    },
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SafeMessage {
    BackupExported,
    WalletLocked,
    WalletUnavailable,
}

impl SafeMessage {
    fn text(self) -> &'static str {
        match self {
            Self::BackupExported => "Backup exported",
            Self::WalletLocked => "Wallet locked",
            Self::WalletUnavailable => "Wallet unavailable",
        }
    }
}

pub struct AccountWindow<P: AccountUiPort, D: AccountDialogs> {
    port: P,
    dialogs: D,
    control: AccountWindowControl,
    accounts: Vec<AccountSummary>,
    selected: Option<String>,
    scene: Scene<P::Restore>,
    message: Option<SafeMessage>,
    list_failed: bool,
    quit_sent: bool,
}

impl<P: AccountUiPort, D: AccountDialogs> AccountWindow<P, D> {
    pub fn new(port: P, dialogs: D, initially_visible: bool) -> Self {
        Self {
            port,
            dialogs,
            control: AccountWindowControl {
                inner: Arc::new(WindowControlInner {
                    visible: AtomicBool::new(initially_visible),
                    quit: AtomicBool::new(false),
                    context: Mutex::new(None),
                }),
            },
            accounts: Vec::new(),
            selected: None,
            scene: Scene::List,
            message: None,
            list_failed: false,
            quit_sent: false,
        }
    }

    pub fn control(&self) -> AccountWindowControl {
        self.control.clone()
    }

    fn service(&mut self, context: &egui::Context) {
        self.control.register_context(context);
        if self.control.inner.quit.load(Ordering::Acquire) {
            scrub_secret_events(context);
            if !self.quit_sent {
                self.clear_private_state();
                self.port.lock_all();
                context.send_viewport_cmd(egui::ViewportCommand::Close);
                self.quit_sent = true;
            }
            return;
        }

        let close_requested = context.input(|input| {
            input.viewport().close_requested() || input.key_pressed(egui::Key::Escape)
        });
        if close_requested && self.control.inner.visible.swap(false, Ordering::AcqRel) {
            self.clear_private_state();
            self.port.lock_all();
            context.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            context.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        if !self.control.visible() {
            scrub_secret_events(context);
            context.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        self.refresh_accounts();
        context.request_repaint_after(Duration::from_secs(1));
    }

    fn refresh_accounts(&mut self) {
        match self.port.list() {
            Ok(mut accounts) => {
                accounts.sort_by(|left, right| left.account_id.cmp(&right.account_id));
                self.accounts = accounts;
                if self.selected.as_ref().is_some_and(|selected| {
                    !self
                        .accounts
                        .iter()
                        .any(|account| &account.account_id == selected)
                }) {
                    self.selected = None;
                }
                if self.list_failed {
                    self.message = None;
                    self.list_failed = false;
                }
            }
            Err(_) => {
                self.accounts.clear();
                self.selected = None;
                self.message = Some(SafeMessage::WalletUnavailable);
                self.list_failed = true;
            }
        }
    }

    fn port_error(&mut self, code: &'static str) {
        self.accounts.clear();
        self.selected = None;
        self.message = Some(if code == "LOCKED" {
            SafeMessage::WalletLocked
        } else {
            SafeMessage::WalletUnavailable
        });
        self.list_failed = false;
    }

    fn clear_private_state(&mut self) {
        self.scene = Scene::List;
        self.selected = None;
        self.message = None;
    }

    fn show_list(&mut self, ui: &mut egui::Ui) {
        ui.heading(TITLE);
        ui.label("Zcash testnet accounts");
        ui.label("Payments are not enabled");
        if let Some(message) = self.message {
            ui.label(message.text());
        }

        let body_height = (ui.available_height() - 170.0).max(40.0);
        egui::ScrollArea::vertical()
            .id_salt("account-list")
            .max_height(body_height)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for account in &self.accounts {
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                self.selected.as_deref() == Some(account.account_id.as_str()),
                                &account.account_id,
                            )
                            .clicked()
                        {
                            self.selected = Some(account.account_id.clone());
                        }
                        ui.label(if account.locked { "Locked" } else { "Unlocked" });
                    });
                }
            });

        let selected = self.selected.as_ref().and_then(|selected| {
            self.accounts
                .iter()
                .find(|account| &account.account_id == selected)
                .map(|account| (account.account_id.clone(), account.locked))
        });
        if ui.button("Create account").clicked() {
            self.message = None;
            self.scene = Scene::Create {
                passphrase: MaskedInput::new(),
                confirmation: MaskedInput::new(),
            };
            return;
        }
        if ui.button("Restore backup").clicked() {
            self.begin_restore_dialog();
            return;
        }
        if ui
            .add_enabled(
                selected.as_ref().is_some_and(|(_, locked)| *locked),
                egui::Button::new("Unlock"),
            )
            .clicked()
            && let Some((account_id, _)) = selected.as_ref()
        {
            self.message = None;
            self.scene = Scene::Unlock {
                account_id: account_id.clone(),
                passphrase: MaskedInput::new(),
            };
            return;
        }
        if ui
            .add_enabled(
                selected.as_ref().is_some_and(|(_, locked)| !*locked),
                egui::Button::new("Lock"),
            )
            .clicked()
            && let Some((account_id, _)) = selected.as_ref()
        {
            let account_id = account_id.clone();
            match self.port.lock(&account_id) {
                Ok(()) => self.refresh_accounts(),
                Err(error) => self.port_error(error),
            }
            return;
        }
        if ui
            .add_enabled(selected.is_some(), egui::Button::new("Export backup"))
            .clicked()
            && let Some((account_id, _)) = selected
        {
            self.begin_export_dialog(&account_id);
        }
    }

    fn begin_export_dialog(&mut self, account_id: &str) {
        let chosen = catch_unwind(AssertUnwindSafe(|| self.dialogs.export_path()));
        match chosen {
            Ok(Some(path)) => match self.port.export(account_id, &path) {
                Ok(()) => self.message = Some(SafeMessage::BackupExported),
                Err(error) => self.port_error(error),
            },
            Ok(None) => {}
            Err(_) => {
                self.clear_private_state();
                self.port.lock_all();
                self.port_error(UNAVAILABLE);
            }
        }
    }

    fn begin_restore_dialog(&mut self) {
        let chosen = catch_unwind(AssertUnwindSafe(|| self.dialogs.restore_path()));
        match chosen {
            Ok(Some(path)) => {
                self.message = None;
                self.scene = Scene::RestorePassword {
                    path,
                    passphrase: MaskedInput::new(),
                };
            }
            Ok(None) => {}
            Err(_) => {
                self.clear_private_state();
                self.port.lock_all();
                self.port_error(UNAVAILABLE);
            }
        }
    }

    fn show_create(&mut self, ui: &mut egui::Ui) {
        ui.heading("Create account");
        if let Some(message) = self.message {
            ui.label(message.text());
        }
        if let Scene::Create {
            passphrase,
            confirmation,
        } = &mut self.scene
        {
            ui.label("Passphrase");
            passphrase.ui(ui, "create-passphrase");
            ui.label("Confirm passphrase");
            confirmation.ui(ui, "create-confirmation");
        }
        let create = ui.button("Create").clicked();
        let cancel = ui.button("Cancel").clicked();
        if cancel {
            self.clear_private_state();
        } else if create {
            self.submit_create();
        }
    }

    fn submit_create(&mut self) {
        let valid = matches!(
            &self.scene,
            Scene::Create {
                passphrase,
                confirmation,
            } if !passphrase.is_empty() && !confirmation.is_empty() && passphrase.matches(confirmation)
        );
        if !valid {
            if let Scene::Create {
                passphrase,
                confirmation,
            } = &mut self.scene
            {
                passphrase.clear();
                confirmation.clear();
            }
            self.message = Some(SafeMessage::WalletLocked);
            return;
        }
        let secret = if let Scene::Create {
            passphrase,
            confirmation,
        } = &mut self.scene
        {
            confirmation.clear();
            passphrase.take_secret()
        } else {
            None
        };
        let Some(secret) = secret else {
            self.message = Some(SafeMessage::WalletLocked);
            return;
        };
        match self.port.create(secret) {
            Ok(_) => {
                self.scene = Scene::List;
                self.message = None;
                self.refresh_accounts();
            }
            Err(error) => self.port_error(error),
        }
    }

    fn show_unlock(&mut self, ui: &mut egui::Ui) {
        ui.heading("Unlock account");
        if let Some(message) = self.message {
            ui.label(message.text());
        }
        if let Scene::Unlock { passphrase, .. } = &mut self.scene {
            ui.label("Passphrase");
            passphrase.ui(ui, "unlock-passphrase");
        }
        let unlock = ui.button("Unlock account").clicked();
        let cancel = ui.button("Cancel").clicked();
        if cancel {
            self.clear_private_state();
        } else if unlock {
            self.submit_unlock();
        }
    }

    fn submit_unlock(&mut self) {
        let old = std::mem::replace(&mut self.scene, Scene::List);
        let Scene::Unlock {
            account_id,
            mut passphrase,
        } = old
        else {
            return;
        };
        let Some(secret) = passphrase.take_secret() else {
            self.scene = Scene::Unlock {
                account_id,
                passphrase,
            };
            self.message = Some(SafeMessage::WalletLocked);
            return;
        };
        match self.port.unlock(&account_id, secret) {
            Ok(()) => {
                self.message = None;
                self.refresh_accounts();
            }
            Err(error) => {
                self.scene = Scene::Unlock {
                    account_id,
                    passphrase,
                };
                self.port_error(error);
            }
        }
    }

    fn show_restore_password(&mut self, ui: &mut egui::Ui) {
        ui.heading("Restore backup");
        if let Some(message) = self.message {
            ui.label(message.text());
        }
        if let Scene::RestorePassword { passphrase, .. } = &mut self.scene {
            ui.label("Passphrase");
            passphrase.ui(ui, "restore-passphrase");
        }
        let inspect = ui.button("Inspect backup").clicked();
        let cancel = ui.button("Cancel").clicked();
        if cancel {
            self.clear_private_state();
        } else if inspect {
            self.submit_restore_password();
        }
    }

    fn submit_restore_password(&mut self) {
        let old = std::mem::replace(&mut self.scene, Scene::List);
        let Scene::RestorePassword {
            path,
            mut passphrase,
        } = old
        else {
            return;
        };
        let Some(secret) = passphrase.take_secret() else {
            self.scene = Scene::RestorePassword { path, passphrase };
            self.message = Some(SafeMessage::WalletLocked);
            return;
        };
        match self.port.prepare_restore(&path, secret) {
            Ok(prepared) => {
                let summary = self.port.restore_summary(&prepared);
                self.message = None;
                self.scene = Scene::RestoreConfirm { prepared, summary };
            }
            Err(error) => {
                self.scene = Scene::RestorePassword { path, passphrase };
                self.port_error(error);
            }
        }
    }

    fn show_restore_confirm(&mut self, ui: &mut egui::Ui) {
        ui.heading("Restore backup");
        if let Some(message) = self.message {
            ui.label(message.text());
        }
        if let Scene::RestoreConfirm { summary, .. } = &self.scene {
            ui.label(&summary.account_id);
            ui.label(summary.network);
        }
        let confirm = ui.button("Confirm restore").clicked();
        let cancel = ui.button("Cancel").clicked();
        if cancel {
            self.clear_private_state();
        } else if confirm {
            self.submit_restore_confirmation();
        }
    }

    fn submit_restore_confirmation(&mut self) {
        let old = std::mem::replace(&mut self.scene, Scene::List);
        let Scene::RestoreConfirm { prepared, .. } = old else {
            return;
        };
        match self.port.confirm_restore(prepared) {
            Ok(_) => {
                self.message = None;
                self.refresh_accounts();
            }
            Err(error) => self.port_error(error),
        }
    }
}

impl<P: AccountUiPort, D: AccountDialogs> eframe::App for AccountWindow<P, D> {
    fn logic(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.service(context);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.service(ui.ctx());
        if self.quit_sent || !self.control.visible() {
            return;
        }
        egui::CentralPanel::default().show(ui, |ui| {
            if matches!(&self.scene, Scene::List) {
                self.show_list(ui);
            } else if matches!(&self.scene, Scene::Create { .. }) {
                self.show_create(ui);
            } else if matches!(&self.scene, Scene::Unlock { .. }) {
                self.show_unlock(ui);
            } else if matches!(&self.scene, Scene::RestorePassword { .. }) {
                self.show_restore_password(ui);
            } else if matches!(&self.scene, Scene::RestoreConfirm { .. }) {
                self.show_restore_confirm(ui);
            }
        });
        scrub_secret_events(ui.ctx());
    }
}

impl<P: AccountUiPort, D: AccountDialogs> Drop for AccountWindow<P, D> {
    fn drop(&mut self) {
        self.control.inner.quit.store(true, Ordering::Release);
        self.clear_private_state();
        self.port.lock_all();
    }
}
