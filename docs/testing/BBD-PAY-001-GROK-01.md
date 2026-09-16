# BBD-PAY-001 Grok inbox 01

Actor: Grok Build 4.6 High (xAI), owner-relayed. Reviewer: Codex.
Handoff: [GROK_BUILD_BBD_PAY_001_INBOX_01.md](../handoff/GROK_BUILD_BBD_PAY_001_INBOX_01.md).
Ticket: [BBD-PAY-001](../../tickets/BBD-PAY-001.md).

This is Grok's focused-source completion record. It is not Hermes acceptance.
No Git mutation, dependency install, real daemon/wallet, or user-data access.

## Baseline

Desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`. Staging index empty.
Starting working-file SHA-256 values matched the handoff table. Frozen inputs
matched. New ticket paths were absent. Frozen daemon
`../bb-go/modern/localclient/server.go` SHA-256
`5b00cc6e10694a5fbaccda27637751b866cbc6bb1f0bfd98f05dc94479cc7a93`.

Filesystem: `stat -f -c '%T'` on the repository root and `dist/pay001-grok` was
`ext2/ext3`. Evidence is under `dist/pay001-grok/` only.

Tool identity captured in this drop: Node.js v22.23.1; Electron 44.0.0 from
`node_modules/.bin/electron`; `xvfb-run` at `/usr/bin/xvfb-run`. Exact Electron
binary SHA-256: **unavailable** (not hashed).

## What landed

A **Requests** tab and Payment requests screen. Main-process
`wallet-pay/inbox-client.js` reads `local-client/connection.json` (Linux 0700/0600,
O_NOFOLLOW), GET `/v1/payment/records` with Bearer + instance headers, validates
records through frozen `decodeSignedObject`, and returns a closed DTO. Renderer
`social/payment-inbox.js` has no credentials or transport. Preload adds
no-argument `getPaymentInbox` and `connectPaymentInbox` (eight methods). Main
registers `payment:inbox:get` and `payment:inbox:connect` (seven handlers).

Screenshot data would have been in-process HTTP fixtures, not a live owner daemon.

## Changed paths (after focused green / restored source)

| Path | Lines | Bytes | SHA-256 |
| --- | ---: | ---: | --- |
| test/paymentInbox.node.js | 551 | 21204 | `4bca4f333e30708cbc3e8360d8a9c311aa73b88a38feb06dddff8a95657a9dcd` |
| test/paymentInbox.electron.js | 227 | 8448 | `f4d1a8d1af1b34742ffb1ebb14fdd0ebe522072e04434eb9836b027f0aed297d` |
| test/fixtures/payment-inbox/records-v1.json | 56 | 4157 | `c9d9cdd6b92b137a649e8a7bdf3bbf21949111ca5e352011659bac0acfefa0e2` |
| test/walletPreload.node.js | 140 | 6692 | `373801529b001ea700872d4330d09b1c31c4bc98cc6e1ae0824bcbabf1cffa6e` |
| test/electronSecurity.node.js | 2098 | 77748 | `6c43869786ac3f4c3d0112835b94a7ab9d3a2690eaeb87ab16c6a05b60f0e8e1` |
| test/walletPay.node.js | 790 | 33869 | `ba18dd463e552cff33f104fdd80c403e9dac7f370af84f8db8906d5114fc1c12` |
| wallet-pay/inbox-client.js | 657 | 20213 | `97c311f95f08f40e42b74ef278ac772c22bbb3ab0394741c04968fe94964353d` |
| social/payment-inbox.js | 151 | 5376 | `4df5059eb2474c58b8d1870704f931c418c2c9b140b0758e007272bddd022fe6` |
| social-main.js | 322 | 11348 | `d5f85a89c55e132e83fafbc478ad6ac87d013ec5b14ba54011b1243d3fa55eb7` |
| wallet-preload.js | 72 | 2774 | `6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025` |
| social/app.js | 765 | 27300 | `699d66977de0793269dcf045f0442275842c3fb9300cd7a9238035cb5fd8147a` |
| social/index.html | 192 | 8156 | `a9f3a2e8a08f420b9ae04b9b339211d48511c8a4607bdce3ac1882e4f9691774` |
| social/styles.css | 971 | 15015 | `c42dd41d5d403eee40dfcba6cee2bccae2be11fcec8f830d9b9ed7eca558ebc1` |

Starting hashes (before this drop) for files that existed:

| Path | Before SHA-256 |
| --- | --- |
| social-main.js | `d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a` |
| wallet-preload.js | `3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df` |
| social/app.js | `3eff1370db10731a93b618190b82852a894e781bea5b28a47bddb7854285da10` |
| social/index.html | `0c30c232b06ae92019b441fa8a51b9817b7ade1346a7ddae974dde3be36ac931` |
| social/styles.css | `f0c3d15349846b7849dc12c430440520d99c091630ffea71567f9bcb5a5d1c7d` |
| test/walletPreload.node.js | `60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e` |
| test/electronSecurity.node.js | `d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482` |
| test/walletPay.node.js | `18969bd83cc0630c98839b9bc3e01a2cddd32a1723e77e6463eb750e31666315` |

## Commands (cwd `bb-desktop`)

Environment for Electron: `BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok`.
Node tests used the process default temp for tiny owned descriptor trees.

### Targeted red

A missing-module red log was **not retained**. Tests and `inbox-client.js` were
authored in the same pass; the first `node test/paymentInbox.node.js` execution
already had production present and passed (17 tests). That is a process gap.

### Focused green

```
node test/paymentInbox.node.js
```
exit 0. 17 tests. Log: [05-paymentInbox-green.log](../../dist/pay001-grok/05-paymentInbox-green.log)
SHA-256 `03e4a629582e9ea5d55caac3a8128f8a6490c83ec365470abaf8c34894a5a97f`.

```
node test/walletPreload.node.js
```
exit 0. 6 tests. Log: [06-walletPreload-green.log](../../dist/pay001-grok/06-walletPreload-green.log)
SHA-256 `448821de669b9f6796e2056dea9b24225b40720b680379aec555e3e86f731b12`.

```
node test/electronSecurity.node.js
```
exit 0. 31 tests. Log: [07-electronSecurity-green.log](../../dist/pay001-grok/07-electronSecurity-green.log)
SHA-256 `7a3440d3ffafd41767378255ce213d703691434cdc040c1b0373841097238758`.

```
node test/walletPay.node.js
```
exit 0. 37 tests (prior Pay tests plus the 17 inbox tests). Log:
[08-walletPay-green.log](../../dist/pay001-grok/08-walletPay-green.log)
SHA-256 `0adb552e4f93632ef90f73861fcadfc0df34de1603aa16d69967728d8664f1a5`.

Zero failures/skips on these four commands.

### Instance-binding falsification

Pre-mutation `wallet-pay/inbox-client.js` SHA-256
`97c311f95f08f40e42b74ef278ac772c22bbb3ab0394741c04968fe94964353d`.
Temporarily set `instanceBindingEnabled()` to return false.

```
node test/paymentInbox.node.js
```
exit 1. Log: [01-falsify-wrong-instance.log](../../dist/pay001-grok/01-falsify-wrong-instance.log)
SHA-256 `17a732cc743bd303d41fece9ee2770187cdf45a32a60a98af1d22c16be26a9a8`.

`wrong instance is invalid and credentials stay off the reply` failed:
`AssertionError: 'ready' !== 'invalid'`.

Source restored. Post-restore SHA-256 identical to pre-mutation
`97c311f95f08f40e42b74ef278ac772c22bbb3ab0394741c04968fe94964353d`.

```
node test/paymentInbox.node.js
```
exit 0. 17 tests. Log: [02-restored-green.log](../../dist/pay001-grok/02-restored-green.log)
SHA-256 `03e4a629582e9ea5d55caac3a8128f8a6490c83ec365470abaf8c34894a5a97f`.

Mutated source was not left in the tree.

### Electron fixture smoke

```
BBD_PAY001_ARTIFACT_DIR=dist/pay001-grok xvfb-run -a node_modules/.bin/electron test/paymentInbox.electron.js
```

exit 1 (SIGTRAP). `--no-sandbox` was **not** used.

Logs:
- [03-electron-smoke.log](../../dist/pay001-grok/03-electron-smoke.log) SHA-256 `4d65165d6b34c42be119d129bfc40fc0128b63d6407a7da9666a847989bbb1d8`
- [04-electron-smoke.log](../../dist/pay001-grok/04-electron-smoke.log) SHA-256 `fd289ffb0e40b132ba3c48a5ad13136dccac4b43eb44ac6ebe18655cc4ff5dc3`

Chromium aborted because `node_modules/electron/dist/chrome-sandbox` exists but is
not root-owned mode 4755. `kernel.unprivileged_userns_clone = 1` did not help while
that helper binary is present. Screenshots were **not captured**.

| Screenshot | Status |
| --- | --- |
| fixture-nonempty-1180x780.png | **unavailable** |
| fixture-nonempty-860x620.png | **unavailable** |
| fixture-empty-1180x780.png | **unavailable** |
| fixture-unavailable-860x620.png | **unavailable** |

No PNG files exist under `dist/pay001-grok/`. Dimensions/hashes: **unavailable**.

## Gaps

- Electron UI screenshots at both window sizes: unavailable (sandbox helper).
- First missing-module red: not retained.
- Broader acceptance (`npm run test:social`, `npm run build`, gitleaks, audit):
  not run (Hermes later).
- Real app restart to load new main/preload: not performed (not authorized).

Stop for Codex source review.
