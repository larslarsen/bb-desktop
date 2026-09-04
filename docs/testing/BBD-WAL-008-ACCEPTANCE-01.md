# BBD-WAL-008 Acceptance 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Implementation commit: `369d811cfdb7d659eaba13e8b58d1c07c3624c84`

Final-security evidence commit: `f9ab2b2d1f7d16d428fec3369f50f173262ce85e`

Result: **ZCASH HARDWARE CAPABILITY AND DEVICE-TRUST GATE ACCEPTED**

BBD-WAL-008 is accepted. The implementation adds the broker-internal, fail-closed
Zcash hardware capability boundary: exact bounded fingerprints, an empty positive
production table, reviewed/live capability intersection, metadata-only route
selection, persisted narrowing, corrupt/stale-state denial, redacted status, and a
synthetic test-only Keystone-like profile. No real device is positively pinned.

## Accepted verification

- exact stale-expansion falsification failed as intended and was restored;
- `zec_hardware`: 18 passed, 0 failed;
- `zec_prepare`, `zec_store`, and `zec_hygiene`: 11/8/8 passed;
- warning-denied scoped Clippy and native compilation passed;
- wallet contract: 48 passed, 0 failed;
- security policy: 87 passed, 0 failed, with production policy passing;
- npm audit passed with zero vulnerabilities;
- cargo-audit passed with only the accepted `atomic-polyfill` RUSTSEC-2023-0089
  unmaintained warning;
- cargo-deny advisories, bans, licenses, and sources passed; and
- complete-history and working-tree Gitleaks scans passed with no leaks.

The final evidence is 129 lines with SHA-256
`72fc9b5b9fb912d0ab24b58cf631d816e5f77a97c24b03c3ddad16dd5c9ce43f`.
It now records the exact Resume-01 protected parent, Hermes upstream identity,
provider/model, command results, immutable inputs, and transcript deviations.

## Evidence-process record

The first correction attempt stopped cleanly on a reviewer-authored stale
`CURRENT_TASK.md` hash. Correction Resume 01 fixed the three metadata fields but
omitted the required transcript section and used an incomplete final Git proof.
Correction 02 inserted the supplied section exactly and proved the final clean pushed
repository, but changed a historical current-task paragraph instead of the leading
state and did not read all named governance records. This reviewer acceptance restores
that paragraph and corrects the leading governance state. None of these documentation
events reran a gate or changed source or immutable gate inputs.

## CI and acceptance boundary

GitHub run `33813477614` independently passed the npm build, all maintained Node
suites, full no-default Rust tests, and Rust formatting for the implementation. Its
repository-wide all-targets/all-features Clippy failure is confined to the parked
BBD-WAL-007/XMR test boundary and does not invalidate this Zcash-only acceptance. It
does mean the repository-wide release pipeline is not globally green.

Acceptance does not claim real-device enumeration or transport, QR exchange, PCZT
parsing/display/proving/signing/finalization/extraction, payment broadcast or
confirmation, UX, packaging, or mainnet readiness. Those remain follow-on work,
beginning with BBD-WAL-009. BBD-WAL-007/Monero remains parked pending owner testing;
this acceptance grants no Monero edit or execution authority.
