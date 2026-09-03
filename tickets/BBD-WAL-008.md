# BBD-WAL-008 — Zcash Hardware Capability Attestation and Device-Trust Gate

Status: FINAL SECURITY RESULTS VALID — EVIDENCE CORRECTION 01 AUTHORIZED — HERMES ONLY

Reviewer: Lead Engineer/Reviewer — Codex at High

Planned test and production source actor: Principal Dev — Codex Sol
(`gpt-5.6-sol`, High). Grok Build remains the default senior route but its weekly usage
is exhausted; this owner-reported outage is the documented fill-in condition.

Planned integration actor: Jr Dev — Hermes

Source baseline: `48b5a7565478099eb78e9a112cd2f78fa76cac49`

Dependencies: BBD-WAL-002 capability contract and reviewer-accepted BBD-WAL-006 Zcash
adapter (`996444e9`, final evidence `14a68187`). BBD-WAL-007 Phase C is accepted; its
real offline gate is independently parked and does not block this Zcash-only ticket.

## Security invariant

A vendor name, attached device, self-reported feature, or syntactically valid PCZT can
never grant Zcash signing authority. Authority is the intersection of an exact
reviewed source pin and a live probe, is persisted fail-closed, and may only narrow.
Unknown, changed, disconnected, corrupt, stale, transparent-only, pre-v6, non-Ironwood,
or PCZT-v1 devices cannot become private Pay signers and cannot trigger software or
another-device fallback.

## Objective

Add the broker-internal capability and route-selection boundary needed before any real
Zcash hardware transport or signing work:

1. exact typed device fingerprints and a source-pinned capability table;
2. live-probe intersection that can narrow but never expand reviewed authority;
3. atomic persistence of the narrowed decision and fail-closed reopen;
4. explicit Keystone-PCZT-v2 route selection only behind the complete capability set;
5. transparent-only Trezor and unverified Ledger negative representations; and
6. redacted status suitable for the later native confirmation surface.

This ticket does **not** enumerate a real device, open HID/USB/serial/camera, render or
scan QR, export/import PCZT bytes, prove, sign, finalize, extract, broadcast, access a
seed, or claim support for any shipping firmware/app. BBD-WAL-009 owns PCZT signing and
post-sign verification. A later real-device ticket owns positive production pins and
transport evidence.

## Fixed compatibility values

The accepted Zcash adapter prepares transaction version `6` for consensus branch
`37a5165b` and uses `pczt = 0.9.3`. Its v6/Ironwood PCZTs require serialized PCZT
encoding version `2`; PCZT encoding v1 is incompatible even if a device claims v6.

The shipped production table in this ticket has **zero positive real-device entries**.
Only the test harness may inject a positive synthetic profile. Production cannot load
a table from a file, environment, network, renderer, Electron, `bb-go`, device, or
caller.

## Closed model

The production module owns typed values equivalent to:

- vendor: `keystone`, `ledger`, or `trezor` enum, never an authority-bearing string;
- fingerprint: vendor plus bounded model, app name, and app version;
- reviewed profile: exact fingerprint, table revision, transaction version, consensus
  branch, PCZT encoding version, allowed signing pools, and allowed verified fields;
- live probe: presence plus the same capability dimensions; and
- persisted decision: fingerprint digest, table revision, narrowed capability set, and
  decision status.

Fingerprint components are 1–64 bytes from the exact alphabet `A-Za-z0-9._+-`, use
exact byte equality, and reject controls, whitespace, every other separator, NUL,
non-ASCII, truncation, case folding, normalization, prefixes, ranges, wildcards, and
unknown fields. They contain no serial, USB path, account identity, public key,
address, or other stable device identifier.

The only positive test profile is unmistakably synthetic and is injected through
`zec::test_support`; it represents a Keystone-like device with exact branch
`37a5165b`, transaction version `6`, PCZT encoding `2`, Ironwood signing, on-device PCZT
verification, and an explicit verified-field subset. It must never appear in the
production table or a production serialized record.

## Capability reduction and decisions

For every boolean capability and verified field:

```text
narrowed = reviewed AND live
```

Live data cannot introduce an entry, vendor, pool, branch, version, route, or verified
field absent from the reviewed profile. A later probe may further narrow a persisted
decision. It cannot restore a capability without a fresh exact reviewed-table match.

