# BBD-WAL-004 Correction 1 Test Source Review

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `2531ab701f3c4420458025956fda1e678e3d0a69`

Result: **ACCEPTED FOR EXPECTED-RED EXECUTION**

No Rust, Cargo, Node, npm, formatter, build, scanner, SBOM generation, native window,
wallet, node, device, staging, or production commit ran during this review.

## Scope and integrity

Sol edited exactly the five test paths authorized by
`CODEX_SOL_BBD_WAL_004_CORRECTION_1_TESTS.md`. The reported 3,381 total lines and every
SHA-256 independently matched:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/vault_store.rs` | 483 | `5774432aef4173a2a1d64bf2dc2b2d9272b93df310f57aee28d7170b953082b9` |
| `wallet-broker/tests/vault_session.rs` | 224 | `9161e738ae33771a347c782582c2875090295f58ef8c02bc233940e4d9368209` |
| `wallet-broker/tests/native_surface.rs` | 376 | `2936ec15e13b7ecabad9c7340a269c741ea964e06c1df3649de9c7d7cbcb41ee` |
| `wallet-broker/tests/secret_hygiene.rs` | 260 | `804c66c4cdec073990e4c4996acd993b6542183111939ca3a95f4797b03a50f0` |
| `test/securityPolicy.node.js` | 2,038 | `dd2e5eef306037dffd846f0d9d239ca0493fd78e01c2ddee0f70816b8488cb84` |

All 15 paths in rejected production drop 01 independently matched their frozen hashes.
No production, manifest, lockfile, fixture, workflow, validator, package, deny, or
unlisted tracked path moved. `git diff --check` passed.

## Review result

The Rust regressions directly exercise the reviewed failure surfaces through public
ports and observable state: exact-deadline timeout and wipe, invalid session/native
account identifiers, empty and oversized controller passphrases, closed diagnostic
operation/code/account fields, and direct Linux port no-follow behavior with target
bytes and mode preserved. Scratch state remains explicit under `target/wal004-scratch`,
with no race, sleep, thread, process, recursion, `/tmp`, or nondeterministic input.

The Node regressions close workflow path filters, the seven-file Rust source inventory,
reviewed `secrecy`/`base64ct` and non-lossy path primitives, exact cargo-deny policy,
and the broker-rooted direct-component CycloneDX graph. Positive controls and one
mutation per required failure class prevent purely vacuous policy assertions.

The test changes preserve all prior coverage and are structurally compatible with the
frozen public interfaces. They are accepted only as expected-red source. They do not
accept or authorize a production correction. Luna must now execute the exact offline
red commands in `CODEX_LUNA_BBD_WAL_004_CORRECTION_1_RED.md`, record the observed named
failures, and commit only the five accepted test paths plus its bounded evidence/state
files. The 15-path flawed production drop remains uncommitted and byte-identical.
