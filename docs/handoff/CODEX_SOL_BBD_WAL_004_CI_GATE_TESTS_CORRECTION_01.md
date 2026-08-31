# Codex Sol Handoff — BBD-WAL-004 CI Gate Test Fixture Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-004.md`, the prior CI-gate reviews and
evidence, `docs/testing/BBD-WAL-004-CI-GATE-GREEN-RUN-01.md`, and the complete accepted
`test/securityPolicy.node.js`.

Your sole task is to edit exactly:

- `test/securityPolicy.node.js`

Use `apply_patch`. Do not edit the three accepted unstaged production paths or any other
test, source, policy, workflow, ignore, ticket, evidence, handoff, or repository path.

In the strict nine-line Gitleaks ratchet test, correct exactly the three no-op mutations
that still assume array index 0 is the inherited `index.html` fingerprint:

1. `wrongPath[0]` must replace exact `tickets/BBD-WAL-004.md` with exact
   `tickets/BBD-WAL-004.mdx`, then explicitly assert that the mutated line is not equal
   to `GITLEAKS_RATCHET_FINGERPRINTS[0]` before calling `assertRejects`.
2. `wrongLine[0]` must replace the exact final `:110` with `:1`, then explicitly assert
   non-equality before calling `assertRejects`.
3. `wildcard[0]` must replace exact `tickets/BBD-WAL-004.md` with `*`, then explicitly
   assert non-equality before calling `assertRejects`.

Use clear assertion messages stating that each mutation must change its fingerprint.
Preserve the expected rejection regexes and every other constant, test name, assertion,
mutation, vector byte, and file byte outside the smallest formatting required for those
three fixes. Do not change production behavior to accommodate a test no-op.

Do not execute Node, npm, Rust, Cargo, tests, builds, formatters, scanners, Git, GitHub,
network, installs, child processes, Electron, wallets, nodes, hardware, or devices. You
may perform read-only inspection and final `wc -l`/`sha256sum` reporting over only the
authorized test path. Do not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or
unresolved destructive targets.

Stop and report the exact three replacements/non-vacuity assertions, final line count
and SHA-256, confirmation that only the authorized test path changed in addition to the
already-frozen three production paths, and confirmation that no prohibited command ran.
