# Messages UI evidence review 14

Reviewer: Codex. Decision: UI05 failed; accept partial keyboard proof, authorize a
bounded test-driver correction. No tests, syntax checks, builds or Electron executed
by reviewer. HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

## Evidence reviewed

Actor report: [Hermes UI05](BBD-PAY-001-HERMES-UI-VERIFY-05.md).
Retained artifacts: `dist/pay001-ui-verify05/`. Current driver matches its saved copy:
1392 lines, SHA-256 `ace2bddc1286b17ce9a99c4b031ca1ac65765c5417fc9eb9cc35895440e10123`.
All 21 retained before-manifest hashes match the current files; other 20 inputs remain
at review-05 pins. No source change is attributed to this execution.

Raw stderr is `pointer did not close details before focus proof`, matching the wait at
driver lines 1134–1139. The journey reached it after native pointer opening, Enter
closing and Enter reopening, request identity/content assertions, wide/narrow layout
checks and composer draft entry. Bootstrap exact-document setup and renderer sandbox
proof passed (Seccomp 2, NoNewPrivs 1, additional NSpid level). This is useful partial
proof, not complete UI acceptance. The later focus/anchor/node preservation, combined
automatic updates, bottom following, cancellation/expiry, identity and recovery journey
has not passed in this run.

## Findings and bounded decision

1. **Pointer target is not made visible or hit-tested.** clickElement at lines 809–822
   uses the raw bounding-box center even when it lies outside the visible scrollport.
   Neither layoutOk nor transcriptState proves that the target can receive a pointer.
   Resizing changes text wrapping/scroll geometry. Both retained narrow screenshots show
   messages 76–79 with the payment card below the visible transcript. The failed summary
   click therefore lacks a valid input precondition; source plus screenshots support an
   offscreen-target diagnosis. Exact click coordinates were not captured. This is not
   proof of a broken native details control or justification for a product change.
2. **The named card screenshot does not show the card.**
   fixture-request-card-980x720.png is a real 980x654 image but contains only the preceding
   text messages. It is not accepted as populated-card visual evidence. Position the
   card and assert visible content before capture. Preserve long-history overflow and
   the original geometry assertions; never shrink/remove the fixture to fit it all.
3. **Typing POST is rejected by the social fixture.** Diagnostics record POST /ob/chat
   with absent Origin, status 403, and fixture_failures count 1. insertText triggers the
   production input listener/sendTyping (social/app.js:1111); it sends an empty-message
   typing notification. The retained diagnostics omit its body, so that attribution is
   source-based. The actor's claim that this rejection is expected is incorrect: the
   final driver requires zero fixture failures. Review 11's GET-only absent-Origin
   allowance was too narrow for the observed fixture traffic. Permit absent Origin for
   exact allowlisted actual GET/POST methods matching socialMethod; keep absent-Origin
   OPTIONS rejected, supplied Origin validation, path/query/method/preflight checks and
   network confinement intact. Payment authentication and production policy are unchanged.

## Evidence limitations

UI05 again lacks syntax/UI metadata.json, source-after.json, runtime-after.json and raw
preflight output. runtime-before.json contains only counts/comparison booleans, not the
required per-path manifests. The report's claimed before/after verification is not
supported by retained after manifests. Current reviewer hashing establishes only current
identity, not execution-time verification. Syntax exit 0 and UI exit 1 are actor-reported;
raw stderr corroborates UI failure, but exact process exit/timing/timeout evidence was not
retained. Do not reconstruct those facts or rerun unchanged suites to fill the gaps.
All 21 source-before line counts are one too high (trailing newline counted as an extra
line); the report table's counts are correct, including the 1392-line driver.

The missing required preflight files should have stopped UI execution under its handoff.
Future execution must verify those files before spawning and report missing evidence
explicitly. The failed run remains diagnostic evidence, not acceptance evidence.

## Artifact identities

Paths below are relative to dist/pay001-ui-verify05/.

| Artifact | SHA-256 |
| --- | --- |
| ui.stderr.log | c3306846dff29ca6b6590347b2adeeb61a17e53d7c50afe1531d3dda161cecc6 |
| source-before.json | 7bacf4479117736f94f7afd65064362b549706b4cb721440ab7c7211bb5c5046 |
| ui-01/bootstrap-diagnostics.json | 049f413f55752ce831656ae49adf01a842b208f1eba468e61201a3632eea77bc |
| ui-01/sandbox-status.json | 8e65de58a1d07b4d705e3cfc40e8a8a16ef6744e2d6af5e9355a47a55e3ec05c |
| ui-01/ui-failure-diagnostics.json | 0393b7a16d8b097455f612d4de58c4286f30e26f63265cf798bcdd6c8e802217 |
| ui-01/fixture-empty-messages-1440x1000.png | 823a825e90b8f6553857416fac68ae7c1394db3f125768229d44daecbf1c25ee |
| ui-01/fixture-request-card-980x720.png | be2f05f8cb0ae21d8a0e034e4decd6fd0d088bfba3e0a3ffdadfc4591d7969eb |
| ui-01/fixture-failure.png | dfa8aed91ea715f6ebaef80f346ef2870ca689b0177e8bc1f996ff5f2748f03e |

PNG dimensions verified from bytes: empty 1440x934; both narrow images 980x654.
Data is isolated test-fixture data, not owner payments.

## Next authority

[Sol pointer/fixture correction 01](../handoff/SOL_BBD_PAY_001_MESSAGES_UI_POINTER_01.md)
is the sole active source authority, using the documented Grok-usage-exhaustion fallback.
Hermes UI05 is closed, with no unchanged rerun. Reviewer inspects the corrected driver
before authorizing fresh syntax/UI execution. Prior Node results stand. No product edits,
scans, publication or user-process restart. Identity/WebRTC/video direction is unchanged.

Governance paths: this review, new Sol handoff, closed Hermes UI05 handoff, CURRENT_TASK,
PAY-001 ticket and end-to-end status map.
