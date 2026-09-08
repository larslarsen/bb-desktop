# WAL-009 retained-spend repair 02 source review

Decision: ACCEPT the four-path source drop for grouped Hermes validation.
Execution and falsification remain required; this is not pipeline acceptance.
Reviewer: Codex at XHigh; baseline 0453ad50.

Grok session d91bf773-249e-4701-9da4-620856c899be, outer 8471, was collected
once after done, exit 0. CLI grok-4.6 High; transcript grok-4.6-build High.
Authorizing parent ab81e291. The transcript authored the new unit module,
external success assertions, and read-only test accessor before production edits.
No test/compiler/formatter/Cargo/Git execution appears. Unnecessary broad
read-only discovery commands were issued; this procedural deviation does not
invalidate the independently verified source. No corrective report cycle is opened.

The reviewer reversed all saved source replacements in memory and recovered all
three exact authorized starting hashes. The final new test module was read in
full. Only the four authorized source paths differ from the previous inventory;
all twenty pending source/package/lock/evidence identities were checked.
Frozen ShieldedVerificationAuth/Context and independently_verify are byte-for-byte
unchanged in the reconstructed diff; the 528-line context test hash also matches.

| Changed path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1046 | 8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a |
| wallet-broker/src/zec/test_support.rs | 4369 | c34aa4dc95011c7585b44553e6e1680dd3c918ea78ffa98e07d2e271f7991159 |
| wallet-broker/tests/zec_sign_verify.rs | 1117 | 2b75901787126ec318bfe481fedd6d388bfe4afa4184cb03ca052447e803518d |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |

The implementation follows the corrected public API route. validate_prepared
runs on the original compact PCZT; the private binding captures the unique
unsigned slot, randomized key, nullifier, and positive actual spend value.
The owner-consuming Verifier role checks present values on exactly two actions,
requires zero-value padding, and returns the PCZT into the real signer. Final
inspection rechecks that retained identity/value and both signatures, retains
output/receiver/memo/fee/proof checks, and selects the real signature by index.
Actual proving, extraction, decoded signature/proof verification, counters,
fault points, and post-sign publication barriers remain in place.

External expected index/key now come from the authorizer's retained PCZT. Typed
pool/index/key/signature metadata checks precede the pinned signature application.
The signer view checks its unsigned index/key before signing; view and contribution
metadata propagate the actual action index. Outer metadata must match inner signed
metadata, with existing intent, route, count, replay, and test-only checks intact.
Batch application remains unconditionally closed.

Deterministic unit cases cover retained slots zero and one plus wrong slot/range,
pool, key, and zero signature. Their successful cases claim only metadata validity.
The strengthened existing external integration test compares view/contribution
indices with the pinned signature's actual index and still exercises the full
real pipeline. Existing understood software/external red supplies the regression
baseline. No random retry fixture, shortcut verifier, or source-text test is added.

Open [grouped retained-spend validation](../handoff/HERMES_BBD_WAL_009_RETAINED_SPEND_VALIDATION_01.md).
Hermes runs focused and affected integration green, the two isolated falsifications,
and restored library green, stopping after any unexpected result and restoration.
Mutation hashes were calculated in memory; the reviewer ran no acceptance command.
The source remains uncommitted. Full recovered-effect validation, actual-secret
cleanup, native confirmation, security gates, and integration stay unaccepted.

Reviewer publication scope: this review, the linked Hermes handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
