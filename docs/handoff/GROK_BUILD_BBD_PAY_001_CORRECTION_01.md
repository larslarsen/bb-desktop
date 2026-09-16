# Grok — BBD-PAY-001 correction 01

Actor: Grok Build 4.6 High, owner-relayed. Reviewer: Codex.
Status: authorized test-first corrections and focused checks; stop for source review.

Read AGENTS.md, TESTING.md, CURRENT_TASK's active prefix,
`tickets/BBD-PAY-001.md` and `docs/testing/BBD-PAY-001-SOURCE-REVIEW-01.md`.
The original inbox handoff is superseded as an execution instruction. The original
ticket's product/trust requirements continue, with the corrections below.

## Baseline and scope

Require desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index,
all thirteen final hashes in `docs/testing/BBD-PAY-001-GROK-01.md`'s Changed paths
table and the six frozen input hashes in the original handoff. Codex independently
verified them. Preserve all unrelated dirty work. Stop and record unexpected drift.

Writable tests first:

- `test/paymentInbox.node.js`
- `test/paymentInbox.electron.js`
- `test/fixtures/payment-inbox/records-v1.json`
- `test/electronSecurity.node.js`
- `test/walletPreload.node.js`

Then writable production:

- `wallet-pay/inbox-client.js`
- `social-main.js` (payment handlers/lifecycle only; wallet semantics stay fixed)
- `social/payment-inbox.js`
- `social/styles.css` (only inbox layout/focus/overflow fixes demonstrated by smoke)

Preserve `test/walletPay.node.js`, `wallet-preload.js`, `social/app.js` and
`social/index.html` at the reviewed drop hashes. No other source, dependency,
policy, Rust, daemon or governance edits. Report only in the new
`docs/testing/BBD-PAY-001-GROK-CORRECTION-01.md`.
Append-only captures/runtime fixture storage: `dist/pay001-grok-correction01/`.
Do not overwrite the original report or its eight logs.

## Exact repairs and regression requirements

### 1. Owned and bounded HTTP (review R1)

Set `maxHeaderSize: 16 * 1024` on the actual HTTP request. Use one settlement/cleanup
path that clears the absolute deadline, detaches abort listeners and destroys owned
request/response resources on rejection/abort. Rejected responses must not continue
draining after return. Keep successful 200 bodies limited to 4 MiB; for 503 bound
stream consumption to 256 bytes, then return unavailable on excess, preserving
too_large for a valid bounded TOO_LARGE response. Other rejected statuses/encodings
terminate immediately. Capture premature close/truncation into a closed error.
Test streaming 302/500, unsupported encoding, oversized 503, header limit, aborted
responses and server-observed socket closure, including dispose after a rejection.

### 2. Generation, picker and lifecycle ownership (review R2)

Recheck generation/disposal after descriptor I/O and before any HTTP or identity
mutation; stale work must never launch HTTP. Guard all completion paths. Clear the
pending promise only if it is the same promise being settled. Switching selections
aborts previous work, and stale success/error completion cannot unpin or affect the
new selection. After a native picker resolves, recheck disposal before mutating state.
Convert picker exceptions to a sanitized unavailable DTO and retain existing selection.
Renderer increments an operation sequence and ignores superseded results; handle
synchronous bridge failure in the same sanitized path.

Dispose the inbox on approved quit and actual window destruction. A rejected wallet
shutdown keeps the app and inbox operational. Preserve every existing wallet shutdown
assertion. Add controlled deferred-I/O regressions for dispose/reselect during descriptor
read, late old completion after a new request starts, coalescing after that completion,
concurrent picker calls, failed picker and failed-versus-approved wallet shutdown.
Use a test-only fs mock before requiring the module if needed; no product test flags.

### 3. Descriptor and record boundaries (review R3/R4)

lstat descriptor to reject non-regular types, open with O_NOFOLLOW|O_NONBLOCK|O_RDONLY,
then fstat and enforce ownership/type/mode/size. Read within cap, reject short reads
and detect file growth using a capped extra byte; close the handle on every path.
Add an owned FIFO regression that completes promptly without network; using the
system `mkfifo` from the Node test for this fixed fixture is authorized. Leave fixture
directories for later explicit cleanup, not computed recursive deletion.

Keep all decoded request associations (including outbound) until cancellation
validation ends. Wrong-party inbound requests and linked signer/direction conflicts
invalidate the entire snapshot. Only truly unmatched cancellations are ignored.
Validate signature as canonical nonempty base64 bounded to 1024 encoded characters,
matching the existing public-key bound. Validate received_at as an actual calendar
timestamp in the accepted daemon UTC RFC3339 format, rejecting normalized impossible
dates. No cryptographic reimplementation and no frozen decoder edits.

Use valid descriptor/envelope JSON padded with whitespace for below/at/above cap
cases; below/at must succeed before above is rejected. Supply otherwise-valid paid
and expired status fixtures with independently computed domain-separated SHA-256 so
the event-policy assertion cannot pass on a wrong digest. A standalone standard-library
hash of fixed canonical bytes/domain, independent of the production decoder, is allowed.
Add same-peer rotation after a successful initial read, changed response peer, hostile
endpoint table with zero HTTP hits, wrong ownership through a scoped stat mock,
root/private-dir symlinks, invalid descriptor encoding, cancelled-after-expiry, exact
amount/tie-order property cases and all cleanup regressions above. Keep the original
useful cases, remove no assertions to obtain green.

