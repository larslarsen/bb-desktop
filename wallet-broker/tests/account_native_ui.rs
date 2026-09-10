#![cfg(feature = "native-ui")]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use bitbook_wallet_broker::account_ui::{
    AccountDialogs, AccountUiPort, AccountWindow, SharedAccountPort,
};
use bitbook_wallet_broker::accounts::{AccountSummary, LocalAccountManager};
use bitbook_wallet_broker::vault::{
    SecretBytes, VaultError, VaultWorkObserver, WipeEvent, WipeObserver, open_vault_bytes,
    parse_vault,
};
use bitbook_wallet_broker::zec::test_support::decode_unified_address;
use bitbook_wallet_broker::zec::{AccountId, FreshReceiverV1, Network as ZecNetwork};
use eframe::egui;
use rusqlite::Connection;
use zcash_keys::keys::{UnifiedAddressRequest, UnifiedSpendingKey};
use zcash_protocol::consensus::Network::TestNetwork;

const FRAME_DT: f64 = 1.0 / 60.0;
const NATIVE_SIZE: [f32; 2] = [520.0, 720.0];
const SMALL_SIZE: [f32; 2] = [360.0, 480.0];
const TITLE: &str = "BitBook accounts";
const NOTICE_TESTNET: &str = "Zcash testnet accounts";
const NOTICE_PAYMENTS: &str = "Payments are not enabled";
const CREATE_ACCOUNT: &str = "Create account";
const RESTORE_BACKUP: &str = "Restore backup";
const UNLOCK: &str = "Unlock";
const LOCK: &str = "Lock";
const EXPORT_BACKUP: &str = "Export backup";
const PASSPHRASE: &str = "Passphrase";
const CONFIRM_PASSPHRASE: &str = "Confirm passphrase";
const CREATE: &str = "Create";
const CANCEL: &str = "Cancel";
const UNLOCK_ACCOUNT: &str = "Unlock account";
const INSPECT_BACKUP: &str = "Inspect backup";
const CONFIRM_RESTORE: &str = "Confirm restore";
const EXPORT_SUCCESS: &str = "Backup exported";
const UNAVAILABLE_MESSAGE: &str = "Wallet unavailable";
const LOCKED_MESSAGE: &str = "Wallet locked";
const STATUS_LOCKED: &str = "Locked";
const STATUS_UNLOCKED: &str = "Unlocked";
const BULLET: char = '•';
const UNICODE_PASS: &str = "páss-テスト-🔐";
const WRONG_UNLOCK: &str = "wrong-unlock-secret";
const RIGHT_UNLOCK: &str = "right-unlock-secret";
const RESTORE_PASS: &str = "restore-inspect-secret";
const SECRET_CANARY: &str = "CANARY_WAL013_UI_SECRET_VALUE";
const RAW_ERROR_CANARY: &str = "CANARY:/tmp/raw-account.vault";
const EXPORT_PATH_CANARY: &str = "CANARY_WAL013_EXPORT_PATH.vault";
const RESTORE_PATH_CANARY: &str = "CANARY_WAL013_RESTORE_PATH.vault";
const LOCKED_ID: &str = "00112233445566778899aabbccddeeff";
const UNLOCKED_ID: &str = "ffeeddccbbaa99887766554433221100";
const CREATED_ID: &str = "cafecafecafecafecafecafecafecafe";
const RESTORE_ID: &str = "abcdefabcdefabcdefabcdefabcdefab";
const RECEIVE: &str = "Receive";
const RECEIVE_TITLE: &str = "Receive Zcash";
const RECEIVE_NETWORK: &str = "Zcash testnet";
const BALANCE_UNAVAILABLE: &str = "Balance unavailable — not synced";
const COPY_ADDRESS: &str = "Copy address";
const BACK: &str = "Back";
const UI_SEED_A: [u8; 32] = [0x71; 32];
const UI_SEED_B: [u8; 32] = [0x92; 32];

struct PersistentUi {
    ctx: egui::Context,
    time: f64,
    screen: egui::Rect,
    last_repaint_ms: Arc<AtomicU64>,
}

struct FrameObservation {
    screen: egui::Rect,
    painted: Vec<(String, egui::Rect, egui::Rect)>,
    rects: Vec<(egui::Rect, egui::Rect)>,
    close: usize,
    cancel_close: usize,
    visible: Vec<bool>,
    focus: usize,
    title: Option<String>,
    copy_texts: Vec<String>,
    widget_text: Vec<String>,
    accesskit: String,
    events_description: String,
    leftover_events: Vec<String>,
    leftover_raw_events: Vec<String>,
}

struct LogicObservation {
    focus: usize,
    repaint_ms: u64,
}

struct FakeRestore {
    summary: AccountSummary,
}

struct PortState {
    accounts: Vec<AccountSummary>,
    list_error: Option<&'static str>,
    create_error: Option<&'static str>,
    unlock_failures: usize,
    next_id: String,
    restore_account: AccountSummary,
    list_calls: usize,
    create_calls: Vec<Vec<u8>>,
    unlock_calls: Vec<(String, Vec<u8>)>,
    lock_calls: Vec<String>,
    lock_all_calls: usize,
    export_calls: Vec<(String, PathBuf)>,
    prepare_calls: Vec<(PathBuf, Vec<u8>)>,
    confirm_calls: usize,
    receive_calls: Vec<String>,
    receive_results: VecDeque<Result<FreshReceiverV1, &'static str>>,
}

struct FakePort {
    inner: Rc<RefCell<PortState>>,
}

struct DialogState {
    export_queue: Vec<Option<PathBuf>>,
    restore_queue: Vec<Option<PathBuf>>,
    export_calls: usize,
    restore_calls: usize,
    panic_export: bool,
}

struct FakeDialogs {
    inner: Rc<RefCell<DialogState>>,
}

struct Ignore;

struct Work;

struct Scratch {
    root: PathBuf,
    zec_accounts: Vec<String>,
}

impl PersistentUi {
    fn new(size: [f32; 2]) -> Self {
        let ctx = egui::Context::default();
        let last_repaint_ms = Arc::new(AtomicU64::new(u64::MAX));
        let captured = Arc::clone(&last_repaint_ms);
        ctx.set_request_repaint_callback(move |info| {
            let ms = info.delay.as_millis().min(u128::from(u64::MAX)) as u64;
            captured.store(ms, Ordering::Relaxed);
        });
        Self {
            ctx,
            time: 0.0,
            screen: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(size[0], size[1])),
            last_repaint_ms,
        }
    }

    fn take_input(&mut self, events: Vec<egui::Event>, close_requested: bool) -> egui::RawInput {
        let mut input = egui::RawInput {
            screen_rect: Some(self.screen),
            time: Some(self.time),
            events,
            ..egui::RawInput::default()
        };
        if let Some(info) = input.viewports.get_mut(&egui::ViewportId::ROOT) {
            info.inner_rect = Some(self.screen);
            if close_requested {
                info.events.push(egui::ViewportEvent::Close);
            }
        }
        input
    }

    fn run<P: AccountUiPort, D: AccountDialogs>(
        &mut self,
        app: &mut AccountWindow<P, D>,
        events: Vec<egui::Event>,
        close_requested: bool,
    ) -> FrameObservation {
        let input = self.take_input(events, close_requested);
        let mut frame = eframe::Frame::_new_kittest();
        let output = self.ctx.run_ui(input, |ui| {
            eframe::App::logic(app, ui.ctx(), &mut frame);
            eframe::App::ui(app, ui, &mut frame);
        });
        let leftover = leftover_event_text(&self.ctx);
        let observation = FrameObservation::from_output(self.screen, &output, leftover);
        output.drop_without_applying_deltas();
        self.time += FRAME_DT;
        observation
    }

    fn run_logic<P: AccountUiPort, D: AccountDialogs>(
        &mut self,
        app: &mut AccountWindow<P, D>,
        close_requested: bool,
    ) -> LogicObservation {
        self.last_repaint_ms.store(u64::MAX, Ordering::Relaxed);
        let input = self.take_input(Vec::new(), close_requested);
        let mut frame = eframe::Frame::_new_kittest();
        let output = self.ctx.run_logic(&input, |ctx| {
            eframe::App::logic(app, ctx, &mut frame);
        });
        let commands = output
            .viewport_commands
            .get(&egui::ViewportId::ROOT)
            .cloned()
            .unwrap_or_default();
        LogicObservation {
            focus: count_command(&commands, |command| {
                matches!(command, egui::ViewportCommand::Focus)
            }),
            repaint_ms: self.last_repaint_ms.load(Ordering::Relaxed),
        }
    }
}

