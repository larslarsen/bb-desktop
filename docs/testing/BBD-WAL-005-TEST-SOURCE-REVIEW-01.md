# BBD-WAL-005 Test-Source Review 01

Decision: ACCEPTED FOR EXPECTED-RED EXECUTION

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Production baseline: `14a6818740a1da0641f88a1f91dea3346554dddd`

Reviewer governance HEAD before the drop: `cf1768b7`

## Accepted source identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/walletPay.node.js` | 778 | `18969bd83cc0630c98839b9bc3e01a2cddd32a1723e77e6463eb750e31666315` |
| `test/walletSupervisor.node.js` | 350 | `82d5a7a9e352697bf4fe32871e808bd804201b3ecf85d5a592440f684065a16c` |
| `test/electronSecurity.node.js` | 873 | `cc9bd07a687a07bd5852aa6849e19622e52fc56ab2f8e9e76cc32f9796b5cad8` |
| `test/securityPolicy.node.js` | 2645 | `116dbdf90c2f380b27dfce0bf31cc6155133213990710c7e5dac0b7a50a02e8e` |
| `test/fixtures/wallet-pay/snapshots-v1.json` | 155 | `bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252` |

`git diff --check` is clean and the source actor changed no path outside the five-path
test authorization.

## Review result

The drop reserves observable behavior rather than implementation text for the Pay model:
closed descriptor-safe sanitization, exact capability rows, payer/payee gating, exact
`receiver.fresh` parameters, fixed preview labels, stable error reduction, cloning, and
secret/authority canaries. It exercises the existing supervisor and Electron boundary,
keeps the preload/IPC allowlists closed, and reserves package/workflow/source policy
without adding a dependency.

The high-value assertions are non-vacuous:

- transparent-only Zcash hardware cannot become private Pay eligible;
- Orchard-restored Zcash funds produce `MIGRATION_REQUIRED` for spend while private
  receive remains possible;
- v6/Ironwood/PCZT omissions fail closed;
- a requested ineligible payee account does not fall back to another account;
- broker-private receiver, fee, request, memo, peer, raw artifact, rate, and secret fields
  are absent from the Electron preview; and
- supervisor sync publication must traverse the same sanitizer exported by the Pay model.

The source actor ran no test or build. Passing behavior is not claimed. Hermes alone is
authorized to record the expected-red results and integrate this accepted identity.

Correction 01 replaced only the mismatched-parenthesis assertion at line 310 after the
first Hermes attempt stopped on a syntax error. It made one insertion and one deletion,
kept the file at 778 lines, and left every test input and assertion meaning unchanged.
The corrected hash above supersedes the initial accepted hash for that path.
