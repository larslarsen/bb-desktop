# WAL-009 signature-context tests 01

Actor: Grok Build, CLI model `grok-4.6`, reasoning High. Governance parent: the
commit containing this handoff. Author test source only; do not implement repairs.

Read AGENTS.md, TESTING.md, CURRENT_TASK.md's active section, this handoff, the
ticket's verification/error requirements, and the XHigh prerequisite-02 scoping
review. Inspect only the relevant local fixture, prepare/store, spend, vault,
and test-support APIs and the locally pinned dependency definitions needed below.
No historical handoff reload, web search, dependency-tree scan, or other actor.

## Exact source scope

1. wallet-broker/src/zec/spend/verification_context_tests.rs: new private test
   module containing the four tests below and their test-only helpers.
2. wallet-broker/src/zec/spend.rs: append only a cfg(test) declaration for
   `verification_context_tests`. Preserve its entire existing 880-line prefix,
   SHA-256 `ae665e4742925d88e1f7cf6a66172e89a268f613a5fd0efc0b3cc3581ac4cdf4`.
3. wallet-broker/src/zec/test_support.rs: preserve its entire existing 4359-line
   prefix, SHA-256
   `461fdd070318cc5f31a29af2cdceaab1d0b63dc23473641b61ea9aafc145cd1b`.
   Append only this test-build accessor, with normal separating whitespace:

```rust
#[cfg(test)]
pub(crate) fn verification_context_test_root(label: &str) -> StateRoot {
    TestStateRoot::fresh(label).inner
}
```

Stop if a baseline differs or the new file already exists. No production byte,
visibility, existing test, fixture, manifest, lockfile, evidence, or governance
change is authorized. The root accessor reuses existing test allocation under
wallet-broker/target/wal006-state. Do not add a second allocation/cleanup scheme.

Read-only anchors:

| Path | SHA-256 |
| --- | --- |
| wallet-broker/src/zec/prepare.rs | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| wallet-broker/src/zec.rs | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/Cargo.toml | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |

## Fixed future private interface — leave absent

Tests call these future private items in their parent spend module:

- `ShieldedVerificationContext::new(transaction: Transaction) -> Result<Self, ZecError>`;
- `context.data() -> &TransactionData<ShieldedVerificationAuth>`;
- `context.transaction_id() -> zcash_protocol::TxId`;
- `context.shielded_sighash() -> [u8; 32]`.

The context and marker remain private, without Clone/Debug/serialization or a
public constructor/injection surface. They do not return VerifiedEffects or an
approval capability. Use matches/assertions that do not require their Debug impl.

The future marker uses TransparentAuth projected from pczt::EffectsOnly and
SaplingAuth/OrchardAuth projected from transaction::Authorized. `new` retains the
decoded library txid, consumes Transaction::into_data, and calls try_map_bundles.
The transparent closure rejects EVERY Some(bundle) with INTENT_MISMATCH, including
an empty bundle; only None maps to None. Sapling and both Orchard-family domains
move unchanged. Compute TxIdDigester and the pinned signature_hash with
SignableInput::Shielded on this same typed data. No transaction cloning,
reserialization, unsafe cast, fake prevout context, manually constructed EffectsOnly
bundle, dependency change, signer-supplied hash, or txid-as-sighash shortcut.

This is an authorization-type adapter. Existing ticket validation of supported
version/branch/pools and recovered effects remains the caller's responsibility;
the later production handoff must preserve fail-closed validation before publishing
VerifiedEffects. Do not invent additional constructor behavior in these tests.
Later independently_verify must consume the decoded transaction and use this SAME
context for its actual signature/proof calls. An unused test-only implementation
or a second verification path will be rejected.

## Real fixture and independent oracle

Build fixture material in the private test module with the existing validated
tests/fixtures/zec compact fixture, LocalNetwork manifest values, a unique test
state root from the accessor, AddressAccount::bootstrap, scan_fixture(Canonical),
and build_prepared_pczt. Use the accepted fixture's synthetic zero seed, account
ID, destination receiver, 100000000 zat, coffee memo, and nonempty synthetic
request/intent bindings. Do not use SignVerifyHarness, verification flags, wipe
counters, or its expected/actual inspection values as a cryptographic oracle.

Parse the prepared SecretBytes with pczt::Pczt::parse. Derive the fixture USK using
the pinned UnifiedSpendingKey and local parameters. Assert exactly one unsigned
real Ironwood action, two total actions, and absent legacy bundles. Instantiate
the pinned PCZT Signer and capture Signer::shielded_sighash BEFORE signing, then
sign the real Ironwood action. Prove, finalize, and extract using pinned Prover,
SpendFinalizer, and TransactionExtractor directly. Obtain the Ironwood circuit
version through the pinned branch/pool API and use cached_orchard_proving_key.
Do not call production finish_pipeline or independently_verify to build the oracle.

Serialize with Transaction::write into Zeroizing<Vec<u8>>, and independently read
using Transaction::read with BranchId::Nu6_3 and complete-consumption assertions.
Keep sensitive test material scoped to the test; no raw artifact file, log, panic
dump, Debug snapshot, or public status export. No global transaction cache or new
fixture generation. Randomized signatures/proofs may use pinned library randomness;
assert deterministic validity and equality relationships, not random byte constants.

