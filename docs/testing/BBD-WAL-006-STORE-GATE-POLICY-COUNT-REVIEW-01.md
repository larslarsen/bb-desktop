# BBD-WAL-006 Store Gate Policy-Count Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Execution parent: `85d4d816ed7edfb8127252eea0ec66d62b2edadb`

Result: **REPOSITORY-SAFE STOP — HANDOFF COUNT CORRECTED TO 68/6**

Hermes v0.18.2 (`nous`, `meituan/longcat-2.0:free`) re-proved the protected parent,
five-path worktree, eight non-source identities, clean index, diff checks, disk-backed filesystem,
and ignored paths. The retained transcript records:

- Rust 1.98.0 formatter: exit 0, no mutation or diagnostic;
- locked/offline/no-default library Clippy with warnings denied: exit 0, no warning;
- locked/offline/no-default `zec_store`: exactly 8 passed, 0 otherwise;
- locked/offline/no-default `zec_address`: exactly 8 passed, 0 otherwise; and
- Node policy: exit 1, exactly 68 `ok`, exactly 6 `not ok`, exact final line
  `6 security policy test(s) failed`, and the same six frozen failure names.

Hermes treated 68 rather than the handoff's required 69 `ok` as the first mismatch and did not
create evidence, edit source/tests, stage, commit, or push. It then exceeded the immediate-stop
boundary by running read-only count/diff diagnostics. During that investigation its provider
stream hung. The reviewer confirmed the worktree/index/parent were unchanged and terminated only
the hung client. This procedural deviation invalidates integration from that run but did not
change repository state or rerun a gate command.

## Count reconciliation

The protected `test/securityPolicy.node.js` is 2,454 lines/SHA-256
`f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647` and contains exactly 74
top-level named tests. It is byte-identical to Support-Dependency Expected-Red Evidence 01, which
correctly recorded 66 `ok` plus 8 `not ok` = 74. Later manifest/support integration made two of
those failures green without adding a test, so the current frozen partial-red count is exactly
68 `ok` plus 6 `not ok` = 74.

The 69/6 claims in Address Gate Evidence 01, its result review, and the earlier store handoffs are
arithmetically inconsistent with the protected 74-test source and are superseded on the `ok`
count only. The six accepted failure names and final summary remain unchanged. Because all 74
named tests emitted a result and the only failures are the exact frozen six, 68/6 is not a new
policy regression.

Hermes must restart the complete gate under a corrected handoff. After any future mismatch it
must make no diagnostic call; it must return the retained result immediately.
