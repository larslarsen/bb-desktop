# BBD-PAY-001 Grok automatic inbox 01

Actor: Grok Build 4.6 High (xAI), owner-relayed. Reviewer: Codex.
Handoff: [GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md](../handoff/GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md).
Ticket: [BBD-PAY-001](../../tickets/BBD-PAY-001.md).

This is Grok's focused-source completion record. It is not Hermes acceptance.
No Git mutation, dependency install, real daemon/wallet, or Electron launch.

Desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Starting hashes of the four writable paths matched the handoff table. Capture
filesystem `dist/pay001-automatic-inbox01` is `ext2/ext3`. Prior reports and
unrelated dirty work were not overwritten.

## Behavior

Requests no longer has Connect or Refresh. There is no daemon/folder instruction
and no replacement setting. `app.js` mounts the component with `get` only
(`getPaymentInbox`), activates on the Requests view, deactivates when leaving,
and disposes on `pagehide`/`unload`.

While Requests is selected and the document is visible, the component reads the
existing authenticated snapshot immediately, then 5000 ms after each completed
read. Hidden/minimized windows pause the timer and read immediately on return.
At most one bridge read and one timeout are owned. Repeated activation does not
duplicate work; a pending read records one fresh-read need. Deactivate/dispose
bump generation so late results cannot change the DOM.

Loading copy is shown only when no snapshot exists. Unchanged DTOs leave the DOM
intact, including open details and scroll position. Changed snapshots restore
open details by `request_id`. Failed reads clear rows and show unavailable, then
retry on the same cadence.

Exact empty/unavailable/invalid/identity copy is in `social/payment-inbox.js`.
Identity changes stay fail-closed until a normal app restart. Internal
client/IPC/preload APIs were not refactored.

## Hashes after this pass

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| social/payment-inbox.js | 309 | `2de139dbd8bbbc3a8a6a7d03ad1f3e5003ede6b374f6d432582025a4994071b0` |
| social/app.js | 775 | `d26bf465457a47b9a72e6e0bc0a68d0fb7f8b2ff3108919f15899774ef327214` |
| test/paymentInbox.node.js | 1605 | `4b6a858ccf3c4ae3bfcf637e11cad1cc2bd1b21c20597b4ce040f732d13ba7eb` |
| test/paymentInbox.electron.js | 646 | `113904ede628e660246a294ccaaa38972f3f77cada92ffd306f9530d649b801b` |

Unchanged frozen inputs sampled from review 04 / correction 02 still match,
including `wallet-pay/inbox-client.js`, `social-main.js`, `wallet-preload.js`,
`social/index.html`, `social/styles.css`, `test/electronSecurity.node.js` and
`wallet-contract/canonical.js`.

## Commands (cwd `bb-desktop`)

Capture driver: [capture-run.js](../../dist/pay001-automatic-inbox01/capture-run.js)
SHA-256 `aa42cfc676934f3aaa9db5a457a9d1e024530793e99bd10e98f24e534cc17428`.

### Red (lifecycle tests vs previous UI)

`node test/paymentInbox.node.js` under
[run-01-red](../../dist/pay001-automatic-inbox01/run-01-red/).

| Field | Value |
| --- | --- |
| metadata | [inbox.json](../../dist/pay001-automatic-inbox01/run-01-red/inbox.json) SHA `d2e01083e05c3564a1c90c5ab65379a84f7c2d474b752160991fb0d9a1d98fce` |
| started_utc | `2026-09-15T22:44:37.046Z` |
| ended_utc | `2026-09-15T22:44:42.269Z` |
| exit_code | 1 |
| stdout SHA | `fbd6f318cf5ca241bf11ff96991cc451773729ee0be08d7b80e59ae3ff844920` |
| stderr SHA | `837d3389ca9dc71d1822e8096e3ed0537d6e79c371b7fda37d2b57dab24bac60` |

The prior 38 transport tests passed. All ten new renderer/app tests failed:
missing `deactivate`, buttons still present, no five-second poll, triple
activation started three reads, old empty/unavailable copy, and
`connectPaymentInbox` still in `app.js`.

### Green

| Command | exit | result | metadata |
| --- | ---: | --- | --- |
| `node test/paymentInbox.node.js` | 0 | 48 tests | [inbox.json](../../dist/pay001-automatic-inbox01/run-02-green/inbox.json) SHA `6757719921dfdd84f598d25917f5bff233a1807217fc748d2b381c0407654adf` |
| `node --check social/payment-inbox.js` | 0 | syntax | [check.json](../../dist/pay001-automatic-inbox01/run-03-check-inbox/check.json) SHA `013db4a0b00d6ceea815483d0e50a6013fad40d5e0a6755aa9b336003a3ba85a` |
| `node --check social/app.js` | 0 | syntax | [check.json](../../dist/pay001-automatic-inbox01/run-03-check-app/check.json) SHA `16fe7b403b53a9618806347108d03e10cf4b2c7f9ba88d9a03734b48a8c3ed83` |
| `node --check test/paymentInbox.electron.js` | 0 | syntax | [check.json](../../dist/pay001-automatic-inbox01/run-03-check-electron/check.json) SHA `7b48fae59432bf31a0cff449b3033f933ad4fb4888f1524a4ae3e5c85934fd03` |
| `node test/socialCore.node.js` | 0 | passed | [socialcore.json](../../dist/pay001-automatic-inbox01/run-03-socialcore/socialcore.json) SHA `e9c622ea435241444b6c5322238b4894d67d7440e141d6db26b4323bbd37976d` |

Green inbox stdout SHA `94240b6b5403a03a2f9848c4bb1c6f89b2969a208d5b5fad79f10d0654e698a7`.
Green inbox and all check stderr SHA
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` (empty).
Social-core stdout SHA `531bd421ff7ee0c247ac006087fe9f079f13508420a5a2c01687c5ec441883ab`.
Wallet/combined and preload suites were not rerun. Instance-binding falsification
was not rerun.

Renderer lifecycle tests use the real component in a deterministic DOM/event
fixture with a fake clock. They are not a substitute for the Electron smoke.

## Electron smoke

The existing smoke was updated for automatic transitions (empty → populated →
status/expiry → empty → disconnected), zero picker calls, no action container,
preserved open details, and the isolated stale-read check after deactivate.
Known-failing Electron launches were not attempted. Hermes UI runtime capture
remains pending after source review. That execution is not green.

Stop for Codex source review.
