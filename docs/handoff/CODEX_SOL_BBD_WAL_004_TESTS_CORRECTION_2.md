# Codex Sol Handoff — BBD-WAL-004 Test Source Correction 2

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Reviewer governance baseline: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-004.md`,
`docs/handoff/CODEX_SOL_BBD_WAL_004_TESTS.md`,
`docs/handoff/CODEX_SOL_BBD_WAL_004_TESTS_CORRECTION_1.md`,
`docs/handoff/CURRENT_TASK.md`, and `wallet-broker/tests/vault_store.rs`.

Correction 1's `wallet-broker/tests/vault_store.rs` has 445 lines and SHA-256
`6a4be900c8cf4128fb8da5c04080fb0dcdc01918ae64ebdbefbd08a4330ead61`. Its behavioral
corrections are accepted, but its reported future API included `VaultStore::port_mut()`.
That production-mutating accessor is rejected because it bypasses the `VaultStore`
invariants that this ticket exists to enforce.

Your sole task is to edit exactly `wallet-broker/tests/vault_store.rs`. Use `apply_patch`.
Replace `FakePort.fail: Option<FaultPoint>` with test-owned shared fault state such as a
small cloneable `Rc<Cell<Option<FaultPoint>>>` wrapper. Keep one clone in
`failure_during_recovery_remains_fail_closed_with_recoverable_staging`, move the port
into the store, and change the clone from `FileSync` to `Replace` before calling
`recover_account`. Remove every use and future reservation of `VaultStore::port_mut()`.
The fake's behavior, every existing test name/assertion, the real `LinuxStorePort` test,
and all other bytes outside the minimal import/fake/fault-construction changes must be
preserved. Do not replace the accessor with any other mutable production escape hatch or
test-only production API.

You may perform read-only inspection and final `wc -l`/`sha256sum` reporting over that
one file. Do not execute Rust, Cargo, Node, npm, tests, builds, formatters, scanners, Git,
GitHub, network, install, child processes, Electron, native windows, wallets, nodes,
hardware, or devices. Do not create or edit any other path. Do not use root, `sudo`,
`/tmp`, deletion, cleanup, `rm`, globs, or variable/substitution-resolved destructive
targets.

Stop and report the final line count/SHA-256, exact changed lines/API effect, why fault
switching remains non-vacuous, confirmation that `port_mut` is absent, and confirmation
that no unlisted path changed and no prohibited command ran. Lead Engineer/Reviewer —
Codex XHigh will re-review before any install, execution, Git integration, or production
source authorization.
