# Codex Sol Handoff — BBD-WAL-004 CI Gate Production Correction

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-CI-GATE-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-CI-GATE-TEST-SOURCE-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-CI-GATE-EXPECTED-RED.md`,
`docs/testing/BBD-WAL-004-CI-GATE-RED-REVIEW-01.md`, the complete accepted
`test/securityPolicy.node.js`, and the complete production files authorized below.

The reviewer has already changed only the live governance-ticket label from `key    =`
to `expand =`; the synthetic 32-byte vector hex is unchanged. Do not edit the ticket.

Your sole task is to edit exactly:

- `.gitleaksignore`
- `scripts/security-policy.js`
- `.github/workflows/sbom.yml`

Use `apply_patch`. Do not edit tests, ticket, validators, manifests, lockfiles, wallet
source, other workflows, evidence, handoffs, or any other path.

## Exact Gitleaks production correction

1. Insert this exact historical fingerprint as the first line of `.gitleaksignore` so
   the file contains exactly nine newline-terminated, lexically sorted entries:

   ```text
   12a493196bb4304750e4ae44484a7fa604b82ce4:tickets/BBD-WAL-004.md:generic-api-key:110
   ```

   Preserve the eight existing fingerprints byte-for-byte and in their current order.
   Add no comment, blank, current-tree, global, path-wide, rule-wide, wildcard, or
   secret-bearing suppression.

2. In `scripts/security-policy.js`, insert the same fingerprint first in
   `GITLEAKS_RATCHET_FINGERPRINTS` and set the rationale to exactly:

   ```text
   eight inherited 2016–2018 upstream fingerprints plus one reviewer-published WAL-004 synthetic-vector assignment fingerprint; current-tree copies are removed, never ignored
   ```

3. Set the removal condition to exactly:

   ```text
   remove an exact fingerprint only when an authorized history rewrite makes its commit unreachable; current-tree triggers remain relabeled, never ignored
   ```

4. Change the missing-entry error text from `missing an inherited commit fingerprint`
   to `missing a reviewed commit fingerprint`. Change the terminal exactness error to
   `.gitleaksignore must be the exact nine lexically sorted reviewed commit fingerprints`.
   Preserve every parser, exact-byte, duplicate, malformed, wrong-field, ordering,
   BOM/CRLF/trailing, global/current-tree, wildcard, comment, blank, and secret-bearing
   rejection.

## Exact Rust SBOM production correction

In both the `WAL004_SBOM` constant and the Rust generation step in
`.github/workflows/sbom.yml`, change the exact command to:

```text
cargo +1.98.0 cyclonedx --manifest-path wallet-broker/Cargo.toml --format json --all-features
```

Preserve cargo-cyclonedx 0.5.9, the npm SBOM, both validators, both artifact uploads,
retention, toolchain, action pins, and manual-only workflow trigger. Add no package build
or platform matrix.

Do not execute Node, npm, Rust, Cargo, tests, builds, formatters, scanners, Git, GitHub,
network, installs, child processes, Electron, wallets, nodes, hardware, or devices. You
may perform read-only inspection and final `wc -l`/`sha256sum` reporting over only the
three authorized paths. Do not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs,
or unresolved destructive targets.

Stop and report exact changed fields, final line counts and SHA-256 for all three paths,
confirmation that only authorized paths changed, and confirmation that no prohibited
command ran. Codex XHigh will inspect the production drop before Luna receives a
separate green integration handoff.
