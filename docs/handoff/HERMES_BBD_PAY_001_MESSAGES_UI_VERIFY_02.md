# Hermes — Messages UI verification 02

CLOSED: partial bootstrap/sandbox evidence reviewed; UI readiness failed.
See [review 08](../testing/BBD-PAY-001-MESSAGES-REVIEW-08.md) and
[Sol diagnostics 01](SOL_BBD_PAY_001_MESSAGES_UI_DIAGNOSTICS_01.md).
Execution instructions below are historical. Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[source review 07](../testing/BBD-PAY-001-MESSAGES-REVIEW-07.md).
This is the sole execution authority. Older handoffs stay closed. No actor launched.

## Scope and frozen identities

Work in bb-desktop at HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Require all 21 input hashes in review 05 except the reviewed new driver:
`test/paymentInbox.electron.js`, 1110 lines, SHA-256
`ead9f500b8eaafbc19d30961e8327b992437838b09d033e3edacf0d4f79192e1`.
Preserve unrelated dirty files and all previous evidence. Drift stops execution.
No product/test changes, dependencies, Git mutation, actor launch or user-process restart.
No live daemon/wallet/user profile or funds. No repeated Node suites, falsification,
broader build/security scans: the accepted partial results stand.

Writable report: `docs/testing/BBD-PAY-001-HERMES-UI-VERIFY-02.md`.
New ignored capture directory: `dist/pay001-ui-verify02/`.
Require new capture files; if this directory already exists, inspect/report the existing
state before any run rather than overwrite/repeat. Reuse the old runtime read-only at
`dist/pay001-finish-verify01/runtime/`; never overwrite old logs or ui-01.
Only executor metadata/capture plumbing and snapshots under the new directory are allowed.

## Record before executing

Previous runs repeatedly omitted command metadata. Retain the following actual files,
not only report prose; use no approximate or reconstructed timestamps/results:

- actor.txt: actual `hermes --version`, resolved provider/model and `node --version`.
- source-before.json: every one of the 21 paths, SHA-256 and line count; compare pins.
- driver-source.js: an unchanged evidence copy of the reviewed Electron driver.
- preflight.json plus raw command output: destination filesystem/free space, actual host
  uid/confinement, helper/parent ownership/type/modes/hash facts below.
- runtime-before.json: complete original and copied runtime manifests and comparison.
- For each syntax/UI command: separate stdout.log, stderr.log and metadata.json with
  exact argv, cwd, allowlisted environment, actual start/end, exit code, signal/timeout
  and both log hashes. Save start metadata before spawn and finalize after stream closure.

Use a bounded executor wrapper that owns its spawned process group. It may record the
actual child exit/signal and terminate only that group on deadline. Never dump the whole
environment, credentials, descriptor content or user files. Actor/version/preflight
read-only commands are authorized. A missing required preflight stops before the UI run.
No metadata file may assert a future check passed.

## Runtime preflight and reuse

Confirm disk-backed artifact storage and adequate free space. No runtime recopy needed.
Use the actual execution context; namespace-mapped ownership is not host-ownership proof.
Normal tool escalation is allowed when confinement prevents the already-authorized
sandboxed test; if unavailable, document the exact blocker. Do not bypass confinement.

Verify `/opt/google/chrome/chrome-sandbox` regular non-symlink, host uid/gid 0/0,
mode 4755, size 15232 and SHA-256
`c100b678a8c171ad0733e51b6f18d98d936d38ab945681c41da00f2ee22e7571`.
Parent directories must be root-owned, not group/other writable, no unexpected symlink
resolution. Verify original and copied Electron executables both SHA-256
`180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`.

Compare `node_modules/electron/dist/` against `dist/pay001-finish-verify01/runtime/`
for every relative path/type/mode/file size/hash. Require exactly one omission, the
top-level chrome-sandbox, no extra entries and no differences. Reject special files or
unexpected links; the copied executable must not link back to the original. Confirm no
adjacent helper file/link. Save the complete manifest/comparison now; the reviewer's
present-file comparison does not replace this evidence. No chmod/chown, helper install,
AppArmor/sysctl change or sandbox/security-disable flags.

## Execute once, sequentially, from bb-desktop

Syntax, with independent raw captures and metadata:

```text
timeout --signal=TERM --kill-after=5s 30s node --check test/paymentInbox.electron.js
```

Require exit 0. Then the corrected fixture journey, again with independent captures:

```text
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-verify02/ui-01 timeout --signal=TERM --kill-after=5s 300s xvfb-run -a dist/pay001-finish-verify01/runtime/electron test/paymentInbox.electron.js
```

Retain CHROME_DEVEL_SANDBOX, BBD_PAY001_ARTIFACT_DIR and assigned DISPLAY only as
command-local environment evidence. Retain timeout wrapper status and actual child
exit/signal when the wrapper exposes them; do not infer a missing child status. Treat
124/137 or any forced termination as failure. No unchanged rerun and no repair by Hermes.

Require exit 0 and all of:

- bootstrap-diagnostics.json with successful exact-document setup. This alone is not a
  passing UI run. Preserve setup failure diagnostics if startup fails.
- sandbox-status.json showing renderer Seccomp 2, NoNewPrivs 1 and additional renderer
  NSpid level. The unchanged assertions also require sandbox/contextIsolation/webSecurity
  true and nodeIntegration false on the configured renderer.
- electron-smoke.json confirming no unexpected blocked requests/fixture failures and
  the complete actual main/preload/renderer journey.
- Four real PNGs: fixture-empty-messages-1440x1000.png,
  fixture-request-card-980x720.png, fixture-mixed-1440x1000.png and
  fixture-unavailable-980x720.png. Record actual dimensions/SHA-256 and link each from
  the report. Label the displayed data as test fixtures, not live owner payments.

Keep all request-only/mixed conversation, automatic update, cancellation/expiry,
keyboard/pointer, focus/draft/anchor/node stability, bottom-following, identity mismatch
and unavailable/recovery assertions. No fabricated screenshot or manual observation.
If any phase fails, retain exact error/diagnostics/partial images and stop. Classify
what the evidence proves; do not declare an environment cause without captured support.

## Finish the report even on failure

Save source-after.json and runtime-after.json, compare against before/pins, and check
HEAD/index unchanged. Record only facts actually observed. Link all metadata/raw logs,
manifest hashes, diagnostics, UI proof and screenshots. State missing files explicitly.
Old run metadata gaps remain in review 06 and must not be reconstructed.

Return the report pointer. Only Codex accepts the UI result or authorizes security scans
and exact-path publication. No further owner manual check is requested by this handoff.
Inherited policy/release blockers remain; there is no acceptance or release claim yet.
