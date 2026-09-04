# Codex Sol Handoff — BBD-WAL-009 Phase A1 Sign/Verify Tests 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Repository:
`/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-009.md`, BBD-WAL-006 and WAL-008
acceptance records, `wallet-broker/Cargo.toml`, the Zcash module/prepare/store/test
support, existing Zcash prepare/hardware tests, native authority source/tests, and the
accepted generic wallet intent contract/tests. You may inspect the pinned local
librustzcash and `pczt 0.9.3` sources read-only.

Grok remains owner-reported weekly-usage exhausted. This is the documented Sol fill-in.

Edit only:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_sign_verify.rs`

Add exactly one explicit test target named `zec_sign_verify`. Author the smallest
non-vacuous test contract satisfying every Phase-A1 requirement in BBD-WAL-009. Tests
lead production and must fail later for the missing typed sign/verify implementation,
not because of syntax, formatting, dependencies, or fixtures.

Use the accepted local v6/Ironwood fixture and a typed `zec::test_support` API. Require
independent decoded-effects observations, real call/wipe counters, exact binding and
barrier outcomes, production-empty hardware denial, and test-only synthetic Keystone
v2 signature contributions. Do not place real secret-looking material in source or
test output. Do not weaken the existing empty-production-table or no-broadcast
contracts. Avoid broad public API: test-only mutation/fault/oracle support belongs
under `zec::test_support` in the future production phase.

Current immutable identities include:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/src/zec.rs` | 253 | `6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2` |
| `wallet-broker/src/zec/prepare.rs` | 964 | `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e` |
| `wallet-broker/src/zec/store.rs` | 2849 | `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |

Do not run a formatter, test, build, lint, audit, scanner, dependency command, product,
Git command, network, wallet/node process, hardware/device action, or another actor.
Do not edit production source. Stop after writing the two authorized paths and report
their exact line counts and SHA-256 hashes plus confirmation that nothing ran.
