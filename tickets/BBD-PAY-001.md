# BBD-PAY-001 — visible received payment requests

Status: COMPLETE — ACCEPTED AND PUBLISHED.
Feature: `f23950245c338d83072caa3d96503b69a21682e2`.
Actor closeout: `48c60b3f05591323b06f493df974819c50add76f`.
[Final reviewer acceptance](../docs/testing/BBD-PAY-001-FINAL-REVIEW-01.md) verifies
publication and closes all PAY handoffs. Read-only received-request display/automatic
updates are complete; creation/approval/transfer remain future work. Inherited release
blockers and historical evidence limitations remain explicit. No next task is authorized.
[Identity/transport direction](../docs/architecture/BB-IDENTITY-TRANSPORT-DIRECTION-01.md)
records portable accounts/readable names/WebRTC/video calls. Current read-only v1 binding
remains; migration must precede the next signed-recipient creation/approval contract.
All older execution authorizations below are historical.

## Hermes finish red history (closed)

Former handoff: [Hermes finish red 01](../docs/handoff/HERMES_BBD_PAY_001_MESSAGES_FINISH_RED_01.md).
[Review 03](../docs/testing/BBD-PAY-001-MESSAGES-REVIEW-03.md) pins the two authored
test files and unchanged production. Execute only its syntax checks and bounded Node
suite; no source edits or Electron. Earlier source authority below is closed.

## Sol finish test-source history (closed)
Recovery reviewed in [review 02](../docs/testing/BBD-PAY-001-MESSAGES-REVIEW-02.md).
Former handoff: [Sol finish tests 01](../docs/handoff/SOL_BBD_PAY_001_MESSAGES_FINISH_TESTS_01.md),
two test files only, gpt-5.6-sol High. No execution or production edits in this phase.

## Interrupted correction history (recovery closed)
Grok exhausted weekly usage after source edits and retained command captures, including
63-test restored green. Its completion report is absent. Former handoff:
[Hermes record recovery](../docs/handoff/HERMES_BBD_PAY_001_CORRECTION_RECORD_RECOVERY_01.md),
documentation only. No tests/source/Git execution. Sol High is the fallback if source
work remains after review. No acceptance is implied by the passing capture.
Governing design: [requests in Messages](../docs/architecture/BBD-PAY-MESSAGES-UX-01.md).
Show received request cards within the requesting peer's conversation, automatically
updated and discoverable without prior text history. Remove the standalone Requests tab.
Text messages must also update automatically: retain live events and reconcile the
conversation list/active history to recover missed events without user refresh.
Paused source contract: [Grok Messages correction 01](../docs/handoff/GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md).
[Review 01](../docs/testing/BBD-PAY-001-MESSAGES-REVIEW-01.md) verifies saved evidence
but requires session/lifecycle, rendering and Electron driver corrections.
Its four writable paths, tests, focused commands and report supersede all earlier source
execution scopes below. The earlier screen contract is historical; no actor may execute
it. Its frozen transport semantics and inherited final security thresholds remain.
Grok's automatic-inbox report is delivered; its handoff is closed. Hermes standalone
capture is superseded. No Hermes execution or publication is authorized yet.

## Previous automatic-inbox direction (superseded placement)

Former source handoff: [automatic inbox](../docs/handoff/GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md).
Owner decision: remove Connect and Refresh, including daemon/folder instructions.
Requests should connect and update automatically. Use existing default authenticated
access on activation and a five-second delay after each completed visible-view read;
pause while hidden and preserve displayed content during unchanged background reads.
This supersedes the original manual actions, no-polling clause, settings suggestion
and remove-connect-only handoff. No new protocol/subscription API. Hermes runtime
capture stays paused for revised source hashes and updated automatic-behavior assertions.

## Prior source review and execution history
Source review: [review 04](../docs/testing/BBD-PAY-001-SOURCE-REVIEW-04.md).
Paused execution: [Hermes UI runtime 01](../docs/handoff/HERMES_BBD_PAY_001_UI_RUNTIME_01.md).
The UI harness correction is source-accepted; runtime capture now belongs to Hermes.
An isolated runtime copy uses the existing installed sandbox helper without modifying
the original installation or host permissions. No product/test edits are authorized.
Supplementary owner checklist: [manual check](../docs/testing/BBD-PAY-001-OWNER-UI-CHECK-01.md).
Owner reports the empty inbox works, but "Connect local daemon" is unclear. The
manual-check document records a queued plain-language custom-folder setting follow-up;
no source changes during the active frozen Hermes execution.
Timestamp and transport fixes are source-accepted. Actual Electron UI remains
unverified. UI finish 01 bounds the existing-helper route and harness/layout fixes;
prior correction execution is closed. No final acceptance or publication yet.
Correction 01 addressed most first-review defects. Correction 02 fixes fractional
daemon receipt timestamps, closes remaining focused/UI assertions and records the
isolated screenshot execution context. Prior correction-01 execution is closed.
The original drop has visible UI source and focused Node green logs, but no successful
Electron smoke/screenshots. Correction 01 fixes the reviewed boundary defects and
authorizes a conditional sandboxed test-runner route; its exact paths/commands supersede
the original focused execution below. Broader product/trust requirements remain.
Reviewer: Codex. Source actor: Grok Build 4.6 High, owner-relayed.
Owner priority: a visible payment UI after the accepted daemon read endpoint.