impl FrameObservation {
    fn from_output(
        screen: egui::Rect,
        output: &egui::FullOutput,
        leftover: (Vec<String>, Vec<String>),
    ) -> Self {
        let root = output.viewport_output.get(&egui::ViewportId::ROOT);
        let commands = root
            .map(|viewport| viewport.commands.as_slice())
            .unwrap_or(&[]);
        let title = root.and_then(|viewport| viewport.builder.title.clone());
        let (leftover_events, leftover_raw_events) = leftover;
        Self {
            screen,
            painted: collect_painted_text(&output.shapes),
            rects: collect_rects(&output.shapes),
            close: count_command(commands, |command| {
                matches!(command, egui::ViewportCommand::Close)
            }),
            cancel_close: count_command(commands, |command| {
                matches!(command, egui::ViewportCommand::CancelClose)
            }),
            visible: commands
                .iter()
                .filter_map(|command| match command {
                    egui::ViewportCommand::Visible(visible) => Some(*visible),
                    _ => None,
                })
                .collect(),
            focus: count_command(commands, |command| {
                matches!(command, egui::ViewportCommand::Focus)
            }),
            title,
            copy_texts: output
                .platform_output
                .commands
                .iter()
                .filter_map(|command| match command {
                    egui::OutputCommand::CopyText(text) => Some(text.clone()),
                    _ => None,
                })
                .collect(),
            widget_text: output
                .platform_output
                .events
                .iter()
                .map(|event| format!("{event:?}"))
                .collect(),
            accesskit: format!("{:?}", output.platform_output.accesskit_update),
            events_description: output.platform_output.events_description(),
            leftover_events,
            leftover_raw_events,
        }
    }

    fn paints(&self, label: &str) -> bool {
        self.painted.iter().any(|(text, _, _)| text == label)
    }

    fn visible_label(&self, label: &str) -> egui::Rect {
        let mut hits = Vec::new();
        for (text, visual, clip) in &self.painted {
            if text == label {
                hits.push((*visual, *clip));
            }
        }
        let (visual, clip) = *hits.last().expect("exact label must be painted");
        assert!(visual.is_finite());
        assert!(visual.is_positive());
        assert!(
            clip.contains_rect(visual),
            "label must be fully within clip"
        );
        assert!(
            self.screen.contains_rect(visual),
            "label must be fully within screen"
        );
        visual
    }

    fn field_near(&self, label: &str) -> egui::Rect {
        let label_rect = self.visible_label(label);
        let mut best: Option<(f32, f32, egui::Rect, egui::Rect)> = None;
        for (rect, clip) in &self.rects {
            if !rect.is_finite() || !rect.is_positive() {
                continue;
            }
            if !(12.0..=56.0).contains(&rect.height()) || rect.width() < 48.0 {
                continue;
            }
            if label_rect.contains_rect(*rect) {
                continue;
            }
            let vertical_gap = rect.min.y - label_rect.max.y;
            let below = (-2.0..=48.0).contains(&vertical_gap)
                && rect.max.x > label_rect.min.x
                && rect.min.x < label_rect.max.x + 480.0;
            if below {
                let horizontal_gap = if rect.max.x < label_rect.min.x {
                    label_rect.min.x - rect.max.x
                } else if rect.min.x > label_rect.max.x {
                    rect.min.x - label_rect.max.x
                } else {
                    0.0
                };
                if best
                    .map(|(best_vertical_gap, best_horizontal_gap, _, _)| {
                        vertical_gap < best_vertical_gap
                            || (vertical_gap == best_vertical_gap
                                && horizontal_gap < best_horizontal_gap)
                    })
                    .unwrap_or(true)
                {
                    best = Some((vertical_gap, horizontal_gap, *rect, *clip));
                }
            }
        }
        let (_, _, field, clip) =
            best.expect("masked field must have painted geometry near its label");
        assert!(clip.contains_rect(field), "field must be fully within clip");
        assert!(
            self.screen.contains_rect(field),
            "field must be fully within screen"
        );
        field
    }

    fn bullet_count(&self) -> usize {
        self.painted
            .iter()
            .filter(|(text, _, _)| !text.is_empty() && text.chars().all(|ch| ch == BULLET))
            .map(|(text, _, _)| text.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn assert_usable(&self, label: &str) {
        let _ = self.visible_label(label);
    }

    fn assert_no_substr(&self, needle: &str) {
        for (text, _, _) in &self.painted {
            assert!(
                !text.contains(needle),
                "painted text leaked {needle}: {text}"
            );
        }
        for text in &self.copy_texts {
            assert!(!text.contains(needle), "clipboard leaked {needle}: {text}");
        }
        for text in &self.widget_text {
            assert!(
                !text.contains(needle),
                "widget info leaked {needle}: {text}"
            );
        }
        assert!(
            !self.accesskit.contains(needle),
            "accessibility tree leaked {needle}"
        );
        assert!(
            !self.events_description.contains(needle),
            "platform events leaked {needle}"
        );
        if let Some(title) = &self.title {
            assert!(!title.contains(needle), "title leaked {needle}: {title}");
        }
        for text in self.leftover_events.iter().chain(&self.leftover_raw_events) {
            assert!(
                !text.contains(needle),
                "input event copy leaked {needle}: {text}"
            );
        }
    }
}

impl WipeObserver for Ignore {
    fn observe(&mut self, _event: WipeEvent) {}
}

impl VaultWorkObserver for Work {
    fn before_allocation(&mut self, _bytes: usize) -> Result<(), VaultError> {
        Ok(())
    }

    fn before_kdf(&mut self) {}
}

impl Default for PortState {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
            list_error: None,
            create_error: None,
            unlock_failures: 0,
            next_id: CREATED_ID.to_owned(),
            restore_account: summary(RESTORE_ID, true),
            list_calls: 0,
            create_calls: Vec::new(),
            unlock_calls: Vec::new(),
            lock_calls: Vec::new(),
            lock_all_calls: 0,
            export_calls: Vec::new(),
            prepare_calls: Vec::new(),
            confirm_calls: 0,
            receive_calls: Vec::new(),
            receive_results: VecDeque::new(),
        }
    }
}

impl FakePort {
    fn new(state: PortState) -> (Self, Rc<RefCell<PortState>>) {
        let inner = Rc::new(RefCell::new(state));
        (
            Self {
                inner: Rc::clone(&inner),
            },
            inner,
        )
    }
}

impl AccountUiPort for FakePort {
    type Restore = FakeRestore;

    fn list(&mut self) -> Result<Vec<AccountSummary>, &'static str> {
        let mut state = self.inner.borrow_mut();
        state.list_calls += 1;
        if let Some(error) = state.list_error {
            return Err(error);
        }
        Ok(state.accounts.clone())
    }

    fn create(&mut self, passphrase: SecretBytes) -> Result<AccountSummary, &'static str> {
        let secret = passphrase.expose(|bytes| bytes.to_vec());
        let mut state = self.inner.borrow_mut();
        state.create_calls.push(secret);
        if let Some(error) = state.create_error {
            return Err(error);
        }
        let created = summary(&state.next_id, true);
        state.accounts.push(created.clone());
        Ok(created)
    }

    fn unlock(&mut self, id: &str, passphrase: SecretBytes) -> Result<(), &'static str> {
        let secret = passphrase.expose(|bytes| bytes.to_vec());
        let mut state = self.inner.borrow_mut();
        state.unlock_calls.push((id.to_owned(), secret));
        if state.unlock_failures > 0 {
            state.unlock_failures -= 1;
            return Err("LOCKED");
        }
        let account = state
            .accounts
            .iter_mut()
            .find(|account| account.account_id == id)
            .ok_or("SCHEMA")?;
        account.locked = false;
        Ok(())
    }

    fn lock(&mut self, id: &str) -> Result<(), &'static str> {
        let mut state = self.inner.borrow_mut();
        state.lock_calls.push(id.to_owned());
        let account = state
            .accounts
            .iter_mut()
            .find(|account| account.account_id == id)
            .ok_or("SCHEMA")?;
        account.locked = true;
        Ok(())
    }

    fn lock_all(&mut self) {
        let mut state = self.inner.borrow_mut();
        state.lock_all_calls += 1;
        for account in &mut state.accounts {
            account.locked = true;
        }
    }

    fn receive(&mut self, id: &str) -> Result<FreshReceiverV1, &'static str> {
        let mut state = self.inner.borrow_mut();
        state.receive_calls.push(id.to_owned());
        state
            .receive_results
            .pop_front()
            .unwrap_or(Err("UNAVAILABLE"))
    }

    fn export(&mut self, id: &str, path: &Path) -> Result<(), &'static str> {
        self.inner
            .borrow_mut()
            .export_calls
            .push((id.to_owned(), path.to_path_buf()));
        Ok(())
    }

    fn prepare_restore(
        &mut self,
        path: &Path,
        passphrase: SecretBytes,
    ) -> Result<Self::Restore, &'static str> {
        let secret = passphrase.expose(|bytes| bytes.to_vec());
        let mut state = self.inner.borrow_mut();
        state.prepare_calls.push((path.to_path_buf(), secret));
        Ok(FakeRestore {
            summary: state.restore_account.clone(),
        })
    }

    fn restore_summary(&self, prepared: &Self::Restore) -> AccountSummary {
        prepared.summary.clone()
    }

    fn confirm_restore(&mut self, prepared: Self::Restore) -> Result<AccountSummary, &'static str> {
        let mut state = self.inner.borrow_mut();
        state.confirm_calls += 1;
        state.accounts.push(prepared.summary.clone());
        Ok(prepared.summary)
    }
}

