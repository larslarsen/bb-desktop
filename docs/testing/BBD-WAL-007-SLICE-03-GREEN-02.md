# BBD-WAL-007 Slice-3 Green Evidence 02

Hermes identity:
- Hermes Agent v0.18.2 (2026.7.7.2) · upstream 1cb3ab61 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

Disk-backed filesystem: wallet-broker/target resides on ext4 (/dev/mapper/ubuntu--vg-ubuntu--lv), not tmpfs.

Protected governance parent: 1850565d (HEAD == origin/master, clean index, git diff --check clean, exactly one worktree path modified).

Protected identities verified before execution:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/rpc.rs` | 1,913 | `7593322a5aef2fc146698d2e07a541cd9fb796b92e1f8e3fd699bcfbb2b219f9` |
| `wallet-broker/tests/xmr_rpc.rs` | 694 | `a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b` |
| `wallet-broker/src/xmr.rs` | 5 | `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411` |
| `wallet-broker/src/xmr/model.rs` | 151 | `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9` |
| `wallet-broker/src/xmr/test_support.rs` | 2,705 | `2c445494b36fd645240cc9a25405782eff6d3e8187b4762aee6a0fd7ca640a40` |
| `wallet-broker/Cargo.toml` | 113 | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/process.rs` | 1,184 | `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/xmr_process.rs` | 453 | `25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833` |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5dffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

Formatter: `/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check` — exit 0, no output diagnostic, no mutation.

Falsification: temporarily removed `|| !info.bootstrap_daemon_address.is_empty()` from `evaluate_node_policy` in `wallet-broker/src/xmr/rpc.rs`. Test `node_syncing_is_distinct_from_bootstrap_remote_and_unavailable` run exactly once. Result: exit 101, 0 passed, 1 failed, 0 ignored, 0 measured, 14 filtered out, test failed with `called Result::unwrap_err() on an Ok value: NodeProbeView { state: NodeStateView(Ready) }` — confirming the production core accepted the injected current bootstrap address. Immediately restored the exact line. `wallet-broker/src/xmr/rpc.rs` restored to 1,913 lines, SHA-256 `7593322a5aef2fc146698d2e07a541cd9fb796b92e1f8e3fd699bcfbb2b219f9`. No staging or commit of the temporary mutation.

Warning-free normalized green command results:

| Command | Result |
| --- | --- |
| `cargo test --test xmr_rpc` | 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| `cargo test --test xmr_process` | 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| `cargo test --test xmr_distribution` | 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| `cargo test --test native_surface` | 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| `cargo check --features native-ui --test native_surface` | exit 0, no warning or diagnostic |
| `node test/securityPolicy.node.js` | 86 `ok`, no `not ok`, final `BitBook security policy tests passed (86).` |
| `node scripts/security-policy.js` | exit 0, final `BitBook desktop security policy checks passed.` |

The formatter, falsification compile stage, and normalized green Rust commands emitted no warning or compile diagnostic. The falsification emitted only the expected runtime failure already recorded above. No accepted source/test remained mutated after restoration.

Scope: Resume 07 integrated only the accepted warning correction in `wallet-broker/src/xmr/rpc.rs`, Green Evidence 02, and `CURRENT_TASK.md`. `6eb566d6` integrated the base Slice-3 RPC transport. `c4bda0e9` newly integrated the accepted warning correction. No other source/test was integrated.

Green Evidence 01 rejection: Green Evidence 01 is rejected and not reused. Hermes continued after two warnings, altered a command with an output pipeline, and reran every green command after commit/push. This Green Evidence 02 is a wholly fresh execution with no reused Resume-06 results.

Prohibited-action confirmation: no formatter/test/check/Node/npm/policy/security/build/product command run after commit/push; no source repair; no Slice 4; no real local-Monero gate; no formatter, falsification, or normalized-green command used a wrapper, redirection, or pipeline, and no Resume-06 result was reused; no `/tmp`, download, network, personal Monero path, or product/Monero binary used.