## Outcome and boundaries

Add a **Requests** navigation tab with a **Payment requests** screen. It displays
received requests belonging to the selected local daemon: amount, asset, network,
requesting peer, memo, creation/expiry time and requested/cancelled/expired status.
Include Refresh, Connect local daemon, loading, empty and disconnected states.
The tab works independently of social API connectivity. Supply screenshots of the
actual Electron screen populated with clearly identified test fixtures.

This is a read-only inbox. Request creation, payment approval, sending funds, wallet
changes, paid receipts, daemon changes and packaging redesign require later contracts.
Do not add inactive Pay/Send buttons or claim that a peer status proves payment.

## Baseline and writable paths

Require desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, preserving the dirty
working tree. Daemon reference HEAD is `68aec8538e19722e56918645b4c7b8a897d77161`;
accepted endpoint feature is `82ed5f9c62ab22687a4972ba0ad59731bf43013e`.
Read `../bb-go/tickets/BBGO-PAY-003.md`, its final review and
`../bb-go/modern/localclient/server.go` as frozen references, not execution authority.

Author tests first, only in these paths:

- `test/paymentInbox.node.js` (new): export an awaited runner and standalone CLI.
- `test/paymentInbox.electron.js` (new): actual Electron fixture UI driver.
- `test/fixtures/payment-inbox/records-v1.json` (new): independent canonical fixtures.
- `test/walletPreload.node.js`: extend the closed bridge contract.
- `test/electronSecurity.node.js`: extend fixed IPC handlers and maintained-source
  coverage for both new modules; retain every existing security assertion.
- `test/walletPay.node.js`: only integrate the new awaited suite into the existing
  runner, preserving all old tests, exports and nonzero failure propagation.

Then production, only in these paths:

- `wallet-pay/inbox-client.js` (new): main-only descriptor/HTTP/schema/DTO boundary.
- `social/payment-inbox.js` (new): renderer component with no credentials or transport.
- `social-main.js`: own the client, fixed IPC handlers and lifecycle cleanup.
- `wallet-preload.js`: two fixed no-argument methods on frozen `bitbookWallet`.
- `social/app.js`: initialize the component and activate it through existing navigation.
- `social/index.html`: Requests tab, view and local script loading.
- `social/styles.css`: accessible layout within existing window dimensions.

Grok's sole writable report: `docs/testing/BBD-PAY-001-GROK-01.md`.
Ignored raw logs/screenshots/fixture runtime state: `dist/pay001-grok/` only. Inspect
filesystem type before storage; never place substantial artifacts in RAM-backed temp.
No dependencies, manifests, lockfiles, Rust, broker, other tests, policy, workflows,
user profiles, daemon-owned state or other repository documents may change.

Verify the handoff's starting hashes before editing. Stop and document unexpected
baseline drift or a necessary out-of-scope change; do not overwrite existing work.

## Fixed main-process trust contract

1. Linux only, matching the accepted daemon endpoint. Other platforms return
   `unsupported` and show an explanatory screen. Default directory is
   `path.join(app.getPath('home'), '.bitbook', 'modern')`, matching Go's default.
   Connect opens a native directory picker for the BitBook data folder. Selection
   lasts for this app session; do not invent persistent connection settings.
   Picker cancellation retains the existing selection. Renderer input never supplies
   a path, endpoint, peer, token, HTTP headers or arbitrary operation.
2. Read only `<selected-data-dir>/local-client/connection.json`. Reject symlink
   selected roots/private directories/descriptors, wrong ownership of the private
   directory or descriptor, and non-directory/non-regular types. Require private
   directory mode 0700 and descriptor mode 0600. Open descriptor with O_NOFOLLOW,
   fstat the opened file, cap reads at 16 KiB; do not chmod or create daemon files.
   The trust boundary assumes the current OS account; do not claim protection from
   an adversary already controlling that same account.
3. Require the descriptor's closed schema: `v:1`, nonempty bounded `peer_id`,
   `instance_id` of 32 lowercase hex digits, `token` of 64 lowercase hex digits,
   and endpoint exactly `http://127.0.0.1:<decimal port 1..65535>` with no suffix,
   credentials, path, query or fragment. Bound peer strings to 256 UTF-8 bytes.
