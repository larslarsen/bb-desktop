# Grok Build — WAL-019 one-click Sync production 01

Actor: Sr Dev Grok Build 4.6 High, manually relayed by the owner. Read AGENTS.md,
TESTING.md, CURRENT_TASK.md, tickets/BBD-WAL-019.md, test-source review 02 and
docs/testing/BBD-WAL-019-EXPECTED-RED-REVIEW-01.md. The formal red is accepted.
The original Hermes narrative's inaccurate versions/failure description are
superseded by the reviewer record, which was checked against raw captures.

## Baseline and writable scope

HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`. Require all seven working-tree
pins in `docs/handoff/HERMES_BBD_WAL_019_SYNC_EXPECTED_RED_01.md` before editing.
In particular:

- Writable production: `wallet-broker/src/account_ui.rs`, 1281 lines, SHA-256
  `cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e`.
- Frozen test: `wallet-broker/tests/account_native_ui.rs`, 2654 lines, SHA-256
  `e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446`.

Only the production path above may change. Preserve its existing port/manager
implementation and native custody operations; scope is AccountWindow presentation,
window-local endpoint state and start/cancel UI wiring. Do not touch tests, other
production, dependencies, lockfiles, policy files, evidence or governance records.
Preserve the dirty tree's unrelated WAL-009 and other source changes exactly.

## Required behavior

Implement the full fixed contract in the ticket, including:

1. Show the editable server and existing IP disclosure before an explicit list
   Sync action. Keep the current default endpoint. Selection, editing, opening or
   repainting never starts network work.
2. One enabled list Sync click calls the existing port exactly once with the
   selected unlocked account and displayed endpoint, then displays its returned
   job's progress. Keep existing job/account checks and no duplicate start during
   an active job. The running screen may continue hiding its Sync control.
3. Rename the old navigation-only Sync balance action to Balance. It opens the
   existing balance screen without starting anything. Retain its existing gating.
4. Retain the endpoint within the window across Back/re-entry; no disk settings.
   Disable editing during a running job. After cancellation, editing is available
   and only the next explicit start uses the new endpoint.
5. Preserve fail-closed selection/list/lock behavior. Start errors leave no running
   job, show the safe existing message on the visible screen and permit explicit
   retry. No raw errors, automatic retry, auto-sync or provider fallback.
6. Preserve Cancel, Back, close/hide, account invalidation, stale results, locked
   balance masking and inline unlock behavior. Keep receive/copy, create, backup,
   restore and lock controls available. Sync status cannot become a spendable or
   confirmed balance claim before the existing validation allows it.
7. Keep controls and disclosure usable at 520x720 and 360x480. Use compact layout
   or scrolling as needed while honoring the frozen real-pointer regression suite.
   No theme rewrite, payment enablement, live-server call or wallet-data access.

Use the existing port and worker lifecycle; do not introduce a second sync engine
or duplicate background starts. Routine internal helper/layout choices within the
single authorized source path are yours. If a frozen test exposes a contract
contradiction, report it with exact evidence; never weaken or edit that test.

## Authorized focused format/test loop

From bb-desktop, inspect the filesystem type of the existing wallet-broker/target
before builds. Use disk-backed storage and installed Rust 1.98.0. Run these commands;
you may repair your own source/compiler/focused-test failures and repeat them.

```bash
rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/src/account_ui.rs
timeout --signal=TERM --kill-after=10s 600s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui wal019_ -- --nocapture
timeout --signal=TERM --kill-after=10s 900s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui -- --nocapture
```

Expected green: all 6 WAL-019 tests pass; the whole native account UI target has
22 passing tests, zero failures/ignored. The second command is a focused regression
check of the touched UI, not broader wallet acceptance. No full Rust/proving suite,
Clippy sweep, security scan, falsification or release build is authorized here.
Keep polling the same command session when a wait yields; do not kill/restart an
active runner. Timeout or missing dependencies is not a behavioral result.

Return the changed path, line count, SHA-256, concise implementation explanation,
exact commands/exits/test counts and any unresolved limitation. Confirm the six
frozen inputs stayed unchanged. Do not author evidence files or use Git. Stop for
Codex source review; Hermes later owns recorded green, falsification, report
correction and integration. No other actor, mainnet, funds or deployment.
