# Codex Sol Handoff — BBD-WAL-008 Phase-C Slice 02

Status: AUTHORIZED — DURABLE DECISION PERSISTENCE SOURCE ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected source baseline: `03d3213f0570b860dc4371645fedfc28016dae6e`

Grok Build remains unavailable because the owner reports its weekly usage exhausted;
this is the documented Sol fill-in condition.

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Slice-01 Acceptance 01, the
complete hardware test, and all three authorized paths.

## Exact source boundary

| Path | Starting identity |
| --- | --- |
| `wallet-broker/src/zec/hardware.rs` | 864 lines; SHA-256 `48233ec9ceea26e4b8ac499c00ee5d8c00ca546f2e036dfd16d1deb59d565285` |
| `wallet-broker/src/zec/store.rs` | 2,105 lines; SHA-256 `0c3f830b8d09c697832689e6fcd1ffb630341d25755090f90a987623a8200feb` |
| `wallet-broker/src/zec/test_support.rs` | 2,385 lines; SHA-256 `7d73c8ce55073198a345a71eadf2ac47589665dc1580dc8f485ab3d0f38a5a55` |

Every test, manifest, lockfile, dependency, other source/document/policy/workflow, Node,
Electron, repository, and path is read-only.

## Persistence boundary

Implement the remaining five tests through the actual existing per-account Zcash
`wallet.sqlite3` boundary in `zec::store`; a successful in-memory persistence model is
forbidden. Test support may bootstrap an isolated fixed synthetic account/root using
the existing disk-backed `target` test-root machinery, then call production store
functions. The hardware record is a singleton within that per-account database and
must not duplicate a raw account ID, fingerprint component, label, path, probe reply,
device identifier, address, transport detail, or artifact.

Use an exact broker-owned schema recognized and validated by the store. If extending
the existing schema version, preserve recognition and behavior of all prior versions
and treat the new version as compatible wherever existing V1 data is required. Reject
unknown/extra/missing schema objects, columns, constraints, or record fields. Do not
hide the table from broker schema validation by using an unowned name.

Persist one canonical strictly decoded record containing schema revision, fingerprint
digest, reviewed-table revision, decision status/privacy, every bounded capability,
exact protocol values, signing pools, verified and host-trusting fields, route, and a
monotonic record generation/check value. Validate before write and again on reopen:
64-byte lowercase digest grammar; known revisions/status/route/privacy; canonical
booleans; exact `37a5165b`/`6`/`2`; allowlisted unique pools/fields in canonical order;
host-trusting complement; no live expansion; and complete ready-route invariants.
Unknown fields, duplicate fields, invalid values, partial/inconsistent generation,
table/consensus drift, and the test's record-level rollback mutation return
`STATE_CORRUPT` and publish no ready decision. Do not claim detection of a wholesale
rollback of the entire SQLite database without an external trusted monotonic anchor;
the ticket's `Rollback` mutation is the paired-record/generation inconsistency tested
here.

Use `PRAGMA synchronous=FULL` and one `IMMEDIATE` SQLite transaction for each successful
write. Write/file-sync/directory-sync/commit fault ports are test-only, one-shot, and
must return `INTERNAL` before publication while preserving the exact prior committed
record bytes. No fault may leave schema or decision partial. A successful reopen
publishes exactly one validated ready decision.

A persisted decision may narrow. It may not expand after reopen merely because a wider
value is supplied. Expansion is allowed only when that exact wider decision was freshly
recomputed by the current harness from an exact reviewed-table/live-probe match; keep
that authorization ephemeral and never serialize it. Failed writes preserve the prior
ready decision and bytes.

`persisted_bytes()` must return the canonical hardware record bytes, not the whole
SQLite file, and must be empty before a record exists. `reopen`, `reopen_in_place`,
`persist`, fault injection, and mutation methods must traverse the production store
implementation. Preserve the nine-slot canary zero-to-positive observation and ensure
record bytes, `Debug`, display, logs, diagnostics, errors, and panic payload remain
redacted.

Do not add dependencies/features; use `unsafe`; access network/environment; add a real
device pin/transport; parse or mutate PCZT; sign/prove/finalize/extract/broadcast; add
fallback; change tests; or alter existing account data semantics beyond the strictly
versioned hardware record.

## Stop boundary

Do not run Git, GitHub, Cargo, Rust, formatter, compiler, test, Clippy, build, Node/npm,
policy/security tools, product/device/network commands, or another actor. Stop after
editing and report the three resulting line counts/SHA-256 identities, exact persistence
schema/version choice, and confirmation that all other paths remained unchanged.
Reviewer source inspection is required before execution.
