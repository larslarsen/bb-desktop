# WAL-015 core implementation split

Principal Dev Sol gpt-5.6-sol High escalation inherits the recorded ticket rationale.
Baseline e85dfdc8b6e06fdd09f84cee3c6ac37037448437. Read AGENTS.md, TESTING.md,
tickets/BBD-WAL-015.md and SOL_BBD_WAL_015_PRODUCTION_01.md. The reviewed initial
14-test RED authorizes implementation. You own ONLY wallet-broker/src/zec/live.rs.
Do not execute tests, Cargo, formatters, Git, evidence, or spawn actors. Read local
upstream source as needed. Primary actor /root/wal015_tests owns all other source.
Complete the core mechanisms; the current live.rs is an incomplete draft.

Final shared interface fixed by reviewer:
SourceMetadata { pub info: proto::service::LightdInfo, pub tip: proto::service::BlockId }
LiveSource.metadata(network,cancel) returns those raw two RPCs only, no birthday.
LiveSource.tree_state(height,cancel) returns RAW proto::service::TreeState.
LiveSource.blocks retains its existing signature. Core validates raw metadata and
TreeState network, consensus branch, checked heights/hash/string bounds before
upstream conversion. Coordinate other necessary API changes with primary actor.

Implement actual schema/official-account UFVK/birthday binding, validated ancestors
and sidecars, recognized initialized state, exact stored checkpoint invariants,
full batch continuity, tip/checkpoint revalidation and bounded reorg replacement
transaction. Keep original receive files unchanged. Upstream scanner/import/rewind
only. Accurate Orchard plus Ironwood balances, three confirmations; no fake zeros.
Emit actual committed progress through a bounded callback/slot that primary manager
can wire. Cancellation after committed work returns a stale snapshot, first-start
cancellation stays unsynced/cancelled. Idempotent replay must not mutate live bytes.
Fault injection must interrupt actual scan/checkpoint/commit transaction stages.
Thin test harness uses the same production engine and validators; agree observer/
fault plumbing with primary actor, never manufacture successful observations.

Read wallet-broker/tests/zec_live_sync.rs and the new test-support draft to match
interfaces and known fixture amounts. Testnet birthday comes from upstream NU5;
explicit Local fixture uses its own consensus schedule and ChainState. Reorg has
same-height replacement and deep-reorg failure cases. Preserve committed bytes on
bad responses and injected transaction errors. Inspect pinned upstream APIs before
writing; source actor cannot compile. Drop a coherent implementation for Hermes.

Endpoint accepts HTTPS DNS/IP with optional valid port (default443) and root slash,
rejects userinfo/query/fragment/non-root path/control/non-ASCII and size excess.
No extra direct dependencies in this file. Primary may add existing resolved
incrementalmerkletree0.8.2 legacy-api for fixture tree serialization only.
