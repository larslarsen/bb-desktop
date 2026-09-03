# BBD-WAL-008 — Zcash Hardware Capability Attestation and Device-Trust Gate

Status: PHASE A TEST SOURCE AUTHORIZED — NO EXECUTION OR PRODUCTION SOURCE AUTHORIZED

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

## Acceptance boundary

Completion proves only a fail-closed Zcash hardware capability/route-selection boundary
with fake devices and no positive production pin. It does not prove that a real device
can parse, display, prove, sign, or return an Ironwood v6 PCZT. Real-device support,
PCZT byte exchange/signing/verification, payment execution, UX, packaging, and mainnet
remain BBD-WAL-008 follow-up, BBD-WAL-009/010/011, and BBD-WAL-012 work.
