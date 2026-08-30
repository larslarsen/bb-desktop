# BBD-WAL-002 Expected-Red Evidence

Timestamp: 2026-08-30T14:25:34-0700 (PDT)
Governance HEAD before evidence: `5b2e5f54c9e9abdb765ef364744b0c7da6c4e765`

Accepted test source was verified before execution. The exact paths, line counts,
and SHA-256 values were:

- `test/fixtures/wallet-contract/golden-v1.json` — 231 lines — `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/walletContract.node.js` — 1,344 lines — `a814bf327345dbdde276343fc40ff6fd8ca770569b12afc0860c664a8c99b7d9`
- `test/electronSecurity.node.js` — 639 lines — `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- `test/securityPolicy.node.js` — 1,396 lines — `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`

`git diff --check` passed. The worktree before evidence contained only those four
exact test-source paths.

## Authorized commands

1. `node test/walletContract.node.js` — exit status 1. The fixture was read, parsed,
   inventoried, byte-checked, and independently hashed before the deliberately late
   implementation import; the only failure was `MODULE_NOT_FOUND` for `../wallet-contract`.
2. `node test/electronSecurity.node.js` — exit status 1. Exactly 13 inherited tests
   reported `ok`; the appended wallet test reported `not ok` because
   `wallet-contract/canonical.js` is missing.
3. `node test/securityPolicy.node.js` — exit status 1. Exactly 50 inherited tests
   reported `ok`; the four appended wallet tests reported `not ok` only for the absent
   package script/build contract, policy exports/source checker, workflow filters, and
   routine CI command.

No production, dependency, lockfile, build, formatter, scanner, audit, install,
network, wallet, node daemon, device, USB/HID, or other repository action occurred.
