# WAL-009 prerequisite 02 evidence correction review

Decision: accept the recovered command, runtime, hash table, and prerequisite-stop
record with the authoritative errata below. Close the correction task. This does
not accept its claims of verbatim transcription or full procedural compliance.
No compiler rerun or another documentation retry is required for source scoping.

Reviewed by Codex at XHigh against governance HEAD
`4f433042fa438b4eeffa242228242d1af22bc762`. The owner reported done; outer
session 6932 was collected once and returned exit 0. Correction session
`20260907_222229_2bc84d` used Hermes v0.18.2, provider `nous`, model
`poolside/laguna-s-2.1:free`, under the `09cc3d5b` authorization. The original
execution is separately attributed to session `20260907_220511_907287`.

Corrected record: [native prerequisite 02](BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md),
181 lines, SHA-256
`33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2`.

The reviewer independently parsed the original terminal-result JSON from messages
77518 and 77523 using SQLite URI read-only mode. All fourteen full before/after
hashes are identical, match the corrected table, and match the current files.
Session metadata confirms both runtime provider/model attributions. Original
message 77521 records exit 101, six errors, one warning, and zero executed tests.
The six declared native tests are distinguished correctly from that executed count.
The original exact one-command execution remains a valid mixed prerequisite stop,
not an isolated missing-contract red and not evidence of behavior passing.

## Authoritative errata and integration limitation

- The diagnostic excerpts are transcriptions, not verbatim recovered output.
  Caret spans and annotations differ, including the EffectsOnly trait span, the
  signature_hash bound note, and the wipe_exit_for_fault signature annotation.
  Original message 77521 remains authoritative for exact diagnostics. All six
  error identities and the warning are represented; no changed caret is evidence
  of a source repair.
- Local registry and build-artifact paths remain in the evidence despite its
  normalization statement. The evidence must remain uncommitted. A later explicit
  Hermes integration handoff must mechanically render and fence the original
  diagnostic output with local prefixes normalized, then record the resulting
  identity. AGENTS.md prohibits publishing local absolute paths. This review may
  be published independently and must accompany that evidence at integration.
- The hash values are correct, but the claim that none was retyped is not
  established by the correction procedure: the actor's comparison code contains
  manually supplied copies of the hash output. The independent parsed comparison
  above establishes their correctness without endorsing that procedure.
- Read-only Git operations occurred; the blanket phrase "no Git operation"
  means no Git mutation in this accepted record. The evidence file itself changed;
  its final "bytes unchanged" phrase applies only to the preserved source and
  fourteen original measured paths, not to this rewritten evidence file.
- The correction made unnecessary read-only Git/process/file-discovery and
  unrelated session-metadata reads. Its SQLite connections were not opened in
  the required URI read-only mode, although the recorded SQL consists of reads
  and schema inspection. No database mutation is inferred. Exact compliance with
  the narrow retrieval procedure is not accepted.

The correction transcript contains no compiler, Cargo, rustup, test, formatter,
source edit, Git mutation, or new actor launch. Only the named evidence file
changed. These errata close the current correction without changing the original
execution classification or opening integration.

## Next source boundary

Open only [Grok signature-context tests 01](../handoff/GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_TESTS_01.md).
The [XHigh scoping review](BBD-WAL-009-PREREQUISITE-02-XHIGH-SCOPING-REVIEW.md)
fixes the future move-only authorization adapter. The new handoff freezes its
private test interface, a real local-fixture PCZT oracle, and transparent-bundle
rejection falsification. No production implementation or execution is open.

The test-only root accessor reuses existing fixture-state allocation; it does not
export a transaction or widen a production API. The pinned primitives crate's
public test-data module is available through the already enabled sqlite
test-dependencies feature. Its frozen testnet vector supplies hostile transparent
bundle values through the library decoder, avoiding a new dependency or codec.
That vector is never submitted or treated as a valid local payment.

Independent recovered-effect comparison, observation of actual secret cleanup,
and the native modal/capability path remain blockers. Signature-context tests
will not accept the current metadata-copying verifier or canary-only wipe counters.
XHigh remains appropriate for source/security review; no reasoning change is
requested at this boundary. Collect the next actor only when the owner says done.

Reviewer publication scope: this review, the linked Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. All source and
implementation evidence remain excluded.
