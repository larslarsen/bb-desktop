# BBD-WAL-006 Test-Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Source baseline: `363c0046`

Protected governance HEAD at review: `779fbfc87acba0d443fa7c44064323f91325bf68`

Result: **PHASE-A TEST SOURCE ACCEPTED FOR FIXTURE GENERATION AND EXPECTED RED;
PRODUCTION NOT AUTHORIZED**

## Accepted uncommitted paths

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `420559e61a7c81338c97301f7d470a8be6560c0f71aa9f666d79bbcc1424361d` |
| `wallet-broker/tests/zec_fixture_builder.rs` | 884 | `4d06eb0abc1a7c6529af4afbd7b862f907628f783dcb9a34f249331da9f98577` |
| `wallet-broker/tests/zec_address.rs` | 211 | `554db983c2581bfbf86b20150e5016265181ce874419cf4be3ae5aa110e84a0a` |
| `wallet-broker/tests/zec_store.rs` | 259 | `abf28745cd21149e36b3541e1fe12c1f3aa98ea4b33f5c5be23e2678705e1911` |
| `wallet-broker/tests/zec_scan.rs` | 258 | `847803dadd8177ab736cd7e87cab98fbb55767ea22f782207bc19aad22236af2` |
| `wallet-broker/tests/zec_prepare.rs` | 361 | `89900920ef8daf01aff4ba34fa08ea8a13fc5e26b81fa8f5d14eda3b86a75daf` |
| `wallet-broker/tests/zec_hygiene.rs` | 277 | `7ffacf053f83252b30890770ee4ab692698b5da7618c3f4e7789cbd9dd5e9d80` |
| `test/securityPolicy.node.js` | 2,340 | `ef74c328719a374cbfacbba1f2b0a34e164c27541e58cd1ef0a876acccc348b2` |

The accepted inventory is 48 Rust tests: fixture builder 4, address 8, store 8, scan 9,
prepare 11, and hygiene 8. The Node policy inventory is 73 tests: the 69 previously
accepted cases plus four WAL-006 cases. Only the eight authorized paths differ from the
governance HEAD; the index is clean, `git diff --check` passes for tracked changes, and
no reviewed path has trailing whitespace.

## Review outcome

The manifest preserves the Rust 1.98.0 package/native boundary and adds only the six
reviewed exact defaults-off Zcash pins and six explicit integration-test targets. The
Node tests freeze those declarations, distinguish unavoidable transitive PCZT capability
from BitBook authority, reject live-network/sign/prove/finalize/extract/broadcast
authority, and keep the Phase-A ZEC production-source inventory empty.

The upstream-only fixture builder uses the reviewed `LocalNetwork` schedule and official
test APIs. It deterministically records eight canonical compact blocks at heights
100–107, including the NU6.3 boundary, an older Orchard note, an unrelated output, a
confirmed 150,000,000-zatoshi Ironwood note, and a recognized 30,000,000-zatoshi
one-block-reorg victim. The height-107 fork replaces that victim with a distinct
120,000,000-zatoshi Ironwood note. The closed manifest contains all six generator
versions, exact hashes/lengths/links, 15 unique compact files, fixed public values, and
hostile/corrupt/reorg scenarios. The writer accepts only the fixed ignored target path,
rejects symlink/non-directory ancestors, creates no overwrite path, and imports no future
BitBook adapter.

The adapter tests bind the fixed seed/index-0 receiver to the independent fixture oracle
and decode its exact Orchard-only composition. They reserve durable monotonic/concurrent
issuance, every required seed-wipe exit, viewing-only SQLite state, all forbidden
persistence canaries with independently computed SHA-256 receipts, Linux owner/mode/type
failure cases, atomic migrations, contiguous and idempotent scanning, non-vacuous
confirmation and note-bearing reorg rollback, corruption/compound-failure behavior,
checked limits, pool selection, authoritative fee bounds, expiry/session binding, decoded
unsigned v6 Ironwood PCZT fields, bounded opaque handles, lifecycle wiping, redacted
diagnostics, and negative authority.

Three correction rounds removed vacuous confirmation/reorg assertions, circular canary
receipts, malformed Node mutations, missing immediate boundaries/session cases, an output
ancestor gap, missing wrong-owner and secret-class coverage, incomplete manifest binding,
and the missing cancellation wipe case. The final tests use fixed independent values and
typed test seams; they contain no adapter stub, mock production implementation,
`compile_error!`, ignored/conditional test, generic raw PCZT accessor, live endpoint,
mainnet material, valid mnemonic, or signing/proving/extraction/broadcast path.

## Execution boundary

Neither Sol nor the reviewer ran Rust, Cargo, Node, npm, tests, formatters, scanners,
dependency resolution, fixture generation, wallets, nodes, devices, or Git integration.
Compile/API compatibility, formatting, resolved graph/checksums/licenses/build scripts,
fixture bytes, and expected red remain unproved. Luna may integrate only these exact
hashes and execute only the Phase-B handoff at
`docs/handoff/CODEX_LUNA_BBD_WAL_006_FIXTURE_RED.md`.

Production source, policy implementation, broader tests, falsification, signing,
proving, extraction, broadcast, live network, mainnet, hardware, packaging, and another
repository remain unauthorized. XHigh must accept the resolved graph, frozen fixture,
and exact red evidence before a Phase-C source handoff exists.
