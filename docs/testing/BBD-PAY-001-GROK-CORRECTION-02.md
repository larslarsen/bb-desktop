# BBD-PAY-001 Grok correction 02

Actor: Grok Build 4.6 High (xAI), owner-relayed. Reviewer: Codex.
Handoff: [GROK_BUILD_BBD_PAY_001_CORRECTION_02.md](../handoff/GROK_BUILD_BBD_PAY_001_CORRECTION_02.md).
Review: [BBD-PAY-001-SOURCE-REVIEW-02.md](BBD-PAY-001-SOURCE-REVIEW-02.md).
Correction 01 is closed for execution. Prior reports and captures in
[BBD-PAY-001-GROK-CORRECTION-01.md](BBD-PAY-001-GROK-CORRECTION-01.md),
[BBD-PAY-001-GROK-01.md](BBD-PAY-001-GROK-01.md), `dist/pay001-grok-correction01/`,
and `dist/pay001-grok/` were not overwritten.

Desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
The eight correction-01 changed hashes and five frozen drop hashes matched before
this pass except the five writable paths authorized here. The six original frozen
input hashes still match. Capture filesystem `dist/pay001-grok-correction02` is
`ext2/ext3`. Unrelated dirty work was left untouched.

Tool identity: Node v22.23.1, executable SHA-256
`93956de2e59480474a7b46571da1651180b1a050cdf32641ebec4ce6e478e068`.
Electron dist SHA-256
`180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`.

This actor has **no** `require_escalated` (or equivalent) tool capability. The
isolated UI command was therefore run once in the ordinary tool shell. That is
an execution-context fact, not a source/test result.

## Review-02 map

| Review item | Changed source | Regression / evidence |
| --- | --- | --- |
| 1 daemon UTC `received_at` | `wallet-pay/inbox-client.js` `RECEIVED_AT` / `parseReceivedAt`: optional `.[0-9]{1,9}` before `Z`; calendar/time checked without the fraction; first three fraction digits padded to JS ms; ISO prefix check independent of fraction | whole-second, `.1Z`, `.123Z`, `.123456Z`, `.123456789Z`, leap-day positive, missing/excess fraction, `+00:00`, non-leap 29 Feb, Feb 31. Red: `.1Z` `'invalid' !== 'ready'` vs correction-01 parser. Canonical decoder untouched. Electron fixture default `.123456789Z` |
| 2 wrap, do not clip | `social/styles.css` inbox only: list/request/status no longer hide overflow; memo/meta/subtitle/amount/details wrap with `overflow-wrap: anywhere` and `word-break: break-word` | harness `layoutOk` requires `scrollWidth <= clientWidth + 1`, child bounds inside parent, no sibling overlap; Requests tab, header/actions, every fixture row, requested/cancelled/expired, full peer id, 180-char memo; await nonbusy after picker cancel; `getLastWebPreferences` sandbox/contextIsolation/nodeIntegration/webSecurity. **UI smoke failed** (see Electron) |
| 3 remaining focused regressions | tests only for these cases; HTTP production already had `maxHeaderSize: 16384` and close-before-end from correction 01, so no extra HTTP repair | header flood → `unavailable`; truncated `Content-Length` body → not `ready`; padded envelope below/at 4 MiB `ready`, +1 `too_large`; get after old completion while newer pending, `finishes.length === 2`; `fs.promises.open` `entered` before dispose; selected-root symlink; get **and** connect sender/frame/URL/arity rejection plus cloned DTO mutation. Timeouts/mocks/servers cleared in `finally` |
| 4 out-of-order renderer | test-only isolated `#staleBridgeRoot` mounts real `BitBookPaymentInbox` with two deferred fixture promises; resolve newer then older | not executed: zygote died before renderer. Screenshot rows remain specified as real main-only client + owned loopback; stale-bridge is separate and is not a screenshot source |

Signed canonical timestamp codec and `decodeSignedObject` were not modified.
Instance-binding falsification and preload tests were not rerun.

## Hashes after correction

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.node.js | 1190 | `9c166540f59498134e544fa3ead3e71dd340316e0c9bdaf44aeb3179c85c2b2b` |
| test/paymentInbox.electron.js | 487 | `a19ec5671eedd8a1b77876b3d590a2606892ba2910bb3e646fda7ecac38a3893` |
| test/electronSecurity.node.js | 2195 | `22962ae4a3259290d9d64250f3cbac5bcf56ce8cf0ccb0764842ec90664c094d` |
| wallet-pay/inbox-client.js | 609 | `a41463f491fdf1aa317d972f69438b62c273ccb59ad8e2a99444fabe563de8e5` |
| social/styles.css | 985 | `4673ece3b62454458c0cc2f538a120073b8d9e0d73f83834c1d63716c5a31b31` |

