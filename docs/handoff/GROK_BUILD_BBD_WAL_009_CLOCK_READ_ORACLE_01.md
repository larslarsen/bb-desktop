# WAL-009 clock-read oracle correction 01

Actor: Grok Build, CLI grok-4.6, High. Governance parent: this handoff's commit.
One test-only correction. No production change or execution is authorized.

Read AGENTS.md, TESTING.md, the active leading CURRENT_TASK.md section, this
handoff, and docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-STOP-REVIEW.md.
Read only the named cancellation/expiry test below and the relevant execute/publish
clock-barrier branches in test_support.rs, plus prepare.rs::revalidate_after_sign.
These paths are already known; no filesystem/dependency discovery or history reload.

Only writable path: wallet-broker/tests/zec_sign_verify.rs, 1117 lines, SHA-256
2b75901787126ec318bfe481fedd6d388bfe4afa4184cb03ca052447e803518d.
Verify that complete baseline before editing. Frozen related baselines:

- src/zec/test_support.rs: 4369 lines,
  c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159.
- src/zec/prepare.rs: 1238 lines,
  44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07.
- src/zec/spend.rs: 1046 lines,
  8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a.

The three src paths above are under wallet-broker. All other source/tests, fixtures,
context implementation/tests, dependencies, evidence, and governance remain frozen.

In cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries, replace
only the assertion that post_sign_clock_reads == 1 with an assertion requiring
post_sign_clock_reads >= 1. Add a short adjacent comment: the valid path may
recheck before publication; the outcome cases enforce the exact expiry boundary.
No assertion reordering, helper extraction, loops, new cases, or unrelated formatting.

Preserve the strict assertions for one signer and prover call, cancellation,
zero verified/broadcast publication on cancellation or expiry, verified success
just before expiry, and EXPIRED at and after expiry. Keep every other test byte
unchanged. Do not alter production checks or counters, substitute exact two reads,
remove the positive-read requirement, or turn it into a source-text assertion.

The understood red is saved final result 77956/full log 77962 in completed Hermes
session 20260908_102051_01693b: the first pre-expiry case observes two legitimate
checks, failing only the old exact-one assertion at line 759. Read the reviewer
summary, not that database. No repeat red or test execution is authorized to Grok.

After review, Hermes alone will receive the focused green command:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify cancellation_and_expiry_are_reread_after_proving_at_exact_boundaries -- --exact
```

Later validation will also falsify the actual expiry boundary by changing only
prepare.rs::revalidate_after_sign's now >= expires_at comparison to now > expires_at.
The same corrected test must reject success exactly at expiry, then the original
production hash must be restored. That future temporary mutation is NOT authorized
to Grok. The two existing index/witness falsifications and restored library green
remain pending. No whole integration-suite repeat is required for this test-only fix.

No tests/compiler/formatter, Cargo/npm, Git, evidence, integration, native launch,
network, secret output, or subagent. Report the one final hash/line count, exact
semantic correction, and unchanged related source identities, then stop.
Reviewer collects once after done or explicit collection; no polling.
