# WAL-009 signature-context test correction 01 review

Decision: ACCEPT the corrected four-test source and close Grok's correction.
Authorize only the single targeted Hermes expected-red attempt linked below.
Compilation and behavior remain unproved; source acceptance is not test acceptance.

Reviewer: Codex at XHigh, baseline
`c326687a0a63aeb642bc93d470b77430cbc99d7b`.
The owner reported done. Outer session 27805 was collected once, exit 0.
Grok session `89408f90-bfc2-4d50-a3a2-9895844f7c37`, governance `2280f01e`,
used CLI model `grok-4.6` at High; its transcript identifies runtime
`grok-4.6-build` at High. Both completed Grok source tasks are closed.

## Source identity and correction verification

| Path | Lines | SHA-256 |
| --- | --- | --- |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec |
| wallet-broker/src/zec/spend.rs | 883 | c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d |
| wallet-broker/src/zec/test_support.rs | 4364 | 6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce |

The reviewer reconstructed the original test module from the first Grok session's
saved write/replacement arguments and obtained the exact authorized starting hash
`0a624b399d1d6b689e7133b6a1a6804c91d39aaa2272c48e6ef212950efd8648`.
The byte diff is confined to the four authorized corrections:

- Cursor is imported alone, and the NonEmpty::first call uses its direct Action
  reference without an invalid expect.
- The complete zeroizing bundle buffers still compare byte for byte. Failure
  uses a constant message without formatting either raw buffer.
- Each authorization mutation requires its actual computed message to equal the
  independently captured PCZT oracle. Verification still uses the actual message.
- The spend mutation requires exactly one matching changed signature to fail,
  exactly one untouched action to pass, and the unchanged binding signature to
  pass. The binding mutation requires both untouched actions to pass and the
  changed binding signature to fail. Complete decoding and unique-byte mutation
  remain intact.

The four test names, fixture/oracle construction, transparent cases, proof test,
and absent future production context/marker are unchanged. Both accepted append-only
source files retain their previous identities and original production prefixes.
The other twelve paths measured in prerequisite 02 remain unchanged.

The correction transcript records only this one source file being edited. No
compiler, formatter, tests, Cargo/npm, Git, evidence, network call, or new actor
was invoked. Read scope did exceed the handoff: the actor read a roles document
and attempted a home-wide filename search for pinned crates, then terminated
that search and used the known Cargo registry. This read-scope deviation is
recorded; it does not invalidate the verified bounded source diff. The next
Hermes task permits no filesystem/session discovery search.

## Next boundary and remaining limits

Only [Hermes signature-context expected red 01](../handoff/HERMES_BBD_WAL_009_SIGNATURE_CONTEXT_EXPECTED_RED_01.md)
may run. The exact no-default-features library filter targets these four tests
without native-ui. Missing ShieldedVerificationContext is intentional. The known
spend.rs authorizing-context, borrowed Option/proof, and recipient lifetime errors,
plus test_support.rs's fault-type error and unused-mut warning, are still present.
If these appear, the result is prerequisite-blocked, not isolated behavioral red.
Unexpected new diagnostics require reviewer triage; no repair or rerun is open.

The pending source/lock/evidence remains uncommitted. Hermes may create only its
new result record, mechanically copying command/output/hash data and recording
actual runtime identity. It may not integrate, change production, or repair prior
evidence. The existing prerequisite-02 evidence errata and integration limitation
remain in force. Independent recovered effects, actual-secret cleanup observations,
and native modal/capability behavior remain A3 blockers.

No reviewer compiler or acceptance command was run. XHigh remains appropriate for
the continuing security review. Do not poll Hermes; collect once after the owner
reports completion.

Reviewer publication scope: this review, the linked Hermes handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
