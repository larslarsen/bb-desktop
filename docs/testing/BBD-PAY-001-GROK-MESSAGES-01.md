# BBD-PAY-001 Grok Messages integration 01

Actor: Grok Build 4.6 High (xAI), owner-relayed. Reviewer: Codex.
Handoff: [GROK_BUILD_BBD_PAY_001_MESSAGES_01.md](../handoff/GROK_BUILD_BBD_PAY_001_MESSAGES_01.md).
Decision: [BBD-PAY-MESSAGES-UX-01.md](../architecture/BBD-PAY-MESSAGES-UX-01.md).

This is Grok's focused-source completion record. It is not Hermes acceptance.
No Git mutation, real daemon/wallet, or Electron launch.

Desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Starting hashes of the six writable paths matched the handoff. Capture filesystem
`dist/pay001-messages01` is `ext2/ext3`. Prior reports were not overwritten.

## Behavior

Received requests appear as cards in the requesting peer's Messages conversation,
including peers with no text history. The Requests tab and standalone screen are
removed. There is no payment Connect, Refresh, settings, Pay or Send control.
The existing text composer Send button remains.

`social/payment-inbox.js` is now a snapshot controller plus card/merge helpers
(`createReader`, `bindSnapshot`, `mergeConversationList`, `mergeTranscript`,
`renderCard`). `app.js` owns one reader for the session, independent of Feed /
posts / Network. It reads at startup and on visibility, then 5000 ms after each
settlement. Chat reconciliation is a separate 5000 ms pass over conversations
plus the active history, with AbortController bounds. Failures in one loop do
not stall the other.

Payment cards bind only when `/ob/config` `peerID` exactly equals the
authenticated payment DTO `peer_id`. Missing or mismatched identity hides cards
and synthetic rows and shows `Payment requests unavailable for this identity`.
Session changes clear projection and abort stale social reads. Unchanged
snapshots do not rebuild the transcript. Electron execution is pending.

## Requirement → test map

| Requirement | Test |
| --- | --- |
| Arrival while Feed selected, request-only row, open it | `request-only peer appears in Messages while Feed is selected`; `opening a request-only conversation renders the card without text history` |
| Mixed ordering, two peers, no duplication | `mixed text and request ordering…`; `two peers stay distinct…` |
| Cancel/expiry, precise amount, hostile memo, details/composer | `cancellation updates the same card…` |
| Failed/empty history keeps request-only chat | `failed text history still keeps a request-only conversation open` |
| Missing/mismatch/match identity; API switch; A→B switch | `missing and mismatched identities…`; `switching API invalidates…`; `fast A to B chat switch…` |
| Ancillary social failure still binds; payment unavailable→recovered; no payment on social wire | `matching config binds payments when ancillary…`; `payment unavailable recovers…` |
| Poll, 5s delay, one call, hidden pause | `payment reader uses one call, five-second delay…` |
| No Requests tab; Send preserved; no focus steal | `standalone Requests tab…`; `other-peer request does not steal…` |
| Text auto-reconcile, other-peer row, hung abort, hidden receipts | `text reconciliation recovers missed events…`; `hung history aborts…` |
| 38 transport cases | unchanged names through `selected-root symlink is refused` |

Superseded standalone UI assertions (activate/deactivate Requests, Connect/Refresh
copy, isolated mount poll) were replaced by the Messages cases above. Scheduling
invariants now live on the session reader plus chat reconcile loop.

## Hashes after this pass

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.node.js | 2059 | `fd6f1eab04e7121bc6ed68f5ca8785465ee9f810265ed3efb09455f8027ad1ae` |
| test/paymentInbox.electron.js | 523 | `c5350be482f84bd276289e017accf3aaa9a3927f209deee0d67c8b3885d3502f` |
| social/payment-inbox.js | 340 | `3aa647a76f66c90c7b2929aa67f041b565675f4ea57eb4d7dd4978d1ac841884` |
| social/app.js | 1013 | `b2ff043b5645f9c19bcf79ed965dfa7d758f9fe51b2a9a65dcb65c01cd32b17e` |
| social/index.html | 189 | `6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24` |
| social/styles.css | 988 | `f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898` |

Frozen inputs still match the handoff table: `wallet-pay/inbox-client.js`,
`social-main.js`, `wallet-preload.js`, `social/core.js`, `test/socialCore.node.js`.
Fixture JSON and the 38 transport tests were preserved.

## Commands (cwd `bb-desktop`)

