# Codex Sol Handoff — BBD-WAL-006 Fixture Reorg Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, the ticket, fixture-generation review 01, fetched
`zcash_client_backend 0.24.0` `TestState::generate_block_at` implementation,
`zcash_client_sqlite 0.22.0` `BlockCache` implementation, and the complete fixture test.
Require protected `HEAD == origin/master`, clean index, `git diff --check`, exactly six
untracked ZEC tests, an absent `wallet-broker/target/wal006-fixture-build`, and:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_fixture_builder.rs` | 890 | `efb104bedeaf48f5e3a0850f84a6b504651bad2267eb3fc4a443864ae2fd3c81` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The other five ZEC tests remain at their accepted format-correction hashes.

Edit only `wallet-broker/tests/zec_fixture_builder.rs` with `apply_patch`:

1. Add `FakeCompactOutput` to the existing
   `zcash_client_backend::data_api::testing` import.
2. In `RecordingCache::insert`, before recording/inserting a block whose exact height is
   already present in the observation vector, truncate only `self.inner` to the preceding
   height via its `TestCache::truncate_to_height` implementation. Use checked u64-to-u32
   and predecessor conversion with fixed diagnostic text. Do not remove the prior block
   from `recorded`; the vector must retain canonical height 107 followed by replacement
   height 107. Sequential first inserts must retain existing behavior.
3. Remove the `state.truncate_to_height(CONFIRMATION_HEIGHT)` wallet/cache rewind and the
   following `generate_next_block` replacement call.
4. Find the recorded canonical block at `CONFIRMATION_HEIGHT` and require its
   `chain_metadata`. Construct one `FakeCompactOutput` for the same
   `IronwoodFvk(account_orchard)`, external address type, and exact replacement value.
5. Call `state.generate_block_at` for `CONFIRMATION_HEIGHT + 1` using exactly the recorded
   parent block hash, its Sapling/Orchard/Ironwood commitment-tree sizes, and
   `allow_broken_hash_chain = false`.
6. Retain the existing last-recorded-block extraction and explicitly require its height
   to equal `CONFIRMATION_HEIGHT + 1` before `assemble_fixture`.

Preserve every generator version, network activation, seed/account derivation, receiver,
amount, canonical block sequence, RNG ownership/order up to canonical height 107,
scenario, manifest field, encoder, path/mode rule, test name/assertion, hygiene negative,
and all other bytes/semantics. Add no scan, wallet rewind, production import, fixture
byte, dependency, feature, network/live data, unsafe code, ignored test, conditional
skip, or fallback.

Run no executable, formatter, Cargo, Rust, Node, npm, test, resolution, network, fixture,
Git, cleanup, wallet, node, or device command. After editing, only read-only `wc -l`,
`sha256sum`, literal/diff inspection, and `git diff --check` over the authorized test are
allowed. Report exact line count/hash/diff, unchanged four tests and five sibling hashes,
frozen committed inputs, and blockers. Luna owns rerun, fixture output, evidence,
integration, commit, and push.