impl Default for DialogState {
    fn default() -> Self {
        Self {
            export_queue: Vec::new(),
            restore_queue: Vec::new(),
            export_calls: 0,
            restore_calls: 0,
            panic_export: false,
        }
    }
}

impl FakeDialogs {
    fn new(state: DialogState) -> (Self, Rc<RefCell<DialogState>>) {
        let inner = Rc::new(RefCell::new(state));
        (
            Self {
                inner: Rc::clone(&inner),
            },
            inner,
        )
    }
}

impl AccountDialogs for FakeDialogs {
    fn export_path(&mut self) -> Option<PathBuf> {
        let mut state = self.inner.borrow_mut();
        if state.panic_export {
            panic!("CANARY_DIALOG_PANIC");
        }
        state.export_calls += 1;
        if state.export_queue.is_empty() {
            None
        } else {
            state.export_queue.remove(0)
        }
    }

    fn restore_path(&mut self) -> Option<PathBuf> {
        let mut state = self.inner.borrow_mut();
        state.restore_calls += 1;
        if state.restore_queue.is_empty() {
            None
        } else {
            state.restore_queue.remove(0)
        }
    }
}

impl Scratch {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let token = NEXT.fetch_add(1, Ordering::Relaxed);
        let name = format!("wal013-native-ui-{label}-{}-{token}", std::process::id());
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(name);
        match fs::symlink_metadata(&root) {
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Ok(_) => panic!("refusing to reuse stale WAL-013 UI root {}", root.display()),
            Err(error) => panic!("cannot inspect WAL-013 UI root: {error}"),
        }
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        Self {
            root,
            zec_accounts: Vec::new(),
        }
    }

    fn accounts(&self) -> PathBuf {
        self.root.join("accounts")
    }

    fn track_zec_account(&mut self, account_id: &str) {
        assert!(AccountId::parse(account_id).is_ok());
        if !self.zec_accounts.iter().any(|known| known == account_id) {
            self.zec_accounts.push(account_id.to_owned());
        }
    }

    fn cleanup(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        let zec_network = self.root.join("zec-testnet");
        for account_id in &self.zec_accounts {
            push_cleanup_error(&mut errors, unlink_zec_account(&zec_network, account_id));
        }
        push_cleanup_error(&mut errors, unlink_any(&zec_network));
        let accounts = self.accounts();
        push_cleanup_error(&mut errors, unlink_dir_contents(&accounts));
        push_cleanup_error(&mut errors, unlink_any(&accounts));
        push_cleanup_error(&mut errors, unlink_dir_contents(&self.root));
        push_cleanup_error(&mut errors, unlink_any(&self.root));
        push_cleanup_error(&mut errors, assert_absent(&accounts));
        push_cleanup_error(&mut errors, assert_absent(&self.root));
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}

fn unlink_zec_account(network: &Path, account_id: &str) -> Result<(), String> {
    let network_metadata = match fs::symlink_metadata(network) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err("cannot inspect owned Zcash network directory".to_owned()),
        Ok(metadata) => metadata,
    };
    if !network_metadata.file_type().is_dir() || network_metadata.file_type().is_symlink() {
        return Ok(());
    }
    let directory = network.join(account_id);
    let metadata = match fs::symlink_metadata(&directory) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(format!("cannot inspect {}: {error}", directory.display()));
        }
        Ok(metadata) => metadata,
    };
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return unlink_any(&directory);
    }
    for name in [
        "wallet.sqlite3",
        "wallet.sqlite3-journal",
        "wallet.sqlite3-wal",
        "wallet.sqlite3-shm",
        "compact.sqlite3",
        "compact.sqlite3-journal",
        "compact.sqlite3-wal",
        "compact.sqlite3-shm",
    ] {
        unlink_any(&directory.join(name))?;
    }
    unlink_any(&directory)
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            if std::thread::panicking() {
                eprintln!("WAL-013 native UI scratch cleanup failed during unwind: {error}");
            } else {
                panic!("WAL-013 native UI scratch cleanup failed: {error}");
            }
        }
    }
}

fn summary(account_id: &str, locked: bool) -> AccountSummary {
    AccountSummary {
        account_id: account_id.to_owned(),
        asset: "ZEC",
        network: "zec-testnet",
        kind: "software",
        locked,
    }
}

fn oracle_receiver(seed: &[u8], index: u64) -> String {
    let viewing = UnifiedSpendingKey::from_seed(&TestNetwork, seed, Default::default())
        .expect("reviewed seed must derive")
        .to_unified_full_viewing_key();
    let (address, actual_index) = viewing
        .find_address(index.into(), UnifiedAddressRequest::ORCHARD)
        .expect("reviewed Orchard-only request must derive");
    assert_eq!(u64::try_from(actual_index).unwrap(), index);
    address.encode(&TestNetwork)
}

fn fake_receiver(account_id: &str, seed: &[u8; 32], index: u64, sequence: u64) -> FreshReceiverV1 {
    FreshReceiverV1 {
        account_id: AccountId::parse(account_id).unwrap(),
        network: ZecNetwork::Testnet,
        receiver: oracle_receiver(seed, index),
        diversifier_index: index.to_string(),
        issued_at_sequence: sequence.to_string(),
    }
}

fn assert_testnet_orchard(receiver: &str) {
    let decoded = decode_unified_address(receiver).unwrap();
    assert_eq!(decoded.network, ZecNetwork::Testnet);
    assert_eq!(decoded.receivers.len(), 1);
    assert!(decoded.receivers[0].is_orchard_protocol());
    assert!(!decoded.receivers[0].is_p2pkh());
    assert!(!decoded.receivers[0].is_p2sh());
    assert!(!decoded.receivers[0].is_sapling());
    assert!(!decoded.receivers[0].is_tex());
    assert!(!decoded.receivers[0].is_unknown());
}

fn push_cleanup_error(errors: &mut Vec<String>, result: Result<(), String>) {
    if let Err(error) = result {
        errors.push(error);
    }
}

fn assert_absent(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot inspect {}: {error}", path.display())),
        Ok(_) => Err(format!("owned temp leaked: {}", path.display())),
    }
}

fn unlink_any(path: &Path) -> Result<(), String> {
    let meta = match fs::symlink_metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("cannot inspect {}: {error}", path.display())),
        Ok(meta) => meta,
    };
    let result = if meta.file_type().is_dir() && !meta.file_type().is_symlink() {
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    };
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot remove {}: {error}", path.display())),
    }
}

fn unlink_dir_contents(dir: &Path) -> Result<(), String> {
    let meta = match fs::symlink_metadata(dir) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("cannot inspect {}: {error}", dir.display())),
        Ok(meta) => meta,
    };
    if !meta.file_type().is_dir() || meta.file_type().is_symlink() {
        return Ok(());
    }
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => return Err(format!("cannot read {}: {error}", dir.display())),
    };
    let mut errors = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => push_cleanup_error(&mut errors, unlink_any(&entry.path())),
            Err(error) => errors.push(format!("cannot read entry in {}: {error}", dir.display())),
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

fn mode(path: &Path) -> u32 {
    fs::symlink_metadata(path).unwrap().permissions().mode() & 0o777
}

