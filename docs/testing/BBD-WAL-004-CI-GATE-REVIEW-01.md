# BBD-WAL-004 CI Gate Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Reviewed commit: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Result: **TWO BOUNDED TEST-FIRST CORRECTIONS REQUIRED**

The production green and seven isolated falsifications remain accepted. The two manual
non-packaging GitHub gates ran at the reviewed commit and failed for independent,
specific reasons. Neither failure indicates a leaked credential, a Rust advisory, or a
package build problem.

## Security run 33359184973

The Security workflow passed npm audit, both Node security suites, repository policy,
the pinned Rust toolchain, cargo-audit installation and scan, cargo-deny installation
and its all-features advisory/bans/licenses/sources gate, and the pinned Gitleaks
installation. It failed only at the complete-history Gitleaks scan; the current-tree scan
then skipped because the job is fail-fast.

A local pinned Gitleaks 8.30.1 scan with 100-percent redaction identified exactly one new
finding. It is not a credential. It is the reviewer-published synthetic WAL-004 HKDF
output in `tickets/BBD-WAL-004.md`, labeled `key    =` for the independent cryptographic
test vector. The current-tree fingerprint is:

```text
tickets/BBD-WAL-004.md:generic-api-key:110
```

The exact historical fingerprint is:

```text
12a493196bb4304750e4ae44484a7fa604b82ce4:tickets/BBD-WAL-004.md:generic-api-key:110
```

The correction must preserve complete-history scanning. The historical false positive
may be suppressed only by that exact commit/path/rule/line fingerprint. The live ticket
must be relabeled from `key    =` to `expand =` while preserving every vector hex byte.
No current-tree, global, path-wide, rule-wide, wildcard, or secret-bearing ignore is
permitted. The ratchet becomes exactly nine lexically sorted fingerprints: eight
inherited OpenBazaar records and this one reviewer-published synthetic-vector record.
An exact fingerprint is removable only if a separately authorized history rewrite makes
its commit unreachable; live triggers remain repaired or relabeled, never ignored.

## SBOM run 33359223628

The SBOM workflow passed npm installation/audit, the pinned Rust toolchain,
cargo-cyclonedx 0.5.9 installation, and npm CycloneDX generation and validation. The Rust
document was generated, then the repository validator correctly rejected it because the
default-feature graph omitted the optional direct `eframe` and `rfd` components. Both
artifact uploads skipped after validation failed.

The reviewed cargo-cyclonedx release supports Cargo feature selection. The Rust SBOM
command must add `--all-features` so it represents the native broker feature graph that
the all-features dependency, Clippy, and cargo-deny gates already review. The validator
and direct-component allowlist remain unchanged. A regression mutation must remove
`--all-features` and prove the exact workflow policy rejects that omission.

## Test-first routing

Sol may first edit only `test/securityPolicy.node.js` under the accompanying durable
handoff. Luna then integrates that exact test-only drop and records the expected two-test
red result against the current policy, workflow, ignore file, and ticket label. No
production/policy/workflow/ratchet or ticket correction is authorized before that red
evidence is reviewed.

No package build, root access, `/tmp`, cleanup, deletion, scanner reconfiguration, secret
body inspection, wallet action, or unrelated source change is part of this correction.
