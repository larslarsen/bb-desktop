# WAL-009 signature-context expected red 01 review

Decision: ACCEPT the exact single execution as a mixed prerequisite stop. The
command, diagnostics, runtime, and sixteen-path measurement record are verified,
with the attribution/procedure errata below. Zero tests ran; this is not an
isolated absent-interface red or behavioral acceptance. Close Hermes execution.

Reviewer: Codex at XHigh, baseline
`67f22e0f82ccd19ed645b9d8206bd9b739a17e1e`.
The owner reported done; outer session 24941 was collected once, exit 0.
Hermes session `20260907_232730_9fdc2a` used v0.18.2, provider `nous`, model
`poolside/laguna-s-2.1:free`. Authorization was governance commit `bf39113b`;
the observed HEAD was the launch checkpoint `67f22e0f`.

Evidence: [signature-context expected red 01](BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md),
216 lines, SHA-256
`79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2`.

## Verified execution

Saved message 77602 contains exactly the authorized no-default-features library
test command, with filter zec::spend::verification_context_tests. Message 77603
records exit 101 and the five errors below, plus one unused-mut warning. The
transcript has one such execution, without command additions or rerun.

| Diagnostic | Location at execution | Classification |
| --- | --- | --- |
| E0432, missing ShieldedVerificationContext | verification_context_tests.rs:23 | Intended absent private interface |
| E0277, Authorized lacks TransparentAuthorizingContext | spend.rs:517 | Known signature-hash type prerequisite |
| E0599, cloned on borrowed Option<Vec<u8>> | spend.rs:766 | Known proof Option API prerequisite |
| E0308, PipelineFault passed as FaultPoint | test_support.rs:1326 | Known fault-type prerequisite |
| E0515, returned slice borrows a closure-owned recipient | spend.rs:669 | Known recipient-borrow prerequisite |
| unused-mut warning on PipelineOutcome | test_support.rs:1131 | Known unused binding modifier |

Four tests are declared; zero executed. No additional unexpected diagnostic was
reported, but missing-type errors can suppress later checking. This result does
not prove the test module will compile or its fixture will succeed once the
context exists.

The reviewer parsed the saved terminal JSON and compared the evidence's entire
compiler block with the original output after the stated path replacements.
They match exactly, apart from the newline needed before the closing fence.
The before table matches original measurement message 77599. The actor generated
after measurements from filesystem reads, reported all sixteen matches in 77625,
and rendered the table from parsed data. The reviewer independently confirmed
all sixteen table rows, line counts, and current file hashes. The command and
runtime also match the saved arguments and session metadata.

## Authoritative errata and integration limit

- The runtime section's second "Governance parent" is observed HEAD 67f22e0f;
  the authorization parent is bf39113b, correctly stated in the document header.
- The recorded stat filesystem label ext2/ext3 is the tool's generic family
  label. Reviewer findmnt inspection confirms wallet-broker/target is on ext4;
  /tmp is tmpfs. No substantial build artifact was placed in tmpfs by this gate.
- Read-only Git operations occurred; the completion report's "no Git operations"
  means no Git mutation. Only the new evidence file was added inside the repository.
- Contrary to the narrow retrieval scope, the actor read historical CURRENT_TASK
  content, enumerated the Hermes directory, and queried metadata for other recent
  sessions. An initial SQLite connection omitted uri=True and failed; later
  retrieval used URI read-only mode. No database mutation is inferred.
- Four unapproved scratch files were written under /tmp: cargo_test_output.txt,
  hermes_session_data.json, hermes_evidence_bundle.json, and normalized_output.txt.
  The claim that the new repository evidence was the only added artifact is
  therefore too broad. These were evidence scratch data, not build outputs.
  This review does not claim full procedural compliance or authorize cleanup.
- Local installation/path-prefix strings remain in evidence prose even though
  the compiler block is normalized. The record stays uncommitted. A later explicit
  Hermes integration handoff must remove those local prose paths and apply these
  errata before publication. No immediate evidence retry or gate rerun is needed.

The exact gate, its diagnostic record, and preserved source identities remain
valid despite these deviations. No formatter, other test, lint, scanner, native
launch, dependency operation, source edit, Git mutation, or other actor occurred.

## Next bounded source work

Open only [Grok compile prerequisites 01](../handoff/GROK_BUILD_BBD_WAL_009_COMPILE_PREREQUISITES_01.md).
The existing compiler output is the concrete failing reproduction for these
mechanical API/type repairs: borrow the recipient Option, clone the actual proof
Option once, explicitly map the six fault variants, and remove the unused mut.
The fixes preserve the existing comparison, ownership result, error, and exit
categories. Test source already precedes these corrections and remains frozen.

Keep this slice separate from the security-sensitive signature-context helper and
the native modal path, as fixed by the XHigh scoping review. The missing context
and signature_hash type error remain for a later production handoff. After source
review, choose the next bounded step without automatically rerunning a known
blocked command. Hermes execution/evidence/integration is now closed.

Recovered effects, actual-secret cleanup observations, and native confirmation
remain A3 blockers. In particular, the proof ownership repair is not wipe evidence,
and the explicit fault mapping does not validate synthetic cleanup observations.
XHigh remains appropriate for continuing review. Do not poll the next actor.

Reviewer publication scope: this review, the linked Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
