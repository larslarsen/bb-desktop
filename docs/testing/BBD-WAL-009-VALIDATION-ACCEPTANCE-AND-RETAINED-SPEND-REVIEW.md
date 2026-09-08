# WAL-009 validation acceptance and retained-spend repair

Decision: ACCEPT the corrected validation data with the attribution/procedure
errata below. Close Hermes correction. Open one combined Grok test/production
repair for retained spend identity and external action-index propagation.
The owner requested fewer handoffs and shorter reports; related repairs are batched.

Correction session 20260908_090052_c07851, outer 60108, was collected once after
the owner said Continue; exit 0. Runtime: Hermes v0.18.2, nous,
poolside/laguna-s-2.1:free. Authorization: 8e6923a4; observed checkpoint: 395de708.
Reviewer: Codex at XHigh, baseline 395de708c57c1f98566bec818a33b1c913a86556.

The corrected evidence is 677 lines, SHA-256
6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58.
All five complete output blocks and command strings match original saved data.
All seventeen before/after rows match current full source identities; both
mutations were restored. The verified result remains: context 4 passed; three
required falsification failures; prepare 11 passed; sign/verify 6 passed/8 failed;
full library unrun. No test rerun occurred during correction.

Remaining errata are deferred to integration, without another correction cycle:
the preflight section mixes original/correction attribution; the no-rustc-probe
statement applies only to the correction. The correction attempted out-of-scope
recent-session queries, successfully used a broad recent-session selection,
wrote an unauthorized Hermes-home parse_cache.json scratch file, and ran a final
Git log check. Its blanket scratch/command-compliance statements are too broad.
Database access was read-only after initial failed URI/helper attempts. No source,
dependency, or Git mutation is evidenced. No cleanup is authorized here.

## Two source defects and their fixed repair boundary

1. spend.rs:782 counts every witness as a real spend. Pinned Orchard 0.15.5
   create_proof requires a witness for every action, including padding; PCZT
   reserialization retains those witnesses. With two proven actions this count
   is two, so the real_spends != 1 check rejects a valid transaction before
   extraction/context verification. Retain the unique unsigned real-spend index,
   randomized key, nullifier, and positive value before signing; verify that
   same spend after proving. Padding must have zero value, not no witness.
2. The builder shuffles action placement. test_support.rs:1106 passes zero as the
   expected index, and its view/contribution helpers repeatedly label action zero.
   A valid signature for retained slot one is rejected. Carry the actual index
   through the view and signed contribution; compare it with the original retained
   PCZT inside authorize_external. A batch position is not an action index.

The exact failed software/external success tests in saved result 77693 are the
existing red regressions. The witness defect is established by source/API flow;
the hard-coded-index defect is established independently of which random slot
that particular external fixture used. Do not claim all eight failures resolved
before execution.

Open [Grok retained-spend repair 01](../handoff/GROK_BUILD_BBD_WAL_009_RETAINED_SPEND_REPAIR_01.md).
Author strengthened regression tests first, then the fixed production repair in
the same source task, using the understood existing integration red. Later Hermes
must execute the affected integration/library tests and falsify retained-spend
classification and slot-one binding. No separate repeat of the old red is needed.

This repairs pipeline prerequisites. The metadata-copying effect comparison,
synthetic wipe observations, native confirmation, and remaining security gates
remain unaccepted. Mainnet, network, broadcast, Monero, and integration stay closed.
XHigh remains appropriate. No actor polling or recollection of completed sessions.

Reviewer publication scope: this review, the linked Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
