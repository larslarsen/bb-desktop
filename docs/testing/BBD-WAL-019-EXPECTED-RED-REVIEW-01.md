# WAL-019 expected-red review 01

Decision: runtime expected red accepted from the retained raw captures. The
executor narrative is accepted only with the corrections below. No rerun needed.
Reviewer: Codex. No acceptance commands were executed by the reviewer.

## Verified result

HEAD remains `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`. All seven frozen inputs
match both capture snapshots and the current working tree. Each of the four command
logs hashes to its recorded JSON metadata; command order, exact argv, exit codes
and timestamps match the authorized driver.

Formatter: exit 0, empty output. Focused Cargo run: exit 101, 6 selected, 0 passed,
6 failed, 0 ignored, 0 measured, 16 filtered out. Cargo reported the test target
ready in 0.23s and test execution in 0.05s; captured process wall time was about
0.415s. This proves successful compilation/execution, not a fresh rebuild claim.
All six tests failed at `tests/account_native_ui.rs:319:43` with
`exact label must be painted`. Source inspection establishes that their first
missing lookup is `Server` on the account list, before sync starts. This is the
intended missing-UI red, not a compiler, dependency or timeout failure. No claim
is made that later assertions about running jobs have executed yet.

## Corrections to the Hermes narrative

The report `BBD-WAL-019-SYNC-EXPECTED-RED-01.md` contains these inaccuracies:

- Actual Hermes output is `Hermes Agent v0.18.2 (2026.7.7.2)` with upstream
  `110baa09`, local `10b6d1a9` (+1 carried commit), not upstream `a7254e2d`.
- Actual Rust output is `rustc 1.98.0 (88d9e12ae 2026-08-18)`, not the report's
  abbreviated version dated 2026-08-04.
- The outcome paragraph incorrectly says the missing label is in the running
  view. The tests fail on the account list before starting a job.
- The report omitted the requested session identifier and raw-file hashes, and
  included a local absolute PATH entry despite the publishable-prose instruction.
  Provider/model `nous` / `meituan/longcat-2.0:free` are executor-reported; version
  output does not independently establish the resolved model or session.

Raw evidence supports the runtime decision despite these reporting defects. This
review supplies the corrected facts and artifact hashes. Before publication,
Hermes must correct the narrative, provide the actual session identifier if
recoverable (otherwise explicitly mark it unavailable), and replace the absolute
PATH detail with a relative/general description. That record-only correction is
reserved for the next Hermes handoff; do not rerun tests or invent session metadata.
The report's suggestion that production was already authorized was premature;
production is authorized only by this reviewer decision and the new Grok handoff.

## Raw artifact identities

Paths below are within `wallet-broker/target/wal019-sync-expected-red-01/`.

| Artifact | SHA-256 |
| --- | --- |
| `preflight.json` | `4d76258aac72e25ab296a4244f5f637be4c8e7f105e8c6a30ae9998a74d1253f` |
| `postflight.json` | `b71c3063f3570ad2331c9be0067d480c28dbc31fcff2543a4335e07346f9ac26` |
| `00-hermes-version.log` | `7171f1418b425b98c7040b6e838fab15ef17a048bda0b419b5638867ba5768bf` |
| `00-hermes-version.json` | `e3974be4ca2c7a7cf90e54e9b9264e6753bb59dc0f5ac28816564f8416c0050b` |
| `01-rust-version.log` | `785f1364c0d5bf077f7bfb885fd4bde99899dfd80ce0fb6226a4527d9b27724c` |
| `01-rust-version.json` | `90fe8f95f16d6d30f4f263cd859f40192ddf7b1cdf1e1cb590bcbe8b49e619f8` |
| `02-format.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `02-format.json` | `a340b555e934e6a4fcd86575c514da2c6358bc4460c94f0926e94136eccdd674` |
| `03-focused-red.log` | `cb5061ad15c1e8d4067f6fe611b4f7760054145d1252124ef7efbe55f7420018` |
| `03-focused-red.json` | `bcef23c2a6c3181c14fbbcc222695c88685c5600df0c8f34258354cae2cea2b3` |

## Next authorization

Grok Build 4.6 High may implement the fixed WAL-019 UI contract in
`wallet-broker/src/account_ui.rs` only, per
`docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_PRODUCTION_01.md`. Test source is frozen.
Grok may format that source and iterate the focused six-test and 22-test native
account UI commands. Hermes final acceptance, falsification and publication require
later review and authorization. Preserve all unrelated work and existing captures.
