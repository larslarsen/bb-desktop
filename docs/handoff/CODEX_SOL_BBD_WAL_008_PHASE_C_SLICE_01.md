# Codex Sol Handoff — BBD-WAL-008 Phase-C Slice 01

Status: AUTHORIZED — CORE CAPABILITY/ROUTE SOURCE ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected source baseline: `2eb897b0f6b995e119f1a6fa8acbabf3dc2f6332`

Grok Build remains the default senior actor, but the owner reports its weekly usage is
exhausted. This is the documented Sol fill-in condition.

Read completely before editing: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Phase-A Expected-Red
Acceptance 01, the complete `zec_hardware` test, and all three authorized source paths.

## Exact source boundary

Sol may edit only:

| Path | Starting identity |
| --- | --- |
| `wallet-broker/src/zec.rs` | 252 lines; SHA-256 `1061adff987aefd8a641dfed11e06c85d0bc56ddb39f17a5c95d495d6aea387b` |
| `wallet-broker/src/zec/hardware.rs` | must not exist |
| `wallet-broker/src/zec/test_support.rs` | 1,830 lines; SHA-256 `5a1c30199874ad13eb753deaf83c3080534ba495fdfaf225a3d8d03c6ef1ac77` |

Every test, manifest, lockfile, store module, dependency, fixture, document, policy,
workflow, Node/Electron path, repository, and other file is read-only.

## Slice-01 contract

Implement the production-internal typed capability boundary and its test adapter:

- a private `zec::hardware` module containing validated vendor/fingerprint, reviewed
  profile, live-probe, capability-set, decision/status/privacy, verified-field, signing
  pool, and route-metadata types;
- an immutable production reviewed-profile table with zero positive entries and no
  loader or override;
- fingerprint components accepted only at 1–64 ASCII bytes from exact alphabet
  `A-Za-z0-9._+-`, with exact case-sensitive equality;
- a domain-separated SHA-256 fingerprint digest over the typed vendor and
  unambiguously length-prefixed components, emitted only as 64 lowercase hex bytes;
- exact reviewed/live boolean, pool, and verified-field intersection—never union or
  live expansion—and exact equality for branch `37a5165b`, transaction version `6`,
  and PCZT encoding version `2`;
- fixed decision precedence for malformed data, absence, unknown fingerprints,
  protocol mismatch, transparent-only capability, and the complete ready route;
- metadata-only `keystone_pczt_v2` selection for the complete synthetic test profile,
  with no PCZT input/output/mutation or proof/sign/finalize/extract/broadcast/fallback;
  and
- redacted `Debug`/display/errors containing stable codes and bounded capability names,
  never raw fingerprint components, probe contents, labels, identifiers, or artifacts.

`hardware.rs` must contain no synthetic fixture strings or positive real-device pin.
All synthetic Keystone/Trezor/Ledger profiles and probes live only in
`zec::test_support`. The Trezor negative may retain transparent signing but is not
private or Pay-eligible. The unverified Ledger cannot gain spend, Ironwood, on-device
verification, or a route from live claims. Disconnection clears device-derived spend,
route, and verified-field authority and never selects another signer; it does not make
claims about separately held viewing material.

Expose through `zec::test_support` every test-referenced name and signature so the full
test target can compile. Slice 01 owns the decision behaviors through the Ledger
negative plus fingerprint grammar and metadata-only counters. Persistence, reopen,
fault injection, and durable canary behavior remain Slice 02: their test-support
interfaces may be inert, explicit fail-closed scaffolding only. Do not touch the
filesystem or SQLite, publish a ready persisted decision, or simulate successful
persistence in this slice. `HardwareStateRoot::fresh` must be an opaque in-memory test
token only.

Do not use `unsafe`; add dependencies/features; expose this as a shipping UI/API;
enumerate a device; access HID/USB/serial/camera/QR/network/environment/files; parse,
serialize, accept, or return a PCZT; or alter existing Zcash behavior.

## Stop boundary

Do not run Git, GitHub, Cargo, Rust, a formatter, compiler, test, Clippy, build, Node,
npm, policy/security tool, product binary, network, or another actor. Stop after source
editing and report only the three path line counts and SHA-256 identities, confirmation
that no other path changed, and any compile-complete persistence scaffolding left
deliberately fail-closed. Reviewer inspection is required before any execution.
