# BBD-WAL-007 Expected Red Resume 04 — COMPLETE

State: COMPLETE

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9
Provider: meituan/longcat-2.0:free
Node.js v22.23.1
Branch: master
HEAD: 4d1cf1c7ba22fdb0846921e788fea7c0cd0526be
origin/master: 4d1cf1c7ba22fdb0846921e788fea7c0cd0526be

## Source identity (verified)

| Path | Lines | Named tests | SHA-256 | Status |
| --- | ---: | ---: | --- | --- |
| `scripts/security-policy.js` | 2,676 | — | `a9274004a2fd80674f833fe493e7007ff3e90fe08d1c7d3345648f1dec185da3` | OK |
| `test/securityPolicy.node.js` | 3,067 | 86/4 | `c7d8f7a16b58ffa4224ee9975829d394f8a290b72fde55d14942580cf1c6905c` | OK |
| `wallet-broker/Cargo.toml` | 113 | — | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` | OK |
| `wallet-broker/tests/native_surface.rs` | 664 | 17/4 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` | OK |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | 12 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` | OK |
| `wallet-broker/tests/xmr_process.rs` | 374 | 12 | `db0bb2272fb145a2317884dd98ed339cc248ed28b0250802f2462a0f88a781e0` | OK |
| `wallet-broker/tests/xmr_rpc.rs` | 422 | 15 | `0046a94d8a3f7932c02e872f90afdcd8e0a79641f3b87db6cac4e2db25311b86` | OK |
| `wallet-broker/tests/xmr_account.rs` | 586 | 16 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` | OK |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | 15 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` | OK |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | 9 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` | OK |
| `wallet-broker/tests/xmr_local_gate.rs` | 458 | 1 | `00a1c7f7e4d01254a94f35b9d38b4a7374d0b74fe3c80d42ef258d7fdcc8728d` | OK |
| `wallet-broker/Cargo.lock` | — | — | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` | OK |

`git diff --check`: clean.

## Resolved dependency

Lockfile delta relative to protected HEAD:
1. Add `md-5` to the existing `bitbook-wallet-broker` dependency list
2. Add `zeroize` to the existing `block-buffer 0.11.0-rc.3` dependency list
3. Add `zeroize` to the existing `digest 0.11.0-pre.9` dependency list
4. Add one registry package, `md-5 0.11.0-pre.4`, checksum `117b97b6b9ae1ec9a396b357698efa3ecff4fc1f40e0ec59ae7c1270b460ac1d`, depending only on `cfg-if` and `digest 0.11.0-pre.9`

No existing version/checksum/source changes.

## Preserved Resume 03 results

- Cargo formatting exited zero without mutation
- `native_surface` exited 101 only on absent `XmrInstallationSelectionPort` and `XmrSelectionController`
- `xmr_distribution`, `xmr_process`, `xmr_rpc`, `xmr_account`, `xmr_receiver`, `xmr_hygiene` exited 101 only because `bitbook_wallet_broker::xmr` is absent

## Resume 04 Node policy result

| Command | Result | Exit |
| --- | --- | ---: |
| `node test/securityPolicy.node.js` | BitBook security policy tests passed (86). | 0 |

## Path audit

Modified/added paths (only authorized eleven paths):
- `scripts/security-policy.js` (modified)
- `test/securityPolicy.node.js` (modified)
- `wallet-broker/Cargo.toml` (modified)
- `wallet-broker/Cargo.lock` (modified)
- `wallet-broker/tests/native_surface.rs` (modified)
- `wallet-broker/tests/xmr_distribution.rs` (new)
- `wallet-broker/tests/xmr_process.rs` (new)
- `wallet-broker/tests/xmr_rpc.rs` (new)
- `wallet-broker/tests/xmr_account.rs` (new)
- `wallet-broker/tests/xmr_receiver.rs` (new)
- `wallet-broker/tests/xmr_hygiene.rs` (new)
- `wallet-broker/tests/xmr_local_gate.rs` (new)

Plus evidence/handoff: `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md`, `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md`, `docs/handoff/HERMES_BBD_WAL_007_EXPECTED_RED_RESUME_04.md`, `docs/handoff/CURRENT_TASK.md`.

`git diff --check`: clean. All hashes verified. No production source, wallet, node, network, or real Monero boundary ran.