4. Use Node core HTTP, fixed hostname 127.0.0.1, numeric port, GET
   `/v1/payment/records`, no query/body, `Authorization: Bearer <token>` and
   `X-BitBook-Instance: <instance>`. No Origin. Disable connection pooling;
   no proxy, redirects, cookies or decompression. Request identity encoding.
   Set maximum response headers 16 KiB, absolute request deadline 5 seconds, and
   maximum streamed response body 4 MiB before concatenation. Reject invalid UTF-8,
   non-JSON, unexpected encoding, non-200 and malformed/truncated responses safely.
5. Read a fresh descriptor on every refresh. Response `v`, `peer_id`, `instance_id`
   must match the descriptor. Pin peer identity only after a successful read; allow
   new port/token/instance for that same peer. A different peer returns
   `identity_changed` until an explicit successful Connect selection. Changing
   selection aborts old work; generation guards prevent stale responses overwriting
   newer state. Coalesce refreshes, bound picker concurrency, clean requests/timers
   on window destruction/quit. No automatic retry loop or periodic polling.
6. Use the existing exact main-window/frame/file-URL guard before either IPC handler
   can read files, make requests or open a dialog. Register only
   `payment:inbox:get` and `payment:inbox:connect`; reject any supplied arguments.
   Preload exposes only `getPaymentInbox()` and `connectPaymentInbox()` in addition
   to the six current methods. Preserve sandbox, isolation, permissions, navigation,
   wallet authorization and shutdown behavior. Never use the renderer's social API URL.

## Records and sanitized reply

Daemon returns a closed envelope `{v:1,peer_id,instance_id,records:[...]}`.
Each record has exactly `signed`, `digest`, `direction`, `received_at`; signed has
exactly `version:1`, `kind`, `canonical`, `public_key`, `signature`. Validate bounded
types, timestamps, digest and direction before decoding. Limit canonical UTF-8 data
to 64 KiB per record. Bound nesting before invoking a recursive parser. Reuse frozen
`wallet-contract/canonical.js`'s `decodeSignedObject` for request/status schemas and
canonical digest equality; do not duplicate its validation or change its policy.
Require canonical text to equal the decoder's canonical form. No new cryptographic
implementation: the authenticated daemon verifies signatures and signer/party binding.

Decode and validate all records before returning any rows. Allow request and
cancelled status only (`tx_ref` empty); malformed, contradictory or duplicate request
records fail the snapshot closed. Only show inbound requests whose payer is the
selected local peer. Validate outbound requests' payee against that peer but omit
them. Apply cancellations to matching request IDs with matching signed public key
and direction, preserving the daemon's payee-authority binding. Ignore orphan
cancellations, which the daemon can legitimately retain without a displayable request.
Do not interpret a paid or expired signed event as an allowed daemon record.

Return only a closed cloned DTO:

```text
{v:1, state:ready|unavailable|unsupported|invalid|too_large|identity_changed,
 peer_id:string, requests:[{
   request_id,digest,payee_peer_id,asset,network,amount_atomic,amount_display,
   memo,created_at,expires_at,status:requested|cancelled|expired
 }]}
```

Non-ready replies contain no request rows. No raw exception, response body, signature,
public key, canonical text, receiver, nonce, descriptor, token, instance, endpoint or
filesystem path crosses to renderer, logs or report. Peer ID is display context, not
renderer-granted authority. Asset/network remain validated by the existing decoder.
Format amount with decimal string arithmetic (ZEC 8, XMR 12 fractional digits), never
Number or fiat conversion. Status precedence: cancelled, then expired when
`now >= expires_at`, otherwise requested. Sort newest creation first, then request ID.
Refresh/activation recomputes expiry; timestamps are displayed with timezone context.

## Visible UI contract

Use textContent/DOM construction for all record values; no innerHTML or CSP expansion.
Show "Requests for <peer>" so the daemon identity is distinct from the social profile.
Show full peer and request IDs in accessible details, alongside useful amounts/memo.
Use explicit loading, no received requests, daemon unavailable, unsupported platform,
invalid response, too-large inbox and changed-identity states. Errors must remove stale
rows. Explain unavailable as needing a running compatible local daemon. Connection is
chosen through the native picker, not a credential input field. Do not display fixture
records in normal product startup or persist records in localStorage.

Keep navigation/rows usable at 1180x780 and the existing minimum 860x620, with keyboard
navigation, visible focus and semantic headings/buttons. Connect and Refresh are the
only new actions; the Requests tab must be discoverable even when the social API fails.

## Required tests and focused loop

Write tests before production, capture intended red, then implement and repair only
this scope until focused green. Initial missing-module red may establish scaffolding;
retain behavioral assertions that subsequently traverse the real boundaries.

```text
node test/paymentInbox.node.js
node test/walletPreload.node.js
node test/electronSecurity.node.js
node test/walletPay.node.js
BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok xvfb-run -a node_modules/.bin/electron test/paymentInbox.electron.js
```

