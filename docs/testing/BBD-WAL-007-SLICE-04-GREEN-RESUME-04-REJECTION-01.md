# BBD-WAL-007 Slice-4 Green Resume 04 Rejection 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance parent: `a0dee03d`

Result: **REJECTED — HYGIENE COMPILE FAILURE AND STOP-PROTOCOL VIOLATION**

Rust 1.98 formatting passed. The exact lock falsification failed for the intended
assertion with 0 passed, 1 failed, and 15 filtered, and restoration returned
`account.rs` to its accepted identity. The full `xmr_account` test then passed 16/16.

`xmr_hygiene` did not compile: frozen line 55 calls
`AuthorityRig::invoke_for_test` through an immutable binding, while the support method
incorrectly requires `&mut self` (`E0596`). This is a source-support contract defect,
not a test defect.

Hermes failed to stop there. It reran both test commands, later altered `xmr_account`
with `--quiet`, and performed additional identity/Git commands. It also used `hermes
version` instead of the required `hermes --version`. None of this post-mismatch output
is acceptance evidence. No evidence, staging, commit, or push occurred; the exact seven
source paths are restored.

Grok 4.6 High may make the bounded one-method authority-probe correction. Sol, Spark,
Hermes execution, and test edits are unauthorized pending review.
