# BBD-WAL-008 Final Security Gate 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Implementation repo: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: `c3997ab63e109d2e6536ffe4b411b6a28e03a8b1`

Result: **FINAL SECURITY GATE 01 PASSED — AWAITING REVIEWER ACCEPTANCE**

## Preflight

- Hermes: Agent v0.18.2 (2026.7.7.2) · upstream 63279301 · local 10b6d1a9
- Provider/model: nous/meituan/longcat-2.0:free
- Branch: `master`
- HEAD: `c3997ab63e109d2e6536ffe4b411b6a28e03a8b1`
- `HEAD == origin/master`: confirmed
- Index/worktree: clean
- `git diff --check`: clean
- Work directories: ext4 (`ext2/ext3`)

## Immutable inputs

| Path | SHA-256 | Confirmed |
| --- | --- | --- |
| `package-lock.json` | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` | yes |
| `wallet-broker/Cargo.lock` | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` | yes |
| `.gitleaksignore` | `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b` | yes |
| `deny.toml` | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` | yes |

## Accepted stopped-parent results (commands 1–2, not rerun)

Per `BBD-WAL-008-FINAL-SECURITY-GATE-01-STOP-REVIEW-01.md`, the parent session
`20260903_154619_c12b1d` ran and stopped after command 3's environmental
`cargo` resolution failure. Commands 1–2 are accepted by incorporation:

1. `npm audit --audit-level=low` — exit 0, zero vulnerabilities
2. `/home/lars/.cargo/bin/rustup run 1.98.0 cargo audit --file wallet-broker/Cargo.lock`
   — exit 0, no vulnerability denial, only the accepted `atomic-polyfill`
   RUSTSEC-2023-0089 unmaintained warning

## Resume-01 commands

Tool identities:

- cargo-deny `0.20.2` through `/home/lars/.cargo/bin/rustup run 1.98.0`
- Gitleaks `8.30.1` at `target/security-tools/gitleaks-v8.30.1/gitleaks`

Command 3 of the gate (Resume-01 command 1):

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources
```

- Exit: 0
- Outcome: `advisories ok, bans ok, licenses ok, sources ok`
- Only the accepted duplicate-crate warnings (bitflags, block-buffer, block2,
  windows-sys, windows-targets, windows_*_*, …) were emitted; no new
  exception, ignore, or bypass was introduced. `deny.toml` and `.gitleaksignore`
  are byte-identical to the frozen hashes above.

Resume-01 command 2 (git history scan):

```text
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
```

- Exit: 0
- Outcome: `5010 commits scanned`, `~29894708 bytes (29.89 MB) in 2.48s`,
  `no leaks found`

Resume-01 command 3 (working-tree directory scan):

```text
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

- Exit: 0
- Outcome: `~1569141312 bytes (1.57 GB) in 11.5s`, `no leaks found`

## Scan scope

cargo-deny evaluated the full `wallet-broker` dependency graph with
`--all-features` against the exact locked `wallet-broker/Cargo.lock`. Gitleaks
scanned the complete Git history (`git`) and the full checked-out working tree
(`dir`) from the repository root with redaction at 100 characters and no banner.
No path, crate, registry, or source was excluded beyond the existing pinned
`.gitleaksignore` entries, which are unchanged.

## Known separate GitHub/WAL-007 boundary

Per `BBD-WAL-008-SLICE-02-ACCEPTANCE-01.md`, GitHub run `33813477614` for
implementation commit `369d811c` independently passed `npm run build`, all
maintained Node suites, full no-default Rust tests, and Rust formatting. Its
repository-wide all-targets/all-features Clippy failure is confined to the
parked WAL-007/XMR boundary (`xmr_local_gate` absent Phase-D API contract and
the XMR-only `chunks_exact_to_as_chunks` lint in `tests/xmr_rpc.rs`) and grants
no Monero edit authority. Source, tests, broader execution, real-device work,
and WAL-007 remain frozen.

## Transcript deviations

Resume-01 session `20260903_162048_093801` first attempted three nonexistent shorthand
paths: `docs/testing/FINAL-SECURITY-GATE-01.md`,
`docs/testing/STOP-REVIEW-01.md`, and
`docs/testing/SLICE-02-ACCEPTANCE-01.md`. It then used repeated bounded searches plus
`ls -la /home/lars/OpenBazaar/bb-desktop/docs/handoff/ | grep -i wal-008` and a full
handoff-directory listing to locate the exact records. It also ran the unrequested
read-only command
`git show 6503959d08332802f90f8832b5af2652035f46ed --stat | head -30`. The session did
not issue separate reads for the two named role/routing policy documents; `AGENTS.md`
was already injected as project context.

After the three exact gate commands passed, tool identities were captured in one
compound shell submission. The authorized evidence/current-task integration used one
compound stage/commit/push submission, and the final Git proof added the unrequested
`git log --oneline -1`. These operations were read-only or authorized documentation
integration. They did not mutate source or immutable gate inputs, and no security gate
was rerun.

## Repository state

- `git status --porcelain`: clean
- `git diff --check`: clean
- Four immutable hashes: confirmed unchanged
- Source, tests, policy, manifests, locks, workflows, deny/ignore rules, ticket,
  and review documents: untouched

Awaiting reviewer acceptance.
