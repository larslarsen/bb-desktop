# BBD-WAL-006 Final Policy Gate 01 Evidence

Execution timestamp: 2026-09-01

Protected governance parent: 5c429d2d6c55e369a40dffc2aeaff10bb77c631d (commit containing HERMES_BBD_WAL_006_FINAL_POLICY_INTEGRATION_01.md)

Filesystem type: ext4 (disk-backed, /dev/mapper/ubuntu--vg-ubuntu--lv)

Hermes identity:
- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Protected preconditions (all exact)

HEAD == origin/master == 5c429d2d6c55e369a40dffc2aeaff10bb77c631d, exactly one dirty path, clean `git diff --check` (exit 0, no conflict markers or whitespace errors):

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2,482 | d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5 |
| test/securityPolicy.node.js | 2,525 | 2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169 |

## Exact execution (each run once, in order, no network)

1. `node test/securityPolicy.node.js`
   - Exit: 0. Exactly 75 `ok`, zero `not ok`, final line `BitBook desktop security policy tests passed (75).`

2. `node scripts/security-policy.js`
   - Exit: 0. Exact line `BitBook desktop security policy checks passed.`

3. `npm run test:security`
   - Exit: 0. Electron security exactly 19 passed (`BitBook electron security tests passed (19).`), policy exactly 75 passed (`BitBook security policy tests passed (75).`), no failure.

## Closure of the six prior failures

The six failing groups recorded in `docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md` are now closed. The accepted WAL-006 policy implementation in `scripts/security-policy.js` (181 insertions, 5 deletions) adds:

- `checkWal006ResolvedFeatures`: enforces the exact direct dependency contract, no forbidden enabled feature, exact compiled-PCZT capability inventory, and exact BitBook authority `receiver.fresh`, `fixture.scan`, `pczt.prepare`.
- `checkWal006RustSourceInventory`: enforces the exact seven sorted paths from `WAL006_ALLOWED_RUST_SOURCE_PATHS`.
- Contextual `checkRustWalletSource` extension: permits reviewed offline Zcash library use (including `zcash_client_backend` transitives) while rejecting direct network/listener/endpoint/lightwalletd/service-client authority, PCZT or transaction sign/prove/finalize/extract calls, broadcast calls, and `Network::MainNetwork` in product source.
- Recursive `checkRepository` enumeration of `wallet-broker/src/zec.rs` and `wallet-broker/src/zec/**/*.rs`, enforcing its exact closed set.

The three WAL-004 inventory groups remain intact; `checkRustWalletSourceInventory` now accepts either the exact legacy top-level set or that set plus the now-required `wallet-broker/src/zec.rs` entry, remaining order-independent and rejecting malformed, duplicate, missing, and every other extra path.

## Exact closed WAL-004 inventory

`WAL004_RUST_SOURCE_PATHS` (7 paths, byte-for-byte preserved):
- wallet-broker/src/lib.rs
- wallet-broker/src/vault.rs
- wallet-broker/src/store.rs
- wallet-broker/src/session.rs
- wallet-broker/src/native.rs
- wallet-broker/src/native_ui.rs
- wallet-broker/src/hygiene.rs

## Exact closed WAL-006 inventory

`WAL006_ALLOWED_RUST_SOURCE_PATHS` (7 paths, exact sorted set):
- wallet-broker/src/zec.rs
- wallet-broker/src/zec/address.rs
- wallet-broker/src/zec/fixture.rs
- wallet-broker/src/zec/prepare.rs
- wallet-broker/src/zec/scan.rs
- wallet-broker/src/zec/store.rs
- wallet-broker/src/zec/test_support.rs

## Forbidden-feature/authority enforcement

`WAL006_FORBIDDEN_FEATURES` (8 forbidden enabled features): sync, lightwalletd-tonic, lightwalletd-tonic-tls-webpki-roots, lightwalletd-tonic-transport, tor, zcashd-compat, zewif, non-standard-fees.

`WAL006_EXPECTED_COMPILED_PCZT_CAPABILITIES` (10 compiled upstream capabilities, not BitBook authority): io-finalizer, orchard, prover, sapling, signer, spend-finalizer, transparent, transparent-inputs, tx-extractor, zcp-builder.

BitBook authority is exactly `receiver.fresh`, `fixture.scan`, `pczt.prepare`. Raw PCZT, sign/prove/finalize/extract/broadcast, and network authority are rejected.

## Legitimate offline upstream allowance

The contextual source permit explicitly allows reviewed offline Zcash library use including `zcash_client_backend` transitives. `wallet-broker/src/zec/test_support.rs` retains its reviewed wrong-network MainNetwork test vector. The implementation avoids broad `finalize` matching that would reject cryptographic hasher `.finalize()`.

## Source/test identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2,482 | d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5 |
| test/securityPolicy.node.js | 2,525 | 2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169 |

## Integration

Staged exactly:
- scripts/security-policy.js (181 insertions, 5 deletions)
- docs/testing/BBD-WAL-006-FINAL-POLICY-GATE-01.md (new evidence)
- docs/handoff/CURRENT_TASK.md (leading current-task block updated)

Commit: `fix: enforce WAL-006 security policy`

Push: master

Final state: HEAD == origin/master, clean index, clean tracked worktree.

The reviewer alone accepts the result and authorizes the next task.
