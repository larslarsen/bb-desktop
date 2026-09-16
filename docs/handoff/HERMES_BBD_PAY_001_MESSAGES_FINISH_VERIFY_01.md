# Hermes — Messages finish verification 01

CLOSED: partial results reviewed in [review 06](../testing/BBD-PAY-001-MESSAGES-REVIEW-06.md).
UI failed before screenshots; no unchanged rerun. Current authority is
[Sol bootstrap correction 01](SOL_BBD_PAY_001_MESSAGES_UI_BOOTSTRAP_01.md).
Execution instructions below are historical. Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[source review 05](../testing/BBD-PAY-001-MESSAGES-REVIEW-05.md).
This is the sole execution authority. All older execution handoffs remain closed.
No actor has been launched by the reviewer.

## Baseline, writes and stops

Work in **bb-desktop**, HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Require every source/input hash in review 05. Preserve unrelated dirty work and old
captures. Drift stops execution; do not reset, integrate different bytes or repair source.
The old sixteen-file BBGO-PAY-003 publication set is unrelated and already complete.

Writable report: `docs/testing/BBD-PAY-001-HERMES-FINISH-VERIFY-01.md`.
New ignored artifacts: `dist/pay001-finish-verify01/`. Create exclusively; if it already
exists, inspect and document its contents rather than overwrite or blindly repeat runs.
The frozen Node suite may create/clean its own process-specific fixture trees beneath
`dist/pay001-grok-correction01/trees/`; preserve all prior evidence there.
No production/test edits, dependency installation, Git mutation, actor launches,
real daemon/wallet/user-profile access or user-process restart. The isolated mutation
below is the only authorized test-input change. Executor capture/staging plumbing in
the new ignored directory is allowed; it must not change test assertions or outcomes.

Save actual `hermes --version`, provider/model, Node/npm versions and the source manifest.
For every command retain exact argv, cwd, allowlisted command-local environment, actual
start/end timestamps, exit/signal/timeout, separate stdout/stderr and their SHA-256.
Use unique capture names and wait for streams to close. Do not substitute approximate
prose or a claimed aggregate pass for raw command records. Never dump full environment,
credentials or user files. Capture wrappers must terminate only their owned process group
on deadline, TERM then KILL after five seconds, and record the real result.

An unexpected failure stops later phases. Save the failed report and partial artifacts;
do not retry unchanged, weaken assertions or repair source. Expected falsification red
is the explicit exception. Only the reviewer accepts results or authorizes another task.

## 1. Focused green and integration

Run sequentially from the repository root with separate captures:

```text
node --check social/app.js
node --check test/paymentInbox.node.js
node --check test/paymentInbox.electron.js
timeout --signal=TERM --kill-after=5s 120s node test/paymentInbox.node.js
timeout --signal=TERM --kill-after=5s 120s npm run test:wallet-pay
node test/socialCore.node.js
node test/walletPreload.node.js
node test/electronSecurity.node.js
npm run build
```

Bound other commands to 120 seconds through the capture wrapper as well. Require exit 0,
70 inbox cases and 90 combined wallet-pay cases, with all five prior red regressions
passing. Record actual counts for the remaining suites (preload six; Electron security
32 expected); syntax/build success is not a test count. No skipped or empty suite may
establish green. Both standalone and combined runner must return without forced success
exit or timeout. The existing build checks local source syntax; no Rust/Go rebuild.

## 2. Falsify read-label behavior in an isolated source snapshot

Copy exactly these eight reviewed files, preserving their relative layout, beneath
`dist/pay001-finish-verify01/falsification/`:

- test/paymentInbox.node.js
- test/fixtures/payment-inbox/records-v1.json
- wallet-pay/inbox-client.js
- wallet-contract/canonical.js
- social/core.js
- social/app.js
- social/payment-inbox.js
- social/index.html

Verify all copied hashes against review 05 before changing anything. This Node suite
uses core modules and those local files; no copied node_modules or symlinks are needed.
In the copied social/app.js only, require exactly one match and replace:

```text
if (status) status.textContent = textStatus(entry.message);
```

with:

```text
/* reviewer-authorized read-label falsification */
```

Record the precise diff and copied file's mutated hash. Run from the repository root:

