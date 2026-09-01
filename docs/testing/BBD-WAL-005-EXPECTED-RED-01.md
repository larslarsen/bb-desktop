# BBD-WAL-005 Expected Red 01 — Execution Record (Resumed)

## Environment identity

- Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)
- Provider/model: meituan/longcat-2.0:free
- Node.js v22.23.1
- Working directory: /home/lars/OpenBazaar/bb-desktop
- Governance HEAD before run: 5f5d67c60d297a3b6cefac4bdc0b0eb098e80051

## Accepted source identity (hashes verified)

| Path | Lines | SHA-256 | Match |
| --- | ---: | --- | ---: |
| `test/walletPay.node.js` | 778 | `18969bd83cc0630c98839b9bc3e01a2cddd32a1723e77e6463eb750e31666315` | yes |
| `test/walletSupervisor.node.js` | 350 | `82d5a7a9e352697bf4fe32871e808bd804201b3ecf85d5a592440f684065a16c` | yes |
| `test/electronSecurity.node.js` | 873 | `cc9bd07a687a07bd5852aa6849e19622e52fc56ab2f8e9e76cc32f9796b5cad8` | yes |
| `test/securityPolicy.node.js` | 2645 | `116dbdf90c2f380b27dfce0bf31cc6155133213990710c7e5dac0b7a50a02e8e` | yes |
| `test/fixtures/wallet-pay/snapshots-v1.json` | 155 | `bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252` | yes |

## Path audit

`git diff --name-only HEAD` shows three tracked test files modified; two untracked paths are the authorized new test file and fixture. No production, package, workflow, policy, renderer, dependency, lockfile, ticket, roadmap, current-task, or unlisted path changed.

## Command results

### Command 1 — `node test/walletPay.node.js`

Error: Cannot find module '../wallet-pay/model'
- code: MODULE_NOT_FOUND
- requireStack: [ '/home/lars/OpenBazaar/bb-desktop/test/walletPay.node.js' ]
- fails at require on line 9: `require('../wallet-pay/model')`
- EXIT_CODE=1

Failure identity: WAL-005 production module `wallet-pay/model` absent from the source tree. Expected-red.

### Command 2 — `node test/walletSupervisor.node.js`

Error: Cannot find module '../wallet-pay/model'
- code: MODULE_NOT_FOUND
- requireStack: [ '/home/lars/OpenBazaar/bb-desktop/test/walletSupervisor.node.js' ]
- fails at require on line 6
- EXIT_CODE=1

Failure identity: WAL-005 production module `wallet-pay/model` absent. Expected-red.

### Command 3 — `node test/electronSecurity.node.js`

Error: Cannot find module '../wallet-pay/model'
- code: MODULE_NOT_FOUND
- requireStack: [ '/home/lars/OpenBazaar/bb-desktop/test/electronSecurity.node.js' ]
- fails at require on line 8
- EXIT_CODE=1

Failure identity: WAL-005 production module `wallet-pay/model` absent. Expected-red.

### Command 4 — `node test/securityPolicy.node.js`

Runs 256 assertions; fails 7 security policy assertions that require the WAL-005 production module/package/workflow/source-policy to exist:
- required checker/workflows do not yet exist: `wallet-pay/model.js`
- expected `'wallet-pay/**'` path entry missing from the broker boundary
- expected `'npm run test:wallet-pay'` fragment missing from the wallet contract package command
- WAL-005 Pay model routine CI command resolves to `undefined` instead of `'node test/walletPay.node.js'`
- social workflow omits `wallet-pay/**` path trigger
- WAL-005 Pay model source policy export resolves to `'undefined'` instead of `'function'`
- wallet boundary source policy still expects `'../wallet-pay/model'` and `'../wallet-pay/model.js'` in the broker protocol allowlist
- EXIT_CODE=1

Failure identity: security policy reserves WAL-005 package/workflow/source-policy behavior that the absent production module does not yet satisfy. Expected-red.

## Result

All four commands exit nonzero exclusively because the WAL-005 production module/wiring/policy (`wallet-pay/model`) is absent. No syntax/fixture error, no inherited assertion regression unrelated to WAL-005, no unexpected pass, no hang, no resource leak, no changed hash. Expected-red confirmed. Resume handoff complete.
