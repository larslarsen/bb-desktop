# Codex Sol Handoff — BBD-WAL-004 CI Gate Regression Tests

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-CI-GATE-REVIEW-01.md`, and the complete
`test/securityPolicy.node.js`.

Your sole task is to edit exactly:

- `test/securityPolicy.node.js`

Use `apply_patch`. Author test source only. Do not edit production policy, workflows,
the ignore file, the ticket, evidence, handoffs, or any other path.

## Exact Gitleaks regression contract

1. Insert this exact historical fingerprint into
   `GITLEAKS_RATCHET_FINGERPRINTS` in lexical order:

   ```text
   12a493196bb4304750e4ae44484a7fa604b82ce4:tickets/BBD-WAL-004.md:generic-api-key:110
   ```

2. Set the expected rationale to exactly:

   ```text
   eight inherited 2016–2018 upstream fingerprints plus one reviewer-published WAL-004 synthetic-vector assignment fingerprint; current-tree copies are removed, never ignored
   ```

3. Set the expected removal condition to exactly:

   ```text
   remove an exact fingerprint only when an authorized history rewrite makes its commit unreachable; current-tree triggers remain relabeled, never ignored
   ```

4. Rename the existing strict ratchet test so its name says `strict nine-line reviewed
   Gitleaks ratchet bytes and content are enforced`, assert length 9, and make the
   missing-fingerprint mutation contain eight entries. Preserve every existing malformed,
   duplicate, global/current-tree, wrong-field, ordering, comment, blank, wildcard,
   secret-bearing, BOM, CRLF, and trailing-byte rejection.

5. In that same test, read `tickets/BBD-WAL-004.md` and assert it contains this exact
   live non-triggering vector line:

   ```text
   expand = 142e48008e3e99568fbbdb4c4534bc67f9666fe4853e6b57c1517be00b24f320
   ```

   Also assert the ticket does not contain the same hex value with the old exact
   `key    =` label. Do not weaken or remove any independent-vector assertion.

## Exact Rust SBOM regression contract

1. Change only the test-side `WAL004_SBOM` expectation to:

   ```text
   cargo +1.98.0 cyclonedx --manifest-path wallet-broker/Cargo.toml --format json --all-features
   ```

2. In the existing manual Rust SBOM test, replace the ineffective
   `--all-features=false` mutation with a mutation that removes the exact final
   ` --all-features` suffix. It must require `checkSbomWorkflow` to reject the
   default-feature-only command. Preserve the exact cargo-cyclonedx version pin,
   validators, direct component expectations, and both artifact uploads.

Do not execute Node, npm, Rust, Cargo, tests, builds, formatters, scanners, Git, GitHub,
network, installs, child processes, Electron, wallets, nodes, hardware, or devices. You
may perform read-only inspection and final `wc -l`/`sha256sum` reporting over only the
authorized test path. Do not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or
unresolved destructive targets.

Stop and report the exact assertions/mutations added, final line count and SHA-256,
confirmation that only the authorized path changed, and confirmation that no prohibited
command ran. Codex XHigh will inspect the test source before Luna receives a separate
expected-red handoff.