fn walk_regular_files(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let meta = fs::symlink_metadata(&dir)
            .unwrap_or_else(|error| panic!("cannot inspect {}: {error}", dir.display()));
        if !meta.file_type().is_dir() || meta.file_type().is_symlink() {
            continue;
        }
        let entries = fs::read_dir(&dir)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", dir.display()));
        for entry in entries {
            let entry = entry
                .unwrap_or_else(|error| panic!("cannot read entry in {}: {error}", dir.display()));
            let path = entry.path();
            let meta = fs::symlink_metadata(&path)
                .unwrap_or_else(|error| panic!("cannot inspect {}: {error}", path.display()));
            let file_type = meta.file_type();
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                dirs.push(path);
            } else if file_type.is_file() {
                let bytes = fs::read(&path)
                    .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
                files.push((path, bytes));
            }
        }
    }
    files
}

fn collect_painted_text(
    shapes: &[egui::epaint::ClippedShape],
) -> Vec<(String, egui::Rect, egui::Rect)> {
    let mut painted = Vec::new();
    for clipped in shapes {
        collect_text_shapes(&clipped.shape, clipped.clip_rect, &mut painted);
    }
    painted
}

fn collect_text_shapes(
    shape: &egui::Shape,
    clip_rect: egui::Rect,
    out: &mut Vec<(String, egui::Rect, egui::Rect)>,
) {
    match shape {
        egui::Shape::Text(text) => {
            out.push((
                text.galley.text().to_owned(),
                text.visual_bounding_rect(),
                clip_rect,
            ));
        }
        egui::Shape::Vec(nested) => {
            for child in nested {
                collect_text_shapes(child, clip_rect, out);
            }
        }
        _ => {}
    }
}

fn collect_rects(shapes: &[egui::epaint::ClippedShape]) -> Vec<(egui::Rect, egui::Rect)> {
    let mut rects = Vec::new();
    for clipped in shapes {
        collect_rect_shapes(&clipped.shape, clipped.clip_rect, &mut rects);
    }
    rects
}

fn collect_rect_shapes(
    shape: &egui::Shape,
    clip_rect: egui::Rect,
    out: &mut Vec<(egui::Rect, egui::Rect)>,
) {
    match shape {
        egui::Shape::Rect(rect) => out.push((rect.rect, clip_rect)),
        egui::Shape::Vec(nested) => {
            for child in nested {
                collect_rect_shapes(child, clip_rect, out);
            }
        }
        _ => {}
    }
}

fn count_command(
    commands: &[egui::ViewportCommand],
    predicate: impl Fn(&egui::ViewportCommand) -> bool,
) -> usize {
    commands.iter().filter(|command| predicate(command)).count()
}

fn leftover_event_text(ctx: &egui::Context) -> (Vec<String>, Vec<String>) {
    ctx.input(|input| (event_texts(&input.events), event_texts(&input.raw.events)))
}

fn event_texts(events: &[egui::Event]) -> Vec<String> {
    let mut texts = Vec::new();
    for event in events {
        match event {
            egui::Event::Text(text) | egui::Event::Paste(text) => texts.push(text.clone()),
            egui::Event::Ime(egui::ImeEvent::Commit(text)) => texts.push(text.clone()),
            egui::Event::Ime(egui::ImeEvent::Preedit { text, .. }) => texts.push(text.clone()),
            _ => {}
        }
    }
    texts
}

fn primary_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn escape() -> egui::Event {
    key(egui::Key::Escape, egui::Modifiers::NONE)
}

fn click<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    pos: egui::Pos2,
) -> FrameObservation {
    click_release_and_settle(ui, app, pos).1
}

fn click_release_and_settle<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    pos: egui::Pos2,
) -> (FrameObservation, FrameObservation) {
    let _ = ui.run(app, vec![egui::Event::PointerMoved(pos)], false);
    let _ = ui.run(app, vec![primary_button(pos, true)], false);
    let release = ui.run(app, vec![primary_button(pos, false)], false);
    let settled = ui.run(app, Vec::new(), false);
    (release, settled)
}

fn click_label<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    label: &str,
) -> FrameObservation {
    let first = ui.run(app, Vec::new(), false);
    click(ui, app, first.visible_label(label).center())
}

fn focus_field<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    label: &str,
) -> FrameObservation {
    let first = ui.run(app, Vec::new(), false);
    click(ui, app, first.field_near(label).center())
}

fn type_text<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    text: &str,
) -> FrameObservation {
    ui.run(app, vec![egui::Event::Text(text.to_owned())], false)
}

fn type_chars<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    text: &str,
) -> FrameObservation {
    let mut last = ui.run(app, Vec::new(), false);
    for ch in text.chars() {
        last = ui.run(app, vec![egui::Event::Text(ch.to_string())], false);
    }
    last
}

fn fill_matching<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
    secret: &str,
) {
    let _ = focus_field(ui, app, PASSPHRASE);
    let _ = type_text(ui, app, secret);
    let _ = focus_field(ui, app, CONFIRM_PASSPHRASE);
    let _ = type_text(ui, app, secret);
}

fn open_window(
    accounts: Vec<AccountSummary>,
) -> (
    AccountWindow<FakePort, FakeDialogs>,
    Rc<RefCell<PortState>>,
    Rc<RefCell<DialogState>>,
) {
    let mut state = PortState::default();
    state.accounts = accounts;
    let (port, records) = FakePort::new(state);
    let (dialogs, dialog_records) = FakeDialogs::new(DialogState::default());
    (
        AccountWindow::new(port, dialogs, true),
        records,
        dialog_records,
    )
}

fn first_list<P: AccountUiPort, D: AccountDialogs>(
    ui: &mut PersistentUi,
    app: &mut AccountWindow<P, D>,
) -> FrameObservation {
    let frame = ui.run(app, Vec::new(), false);
    frame.assert_usable(CREATE_ACCOUNT);
    frame.assert_usable(RESTORE_BACKUP);
    frame.assert_usable(UNLOCK);
    frame.assert_usable(LOCK);
    frame.assert_usable(EXPORT_BACKUP);
    assert!(frame.paints(NOTICE_TESTNET));
    assert!(frame.paints(NOTICE_PAYMENTS));
    assert!(frame.paints(TITLE) || frame.title.as_deref() == Some(TITLE));
    frame
}

#[test]
fn empty_and_populated_list_paints_notices_and_selection_controls() {
    let (mut app, records, _) = open_window(Vec::new());
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let empty = first_list(&mut ui, &mut app);
    empty.assert_no_substr(RAW_ERROR_CANARY);
    assert!(!empty.paints(LOCKED_ID));
    let after_disabled_unlock = click_label(&mut ui, &mut app, UNLOCK);
    assert!(after_disabled_unlock.paints(CREATE_ACCOUNT));
    assert!(!after_disabled_unlock.paints(UNLOCK_ACCOUNT));
    assert!(records.borrow().unlock_calls.is_empty());

    records.borrow_mut().list_error = Some(RAW_ERROR_CANARY);
    let errored = ui.run(&mut app, Vec::new(), false);
    errored.assert_no_substr(RAW_ERROR_CANARY);
    assert!(errored.paints(UNAVAILABLE_MESSAGE));
    assert!(!errored.paints(LOCKED_ID));
    records.borrow_mut().list_error = None;
    records.borrow_mut().accounts = vec![summary(UNLOCKED_ID, false), summary(LOCKED_ID, true)];

    let populated = ui.run(&mut app, Vec::new(), false);
    populated.assert_usable(LOCKED_ID);
    populated.assert_usable(UNLOCKED_ID);
    populated.assert_usable(STATUS_LOCKED);
    populated.assert_usable(STATUS_UNLOCKED);
    populated.assert_no_substr(RAW_ERROR_CANARY);

    let selected_locked = click_label(&mut ui, &mut app, LOCKED_ID);
    assert!(selected_locked.paints(CREATE_ACCOUNT));
    let unlock_form = click_label(&mut ui, &mut app, UNLOCK);
    unlock_form.assert_usable(UNLOCK_ACCOUNT);
    unlock_form.assert_usable(PASSPHRASE);
    let cancelled = click_label(&mut ui, &mut app, CANCEL);
    assert!(cancelled.paints(CREATE_ACCOUNT));

    let _ = click_label(&mut ui, &mut app, UNLOCKED_ID);
    let locked = click_label(&mut ui, &mut app, LOCK);
    assert_eq!(records.borrow().lock_calls, vec![UNLOCKED_ID.to_owned()]);
    assert!(locked.paints(CREATE_ACCOUNT));
    let _ = click_label(&mut ui, &mut app, UNLOCKED_ID);
    let now_locked = click_label(&mut ui, &mut app, UNLOCK);
    now_locked.assert_usable(UNLOCK_ACCOUNT);
    now_locked.assert_usable(PASSPHRASE);
    let cancelled = click_label(&mut ui, &mut app, CANCEL);
    assert!(cancelled.paints(CREATE_ACCOUNT));
}

