# WAL-013 native account window production — authorized

Reviewer accepts Hermes session20260910_104514_8fed69: 84 distinct service tests green,
native UI red solely E0432 absent account_ui, both native-origin and idle-deadline
falsifications detected, restored and green. All final hashes independently match;
three formatter diffs contain only formatting. Transcript contains the authorized
launcher, completion waits/log and evidence reads. No tests executed by reviewer.

Sol gpt-5.6-sol High source-only, no subagents. This continues recorded escalation:
Grok native UI actor stopped at32 turns without usable drop. Corrected nine widget
tests now compile through missing-module red. Implement only NEW
wallet-broker/src/account_ui.rs and feature-gated pub mod account_ui in lib.rs.
No test edits, runtime, dependencies, Git, execution, formatting or evidence writes.
Read AGENTS.md, TESTING.md and GROK_BBD_WAL_013_NATIVE_UI_01.md for the fixed complete
contract, accounts.rs and account_native_ui.rs, relevant installed egui/eframe APIs.
Do not reread unrelated history or invoke any other actor. No redesign or shortcuts.

Exact baseline accounts.rs f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1;
lib.rs 96bf49de102f16c026fb3d098cb3b447a269dc5f9176fd29c902ac55d44e7246;
UI tests 5a0facc561cd51b4f5ece6854df1a086ca8cd4a272cf92a45b08abfea0151abe.

Production guidance: custom masked input must also advertise focused native IME:
PlatformOutput.ime = Some(egui::output::IMEOutput { purpose: egui::IMEPurpose::Password,
rect: global field rect, cursor_rect: global thin cursor rect,
should_interrupt_composition: false }); follow installed TextEdit geometry transform.
This contains geometry only, never secret text. Consume/wipe both event collections.
Buffer capacity1024 preallocated; do not clone secrets or use TextEdit undo history.
AccessKit output field is unconditional in installed egui; do not cfg it away.
Match fixed safe messages including Backup exported, Wallet locked, Wallet unavailable.
App logic must work while hidden and honor quit requested before context exists.
Tests use App::ui without eframe Frame construction; honor both logic/ui routes.
Finish source only with exact changed hashes/counts; no tests/formatters/checks or Git.
