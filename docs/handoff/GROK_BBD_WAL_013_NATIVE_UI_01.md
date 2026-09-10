# WAL-013 native account window — tests first

Reviewer fixes these semantics. Grok grok-4.6 High, no subagents. Read AGENTS.md,
TESTING.md, this file, GROK_BBD_WAL_013_ACCOUNTS_01.md and listed source only:
wallet-broker/src/{lib.rs,native_ui.rs,native.rs,vault.rs}, existing
src/native_ui/zec_native_app_tests.rs and installed egui/eframe0.36.1 APIs as needed.
Account service tests are independently active: never modify them or Cargo.toml.
ONLY write NEW wallet-broker/tests/account_native_ui.rs (Cargo automatically discovers
integration tests); put #![cfg(feature = "native-ui")] at top. No production/stubs,
execution/formatting, Git, evidence, other actors or web. Stop with hashes/counts.

Later production ONLY new wallet-broker/src/account_ui.rs and feature-gated pub mod
account_ui in lib.rs. Existing dependencies unchanged. Reviewer will merge lib
registration authorization sequentially after service source; no concurrent lib edits.

## Fixed public Rust interfaces

Use existing AccountSummary and SecretBytes. No Electron/preload/wire access to these
privileged APIs. New AccountUiPort trait (no Send bound) has associated type Restore:
list(&mut self)->Result<Vec<AccountSummary>, &'static str>;
create(&mut self, passphrase:SecretBytes)->Result<AccountSummary,&'static str>;
unlock(&mut self,id:&str,passphrase:SecretBytes)->Result<(),&'static str>;
lock(&mut self,id:&str)->Result<(),&'static str>;
lock_all(&mut self);
export(&mut self,id:&str,path:&std::path::Path)->Result<(),&'static str>;
prepare_restore(&mut self,path:&Path,passphrase:SecretBytes)->Result<Self::Restore,&'static str>;
restore_summary(&self,prepared:&Self::Restore)->AccountSummary;
confirm_restore(&mut self,prepared:Self::Restore)->Result<AccountSummary,&'static str>.
Cancellation simply drops the one-shot prepared value, without calling confirm.
Production SharedAccountPort<C,E,W>::new(Arc<Mutex<AccountManager<C,E,W>>>) wraps the
actual service, uses NativeSurface for every privileged call, maps errors to code(),
never holds mutex while rendering/dialog selection, and locks all on poisoned mutex
by recovering solely for lock_all then returning UNAVAILABLE. list invokes service
list/tick; no periodic extension. Production port must not log or serialize secrets.
Only production clock/entropy/wipes need Send for runtime sharing; trait/UI tests may
use Rc records. SharedAccountPort implements AccountUiPort with PreparedRestore.
AccountDialogs: export_path(&mut self)->Option<PathBuf>, restore_path(&mut self)->Option<PathBuf>.
NativeAccountDialogs uses pinned rfd file dialogs; no shell/env/path on wire.

AccountWindow<P:AccountUiPort,D:AccountDialogs>::new(port:P,dialogs:D,
initially_visible:bool)->Self. Implements eframe::App. control(&self)->AccountWindowControl.
Cloneable control owns only visibility/quit atomics and optional egui Context, never
port/state/secrets. request_open(&self)->bool: false until context registered or after
quit; otherwise request visible/focus and repaint (no authorization). request_quit
sets irreversible quit+repaint. Window registers context from logic/ui on first pass.
Production initially_visible=false; tests may true (visibility grants no custody).
logic and ui both service pending control/close consistently; logic ticks even hidden,
request repaint after <=1second. Actual eframe0.36.1 hidden logic runs on repaint.
Escape or native X close: wipe pending passwords, drop prepared approval, lock_all,
CancelClose+Visible(false). request_open subsequently shows list, no stale form or
approval. Programmatic quit locks/drops and sends Close without CancelClose; Drop
also locks all and wipes passwords. No process::exit from UI. Never request focus
on periodic ticks, only explicit open. Closing a form Cancel clears it but need not
hide the whole window. Any list/port error clears stale account rows and shows a fixed
safe message; fake arbitrary error strings are never rendered. Generic errors can
map recognized codes to fixed text. No paths, raw error, secret or backup data drawn.

## Actual widgets and user behavior

