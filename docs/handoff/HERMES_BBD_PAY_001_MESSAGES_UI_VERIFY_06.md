# Hermes — Messages UI verification 06

CLOSED: assertion-completion evidence accepted with execution-record gaps in
[review 16](../testing/BBD-PAY-001-MESSAGES-REVIEW-16.md).
Next authority: [Hermes evidence closeout and scans](HERMES_BBD_PAY_001_MESSAGES_SCANS_01.md).
No unchanged rerun. Instructions below are historical.
Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[source review 15](../testing/BBD-PAY-001-MESSAGES-REVIEW-15.md).
This is the sole execution authority. Older handoffs stay closed. No actor launched.

## Scope and frozen identities

Work in bb-desktop at HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Require all 21 input hashes in review 05 except the reviewed new driver:
`test/paymentInbox.electron.js`, 1599 lines, SHA-256
`f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54`.
Preserve unrelated dirty files and all previous evidence. Drift stops execution.
No product/test changes, dependencies, Git mutation, actor launch or user-process restart.
No live daemon/wallet/user profile or funds. No repeated Node suites, falsification,
broader build/security scans: the accepted partial results stand.

Writable report: `docs/testing/BBD-PAY-001-HERMES-UI-VERIFY-06.md`.
New ignored capture directory: `dist/pay001-ui-verify06/`.
Require new capture files; if this directory already exists, inspect/report the existing
state before any run rather than overwrite/repeat. Reuse the old runtime read-only at
`dist/pay001-finish-verify01/runtime/`; never overwrite old logs or ui-01.
Only executor metadata/capture plumbing and snapshots under the new directory are allowed.

## Purpose and exact prior failure

UI05 passed Enter close/reopen, then failed `pointer did not close details before
focus proof`. The summary was not established as visible/hittable after resizing.
Its named card screenshot showed only preceding messages; an absent-Origin typing POST
also produced an unexpected fixture403. Review 15 accepts the bounded targeting,
screenshot-framing and exact social GET/POST Origin corrections. Product files and
existing journey assertions remain unchanged. This run verifies the complete journey.

Use this run's actual assertion/error and source lines, including new target diagnostics;
do not copy an old diagnosis. Preserve exact stderr, ui-failure-diagnostics.json,
fixture-failure.png and all prior proof/screenshots on failure. The optional failure-only
payment probe cannot turn the failed UI run into a pass. Do not classify fixture403 as
expected: the journey requires zero fixture failures.

Inspect the actual request-card image: it must show the card heading/amount, not just
preceding messages. Record dimensions from PNG bytes, not filenames/window dimensions.
UI05 captured 1440x934 and 980x654 content at its named outer window sizes.

## Record before executing

Previous runs repeatedly omitted command metadata. This run must retain actual files,
not only report prose; use no approximate or reconstructed timestamps/results:

- actor.txt: actual `hermes --version`, resolved provider/model and `node --version`.
- source-before.json: every one of the 21 paths, SHA-256 and line count; compare pins.
  Count actual lines, not a trailing empty split item (UI05 overcounted every file by one).
- driver-source.js: an unchanged evidence copy of the reviewed Electron driver.
- preflight.json plus raw command output: destination filesystem/free space, actual host
  uid/confinement, helper/parent ownership/type/modes/hash facts below.
- runtime-before.json: complete original and copied per-path/type/mode/size/hash manifests
  and comparison, not just entry counts and an all_match boolean.
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

## Mandatory on-disk gate (before spawning UI)

UI05 omitted these files despite its instructions. This time the executor must read and
validate the following actual files before spawning the UI command. Implement capture
plumbing only under the new ignored directory; this is not permission to edit test source.

- actor.txt, source-before.json and driver-source.js.
- preflight.json and preflight.stdout.log/preflight.stderr.log, recording actual checks
  for disk/uid/helper/parent ownership and confinement. A summary without raw checks fails.
- runtime-before.json, containing original and copied entry arrays with relative path,
  type, mode and file size/hash, plus comparison. Count-only/all_match summaries fail.
- syntax.metadata.json, syntax.stdout.log and syntax.stderr.log. Metadata must already
  contain exact argv/cwd/allowlisted env, actual start/end, observed exit 0, signal/timeout
  status and SHA-256 of both completed logs.
- ui.metadata.json initialized before spawn with exact argv/cwd/allowlisted env and
  actual start time. Completion fields must remain null until actually observed.

Write prelaunch-gate.json with actual validation results and hashes of the checked files.
Require all checks true; missing/incomplete/mismatched files stop before UI. The gate must
check contents, not trust a prior all_match boolean or a promised report. Finalize UI
metadata after streams close on success or failure, recording the observed timeout
wrapper exit/signal without inventing a separate Electron child exit. Record unavailable
child status as null. Keep the wrapper source under the capture directory for review.
Use cleanup/finally to save source-after.json and complete runtime-after.json on failure
too. List missing evidence explicitly if an executor failure prevents this. Never use
reconstructed times or copied old metadata. No unchanged rerun to repair missing capture.

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
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-verify06/ui-01 timeout --signal=TERM --kill-after=5s 300s xvfb-run -a dist/pay001-finish-verify01/runtime/electron test/paymentInbox.electron.js
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
Old run metadata gaps remain in reviews 06, 08, 10, 12 and 14 and must not be reconstructed.
Before returning, list the actual newly written artifact files and verify the required
metadata/manifests are present. Record any gap honestly; do not claim 21 before/after
pins or runtime manifests without the named files. No unchanged rerun to fill a gap.

Return the report pointer. Only Codex accepts the UI result or authorizes security scans
and exact-path publication. No further owner manual check is requested by this handoff.
Inherited policy/release blockers remain; there is no acceptance or release claim yet.