This hardware decision owns device-derived signing and display authority, not the
account's separately stored viewing material. Disconnection always removes every
device spend/route/verified-field capability and forbids fallback, but this module must
not claim that broker-held viewing material was destroyed or decide whether it can
continue to view or derive receivers. Account composition remains outside this ticket.

Decision precedence is fixed:

1. malformed fingerprint/probe/record → `SCHEMA` or `STATE_CORRUPT` before persistence;
2. absent device → `DEVICE_DISCONNECTED` and no route;
3. no exact reviewed profile → `CAPABILITY_MISSING` and all spend flags false;
4. exact profile with branch, tx, or PCZT mismatch → `PROTOCOL_INCOMPATIBLE`;
5. transparent-only capability → `CAPABILITY_MISSING`,
   `privacy=transparent_not_private`, and no private Pay route; and
6. only the complete exact Keystone synthetic intersection may select
   `keystone_pczt_v2` in tests.

Selecting a route returns metadata only. It never returns, accepts, logs, or mutates a
PCZT. `verified_fields` is an allowlisted subset of `amount`, `recipient`, `network`,
`fee`, and `memo`; the persisted/public value is the intersection, and every omitted
field remains explicitly host-trusting. Electron cannot assert device verification.

## Persistence

The narrowed decision is stored in the existing per-account Zcash SQLite boundary in
one transaction. Schema/table revision, fingerprint digest, every capability flag,
decision status, and verified-field set are validated on write and reopen. Unknown
fields, duplicate fields, invalid booleans, out-of-range versions, table-revision drift,
consensus drift, partial writes, rollback, and sync/commit faults fail closed without
publishing a ready decision.

Raw probe replies, fingerprint components, device labels, PCZT bytes, addresses,
account IDs, and transport details are not persisted or emitted. `Debug`, display,
errors, diagnostics, and panics expose only stable decision/error codes and bounded
non-secret capability names.

## Required Phase-A tests

The first drop must define non-vacuous tests for all of these behaviors:

1. the production table contains no positive real-device entry;
2. unknown and one-field-mismatched fingerprints deny every spend/route capability;
3. the injected synthetic Keystone profile selects `keystone_pczt_v2` only for the
   exact complete reviewed/live intersection;
4. live claims never expand reviewed booleans, versions, pools, routes, or fields;
5. branch `37a5165b`, tx `6`, and PCZT encoding `2` use exact equality, with prefix,
   case, range, v1, and one-bit mismatch negatives;
6. disconnected devices never fall back to software or another device;
7. transparent-only Trezor is never private or Pay-eligible;
8. an unverified Ledger never signs Ironwood;
9. persisted narrowed decisions reopen exactly, while corruption, rollback, partial
   write, table revision drift, and consensus drift fail closed;
10. fingerprint size/character boundaries immediately below, at, and above limits;
11. verified fields are intersected and missing fields remain host-trusting;
12. redaction canaries are absent from success, error, `Debug`, panic, and persisted
    representations; and
13. production inventory contains no real-device positive pin, transport, network,
    parser, QR, HID/USB, signing, extraction, broadcast, or software-fallback authority.

Source-text assertions are allowed only for dependency and production-inventory
policy. Behavioral assertions must execute through the typed test harness.

## Phases and authorized paths

### Phase A — test source (authorized now)

Codex Sol may edit only:

- `wallet-broker/Cargo.toml` — add the explicit `zec_hardware` test target only; and
- `wallet-broker/tests/zec_hardware.rs` — new test source for the required groups.

No dependency, feature, build script, production source, fixture, lockfile, Node,
Electron, policy, workflow, documentation, evidence, Git, or other path may change.
Sol may use read-only source inspection and final line/hash reporting. It runs no
formatter, test, build, dependency resolution, product binary, Git, or network and
stops with exact path/line/hash reporting.

### Phase B — expected red (future Hermes handoff)

After reviewer source acceptance, Hermes will run the exact focused test. Expected red
is an absent `zec::hardware`/test-support contract, not a dependency, syntax, fixture,
or unrelated failure. The source actor never executes the red.

### Phase C — bounded production slices (future separate handoffs)

Maximum production inventory:

- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/hardware.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

