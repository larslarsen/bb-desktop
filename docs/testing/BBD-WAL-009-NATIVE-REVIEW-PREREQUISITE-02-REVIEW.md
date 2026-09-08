# WAL-009 native prerequisite check 02 review

Result: the exact single execution is ACCEPTED as a prerequisite-blocked stop.
It is not the intended isolated absent-contract red and does not authorize native
production behavior. The actor-authored evidence is NOT ACCEPTED for integration
because its runtime identity, test inventory, hashes, and diagnostic detail need
correction. No further actor, source edit, or execution is authorized here.

Reviewed at `1a36d1d0a222376bf14b00d449e024ca8a93b285`. The owner reported done;
outer terminal session 25473 was collected once and returned exit 0. The actual
Hermes session is `20260907_220511_907287`, Hermes Agent v0.18.2, provider
`nous`, model `poolside/laguna-s-2.1:free`. The execution handoff was published
at `e5418536`; the actor observed the later checkpoint HEAD `1a36d1d0`.

## Execution audit

The terminal call in original transcript message 77520 submits exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::zec_review_tests
```

Message 77521 returns tool exit 101. No suffix, wrapper, rerun, other acceptance
gate, or source edit appears in the tool record. The only following terminal
command records the explicitly allowed postflight hashes and line counts; the
actor then writes the one new evidence file. The initial read-only Git log was
unnecessary; do not equate this acceptance of execution with exact compliance of
every auxiliary read.

The fourteen full hashes in preflight message 77518 and postflight message 77523
are identical. Independent current-file hashes match all fourteen postflight
values. Source, tests, manifests, lockfiles, package files, and earlier evidence
were preserved. Six native widget tests are declared; zero tests executed.

## Observed diagnostic inventory

| Kind | Location | Finding |
| --- | --- | --- |
| E0432 | native_ui/zec_review_tests.rs:5 | Missing private ZecReviewControls and ZecReviewDialog imports; intended absent contract is now observed. |
| E0277 | zec/spend.rs:517 | signature_hash requires TransparentAuthorizingContext; the supplied transparent Authorized type does not implement it. |
| E0599 | zec/spend.rs:766 | cloned is called on a borrowed Option of proof bytes with an incompatible API shape. |
| E0308 | zec/test_support.rs:1326 | wipe_exit_for_fault takes FaultPoint but receives PipelineFault. |
| E0599 | native_ui.rs:117 | egui Context has no run method in the pinned API. |
| E0515 | zec/spend.rs:669 | recipient mapping returns a slice borrowing a closure-owned temporary. |
| unused_mut | zec/test_support.rs:1131 | PipelineOutcome parameter need not be mutable. |

The compiler reports six errors and one warning. This is source compilation
failure, not an observed dependency-resolution failure. Full spans, help, and type
bound notes remain available in transcript message 77521; this inventory is a
review summary. The repaired delimiter no longer blocks parsing. The mixed result
does not prove widget behavior or authorize bypassing any verification mechanism.
Compiler suggestions are not approved repair designs, particularly at signature
hashing and proof/recipient ownership boundaries.

## Evidence defects requiring correction before integration

Actor file: [BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md](BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md),
102 lines, SHA-256
`9938020da28abdc5991b2ae9a5764d7887def7b6e05abe70984fea3e22563a90`.

1. It substitutes the historical routing-policy adoption model for actual runtime,
   misspells the provider as nouse, and omits the session ID. Use the actual session
   metadata stated above; adoption state is explicitly not a runtime pin.
2. Its purported test names are invented. Use the six names in the accepted
   native test-source review or omit names and accurately report the count.
3. Its fourteen-path before/after table truncates hashes with ellipses. Restore
   full captured values from messages 77518 and 77523; do not rerun the test.
4. It summarizes compiler errors rather than preserving the required full relevant
   spans. Recover them from message 77521 and normalize local path prefixes.
5. Its exit description mentions dependency resolution without supporting evidence.
   State source compilation failure. Its blanket no-deviation assertion must also
   distinguish the unnecessary initial Git log from the valid single gate.

The correct six test names remain in
[test-source review 04](BBD-WAL-009-NATIVE-REVIEW-TEST-SOURCE-04-REVIEW.md).
The faulty evidence stays uncommitted and frozen; no correction actor is launched
by this review. The original expected-red run remains separately rejected, and
its corrected record must still accompany its reviewer errata at integration.

## Reviewer reasoning checkpoint

The owner requested a stop when the next task warrants a different reasoning
level. High completed the bounded delimiter/execution audit. Pause now and request
XHigh before scoping repairs to the transaction signature-hash authorization
context and proof/recipient handling, together with the already unresolved native
authority and test-oracle constraints. This is a reviewer judgment about the next
task, not a tool permission or a claim that this run validated those boundaries.

At resume, first bound the evidence-only correction separately from any source
authorization. Do not reopen broad A3 implementation, change dependencies, replace
verification with self-reported flags, or treat a compiler-only repair as security
acceptance. Preserve all pending work. No active actor remains; the owner prohibits
polling or duplicate launches.

Reviewer publication scope: this review, docs/handoff/CURRENT_TASK.md, and
tickets/BBD-WAL-009.md only. Implementation evidence and developer source are
excluded from this governance commit.
