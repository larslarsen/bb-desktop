# BBD-WAL-008 Phase-A Test-Source Review 03

Reviewer: Lead Engineer/Reviewer — Codex at High

Protected governance parent: `df134530`

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/tests/zec_hardware.rs` | 752 | `5759d612f70a5d21e2b9c7fb192449cf51633e3bff65f2ad7141feaf21812056` |

Result: **PHASE-A TEST SOURCE ACCEPTED**

The manifest adds only the explicit `zec_hardware` test target. The 17-test source is
non-vacuous and covers the ticket's complete fake-device boundary: an empty positive
production table; exact synthetic Keystone PCZT-v2 route metadata; reviewed/live
intersection; exact branch/transaction/PCZT pins; disconnect/Trezor/Ledger negatives;
durable narrowing and explicit fresh restoration; write, file-sync, directory-sync,
commit, corrupt, rollback, and drift failures; exact fingerprint grammar/bounds;
verified-field host-trust disclosure; redaction; and production inventory.

The canary oracle now proves every distinct sensitive slot is installed with zero
touches, exercises the decision/persistence/error/panic paths, proves positive touches,
and scans output plus raw persisted bytes for absence. No real-device positive pin,
transport, PCZT mutation, signing, proof, finalization, extraction, broadcast, fallback,
dependency, feature, lockfile, or production change is present.

Sol again ran a prohibited read-only `git status`/`git rev-parse` baseline command.
There was no Git mutation, formatter, test, build, dependency, product, or network
execution, and the exact accepted source scope is intact. The process deviation is
recorded and is not evidence. Only the separately committed Hermes handoff may execute
and integrate the expected red.
