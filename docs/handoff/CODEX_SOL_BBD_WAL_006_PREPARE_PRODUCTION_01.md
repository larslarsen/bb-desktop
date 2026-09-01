# Codex Sol Handoff — BBD-WAL-006 Prepare Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `7d5e8a4f256b6703aeefb66de7fe8bb01ebe093e`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, Prepare Design Review 01, Prepare NFC
Dependency Integration Review 01, both frozen `zec_prepare` and `zec_hygiene` tests, the complete
current ZEC production source, fixture manifest, and `docs/handoff/CURRENT_TASK.md`. Inspect the
pinned local sources for `zcash_client_backend 0.24.0`, `zcash_client_sqlite 0.22.0`,
`zcash_keys 0.16.1`, `zcash_protocol 0.10.5`, and `pczt 0.9.3` before editing.

## Sole task and exact source boundary

Implement the already frozen unsigned Ironwood v6 PCZT prepare vertical. You may create/edit only:

- `wallet-broker/src/zec/prepare.rs` (currently absent);
- `wallet-broker/src/zec.rs`;
- `wallet-broker/src/zec/store.rs`; and
- `wallet-broker/src/zec/test_support.rs`.

Do not edit tests, fixtures/manifest, `fixture.rs`, `address.rs`, `scan.rs`, `session.rs`, `lib.rs`,
Cargo files/lock, policy, workflow, ticket, documentation, Electron/Node source, or another
repository. Do not add a dependency or feature.