The oracle is the independently captured PCZT signing message, never a second call
to the production context or a caller-supplied expected message. Test helpers may
serialize/decode or clone synthetic values to construct negative inputs; that
permission does not extend to the future production type adapter.

## Four focused tests

1. `preserves_decoded_v6_ironwood_and_matches_pczt_sighash`: construct the real
   fixture and snapshot its decoded txid, version, branch, lock time, expiry,
   absence of transparent/Sprout/Sapling/Orchard, and nonempty Ironwood data before
   consuming it. Require v6, Nu6_3, Ironwood bundle version ironwood_v3, and exactly
   two actions. Compare all header/pool facts after conversion. Compare complete
   before/after Ironwood bundle bytes using the pinned components::orchard::
   write_v6_bundle into scoped zeroizing buffers; this covers every action,
   ciphertext, anchor, flags, balance, proof, and signature without a custom codec.
   Assert retained txid equals the original library txid and the computed message
   equals the independently captured Signer oracle.
2. `rejects_every_present_transparent_bundle`: decode the pinned public
   zcash_primitives::transaction::tests::data::tx_read_write::TX_READ_WRITE with
   BranchId::Canopy solely to obtain its existing transparent bundle. Assert its
   inputs and outputs are nonempty. The module is already enabled through
   zcash_client_sqlite/test-dependencies; no feature/dependency edit is needed.
   For four negative cases transplant that bundle into separately decoded copies
   of the valid local v6/Ironwood fixture with upstream map_bundles::<Authorized>:
   both inputs/outputs, inputs only, outputs only, and both cleared but still Some.
   Keep every other local fixture field intact, freeze with the pinned library,
   assert the intended Some shape, then require constructor INTENT_MISMATCH.
   Do not reserialize/redecode the in-memory empty Some case, which the wire format
   can normalize to None. No signature/proof call precedes constructor rejection.
   Suppressing transparent rejection must therefore make this test fail; failure
   for a missing Ironwood bundle or unrelated invalid signature is insufficient.
3. `uses_real_spend_and_binding_signatures`: with the real fixture, require every
   actual decoded action rk().verify(message, authorization) and the real
   binding_validating_key().verify(message, binding_signature) to succeed. Alter
   the message and require both kinds to fail. In separate copies alter actual
   spend-signature bytes and actual binding-signature bytes, decode again, and
   require the corresponding real verifier to fail against the context's message.
   Use pinned signature serialization/conversion and a uniquely located byte
   sequence in the library-encoded transaction; assert exactly one occurrence
   before mutation. Do not mutate expected flags, skip decode failures, or count
   a panic/parse error as cryptographic rejection. Keep nonempty/action-count
   assertions so loops cannot pass vacuously.
4. `keeps_proof_verification_independent_of_sighash`: require the real fixture's
   nonempty proof to pass ironwood.verify_proof with its real pinned VerifyingKey.
   Alter actual proof bytes in a separate encoded copy, using the same unique
   occurrence discipline, decode it, and require real proof verification to fail.
   Require its computed message to remain equal to the original PCZT oracle and
   its unmodified real signatures to remain valid. This proves that a correct
   signing message cannot substitute for proof verification. It is not acceptance
   of production recovered-effects or wipe behavior.

Use explicit positive controls and Result assertions; no ignored tests, untrusted
reported booleans, source-text behavioral assertions, or stub context definitions.

## Later execution contract — not authorized now

Targeted red and green command, reserved to Hermes after source review:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests
```

Initial absent-interface red is the missing private context/marker. The known
spend.rs trait/Option/lifetime errors and test_support.rs fault-type error remain
unrepaired. If they appear, classify the result as prerequisite-blocked with zero
tests, not behavioral proof. An unexpected fixture/API/syntax error is a new stop.
No source repair follows until the reviewer understands the red and separately
authorizes production. The no-default command intentionally excludes native-ui;
the native compile blockers and modal task remain separately tracked.

Future falsifications, each followed by exact production-source restoration:

- suppress the adapter's transparent Some rejection and return None: test 2 fails;
- corrupt the adapter's computed message: test 1 fails its PCZT oracle assertion
  and test 3 fails its valid-signature positive control.

Mutation locations/identities and execution order must be frozen in the later
Hermes handoff. No falsification edit is authorized for Grok. Later broader
regression commands are:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare --test zec_sign_verify
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib
```

The ticket's remaining native, full signing, security, cleanup, and integration
gates are pending, not waived. No dependency/build-input/security-feature change
is authorized. Existing build artifacts remain on repository ext4, not tmpfs.

## Stop and report

Read-only inspection and bounded source edits only. No formatter, compiler, test,
Cargo/npm, Git, network, native launch, fixture execution, evidence, integration,
or subagent. Do not repair any missing item or other existing error. Report the
three path hashes/line counts, the four test names, unchanged prefix identities,
and any concrete API obstacle, then stop. The reviewer collects once after the
owner reports done and will not poll.
