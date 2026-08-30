# BBD-WAL-002 Correction Expected-Red Evidence

Timestamp: 2026-08-30T14:57:31-0700 (PDT)
Baseline `HEAD` and `origin/master`: `d41051afc26aa8688c0881a35984c003788f7268`

Verified before execution:

- `test/walletContract.node.js`: 1,697 lines, SHA-256 `3e51281d16da7eec4a178eeb799ec23e2854206a096ed741cba920fc35825ee9`
- `test/fixtures/wallet-contract/golden-v1.json`: SHA-256 `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/electronSecurity.node.js`: SHA-256 `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- `test/securityPolicy.node.js`: SHA-256 `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`
- `package-lock.json`: SHA-256 `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`

## Authorized command

`node test/walletContract.node.js` — exit status 1.

Exactly 38 previously accepted wallet tests reported `ok`; exactly seven appended
correction tests reported `not ok`:

- `binding: prepared reviews are recomputed and bound to the selected request and account` — first cause: expected `INTENT_MISMATCH` failure returned `ok: true`.
- `capabilities: account, signer, adapter, and exact synthetic protocol pins cannot be substituted` — first cause: an exact protocol-pin substitution incorrectly remained spendable (`true !== false`).
- `recovery locking: crash_recovery retains ownership and restored confirmation acquires it` — first cause: expected `ACCOUNT_BUSY` failure returned `ok: true`.
- `recovery terminal: cancellation and expiry release crash_recovery account locks` — first cause: expected terminal recovery transition returned `ok: false`.
- `recovery restart: repeated recovery crash is inert and durable verified restore cannot broadcast` — first cause: repeated crash in `crash_recovery` did not return `ok: true`.
- `exceptions: injected status, prepare, signer, verify, and broadcast throws return closed failures` — first cause: injected status exception escaped instead of returning a closed failure.
- `secrets: sanitization validates allowlisted values without invoking accessors` — first cause: sanitization invoked an allowlisted getter (`1 !== 0`).

The command output contained no secret-canary value. No test, fixture, production,
wiring, package, workflow, policy, or lockfile path was changed by this evidence
operation; the pre-existing worktree scope remained unchanged, with only this named
evidence file added.
