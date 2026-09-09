# WAL-009 bounded Rust lint cleanup 01

Reviewer: Codex; High is sufficient. Source actor: Sr Dev — Grok Build,
grok-4.6 at High, no subagents. Execution/integration actor: none in this phase.
Protected parent: reviewer commit publishing this handoff, with one subsequent
CURRENT-only reviewer launch checkpoint allowed. Source bytes are those integrated
at 502580fc12bff1789e2ced2a0692b1b35de587a3 and identified below.

## Accepted diagnostic red and scope

Hermes diagnostics are accepted as actual findings: no-default library Clippy
exited 101 with 15 errors; native-ui library Clippy exited 101 with those same 15
plus unused native control geometry. Formatter and security-policy diagnostics
also failed, and remain separate findings. No functional signing test was repeated.
This task resolves all 16 reported Rust lint findings together, without changing
wallet behavior or the accepted authority, verification or cleanup contract.

These are mechanical cleanup changes against an already observed lint failure,
not a new wallet behavior requiring another functional test. Do not add tests
that duplicate existing coverage or redesign argument ownership to satisfy a style
threshold. Source is reviewed before any execution. If a requested change would
require a semantic deviation, stop and report the specific conflict.

Read AGENTS.md, TESTING.md, this handoff, CURRENT active prefix, and ticket active
prefix at tickets/BBD-WAL-009.md. Use bounded source reads to verify callers and
nearby ownership. No historical record reload. Verify the five exact inputs below
before editing. The only allowed source paths are the five rows of that table.
All test source, other production source, manifests, locks, dependencies, policy,
workflows, documentation/evidence and old target backups are frozen.

## Exact corrections

1. native.rs: delete only the unused crate-private
   ZecConfirmationCapability::matches_review method. Repository caller inspection
   finds no call. Preserve mint, consume, nonce/replay enforcement, all getters,
   native-origin enforcement and all actual capability-binding checks. In particular
   do not remove or weaken spend::review_matches_capability.
2. native_ui.rs: add #[cfg(test)] to each of ZecReviewControls' confirm/cancel fields
   and to the corresponding two field initializers returned by ZecReviewDialog::ui.
   Non-test builds retain an empty control-geometry return struct; unit tests retain
   the same two rectangles. Button creation, click/close/cancel precedence, state
   transitions and native event-loop behavior are unchanged. No dead-code allowance.
3. spend/effects.rs: collapse only the two nested if-let/!same_recovered_note checks
   into equivalent let-chain conditions. Preserve short-circuiting, all comparison
   arguments, and INTENT_MISMATCH returns. No other verification logic changes.
4. spend.rs: apply only these narrowly specified changes:
   - Add a function-level clippy::too_many_arguments allowance to authorize_software,
     authorize_software_for and review_matches_capability, with a reason explaining
     that reviewed authority/verification/fault inputs deliberately remain explicit.
     This style exception is restricted to those functions; no crate/module-wide
     lint suppression, signature change, input bundling or warning-gate relaxation.
   - Add #[cfg(test)] to capture_trusted_verification_authority, used only by the
     existing verification_context_tests unit module. Its implementation is unchanged.
   - Replace the two identity closures |sapling| Ok(sapling) and |orchard| Ok(orchard)
     in ShieldedVerificationContext::new with Ok. Preserve generic/error types and
     the transparent-bundle rejection closure.
   - Replace drop(usk) with an inner lexical scope ending at exactly the same point,
     before the existing injected Signer-failure branch. Within the existing
     let (signed_pczt, authority) block, derive/sign inside
     let (signed, pending, pre_sign_sighash) = { ... }; return that tuple from the
     new inner block. Keep the Signer-failure check and the existing
     (signed, pending.with_pre_sign_hash(pre_sign_sighash)) after it, unchanged.
     The unified key and temporary signing authority must not escape the inner
     block. Preserve every seed/FVK/PCZT check, signer call, counter, error mapping,
     fault location and final finish_pipeline call in their original order.
     Add a short truthful comment: this ends ownership, not guaranteed upstream
     memory erasure. Do not add zeroization claims, fake observations, serialization,
     extra key copies, unsafe code or a drop_non_drop allowance.
5. zec/test_support.rs:
   - Add the same narrowly justified function-level too_many_arguments allowance
     to SignVerifyHarness::execute; preserve its signature and call sites.
   - Change only the CancelAfterSignAndProof arm's final
     return revalidated.and(Err(ZecError::cancelled())); to the same tail expression
     without return/semicolon. Preserve revalidation and finish_failed_pipeline order.
   - Delete the unused timestamp_for_test free function only. It has no callers;
     do not change the injected clock, expiry parsing or any active time check.
   - Add pub fn is_empty(&self) -> bool { self.entries.is_empty() } beside len in
     ExternalContributions. Do not change entries, route, visibility of fields,
     contribution validation or replay behavior.
   - Replace only .map(|value| prepared_commitment_from_effects(value)) with
     .map(prepared_commitment_from_effects).

The enumerated changes are exhaustive. Do not manually reformat whole files or
make additional lint fixes based on guesses. Local formatting of changed constructs
is allowed. The subsequent Hermes validation contract will batch the pinned
mechanical formatter with the focused lint checks; this source task runs none of
them. No formatter/compiler/Cargo/test/scanner/product/network/Git/actor command.
Read-only source inspection and Python/hash/line measurements are allowed. No
helper files, scratch listings, backup writes or evidence updates.

Preserve all existing tests and prior results. Full typed-secret erasure remains
an open design/release requirement; a Clippy cleanup does not satisfy it. Package
mapping dependencies and repository security policy are untouched pending their
separate scope/provenance decision. No broadcast, mainnet, XMR, hardware, executable
or Electron behavior is added.

## Frozen source inputs

| Path | Lines | SHA-256 |
| --- | --- | --- |
| wallet-broker/src/native.rs | 489 | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |
| wallet-broker/src/native_ui.rs | 321 | d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 |
| wallet-broker/src/zec/spend.rs | 1160 | 9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9 |
| wallet-broker/src/zec/spend/effects.rs | 379 | cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035 |
| wallet-broker/src/zec/test_support.rs | 4479 | 255600063e348a54654cd5923496a9bba5e2dfa447222997c6bfc79b2d592c27 |

## Stop and report

After the five-path drop, report exact paths, line counts, SHA-256, which listed
corrections were made and any unresolved concern. State that no execution or Git
occurred. Stop for reviewer source acceptance; do not run a formatter or probe
Clippy, author evidence, integrate, delegate or extend the task.

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