Unchanged from the reviewed correction-01 drop:

| Path | SHA-256 |
| --- | --- |
| test/fixtures/payment-inbox/records-v1.json | `76f5a86a7337c3733b36696ad212c976ff8a7c2c848b7b1cdda0e1b968ef0650` |
| social-main.js | `298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6` |
| social/payment-inbox.js | `caac07903a2f97ad4da1f1584f10adef0e6954aafd1f287f897ac44573617db3` |
| test/walletPreload.node.js | `373801529b001ea700872d4330d09b1c31c4bc98cc6e1ae0824bcbabf1cffa6e` |
| test/walletPay.node.js | `ba18dd463e552cff33f104fdd80c403e9dac7f370af84f8db8906d5114fc1c12` |
| wallet-preload.js | `6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025` |
| social/app.js | `699d66977de0793269dcf045f0442275842c3fb9300cd7a9238035cb5fd8147a` |
| social/index.html | `a9f3a2e8a08f420b9ae04b9b339211d48511c8a4607bdce3ac1882e4f9691774` |

Original six frozen inputs (still match the inbox handoff table):

| Path | SHA-256 |
| --- | --- |
| wallet-contract/canonical.js | `32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761` |
| wallet-pay/model.js | `acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e` |
| package.json | `76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5` |
| package-lock.json | `0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8` |
| scripts/security-policy.js | `0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d` |
| test/securityPolicy.node.js | `ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c` |

## Commands (cwd `bb-desktop`)

Capture driver: [capture-run.js](../../dist/pay001-grok-correction02/capture-run.js)
SHA-256 `43e36284368e71c488f586b80373e0bfc5e17c9fc70ec272aaea2c2327c6eb53`.
It waits for stdout/stderr `end` before hashing and writing metadata.

### Correction red (new tests vs correction-01 production)

`node test/paymentInbox.node.js` under
[run-01-red](../../dist/pay001-grok-correction02/run-01-red/).

| Field | Value |
| --- | --- |
| metadata | [inbox.json](../../dist/pay001-grok-correction02/run-01-red/inbox.json) SHA `c7e22371aae174e84cb67c374fb48a16936a911e76eb503bdc368279c92d535e` |
| argv | `node test/paymentInbox.node.js` |
| started_utc | `2026-09-15T21:42:01.174Z` |
| ended_utc | `2026-09-15T21:42:06.390Z` |
| exit_code | 1 |
| signal | null |
| stdout | [inbox.stdout.log](../../dist/pay001-grok-correction02/run-01-red/inbox.stdout.log) SHA `19497a355cfa1b0f965ec43a535a721020d1f3c97cf667685fe07081ec914cf9` |
| stderr | [inbox.stderr.log](../../dist/pay001-grok-correction02/run-01-red/inbox.stderr.log) SHA `a058d2b2bda121a79c0cc856489c8ea2b98e42f2bd5c300715f6e5574e8fa9e2` |

Observed failure: `fractional received_at values and calendar negatives`,
`2026-08-30T12:01:00.1Z` `'invalid' !== 'ready'`.

Already green against correction-01 production (no extra HTTP source repair):
HTTP header limit / truncated body / padded 4 MiB; coalesced get with no third
fetch; descriptor I/O entry before dispose; selected-root symlink. Parser
production was changed only after this red.

### Focused green

| Command | exit | tests | metadata |
| --- | ---: | ---: | --- |
| `node test/paymentInbox.node.js` | 0 | 38 | [inbox.json](../../dist/pay001-grok-correction02/run-02-green/inbox.json) SHA `0a476442dede1d6d3a27ba9cb91ddd1fd899ffa6fc3e4fbe3c51d8ecb3538dbd` |
| `node test/electronSecurity.node.js` | 0 | 32 | [security.json](../../dist/pay001-grok-correction02/run-02-green/security.json) SHA `f8a157621f339436ae3180c82f9f4b26d343bca4685fddfe2a1adce14a76e093` |
| `node test/walletPay.node.js` | 0 | 58 | [walletpay.json](../../dist/pay001-grok-correction02/run-02-green/walletpay.json) SHA `f2c2af7e690a0102f1b6a55202f5abd690d975c49ce1e58544bbd233b87683b7` |

