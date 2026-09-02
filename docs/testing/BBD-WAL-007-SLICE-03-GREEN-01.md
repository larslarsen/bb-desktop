# BBD-WAL-007 Slice 3 Green Evidence 01

## Hermes Identity

- Hermes Agent v0.18.2 (2026.7.7.2) · upstream 1cb3ab61 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Filesystem

- wallet-broker/target is disk-backed (ext4 on /dev/mapper/ubuntu--vg-ubuntu--lv)

## Protected Governance Parent

- HEAD == origin/master == 7d40e7943fd84059f780158f79cfcba35e4b7b89

## Protected Identities (verified before and after execution)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/tests/xmr_rpc.rs | 694 | a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b |
| wallet-broker/src/xmr.rs | 5 | 92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411 |
| wallet-broker/src/xmr/model.rs | 151 | 23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9 |
| wallet-broker/src/xmr/rpc.rs | 1,896 | 2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed |
| wallet-broker/src/xmr/test_support.rs | 2,705 | 2c445494b36fd645240cc9a25405782eff6d3e8187b4762aee6a0fd7ca640a40 |
| wallet-broker/Cargo.toml | 113 | 189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5 |
| wallet-broker/Cargo.lock | 5,394 | 29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420 |
| wallet-broker/src/xmr/distribution.rs | 914 | 4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5 |
| wallet-broker/src/xmr/process.rs | 1,184 | 7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f |
| wallet-broker/tests/xmr_distribution.rs | 297 | 481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8 |
| wallet-broker/tests/xmr_process.rs | 453 | 25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| scripts/security-policy.js | 2,689 | 6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626 |
| test/securityPolicy.node.js | 3,189 | dd22ab83645d5dffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497 |

## Formatter

- Command: `/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check`
- Result: exit 0, no mutation

## Falsification

- Temporarily removed `|| !info.bootstrap_daemon_address.is_empty()` from `evaluate_node_policy` in `wallet-broker/src/xmr/rpc.rs`
- Command: `/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc node_syncing_is_distinct_from_bootstrap_remote_and_unavailable`
- Result: exit 101, test failed because production core accepted the injected current bootstrap address (Ok value: NodeProbeView { state: NodeStateView(Ready) })
- Restored the exact line
- Verified: wallet-broker/src/xmr/rpc.rs is 1,896 lines with SHA-256 2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed

## Green Sequence Results

1. xmr_rpc: 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
2. xmr_process: 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
3. xmr_distribution: 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
4. native_surface: 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered
5. cargo check (native-ui, native_surface): exit 0, no errors
6. node test/securityPolicy.node.js: exit 0, 86 ok, no not ok, final "BitBook security policy tests passed (86)."
7. node scripts/security-policy.js: exit 0, final "BitBook desktop security policy checks passed."

## Scope

- Integrated only the accepted Slice-3 drop (five paths)
- No source design, repair, or formatting
- No Slice 4, broader acceptance, or real local-Monero gate
- No other repository touched

## Prohibited-Action Confirmation

- No /tmp, download, network, personal Monero path, or product/Monero binary used
- No sensitive path, cache/artifact path, environment value, port, credential, process ID, or raw output recorded
- No staging or commit of temporary mutation
