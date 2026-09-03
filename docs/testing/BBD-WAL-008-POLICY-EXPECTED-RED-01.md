# BBD-WAL-008 Policy Expected Red 01

Jr Dev — Hermes integration evidence.

Hermes version: `Hermes Agent v0.18.2 (2026.7.7.2) · upstream 63279301 · local 10b6d1a9 (+1 carried commit)`
Provider: `nous`
Model: `meituan/longcat-2.0:free`

## Preflight

- branch `master` ✓
- `HEAD == origin/master` at `ea9eea4b96d6b0f8edf598cad89fa3a8e40f9db8` ✓
- clean index ✓
- exactly four dirty worktree paths:
  - `test/securityPolicy.node.js` (3,358 lines, SHA-256 `464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07`) ✓
  - `wallet-broker/src/zec/hardware.rs` (924 lines, SHA-256 `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760`) ✓
  - `wallet-broker/src/zec/store.rs` (2,849 lines, SHA-256 `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a`) ✓
  - `wallet-broker/src/zec/test_support.rs` (2,500 lines, SHA-256 `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82`) ✓
- unchanged manifest `wallet-broker/Cargo.toml` (117 lines, SHA-256 `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530`) ✓
- unchanged lockfile `wallet-broker/Cargo.lock` (5,394 lines, SHA-256 `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`) ✓
- unchanged production policy `scripts/security-policy.js` (2,689 lines, SHA-256 `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626`) ✓
- clean `git diff --check` ✓
- repository work directory ext4 ✓
- Cargo work directory ext4 ✓

## Command

```text
node test/securityPolicy.node.js
```

## Result

- exit: `1`
- 80 `ok`
- 7 `not ok`
- final line: `7 security policy test(s) failed`

## Exact seven failing groups

1. `committed workflows satisfy the fail-closed checker`
   - cause: `checkWalletBrokerManifest` reports "wallet Rust manifest integration-test targets differ from review" — frozen production manifest checker omits the accepted `zec_hardware` target.
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
   - cause: same `checkWalletBrokerManifest` mismatch.
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`
   - cause: same `checkWalletBrokerManifest` mismatch.
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`
   - cause: same `checkWalletBrokerManifest` mismatch.
5. `WAL-006 manifest pins the exact production support APIs for RNG and SQLite`
   - cause: same `checkWalletBrokerManifest` mismatch.
6. `WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin`
   - cause: same `checkWalletBrokerManifest` mismatch.
7. `BBD-WAL-008 closes the hardware target and current eight-path ZEC policy inventory`
   - cause: the new WAL-008 export/checker contract is absent from frozen production policy (`scripts/security-policy.js`). The test expected `['zec_hardware']` for the WAL-008 test-target contract but received `undefined`.

## Confirmed scope

- The first six failures are exactly the frozen production manifest checker omitting the accepted `zec_hardware` manifest target. No other reason.
- The seventh failure is exactly the frozen production policy lacking the new WAL-008 export/checker contract. No other reason.
- The renamed historical WAL-006 inventory group (`WAL-006 preserves the exact historical seven-path Phase-C ZEC production inventory`) is `ok`.
- No syntax, module, fixture, exception, unrelated group, or production-source failure.
