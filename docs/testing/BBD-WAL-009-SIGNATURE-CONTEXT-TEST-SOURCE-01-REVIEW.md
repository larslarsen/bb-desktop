# WAL-009 signature-context test-source review 01

Decision: the two append-only test declarations/accessors are source-accepted.
The new test module requires the bounded correction below before Hermes execution.
No compiler, formatter, test, acceptance command, or integration was performed.

Reviewer: Codex at XHigh, baseline
`69980104c8220ee1f8a4ce5041a145a26e9c2509`.
The owner reported done; outer session 88665 was collected once, exit 0.
Grok session `489bf072-2e08-49ad-a3ac-aa6154f1925c` ran under governance
`0e2125b7`, CLI model `grok-4.6`, reasoning High. Its transcript identifies the
runtime model as `grok-4.6-build`, High; that runtime label is not a CLI model ID.

## Exact source identities

| Path | Lines | SHA-256 |
| --- | --- | --- |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 496 | 0a624b399d1d6b689e7133b6a1a6804c91d39aaa2272c48e6ef212950efd8648 |
| wallet-broker/src/zec/spend.rs | 883 | c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d |
| wallet-broker/src/zec/test_support.rs | 4364 | 6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce |

The reviewer independently hashed the first 880 spend.rs lines and first 4359
test_support.rs lines. They exactly retain the respective authorized prefixes
`ae665e4742925d88e1f7cf6a66172e89a268f613a5fd0efc0b3cc3581ac4cdf4` and
`461fdd070318cc5f31a29af2cdceaab1d0b63dc23473641b61ea9aafc145cd1b`.
The only suffixes are the cfg(test) module declaration and exact cfg(test) root
accessor. The other twelve paths measured in prerequisite 02 remain unchanged.

The actor transcript records only source/API inspection, those three source edits,
and read-only identity checks. No compiler, formatter, tests, Cargo/npm, Git,
network call, evidence write, or other actor was invoked. A broad Markdown search
for the launch checkpoint was unnecessary; the correction handoff limits reads.

## Required corrections

1. At test-module lines 337–339, `actions().first().expect(...)` is invalid.
   orchard 0.15.5 Bundle::actions returns a nonempty 0.11.0 NonEmpty reference;
   NonEmpty::first returns `&T` directly, not Option. Remove only that expect,
   preserving the two-action positive assertion and real signature extraction.
   This is a statically identified new test-source compile defect, distinct from
   the deliberately absent context and existing production compile blockers.
2. At line 259, assert_eq on the complete before/after Ironwood byte buffers
   prints both raw bundles if the preservation assertion fails. This contradicts
   the handoff's no-raw-artifact panic-output contract, including during later
   falsification. Use a boolean byte-equality assertion with a constant diagnostic
   that cannot format either buffer. No test ran and no such dump was observed.
3. The signature-mutation cases at lines 394–440 do not establish that their
   computed message remains the independent original PCZT oracle. A helper that
   incorrectly changes its message when authorization bytes change can satisfy
   those negative assertions for the wrong reason. Require unchanged-message
   controls for each mutation. For the spend mutation, require exactly one
   matching modified signature to fail, the untouched action to pass, and the
   unchanged binding signature to pass. For the binding mutation, require both
   untouched action signatures to pass while the changed binding signature fails.
   These controls isolate actual authorization-byte failures; they add no test
   hook, helper implementation, or claimed production acceptance.
4. Line 1 imports std::io::Write without using it. Transaction::write is an
   inherent method; the module uses Cursor but no Write trait method. Remove
   the unused import as part of the same small source correction.

The rest of the bounded test design is retained: the fixture passes through real
bootstrap/scan/prepare, pinned PCZT signing/proving/finalization/extraction, and
complete independent decoding. The positive message oracle is captured from
Signer before signing. Transparent negatives retain the valid local Ironwood
bundle and explicitly cover four Some shapes, including in-memory empty Some.
The proof negative changes actual proof bytes, requires real proof rejection,
and retains the original message and valid signatures. These are static source
findings; compilation, fixture success, and behavioral results remain unproved.

## Next boundary

Only Grok High may perform [signature-context test correction 01](../handoff/GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_TESTS_CORRECTION_01.md).
The four test names, private future interface, independent fixture/oracle, and
original execution/falsification contract remain fixed. Hermes execution and
integration stay closed. No production context, mechanical compile repair,
native modal work, recovered-effect rewrite, or secret-cleanup repair is open.

The prerequisite-02 evidence remains accepted with its existing errata and
uncommitted integration limitation. All pending source/evidence is preserved.
XHigh remains appropriate for review; High is sufficient for this fixed source
correction. Collect the new actor only after the owner reports completion.

Reviewer publication scope: this review, the linked correction handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
