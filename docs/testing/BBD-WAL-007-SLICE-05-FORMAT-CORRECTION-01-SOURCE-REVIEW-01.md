# BBD-WAL-007 Slice-5 Format Correction 01 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `baa392e3`

Result: **ACCEPTED FOR FRESH HERMES EXECUTION**

Codex Spark High verified the exact seven-path accepted preimage and the three frozen
identities, then ran the one authorized Rust 1.98 `rustfmt` command exactly once. It
exited 0. The actor ran no formatter check, compiler, test, Clippy, build, product,
Node/npm, package-manager, policy/security, network, Git, or governance/evidence
mutation. Its read-only post-command identity report confirms that only the seven
authorized source paths changed.

Independent reviewer inspection confirms a clean index, the expected eight-source
Slice-5 worktree plus the frozen untracked Hermes stop draft, exact frozen identities,
and a clean `git diff --check`. The resulting formatter identities are:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce` |
| `wallet-broker/src/xmr/account.rs` | 3,375 | `5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb` |
| `wallet-broker/src/xmr/process.rs` | 1,964 | `66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d` |
| `wallet-broker/src/xmr/receiver.rs` | 871 | `4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0` |
| `wallet-broker/src/xmr/rpc.rs` | 2,576 | `1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33` |
| `wallet-broker/src/xmr/store.rs` | 1,904 | `08f7678c8fa5ce85d313c28e9b1ac79b42698ae6ead80c5e7e994f878d6069cd` |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `a815ab198559e7942d1c91ce0466a52d3b751631dba6dd80d5044682ec90cf33` |

The frozen `xmr/model.rs` remains 214 lines at
`2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb`,
the receiver test remains 588 lines at
`d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`,
and Hermes's untracked 59-line stop draft remains at
`20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637`.

This accepts source layout only. Green status remains unproved. Hermes alone may run
the linked fresh formatter check, durable-replay falsification, focused Slice-5 and
affected-regression tests, warning-denied Clippy, native check, and policy checks, and
may replace the frozen stop draft and integrate only after exact success. Broader/final
acceptance and the real offline local-Monero gate remain unauthorized.
