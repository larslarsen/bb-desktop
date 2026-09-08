# WAL-009 signature-context test correction 01

Actor: Grok Build, CLI model `grok-4.6`, reasoning High. Governance parent: the
commit containing this handoff. This is a one-file test-source correction.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's leading active section, this handoff,
and docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-TEST-SOURCE-01-REVIEW.md. The original
GROK_BUILD_BBD_WAL_009_SIGNATURE_CONTEXT_TESTS_01.md retains the private interface,
fixture/oracle, four tests, and future execution/falsification contract. Do not
reload historical records or repeat broad source/dependency searches.

## Exact path and baseline

Only writable path: wallet-broker/src/zec/spend/verification_context_tests.rs.
Starting identity: 496 lines, SHA-256
`0a624b399d1d6b689e7133b6a1a6804c91d39aaa2272c48e6ef212950efd8648`.
Stop without editing if it differs.

The accepted append-only paths are read-only:

- wallet-broker/src/zec/spend.rs: 883 lines, SHA-256
  `c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d`;
- wallet-broker/src/zec/test_support.rs: 4364 lines, SHA-256
  `6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce`.

All other source, tests, fixtures, dependencies, manifests, locks, evidence,
governance, and pending package work remain frozen. Do not append either accepted
declaration/accessor again. Leave the future production context/marker absent.

## Exact corrections

1. Change the std::io import to import Cursor alone; remove unused Write.
2. In the original signature snapshot, use actions().first().authorization()
   directly. Remove the invalid expect on NonEmpty::first and preserve the
   two-action assertion and pinned byte conversion. The pinned NonEmpty 0.11.0
   API returns &T. Do not substitute a fake action or change the selected bytes.
3. Replace the raw before/after bundle assert_eq with a boolean slice-equality
   assertion and a constant explanatory message. Neither failure formatting nor
   added diagnostics may include bundle, transaction, proof, signature, or other
   raw artifact bytes. Keep the complete byte comparison and both zeroizing
   buffers; do not reduce this to length/hash-only equality.
4. Strengthen only the two real-signature mutation cases in
   uses_real_spend_and_binding_signatures:
   - After obtaining each mutated context message, require byte equality to the
     independently captured fixture.pczt_sighash, using a boolean assertion and
     constant message. Never replace the actual message supplied to verification
     with the expected oracle merely to make the test pass.
   - For the spend mutation, retain the exact serialized-signature match against
     mutated_sig. Count exactly one matching action whose real verifier returns
     Err, and exactly one untouched action whose real verifier returns Ok.
     Verify the untouched binding signature against the same actual message and
     require Ok. Preserve the two-action assertion and unique-byte mutation.
   - For the binding mutation, verify both actual untouched action signatures
     against the actual binding_context message, require Ok, and assert the
     exact count is two. Preserve the existing real binding verifier Err result,
     complete decode, two-action assertion, and unique-byte mutation.

Keep the four existing test names, fixture helper, captured PCZT oracle, positive
signature/wrong-message controls, four transparent cases, proof test, and future
private interface. No new test, production implementation, synthetic verification
flag, source-text assertion, or broader cleanup rewrite is authorized. Preserve
unrelated text; ordinary formatting of the directly edited expressions is allowed,
but no formatter invocation or whole-file reformatting.

Read only this module and, if needed, the exact NonEmpty::first, Bundle::actions,
and RedPallas Signature/VerificationKey definitions in locally pinned nonempty
0.11.0 and orchard 0.15.5. Do not rerun fixture setup or inspect live wallet data.

## Stop

No compiler, formatter, test, Cargo/npm, Git, evidence, integration, network,
native launch, other actor, or production edit. The original future Hermes red/
green/falsification commands are unchanged and remain unauthorized here. Existing
production compile errors and the missing context remain deliberately unrepaired.

Report this file's final hash/line count, the four unchanged test names, exact
corrections, and unchanged hashes of the two accepted append-only paths. Then
stop. The reviewer collects once only after the owner reports done; no polling.
