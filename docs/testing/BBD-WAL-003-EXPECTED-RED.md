# BBD-WAL-003 Expected-Red Evidence

Timestamp: 2026-08-30T16:10:04-0700 (PDT)
Governance baseline: `HEAD == origin/master == 2069497ecfa6022ce34ace6327c0631aff4e0f8f`

Accepted test paths were verified unchanged:

- `test/fixtures/wallet-broker/transcript-v1.json` — 22 lines — `92702c7f8ae18a383b194142986992888e96e7211e0e6a974945b658e854c3f1`
- `test/walletBrokerProtocol.node.js` — 257 lines — `1397fb5e0833c2b58a53d6a8332914b47e4f00102bab4371ee43fc77d1960fd0`
- `test/walletSupervisor.node.js` — 340 lines — `ee17cf2ecd39c65a4c37821c78e04bfdcb2797d3e2cfe846580961840ce925e7`
- `test/walletPreload.node.js` — 134 lines — `60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e`
- `test/electronSecurity.node.js` — 797 lines — `135479b319bfca2d97ce7ca412da04afe79332ebb92e39fcb4b00ef3702b0d55`
- `test/securityPolicy.node.js` — 1,574 lines — `5ebb4cfa7fe91b073cdf8f6769c9e887e2bdcae17ef6b29dd3ef059c8daf9a83`

## Authorized commands

1. `node test/walletBrokerProtocol.node.js` — exit 1; expected `MODULE_NOT_FOUND`
   for `../wallet-broker/protocol` after fixture preflight.
2. `node test/walletSupervisor.node.js` — exit 1; expected `MODULE_NOT_FOUND` for
   `../wallet-broker/supervisor`.
3. `node test/walletPreload.node.js` — exit 1; expected missing
   `/home/lars/OpenBazaar/bb-desktop/wallet-preload.js`.
4. `node test/electronSecurity.node.js` — exit 1; all 19 results emitted, with 13
   inherited tests `ok` and six expected preload/IPC integration tests `not ok`.
5. `node test/securityPolicy.node.js` — exit 1; 54 inherited tests `ok` and four
   expected wallet-broker boundary tests `not ok`, only because package, workflow,
   CI, and source-policy production is absent.

No canary appeared in output. No production path ran or changed, and no out-of-scope
action occurred. The six accepted test paths remain the only unstaged worktree paths.