#[test]
fn create_form_accepts_matching_unicode_and_rejects_invalid() {
    let (mut app, records, _) = open_window(Vec::new());
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);

    let form = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    form.assert_usable(PASSPHRASE);
    form.assert_usable(CONFIRM_PASSPHRASE);
    form.assert_usable(CREATE);
    form.assert_usable(CANCEL);

    let empty = click_label(&mut ui, &mut app, CREATE);
    assert!(empty.paints(PASSPHRASE));
    assert!(records.borrow().create_calls.is_empty());

    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, "one");
    let _ = focus_field(&mut ui, &mut app, CONFIRM_PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, "two");
    let mismatch = click_label(&mut ui, &mut app, CREATE);
    assert!(mismatch.paints(PASSPHRASE));
    assert!(records.borrow().create_calls.is_empty());
    let _ = click_label(&mut ui, &mut app, CANCEL);

    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    let boundary = format!("{}é", "x".repeat(1_022));
    assert_eq!(boundary.len(), 1_024);
    fill_matching(&mut ui, &mut app, &boundary);
    let boundary_created = click_label(&mut ui, &mut app, CREATE);
    assert_eq!(records.borrow().create_calls.len(), 1);
    assert_eq!(
        records.borrow().create_calls[0],
        boundary.as_bytes().to_vec()
    );
    boundary_created.assert_usable(CREATED_ID);
    boundary_created.assert_no_substr(&boundary);

    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    let oversized = format!("{}é", "x".repeat(1_023));
    assert_eq!(oversized.len(), 1_025);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let rejected_passphrase = ui.run(&mut app, vec![egui::Event::Paste(oversized.clone())], false);
    assert_eq!(rejected_passphrase.bullet_count(), 0);
    rejected_passphrase.assert_no_substr(&oversized);
    let _ = focus_field(&mut ui, &mut app, CONFIRM_PASSPHRASE);
    let rejected_confirmation = ui.run(
        &mut app,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(oversized.clone()))],
        false,
    );
    assert_eq!(rejected_confirmation.bullet_count(), 0);
    rejected_confirmation.assert_no_substr(&oversized);
    let _ = click_label(&mut ui, &mut app, CREATE);
    assert_eq!(records.borrow().create_calls.len(), 1);
    let _ = click_label(&mut ui, &mut app, CANCEL);

    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    fill_matching(&mut ui, &mut app, UNICODE_PASS);
    let created = click_label(&mut ui, &mut app, CREATE);
    assert_eq!(records.borrow().create_calls.len(), 2);
    assert_eq!(
        records.borrow().create_calls[1],
        UNICODE_PASS.as_bytes().to_vec()
    );
    created.assert_usable(CREATED_ID);
    created.assert_usable(STATUS_LOCKED);
    assert!(created.paints(CREATE_ACCOUNT));
    assert!(!created.paints(CONFIRM_PASSPHRASE));
    created.assert_no_substr(UNICODE_PASS);
}

#[test]
fn unlock_wrong_then_correct_clears_password_and_lock_calls() {
    let (mut app, records, _) = open_window(vec![summary(LOCKED_ID, true)]);
    records.borrow_mut().unlock_failures = 1;
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);
    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let form = click_label(&mut ui, &mut app, UNLOCK);
    form.assert_usable(UNLOCK_ACCOUNT);
    form.assert_usable(PASSPHRASE);

    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let typed = type_text(&mut ui, &mut app, WRONG_UNLOCK);
    assert!(typed.bullet_count() == WRONG_UNLOCK.chars().count());
    let failed = click_label(&mut ui, &mut app, UNLOCK_ACCOUNT);
    assert_eq!(records.borrow().unlock_calls.len(), 1);
    assert_eq!(records.borrow().unlock_calls[0].0, LOCKED_ID);
    assert_eq!(
        records.borrow().unlock_calls[0].1,
        WRONG_UNLOCK.as_bytes().to_vec()
    );
    assert!(failed.paints(LOCKED_MESSAGE));
    assert!(!failed.paints(EXPORT_SUCCESS));
    assert!(failed.bullet_count() == 0);
    failed.assert_no_substr(WRONG_UNLOCK);

    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, RIGHT_UNLOCK);
    let unlocked = click_label(&mut ui, &mut app, UNLOCK_ACCOUNT);
    assert_eq!(records.borrow().unlock_calls.len(), 2);
    assert_eq!(
        records.borrow().unlock_calls[1].1,
        RIGHT_UNLOCK.as_bytes().to_vec()
    );
    unlocked.assert_usable(STATUS_UNLOCKED);
    unlocked.assert_usable(LOCKED_ID);
    assert!(unlocked.paints(CREATE_ACCOUNT));
    unlocked.assert_no_substr(RIGHT_UNLOCK);

    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let _ = click_label(&mut ui, &mut app, LOCK);
    assert_eq!(records.borrow().lock_calls, vec![LOCKED_ID.to_owned()]);
}

#[test]
fn export_chooser_cancel_skips_port_and_chosen_path_reaches_port() {
    let (mut app, records, dialogs) = open_window(vec![summary(LOCKED_ID, true)]);
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);
    let _ = click_label(&mut ui, &mut app, LOCKED_ID);

    dialogs.borrow_mut().export_queue.push(None);
    let cancelled = click_label(&mut ui, &mut app, EXPORT_BACKUP);
    assert_eq!(dialogs.borrow().export_calls, 1);
    assert!(records.borrow().export_calls.is_empty());
    cancelled.assert_no_substr(EXPORT_PATH_CANARY);
    assert!(cancelled.paints(CREATE_ACCOUNT));

    dialogs
        .borrow_mut()
        .export_queue
        .push(Some(PathBuf::from(EXPORT_PATH_CANARY)));
    let exported = click_label(&mut ui, &mut app, EXPORT_BACKUP);
    assert_eq!(dialogs.borrow().export_calls, 2);
    assert_eq!(records.borrow().export_calls.len(), 1);
    assert_eq!(records.borrow().export_calls[0].0, LOCKED_ID);
    assert_eq!(
        records.borrow().export_calls[0].1,
        PathBuf::from(EXPORT_PATH_CANARY)
    );
    assert!(exported.paints(EXPORT_SUCCESS));
    exported.assert_no_substr(EXPORT_PATH_CANARY);

    dialogs.borrow_mut().panic_export = true;
    let before_lock_all = records.borrow().lock_all_calls;
    let panicked = click_label(&mut ui, &mut app, EXPORT_BACKUP);
    assert!(records.borrow().lock_all_calls > before_lock_all);
    panicked.assert_no_substr("CANARY_DIALOG_PANIC");
    panicked.assert_no_substr(EXPORT_PATH_CANARY);
}

