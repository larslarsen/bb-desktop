# BBD-WAL-007 Phase-C Slice 1 Green 01 — COMPLETE

State: COMPLETE

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream c5c9aa8d · local 10b6d1a9
Provider: nous
Model: meituan/longcat-2.0:free
Node.js v22.23.1
Branch: master
HEAD: 2747ab17258a8a2ba22bedc25259a86485a41bfc
origin/master: 2747ab17258a8a2ba22bedc25259a86485a41bfc

## Filesystem fact

`wallet-broker/target` resides on `/dev/mapper/ubuntu--vg-ubuntu--lv` (ext4, disk-backed).

## Stop chronology

### Stop 1 — Original Green 01 (formatting)

`cargo fmt --check` failed because the accepted `wallet-broker/src/xmr/distribution.rs` and `wallet-broker/src/xmr/test_support.rs` contained formatting deviations. Resolved by Resume 01 mechanical formatting authorization.

### Stop 2 — Green Resume 01 (Node policy)

`node test/securityPolicy.node.js` failed with 3 failures. Root cause: the legacy generic `monero` token rule in `scripts/security-policy.js` was applied to the exact picker title string in `wallet-broker/src/native_ui.rs`, not the `eframe` or `rfd` imports. The policy checker's `checkRustWalletSource` function flagged the picker title as "forbidden unreviewed authority." Resolved by Native Policy Source Review 01, which corrected the policy/test pair to use a more specific token rule.

## Accepted source identity (Native Policy Source Review 01)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 12 | `08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |
| `wallet-broker/src/native_ui.rs` | 149 | `34fda529c4ac6035bb5147720f456a271145deb43878082fbdfe464d320a7bdf` |
| `wallet-broker/src/xmr.rs` | 3 | `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/model.rs` | 93 | `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47` |
| `wallet-broker/src/xmr/test_support.rs` | 368 | `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

## Frozen Phase-A identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |

## Mechanical formatting (Resume 01)

```
cargo fmt --manifest-path wallet-broker/Cargo.toml
```

Exit: 0. Only the two authorized files changed:

| Path | Post-format SHA-256 | Lines |
| --- | --- | ---: |
| `wallet-broker/src/xmr/distribution.rs` | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` | 914 |
| `wallet-broker/src/xmr/test_support.rs` | `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7` | 368 |

All seven other accepted paths retained Green 01 hashes. `git diff --check` clean.

## Restarted gate results (Resume 01)

| # | Command | Result | Exit |
| --- | --- | --- | ---: |
| 1 | `cargo fmt --check` | No diff | 0 ✓ |
| 2 | `cargo test --test native_surface` | 17 passed, 0 failed | 0 ✓ |
| 3 | `cargo test --test xmr_distribution` | 12 passed, 0 failed | 0 ✓ |
| 4 | `cargo check --features native-ui` | Finished without warning | 0 ✓ |

## Node-only resume results (Resume 02)

| # | Command | Result | Exit |
| --- | --- | --- | ---: |
| 5 | `node test/securityPolicy.node.js` | BitBook security policy tests passed (86). | 0 ✓ |
| 6 | `node scripts/security-policy.js` | BitBook desktop security policy checks passed. | 0 ✓ |

## Test-first falsification reference

The already accepted Phase-B expected-red result is the test-first falsification record: the same focused targets failed on the exact absent XMR production boundary before this source existed.

## No-mutation proof

No source bytes were modified beyond the authorized mechanical format. No production source, wallet, node, network, or real Monero boundary ran.

## Prohibited-action confirmation

No `xmr_local_gate`, Monero binaries, wallet, node, Electron, npm, browser, scanner, full suite, or network operation was run.