Capture driver: [capture-run.js](../../dist/pay001-messages01/capture-run.js)
SHA-256 `aa42cfc676934f3aaa9db5a457a9d1e024530793e99bd10e98f24e534cc17428`.

### Red

`run-01-red` was a harness TDZ (`document` before initialization). It is retained
and is not the behavioral red.

Behavioral red: [run-02-red](../../dist/pay001-messages01/run-02-red/)
`node test/paymentInbox.node.js`.

| Field | Value |
| --- | --- |
| metadata | [inbox.json](../../dist/pay001-messages01/run-02-red/inbox.json) SHA `996c003c5be6e48beda3448995cdca801d780e9062bb50c0ba9d123b347b8c1c` |
| started_utc | `2026-09-16T00:11:01.494Z` |
| ended_utc | `2026-09-16T00:11:06.750Z` |
| exit_code | 1 |
| stdout SHA | `d0bc0494c56d571654f12e73481b2fa0ee710afc3b4413efdd151d2156b20a6e` |
| stderr SHA | `44bd8b93cad32600b0b029cc87bacb19625f069b443485b8b79e2251383652bb` |

The 38 transport tests passed. New Messages cases failed on missing request-only
rows, no automatic poll (`paymentCalls` 0), no identity notice, no automatic text
reconcile, and Requests markup still present.

### Green

| Command | exit | result | metadata |
| --- | ---: | --- | --- |
| `node test/paymentInbox.node.js` | 0 | 54 tests | [inbox.json](../../dist/pay001-messages01/run-03-green/inbox.json) SHA `de205524ae9080f3b80598b75f3c17a8854e7fe76405ee6aabab904dc2aae91f` stdout `c631f73d3c6183dbccbd7e8d6497b48dfe02868e529f859da4b106772e8d7a99` |
| `node test/socialCore.node.js` | 0 | passed | [socialcore.json](../../dist/pay001-messages01/run-03-socialcore/socialcore.json) SHA `9dd83c05b44b741815c4c6051bce47d03066ce198030b8c858aca246924ea6a9` |
| `node test/electronSecurity.node.js` | 0 | 32 tests | [security.json](../../dist/pay001-messages01/run-03-security/security.json) SHA `a940ebdbb4bcf3828d8605555891501b2ea9e369062cea16fa81f97478e240b6` |
| `node --check social/payment-inbox.js` | 0 | syntax | [check.json](../../dist/pay001-messages01/run-03-check-inbox/check.json) SHA `59ddcbedb54a42917b90e7640c0efaa9ef9e58182428a9ce5f46df24561233ac` |
| `node --check social/app.js` | 0 | syntax | [check.json](../../dist/pay001-messages01/run-03-check-app/check.json) SHA `7a664739215cd52e17c5cf76b70cd2760d30e57b0149100254c8a512b17dbf78` |
| `node --check test/paymentInbox.electron.js` | 0 | syntax | [check.json](../../dist/pay001-messages01/run-03-check-electron/check.json) SHA `61538361d57c16f938c19aa241893ee2bf8224fcf0f1c0d8813af16c55d53fd3` |

Empty stderr SHA `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

### Falsification and restore

Temporarily replaced `paymentPeer === socialPeer` with `true` in
`identitiesMatch`. `node test/paymentInbox.node.js` exit 1.
[run-04-falsify](../../dist/pay001-messages01/run-04-falsify/) metadata SHA
`bd9eaa71ffc4a3f0d4976b709cfcea06779f03c37c382dc6ce515fd24302e4af`.
Mismatched-identity case: conversation row count `1 !== 0` (request incorrectly
visible). Source restored to SHA-256
`3aa647a76f66c90c7b2929aa67f041b565675f4ea57eb4d7dd4978d1ac841884`.
Restored green: [run-05-restored](../../dist/pay001-messages01/run-05-restored/)
metadata SHA `81c3cab2fcb820359c830d7df7d39a97964f71cda7dbe6c729db3662325925f4`,
exit 0, 54 tests, stdout SHA matches run-03-green.

## Electron

`test/paymentInbox.electron.js` was rewritten for the Messages journey (owned
payment + social loopback fixtures, localStorage API bootstrap/reload, request-only
row, mixed text, automatic cancel, identity mismatch, unavailable). It was not
executed. Known-failing Electron launches were not attempted. Hermes Messages
acceptance remains pending after source review. That execution is not green.

Stop for Codex source review.
