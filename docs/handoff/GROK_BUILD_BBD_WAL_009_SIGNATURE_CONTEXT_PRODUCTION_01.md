# WAL-009 signature-context production 01

Actor: Grok Build, CLI model `grok-4.6`, reasoning High.
Governance parent: the commit containing this handoff.
Author one bounded production source change only.

Read AGENTS.md, TESTING.md, this handoff, the first 90 lines only of
docs/handoff/CURRENT_TASK.md, and
docs/testing/BBD-WAL-009-COMPILE-PREREQUISITES-01-REVIEW.md.
Read the frozen test module and the named spend.rs definitions below. This
handoff carries the already fixed semantics; do not reload historical handoffs
or historical CURRENT_TASK content.

## Exact scope and baseline

Only writable path: wallet-broker/src/zec/spend.rs, 883 lines, SHA-256
`d95bea3aa78c326408cd22cabbb4a470ac30a53dea4d0fc34ecf5428310fa536`.

Verify it and these read-only anchors before editing; stop on a mismatch:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec |
| wallet-broker/src/zec/test_support.rs | 4371 | fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920 |
| wallet-broker/Cargo.toml | 122 | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | 5395 | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |

All tests, declarations/accessors, fixtures, native code, other source, dependency
features, manifests, lockfiles, package changes, evidence, and governance are
frozen. Do not stage or integrate pending work.

## Fixed private adapter

Add private module-level ShieldedVerificationAuth and ShieldedVerificationContext
plus the necessary imports. Neither item nor its fields/methods may be public or
pub(crate). No Clone, Debug, serialization, public injection, mutable accessor,
or approval capability. Implement the interface already used by the frozen tests:

- ShieldedVerificationContext::new(transaction: Transaction) -> Result<Self, ZecError>;
- data(&self) -> &TransactionData<ShieldedVerificationAuth>;
- transaction_id(&self) -> zcash_protocol::TxId;
- shielded_sighash(&self) -> [u8; 32].

ShieldedVerificationAuth implements zcash_primitives::transaction::Authorization:

- TransparentAuth is projected from pczt::EffectsOnly's Authorization;
- SaplingAuth is projected from transaction::Authorized's Authorization;
- OrchardAuth is projected from transaction::Authorized's Authorization.

The constructor saves transaction.txid() locally, consumes transaction.into_data(),
then uses try_map_bundles::<ShieldedVerificationAuth, ZecError>. The transparent
closure rejects EVERY Some(bundle) with ZecError::intent_mismatch(), including an
in-memory empty Some; only None returns Ok(None). Move the Sapling bundle unchanged.
Move each Orchard-family bundle unchanged through the single FnMut closure:
upstream invokes it separately for Orchard and Ironwood. Never swap their domains,
drop an existing shielded bundle, or rebuild headers. Let the upstream mapper
preserve version, branch, lock time, expiry, Sprout, and other fields.

The context owns that mapped data and saved library txid. Its data and txid
accessors return those retained values. shielded_sighash computes TxIdDigester on
the retained typed data and calls the existing pinned signature_hash on exactly
that data with SignableInput::Shielded and those digests, returning the resulting
32 bytes. No message override, mutable cache, signer-reported hash, txid-as-message
shortcut, transaction clone/reserialization, unsafe cast, fake prevout context,
manufactured EffectsOnly bundle, custom codec, or new dependency.

This is a type adapter, not a full payment validator. Do not add constructor
version/branch/pool validation beyond the specified transparent rejection.
Existing verifier checks below remain responsible for fail-closed publication.

## Wire the actual verifier

Only these existing production locations may change:

1. Transaction imports and the new private marker/context definitions.
2. In finish_pipeline, change only the independently_verify argument from
   borrowing decoded to moving decoded. Preserve decoding, fault/call counters,
   error handling, ExtractedTransaction ownership/wiping, and cleanup sequencing.
3. In independently_verify, take Transaction by value. Preserve the initial
   schema and disabled-mainnet checks in their current order. Immediately after
   them construct the context, then use context.data() for the existing decoded
   transaction accesses. Acquire the actual Ironwood bundle from this data.
4. Replace the existing incompatible digest/signature_hash block with the
   context's shielded_sighash. Use these exact bytes for every existing actual
   decoded action's rk().verify and the actual binding signature verification.
   Keep the actual proof verification and circuit-version/key selection on that
   same retained Ironwood bundle, with its present/valid checks and errors.
5. Derive the transaction-id string from context.transaction_id(), preserving the
   existing comparison and returned value. Retain all version, branch, pool,
   action-count, fee, binding, metadata, and expected-txid predicates and errors,
   and the existing VerifiedEffects construction.

The new transparent rejection precedes signatures/proof as required. Do not
otherwise reorder the existing verifier's checks. Keeping its later redundant
transparent-is-present predicate is intentional. Do not change the metadata
copying design, mutation hooks, cleanup observers, persistence, publication,
native review, capability, status/expiry/cancellation barriers, or return shapes.
Those wider A3 issues are pending separate contracts. No unused test-only context
or alternate verifier path is acceptable.

Use ordinary formatting only in directly touched code. Do not reformat the file
or edit the final cfg(test) module declaration. If a concrete API obstacle cannot
be handled within this contract, report it and stop; do not widen the repair.

## Pinned API lookup and validation boundary

If necessary, read only the relevant locally cached definitions in:
zcash_primitives 0.30.1 transaction/mod.rs (Authorization, try_map_bundles,
Transaction::into_data and txid) and transaction/sighash.rs; pczt 0.9.3 lib.rs
(EffectsOnly). The transparent projection is available through the already enabled
signer feature; preserve Cargo inputs. No home-directory or dependency-tree scan,
web search, crate download, registry update, or other actor.

The accepted four-test source precedes implementation. Hermes's exact expected-red
attempt had the missing context, incompatible signature_hash, three mechanical
errors, and one warning; zero tests ran. The reviewer understood that mixed stop
and has source-accepted the four mechanical repairs. Test compilation, fixture
success, and behavior remain unproved.

After source review, Hermes alone may receive the previously frozen focused
command (no-default-features library filter zec::spend::verification_context_tests),
the two falsifications, and broader zec_prepare/zec_sign_verify and library commands.
Their exact command definitions remain in the original signature-context test
handoff; none is authorized now. Falsifications must suppress transparent Some
rejection and corrupt the computed message, with exact source restoration and
independent PCZT/signature positive controls. The reviewer will freeze mutation
locations and execution order after inspecting this source.

No compiler, formatter, test, Cargo/npm, Git, evidence, integration, network,
native process, or subagent. No build artifacts are needed; existing target
storage remains repository ext4. Remaining native, recovered-effect, actual-cleanup,
and security gates are not waived. Mainnet, Monero, and broadcast stay parked.

Report the final spend.rs hash/line count, changed definitions, actual verifier
wiring, and unchanged four anchor hashes/line counts. Then stop. The reviewer
collects once only after the owner reports done; do not poll or duplicate actors.
