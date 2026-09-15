# BBD-WAL-019 phase — Sync expected red 01

Date: 2026-09-15. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and session

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 110baa09 · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Session identifier: not recoverable from Hermes version output (provider session).

## Baseline

HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` (verified before/after).
Branch: master.
Filesystem: ext2/ext3 (disk-backed; not tmpfs/ramfs).
Capture directory: `wallet-broker/target/wal019-sync-expected-red-01/`.

## Frozen input verification

All seven pins matched preflight and postflight. Unchanged during capture.

| Path | Lines | SHA-256 | Match |
|---|---|---|---|
| wallet-broker/src/account_ui.rs | 1281 | cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e | yes |
| wallet-broker/tests/account_native_ui.rs | 2654 | e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446 | yes |
| wallet-broker/Cargo.toml | 126 | 435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b | yes |
| wallet-broker/Cargo.lock | 5771 | a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a | yes |
| wallet-broker/src/accounts.rs | 1006 | 8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d | yes |
| wallet-broker/src/zec/live.rs | 1238 | 17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f | yes |
| wallet-broker/src/zec/live_transport.rs | 214 | a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e | yes |

## Commands and environment

PATH included the Cargo bin directory on the build host so `rustup`/`cargo`/`rustc` resolve.

```
timeout 30s hermes --version
timeout 30s rustup run 1.98.0 rustc --version
timeout 60s rustup run 1.98.0 rustfmt --check --edition 2024 wallet-broker/tests/account_native_ui.rs
timeout --signal=TERM --kill-after=10s 600s rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --test account_native_ui wal019_ -- --nocapture
```

## Stage results

### 00-hermes-version
Exit: 0. Output: `Hermes Agent v0.18.2 (2026.7.7.2)`

### 01-rust-version
Exit: 0. Output: `rustc 1.98.0 (88d9e12ae 2026-08-18)`

### 02-format (rustfmt --check)
Exit: 0. No output (formatter clean). Formatter failure is not expected red.

### 03-focused-red
Exit: 101. Compiled in 0.23s, selected exactly 6 tests, 16 filtered out.

```
running 6 tests
test wal019_one_job_across_frames_and_balance_navigation_starts_zero ... FAILED
test wal019_running_endpoint_stays_frozen_until_next_explicit_sync ... FAILED
test wal019_start_error_is_safe_and_retry_is_explicit ... FAILED
test wal019_missing_locked_failed_and_stale_selection_do_not_start ... FAILED
test wal019_list_server_disclosure_and_one_click_starts_at_both_sizes ... FAILED
test wal019_endpoint_survives_back_and_balance_reentry ... FAILED

test result: FAILED. 0 passed; 6 failed; 0 ignored; 16 filtered out
```

## Selected test names and failure cause

All six failed at the same assertion: `tests/account_native_ui.rs:319:43`,
panicked with `exact label must be painted`. This is the missing list Server/Sync/Balance
UI control that the WAL-019 test suite requires but current production does not render.

| Test name | Failure location | Cause |
|---|---|---|
| wal019_one_job_across_frames_and_balance_navigation_starts_zero | line 319:43 | exact label must be painted |
| wal019_running_endpoint_stays_frozen_until_next_explicit_sync | line 319:43 | exact label must be painted |
| wal019_start_error_is_safe_and_retry_is_explicit | line 319:43 | exact label must be painted |
| wal019_missing_locked_failed_and_stale_selection_do_not_start | line 319:43 | exact label must be painted |
| wal019_list_server_disclosure_and_one_click_starts_at_both_sizes | line 319:43 | exact label must be painted |
| wal019_endpoint_survives_back_and_balance_reentry | line 319:43 | exact label must be painted |

Selected: 6. Passed: 0. Failed: 6. Ignored: 0. Filtered out: 16. Finished in 0.05s.

## Outcome

EXPECTED RED CAPTURED.

This is the intended red: the test looks for a Server/Sync/Balance label/button on the account list, before any sync start, that does not exist yet in production. All six tests fail on the same missing-widget assertion, not on compilation errors, infrastructure errors, or timeouts. The sixteen non-wal019 UI tests were correctly filtered out. The production edit that adds the list Server/Sync/Balance controls is the authorized next step, not taken here.

Raw captures (relative to repository root, disk-backed under wallet-broker/target):

| Artifact | SHA-256 |
|---|---|
| preflight.json | `4d76258aac72e25ab296a4244f5f637be4c8e7f105e8c6a30ae9998a74d1253f` |
| postflight.json | `b71c3063f3570ad2331c9be0067d480c28dbc31fcff2543a4335e07346f9ac26` |
| 00-hermes-version.log | `7171f1418b425b98c7040b6e838fab15ef17a048bda0b419b5638867ba5768bf` |
| 00-hermes-version.json | `e3974be4ca2c7a7cf90e54e9b9264e6753bb59dc0f5ac28816564f8416c0050b` |
| 01-rust-version.log | `785f1364c0d5bf077f7bfb885fd4bde99899dfd80ce0fb6226a4527d9b27724c` |
| 01-rust-version.json | `90fe8f95f16d6d30f4f263cd859f40192ddf7b1cdf1e1cb590bcbe8b49e619f8` |
| 02-format.log | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| 02-format.json | `a340b555e934e6a4fcd86575c514da2c6358bc4460c94f0926e94136eccdd674` |
| 03-focused-red.log | `cb5061ad15c1e8d4067f6fe611b4f7760054145d1252124ef7efbe55f7420018` |
| 03-focused-red.json | `bcef23c2a6c3181c14fbbcc222695c88685c5600df0c8f34258354cae2cea2b3` |

No production authorization follows automatically from red. Production became authorized
only by the later reviewer decision in
[BBD-WAL-019-EXPECTED-RED-REVIEW-01.md](../testing/BBD-WAL-019-EXPECTED-RED-REVIEW-01.md).
Report returned to owner for Codex review.
