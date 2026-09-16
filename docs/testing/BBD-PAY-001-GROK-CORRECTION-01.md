# BBD-PAY-001 Grok correction 01

Actor: Grok Build 4.6 High (xAI), owner-relayed. Reviewer: Codex.
Handoff: [GROK_BUILD_BBD_PAY_001_CORRECTION_01.md](../handoff/GROK_BUILD_BBD_PAY_001_CORRECTION_01.md).
Prior drop evidence in [BBD-PAY-001-GROK-01.md](BBD-PAY-001-GROK-01.md) and
`dist/pay001-grok/` was not overwritten.

Desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Starting thirteen drop hashes and six frozen inputs matched the prior report.
Capture filesystem `dist/pay001-grok-correction01` is `ext2/ext3`.

Tool identity: Node v22.23.1, executable SHA-256
`93956de2e59480474a7b46571da1651180b1a050cdf32641ebec4ce6e478e068`.
Electron dist SHA-256
`180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`.

## R1–R6 map

| Item | Changed source | Regression / evidence |
| --- | --- | --- |
| R1 owned HTTP | `wallet-pay/inbox-client.js` `fetchRecords`: `maxHeaderSize: 16384`, one finish/cleanup, destroy req/res, 4 MiB success, 256-byte 503 then unavailable on excess | streaming 302/500; gzip encoding; oversized 503; bounded TOO_LARGE |
| R2 generation/picker/lifecycle | client `stale()` after descriptor I/O; `pendingGet === pending`; pickerBusy ends before getInbox; picker throw → unavailable DTO; `social-main.js` dispose only in `approveNormalQuit` and window `closed`; renderer `seq` | dispose during descriptor read; stale completion; concurrent/failed picker; security dispose vs rejected shutdown |
| R3 descriptor types | lstat reject non-regular; `O_RDONLY\|O_NOFOLLOW\|O_NONBLOCK`; fstat; extra-byte growth; close in `finally` | FIFO `mkfifo` (red hung; green prompt, zero HTTP); padded JSON below/at/above |
| R4 records | keep outbound until cancel check; wrong-party inbound and signer/direction conflict fail closed; RFC3339 calendar `received_at`; signature base64 ≤1024 | wrong-party; conflicting cancel; Feb 31; oversize signature; independent paid/expired hashes |
| R5 IPC arity | `paymentHandler(event, ...payload)` rejects nonempty payload including `[undefined]`; security mocks factory with counters | trusted get/connect increment; untrusted/extra args do not |
| R6 UI smoke | `test/paymentInbox.electron.js` hold-ready, webRequest cancel http(s)/ws, `app.setPath` only, keyboard Enter on details | **smoke failed** — no screenshots (see Electron) |

Product `instanceBindingEnabled` export/toggle was removed. Falsification edited the real `instance_id` equality line and restored it.

## Hashes after correction

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.node.js | 994 | `8808594d60dc7a4570deebf72add2e48ab640031af27daa05e422cd0207ca5e0` |
| test/paymentInbox.electron.js | 371 | `082e2bb02f460852bb12cab35b36298222ec6afc1008b15303e3a277c47f2641` |
| test/fixtures/payment-inbox/records-v1.json | 81 | `76f5a86a7337c3733b36696ad212c976ff8a7c2c848b7b1cdda0e1b968ef0650` |
| test/electronSecurity.node.js | 2177 | `fbcdebab0c2a7dc5bf20ccd63d6c8ca203ee9745069e0c033bd5f1ab9c23141d` |
| wallet-pay/inbox-client.js | 599 | `ab9abf92423bb481977197b8281560773a5213657b4bde10b92c92925552418a` |
| social-main.js | 322 | `298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6` |
| social/payment-inbox.js | 163 | `caac07903a2f97ad4da1f1584f10adef0e6954aafd1f287f897ac44573617db3` |
| social/styles.css | 983 | `15bbe3549b426059330ddd44fe3830d66233bd5240184558caf0078a17ade467` |

Frozen at the reviewed drop (unchanged this pass):

| Path | SHA-256 |
| --- | --- |
| test/walletPreload.node.js | `373801529b001ea700872d4330d09b1c31c4bc98cc6e1ae0824bcbabf1cffa6e` |
| test/walletPay.node.js | `ba18dd463e552cff33f104fdd80c403e9dac7f370af84f8db8906d5114fc1c12` |
| wallet-preload.js | `6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025` |
| social/app.js | `699d66977de0793269dcf045f0442275842c3fb9300cd7a9238035cb5fd8147a` |
| social/index.html | `a9f3a2e8a08f420b9ae04b9b339211d48511c8a4607bdce3ac1882e4f9691774` |