Inbox stdout SHA `65f121ee9d4468d23129fd5b9a3a782b2daf3fa6ff7702991891baf854d600c5`.
Security stdout SHA `c30aca54576128413acdc7910fdba33070dc1a774b1f481e316604c2814522df`.
Wallet/inbox stdout SHA `3c56b660bbc3312c2835286cf1c74302db99eabfbfe06d741cb9bf7c01742dd6`.
All three stderr SHA `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` (empty).
Zero failures/skips. Preload was not rerun.

`walletPay.node.js` already includes the inbox suite (20 Pay + 38 inbox = 58).
That is the combined wallet/inbox green; a second identical run is recorded below.

### Final combined wallet/inbox green

`node test/walletPay.node.js` under
[run-03-combined](../../dist/pay001-grok-correction02/run-03-combined/).

| Field | Value |
| --- | --- |
| metadata | [combined.json](../../dist/pay001-grok-correction02/run-03-combined/combined.json) SHA `8d776bc9e45b02f3110c8bc20d8385fabb907f0764b4f41c721ab149b16bbfdc` |
| started_utc | `2026-09-15T21:49:54.330Z` |
| ended_utc | `2026-09-15T21:50:02.431Z` |
| exit_code | 0 |
| tests | 58 |
| stdout SHA | `3c56b660bbc3312c2835286cf1c74302db99eabfbfe06d741cb9bf7c01742dd6` |
| stderr SHA | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

### Electron (review item 2/4 UI)

Bounded runner environment: `BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok-correction02/ui-01`.
`xvfb-run -a` assigned an isolated `DISPLAY`. Full environment was not dumped.

```
BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok-correction02/ui-01
xvfb-run -a node_modules/.bin/electron --disable-setuid-sandbox test/paymentInbox.electron.js
```

| Field | Value |
| --- | --- |
| metadata | [smoke.json](../../dist/pay001-grok-correction02/run-04-electron/smoke.json) SHA `f48a510bfbeb3ae742ff5337e35b4477f29926fa843ba324b0a3416ca4c6c104` |
| argv | `xvfb-run -a node_modules/.bin/electron --disable-setuid-sandbox test/paymentInbox.electron.js` |
| started_utc | `2026-09-15T21:50:14.164Z` |
| ended_utc | `2026-09-15T21:50:14.436Z` |
| exit_code | 1 (`xvfb-run` status; child SIGTRAP) |
| stdout SHA | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| stderr | [smoke.stderr.log](../../dist/pay001-grok-correction02/run-04-electron/smoke.stderr.log) SHA `3ed7f1eb033134cad32b6d50f5fec453441f958195a317d7fc31b915cc50b641` |

Chromium zygote: **No usable sandbox** (AppArmor unprivileged user-namespace
restriction after the SUID helper is unusable). `--no-sandbox` was not used.
The command was not retried.

Host facts recorded without policy changes: `chrome-sandbox` mode `0755` owner
not root; `kernel.unprivileged_userns_clone = 1`;
`kernel.apparmor_restrict_unprivileged_userns = 1`. The actor process itself
had a single `NSpid`, `Seccomp 0`, `NoNewPrivs 0` (not a nested bwrap jail).
That still could not create Chromium's sandbox namespaces.

`dist/pay001-grok-correction02/ui-01/` was **not created** (process died in the
zygote, before the harness `mkdir`).

| Screenshot / proof | Status |
| --- | --- |
| fixture-nonempty-1180x780.png | **unavailable** |
| fixture-nonempty-860x620.png | **unavailable** |
| fixture-empty-1180x780.png | **unavailable** |
| fixture-unavailable-860x620.png | **unavailable** |
| sandbox-status.json (Seccomp 2 / NoNewPrivs 1 / extra NSpid) | **unavailable** |
| BrowserWindow webPreferences from a live window | **unavailable** |
| isolated stale-bridge DOM assertion | **unavailable** |

A partial capture is not a passed smoke. This remains an environment/tool
limitation, distinct from the Node source/test greens above.

## Gaps

- UI screenshots, OS sandbox PID proof, live webPreferences, and the isolated
  out-of-order renderer assertion: unavailable on this host without a
  `require_escalated` (or equivalent) execution context that can create
  Chromium sandbox namespaces. Not retried. `--no-sandbox` not used.
- Hermes acceptance scanners: not run.

Stop for Codex source review.
