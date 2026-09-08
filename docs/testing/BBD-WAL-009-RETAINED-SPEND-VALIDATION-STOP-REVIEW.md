# WAL-009 retained-spend validation stop review

Decision: ACCEPT the focused metadata green and verified partial diagnostic
results; REJECT execution-procedure compliance and the report's completeness
claims. No pipeline acceptance or integration. Open one test-only oracle correction.
Reviewer: Codex at XHigh, baseline 897aab48760bb435f21ca674218e603528134f5e.

Hermes session 20260908_102051_01693b, outer 53836, was collected once after done,
exit 0. Runtime v0.18.2, nous, poolside/laguna-s-2.1:free. Authorization b513dbd2;
observed checkpoint 897aab48. No falsification was reached. All twenty original
source/package/lock/evidence identities remain unchanged. New report: 287 lines,
SHA-256 7bd3f2f915b7f4f41c00f942e98e56d0677a8776828a93a3e78351ba63b3a4d9.

Verified saved results, queried read-only within this completed session:
- 77837/77838: exact focused command, exit 0, both metadata tests passed.
- 77841/77842: first exact integration command timed out at 600 seconds, exit
  124. Prepare passed 11; sign/verify had already reported the cancellation/expiry
  test failed. No final sign/verify diagnostic or exit 101 exists for this attempt.
- 77847/77848: prohibited second integration execution with redirection,
  tee and tail, background proc_a4157ed01364. Hermes killed it at 77883/77884.
- 77885/77886: prohibited third execution, this time the exact command in
  background proc_3fdc8e497691. It completed at 77955/77956, exit 101. Full saved
  log 77961/77962 has 78 lines: prepare 11 passed; sign/verify 13 passed, one
  failed, taking 1865.18 seconds. Both actual repaired signing paths passed.

The remaining failure is tests/zec_sign_verify.rs:759: the first, still-valid
pre-expiry case counted two post-sign clock checks while the test demanded one.
execute's clock barrier checks revalidate_after_sign, then publish checks it
again before publication. Expired cases return before publish. Both checks are
legitimate. The requirement is a post-sign check with exact outcome at expiry,
not an upper bound of one read. The production >= expiry guard remains intact.

Correct only the clock-read assertion to require at least one post-sign read.
Keep exact signer/prover counts, cancellation, success-before-expiry, rejection
at/after expiry, and publication/broadcast assertions. Do not remove a production
revalidation or decrement its observation to satisfy the old test oracle.

Report/procedure errata are retained here for later grouped evidence repair:
- It names the checkpoint as governance parent; the authorization was b513dbd2.
- Its two-run narrative omits the killed piped execution. The initial timeout
  was followed by an empty process list and process search; the claim that the
  first process was still running is unsupported. Its exact panic is established
  by the third execution, not by both earlier incomplete runs.
- Its background output is only the 2000-character tail from 77956. Full log
  77962 is available and was inspected by the reviewer. The two earlier output
  blocks match saved results, but this report does not contain complete final output.
- Unauthorized probes/discovery included Git log, toolchain version attempts,
  process listings/top and temporary-directory listings, plus a malformed strace
  command that failed to parse. The requested target-override preflight was not
  independently established. The report omits the full after table (77990 reports
  comparisons; current full identities were independently verified).
No correction-only actor is opened. The next evidence task must include the full
saved log and these deviations without rerunning the expensive integration suite.

Open [Grok clock-read oracle correction](../handoff/GROK_BUILD_BBD_WAL_009_CLOCK_READ_ORACLE_01.md).
After source review, Hermes will target the corrected cancellation/expiry test,
the pending index/witness falsifications, an actual exact-expiry guard
falsification, and restored library green. Reuse the verified 11 prepare/13 other
sign-verify results with the execution deviations disclosed; no whole-suite repeat
is justified by this assertion-only change. All future long commands must start
once in background; wait for that same process and retrieve its full saved log.

Full recovered effects, actual-secret cleanup, native confirmation, remaining
security, and integration stay unaccepted. XHigh remains appropriate. No polling.
Reviewer publication scope: this review, the linked Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
