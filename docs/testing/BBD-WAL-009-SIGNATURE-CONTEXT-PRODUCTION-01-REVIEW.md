# WAL-009 signature-context production 01 review

Decision: ACCEPT the one-file production source drop for later execution.
Close Grok production 01. HOLD execution while Grok makes the single test-order
correction below. Compilation and cryptographic behavior remain unproved.

Reviewer: Codex at XHigh, baseline
`e6913da33bc3ac540c782f2c9cc62ef7d4f032e2`.
The owner reported done; outer session 64906 was collected once, exit 0.
Grok session `d80b2875-cb42-4d53-8b97-926fb3a1026e`, authorized at
`99d7a437`, used CLI grok-4.6 High; its transcript reports grok-4.6-build, high.

## Verified source

wallet-broker/src/zec/spend.rs is 929 lines, SHA-256
`bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`.

Reversing the three saved source replacements in memory reconstructs the entire
883-line starting file and its SHA-256
`d95bea3aa78c326408cd22cabbb4a470ac30a53dea4d0fc34ecf5428310fa536`.
The unified diff contains only the authorized imports, private marker/context,
ownership transfer, and verifier accesses. All sixteen other pending
source/test/package/lock/evidence identities match the preceding measurements.

The private marker projects transparent authorization from pczt::EffectsOnly and
retains Authorized's Sapling/Orchard authorization types. The constructor saves
the library txid, consumes into_data, and rejects every Some transparent bundle.
Its identity closures move Sapling and each Orchard-family domain unchanged through
the pinned mapper. No transaction clone, reserialization, fabricated transparent
context, public access, extra validation, or message override was added.

shielded_sighash digests the retained typed data and passes that same data to the
pinned signature_hash with SignableInput::Shielded. finish_pipeline moves its
independently decoded Transaction into independently_verify. Schema/mainnet checks
remain first; the new transparent guard precedes cryptographic work. The actual
Ironwood action signatures, binding signature, and proof use the same context's
message/data. All later predicates, errors, returned fields, and cleanup sequencing
are preserved, including the redundant transparent-presence predicate.

The actor made no compiler, formatter, test, Cargo/npm, Git, network, evidence,
dependency mutation, or other-actor call. Read scope did exceed the handoff:
repository-wide and registry-root searches and additional transparent/TxId API
reads occurred. The CURRENT_TASK read was correctly limited to 90 lines. These
read-only deviations do not change the verified source result; full procedural
compliance is not claimed.

## Test finding: oracle assertion masks the required signature falsification

At verification_context_tests.rs:346, uses_real_spend_and_binding_signatures
compares the context message to the PCZT oracle before calling either real
signature verifier. Deliberately corrupting the context's message would fail
that equality first. It therefore cannot reach the valid-signature positive
control required by the original falsification contract.

The prior test-source acceptance did not identify this ordering problem.
The real signature/proof tests and oracle remain useful, but their required
falsification readiness is not yet established. This is a source-level finding,
not a claim that any test has executed.

Open only [Grok falsification order 01](../handoff/GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_FALSIFICATION_ORDER_01.md).
Move that one existing equality assertion immediately after the existing valid
binding-signature assertion and before the altered-message block. Keep both
actual spend checks, the two-action counter, binding verification, independent
oracle, all negative cases, and every other byte unchanged. This lets message
corruption fail at the actual spend-signature positive control, while the first
test independently fails its PCZT oracle comparison. No assertion is weakened.

## Continuing limits

XHigh remains appropriate for review; Grok High owns the bounded test correction.
Hermes execution/evidence/integration remains closed until its source review.
Then freeze the focused green and exact falsification mutations/restorations,
followed by the reserved broader regressions. Do not rerun the absent-interface
red or reopen the accepted production adapter.

All pending developer source/package/lock work and four implementation evidence
records stay uncommitted. Earlier evidence errata and local prose-path
normalization remain mandatory at later explicit Hermes integration. The
metadata-copying recovered-effect verifier, synthetic wipe counters, and native
modal/capability behavior remain A3 blockers. This type adapter does not resolve
them. Native, Monero, mainnet, broadcast, and wider work remain parked.
Do not recollect completed actors or poll the next actor.

Reviewer publication scope: this review,
docs/handoff/GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_FALSIFICATION_ORDER_01.md,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