### 4. IPC arity and side effects (review R5)

Payment handlers accept event plus a rest array and reject any nonempty payload array,
including `[undefined]` and `[undefined, object]`, before disk, HTTP or dialog work.
Preload remains a fixed eight-method no-argument bridge for the two new operations.
Mock the client factory in Electron security tests with get/connect/dispose counters.
For both handlers prove correct calls, untrusted sender/frame/URL rejection, exact
arity, zero unauthorized side effects and cloned closed DTOs. Node client tests prove
actual disk/HTTP behavior. Do not weaken the wallet's separate existing IPC contract.

### 5. Real UI smoke and screenshots (review R6)

Initialize disposable userData/home paths, fixture state and module mocks synchronously
before the first async yield, and require the real production entry before its ready
event. Alternatively hold only the test harness's startup registration until fixture
setup completes; never re-emit global ready or add a production bypass. Register
session network interception before the page loads: cancel all renderer http/https/
ws/wss traffic and record the attempted default social connection being blocked.
The real Node loopback inbox client stays active. Mock only the wallet broker and
native picker; no real user data, wallet or social daemon activity.

Do not assign HOME or CODEX_HOME. Use app paths and task-specific variables. Keep
all new runtime fixtures within the designated disk-backed capture directory. Install
failure-safe cleanup for owned sockets, requests, timers, window and module/dialog
patches; leave owned files in place. Await completed UI state after picker cancellation.
Use actual input for keyboard details/focus checks; do not substitute `.open = true`.

For this isolated smoke, this exact command is authorized:

```text
BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok-correction01 xvfb-run -a node_modules/.bin/electron --disable-setuid-sandbox test/paymentInbox.electron.js
```

This bypasses only the misconfigured SUID helper. It is conditional on positive OS
sandbox proof: keep app.enableSandbox and window sandbox/contextIsolation/webSecurity
true, nodeIntegration false. Obtain the actual renderer PID from webContents and
retain that owned PID's Linux status fields showing Seccomp 2 and NoNewPrivs 1.
Compare renderer/browser NSpid fields to prove the renderer has an additional PID
namespace level; retain their uid_map and namespace identifiers where readable.
Electron's ProcessMetric.sandboxed is documented only for macOS/Windows in the local
type declarations, so do not require it on Linux. Do not treat preferences or exit 0
alone as OS sandbox evidence. No
`--no-sandbox`, disable-seccomp flags, root helper changes, sysctl/AppArmor changes,
package-script changes or product installation recommendation. If the environment
still prevents sandboxed execution, record exact failure and stop the UI attempt;
finish independently authorized source/Node fixes. Do not silently downgrade.

Use nonempty fixtures demonstrating requested, cancelled and expired rows, long real-
length peer IDs and bounded long memo values. Main-only injected clock is permitted
in the test factory while using the real client/HTTP boundary; no product fixtures.
Assert visible Requests navigation, exact displayed rows/amounts/status, no overlap or
horizontal clipping in inbox controls/rows at 1180x780 and 860x620, readable details,
keyboard focus, empty/disconnected states and stale reply protection. Capture the
four screenshots required by the ticket and retain actual pixel dimensions/hashes.
Clearly label fixtures in the report; do not fabricate screenshots or accept placeholders.

## Commands, captures and stop

Author regression tests first. Capture failing assertions against the reviewed source
before repairs. The original missing red remains a historical gap; do not relabel a
new correction red as evidence of the original chronology. Allowed focused commands:

```text
node test/paymentInbox.node.js
node test/walletPreload.node.js
node test/electronSecurity.node.js
node test/walletPay.node.js
```

Run the first three affected targets red then green as appropriate; run walletPay once
after focused fixes pass. Require all final commands exit 0, nonzero assertion counts,
zero failures/skips, and promptly terminated owned resources. Preserve a new captured
falsification of response-instance equality and exact source restoration as the ticket
requires. Remove the product's dedicated `instanceBindingEnabled` toggle/export and
mutate the actual equality check temporarily instead; no unnecessary product test hook.

Use a small local capture driver only under the ignored directory, with explicit argv,
cwd, UTC start/end, actual process exit/signal, stdout/stderr log names and hashes in
per-command JSON metadata. Inspect filesystem type first. Capture Node/Electron versions
and executable SHA-256 read-only. Never overwrite captures: use numbered run directories
if retrying, updating BBD_PAY001_ARTIFACT_DIR to that literal directory accordingly.
No acceptance scanners, npm installs, broad tests, Git mutations, actor launches, real
app/daemon restart or modification of the frozen original logs/report.

The new report must map every R1–R6 item to changed source and exact regression/evidence,
list all changed/frozen hashes and counts, and link logs, metadata and screenshots with
SHA-256. Record remaining gaps and actual nonzero results. Save that document before
returning its pointer. Codex reads it directly; the owner does not relay evidence in chat.
