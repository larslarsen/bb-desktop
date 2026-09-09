# WAL-009 decoded effects validation acceptance

Reviewer: Codex, XHigh, following owner switch on 2026-09-08.
Baseline governance: 7e9bf8b2. ACCEPT the bounded decoded-effects repair and the
six collected validation outcomes. This closes the copied-financial-metadata
finding; it does not accept the complete WAL-009 pipeline or authorize integration.

## Basis and boundary

The source and all 30 frozen identities retain the exact validation baselines.
The new 345-line evidence record remains SHA-256
02d078de8a0269ba2dc41664450965a3eec1036ccb5343db6bb8f90518a64f5a.
The saved command, completion and complete-log identities, exact restored source,
and report/execution deviations are retained in the
[resume collection](../handoff/HERMES_BBD_WAL_009_DECODED_EFFECTS_VALIDATION_RESUME_01.md).
That collection governs the raw execution facts; the actor report is not accepted
as a verbatim log. Preserve the valid outcomes without another report-only task.

The prior understood red accepted ten agreeing false metadata claims. The same
regression is now green and compares positive effects against independent per-action
recovery. Suppressing both receiver comparisons makes exactly the receiver case
incorrectly pass while all other nine still reject; the complete corrected source
was restored before the seven-test library green. Both existing real software and
synthetic-signature pipelines remain green. Native-feature compilation succeeded
with recorded warnings. No expensive check needs repeating for this acceptance.

Source review confirms:
- Actual payment receiver, amount and memo come from authenticated recovery of
  decoded Ironwood outputs under account-bound viewing authority. Internal incoming
  recovery and scope checks establish change ownership. Every action is accounted
  once, with exactly one positive payment and one positive internal change output.
- The fee is the decoded signed Ironwood value balance; bounded checked arithmetic
  enforces retained input equals payment plus change plus fee. Comparisons include
  the exact expected fee and bound, counts, pool presence, version and branch.
- Typed pre-sign validation binds the retained real spend, commitments, nullifier,
  randomized key, anchor and account viewing key. Independently computed decoded
  sighash must match the pre-sign hash. Proof, spend signatures and binding signature
  remain independently verified; complete decode rejects trailing transaction bytes.
- Request/intent/network are checked broker bindings, not falsely described as
  fields recovered from transaction bytes. The public prepared binding is checked
  before verification. Trusted authority contains no copied financial inspection.

This acceptance covers the existing local fixture and constrained two-action route.
It does not claim deferred-anchor operation, live testnet submission, arbitrary
payment shapes, OS-window/owning-thread behavior, full native capability integration,
or complete secret erasure. Existing cancellation, expiry and capability results
retain their previously recorded scope. Real hardware, network/broadcast/mainnet,
Monero and Electron send wiring remain parked.

## Next established finding: invented cleanup observations

The remaining cleanup issue is concrete. test_support.rs:1243 allocates and wipes
an unrelated 32-byte 0x5a buffer for each requested sensitive-class label. The
software error path calls this list even when the signer failed before proof
construction or transaction extraction. Thus positive cleanup events claim that
uncreated proof workspace and extracted transaction bytes were touched and wiped.
SignVerifyWipeObserver also maps events using a caller-supplied class/exit instead
of the actual event label. These are not evidence of actual object cleanup.

spend.rs additionally calls a copied seed buffer derived_authority; wiping that
copy does not establish cleanup of the UnifiedSpendingKey or derived authorizing
key used by Signer. Locally installed zcash_keys 0.16.1 keys.rs and orchard 0.15.5
keys.rs expose no Zeroize/Drop implementation for those key types. pczt 0.9.3 role
APIs consume their owning objects during transitions. Do not use unsafe overwrites,
pretend an ordinary drop zeroized memory, or patch dependency internals without a
separately reviewed contract. Actual broker-owned byte wiping and third-party
object lifetime completion require distinct honest evidence. No broader cleanup
acceptance follows from the next regression alone.

The next source task adds one behavioral regression at the real signer-error path,
with a positive seed-cleanup guard and zero observations for unreached proof,
transaction, signer-view and external-contribution classes across all exit labels.
It needs no new production API and stops before the expensive prover. See the
[exact test-source handoff](../handoff/GROK_BBD_WAL_009_CLEANUP_OBSERVATIONS_TESTS_01.md).
After understood red, production work must remove invented accounting and connect
observations to actual owners; an exit-label filter or deleted assertions cannot
close the cleanup finding. Other accepted source and checks remain frozen.

Implementation and thirteen actor evidence records remain uncommitted. No usable
send flow is delivered. Reviewer publication scope: this acceptance, the new test
handoff, docs/handoff/CURRENT_TASK.md and tickets/BBD-WAL-009.md only.
