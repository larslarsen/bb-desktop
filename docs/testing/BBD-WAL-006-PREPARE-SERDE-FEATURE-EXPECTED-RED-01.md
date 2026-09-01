# BBD-WAL-006 Prepare Serde Feature Expected Red 01

Jr Dev — Hermes: expected-red evidence integration for the already-observed 68/7 red.

## Protected identities

- Hermes version: `0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)`
- Provider: `nous`
- Model: `meituan/longcat-2.0:free`
- Protected HEAD: `978906aa2699ac0b72111e28817f702061257b46`
- origin/master: `978906aa2699ac0b72111e28817f702061257b46`
- Source baseline: `432e69c0443dd5233609d578b43d5a43d83d2c3d`
- Integration commit: `4be931150583876fabadf5a6ffb52021c791fdb3`
- Test source: `test/securityPolicy.node.js`, 2,525 lines, `2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169`
- Frozen production: `wallet-broker/src/zec/prepare.rs` (untracked, not staged)
- Diff inventory: exactly `test/securityPolicy.node.js`, 5 insertions, 5 deletions
- `git diff --check`: clean, no warnings

## Sole acceptance command

```text
npm test
```

The test script is `npm run test:social && npm run test:security && npm run test:wallet && npm run test:wallet-broker`. The security policy portion (`npm run test:security`) executes `test/securityPolicy.node.js`.

## Result

- Exit code: **1**
- Final line: `7 security policy test(s) failed`
- Tests passed (ok): **68**
- Tests failed (not ok): **7**
- Total named tests: **75**
- Test file lines: **2,525**

## Exact seven failures

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`
5. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
6. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
7. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

## New manifest-feature failure

Test #4 (`WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`) is the newly added failure. It rejects the still-old manifest state because the test now requires the `serde` feature on the `zcash_client_sqlite` pin, while the committed `Cargo.toml` has not yet been updated to include it.

## Persistent inventory-policy failure

Test #6 (`WAL-006 requires the exact bounded Phase-C ZEC production inventory`) remains red because `scripts/security-policy.js` does not yet export or implement the WAL-006 source inventory contract. The reviewer incorrectly assumed this would turn green merely because `prepare.rs` now exists; it does not.

## Corrected no-load-error interpretation

The final stack footer is the normal Node stack for the seventh substantive `AssertionError`. There is no syntax or module-load failure. Hermes initially misdescribed it as a syntax/load error; a no-command session review corrected that statement.

## Diff inventory

Exactly `test/securityPolicy.node.js` changed, 5 insertions and 5 deletions:

- `WAL006_DIRECT_DEPENDENCIES.zcash_client_sqlite.features`: added `'serde'` to the accepted feature array.
- `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union` test: updated both the expected-accepted and expected-rejected feature strings to include `'serde'`.
- Same test: updated the git-dependency rejection string to include `'serde'`.

## Negative capability record

The 68/7 red proves the new test rejects the still-old policy/manifest state before production correction. The committed `Cargo.toml` does not yet include the `serde` feature on `zcash_client_sqlite`, so the manifest-feature test correctly fails. The persistent inventory-policy failure confirms the source inventory contract is not yet implemented. No production source, manifest, lockfile, policy implementation, Rust tests, fixtures, or other paths were changed. The four production paths (`wallet-broker/src/zec.rs`, `wallet-broker/src/zec/store.rs`, `wallet-broker/src/zec/test_support.rs`, `wallet-broker/src/zec/prepare.rs`) remain unstaged and uncommitted.