Title "BitBook accounts"; visible notice "Zcash testnet accounts" and "Payments are not enabled".
List existing sorted account IDs with Locked/Unlocked, select one using actual UI.
Buttons exact visible labels: "Create account", "Restore backup", "Unlock", "Lock",
"Export backup". Selected-account controls disabled without appropriate account;
Unlock only locked; Lock only unlocked; export allowed locked. Clicking Create opens
form with two masked fields labelled "Passphrase" and "Confirm passphrase", and
"Create"/"Cancel". Require nonempty <=1024 UTF8 bytes and exact matching; mismatches
never invoke port. Create success returns to refreshed list locked. No auto-unlock.
Unlock form one masked "Passphrase", "Unlock account"/"Cancel"; failed unlock clears
password and fixed message, no success. Successful unlock refreshes actual list.
Restore backup opens native chooser; cancel no port. Selected path retained privately,
then masked passphrase, "Inspect backup"/"Cancel". Successful prepare clears secret
and shows returned account ID/network and separate "Confirm restore"/"Cancel".
Only clicking Confirm restore consumes opaque prepared value and writes. Never
re-read chosen source on confirm. Cancellation/close cannot replay approval.
Export backup uses chooser; cancellation no port; successful export fixed success
message. Native-dialog panic/error safely clears operation and locks all; no raw error.
Use scrollable body and visible usable action buttons at360x480 and520x720. No balances,
addresses, seed display/copy/export, payment controls or remote dependencies.

## Password handling

Do NOT pass actual passphrase to egui TextEdit: pinned egui stores password text in
ordinary String undo history even when password(true). Implement a small masked native
entry widget owning Zeroizing<String>; accepts focused Text/Paste/IME commit events
up to1024 UTF8 bytes, backspace one Unicode scalar, Ctrl/Cmd+A select-all replacement,
Delete when selected, pointer/keyboard focus. Paint ONLY bullets; no secret WidgetInfo,
clipboard/copy/cut/undo/accessibility output, UI context storage or logs. Do not clone
secret input events. Take/consume and zeroize handled Text/Paste/IME String events from
ctx.input_mut (including its raw event copies when present); zeroize on rejection,
submit,cancel,scene change,close,Drop. Native event backend may hold transient input;
no claim of erasing OS buffers. Masked field focus indication and label are required.
No plaintext is handed to port except ownership of SecretBytes at explicit submission.
No secret getters or testing authority. Tests observe port input inside fake method,
not through production getter. Empty confirm and wrongpass failure clear all buffers.

## Focused tests (~8-10)

Use persistent egui Context with real pointer/key events through eframe::App::ui;
locate actual painted labels/button rectangles, assert controls present and within
viewport. Do not set production form/submitted flags directly. Fake port via Rc
records with consumable non-Clone Restore; fake dialogs queue paths/cancel. Tests:
1 empty list displays testnet notice and useful Create/Restore controls, no raw fake
error paths; nonempty Locked/Unlocked rows and deterministic selection controls.
2 actual Create widgets accept unicode passphrase+matching confirmation, call create
once with exact secret, then locked refreshed row; mismatch/empty/over1024 never call.
3 Unlock wrong then correct, password cleared between attempts, Lock actually calls.
4 export chooser cancel makes zero export calls; chosen path reaches native port only.
5 restore chooser/password/inspect shows metadata then requires distinct confirm;
cancel,Escape,windowclose cannot commit/replay; fresh successful confirm once only.
6 Copy/Cut/Undo cannot expose secret; painted output/platform clipboard commands and
accessibility output never contain canary; cancelled/reopened form cannot submit old
password. Normal focus/key transitions and Unicode backspace work.
7 windowcontrol reopen after hide yields clean list; close locksall; quit notcancelled;
Drop locksall. list refresh/tick doesn't synthesize authority; closed errors clearrows.
8 small/tall layouts real controls non-vacuously reachable (scroll as needed).
9 production SharedAccountPort/real LocalAccountManager composition: owned0700 root
under wallet-broker/target, actual UI Create then Unlock/Lock and restart read locked;
independent open_vault_bytes authenticates persisted vault (seed length32), no cleartext
in outputs/files. Cleanup individually owned files/dirs, no recursivedelete.
This last test waits for actual service source before green; import missing is red.
Hermes red later cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline
--no-default-features --features native-ui --test account_native_ui (Rust1.98).
Expected missing account_ui import. Green same; existing native_surface/native widget
regressions scoped by reviewer. Falsify confirm-required or close-clear mechanism and
prove corresponding actual-widget test fails, restore. Runtime/process/menu follows
separate contract; do not substitute unit UI proof for whole user flow.
