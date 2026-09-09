# BBD-WAL-009 Consolidated Diagnostics 01 — Evidence

Actor: Jr Dev — Hermes
Provider/model: nous / poolside/laguna-s-2.1:free
Hermes version: v0.18.2 (2026.7.7.2); upstream 990473a7; local 10b6d1a9 (+1 carried commit)
Python: 3.11.15; OpenAI SDK: 2.24.0

## Preflight

- `hermes --version` — exit 0. Output:
  ```
  Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)
  Install directory: <home>/.hermes/hermes-agent
  Install method: git
  Python: 3.11.15
  OpenAI SDK: 2.24.0
  ```
- `git status --short` — exit 0, clean worktree (empty output).
- `git rev-parse HEAD` — exit 0. Starting HEAD: `cd4d3f04f376f3b560b55a141b0df827c0c07ef4`.

The protected parent is `80a5e8ef` ("docs: authorize owner-requested diagnostics resume"). The
current HEAD `cd4d3f04` is the single allowed CURRENT-only reviewer launch commit on top of it
("docs: record Hermes diagnostics resume launch"). Source checkpoint
`502580fc12bff1789e2ced2a0692b1b35de587a3` is an accepted ancestor. The worktree was clean
before and after all diagnostics. No source, manifest, lockfile, policy, or governance files
were changed.

Billing provider is unset in the session identity; no provider identity is inferred for this
run. Provider/model are recorded from the Hermes --version self-report and the active session
configuration above.

## Diagnostic 1 — cargo fmt --check

Command (run separately, exit code captured):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
```
Exit code: 1

Findings: formatting diffs in 8 files under `<repo>/wallet-broker/src/`:

1. `<repo>/wallet-broker/src/native_ui/zec_native_app_tests.rs:34` — function `take_input`
   should be collapsed to a single line.
2. `<repo>/wallet-broker/src/zec/prepare.rs:9` — import order: `super::store::{AddressAccount,
   PreparedBuild}` should precede `super::{...}` (reordering).
3. `<repo>/wallet-broker/src/zec/prepare.rs:660` — `matches!` macro call with two-arm pattern
   should be reformatted to multi-line.
4. `<repo>/wallet-broker/src/zec/prepare.rs:685` — `artifact.inspection.take().ok_or_else(...)`
   should be reformatted to multi-line.
5. `<repo>/wallet-broker/src/zec/prepare.rs:777` — `inner.handles.get_mut(handle)` chain should
   be collapsed to one line.
6. `<repo>/wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs:3` — import reordering:
   `crate::zec::test_support::...` before `crate::zec::ZecError`.
7. `<repo>/wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs:109` — multi-line
   `assert!(reached, ...)` should be collapsed.
8. `<repo>/wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs:138` — multi-line
   `assert_eq!(...)` should be collapsed to one line.

(Additional diffs in effects.rs, spend.rs, spend/verification_context_tests.rs,
test_support.rs, and tests/zec_sign_verify.rs follow the same reformatting patterns.)

Full output preserved (truncated at ~16,460 chars in the terminal capture; the complete
66,387-character diff was produced by the command run).

## Diagnostic 2 — cargo clippy (no-default-features, --lib)

Command (run separately, exit code captured):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```
Exit code: 101

Findings: 15 errors, all blocking compilation under `-D warnings`:

- `dead_code`: method `matches_review` never used — `src/native.rs:77:19`
- `dead_code`: function `capture_trusted_verification_authority` never used — `src/zec/spend.rs:900:15`
- `dead_code`: function `timestamp_for_test` never used — `src/zec/test_support.rs:1614:4`
- `clippy::collapsible_if`: nested `if` at `src/zec/spend/effects.rs:199:9`
- `clippy::collapsible_if`: nested `if` at `src/zec/spend/effects.rs:216:9`
- `clippy::too_many_arguments` (8/7): `authorize_software` — `src/zec/spend.rs:307:1`
- `clippy::too_many_arguments` (8/7): `authorize_software_for` — `src/zec/spend.rs:342:1`
- `clippy::drop_non_drop`: `drop(usk)` where `usk: UnifiedSpendingKey` — `src/zec/spend.rs:378:9`
- `clippy::redundant_closure`: `|sapling| Ok(sapling)` — `src/zec/spend.rs:606:17`
- `clippy::redundant_closure`: `|orchard| Ok(orchard)` — `src/zec/spend.rs:607:17`
- `clippy::too_many_arguments` (8/7): `review_matches_capability` — `src/zec/spend.rs:1126:1`
- `clippy::too_many_arguments` (8/7): `execute` — `src/zec/test_support.rs:946:5`
- `clippy::needless_return`: `return revalidated.and(...)` — `src/zec/test_support.rs:980:17`
- `clippy::len_without_is_empty`: `ExternalContributions::len` — `src/zec/test_support.rs:3619:5`
- `clippy::redundant_closure`: `|value| prepared_commitment_from_effects(value)` — `src/zec/test_support.rs:4392:39`

