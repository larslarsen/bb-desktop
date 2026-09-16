# Grok — BBD-PAY-001 UI finish 01

Actor: Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, the PAY-001 ticket and
`docs/testing/BBD-PAY-001-SOURCE-REVIEW-03.md`.
Correction-02 transport/Node fixes are accepted for this gate; prior source handoffs
are closed for execution. Finish the actual isolated UI validation and screenshots.

## Baseline and narrow scope

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index and all nineteen
current source/input hashes listed in `docs/testing/BBD-PAY-001-GROK-CORRECTION-02.md`.
Only these source files may change:

- `test/paymentInbox.electron.js`, initially SHA-256
  `a19ec5671eedd8a1b77876b3d590a2606892ba2910bb3e646fda7ecac38a3893`;
- `social/styles.css`, initially SHA-256
  `4673ece3b62454458c0cc2f538a120073b8d9e0d73f83834c1d63716c5a31b31`,
  only for actual inbox layout failures demonstrated by the smoke.

All other source/test/dependency/daemon inputs stay frozen. No transport refactor,
Node-suite rerun, acceptance scanner or new fixture framework. Preserve all old reports,
raw captures and unrelated dirty work. New actor report only:
`docs/testing/BBD-PAY-001-GROK-UI-FINISH-01.md`.
Ignored capture/driver/runtime fixture directory: `dist/pay001-ui-finish01/`.

## Sandbox prerequisite — read and use existing installation only

Read-only host checks before the new run:

```text
stat -Lc '%u %g %a %s %n' /opt/google/chrome/chrome-sandbox
sha256sum /opt/google/chrome/chrome-sandbox node_modules/electron/dist/electron
```

Require helper uid/gid 0/0, mode 4755, regular non-symlink file, size 15232,
SHA-256 `c100b678a8c171ad0733e51b6f18d98d936d38ab945681c41da00f2ee22e7571`.
Also check its parent directories are root-owned and not group/other writable, with
no unexpected symlink resolution. Require Electron SHA-256
`180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`.
Record actual runner uid/NSpid/Seccomp/NoNewPrivs and filesystem type. A namespaced
"nobody" owner is not a successful host uid-0 check; use the normal approved host tool
context. If the tool requires escalation, request it normally for the bounded fixture
run. No permission bypass. Stop this route if the host prerequisite cannot be verified.

Use the installed helper by a command-local environment value, keeping Chromium's
renderer sandbox active:

```text
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-finish01/run-01 xvfb-run -a node_modules/.bin/electron test/paymentInbox.electron.js
```

Do not include `--disable-setuid-sandbox` or `--no-sandbox`. Do not set an empty helper
value. No sudo/chown/chmod/copy of privileged helpers, AppArmor/sysctl edits, disabling
seccomp/namespaces, dependency changes, user profiles, real wallet/daemon or app restart.
The documented helper route is a conditional attempt, not an assertion of compatibility.
If helper version or namespace setup fails, retain the error and stop environment
attempts. Do not install another helper or repeat the old failed command.

## Actual UI completion

Run the existing smoke once after prerequisites to expose actual assertion failures.
Correct only the bounded harness issues and demonstrated inbox CSS failures, keeping
the real main/preload/renderer/HTTP path and isolated fixture data intact. Specifically:

- Closed details paragraphs are intentionally not laid out. Do not compare those zero
  rectangles against a visible parent. Require a visible summary for each row and
  nonzero geometry for required controls. Exercise each row's details through keyboard
  input, assert full values visible when expanded and validate their geometry there.
  Then test normal collapsed layout, excluding only the known collapsed descendants.
- Preserve checks for long memo/full peer text, precise amounts and all three statuses,
  no text/control clipping or column overlap at 1180x780 and 860x620, keyboard focus,
  empty/disconnected states, completed picker cancellation and out-of-order renderer
  results. Do not delete/weaken a valid assertion to obtain a screenshot.
- Preserve social HTTP/WebSocket blocking before load, fake wallet/native picker,
  fractional receipt timestamps, actual sandbox/webPreferences checks and cleanup.
  OS proof remains Seccomp 2, NoNewPrivs 1 and additional renderer NSpid level relative
  to browser. Missing proof is a failed smoke, not a reason to remove the assertion.

Repeat only this smoke after bounded edits, using new numbered capture directories
and matching literal BBD_PAY001_ARTIFACT_DIR values. Syntax checks for the changed
harness are allowed (`node --check test/paymentInbox.electron.js`). Do not rerun the
already verified inbox/security/combined/preload suites for a UI-only drop.

Retain four actual screenshots with sizes/hashes and the successful smoke manifest:
nonempty at both sizes, empty at 1180x780, unavailable at 860x620. Label data as
fixtures in the report. Only a fully passing smoke with OS proof satisfies this gate.
If a new failure requires other source paths or host changes, save it as a concrete
blocker without editing outside this handoff.

## Durable return

Capture exact argv, cwd, allowlisted command-local environment, start/end, exit/signal,
stdout/stderr and SHA-256 in new immutable command metadata. The ignored capture driver
must settle once, wait for output streams, use new/exclusive paths and retain spawn
errors/timeouts accurately. Preserve any earlier partial screenshots as failed-run
artifacts. Never overwrite original evidence or present partial captures as green.

Write `docs/testing/BBD-PAY-001-GROK-UI-FINISH-01.md` with host preflight facts,
each assertion correction, source hashes/line counts, frozen-input verification,
command results and links/hashes for logs, metadata, sandbox proof and screenshots.
If blocked, name the exact prerequisite or command failure. No Git operations,
governance edits, actor launches or Hermes acceptance. Return the report pointer;
the owner does not transcribe execution evidence into chat.
