# Grok Build Handoff — BBD-SEC-001 Correction 1

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This is the complete durable
correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `55ce6b05`

The worktree intentionally contains the uncommitted BBD-SEC-001 source drop. Preserve
all existing changes. Read completely: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-SEC-001.md`,
`docs/handoff/GROK_BUILD_BBD_SEC_001.md`, `docs/handoff/CURRENT_TASK.md`, and the four
authorized correction paths below.

## Reviewer rejection

The source drop used `gitleaks/gitleaks-action@v3` and described it as a complete-history
scan because checkout used `fetch-depth: 0`. The action actually supplies event-specific
first-parent ranges on pull requests and pushes; it does not run the ticket's exact
complete-history command and can miss commits reachable only through a merge's second
parent. Its default job summary also conflicts with the no-finding-body rule. This is a
fail-open design defect, not an execution result.

Reviewer also found that routine workflow path filters include the Linux and macOS build
scripts but omit `scripts/build-windows.ps1`.

## Authorized paths only

Test source first:

- `test/securityPolicy.node.js`

Production/policy after tests:

- `scripts/security-policy.js`
- `.github/workflows/security.yml`
- `.github/workflows/social.yml`

## Required correction

Remove Gitleaks Action use and every related token/comment/upload/version environment
contract. Author tests first that require and falsify all of the following, then update
the checker/workflows:

- checkout retains `fetch-depth: 0`;
- the official archive URL is exactly
  `https://github.com/gitleaks/gitleaks/releases/download/v8.30.1/gitleaks_8.30.1_linux_x64.tar.gz`;
- the downloaded 8,230,402-byte archive is verified against exact SHA-256
  `551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb`;
- extraction writes only `gitleaks` beneath GitHub `${RUNNER_TEMP}` and performs no
  cleanup/deletion;
- the immediately following scan from the repository root is exactly
  `gitleaks git --redact=100 --no-banner .`;
- any range/log opts, baseline, config, ignore, report path, suppression, altered exit,
  wrong URL/hash/version/size, mutable release name, Gitleaks Action use, token, comment,
  artifact, or summary behavior is rejected; and
- `social.yml` push and pull-request filters include `scripts/build-windows.ps1` alongside
  the two existing native build scripts.

Preserve every other delivered test, runtime, CSP, package, workflow, pin, SBOM, scope,
and safety invariant. Do not edit the original two Electron files, package, SBOM workflow,
validator, governance/evidence, or any other path.

Do not run tests, validators, npm/npx, scanners, formatters, builds, Git, GitHub, or install
commands. Do not use local `/tmp`, clean, delete, or inspect secret values. Final
read-only hashes and line counts of the four authorized paths are allowed.

Report test-first order; every new mutation/rejection; exact install/scan steps; final
hashes/line counts; ambiguities; and confirmation of no out-of-scope action. Codex Luna
owns all execution.

## Delivered correction report — 2026-08-29

Grok expanded `test/securityPolicy.node.js` first from 34 to 46 tests, then changed only
the three authorized production/policy files. It did not run tests, validators, npm/npx,
scanners, formatters, builds, installs, Git, or GitHub operations.

Delivered hashes and line counts:

- `test/securityPolicy.node.js`: 818 lines,
  `d680126b62fcf2e7778d74a364e88c3b7fe411663f05a4f1abc2ff40dfca984a`
- `scripts/security-policy.js`: 1,363 lines,
  `cce184efe8b4056217057a4543537dcbe3adb052b1b8c8ce3b07b5b05132804b`
- `.github/workflows/security.yml`: 46 lines,
  `629154349962bf2692bee868c9bb753e0f731a950920e676183e7abdf79eb71e`
- `.github/workflows/social.yml`: 132 lines,
  `24bfa9eed1fd86f811dc20b7b82323a61cb73f0052d928a6e5dab8305ca8f1ec`

The corrected security workflow downloads only the official v8.30.1 archive, requires
8,230,402 bytes and SHA-256
`551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb`,
extracts only `gitleaks` beneath `${RUNNER_TEMP}`, performs no cleanup, and immediately
runs `gitleaks git --redact=100 --no-banner .` from the repository root. New mutation
tests reject wrong/mutable URL, hash, size, version, extraction target/member, cleanup,
delayed/altered/ranged scan, Action/token/comment/summary/report behavior, baseline,
config, ignore, and changed exit behavior. Both routine filters now include the Windows
build script.

Reviewer read-only inspection verified every reported hash and accepted the bounded
correction for Luna execution. Luna must verify all original and corrected hashes before
red reconstruction, use the exact disk-backed paths in its handoff, and stop before later
commands or Git on any failure or finding.
