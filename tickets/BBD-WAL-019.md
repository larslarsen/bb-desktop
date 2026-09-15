# BBD-WAL-019 — One-click native wallet sync

State: runtime and local wallet refresh ACCEPTED; feature published, evidence correction active.
Feature: `f2d9c9a513b3285531352df84ada818fdac0faf6`, verified on origin/master.
Final closeout requires `docs/testing/BBD-WAL-019-PUBLICATION-REVIEW-01.md`
findings to be resolved through
`docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_CORRECTION_01.md`.
Only its record/scan/eight-path corrective publication scope is active. Earlier
publication 01 instructions below are historical. No source, test or build work.
Review: `docs/testing/BBD-WAL-019-GREEN-REVIEW-01.md`.
Handoff: `docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md`.
The green handoff's ten-pin table remains frozen. Accepted production: 1332 lines,
SHA-256 `09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a`.
Accepted test source: 2654 lines, SHA-256
`e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446`.
All source and local artifacts are frozen. The temporary mutation was restored and
its authorization is closed. Hermes may correct the named records and perform the
publication handoff's exact 20-path scan/commit/push. No tests/rebuild/app restart.
Original source/execution permissions below are historical. Runtime and local
resource refresh are accepted; publication awaits verification. Inherited release
blockers remain open, and end-to-end payments are still incomplete.
Owner priority: payments and usable UI. Reviewer: Codex. Source actor: manually
relayed Grok Build 4.6 High. Hermes owns formal acceptance and later integration.

## Problem and outcome

The current account list's `Sync balance` action only opens `Scene::Sync`;
the next `Sync` action calls `AccountUiPort::start_sync`. The owner reported having
to click sync twice. Put the editable server and existing connection disclosure on
the account list before its `Sync` action. One click then starts exactly one job
for the selected unlocked account and opens the existing progress view.

This is the first bounded wallet-UI improvement in the payment workstream. It does
not complete payments or authorize a full visual rewrite. The payment integration
sequence is recorded in `docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md`.

## Frozen behavior contract

1. Account selection, opening the window, repainting and editing the server start
   zero jobs. The list presents `Server`, the current endpoint, the existing IP
   disclosure and `Sync` together. The initial endpoint stays
   `https://zaino.testnet.unsafe.zec.rocks:443`. No network request happens until
   an explicit enabled Sync click. Endpoint validation stays in the existing port.
2. On an unlocked selected account, one Sync pointer click invokes the real
   `AccountWindow` event path and calls `start_sync(selected_id, displayed_endpoint)`
   exactly once. Success opens the existing progress/cancel view, bound to the
   returned job. Neither repaint nor a rapid second click starts a second job.
3. No selection, a locked/missing account or a failed account-list refresh disables
   starting sync. Unlocking does not implicitly start it. Keep the existing lock,
   account/job matching, stale-result rejection, cancellation and privacy behavior.
4. Keep the editable endpoint for this window's lifetime, including Back/re-entry;
   do not add disk settings. A changed endpoint is used only for the next explicit
   start, never by altering an active job. Disable editing while a job is running.
5. Keep an independently labeled `Balance` navigation action to open the existing
   balance/progress screen without starting network activity. It replaces the
   misleading `Sync balance` navigation label. Its initial and lock gating remain
   the current navigation behavior. A Sync action on that screen also starts once.
6. Start failure displays the existing safe `Wallet unavailable` message visibly
   on the current screen, leaves no running job and permits an explicit retry.
   Raw error text never enters the UI; there is no automatic retry or fallback.
7. Keep Cancel, Back, hide/close and account invalidation cancellation semantics.
   Completed balance visibility and inline unlock continue to use the existing
   account/job checks. Unknown, unsynced or failed balances are not displayed as
   a confirmed zero or spendable balance.
8. Controls and disclosure must be reachable without clipping at 520x720 and
   360x480. Layout can use scrolling. Preserve account create/unlock/lock, receive,
   copy, backup and restore controls. No changes to wallet cryptography, sync
   engine, default provider, protocol, dependencies or renderer privileges.

## Baseline and boundaries