#[test]
fn restore_requires_distinct_confirm_and_cannot_replay() {
    let (mut app, records, dialogs) = open_window(Vec::new());
    let control = app.control();
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);

    dialogs.borrow_mut().restore_queue.push(None);
    let cancelled_chooser = click_label(&mut ui, &mut app, RESTORE_BACKUP);
    assert_eq!(dialogs.borrow().restore_calls, 1);
    assert!(records.borrow().prepare_calls.is_empty());
    assert_eq!(records.borrow().confirm_calls, 0);
    assert!(cancelled_chooser.paints(CREATE_ACCOUNT));

    dialogs
        .borrow_mut()
        .restore_queue
        .push(Some(PathBuf::from(RESTORE_PATH_CANARY)));
    let inspect_form = click_label(&mut ui, &mut app, RESTORE_BACKUP);
    inspect_form.assert_usable(INSPECT_BACKUP);
    inspect_form.assert_usable(PASSPHRASE);
    inspect_form.assert_no_substr(RESTORE_PATH_CANARY);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, RESTORE_PASS);
    let metadata = click_label(&mut ui, &mut app, INSPECT_BACKUP);
    assert_eq!(records.borrow().prepare_calls.len(), 1);
    assert_eq!(
        records.borrow().prepare_calls[0].0,
        PathBuf::from(RESTORE_PATH_CANARY)
    );
    assert_eq!(
        records.borrow().prepare_calls[0].1,
        RESTORE_PASS.as_bytes().to_vec()
    );
    metadata.assert_usable(CONFIRM_RESTORE);
    metadata.assert_usable(RESTORE_ID);
    metadata.assert_usable("zec-testnet");
    assert!(metadata.bullet_count() == 0);
    metadata.assert_no_substr(RESTORE_PASS);
    metadata.assert_no_substr(RESTORE_PATH_CANARY);
    let confirm_rect = metadata.visible_label(CONFIRM_RESTORE);
    let after_cancel = click_label(&mut ui, &mut app, CANCEL);
    assert_eq!(records.borrow().confirm_calls, 0);
    assert!(after_cancel.paints(CREATE_ACCOUNT));
    assert!(!after_cancel.paints(CONFIRM_RESTORE));
    let replay = click(&mut ui, &mut app, confirm_rect.center());
    assert_eq!(records.borrow().confirm_calls, 0);
    assert!(!replay.paints(CONFIRM_RESTORE));
    if replay.paints(CONFIRM_PASSPHRASE) {
        let restored_list = click_label(&mut ui, &mut app, CANCEL);
        restored_list.assert_usable(RESTORE_BACKUP);
    } else {
        replay.assert_usable(RESTORE_BACKUP);
    }

    dialogs
        .borrow_mut()
        .restore_queue
        .push(Some(PathBuf::from(RESTORE_PATH_CANARY)));
    let _ = click_label(&mut ui, &mut app, RESTORE_BACKUP);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, RESTORE_PASS);
    let pending_escape = click_label(&mut ui, &mut app, INSPECT_BACKUP);
    pending_escape.assert_usable(CONFIRM_RESTORE);
    let escaped = ui.run(&mut app, vec![escape()], false);
    assert_eq!(records.borrow().confirm_calls, 0);
    assert!(escaped.cancel_close >= 1);
    assert!(escaped.visible.contains(&false));
    assert_eq!(escaped.close, 0);
    assert!(control.request_open());
    let reopened = ui.run(&mut app, Vec::new(), false);
    assert!(reopened.paints(CREATE_ACCOUNT));
    assert!(!reopened.paints(CONFIRM_RESTORE));
    reopened.assert_no_substr(RESTORE_PASS);

    dialogs
        .borrow_mut()
        .restore_queue
        .push(Some(PathBuf::from(RESTORE_PATH_CANARY)));
    let _ = click_label(&mut ui, &mut app, RESTORE_BACKUP);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, RESTORE_PASS);
    let pending_close = click_label(&mut ui, &mut app, INSPECT_BACKUP);
    pending_close.assert_usable(CONFIRM_RESTORE);
    let closed = ui.run(&mut app, Vec::new(), true);
    assert_eq!(records.borrow().confirm_calls, 0);
    assert!(closed.cancel_close >= 1);
    assert!(closed.visible.contains(&false));
    assert_eq!(closed.close, 0);
    assert!(control.request_open());
    let reopened_after_close = ui.run(&mut app, Vec::new(), false);
    assert!(reopened_after_close.paints(CREATE_ACCOUNT));
    assert!(!reopened_after_close.paints(CONFIRM_RESTORE));
    reopened_after_close.assert_no_substr(RESTORE_PASS);

    dialogs
        .borrow_mut()
        .restore_queue
        .push(Some(PathBuf::from(RESTORE_PATH_CANARY)));
    let _ = click_label(&mut ui, &mut app, RESTORE_BACKUP);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, RESTORE_PASS);
    let ready = click_label(&mut ui, &mut app, INSPECT_BACKUP);
    ready.assert_usable(CONFIRM_RESTORE);
    let successful_confirm_rect = ready.visible_label(CONFIRM_RESTORE);
    let confirmed = click(&mut ui, &mut app, successful_confirm_rect.center());
    assert_eq!(records.borrow().prepare_calls.len(), 4);
    assert_eq!(records.borrow().confirm_calls, 1);
    confirmed.assert_usable(RESTORE_ID);
    confirmed.assert_usable(STATUS_LOCKED);
    assert!(confirmed.paints(CREATE_ACCOUNT));
    assert!(!confirmed.paints(CONFIRM_RESTORE));
    confirmed.assert_no_substr(RESTORE_PATH_CANARY);
    let again = click(&mut ui, &mut app, successful_confirm_rect.center());
    assert_eq!(records.borrow().confirm_calls, 1);
    assert!(again.paints(CREATE_ACCOUNT));
    assert!(!again.paints(CONFIRM_RESTORE));
}

#[test]
fn copy_cut_undo_and_platform_events_never_expose_secret() {
    let (mut app, records, _) = open_window(Vec::new());
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);
    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);

    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let pasted = ui.run(
        &mut app,
        vec![egui::Event::Paste(SECRET_CANARY.to_owned())],
        false,
    );
    assert!(pasted.bullet_count() == SECRET_CANARY.chars().count());
    pasted.assert_no_substr(SECRET_CANARY);
    let copied = ui.run(&mut app, vec![egui::Event::Copy], false);
    copied.assert_no_substr(SECRET_CANARY);
    let cut = ui.run(&mut app, vec![egui::Event::Cut], false);
    cut.assert_no_substr(SECRET_CANARY);
    let copy_key = ui.run(
        &mut app,
        vec![key(egui::Key::C, egui::Modifiers::COMMAND)],
        false,
    );
    copy_key.assert_no_substr(SECRET_CANARY);
    let cut_key = ui.run(
        &mut app,
        vec![key(egui::Key::X, egui::Modifiers::COMMAND)],
        false,
    );
    cut_key.assert_no_substr(SECRET_CANARY);

    let selected = ui.run(
        &mut app,
        vec![key(egui::Key::A, egui::Modifiers::COMMAND)],
        false,
    );
    selected.assert_no_substr(SECRET_CANARY);
    let deleted = ui.run(
        &mut app,
        vec![key(egui::Key::Delete, egui::Modifiers::NONE)],
        false,
    );
    assert!(deleted.bullet_count() == 0);
    let undone = ui.run(
        &mut app,
        vec![key(egui::Key::Z, egui::Modifiers::COMMAND)],
        false,
    );
    assert!(undone.bullet_count() == 0);
    undone.assert_no_substr(SECRET_CANARY);

    let _ = click_label(&mut ui, &mut app, CANCEL);
    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    let stale = click_label(&mut ui, &mut app, CREATE);
    assert!(records.borrow().create_calls.is_empty());
    assert!(stale.paints(PASSPHRASE));

    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let ime = ui.run(
        &mut app,
        vec![egui::Event::Ime(egui::ImeEvent::Commit(
            SECRET_CANARY.to_owned(),
        ))],
        false,
    );
    assert!(ime.bullet_count() == SECRET_CANARY.chars().count());
    ime.assert_no_substr(SECRET_CANARY);
    let _ = click_label(&mut ui, &mut app, CANCEL);
    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    let _ = click_label(&mut ui, &mut app, CREATE);
    assert!(records.borrow().create_calls.is_empty());

    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let typed = type_chars(&mut ui, &mut app, "aßc");
    assert!(typed.bullet_count() == 3);
    typed.assert_no_substr("aßc");
    let after_backspace = ui.run(
        &mut app,
        vec![key(egui::Key::Backspace, egui::Modifiers::NONE)],
        false,
    );
    assert!(after_backspace.bullet_count() == 2);
    let _ = ui.run(
        &mut app,
        vec![key(egui::Key::Tab, egui::Modifiers::NONE)],
        false,
    );
    let _ = type_chars(&mut ui, &mut app, "aß");
    let created = click_label(&mut ui, &mut app, CREATE);
    assert_eq!(records.borrow().create_calls.len(), 1);
    assert_eq!(records.borrow().create_calls[0], "aß".as_bytes().to_vec());
    created.assert_usable(CREATED_ID);
    created.assert_no_substr("aß");
}

