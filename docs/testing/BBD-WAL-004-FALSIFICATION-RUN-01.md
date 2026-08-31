# BBD-WAL-004 Falsification Run 01

Integrator: Jr Dev — Codex Luna

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance baseline: `a6acb139`

Result: **STOPPED — CASE 1 MUTATION/TARGET PAIRING WAS INCORRECT**

Luna verified the clean production baseline and vault SHA-256
`519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41`, then removed
only `bytes.zeroize();` from `SecretBytes::wipe_with` as authorized. The named
`observed_secret_drop_reports_post_wipe_state_not_predeclared_success` test exited 0.

That result is explained by static path review: the target test exercises the distinct
`SecretBytes::drop` implementation, which contains its own explicit zeroize before its
observer. It does not traverse `wipe_with`; therefore the mutation and target did not
exercise the same mechanism. This is a reviewer handoff error, not evidence that either
zeroization test is vacuous.

Luna immediately restored `wipe_with` by inverse `apply_patch`; the exact vault hash and
clean worktree were recovered. Cases 2–7 did not run. No evidence, staging, commit, or
push occurred. The corrected resume pairs the same mutation with the decrypt panic-
unwind test, which traverses `run_secret_operation`'s guard and `wipe_with`.
