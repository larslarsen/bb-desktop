# BBD-WAL-002 Correction 06 Expected-Red Evidence

Timestamp: 2026-08-30T15:07:31-0700 (PDT)
Authorization baseline `HEAD == origin/master`: `6cda4bceb10cf926df1db9f0f2f147cf79a9c0a9`

Protected hashes verified before execution:

- `test/walletContract.node.js`: 1,803 lines — `43830b1caec19904d23b400974c77c1edbebe32b4927b2f31ee4279611a46dbf`
- `test/fixtures/wallet-contract/golden-v1.json`: `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/electronSecurity.node.js`: `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- `test/securityPolicy.node.js`: `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`
- `package-lock.json`: `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`

## Authorized command

`node test/walletContract.node.js` — exit status 1.

Exactly 45 previously accepted tests reported `ok`; exactly these three appended
tests reported `not ok`:

- `recovery authority: durable signed_unverified restore requires crash recovery and fresh confirmation` — first cause: expected failure returned `ok: true`.
- `capabilities: watch-only receive requires exact synthetic consensus compatibility` — first cause: incompatible watch-only capability remained eligible (`true !== false`).
- `exceptions: untrusted dependency error codes normalize without leaking or retaining locks` — first cause: untrusted prepare error code was not normalized.

No secret-canary value appeared in command output. The pre-existing worktree scope was
unchanged; only this named evidence file was added. No tests, fixtures, production,
wiring, package, workflow, policy, or lockfile path was edited, staged, or committed.