Slice C1 owns typed fingerprints, the empty production table, fake injected profiles,
intersection, decisions, and route metadata. Slice C2 owns atomic persistence and
reopen/fault behavior. No slice may add real transport, real-device pins, or signing.

### Policy transition — test first (authorized by reviewer decision 15)

The historical WAL-006 six-target and seven-path inventories remain exact. WAL-008 adds
an independently named policy contract for the single `zec_hardware` test target and
the complete eight-path ZEC production inventory, including `zec/hardware.rs`.
Test source is integrated and proved red before `scripts/security-policy.js` may change.
The production checker must eventually scan the new module and preserve all existing
transport, signing, broadcast, network, and mainnet denials.

## Planned gates

The focused green is:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware
```

Affected regression includes `zec_prepare`, `zec_store`, `zec_hygiene`,
`wallet_contract`, warning-denied Clippy, native compilation, and the repository policy
checks. Final acceptance retains the ticketed full Rust, Node, security, audit, scanner,
and GitHub gates; exact commands will be frozen only in Hermes handoffs.

Required falsifications include independently enabling a vendor-name shortcut,
replacing reviewed/live intersection with union, accepting PCZT v1, marking the
transparent Trezor fixture private, and reopening a stale persisted decision. Each must
make its intended test fail and be exactly restored before green execution.

## Reviewer source decision 01

The manifest-only test-target edit is accepted. The initial 614-line test source needs
the bounded correction in
`docs/testing/BBD-WAL-008-PHASE-A-TEST-SOURCE-REVIEW-01.md`: separate spend authority
from viewing state, complete the positive-case forbidden assertions, make redaction
canaries non-vacuous and byte-safe, prove persisted narrowing cannot silently expand,
cover directory-sync failure, and freeze the fingerprint alphabet. No execution or
production source is authorized.

## Reviewer source decision 02

Correction 01 resolves the substantive capability and persistence findings. One
redaction-oracle correction remains: canary touch counts must be zero immediately after
installation and positive only after the exercised sensitive flows. See
`docs/testing/BBD-WAL-008-PHASE-A-TEST-SOURCE-REVIEW-02.md`. No execution or production
source is authorized.

## Reviewer source decision 03

Correction 02 establishes the required zero-to-positive canary observation. The exact
117-line manifest and 752-line, 17-test source identities are accepted in
`docs/testing/BBD-WAL-008-PHASE-A-TEST-SOURCE-REVIEW-03.md`. Hermes alone may run the
separately committed formatter and expected-red handoff. Production and every broader
gate remain unauthorized.

## Reviewer execution decision 04

Hermes's first Phase-B command stopped at the required first mismatch: the exact Rust
1.98 formatter check exited 1 with mechanical layout differences in the new test file.
The expected-red test did not run and no mutation or integration occurred. The exact
unchanged source identities and valid stop are recorded in
`docs/testing/BBD-WAL-008-PHASE-B-FORMAT-STOP-REVIEW-01.md`. Codex Spark High alone may
run the one-file pinned `rustfmt` command in the separately committed correction
handoff. Test semantics, the manifest, production source, Hermes execution, and every
broader gate remain unauthorized pending reviewer inspection.

## Reviewer source decision 05

Spark's exact one-file Rust 1.98 formatter mutation is accepted at High in
`docs/testing/BBD-WAL-008-PHASE-A-FORMAT-CORRECTION-01-SOURCE-REVIEW-01.md`. The
manifest remains exact and the formatted 794-line test retains all 17 tests. Hermes
alone may restart the exact formatter check and expected-red test under Resume 02 and
integrate only on the specified absent-production-contract failure. Production source,
broader gates, real-device work, and WAL-007 execution remain unauthorized.

## Reviewer evidence decision 06

Hermes's exact gate outcomes and four-path integration at `eda78545` are valid, but the
evidence mistypes one digit of the manifest SHA-256. The bounded finding is recorded in
`docs/testing/BBD-WAL-008-PHASE-A-EXPECTED-RED-01-EVIDENCE-REVIEW-01.md`. Hermes alone
may correct the exact evidence field and current-task state. Production source and all
execution remain unauthorized pending reviewer acceptance.

## Reviewer acceptance decision 07

The exact Phase-A test contract and expected-red result are accepted at `eda78545`,
with its evidence hash corrected at `2eb897b0`, in
`docs/testing/BBD-WAL-008-PHASE-A-EXPECTED-RED-ACCEPTANCE-01.md`. Because Grok's weekly
usage remains exhausted, Codex Sol High alone may author the three-path Phase-C Slice
01 core capability/route source. Persistence, hardware I/O, real-device pins, signing,
execution, integration, and broader gates remain unauthorized pending source review.

## Reviewer source decision 08

Sol's exact three-path capability/route source is accepted at High in
`docs/testing/BBD-WAL-008-SLICE-01-SOURCE-REVIEW-01.md`. Hermes alone may run the
formatter, exact intersection falsification/restoration, and 12-test partial green,
then integrate only on exact success. Persistence and its five tests, real-device
work, signing, regressions, broader gates, and WAL-007 remain unauthorized.

## Reviewer execution decision 09

Hermes stopped correctly when the first Slice-01 command, the exact formatter check,
exited 1 with mechanical differences in `zec/hardware.rs` and `zec/test_support.rs`.
No falsification, test, mutation, or integration ran. The unchanged-source stop is
recorded in `docs/testing/BBD-WAL-008-SLICE-01-GREEN-STOP-REVIEW-01.md`. Codex Spark
High alone may run the pinned formatter on those two files. Further execution and all
later work remain unauthorized pending reviewer inspection.

## Reviewer source decision 10

Spark's two-file Rust 1.98 formatter output is accepted in
`docs/testing/BBD-WAL-008-SLICE-01-FORMAT-CORRECTION-01-SOURCE-REVIEW-01.md`; its
unnecessary `cd` command prefix is recorded. Hermes alone may restart the exact
formatter, intersection falsification/restoration, and 12-test partial green under
Resume 02 and integrate only on exact success. Persistence and broader work remain
unauthorized.

## Reviewer execution decision 11

Resume 02 proved the formatter and falsification, then passed 13 non-persistence tests;
the handoff's required count of 12 was a reviewer miscount of the frozen 18-test file.
Hermes nevertheless violated the mandatory stop by repeating the green command. The
source remained exactly restored and unintegrated. The result and accepted
falsification are recorded in
`docs/testing/BBD-WAL-008-SLICE-01-GREEN-RESUME-02-REJECTION-01.md`. Hermes alone may
run Resume 03 with the corrected 13-test count and integrate on exact success.

## Reviewer evidence decision 12

Resume 03 passed and integrated at `d55edcec`, but its evidence abbreviates the gate
commands and falsely claims no wrappers; transcript audit also found an unrequested
Node.js version command. The bounded evidence correction is specified in
`docs/testing/BBD-WAL-008-SLICE-01-GREEN-01-EVIDENCE-REVIEW-01.md`. Hermes alone may
correct the two documentation paths. Source and further execution remain closed.

## Reviewer evidence decision 13

Correction 01 disclosed the deviations but still abbreviates both gate commands and
left stale current-task prose. Evidence Review 02 authorizes Hermes alone to insert the
two complete literal command strings and correct current-task state. Source, execution,
and Slice 02 remain closed.

## Reviewer acceptance decision 14

Slice 01 is accepted at `d55edcec`, with exact evidence corrected through `03d3213f`,
in `docs/testing/BBD-WAL-008-SLICE-01-ACCEPTANCE-01.md`. The accepted file has 18
tests. Because Grok remains usage-exhausted, Codex Sol High alone may author the
three-path Slice-02 durable SQLite decision store. Tests, execution, integration,
transport, signing, real-device work, and broader gates remain unauthorized pending
source review.

## Reviewer execution decision 15

Hermes Resume 03 passed the Slice-02 Rust, Clippy, native, and wallet-contract gates,
then stopped when repository policy exposed an omitted WAL-008 transition. The exact
79/7 result and transcript deviations are recorded in
`docs/testing/BBD-WAL-008-SLICE-02-GREEN-RESUME-03-STOP-REVIEW-01.md`. The three
accepted persistence paths remain exact; this is not a source defect. Because Grok's
weekly usage remains exhausted, Codex Sol High alone may author the one-path policy
test contract in `test/securityPolicy.node.js`. Production policy, execution,
integration, and all other paths remain unauthorized pending reviewer source review.

## Reviewer source decision 16

Sol's one-path policy-test drop is accepted in
`docs/testing/BBD-WAL-008-POLICY-TEST-SOURCE-REVIEW-01.md`. It preserves WAL-006's
historical contracts while independently requiring the WAL-008 target, complete
eight-path inventory, new production exports/checker, repository scan, and authority
mutation negatives. Hermes alone may run the frozen focused Node expected-red command
and integrate the test-only drop on exactly 80 `ok` and the seven specified `not ok`.
Production policy and all other work remain unauthorized pending reviewer acceptance.

## Reviewer acceptance decision 17

The policy test and exact 80/7 expected red are accepted at `9c7ef290` in
`docs/testing/BBD-WAL-008-POLICY-EXPECTED-RED-ACCEPTANCE-01.md`. Transcript audit
confirms the focused command, causes, integration scope, and final repository state.
Because Grok remains usage-exhausted, Codex Sol High alone may implement the matching
one-file production contract in `scripts/security-policy.js`. Tests, execution,
integration, and all other paths remain unauthorized pending source review.

## Reviewer source decision 18

Sol's exact one-file production policy is accepted in
`docs/testing/BBD-WAL-008-POLICY-PRODUCTION-SOURCE-REVIEW-01.md`. It preserves the
historical WAL-006 contracts, adds only the required WAL-008 target/inventory/checker,
and routes `hardware.rs` through the existing source denials. Hermes alone may run the
complete exact Slice-02/policy green handoff and integrate the four source paths only
on exact success. Other source, broader/final work, and WAL-007 remain unauthorized.

## Reviewer evidence decision 19

The required Slice-02/policy sequence passed and the exact six-path integration is
pushed at `369d811c`. Transcript audit then found four prohibited post-integration
commands: two `store.rs` hash checks and two repeated focused-test executions. They did
not mutate source, but the evidence omits them. The bounded finding is recorded in
`docs/testing/BBD-WAL-008-SLICE-02-GREEN-01-EVIDENCE-REVIEW-01.md`. Hermes alone may
correct the green evidence and current-task records without rerunning any gate. Source,
tests, broader execution, and all other paths remain unauthorized.

## Reviewer acceptance decision 20

Slice-02 persistence/policy source, required gates, integration, and corrected evidence
are accepted in `docs/testing/BBD-WAL-008-SLICE-02-ACCEPTANCE-01.md`. GitHub run
`33813477614` independently passed build, all maintained Node suites, full no-default
Rust tests, and formatting; its all-features Clippy failure is restricted to the parked
WAL-007/XMR contract and grants no Monero edit authority. Hermes alone may run the five
remaining independent audit/license/secret-scan commands. All source, tests, broader
execution, real-device work, and WAL-007 remain unauthorized.

## Reviewer execution decision 21

The final security gate passed npm audit and cargo-audit, then stopped before
cargo-deny policy evaluation because the standalone binary could not resolve `cargo`.
The exact environmental stop is reviewed in
`docs/testing/BBD-WAL-008-FINAL-SECURITY-GATE-01-STOP-REVIEW-01.md`. Hermes alone may
resume through the absolute Rust 1.98 cargo route and run the two pending Gitleaks
scans. Passed audits, source, tests, Monero, and all other commands remain frozen.

## Reviewer evidence decision 22

Hermes completed and integrated all five final security results at `404a438e`, but the
evidence records the wrong Resume-01 protected parent, mistypes one digit of the Hermes
upstream hash, and falsely identifies the provider/model. Transcript audit also found
bounded read-only/process deviations. The results remain valid; the exact correction
is governed by
`docs/testing/BBD-WAL-008-FINAL-SECURITY-GATE-01-EVIDENCE-REVIEW-01.md`. Hermes alone
may correct the final-security evidence and current-task records without rerunning any
gate. Source, tests, policy, other documentation, real-device work, and WAL-007/Monero
remain frozen.

## Acceptance boundary

Completion proves only a fail-closed Zcash hardware capability/route-selection boundary
with fake devices and no positive production pin. It does not prove that a real device
can parse, display, prove, sign, or return an Ironwood v6 PCZT. Real-device support,
PCZT byte exchange/signing/verification, payment execution, UX, packaging, and mainnet
remain BBD-WAL-008 follow-up, BBD-WAL-009/010/011, and BBD-WAL-012 work.