Desktop HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`. This working tree has
unpublished WAL-009 and policy work. Verify HEAD and these current working-tree
bytes, not a clean-checkout assumption. Preserve every unrelated diff and draft.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/account_ui.rs` | 1281 | `cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e` |
| `wallet-broker/tests/account_native_ui.rs` | 2326 | `0f609842fc3bd39029522b74057bd65f9d2955ec1539c2490cc7606ee6db0211` |
| `wallet-broker/Cargo.toml` | 126 | `435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b` |
| `wallet-broker/Cargo.lock` | 5771 | `a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a` |
| `wallet-broker/src/accounts.rs` | 1006 | `8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d` |
| `wallet-broker/src/zec/live.rs` | 1238 | `17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f` |
| `wallet-broker/src/zec/live_transport.rs` | 214 | `a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e` |

Active writable test source: `wallet-broker/tests/account_native_ui.rs` only.
Later production candidate: `wallet-broker/src/account_ui.rs` only, not yet writable.
No bb-go edits, account data access, public-server calls, real funds, Git operations,
release build or deployment. Keep the existing test-file lint corrections.

## Test source and execution contract

Use the existing `PersistentUi`, pointer press/release, actual painted widget
observations and `FakePort` call records. Do not call private handlers, create a
parallel UI implementation or assert source strings as proof of click behavior.
New tests use `wal019_` names. Cover:

- Pre-click visible server/disclosure and zero starts; default and custom endpoint
  each reach the port after one click; repeat at both specified viewport sizes.
- One job across extra frames and rapid clicks while running, with visible progress
  and Cancel. Balance navigation alone starts zero jobs.
- Missing selection, locked account and list failure start zero jobs; account
  disappearance cannot submit a stale account. Explicit unlock never auto-starts.
- Start error produces visible safe feedback and zero implicit retries; the next
  explicit click makes one additional attempt with the displayed endpoint.
- Endpoint survives Back and Balance re-entry; cancellation targets the correct
  account/job. Existing stale-job, locked-balance, hide and inline-unlock proofs
  remain intact.

Adapt the existing navigation tests from `Sync balance` to `Balance` and update
WAL-018's obsolete endpoint-reset expectation only as needed for the new retention
contract. Preserve all independent account/receive/secret/cancellation assertions.
Do not weaken an assertion merely to make current production pass.

From bb-desktop, Grok may run these focused offline commands and repair its own
format/compile/test-source errors. Use installed Rust 1.98.0 and the existing
disk-backed `wallet-broker/target` (reviewer observed ext2/ext3); inspect filesystem
type before execution. No dependency fetching or large /tmp builds.

```bash
rustup run 1.98.0 rustfmt --edition 2024 wallet-broker/tests/account_native_ui.rs
timeout --signal=TERM --kill-after=10s 600s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui wal019_ -- --nocapture
```

Expected RED: the new tests compile and fail at the actual missing list controls
or missing single-click dispatch. Compilation, timeout, unavailable toolchain or
dependency failures are not the intended red. Some defensive tests may already pass;
report exact selected/passed/failed counts. Do not touch production to get green.

After test review, Hermes receives a separate frozen-input formal-red handoff.
After that evidence is accepted, the reviewer authorizes the one-path production
drop; Grok may format that path and iterate the same focused command to green.
Final Hermes acceptance runs the focused command above and this whole UI target:

```bash
timeout --signal=TERM --kill-after=10s 900s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui -- --nocapture
rustup run 1.98.0 rustfmt --check --edition 2024 wallet-broker/src/account_ui.rs wallet-broker/tests/account_native_ui.rs
```

Falsification: in a separately authorized temporary production mutation, suppress
only the list's `start_sync` dispatch while retaining its controls/navigation. The
single-click test must fail on zero calls, then pass after exact-byte restoration.
Hermes records actual commands, exits, counts, source hashes and restoration. No
full Zcash proving-suite rerun is required for this UI-only boundary. Existing
release blockers remain open; this ticket never claims a clean release or payment
readiness. Publication/secret-scan scope requires a later exact-path handoff.
