# BBD-WAL-007 Phase-A Source Review 05

Decision: ACCEPTED FOR NODE POLICY RERUN AND INTEGRATION

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance HEAD: `b8501686`

Supersedes `BBD-WAL-007-TEST-SOURCE-REVIEW-04.md` by adding the committed policy source
and replacing the Node-policy hash. All other source identities remain accepted.

## Accepted source identity

| Path | Lines | Named tests | SHA-256 |
| --- | ---: | ---: | --- |
| `scripts/security-policy.js` | 2,676 | — | `a9274004a2fd80674f833fe493e7007ff3e90fe08d1c7d3345648f1dec185da3` |
| `test/securityPolicy.node.js` | 3,067 | 86 total / 4 WAL-007 | `c7d8f7a16b58ffa4224ee9975829d394f8a290b72fde55d14942580cf1c6905c` |
| `wallet-broker/Cargo.toml` | 113 | — | `189f067739afd8bc1e775e4d58b3ce7d4cfa6ab53e7b7acc5b02737396c5bff5` |
| `wallet-broker/tests/native_surface.rs` | 664 | 17 total / 4 WAL-007 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` |
| `wallet-broker/tests/xmr_distribution.rs` | 297 | 12 | `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8` |
| `wallet-broker/tests/xmr_process.rs` | 374 | 12 | `db0bb2272fb145a2317884dd98ed339cc248ed28b0250802f2462a0f88a781e0` |
| `wallet-broker/tests/xmr_rpc.rs` | 422 | 15 | `0046a94d8a3f7932c02e872f90afdcd8e0a79641f3b87db6cac4e2db25311b86` |
| `wallet-broker/tests/xmr_account.rs` | 586 | 16 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | 15 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |
| `wallet-broker/tests/xmr_hygiene.rs` | 329 | 9 | `3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f` |
| `wallet-broker/tests/xmr_local_gate.rs` | 458 | 1 gated | `00a1c7f7e4d01254a94f35b9d38b4a7374d0b74fe3c80d42ef258d7fdcc8728d` |

## Policy correction review

Hermes proved Rust formatting passes and the seven focused Rust targets fail for the
exact missing WAL-007 production boundaries. The remaining Node failure was a Phase-A
policy integration defect.

Sol changed `checkWalletBrokerManifest` only to require the already accepted empty
local-gate feature, exact MD5 dependency, and seven ordered XMR test targets. The
WAL-004 dependency/SBOM set and every unrelated policy remain unchanged. In the Node
test, Sol replaced only the path-search pattern. It still rejects literal
`which`/`whereis`, `var`/`var_os("PATH")`, and `env!`/`option_env!("PATH")`, while no
longer treating ordinary `path` identifiers as executable lookup. The negative fixture
remains. Test count is unchanged at 86; `git diff --check` is clean.

The ticket adds 84 Rust tests and four Node tests. No reviewer execution occurred. Sol
ran no tests, formatter, build, Cargo, Node, npm, network, or Git command. Hermes may
rerun only the Node policy and integrate on exact green. Wallet production and the real
local-Monero gate remain unauthorized.
