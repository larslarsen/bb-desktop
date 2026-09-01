# BBD-WAL-006 Prepare NFC Dependency Gate Evidence Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Integrated gate commit: `9cdaa562550f4dc898b09411fb92d820fa64501f`

Result: **COMMANDS AND INTEGRATION CORRECT — EVIDENCE CORRECTION REQUIRED**

The five-path integration is exact. `HEAD` equals `origin/master`, the worktree/index are clean,
and the complete lock diff adds only the broker's direct `unicode-normalization` edge and the
single crates.io 0.1.25 package block with accepted checksum and existing `tinyvec` dependency.
The four command results match the gate: check 0, feature tree 0, policy 69/6 expected partial red,
and custody 11/0.

The evidence has one identity error. Hermes reported provider
`meituan/longcat-2.0:free`; the actual invocation and terminal report identify provider `nous` and
model `meituan/longcat-2.0:free`. The evidence must correct only that field.

The correction should also finish the already required resolved-package statement from inspected
published metadata: `unicode-normalization 0.1.25` has `build = false`, Rust version 1.36, and
license expression `MIT OR Apache-2.0`, which is within the repository's existing allowlist. This
does not change the accepted command result or authorize another command.

No test, Cargo, Node, formatter, source, lock, manifest, policy, fixture, or network action may be
rerun or changed. The gate is not yet final acceptance evidence until Hermes integrates the exact
documentation correction.