```text
timeout --signal=TERM --kill-after=5s 120s node dist/pay001-finish-verify01/falsification/test/paymentInbox.node.js
```

Require nonzero exit due to `outgoing read labels update in place after a live event
and fallback poll`; retain all assertion results. Syntax/module/setup failure or timeout
is not successful falsification. Restore the copied app.js from the still-pinned working
source, verify its exact original hash and rerun that same command for all 70 green.
Even if falsification is invalid, restore the copy, record the failure and stop; never
leave it presented as delivered source. The actual working source is never mutated.

## 3. Stage and run the real Messages UI with Chromium sandbox enabled

Use the already-authored 903-line driver at the review-05 hash. It loads actual
main/preload/renderer with isolated loopback fixtures; this is not a live payment run.
The following runtime recipe replaces execution of the superseded standalone UI handoff.

Before substantial copying, inspect destination filesystem type and free space. Require
disk-backed storage sufficient for runtime and captures. Read actual host uid, NSpid,
Seccomp, NoNewPrivs and execution confinement. A namespace-mapped ownership display is
not proof of host ownership. Use normal tool escalation if confinement prevents this
sandboxed run; do not bypass restrictions. If unavailable, report the exact blocker.

Verify `/opt/google/chrome/chrome-sandbox` is an existing regular non-symlink file,
host uid/gid 0/0, mode 4755, size 15232 and SHA-256
`c100b678a8c171ad0733e51b6f18d98d936d38ab945681c41da00f2ee22e7571`.
Its parents must be root-owned, not group/other writable and without unexpected link
resolution. Require the original Electron executable SHA-256
`180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`.
Mismatch stops this phase; no helper installation, chmod/chown or host policy change.

Copy `node_modules/electron/dist/` to `dist/pay001-finish-verify01/runtime/`, omitting
only the top-level chrome-sandbox. Preserve modes and bytes. Do not hardlink/symlink the
executable to the original (its executable directory must be the isolated runtime).
Reject unexpected special files or links before copying. Record every original/copied
relative path, type, size, mode and SHA-256. Require identical included entries and
exactly that one omission. Confirm no adjacent helper file/link in the new runtime.
The original installation stays untouched.

Run once from the repository root, with a 300-second outer process-group deadline:

```text
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-finish-verify01/ui-01 xvfb-run -a dist/pay001-finish-verify01/runtime/electron test/paymentInbox.electron.js
```

No sandbox/security disabling flags or empty helper variable. Retain CHROME_DEVEL_SANDBOX,
BBD_PAY001_ARTIFACT_DIR and assigned DISPLAY only as command environment evidence.
Keep the driver's OS sandbox, webPreferences, network isolation and DOM checks unchanged.
Require exit 0 and sandbox-status.json proving renderer Seccomp 2, NoNewPrivs 1 and
an additional renderer NSpid level; sandbox/contextIsolation/webSecurity enabled,
Node integration disabled. Keep electron-smoke.json and all four real screenshots:

- fixture-empty-messages-1440x1000.png
- fixture-request-card-980x720.png
- fixture-mixed-1440x1000.png
- fixture-unavailable-980x720.png

Record dimensions and hashes and link each image from the report. The assertions must
exercise request-only conversations, mixed messages/cards, keyboard/pointer details,
read-only amounts/status, automatic updates, cancellation/expiry, reading position,
focus/draft preservation, bottom following, other-peer activity, identity mismatch and
unavailable/recovery. Screenshots alone cannot replace those assertions. Label data as
fixtures. Any helper/API/sandbox/UI failure stops; keep precise error and partial images.
No unchanged rerun or administrative workaround is authorized.

## Report and return

Save the designated report even on failure. Include actual tool identity; input/runtime
manifests before/after; exact commands/results/counts; raw evidence links/hashes;
falsification/restoration; UI proofs/images; any remaining gap. Verify original reviewed
source and installed runtime bytes unchanged and index still empty. No owner observation
may be invented. Return the report pointer; the owner is not required to transcribe logs.

This completes the verification assignment only. Final security scans and exact-path
publication still require reviewer authorization. Inherited 88-pass/four-fail policy
baseline and release blockers remain open. No broader scanner, release or funding run.
