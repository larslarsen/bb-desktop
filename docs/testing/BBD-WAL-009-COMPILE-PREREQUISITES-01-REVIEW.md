# WAL-009 compile prerequisites 01 source review

Decision: ACCEPT the four authorized mechanical source edits. Close that Grok
task and open only the separate signature-context production handoff linked
below. This is source acceptance; no compiler or test result is claimed.

Reviewer: Codex at XHigh, baseline
`b319aa078dfbf28e14aa84d89a50e8247f4e53a1`.
The owner reported done. Outer session 64572 was collected once, exit 0.
Grok session `97461fa7-3a30-4efc-93be-7d3957c8a5b2` was authorized at
`e0bacf99`, CLI model grok-4.6, reasoning High. Its saved transcript reports
runtime model grok-4.6-build, high.

## Exact diff and preserved source

| Path | Lines | Accepted SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 883 | d95bea3aa78c326408cd22cabbb4a470ac30a53dea4d0fc34ecf5428310fa536 |
| wallet-broker/src/zec/test_support.rs | 4371 | fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec |

The reviewer reversed only the four prescribed replacements in memory.
The reconstructed entire files match both starting SHA-256 identities in the
[compile prerequisites handoff](../handoff/GROK_BUILD_BBD_WAL_009_COMPILE_PREREQUISITES_01.md).
The resulting unified diffs contain exactly:

- recipient().as_ref().map borrowing the stored Option before producing a slice;
- one clone of the borrowed proof Option into the existing owned result;
- an exhaustive six-variant PipelineFault-to-FaultPoint match at the existing
  wipe_exit_for_fault call, preserving the matching exit categories;
- removal of mut from SignVerifyHarness::publish's owned PipelineOutcome parameter.

All fifteen other pending source/test/package/lock/evidence paths were measured
and match their preceding identities, including the four-test module. No other
file change appears in repository status. The cfg(test) declaration and root
accessor are preserved. The signature context remains absent and the old
signature_hash type mismatch remains intentionally unrepaired.

The transcript contains four source replacements, scoped source reads/searches,
and two read-only hash/line-count commands. It shows no compiler, formatter,
tests, Cargo/npm, Git, network, evidence, dependency mutation, or other actor.
One procedural deviation remains: CURRENT_TASK.md was requested without a line
limit despite the leading-section instruction. This does not alter the exact
source acceptance. The next launch explicitly bounds that read.

## Next source contract and limits

Open [signature-context production 01](../handoff/GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_PRODUCTION_01.md)
for Grok High on spend.rs only. XHigh remains appropriate for reviewer work.
The accepted four-test source and understood mixed prerequisite red precede this
production slice; no intermediate rerun of the known blocked command is needed.

The reviewer rechecked the pinned local Authorization, EffectsOnly,
try_map_bundles, Transaction::into_data/txid, and signature_hash APIs. The
transaction mapper preserves headers and Sprout and separately moves Sapling,
Orchard, and Ironwood through its closures. The fixed private marker changes only
the transparent authorization type after rejecting every present transparent
bundle. Production independently_verify must consume its decoded Transaction and
use this same context's data and message for the actual Ironwood signatures and
proof. Schema/mainnet preflight retains precedence; the constructor's transparent
rejection necessarily precedes cryptographic verification.

This context supplies the pinned API's authorization type. It does not establish
independently recovered payment effects or actual secret cleanup. Existing
metadata-copy comparisons, synthetic wipe counters, and native modal/capability
behavior remain A3 blockers. The native Context::run error is separately parked.
Hermes execution, evidence, and integration remain closed until source review.
The focused green, transparent-rejection and message-corruption falsifications,
and broader regressions remain required.

The [expected-red review](BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01-REVIEW.md)
remains authoritative for the last execution and its evidence errata. All four
implementation evidence records remain uncommitted; later Hermes integration
must normalize their local prose paths and apply the recorded errata. All pending
developer source/package/lock work remains uncommitted. Do not recollect any
completed actor or poll the next one.

Reviewer publication scope: this review,
docs/handoff/GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_PRODUCTION_01.md,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
