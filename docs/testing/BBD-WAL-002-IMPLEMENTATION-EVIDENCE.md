# BBD-WAL-002 Implementation Evidence

Timestamp: 2026-08-30T15:14:59-0700 (PDT)
Acceptance baseline `HEAD == origin/master`: `80895349e1b21f0765c26d86d92ee24a8d73e013`

Final source/test hashes and line counts:

- `wallet-contract/canonical.js` — 373 lines — `32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761`
- `wallet-contract/framing.js` — 89 lines — `805246cad2cc500bdd819e672e816514cc809f8bb44ab86260dab3e9ed682a0d`
- `wallet-contract/model.js` — 86 lines — `08c6528cc29271bb4b939c2de890cb53558f94c2c32d1b74e1a5e91735069d53`
- `wallet-contract/state-machine.js` — 330 lines — `ea070804111d28d336de6fc5371c2837838669e8a989608551067a4a944927ac`
- `wallet-contract/fakes.js` — 122 lines — `3f3ad73a9b051831406ae37dd8db7325340c4763000abf09d03eb885c62c6b74`
- `wallet-contract/index.js` — 18 lines — `5f97043133522cd2b27f5a45b76d7526c4176704736e0da0c5331ada2f065edc`
- `package.json` — 35 lines — `dd991b9aea4c98c5dc668631cb5dba5b950daa439de28e005ae4a57b1ae6d35a`
- `.github/workflows/social.yml` — 135 lines — `53652bae4f0fd2e2d4f871d6b9c94305ea7df4c7474cf30857fe8b3780bf7fa5`
- `.github/workflows/security.yml` — 50 lines — `515f894a11f94aaacbae86615c638fcdfdf8794c26cc46e1fb3bc957fa49bb7a`
- `scripts/security-policy.js` — 1,689 lines — `9e6d6a14251df8740378e5ccf62e611cc26e5f22a04c878276c07fa89ff0751b`
- `test/fixtures/wallet-contract/golden-v1.json` — 231 lines — `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/walletContract.node.js` — 1,803 lines — `43830b1caec19904d23b400974c77c1edbebe32b4927b2f31ee4279611a46dbf`
- `test/electronSecurity.node.js` — 639 lines — `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- `test/securityPolicy.node.js` — 1,396 lines — `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`
- `package-lock.json` unchanged — `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`

## Acceptance commands

In order, all exited 0:

1. `npm run build` — syntax/build checks passed.
2. `npm test` — social suite passed; wallet 48, Electron security 14, and policy 54 passed.
3. `node scripts/security-policy.js` — security policy checks passed.
4. `npm audit --audit-level=low` — found 0 vulnerabilities.

## Temporary falsifications

Each mutation was applied with `apply_patch`, tested only with
`node test/walletContract.node.js`, failed nonzero for the intended reason, and was
restored with `apply_patch` before the next mutation. Each restore passed `git diff --check`.

1. Fake verify mismatch returned `{ ok: true }`: exit 1; authoritative mutation
   rejection failed (and the related recovered-artifact mismatch rejection also failed).
   Restored `fakes.js` SHA-256: `3f3ad73a9b051831406ae37dd8db7325340c4763000abf09d03eb885c62c6b74`.
2. Vendor `trezor` shortcut returned private spend success: exit 1; transparent-Trezor
   capability protection failed. Restored `model.js` SHA-256:
   `08c6528cc29271bb4b939c2de890cb53558f94c2c32d1b74e1a5e91735069d53`.
3. Fake broadcast returned `{ ok: true, funds_moved: true }`: exit 1; inert-fake
   broadcast assertion failed. Restored `fakes.js` SHA-256:
   `3f3ad73a9b051831406ae37dd8db7325340c4763000abf09d03eb885c62c6b74`.

No secret canary appeared. No dependency or lockfile change, package artifact,
resource leak, real wallet/network/device activity, or out-of-scope change occurred.
Only the 15 paths explicitly staged for this implementation commit are included.
