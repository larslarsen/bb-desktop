# Hermes Handoff — BBD-WAL-006 Scan Gate Resume 02

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-006.md`, `docs/handoff/HERMES_BBD_WAL_006_SCAN_GATE_01.md`, Scan Truth
Correction Review 01, Scan Format Correction Review 01, Scan Gate Clippy Review 01, Scan Compile
Correction Review 01, the complete frozen `zec_scan`, `zec_store`, and `zec_address` tests, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task and precedence

Restart Scan Gate 01 from every protected precondition and its first formatter command. Do not
reuse any prior precondition, formatter, Clippy, identity, capture, or command result. This resume
replaces only the protected governance parent and five-source inventory below. Every other
protected identity, absent-path requirement, disk-path requirement, exact command and order,
expected count, stop rule, evidence requirement, staging restriction, commit message, push, and
final-state proof in Scan Gate 01 remains mandatory.

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and worktree
changes consisting only of:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/scan.rs` | 1,399 | `69a89bcd17a3263b8287ac256375cba40f9241b6e3cfda52567c760121ebd80f` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

Total source is 5,265 lines. Re-prove the 12 protected non-source identities, absent
`wallet-broker/src/zec/prepare.rs`, and source-only and whole-worktree `git diff --check`. Record
the actual Hermes version/provider/model, inspect the wallet-broker filesystem type, and use only
the exact ignored disk-backed paths in Scan Gate 01. Do not use `/tmp`.

Run every exact Scan Gate 01 command once and in its original order: Rust 1.98.0 formatter;
locked/offline/no-default library Clippy with `-D warnings`; `zec_scan` with exact 9/0 counts;
`zec_store` with exact 8/0 counts; `zec_address` with exact 8/0 counts; and Node policy with
expected exit 1, exact 68 `ok`, exact 6 `not ok`, the six exact frozen failure names, and the exact
final summary. Stop immediately at the first mismatch. Do not edit source/tests or run another
command after a mismatch.

Only on complete exact success, perform the Scan Gate 01 evidence and integration steps without
change: create `docs/testing/BBD-WAL-006-SCAN-GATE-01.md`, set `CURRENT_TASK.md` to
`PHASE-C SCAN GATE COMPLETE — REVIEW REQUIRED`, stage only the five source paths, evidence, and
current task, commit exactly `feat: add WAL-006 compact block scanning`, push `master`, and prove
the exact clean final state. Never stage ignored capture or target paths. The reviewer alone
accepts the result and authorizes the next slice.
