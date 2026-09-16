# Grok — BBD-PAY-001 correction 02

Actor: Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, the PAY-001 ticket and
`docs/testing/BBD-PAY-001-SOURCE-REVIEW-02.md`. Correction 01 is closed for execution.

## Baseline and authority

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index, all eight changed
and five frozen drop hashes in `docs/testing/BBD-PAY-001-GROK-CORRECTION-01.md`, and
the six original frozen input hashes. Preserve previous reports/captures and unrelated
dirty work. Unexpected baseline drift must be documented, not overwritten.

Writable tests first: `test/paymentInbox.node.js`, `test/paymentInbox.electron.js`,
`test/electronSecurity.node.js`. Then writable production: `wallet-pay/inbox-client.js`
(received_at parser only, plus any HTTP repair directly required by the authorized
header/truncation regressions), `social/styles.css` (inbox wrapping/layout only).
All other source stays frozen, including the canonical decoder, fixtures, main entry,
preload, renderer JS and wallet runner. New report:
`docs/testing/BBD-PAY-001-GROK-CORRECTION-02.md`.
Ignored append-only logs/driver/runtime fixtures: `dist/pay001-grok-correction02/`.

## Required changes

1. Accept actual daemon UTC receipt timestamps with no fraction or one-to-nine decimal
   fractional digits before Z. Validate calendar/time fields independently of fractional
   precision; use only the first three digits, padded as needed, for a JS millisecond
   numeric value. Receipt time is not used to authorize a payment. Continue rejecting
   impossible dates, missing fraction digits, excess fraction digits and malformed UTC
   suffixes. Do not modify the signed canonical timestamp codec or its policy.
   Test whole seconds, .1Z, .123Z, .123456Z, .123456789Z and leap-day/calendar negatives
   as otherwise-valid loopback records. Capture the fractional positives failing against
   correction 01 before changing production. Include fractional received_at in the real
   Electron fixture records so UI evidence covers actual daemon serialization.
2. Wrap long memo and identity subtitle text within the available width; do not hide
   clipped content to pass layout checks. Preserve readable amounts, focus and details.
   At both window sizes assert header/actions/row contents do not overflow their
   containers and do not overlap adjacent columns; require scrollWidth <= clientWidth
   for relevant text containers and inspect child/control bounds. Check every displayed
   fixture row, the Requests tab, requested/cancelled/expired labels, full peer detail
   and long memo. Await nonbusy completed state after picker cancellation. Assert actual
   BrowserWindow security preferences plus the previously required Linux OS proof.
3. Complete these remaining focused regressions, with explicit resource cleanup and
   finite failure deadlines: HTTP parser header limit, real response socket destroyed
   before declared body completion, valid envelope padded below/at/above 4 MiB, a get
   while the newer generation is pending after old completion (no third fetch), actual
   entry into deferred descriptor I/O before dispose/reselection, selected-root symlink,
   both channels' sender/frame/URL/arity rejection and cloned DTOs. Preserve existing
   successful cases. Clear each test's timeout in finally, restore mocks in finally,
   and close owned servers/sockets even on assertion failure. Do not add broad tests.
4. UI harness must exercise an out-of-order renderer result without changing production:
   an isolated test DOM root may mount the real component with two deferred fixture
   bridge promises, resolve newer first then older, and assert the newer DOM remains.
   Keep this separate from the primary actual preload/IPC/HTTP fixture UI. All screenshot
   rows come from the real main-only client and owned loopback server.

## Focused commands and evidence

```text
node test/paymentInbox.node.js
node test/electronSecurity.node.js
node test/walletPay.node.js
```

Capture new behavioral red before production repair, then require focused green and
one final combined wallet/inbox green. Do not rerun unchanged preload or previously
verified instance-binding falsification: this correction does not alter that comparison.
Capture exact argv, cwd, start/end, process exit/signal and stdout/stderr hashes for
every command. The local capture driver must wait for log streams to finish before
hashing and writing metadata; use exclusive/new filenames and record spawn errors and
timeouts truthfully. Do not reconstruct missing historic metadata. No scanners or
broader acceptance at this source gate.

## UI execution context — recorded reviewer decision

After source corrections, run the isolated fixture smoke, retaining all Chromium
sandbox assertions from correction 01:

```text
BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok-correction02/ui-01 xvfb-run -a node_modules/.bin/electron --disable-setuid-sandbox test/paymentInbox.electron.js
```

Previous nested sandbox attempts already failed. Where the actor's execution tool
supports sandbox escalation, request its normal `require_escalated` execution for
this exact command outside the agent filesystem sandbox. This does not disable
Chromium's own renderer sandbox. Tool-required approval must still be obtained; the
handoff does not bypass it. Explain the narrow reason in that tool request: the isolated
fixture screenshot run needs to create Chromium's sandbox namespaces, which prior
confined execution could not create. No preliminary user transcription or host setup.
Record the actual execution context, and only a bounded allowlist of relevant runner
environment values (artifact directory and DISPLAY); never dump the environment.

If the actor has no such tool capability, record that fact. If escalation is rejected
or an outside-agent-sandbox attempt still fails, preserve the actual reason and finish
the independently authorized source/Node work. Do not repeatedly run the same failing
context. Do not change AppArmor/sysctl, chmod/chown helpers, disable seccomp/namespaces,
use --no-sandbox, access user profiles, run a real daemon/wallet, or restart the app.

Retain sandbox-status.json and four actual screenshots with dimensions/hashes when
successful. A partial capture is not a passed smoke. Keep reports honest about any
remaining environment limitation and distinguish it from a source/test failure.

## Return

Save the new report with review-02 item mapping, final hashes/line counts, unchanged
frozen inputs, exact command results, metadata/log links/hashes and UI artifact links.
Do not edit CURRENT_TASK or other governance. No Git mutation, Hermes execution,
dependency install, actor launch or source work beyond the paths above. Return the
report pointer; Codex reads the evidence directly from repository documents.
