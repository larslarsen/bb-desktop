# Hermes Handoff — BBD-WAL-008 Final Security Gate Resume 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Final Security Gate 01,
its Stop Review 01, Slice-02 Acceptance 01, `deny.toml`, and `.gitleaksignore`.

Commands 1–2 from the stopped parent are accepted and must not be rerun:

- npm audit: exit 0, zero vulnerabilities;
- Rust 1.98 cargo-audit: exit 0, no vulnerability denial, only the accepted
  `atomic-polyfill` RUSTSEC-2023-0089 unmaintained warning.

Preflight records Hermes version/provider/model and proves branch `master`, exact
`HEAD == origin/master` at the protected parent, clean index/worktree, clean
`git diff --check`, disk-backed ext4 work directories, and the same four immutable
hashes frozen in Final Security Gate 01. Stop on any mismatch.

Submit each command byte-for-byte, alone, once, in order, without a `cd`, wrapper,
redirection, pipeline, environment prefix, or appended shell text:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

Require cargo-deny exit 0 with advisories, bans, licenses, and sources all passing and
no new exception/ignore/bypass. Require both Gitleaks scans to exit 0 with no leaks.
Stop at the first mismatch.

On exact success, confirm all four immutable hashes and a clean repository, create
`docs/testing/BBD-WAL-008-FINAL-SECURITY-GATE-01.md`, and update only the leading active
block of `docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. The evidence must
incorporate the two accepted stopped-parent audit commands/results, then record all
three Resume-01 literal commands/results, tool identities, immutable inputs, scan scope,
and the known separate GitHub/WAL-007 boundary from Slice-02 Acceptance 01.

Stage exactly those two documentation paths. Commit exactly
`docs: record WAL-008 final security gate`, push `master`, run only the minimum Git
final-state proof, and stop.

Do not edit source, tests, policy, manifests, locks, workflows, deny/ignore rules,
ticket, review documents, or other paths. Do not repeat passed commands, install or
update a tool, run another gate, repair, use Grok, invoke another actor, touch
WAL-007/Monero, or broaden the scope. On mismatch, stop without evidence, commit, push,
or post-stop command.
