# WAL-009 signature-context falsification order 01 review

Decision: ACCEPT the exact one-assertion reorder. Close Grok source work and open
only the Hermes signature-context validation handoff below. Neither compilation
nor test behavior is accepted yet.

Reviewer: Codex at XHigh, baseline
`a78df8d979fc39bb3e0ece5d9d0cebc9b254b3e7`.
The owner reported done; outer session 96396 was collected once, exit 0.
Grok session `b1a63fe8-d204-4f7e-9af0-201648a936be`, authorized at
`94875511`, used CLI grok-4.6 High; saved runtime is grok-4.6-build, high.

## Exact accepted change

verification_context_tests.rs remains 528 lines and now has SHA-256
`45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a`,
exactly the identity proposed in the handoff. Reversing the saved replacement in
memory reconstructs the entire preceding hash
`78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec`.

The diff only moves the existing message-to-PCZT equality after the valid
spend/binding signature assertions and before altered-message tests. Every check
and all other bytes are preserved. Message corruption can now reach the real
spend-signature positive control before the oracle equality. The separate first
test retains its PCZT-oracle comparison.

All sixteen other pending file identities are unchanged. In particular spend.rs
is frozen at 929 lines, SHA-256
`bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361`,
and test_support.rs at 4371 lines, SHA-256
`fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920`.

The saved tool calls show one test-source replacement, bounded reads, and two
hash/line-count commands. No compiler, formatter, test, Cargo/npm, Git, network,
evidence, dependency operation, or other actor ran. No broader source mutation
appears in the reconstructed diff or current measurements.

## Validation authorization

Open [Hermes signature-context validation 01](../handoff/HERMES_BBD_WAL_009_SIGNATURE_CONTEXT_VALIDATION_01.md).
It freezes six sequential test commands, with advancement conditional on each
required result: focused green, transparent-rejection falsification, two distinct
message-corruption failures, then restored-source integration and library tests.
An unexpected result stops execution; an active mutation must first be restored
using the exact bounded reversal. Expected assertion failures are required
falsification results, not unexpected gate failures.

The reviewer computed both proposed mutation identities and verified unique
reversible replacements in memory without writing production source or running
tests. Hermes may apply only those mechanical test-falsification replacements.
No test design, production repair, formatting, dependency change, or integration
is authorized. The final library command also verifies restored focused tests;
an additional restored focused rerun is unnecessary.

Command spelling now uses the quoted HOME route to the existing pinned rustup
executable, avoiding a machine-specific absolute path in new published records.
The Rust 1.98.0 toolchain and original focused/broader Cargo arguments are preserved;
individual falsification commands add only their exact test selection.
Reviewer filesystem inspection confirms target storage remains ext4. The stat
ext2/ext3 family label is not a different filesystem. Temporary storage is tmpfs.

The new evidence must be generated from actual captured tool data, with complete
diagnostics/results, full before/after identities, runtime, all executed and
unrun stages, and mutation/restoration measurements. It remains uncommitted.
Prior evidence errata and local prose-path normalization remain deferred to an
explicit later Hermes integration contract; earlier actors stay closed.

XHigh remains appropriate for review. Full recovered effects, actual secret
cleanup, and native confirmation remain A3 blockers; the metadata-copying
verifier and synthetic wipe counters stay unaccepted even if this validation
passes. Native, mainnet, broadcast, Monero, network, security, and wider gates
remain separately pending. Do not poll Hermes.

Reviewer publication scope: this review,
docs/handoff/HERMES_BBD_WAL_009_SIGNATURE_CONTEXT_VALIDATION_01.md,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