## Commands (cwd `bb-desktop`)

Capture driver: [capture-run.js](../../dist/pay001-grok-correction01/capture-run.js).

### Correction red (tests vs reviewed production)

`node test/paymentInbox.node.js` under
[run-01-red](../../dist/pay001-grok-correction01/run-01-red/).
Stdout [inbox.stdout.log](../../dist/pay001-grok-correction01/run-01-red/inbox.stdout.log)
SHA-256 `cdbe1b29d3cf4145f9be0845f8a0f465093bb9afcf4f35d8eaa649498e8aea46`.
Stderr SHA-256 `7c3f2846da862f13174da803a53313b031759e034935ef6ab5a9a6abdd188a69`.
JSON metadata: **unavailable** (process was killed after FIFO/streaming hang).

Intended failures observed: FIFO hung; wrong-party inbound stayed `ready`;
`2026-02-31T12:00:00Z` stayed `ready`. This is a new correction red, not the
original missing-module chronology.

### Focused green

| Command | exit | tests | metadata |
| --- | ---: | ---: | --- |
| `node test/paymentInbox.node.js` | 0 | 33 | [inbox.json](../../dist/pay001-grok-correction01/run-03-green/inbox.json) SHA `c70f74bba290d644f5047c9708ef0ddd29fbec33e0b0b3fbcad0e38561ee1bd2` |
| `node test/walletPreload.node.js` | 0 | 6 | [preload.json](../../dist/pay001-grok-correction01/run-03-green/preload.json) SHA `456bdea5ebc17cd9c55fb561894cbd2c949c66c21826598c176764d5b77dead8` |
| `node test/electronSecurity.node.js` | 0 | 32 | [security.json](../../dist/pay001-grok-correction01/run-03-green/security.json) SHA `c0fa5a38786921b68f54dfa1bf25e9c59212971a33476a6709ffdc0c80bdc0bb` |
| `node test/walletPay.node.js` | 0 | 53 | [walletpay.json](../../dist/pay001-grok-correction01/run-03-green/walletpay.json) SHA `b99c8ceea155ce4c628def151f23b936c4b9a0e4f3c497c0bfeccb3cd70c34a6` |

Zero failures/skips on those four greens.

### Falsification

Mutated `body.instance_id !== descriptor.instance_id` to `false && …`.
`node test/paymentInbox.node.js` exit 1.
[run-02-falsify/inbox.json](../../dist/pay001-grok-correction01/run-02-falsify/inbox.json)
SHA `ca1284de0e60047cee611c1a247151aacef6223fdd54d58563ecd20c7b349f7a`.
Wrong-instance assertion: `'ready' !== 'invalid'`.
Source restored; inbox-client hash above is post-restore.

### Electron (R6)

```
BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok-correction01/run-04-electron
xvfb-run -a node_modules/.bin/electron --disable-setuid-sandbox test/paymentInbox.electron.js
```

exit 1, ~0.22s. Metadata
[smoke.json](../../dist/pay001-grok-correction01/run-04-electron/smoke.json)
SHA `2d81313131f19aa153da81a87e6bfa73e058942f7251d9ddf236b9907b18ec8f`.
Stderr SHA `bda894205d3609ed91778e6453f280a97c3089e4b404a9a8bcba9a868e3c85f5`.

Chromium zygote: **No usable sandbox** (AppArmor/user-namespace restriction after
disabling the misconfigured SUID helper). `--no-sandbox` was not used.

| Screenshot | Status |
| --- | --- |
| fixture-nonempty-1180x780.png | **unavailable** |
| fixture-nonempty-860x620.png | **unavailable** |
| fixture-empty-1180x780.png | **unavailable** |
| fixture-unavailable-860x620.png | **unavailable** |

Renderer Seccomp/NoNewPrivs/NSpid proof: **unavailable** (process never started).

## Gaps

- UI screenshots and OS sandbox PID proof: unavailable on this host.
- run-01-red command JSON: unavailable (killed after hang).
- Hermes acceptance scanners: not run.

Stop for Codex source review.