The first three commands are the targeted red/green commands. Run walletPay after
runner integration and the Electron smoke after visible wiring. All final focused
commands require exit 0 with nonzero executed assertions, zero failures/skips. No
`--no-sandbox`, live public service, real wallet/broker, real daemon or user data.

Node tests use owned temporary descriptors and an actual loopback fixture HTTP server.
Assert correct method/path/auth/instance, bodyless request, refused hostile descriptors
before network activity, permissions/symlinks, same-peer rotation, wrong peer/instance,
late stale responses, unavailable server, absolute deadline and complete cleanup.
Exercise descriptor/body limits below/at/above bounds, invalid UTF-8, truncation,
malformed/deep payloads, paid events, precise large amounts, inbound/outbound filtering,
cancelled-before-expired precedence, boundary expiry and orphan cancellation. Include
nonempty valid independently pinned canonical/digest fixtures and deterministic property
cases; no oracle that merely calls the production function to generate expected data.
Assert credentials and raw transport details never appear in reply/error/log surfaces.

IPC tests reject wrong sender/frame/URL and extra arguments before side effects, and
verify the frozen exact eight-method preload and seven-handler main contracts.
Electron driver loads the actual main/preload/HTML/component with a fixture transport
and fake wallet supervisor installed only by the test harness before entry loading.
Use disposable userData and isolate social HTTP; no production test bypass flags.
Exercise tab clicks, refresh, picker cancellation, details, empty and disconnected
states through the actual renderer/preload/IPC path. Capture real screenshots at both
window sizes, nonempty and empty/unavailable states, and assert DOM contents/visibility.
Clearly identify screenshot data as fixtures in the report. Driver must exit nonzero
on assertions and clean only its own resources.

Falsification in the same focused loop: temporarily disable only the response instance
equality rejection. Run `node test/paymentInbox.node.js` and require its wrong-instance
case to fail; restore the exact pre-mutation source hash and rerun green. Retain both
results and restoration hashes. Never deliver the mutated source. No separate red
handoff or broad Rust proving-suite run is required.

## Final acceptance plan (Hermes after reviewer source gate)

Grok does not execute this section. Reviewer will enumerate final hashes and write the
Hermes execution handoff after inspecting source, focused evidence and screenshots.
Hermes records actual version/provider/model and exact commands, exits and raw logs in
`docs/testing/BBD-PAY-001-ACCEPTANCE-01.md`. Minimum final commands:

```text
npm run test:wallet-pay
node test/walletPreload.node.js
node test/electronSecurity.node.js
npm run test:social
npm run build
node --check wallet-pay/inbox-client.js
node --check social/payment-inbox.js
BBD_PAY001_ARTIFACT_DIR=dist/pay001-acceptance xvfb-run -a node_modules/.bin/electron test/paymentInbox.electron.js
node scripts/security-policy.js
node test/securityPolicy.node.js
npm audit --json
gitleaks version
gitleaks dir --redact --report-format json --report-path dist/pay001-acceptance/gitleaks.json dist/pay001-scan-input
```

Use pinned Gitleaks 8.30.1 on a manifest-hashed snapshot of the exact prospective
task publication paths, then verify staged bytes match it. The later handoff must
enumerate the snapshot paths; no dirty whole-repository staging or scan-result edits.
Require zero secret findings and no new dependency/security findings attributable to
this slice. Preserved npm inputs and policy files stay hash-identical. The inherited
policy baseline is 88 passing / four failing tests recorded in
`docs/testing/BBD-WAL-009-EXTRACTOR-POLICY-ACCEPTANCE-01-REVIEW.md`; failures of the
checker and those four tests remain release blockers, never reported as green. Any
additional failure or plausibly exploitable advisory requires review, not a waiver.
No blanket Rust/network scanner reruns, release build or SBOM claim for this JS slice.

The local Electron entry loads these JS/HTML/CSS files directly. Verify their final
identities at acceptance; no untouched Rust or Go rebuild is required. A full app
restart is needed to load new main/preload code; do not restart the user's processes.
Report the ordinary sandboxed launch command `npm run start:sandboxed` and screenshot
paths at closeout. An old daemon without the accepted endpoint shows unavailable.

## Completion gate

Grok saves exact changes, before/after hashes, line counts, tool identity, command
arguments/cwd/exits, test counts, falsification/restoration, immutable raw-log links and
SHA-256, screenshot links/dimensions/hashes and remaining gaps in its named report.
Sanitize fixture secrets; retain real command failures, never replace them with an
aggregate success. Every correction appends a new capture; do not overwrite evidence.
The user relays only the completion pointer. Codex reads the report and actual source;
no next phase is authorized on chat claims or a missing UI screenshot alone.
