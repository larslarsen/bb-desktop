# WAL-009 clock-read oracle source review

Decision: ACCEPT the test-only correction and open grouped focused validation.
Reviewer: Codex at XHigh, baseline 2d1a48e6. No acceptance command was run here.

Grok session 96a173e5-b8d4-464c-8230-b02bac98b9d7, outer 1158, was collected
once after done, exit 0. Authorization 8d02e3c2; CLI grok-4.6 High, transcript
runtime grok-4.6-build High. Only the two baseline/final hash-and-count commands,
bounded reads, and one source replacement appear; no tests or production edits.

The sole replacement changes the cancellation/expiry test's exact-one post-sign
clock-read assertion to >= 1 and adds its explanatory comment. Reversing the
saved replacement in memory reproduces the exact 1117-line starting hash
2b75901787126ec318bfe481fedd6d388bfe4afa4184cb03ca052447e803518d.
Final wallet-broker/tests/zec_sign_verify.rs: 1118 lines, SHA-256
7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987.
All other twenty pending file identities match their previous baselines,
including production, private context/binding tests, and the prior evidence.

This corrects an overrestrictive observation count. It still requires a post-sign
check and preserves exact signer/prover counts, cancellation, verified success
before expiry, EXPIRED at/after expiry, and no publication/broadcast on rejection.
The exact deadline guard in prepare.rs remains >=; the extra publication-time
revalidation remains in production. Outcome assertions were not reordered.

Open [focused validation resume 01](../handoff/HERMES_BBD_WAL_009_RETAINED_SPEND_VALIDATION_RESUME_01.md).
Run the corrected test green; falsify its actual exact-expiry guard; run the
pending slot-zero and witness-count falsifications; then restored library green.
Each mutation has a separately calculated exact hash and mandatory restoration.
The prior focused metadata 2-pass and verified prepare 11-pass/other sign-verify
13-pass data are retained with the prior execution deviations disclosed. The
whole integration suite is not rerun for this assertion-only change.

Every Cargo command starts once in background, and its same process supplies
completion and the full saved output. No retries, pipes, process kills, global
process discovery, or version probes beyond hermes --version. The same Hermes
turn also corrects the prior report using the exact named completed-session
results. Source repair/integration and broader acceptance remain unauthorized.

Reviewer publication scope: this review, the linked Hermes handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. XHigh remains
appropriate. No actor polling or recollection of completed sessions.
