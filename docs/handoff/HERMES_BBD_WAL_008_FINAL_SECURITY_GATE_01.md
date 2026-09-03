# Hermes Handoff — BBD-WAL-008 Final Security Gate 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Slice-02 Acceptance 01,
the corrected Slice-02 Green 01 evidence, `deny.toml`, and `.gitleaksignore`.

This gate closes only the independent security checks that are not blocked by parked
WAL-007. Do not run any Rust/Node functional test, formatter, compiler, Clippy, product,
device, or Monero command.

Preflight records Hermes version/provider/model and proves branch `master`, exact
`HEAD == origin/master` at the protected parent, clean index/worktree, clean
`git diff --check`, disk-backed ext4 scan/Cargo directories, and these immutable inputs:

| Path | SHA-256 |
| --- | --- |
| `package-lock.json` | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` |
| `wallet-broker/Cargo.lock` | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `.gitleaksignore` | `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b` |
| `deny.toml` | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` |

Require the existing executable paths
`/home/lars/.cargo/bin/cargo-audit`, `/home/lars/.cargo/bin/cargo-deny`, and
`target/security-tools/gitleaks-v8.30.1/gitleaks`. Do not install, update, download, or
use network except as inherently required by the exact npm/cargo advisory checks.

Submit each command byte-for-byte, alone, once, in order, without a `cd`, wrapper,
redirection, pipeline, environment prefix, or appended shell text:

```text
npm audit --audit-level=low
/home/lars/.cargo/bin/rustup run 1.98.0 cargo audit --file wallet-broker/Cargo.lock
/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

Require: npm exit 0 with zero vulnerabilities; cargo-audit exit 0 with zero vulnerability
or advisory denial (the existing non-denying `atomic-polyfill` RUSTSEC-2023-0089
unmaintained warning is allowed only if unchanged); cargo-deny advisories/bans/licenses/
sources all pass with no new exception/ignore/bypass; and both Gitleaks scans exit 0
with no leaks. Stop at the first mismatch.

On exact success, confirm all four immutable hashes and a clean repository, create
`docs/testing/BBD-WAL-008-FINAL-SECURITY-GATE-01.md`, and update only the leading active
block of `docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. Record every full
literal command, exit/outcome, tool identity, immutable input, scan scope, and the
known separate GitHub/WAL-007 boundary from Slice-02 Acceptance 01.

Stage exactly those two documentation paths. Commit exactly
`docs: record WAL-008 final security gate`, push `master`, run only the minimum Git
final-state proof, and stop.

Do not edit source, tests, policy, manifests, locks, workflows, deny/ignore rules,
ticket, review documents, or other paths. Do not rerun, repair, use Grok, invoke another
actor, touch WAL-007/Monero, or broaden the gate. On mismatch, stop without evidence,
commit, push, or post-stop command.
