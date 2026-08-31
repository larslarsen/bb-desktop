# BBD-WAL-006 Fixture Generation Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `1f4b8a5d`

Result: **FIRST FIXTURE RUN REJECTED — TEST-ONLY REORG GENERATOR CORRECTION AUTHORIZED**

Luna's first locked/offline fixture-builder run exited 101 with two passed and two failed.
Both generation paths stopped at upstream
`zcash_client_backend-0.24.0/src/data_api/testing.rs:970` with:

```text
RequestedRewindInvalid { safe_rewind_height: None, requested_height: BlockHeight(106) }
```

The builder calls `TestState::truncate_to_height(106)` after generating the canonical
height-107 block. Upstream first asks the wallet database to rewind, but this independent
oracle intentionally has not scanned the generated chain, so no safe wallet rewind
height exists. The cache truncation is never reached. This is an unintended test-harness
construction error, not a fixture, dependency, production, or product error.

The published harness already exposes `generate_block_at`, which can generate the
replacement height-107 block directly from the recorded height-106 hash and exact
Sapling/Orchard/Ironwood tree sizes. The recording cache can safely truncate only its
inner duplicate height before insertion while preserving its append-only observation
vector containing both canonical and replacement blocks. No wallet rewind is needed.

Luna correctly stopped: it did not run the second builder invocation, freeze/inspect a
fixture, run Node or adapter red, create evidence, or perform Git. Reviewer inspection
confirms the fixture output directory remains absent, `HEAD == origin/master ==
1f4b8a5d`, the index is clean, exactly six accepted ZEC tests remain untracked, all
hashes are frozen, and `git diff --check` passes.

Sol may edit only `wallet-broker/tests/zec_fixture_builder.rs` under the active handoff.
Execution, fixture bytes, evidence, integration, and Git remain Luna-owned. ZEC
production remains unauthorized.