Starting protected identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/scan.rs` | 1,661 | `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad` |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 238 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |

Preserve every accepted address/store/scan behavior and all frozen test bytes.

## Required upstream construction

The success path must use the real wallet/cache state and this call graph:

1. Decode the typed destination with `zcash_keys::address::Address::decode` for the bound local
   parameters and require exactly the accepted Orchard-protocol receiver composition.
2. Construct one `zcash_client_backend::zip321::Payment` and a one-payment
   `TransactionRequest`; include `MemoBytes` only for the accepted memo.
3. Use `GreedyInputSelector::new()` and
   `fees::standard::SingleOutputChangeStrategy::new(StandardFeeRule::Zip317, None,
   ShieldedPool::Ironwood, DustOutputPolicy::default())`.
4. Call `propose_transfer` with `SpendPolicy::shielded_pools([ShieldedPool::Ironwood])`,
   `ConfirmationsPolicy::MIN`, `lock_inputs: None`, and `proposed_version: Some(TxVersion::V6)`.
5. Cross-check the proposal's payment pool, Ironwood input/output/change counts, and
   `balance().fee_required()` before building. The proposal fee is authoritative; compare it with
   the caller's bound only after the proposal exists.
6. Call `create_pczt_from_proposal` with the persisted UFVK/witnesses,
   `OvkPolicy::Sender`, no spend keys, and `BundlePadding::DEFAULT`.
7. Serialize only with `pczt::Pczt::serialize` and parse only with `pczt::Pczt::parse` for
   independent inspection.

Use Creator plus the upstream IO Finalizer and Updater only. Never invoke Prover, Signer, Spend
Finalizer, Transaction Extractor, transaction storage, or any broadcast/network path. The
upstream IO Finalizer may internally create and clear protocol-padding dummy authority; unsigned
means every real positive-value spend has no spend authorization signature. Do not reject a
cleared upstream dummy-padding action.

For the raw parsed-artifact memo oracle, use the authority-free
`pczt::roles::redactor::Redactor` and compact resolvable Ironwood fields before final
serialization. The resulting public `EncCiphertext::MemoPlaintext` must be inspected and hashed
from its stripped memo bytes. Do not retain or report a parallel input-side memo hash as the PCZT
inspection oracle. Inspect the parsed global v6/branch fields and all transparent, Sapling,
Orchard, and Ironwood bundles directly. Count real Ironwood inputs/outputs by positive value so
dummy zero-value padding cannot satisfy the frozen one-input/two-output assertion; require no
proof, no real signature, no extraction, and no legacy output bundle.

`propose_transfer(..., lock_inputs: None)` and `create_pczt_from_proposal` must leave both SQLite
files byte-for-byte unchanged. Add only a narrow `store.rs` bridge that holds the existing account
gate while proposal/build reads the actual `WalletDb` and compact-cache state. Do not add schema,
rows, output locks, transactions, alternate balances, raw SQL/PCZT access, or persisted session
state.

## Closed validation and outcome contract

Expose the exact `PrepareZecV1`, `PreparedZecV1`, `HandleBinding`, and `HandleInvalidation` shapes
required by the frozen tests, plus `MAX_MEMO_BYTES = 512`, `MAX_PREPARED_HANDLES = 64`, and
`MAX_DIAGNOSTIC_BYTES = 4096`. Extend `ZecError` only with the ticket's closed stable codes needed
by these tests: `LOCKED`, `WATCH_ONLY`, `CAPABILITY_MISSING`, `MIGRATION_REQUIRED`,
`INSUFFICIENT_FUNDS`, `FEE_BOUND`, and `EXPIRED`.

Validate every typed field before spend-material access: exact account/network binding, 32-lower
hex request/session/handle IDs, 64-lower hex intent, canonical positive u64 decimal amount and fee
bound, decoded Orchard-protocol-only receiver, memo, and expiry. Timestamps are exact
`YYYY-MM-DDTHH:MM:SSZ`, strict Gregorian dates in years 2020 through 2100, and expiration occurs
at equality. Memo bytes are UTF-8 length-bounded, already NFC according to
`unicode_normalization::UnicodeNormalization`, and exclude the full WAL-002 forbidden set:
C0/C1 controls, noncharacters, bidi controls U+202A–U+202E and U+2066–U+206F, U+200B–U+200F,
U+061C, U+2060, U+FEFF, U+FFF9–U+FFFB, and U+E0001–U+E007F.

Use the official scanned inventory as product truth. The hidden `PoolInventory` seam may select
only the frozen error-table outcome; it may not build a mock PCZT or replace success construction.
Sufficient confirmed Ironwood takes the real path. If Ironwood is insufficient, sufficient
Orchard yields `MIGRATION_REQUIRED`, transparent/Sapling sufficiency yields
`CAPABILITY_MISSING`, and all other ineligible/unconfirmed/locked-note cases yield
`INSUFFICIENT_FUNDS`. Mixed legacy value is ignored when Ironwood alone suffices. Never substitute
total value for confirmed Ironwood spendable value.

Count the standard fee rule exactly once and caller fee rules zero times. No failure may retain a
new handle. The hidden capacity helper reserves real map slots; it must not manufacture PCZT bytes
or evict an existing entry.

## Session, handle, wipe, and disclosure boundary

Own prepare state beside the viewing account; do not modify or reuse generic `session.rs`. The
process-local state contains:

- a random 32-lowercase-hex session ID bound to the account;
- active derived material held in `SecretBytes` solely as the WAL-004 unlocked-session policy
  gate and never passed to upstream PCZT construction;
- a no-eviction map capped at 64 random 32-lowercase-hex handles; and
- for each artifact, `SecretBytes` serialized PCZT and exact account/session/request/intent/expiry
  binding.

Use the existing randomness source with bounded collision retry. A miss or any binding mismatch
returns one constant-shape `LOCKED` result and zero bytes. Recheck lock and exact expiry before
lookup/use. `PreparedZecV1` exposes exactly the 17 frozen sanitized fields, has redacted
`Debug`, and never contains raw PCZT, transaction, proof, key, txid, endpoint, path, diagnostic,
rate, signature, or nullifier data.

Implement every exact invalidation edge and stable `as_str` label: lock, timeout, cancel, expiry,
account-replacement, database-rollback, operation-error, panic-unwind, and broker-exit. Each edge
removes all handles and explicitly calls `SecretBytes::wipe_with("zec-prepared-pczt", observer)`
before Drop. Lock, account replacement, rollback, operation error, panic unwind, broker exit, and
owner Drop also destroy derived material with label `zec-derived-spend` where required. A panic
inside prepare must wipe pre-existing handles and derived material before unwinding reaches the
caller. `close(self)` performs broker-exit invalidation and returns the state root while preserving
both SQLite files byte-exact.

Keep observations bounded and typed behind `test_support`. Store a concrete thread-safe test
observer, not an unconstrained callback/trait object that breaks `TestAccount: Sync`. Set the
current exit label before observing the post-wipe bytes. Canaries are commitments only. Logs,
diagnostics, errors, `Debug`, and `Display` may expose no canary or raw upstream error. Diagnostics
have exactly `operation`, `account_id`, `network`, and `code`; accepted inputs up to 4096 bytes
return a fixed redacted value, while 4097 returns `LIMIT` without echo.

The public operation list is exactly `account.bootstrap`, `receiver.fresh`, `fixture.scan`, and
`pczt.prepare`; all sign/prove/finalize/extract/broadcast/network/mainnet/raw names return
`CAPABILITY_MISSING`. All capability booleans stay false, including `can_sign` while the policy
session is active and after viewing-only reopen.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run a formatter, Cargo,
Rust, Clippy, tests, Node, policy, dependency, Git, network, fixture-generation, wallet/node/device,
cleanup, or deletion command. Do not stage, commit, or push.

Return all changed paths with line counts/SHA-256, the exact upstream APIs used, the proposal and
raw-PCZT inspection invariants, the store-lock/read-only design, session/handle ownership, every
wipe path, and any ambiguity. If an upstream API cannot satisfy a frozen assertion without a
mock, parallel oracle, persisted secret, authority-bearing role, or source-policy expansion, stop
and report the exact contradiction instead of weakening the contract.
