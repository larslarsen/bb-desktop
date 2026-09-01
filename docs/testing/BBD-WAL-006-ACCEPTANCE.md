# BBD-WAL-006 Acceptance

Ticket: BBD-WAL-006

Decision: **ACCEPTED — COMPLETE**

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Accepted implementation commits:

- `be3b23ec295f144d532ae21df95c15223efeeefa` — unsigned Ironwood PCZT preparation and
  byte-exact database rollback;
- `cdc956f6d67ad3bd0ffcce70b227870fe3b9b3e4` — closed WAL-006 source/feature/authority policy;
- `996444e905c9e61e01cfec8b2b6efd8c39423649` — reviewed CC0-1.0 transitive-license policy and
  final local acceptance evidence.

## Accepted behavior

BBD-WAL-006 now provides the offline Rust Zcash adapter defined by the ticket:

- deterministic Orchard-only Unified Address issuance and viewing-only SQLite state;
- synthetic compact-block scanning with Ironwood/Orchard classification, continuity, rollback,
  one-block reorg, reopen, and corruption fail-closed behavior;
- sanitized, handle-owned unsigned v6 Ironwood PCZT preparation using the official upstream
  proposal/PCZT construction path;
- one authorization-required unsigned Ironwood action plus one IO-finalized signed padding
  action, with no real-spend signature, proof, finalization, extraction, broadcast, or network
  authority;
- outer SQLite transaction rollback proving prepare leaves wallet database bytes unchanged;
- closed source, dependency, feature, license, and negative-capability policy.

This acceptance does not create a user-facing wallet and does not authorize mainnet, signing,
proving, extraction, broadcast, networking, hardware access, or movement of funds.

## Accepted verification

The final local gate is recorded in
`docs/testing/BBD-WAL-006-FINAL-LOCAL-GATE-01.md`:

- formatter, all-targets/all-features Clippy, and native-feature compilation passed;
- all 127 Rust integration tests passed with zero failed or ignored;
- build and every Node suite passed: Electron security 19, repository policy 75, wallet contract
  48, broker protocol 11, supervisor 11, and preload 6;
- npm audit and cargo-audit reported zero vulnerabilities; cargo-audit retained only the existing
  non-denying `atomic-polyfill` RUSTSEC-2023-0089 unmaintained warning;
- cargo-deny advisories, bans, licenses, and sources passed with no exception/ignore/bypass;
- Gitleaks full-history and current-directory scans found no unsuppressed leak.

The five required isolated falsifications are recorded in
`docs/testing/BBD-WAL-006-FALSIFICATION-GATE-01.md`. Each required guard failed under its exact
temporary mutation and each source file was restored byte-for-byte before the next case. No
falsification byte was committed.

The final applicable GitHub Social client workflow run `33542660010` succeeded at
`996444e905c9e61e01cfec8b2b6efd8c39423649`. Build, all Node suites, all Rust tests, formatter,
all-targets/all-features Clippy, and native-feature compilation were green. Package jobs were
correctly skipped because this ticket requires no platform package.

## Final state

The accepted source/policy commit is `996444e905c9e61e01cfec8b2b6efd8c39423649`. The final
acceptance commit is documentation-only. At acceptance, the tracked worktree and index were clean,
`HEAD == origin/master`, and all accepted source/lock identities matched the final local evidence.

BBD-WAL-008 remains responsible for hardware capability, BBD-WAL-009 for signing/verification/
broadcast/recovery, BBD-WAL-011 for packaged native component/SBOM release evidence, and
BBD-WAL-012 for mainnet authorization.