## Diagnostic 3 — cargo clippy (native-ui feature, --lib)

Command (run separately, exit code captured):
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib -- -D warnings
```
Exit code: 101

Findings: 16 errors. All 15 errors from Diagnostic 2 are present, plus one additional:

- `dead_code`: fields `confirm` and `cancel` are never read — `src/native_ui.rs:121:5`
  (struct `ZecReviewControls`).

## Diagnostic 4 — node scripts/security-policy.js

Command (run separately, exit code captured):
```
node scripts/security-policy.js
```
Exit code: 1

Findings: single policy failure:
- `package.json must not add runtime dependencies`

This corresponds to the existing CI failure recorded in the handoff: the checkpoint contains
`leaflet ^1.9.4` and `maplibre-gl ^6.8.0` as runtime dependencies in `package.json`, which the
repository policy suite rejects.

## Post-diagnostics source-status check

`git status --short` — exit 0, clean worktree (empty output). No untracked or modified files
after all four diagnostics completed. No source, manifest, lockfile, policy, or governance
changes were introduced.

## Summary

| # | Command | Exit | Result |
|---|---------|------|--------|
| 1 | `cargo fmt --check` | 1 | formatting diffs (8+ files) |
| 2 | `cargo clippy` (no-default) | 101 | 15 clippy errors |
| 3 | `cargo clippy` (native-ui) | 101 | 16 clippy errors (15 + 1 native_ui dead code) |
| 4 | `node scripts/security-policy.js` | 1 | 1 policy failure (runtime deps) |

All four diagnostics ran once, sequentially, with no retries. Exit codes 1, 101, 101, and 1
are ordinary diagnostic data (format/lint/compile/policy nonzero exits) and were collected
without repair or rerun. No tool invocation failures, missing executables, dependency
downloads, or source drift occurred. No functional tests, proofs, source edits, formatter
mutation, dependency changes, network/native/device work, other actors, Git
stage/commit/push, or CI polling were performed.

No functional test or proof execution was performed in this pass — these are formatter,
library lint, and policy diagnostics only.

---
Evidence file identity (measured after write):

- Path: `<repo>/docs/testing/BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md`
- Line count, byte count, and SHA-256 recorded by the measuring step that terminates this
  evidence collection.---

<!-- Archival note: This diagnostic report was collected by Hermes during the 
WAL-009 Rust lint validation 01 pass (commit 253b65a9). The appended 
collected-acceptance appendix below is historical: its then-active source 
authorization for the five-file Grok lint cleanup is closed, and the owner 
mapping decision is now keep MapLibre/remove Leaflet, which is the next 
dependency-policy correction and remains frozen during this Rust checkpoint. 
The appendix supersedes earlier identity/capture prose in the original 
diagnostic record above. -->

## Collected source acceptance — 2026-09-09

ACCEPT the five-file drop for formatting and focused validation, not final green.
Outer 75023 closed with exit 0; saved Grok session
9aa3e29e-8c84-4cad-9d28-333ab7038ec8 confirms grok-4.6 at High. The reviewer inspected
the full diff and reconstructed all five files exactly from the transcript's 15
unique old/new source replacements. No outside source change occurred. The runtime
contains 37 file reads, 16 searches, 15 replacements, two read-only Python identity
measurements and three todo updates; no tests, formatter, Cargo, scanner or Git ran.

Removed the two unused helpers with no callers; gated the test-only authority helper
and control geometry with cfg(test); collapsed two equivalent conditions; preserved
explicit authority arguments through four narrow, reasoned lint allowances; removed
three identity closures and one needless return; supplied is_empty beside len.
The existing dead_code attribute on review_matches_capability was not introduced.
The USK now ends in an inner lexical scope at the same pre-fault boundary, preserving
checks, signing order and pending/hash ownership. Ending ownership does not establish
upstream key erasure. No native confirmation, verification or cleanup invariant was
weakened. No new functional behavior/test design is involved; the observed lint red
is already accepted. Source identities are frozen in the next validation handoff.

Source authorization is closed. Hermes alone may execute the combined formatter,
focused validation and integration in HERMES_BBD_WAL_009_RUST_LINT_VALIDATION_01.md.
High is sufficient for this bounded review and pass. Owner mapping decision is now
keep MapLibre/remove Leaflet; package-policy correction follows separately.
