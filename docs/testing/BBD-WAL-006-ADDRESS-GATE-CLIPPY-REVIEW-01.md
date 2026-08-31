# BBD-WAL-006 Address Gate Clippy Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `8c509b07`

Result: **SAFE STOP — TWO-LINE CLIPPY CORRECTION REQUIRED**

Luna restarted the gate from the beginning. The fresh formatter check exited 0. The next exact
command, warnings-denied production-library Clippy, exited 1 only at:

- `wallet-broker/src/zec/address.rs:33`: `drop(spending)` — `clippy::drop_non_drop`
- `wallet-broker/src/zec/address.rs:182`: `drop(spending)` — `clippy::drop_non_drop`

Luna stopped before Rust tests, Node policy, evidence, staging, commit, or push and made no repair.

Cached upstream source confirms that `zcash_keys::UnifiedSpendingKey` has no `Drop` implementation;
the explicit calls add no erasure behavior. Removing them leaves each value naturally scoped only
through immediate UFVK derivation and closure return. The accepted security claim remains precise:
the owned mutable seed buffer is observed zero after wipe on every tested exit; derived upstream
key material stays in-process, is never persisted or exposed, and falls out of scope immediately.
BitBook does not claim allocator, register, stack, copy, swap, core-dump, or upstream-type erasure.

No lint suppression is accepted. The correction is exactly deletion of the two explicit
`drop(spending);` statements, with no semantic or additional source change. All execution remains
frozen pending a corrected hash review and fresh gate resume.
