# Hermes — Messages UI diagnostic execution 03

CLOSED: captured social-fixture GET rejection reviewed in
[review 10](../testing/BBD-PAY-001-MESSAGES-REVIEW-10.md).
Next authority: [Sol Origin correction 01](SOL_BBD_PAY_001_MESSAGES_UI_ORIGIN_01.md).
Execution instructions below are historical. Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[source review 09](../testing/BBD-PAY-001-MESSAGES-REVIEW-09.md).
This is the sole execution authority. Older handoffs stay closed. No actor launched.

## Scope and frozen identities

Work in bb-desktop at HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Require all 21 input hashes in review 05 except the reviewed new driver:
`test/paymentInbox.electron.js`, 1371 lines, SHA-256
`698ae487068f24c4bdaff22912268acd3fa81cd9c6f917b38f06fdf6bc1181e9`.
Preserve unrelated dirty files and all previous evidence. Drift stops execution.
No product/test changes, dependencies, Git mutation, actor launch or user-process restart.
No live daemon/wallet/user profile or funds. No repeated Node suites, falsification,
broader build/security scans: the accepted partial results stand.

Writable report: `docs/testing/BBD-PAY-001-HERMES-UI-DIAGNOSTIC-03.md`.
New ignored capture directory: `dist/pay001-ui-diagnostic03/`.
Require new capture files; if this directory already exists, inspect/report the existing
state before any run rather than overwrite/repeat. Reuse the old runtime read-only at
`dist/pay001-finish-verify01/runtime/`; never overwrite old logs or ui-01.
Only executor metadata/capture plumbing and snapshots under the new directory are allowed.

## Purpose and failure evidence

The last run passed configured bootstrap and sandbox assertions, then failed
`configured ready empty payment state missing`. The preceding No conversations wait
passed. Do not repeat the prior report's incorrect interpretation. This driver adds
observations; it does not claim to fix that failure.

On failure retain ui-failure-diagnostics.json and fixture-failure.png where produced.
Read each readiness condition and the pre-probe fixture activity. Separately report the
single failure-only payment probe; it occurs after the original observations and cannot
establish that the earlier UI state was ready. Link the failed screenshot with dimensions/
hash and clearly label it diagnostic. Preserve bootstrap/sandbox proof and exact stderr.
Do not replace a failing result with a successful diagnostic probe or image.

## Record before executing

Previous runs repeatedly omitted command metadata. This run must retain actual files,
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
Before launching UI, verify actor.txt, source-before.json, driver-source.js,
preflight.json/raw preflight, runtime-before.json and completed syntax metadata/logs
actually exist and match their requirements. Initialize UI metadata before spawning.
Do not proceed on a prose-only preflight or an actor record named preflight.json.

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

Require exit 0. Then the diagnostic fixture journey, again with independent captures:

```text
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-diagnostic03/ui-01 timeout --signal=TERM --kill-after=5s 300s xvfb-run -a dist/pay001-finish-verify01/runtime/electron test/paymentInbox.electron.js
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
Old run metadata gaps remain in reviews 06 and 08 and must not be reconstructed.
Before returning, list the actual newly written artifact files and verify the required
metadata/manifests are present. Record any gap honestly; do not claim 21 before/after
pins or runtime manifests without the named files. No unchanged rerun to fill a gap.

Return the report pointer. Only Codex accepts the UI result or authorizes security scans
and exact-path publication. No further owner manual check is requested by this handoff.
Inherited policy/release blockers remain; there is no acceptance or release claim yet.