#[test]
fn window_control_reopen_quit_drop_and_closed_list_errors() {
    let (mut app, records, _) = open_window(vec![summary(LOCKED_ID, true)]);
    let control = app.control();
    assert!(!control.request_open());
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let first = first_list(&mut ui, &mut app);
    assert!(first.focus == 0);
    assert!(control.request_open());
    let opened = ui.run(&mut app, Vec::new(), false);
    assert!(opened.focus >= 1);
    assert!(opened.visible.contains(&true));

    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, SECRET_CANARY);
    let lists_before_hide = records.borrow().list_calls;
    let lock_all_before_hide = records.borrow().lock_all_calls;
    let hidden = ui.run(&mut app, vec![escape()], false);
    assert!(hidden.cancel_close >= 1);
    assert!(hidden.visible.contains(&false));
    assert_eq!(hidden.close, 0);
    assert!(records.borrow().lock_all_calls > lock_all_before_hide);
    hidden.assert_no_substr(SECRET_CANARY);
    let hidden_tick = ui.run_logic(&mut app, false);
    assert!(records.borrow().list_calls > lists_before_hide);
    assert_eq!(hidden_tick.focus, 0);
    assert!(hidden_tick.repaint_ms <= 1_000);
    assert!(control.request_open());
    let reopened = ui.run(&mut app, Vec::new(), false);
    assert!(reopened.paints(CREATE_ACCOUNT));
    assert!(!reopened.paints(CONFIRM_PASSPHRASE));
    reopened.assert_usable(LOCKED_ID);
    reopened.assert_no_substr(SECRET_CANARY);
    assert!(records.borrow().create_calls.is_empty());

    records.borrow_mut().list_error = Some(RAW_ERROR_CANARY);
    let errored = ui.run(&mut app, Vec::new(), false);
    errored.assert_no_substr(RAW_ERROR_CANARY);
    assert!(errored.paints(UNAVAILABLE_MESSAGE));
    assert!(!errored.paints(LOCKED_ID));
    records.borrow_mut().list_error = None;

    control.request_quit();
    assert!(!control.request_open());
    let lock_all_before_quit = records.borrow().lock_all_calls;
    let quitting = ui.run(&mut app, Vec::new(), false);
    assert!(quitting.close >= 1);
    assert_eq!(quitting.cancel_close, 0);
    assert!(records.borrow().lock_all_calls > lock_all_before_quit);

    let (drop_app, drop_records, _) = open_window(vec![summary(LOCKED_ID, true)]);
    let mut drop_ui = PersistentUi::new(NATIVE_SIZE);
    let mut drop_app = drop_app;
    let _ = first_list(&mut drop_ui, &mut drop_app);
    let lock_all_before_drop = drop_records.borrow().lock_all_calls;
    drop(drop_app);
    assert!(drop_records.borrow().lock_all_calls > lock_all_before_drop);
}

#[test]
fn small_and_tall_layouts_keep_action_controls_usable() {
    let accounts: Vec<AccountSummary> = (0..16)
        .map(|index| summary(&format!("{index:032x}"), true))
        .collect();
    for size in [SMALL_SIZE, NATIVE_SIZE] {
        let (mut app, _, _) = open_window(accounts.clone());
        let mut ui = PersistentUi::new(size);
        let frame = first_list(&mut ui, &mut app);
        frame.assert_usable(CREATE_ACCOUNT);
        frame.assert_usable(RESTORE_BACKUP);
        frame.assert_usable(UNLOCK);
        frame.assert_usable(LOCK);
        frame.assert_usable(EXPORT_BACKUP);
        frame.assert_usable(NOTICE_TESTNET);
        frame.assert_usable(NOTICE_PAYMENTS);
        let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
        let form = ui.run(&mut app, Vec::new(), false);
        form.assert_usable(CREATE);
        form.assert_usable(CANCEL);
        form.assert_usable(PASSPHRASE);
    }
}

#[test]
fn wal014_receive_scene_issues_once_paints_honestly_and_copies_release_output() {
    let (mut app, records, _) =
        open_window(vec![summary(LOCKED_ID, true), summary(UNLOCKED_ID, false)]);
    let first = fake_receiver(UNLOCKED_ID, &UI_SEED_A, 0, 1);
    let second = fake_receiver(UNLOCKED_ID, &UI_SEED_A, 1, 2);
    let first_address = first.receiver.clone();
    let second_address = second.receiver.clone();
    records
        .borrow_mut()
        .receive_results
        .extend([Ok(first), Ok(second)]);
    let mut ui = PersistentUi::new(SMALL_SIZE);
    let _ = first_list(&mut ui, &mut app);

    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let locked_click = click_label(&mut ui, &mut app, RECEIVE);
    assert!(locked_click.paints(CREATE_ACCOUNT));
    assert!(records.borrow().receive_calls.is_empty());

    let _ = click_label(&mut ui, &mut app, UNLOCKED_ID);
    let scene = click_label(&mut ui, &mut app, RECEIVE);
    assert_eq!(records.borrow().receive_calls, vec![UNLOCKED_ID]);
    scene.assert_usable(RECEIVE_TITLE);
    scene.assert_usable(RECEIVE_NETWORK);
    scene.assert_usable(UNLOCKED_ID);
    scene.assert_usable(&first_address);
    scene.assert_usable(BALANCE_UNAVAILABLE);
    scene.assert_usable(COPY_ADDRESS);
    scene.assert_usable(BACK);
    assert!(scene.copy_texts.is_empty());
    assert!(!scene.painted.iter().any(|(text, _, _)| {
        text.starts_with("Balance") && text.chars().any(|character| character.is_ascii_digit())
    }));
    assert!(!scene.paints("Sync ready"));
    assert!(!scene.paints("QR code"));
    assert_testnet_orchard(&first_address);

    let repaint = ui.run(&mut app, Vec::new(), false);
    assert_eq!(records.borrow().receive_calls, vec![UNLOCKED_ID]);
    assert!(repaint.copy_texts.is_empty());
    repaint.assert_usable(&first_address);

    let copy_rect = repaint.visible_label(COPY_ADDRESS);
    let (copy_release, copy_settled) =
        click_release_and_settle(&mut ui, &mut app, copy_rect.center());
    assert_eq!(copy_release.copy_texts, vec![first_address.clone()]);
    assert!(copy_settled.copy_texts.is_empty());
    assert_eq!(records.borrow().receive_calls, vec![UNLOCKED_ID]);

    let list = click_label(&mut ui, &mut app, BACK);
    assert!(list.paints(CREATE_ACCOUNT));
    assert!(!list.paints(&first_address));
    let _ = click_label(&mut ui, &mut app, UNLOCKED_ID);
    let next = click_label(&mut ui, &mut app, RECEIVE);
    assert_eq!(
        records.borrow().receive_calls,
        vec![UNLOCKED_ID.to_owned(), UNLOCKED_ID.to_owned()]
    );
    next.assert_usable(&second_address);
    assert!(!next.paints(&first_address));
    assert!(next.copy_texts.is_empty());
}

#[test]
fn wal014_receive_state_clears_on_lock_catalog_change_error_selection_and_hide() {
    let (mut app, records, _) =
        open_window(vec![summary(LOCKED_ID, false), summary(UNLOCKED_ID, false)]);
    let issued = [
        fake_receiver(LOCKED_ID, &UI_SEED_A, 0, 1),
        fake_receiver(LOCKED_ID, &UI_SEED_A, 1, 2),
        fake_receiver(LOCKED_ID, &UI_SEED_A, 2, 3),
        fake_receiver(UNLOCKED_ID, &UI_SEED_B, 0, 1),
        fake_receiver(LOCKED_ID, &UI_SEED_A, 3, 4),
    ];
    let addresses: Vec<String> = issued
        .iter()
        .map(|receiver| receiver.receiver.clone())
        .collect();
    records
        .borrow_mut()
        .receive_results
        .extend(issued.into_iter().map(Ok));
    let control = app.control();
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);

    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let first_scene = click_label(&mut ui, &mut app, RECEIVE);
    first_scene.assert_usable(COPY_ADDRESS);
    first_scene.assert_usable(&addresses[0]);
    records
        .borrow_mut()
        .accounts
        .iter_mut()
        .find(|account| account.account_id == LOCKED_ID)
        .unwrap()
        .locked = true;
    let expired = ui.run(&mut app, Vec::new(), false);
    assert!(!expired.paints(&addresses[0]));
    assert!(!expired.paints(COPY_ADDRESS));
    assert!(
        ui.run(&mut app, vec![egui::Event::Copy], false)
            .copy_texts
            .is_empty()
    );

    records
        .borrow_mut()
        .accounts
        .iter_mut()
        .find(|account| account.account_id == LOCKED_ID)
        .unwrap()
        .locked = false;
    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let second_scene = click_label(&mut ui, &mut app, RECEIVE);
    second_scene.assert_usable(&addresses[1]);
    records.borrow_mut().list_error = Some(RAW_ERROR_CANARY);
    let list_failed = ui.run(&mut app, Vec::new(), false);
    list_failed.assert_no_substr(RAW_ERROR_CANARY);
    assert!(list_failed.paints(UNAVAILABLE_MESSAGE));
    assert!(!list_failed.paints(&addresses[1]));
    assert!(!list_failed.paints(COPY_ADDRESS));

    records.borrow_mut().list_error = None;
    let _ = ui.run(&mut app, Vec::new(), false);
    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let third_scene = click_label(&mut ui, &mut app, RECEIVE);
    third_scene.assert_usable(&addresses[2]);
    records
        .borrow_mut()
        .accounts
        .retain(|account| account.account_id != LOCKED_ID);
    let removed = ui.run(&mut app, Vec::new(), false);
    assert!(!removed.paints(&addresses[2]));
    assert!(!removed.paints(COPY_ADDRESS));

    let _ = click_label(&mut ui, &mut app, UNLOCKED_ID);
    let other_scene = click_label(&mut ui, &mut app, RECEIVE);
    other_scene.assert_usable(&addresses[3]);
    let back = click_label(&mut ui, &mut app, BACK);
    assert!(!back.paints(&addresses[3]));
    records
        .borrow_mut()
        .accounts
        .push(summary(LOCKED_ID, false));
    let _ = ui.run(&mut app, Vec::new(), false);
    let selected_other = click_label(&mut ui, &mut app, LOCKED_ID);
    assert!(!selected_other.paints(&addresses[3]));
    assert!(!selected_other.paints(COPY_ADDRESS));

    let hidden_scene = click_label(&mut ui, &mut app, RECEIVE);
    hidden_scene.assert_usable(&addresses[4]);
    let hidden = ui.run(&mut app, vec![escape()], false);
    assert!(hidden.visible.contains(&false));
    assert!(!hidden.paints(&addresses[4]));
    assert!(!hidden.paints(COPY_ADDRESS));
    assert!(control.request_open());
    let reopened = ui.run(&mut app, Vec::new(), false);
    assert!(reopened.paints(CREATE_ACCOUNT));
    assert!(!reopened.paints(&addresses[4]));
    assert!(!reopened.paints(COPY_ADDRESS));

    records
        .borrow_mut()
        .receive_results
        .push_back(Err(RAW_ERROR_CANARY));
    records
        .borrow_mut()
        .accounts
        .iter_mut()
        .find(|account| account.account_id == LOCKED_ID)
        .unwrap()
        .locked = false;
    let _ = ui.run(&mut app, Vec::new(), false);
    let _ = click_label(&mut ui, &mut app, LOCKED_ID);
    let receive_failed = click_label(&mut ui, &mut app, RECEIVE);
    receive_failed.assert_no_substr(RAW_ERROR_CANARY);
    assert!(receive_failed.paints(UNAVAILABLE_MESSAGE));
    assert!(!receive_failed.paints(COPY_ADDRESS));
}

