# WAL-009 native review evidence correction 01 review

Result: the corrected command/result record is ACCEPTED as rejected-run
documentation, with the authoritative errata below. The original execution remains
REJECTED. This closes the documentation-only task without another execution or
documentation retry; this review must accompany the evidence at later integration.

Reviewed at `e1e3966d64c514d0c6a64593d5a1ab0d851e12fc` after the owner reported
completion. Outer session 98069 returned exit 0 on one collection. The correction
transcript is Hermes session `20260907_212200_134a07`, provider `nous`, model
`poolside/laguna-s-2.1:free`, launched from `13872efa`. The original execution
remains separately attributed to `20260907_211536_310238` at `7afe2800`.

Corrected evidence: [BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md](BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md),
199 lines, SHA-256
`42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa`.
The completion report's identity matches the actual file.

The evidence now accurately distinguishes the authorized command from the two
actual strings, discloses the prohibited redirection/rerun, and distinguishes the
first tool exit 101 from the second tool exit 0 after echo printed EXIT_CODE=101.
Both actual strings match the original saved tool arguments. The diagnostic,
six-declared/zero-executed count, corrected manifest line count/native hash, and
absence of an original lock-evidence hash measurement are accurately recorded.

## Authoritative errata

The evidence's read-scope paragraph misidentifies an authorized source-review read
as forbidden and attributes it to the wrong message. The initial-red handoff
explicitly required BBD-WAL-009-NATIVE-REVIEW-TEST-SOURCE-04-REVIEW.md. That read
was authorized and appears in original messages 77411 and 77421, not 77436.
The actual out-of-scope historical reads include the A1 expected-red acceptance,
A1 test-source review, transaction-cleanup review, lock-sync evidence, and A3
Grok-correction review in messages 77411, 77417, and 77421. This paragraph
supersedes the evidence's read-scope paragraph and the completion report's claim.

The completion report also says the correction added a previously omitted
Cargo.lock row and fixed a malformed native hash in both original tables. In
fact Cargo.lock was already present, and the malformed hash was confined to the
original preservation table. These completion-report claims are superseded here.
The final table values are correct. The prose cargo identity describes a repository
directory imprecisely; the original tool output identified the Rust 1.98.0 cargo
binary, and the exact recorded rustup command is the execution authority.

The correction transcript shows only read/search/record-edit/hash/status operations;
no Cargo, rustup, compiler, formatter, test, source edit, or Git mutation occurred.
It did include unnecessary read-only Git log/file-list/status calls beyond the
narrow record/diff verification scope. Exact procedural compliance is not claimed.
These details do not alter the valid rejection or the preserved syntax diagnostic.

## Next boundary

Grok alone may perform the [single-delimiter correction](../handoff/GROK_BUILD_BBD_WAL_009_HARNESS_DELIMITER_05.md).
The prerequisite is an unclosed SignVerifyHarness impl ending at line 4358.
Appending its closing brace is a syntax-only repair, not acceptance of the harness,
its cleanup observations, its transaction-effect oracle, or native authorization.
The reviewer found the existing helper definitions earlier in the same module;
an earlier conversational suggestion that they were absent was corrected and is
not a source finding. No new helper implementation is authorized.

Hermes execution and integration remain closed. The accepted native regression
tests, lockfile/evidence, and all other pending work are preserved. The corrected
evidence remains uncommitted for later Hermes integration together with this review.
High is sufficient for this exact source boundary; reassess reasoning level before
the native authorization and test-oracle review as the owner requested.

Reviewer publication scope: this review, the Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
