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

