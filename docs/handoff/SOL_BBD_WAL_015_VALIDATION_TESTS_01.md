# WAL-015 compact message preflight regressions

Principal Dev Sol High under existing ticket escalation. Own ONLY new file
wallet-broker/tests/zec_live_validation.rs. Source authoring only: no execution,
formatting, dependencies, Git, evidence, other paths or actors. Baseline unchanged.
This is independent of the currently frozen combined source and compiler fixes.

Reviewer read pinned backend proto.rs: CompactBlock.hash/prev_hash prefer a parsed
header when nonempty, and CompactTx.txid copies into a32-byte array without checking
length. Existing batch validator checks only explicit hash fields; test both gaps.

Author two deterministic tests using a proposed hidden test_support helper
validate_live_batch_for_test(blocks:&[CompactBlock],from:u32,through:u32)->Result<(),ZecError>.
It must ultimately call the actual live::validate_batch used by the scanner, not a
separate test implementation. Keep this helper missing until Hermes records RED.

1. Validly parsed header whose upstream hash/previous hash disagrees with explicit
CompactBlock fields is rejected PROTOCOL_INCOMPATIBLE. Construct/serialize header
with pinned upstream BlockHeader APIs after reading local source, no handwritten
cryptography. Assert independently that upstream parsing succeeds and the two hash
representations differ. Include a consistent header positive case if practical.
2. Compact transaction with31-byte or33-byte txid is rejected before scanner; a32-byte
control passes structural validation. Use prost generated types; all other block
height/hash/prev fields valid and tiny. No network, DB, runtime keys or files needed.

Do not weaken existing tests or implement production/helper code. Report source-ready
and exact helper signature. Parent will add this target to next combined compile.