#[test]
fn wal014_shared_port_pointer_receive_and_copy_persist_real_issuance() {
    let mut scratch = Scratch::new("wal014-real-receive");
    let manager = LocalAccountManager::open(&scratch.root).expect("open local manager");
    let port = SharedAccountPort::new(Arc::new(Mutex::new(manager)));
    let (dialogs, _) = FakeDialogs::new(DialogState::default());
    let mut app = AccountWindow::new(port, dialogs, true);
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);

    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    fill_matching(&mut ui, &mut app, UNICODE_PASS);
    let created = click_label(&mut ui, &mut app, CREATE);
    created.assert_no_substr(UNICODE_PASS);
    let vaults: Vec<_> = walk_regular_files(&scratch.accounts())
        .into_iter()
        .filter(|(path, _)| path.extension().and_then(|ext| ext.to_str()) == Some("vault"))
        .collect();
    assert_eq!(vaults.len(), 1);
    let (vault_path, vault_bytes) = &vaults[0];
    let account_id = vault_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("vault stem")
        .to_owned();
    scratch.track_zec_account(&account_id);
    let mut passphrase = SecretBytes::new(UNICODE_PASS.as_bytes().to_vec()).unwrap();
    let opened = open_vault_bytes(vault_bytes, &mut passphrase, &mut Work, &mut Ignore)
        .expect("open persisted vault");
    let expected = opened.expose(|seed| oracle_receiver(seed, 0));

    let _ = click_label(&mut ui, &mut app, &account_id);
    let _ = click_label(&mut ui, &mut app, UNLOCK);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, UNICODE_PASS);
    let unlocked = click_label(&mut ui, &mut app, UNLOCK_ACCOUNT);
    unlocked.assert_usable(STATUS_UNLOCKED);
    let _ = click_label(&mut ui, &mut app, &account_id);
    let received = click_label(&mut ui, &mut app, RECEIVE);
    received.assert_usable(&expected);
    received.assert_usable(BALANCE_UNAVAILABLE);
    received.assert_no_substr(UNICODE_PASS);
    assert!(!received.painted.iter().any(|(text, _, _)| {
        text.starts_with("Balance") && text.chars().any(|character| character.is_ascii_digit())
    }));
    assert_testnet_orchard(&expected);

    let copy_rect = received.visible_label(COPY_ADDRESS);
    let (copy_release, copy_settled) =
        click_release_and_settle(&mut ui, &mut app, copy_rect.center());
    assert_eq!(copy_release.copy_texts, vec![expected.clone()]);
    assert!(copy_settled.copy_texts.is_empty());

    let wallet_db = scratch
        .root
        .join("zec-testnet")
        .join(&account_id)
        .join("wallet.sqlite3");
    let connection = Connection::open(&wallet_db).unwrap();
    let state: (Option<i64>, i64) = connection
        .query_row(
            "SELECT r.last_diversifier_index, s.issued_at_sequence
             FROM ext_bitbook_receiver_state r
             JOIN ext_bitbook_sequence_state s USING (account_id)
             WHERE r.account_id = ?1",
            [&account_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(state, (Some(0), 1));
}

#[test]
fn shared_port_and_local_manager_create_unlock_lock_and_restart() {
    let scratch = Scratch::new("compose");
    assert_eq!(mode(&scratch.root), 0o700);
    let manager = LocalAccountManager::open(&scratch.root).expect("open local manager");
    let port = SharedAccountPort::new(Arc::new(Mutex::new(manager)));
    let (dialogs, _) = FakeDialogs::new(DialogState::default());
    let mut app = AccountWindow::new(port, dialogs, true);
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let _ = first_list(&mut ui, &mut app);

    let _ = click_label(&mut ui, &mut app, CREATE_ACCOUNT);
    fill_matching(&mut ui, &mut app, UNICODE_PASS);
    let created = click_label(&mut ui, &mut app, CREATE);
    created.assert_no_substr(UNICODE_PASS);
    created.assert_usable(STATUS_LOCKED);
    assert_eq!(mode(&scratch.accounts()), 0o700);

    let vaults: Vec<_> = walk_regular_files(&scratch.accounts())
        .into_iter()
        .filter(|(path, _)| path.extension().and_then(|ext| ext.to_str()) == Some("vault"))
        .collect();
    assert_eq!(vaults.len(), 1);
    let (vault_path, vault_bytes) = &vaults[0];
    assert_eq!(mode(vault_path), 0o600);
    let account_id = vault_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("vault stem")
        .to_owned();
    created.assert_usable(&account_id);

    let mut passphrase = SecretBytes::new(UNICODE_PASS.as_bytes().to_vec()).unwrap();
    let opened = open_vault_bytes(vault_bytes, &mut passphrase, &mut Work, &mut Ignore)
        .expect("open persisted vault");
    assert_eq!(opened.expose(|seed| seed.len()), 32);
    let parsed = parse_vault(vault_bytes, &mut Work).unwrap();
    opened.expose(|seed| assert_ne!(parsed.ciphertext(), seed));

    let _ = click_label(&mut ui, &mut app, &account_id);
    let _ = click_label(&mut ui, &mut app, UNLOCK);
    let _ = focus_field(&mut ui, &mut app, PASSPHRASE);
    let _ = type_text(&mut ui, &mut app, UNICODE_PASS);
    let unlocked = click_label(&mut ui, &mut app, UNLOCK_ACCOUNT);
    unlocked.assert_usable(STATUS_UNLOCKED);
    unlocked.assert_no_substr(UNICODE_PASS);
    let _ = click_label(&mut ui, &mut app, &account_id);
    let locked = click_label(&mut ui, &mut app, LOCK);
    locked.assert_usable(STATUS_LOCKED);

    drop(app);
    let restarted = LocalAccountManager::open(&scratch.root).expect("reopen local manager");
    let port = SharedAccountPort::new(Arc::new(Mutex::new(restarted)));
    let (dialogs, _) = FakeDialogs::new(DialogState::default());
    let mut app = AccountWindow::new(port, dialogs, true);
    let mut ui = PersistentUi::new(NATIVE_SIZE);
    let listed = first_list(&mut ui, &mut app);
    listed.assert_usable(&account_id);
    listed.assert_usable(STATUS_LOCKED);
    listed.assert_no_substr(UNICODE_PASS);

    for (_, bytes) in walk_regular_files(&scratch.root) {
        assert!(
            !bytes
                .windows(UNICODE_PASS.len())
                .any(|window| window == UNICODE_PASS.as_bytes()),
            "passphrase leaked into persisted bytes"
        );
    }
}
