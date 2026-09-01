# BBD-WAL-006 Prepare Design Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `ec8cec39`

Result: **CORRECT WITH REVIEWER CHANGES — NFC DEPENDENCY TEST SOURCE AUTHORIZED**

Grok Build completed the bounded upstream and lifecycle review. The reviewer independently
verified the pinned `zcash_client_backend 0.24.0` and `pczt 0.9.3` sources and accepts a real,
unsigned Ironwood PCZT path. One prerequisite must remain test-first: the Rust adapter has no
reviewed Unicode-normalization dependency and cannot honestly enforce the frozen WAL-002 NFC memo
contract with the standard library or the current direct graph. Production preparation therefore
remains frozen while an exact dependency-policy test is added and falsified.

## Accepted upstream call graph

The production builder must use:

1. a single-payment `zip321::TransactionRequest` to the already decoded Orchard-protocol-only UA;
2. `GreedyInputSelector::new()`;
3. `fees::standard::SingleOutputChangeStrategy::new(StandardFeeRule::Zip317, None,
   ShieldedPool::Ironwood, DustOutputPolicy::default())`;
4. `SpendPolicy::shielded_pools([ShieldedPool::Ironwood])` and
   `ConfirmationsPolicy::MIN` for the frozen local fixture;
5. `propose_transfer(..., None, Some(TxVersion::V6))`; and
6. `create_pczt_from_proposal(..., OvkPolicy::Sender, ..., None, BundlePadding::DEFAULT)`.

The explicit Ironwood-only `SpendPolicy` is mandatory. The default policy permits Sapling,
Orchard, and Ironwood and may combine pools. Passing `lock_inputs: None` is also mandatory: pinned
`propose_transfer` then performs no output-lock write. `create_pczt_from_proposal` obtains the
persisted UFVK and witnesses and does not require a seed or USK. It builds through Creator and the
IO Finalizer, adds metadata through Updater, and returns `pczt::Pczt`; BitBook must not call Prover,
Signer, Spend Finalizer, Transaction Extractor, or transaction storage.

The resulting PCZT is serialized only with `pczt::Pczt::serialize`, retained only as secret
process memory, and decoded with `pczt::Pczt::parse` for inspection. The IO Finalizer may create
and clear protocol-padding dummy authority. Therefore the frozen `has_signatures == false` means
no real spend authorization signature; it must not reject a protocol dummy that upstream already
finalized and cleared. `zkproof == None`, unsigned real spends, absence of extraction, v6 global
fields, and exact bundle/output metadata remain independently inspectable without invoking an
authority-bearing role.

## Pool and error invariants

- Official scanned state is the product balance truth. A test inventory override may select a
  frozen outcome but may not construct a mock PCZT or alternate transaction.
- Sufficient confirmed, unlocked Ironwood proceeds through the real Ironwood-only proposal.
- Insufficient Ironwood with sufficient Orchard returns `MIGRATION_REQUIRED`; transparent or
  Sapling-only sufficiency returns `CAPABILITY_MISSING`; otherwise return `INSUFFICIENT_FUNDS`.
- Mixed legacy value is ignored when Ironwood alone suffices. No account total may substitute for
  Ironwood spendable value.
- The standard proposal fee is authoritative and compared with the caller's bound only after the
  proposal exists. No caller fee rule is accepted.

## Memory and authorization boundary

Do not modify or reuse generic `session.rs`. It has no session ID, spend accessor, prepared map,
or required invalidation labels. ZEC prepare owns a narrow process-local state beside the viewing
`AddressAccount`:

- a random 32-lowercase-hex session ID bound to the account;
- active derived spend material in `SecretBytes`, used only as the WAL-004 unlocked-session policy
  gate and never passed to the upstream PCZT builder;
- a capped, no-eviction map from random 32-lowercase-hex handle to serialized raw PCZT plus the
  exact account/session/request/intent/expiry binding; and
- bounded observations needed by the hidden typed test facade, never alternate prepare logic.

All typed input, receiver, memo, timestamp, account, and binding checks precede spend-material
access. A handle miss or any account/session/request/intent mismatch has one constant-shape
`LOCKED` response and returns no bytes.

Every `HandleInvalidation` edge removes every entry and explicitly calls
`SecretBytes::wipe_with("zec-prepared-pczt", ...)` before Drop. Lock, timeout, replacement,
rollback, operation error, panic unwind, broker exit, and owner Drop also destroy derived spend
material. Panic handling must wipe wallet-owned state, including handles that predate the
panicking call, before unwinding reaches the caller. Raw PCZT state never enters either SQLite
file, a sanitized DTO, Debug/Display output, logs, diagnostics, or an IPC/public capability.

## Exact NFC prerequisite

WAL-002 accepts general Unicode memos only when they are already NFC. An ASCII-only rule would be
a semantic regression, and checking only the frozen combining-acute example would be a test
shortcut. Authorize exactly:

```text
unicode-normalization = { version = "=0.1.25", default-features = false, features = ["std"] }
```

Version 0.1.25 is the current published UAX #15 implementation, supports the pinned Rust toolchain,
and reuses the already locked `tinyvec` family. It adds no process, filesystem, network, FFI,
unsafe first-party, signing, proving, extraction, or transport authority. Its manifest, policy,
resolved lock entry, source, checksum, license, and advisory state must still be inspected before
acceptance.

The first authorized action is only the independent policy test. No manifest, policy
implementation, lockfile, Rust production, frozen ZEC test, or fixture byte may change in that
drop.

