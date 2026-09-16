# Hermes — BBD-PAY-001 isolated UI runtime 01

SUPERSEDED: owner feedback places requests in Messages. See
[the governing design](../architecture/BBD-PAY-MESSAGES-UX-01.md) and CURRENT_TASK.
Do not execute this standalone-screen handoff. Preserve existing artifacts and runtime
findings for reuse in a later Messages acceptance contract. Instructions below are historical.

Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
`docs/testing/BBD-PAY-001-SOURCE-REVIEW-04.md`.
Record `hermes --version` and actual provider/model. No actor has been launched by Codex.

## Baseline and authority

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index and all nineteen
source/input hashes in `docs/testing/BBD-PAY-001-GROK-CORRECTION-02.md`, except the
accepted final `test/paymentInbox.electron.js` hash is now
`89d82e8ad73da7d08e3f03a1e9d953521a75a4f55803d96f1265f87bd8c8ce54` (580 lines).
Preserve every source/test/dependency/governance input and all existing captures.
Unexpected drift stops execution; do not reset or repair source.

Writable repository report only: `docs/testing/BBD-PAY-001-HERMES-UI-RUNTIME-01.md`.
New ignored runtime/capture/driver directory only: `dist/pay001-hermes-ui01/`.
No source/test authoring, Git mutation, broader suites/scanners, dependency install,
real daemon/wallet or user profile access. This handoff stages test runtime bytes and
executes the already-authored harness; it does not grant engineering acceptance.

## Host preflight

Before substantial copying, inspect the destination filesystem and free space. Require
disk-backed storage and enough space for the existing Electron runtime plus artifacts.
Use a new directory; stop if the target already exists rather than overwrite evidence.
Read actual host uid, NSpid, Seccomp, NoNewPrivs and execution confinement. Do not use
a confined tool context to bypass its restrictions; request normal tool escalation
when required. If unavailable, record that exact blocker and stop without host changes.

Verify `/opt/google/chrome/chrome-sandbox` is a regular non-symlink file, host uid/gid
0/0, mode 4755, size 15232 and SHA-256
`c100b678a8c171ad0733e51b6f18d98d936d38ab945681c41da00f2ee22e7571`.
Verify its parent directories are root-owned, not group/other writable and have no
unexpected symlink resolution. Require existing Electron executable SHA-256
`180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`.
Record facts and exact preflight commands; do not change installed helpers or policy.

## Stage the isolated test runtime

Copy `node_modules/electron/dist/` to the new
`dist/pay001-hermes-ui01/runtime/`, omitting only the top-level `chrome-sandbox` file.
Use a standard filesystem copy that preserves normal file modes; do not hardlink or
symlink the executable back to the original directory (DIR_EXE must be the new runtime).
Do not change the original installation or create a replacement privileged helper.
Reject unexpected special files or links before copying rather than follow arbitrary
targets. A small standard-library staging/capture driver under the ignored directory
is authorized as executor plumbing; do not author product/test code.

Record a manifest of every original/copied relative path, type, size, mode and SHA-256.
Require an exact match for all included entries and exactly one omission:
`chrome-sandbox`. Confirm the copied executable matches the original and the new
runtime has no adjacent chrome-sandbox file or link. All other runtime resources,
including resources/default_app.asar and locales, stay identical.

## Execute once and retain evidence

From bb-desktop, run this exact frozen fixture harness through the copied executable:

```text
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-hermes-ui01/ui-01 xvfb-run -a dist/pay001-hermes-ui01/runtime/electron test/paymentInbox.electron.js
```

No `--disable-setuid-sandbox`, `--no-sandbox`, security-disable flags or empty helper
value. This is a different runtime layout from the previously failed adjacent-helper
attempt, not a repetition of that command. The installed helper must be selected and
its API accepted; keep all harness OS sandbox, webPreferences, network isolation and
DOM assertions unchanged. Use a bounded outer timeout of 120 seconds, terminate only
the owned runner process group on expiry, retain actual exit/signal and captured output.
No repeating an unchanged failed run. No test edits by Hermes.

Capture argv, cwd, actual start/end, process status, bounded command-local environment,
stdout/stderr files and hashes. Capture only CHROME_DEVEL_SANDBOX,
BBD_PAY001_ARTIFACT_DIR and assigned DISPLAY from the environment. Never dump secrets.
Settle capture once; wait for streams; use exclusive new files. Record the copied
runtime manifest and hashes before/after execution and verify source inputs unchanged.

Expected success: exit 0, `sandbox-status.json` proving Seccomp 2/NoNewPrivs 1/additional
renderer NSpid level, `electron-smoke.json`, four real PNGs with dimensions/hashes, and
all existing UI assertions passing. Link the screenshots from the report so the owner
and reviewer can inspect them. A partial screenshot is a failed-run artifact, not a pass.

If helper selection/API/permissions or any UI assertion fails, retain the precise
message and partial artifacts, report the failing phase and stop. Do not chmod/chown,
install helpers, change AppArmor/sysctl, weaken the sandbox or modify a test to pass.
If the system needs an administrative change, document the specific proposed change
for later review; this handoff does not authorize it.

## Return and stop

Save `docs/testing/BBD-PAY-001-HERMES-UI-RUNTIME-01.md` with actual tool identity,
preflight, runtime manifest, all commands/statuses, screenshot/proof links/hashes,
source identity verification and remaining gaps. No builds of unchanged Rust/Go, no
Node suite reruns, no acceptance scanners, no commit/push, no app restart. Owner's
separate manual checklist is supplementary; do not invent observations for them.
Return the report pointer; Codex reads the repository evidence directly.
