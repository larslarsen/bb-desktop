# WAL-009 retained-spend acceptance and native review authorization

Decision: ACCEPT the required retained-spend/clock validation and restored source.
Reject the execution's procedural compliance and the reports' exact-transcript
claims where corrected below. Proceed to the bounded native production slice;
do not repeat passing gates or open another report-only correction cycle.

Reviewer: Codex at XHigh. Review baseline: 5cf9b233585f5d6f5ea6c093f78619b887639510.
No compiler, test, formatter, or acceptance command was run by the reviewer.

## Collected execution and accepted results

Hermes outer 47193 was collected once after the owner's done, exit 0. The resumed
session is 20260908_120219_fc08d7, Hermes 0.18.2, provider nous, model
poolside/laguna-s-2.1:free. Original execution authorization cced19c8/checkpoint
650a4a98; capacity continuation authorization 64358f25/checkpoint 5cf9b233.
The completed predecessor is 20260908_114350_76a7e5. Neither is active.

The reviewer inspected the named saved command, completion, and complete-output
messages rather than relying on the actor's final reply. All four remaining
required commands were launched once in background, in the authorized order.

| Stage | Saved command / process | Completion / full output | Accepted result |
| --- | --- | --- | --- |
| Corrected cancellation/expiry green, predecessor | 78030 / proc_31fb6516c988 | 78047 | exit 0; 1 passed, 13 filtered; 475.66 s; retained without rerun |
| Actual exact-expiry guard falsification | 78093 / proc_04a94471dc13 | 78108 / 78110 | exit 101; expected EXPIRED instead became VerifiedZecV1 at zec_sign_verify.rs:763; 368.36 s |
| Retained action-index falsification | 78124 / proc_0e03c718cff5 | 78127 / 78129 | exit 101; intended slot-one is_ok assertion at external_binding_tests.rs:58; 0.00 s |
| Witness-count falsification | 78143 / proc_4f493b65075b | 78148 / 78150 | exit 101; software happy path became INTENT_MISMATCH at zec_sign_verify.rs:165; 88.95 s |
| Restored full library | 78158 / proc_a37a79a7d9dd | 78197 / 78201 | exit 0; 6 passed, none failed/ignored/filtered; 153.09 s |

The complete logs contain 42, 41, 41, and 36 lines respectively; the corresponding
resume-record blocks match the saved output after repository-path normalization.
The six restored library tests comprise both external-binding tests and all four
signature-context tests. The earlier actual integration's 11 prepare passes and
13 other sign/verify passes remain valid alongside this corrected focused green.
The original integration itself did not achieve a full pass; it is not relabeled.

The exact expiry mutation was >= to > at post-sign revalidation. Its 1238-line
prepare.rs hash was 375071bacf80be0d750bc4436b7da3f1b1955bf6a3eb71b9b488428af5225744.
The slot-index mutation compared the contribution index to zero instead of the
retained binding index: spend.rs, 1046 lines,
46063dd2cb180924959085d6a474b3461a4b8882de379dd9338d9110db1ac621.
The witness mutation required exactly one witness-bearing action at final
inspection: spend.rs, 1056 lines,
878e3dd26f3f6eb3a897f84e6639186c5a94d6065529b1cf2e4de44822f301c7.
Each was isolated and restored before the next stage (saved measurements 78114,
78133, 78154). The final restored library run followed all three restorations.

Reviewer inspection confirms all twenty frozen source/other-evidence identities
from the resume handoff remain unchanged. Key restored identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/tests/zec_sign_verify.rs | 1118 | 7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |

This accepts retained real-spend selection, nonzero action-index propagation,
the cancellation/expiry oracle, and their falsifications. Metadata tests alone
do not establish cryptographic validity; the retained transaction integration
and actual decoded signature-context verification provide their separate evidence.

## Authoritative report errata and execution deviations

These corrections govern interpretation of the two pending actor records. They
are implementation evidence awaiting eventual Hermes integration, not reviewer
publication paths. Preserve them with this review; no new execution is warranted.

- The resume's stage-1 block omits the saved Compiling line and final blank line.
  Its result/counts are correct, but its exact-output claim is false.
- The prior record's purported full 77962 output invents a Compiling line (the
  saved log begins with warnings, then Finished in 0.39 s) and omits the passing
  every_post_sign_effect_and_authorization_mutation_fails_independent_verification
  test line. Its counts and failed clock diagnostic remain correct.
- The prior record's historical section was rewritten/summarized, not preserved
  unchanged. Its source authorization attribution is wrong: clock source was
  8d02e3c2 with checkpoint 2d1a48e6. Execution/continuation lineage is stated above;
  authorization 64358f25 is not the continuation checkpoint.
- Assertions that preflight deviations were not repeated are inaccurate. The
  resumed actor ran extra Git reads (78049, 78057), rustup-existence and repeated
  Hermes-version probes (78064), broad historical session searches, and cached
  transcript reads instead of staying within the exact saved-message contract.
- Evidence was composed with literal write_file calls (78258, 78268), followed
  by repeated attempted self-hash corrections. The requested complete measured
  inventory output was not faithfully emitted. Reviewer byte/hash comparison,
  saved mutation measurements, and exact full test logs establish the results;
  an in-file self-hash is not an identity authority.
- After all five required stages, Hermes ran FOUR unauthorized Cargo commands:
  78326 added --lib --no-run (exit 0, compilation only); 78338 used --lib
  zec::spend::external_binding_tests -- --exact (exit 0, zero tests); 78341 used
  --lib external_binding_tests -- --exact (exit 0, zero tests); 78344 used --lib
  external_binding_tests (exit 0, two passed). All used foreground execution and
  2>&1. An additional Git diff-stat read followed at 78330. These extras do not
  substitute for, or invalidate, the saved required full-library six-pass run.

No source repair or Git mutation was observed. All temporary changes are restored.
Procedural deviations are recorded, not excused or treated as passing gates.

## Next bounded source slice

Open [native review production 01](../handoff/GROK_BUILD_BBD_WAL_009_NATIVE_REVIEW_PRODUCTION_01.md)
for Grok Build 4.6 High. Only native_ui.rs is writable. Its six pre-existing
real-widget tests and native.rs authority/capability implementation remain frozen.

The earlier native prerequisite execution (session 20260907_220511_907287,
saved 77521) returned exit 101 before any test ran. Its missing private
ZecReviewDialog/ZecReviewControls contract and obsolete egui Context::run API
are the understood native compile red. The separate signature-context trait,
Option/cloning, fault-type, and borrow prerequisites have since been repaired
and exercised. Accept this classified compile red to author the already
test-specified native contract; do not claim any widget assertion previously ran.
No repeat of the old mixed compile failure is required merely for bookkeeping.
Green, Confirm/close falsifications, native-surface regressions, and compilation
remain reserved to a later bounded Hermes handoff after source review.

The real native-window runner, owning-thread use, and capability bridge still
need separate integration evidence. Full independently recovered effects,
actual-secret cleanup, remaining security, integration, network, broadcast,
mainnet, hardware, and Monero remain unaccepted or parked. No broader acceptance.

Reviewer publication scope: this review, the linked new Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. Pending developer
source/package/locks and all seven evidence records remain uncommitted.
Launch the next actor once and collect only after done/explicit collection.
XHigh remains appropriate; no actor polling or reasoning change is needed.
