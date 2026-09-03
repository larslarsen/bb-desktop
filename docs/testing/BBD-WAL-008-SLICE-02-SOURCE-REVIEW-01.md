# BBD-WAL-008 Slice-02 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance parent: `ccb1ab3a46f6c67ef10840dfe2da57aa2430c543`

Result: **ACCEPTED FOR HERMES EXECUTION — NOT YET INTEGRATED**

## Reviewed drop

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/hardware.rs` | 925 | `39feb5c6ce943546f1b1d823f35cef405b81a2c3eb0cb8a6687152b93910784d` |
| `wallet-broker/src/zec/store.rs` | 2,865 | `852b32a8d8ff5ff3a243d5cdaa4e00dae17b82a2602b73626ba6b4aeb8565e4e` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |

The drop changes only the three authorized paths and `git diff --check` is clean. No
formatter, compiler, test, Clippy, native, policy, product, device, or network command
was run by the reviewer.

## Decision

The implementation uses the real per-account `wallet.sqlite3` boundary. It extends the
broker-owned exact schema to V2 with one singleton hardware-decision table while
retaining V0/V1 recognition and treating both V1 and V2 as store-bearing everywhere
the prior code required V1 data.

The record is canonical and versioned, has exact ordered fields, serializes all bounded
capability and route inputs, rejects noncanonical values and extra or missing data, and
cross-checks its generation/check values against separately typed table columns. Reopen
revalidates the reviewed fingerprint/table pin and complete ready-route invariant before
publication. The ticketed mutation ports create record-level inconsistencies; this does
not claim detection of a wholesale SQLite-file rollback.

Writes use `synchronous=FULL` and one `IMMEDIATE` transaction. Every injected fault
returns before commit, so table creation or replacement rolls back with the prior bytes
preserved. The explicit file/directory sync checkpoints occur while rollback remains
possible; SQLite's FULL synchronous commit remains the successful transaction's durable
commit boundary.

Persisted authority can narrow. Any non-narrowing replacement requires an exact,
ephemeral decision produced by the current harness; reopen clears that authorization.
The harness now bootstraps an isolated real Zcash account/store, and persistence,
reopen, mutation, and byte inspection all traverse production store code. No transport,
device pin, PCZT handling, signing, fallback, dependency, or broader source authority is
introduced.

Hermes must first prove formatter cleanliness, falsify the stale-expansion guard, restore
the frozen identities, and then run the focused and affected green gates. Any mismatch
stops without integration.
