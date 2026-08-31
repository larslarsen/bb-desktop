# Codex Sol Handoff — BBD-WAL-006 Address Production 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `c660549724ba5dcd30cb9f3b68909d1383d96b48`

Protected governance parent: the commit containing this handoff. Its changes after the
implementation source baseline are reviewer-authored acceptance and routing records
only; they change no production, test, fixture, manifest, lockfile, or policy byte.

Read completely before editing: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`,
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`, the fixture/expected-red and Phase-C
policy-red evidence/reviews, `docs/handoff/CURRENT_TASK.md`, `wallet-broker/src/lib.rs`,
`wallet-broker/src/vault.rs`, `wallet-broker/Cargo.toml`, the frozen fixture manifest,
and the complete committed `wallet-broker/tests/zec_address.rs`. You may use read-only
`sed`/`rg` inspection of the repository and already-cached exact crate sources. Do not
use network or execute a compiler, formatter, test, build, policy tool, or Git command.

## Sole task

Author the smallest real production implementation that satisfies the complete accepted
`zec_address` test without changing or bypassing it. This slice derives account zero's
Orchard-protocol-only Unified Addresses through the accepted upstream Zcash APIs,
persists viewing/issuance state in a real SQLite wallet boundary, supports viewing-only
reopen, and proves seed wiping and atomic serialized issuance. It remains offline,
synthetic, non-mainnet, unsigned, and unable to move funds.

## Exact authorized paths

You may create or edit only:

- `wallet-broker/src/lib.rs`
- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/address.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

`lib.rs` may add only `pub mod zec;`. Do not create the future `prepare.rs` or `scan.rs`
paths, a placeholder/stub module, a network/raw/sign/prove/finalize/extract/broadcast
module, or any unlisted file. Do not edit `Cargo.toml`, `Cargo.lock`, tests, fixtures,
`scripts/security-policy.js`, documentation, evidence, handoffs, workflows, or package
files. The exact six Zcash crates are already normal defaults-off dependencies; stop if
another direct crate, feature, version, or lockfile change appears necessary.

## Closed production model

Keep public items in `zec.rs`; keep implementation modules private and re-export only
the test-required closed boundary. Implement:

- `AccountId` as exactly 32 lowercase ASCII hex with `SCHEMA` on every other input;
- `Network` limited to `Testnet` and the injected `Local(LocalNetwork)` schedule;
- `LocalNetwork::new(birthday, nu6_3, confirmation)` with checked ordering and an exact
  upstream `zcash_protocol::local_consensus::LocalNetwork` mapping: Overwinter at 1,
  Sapling through NU6.2 at birthday, and NU6.3 at the supplied activation height;
- `FreshReceiverV1` with exactly the five tested public fields and canonical decimal
  index/sequence strings;
- stable `ZecError` codes/messages with redacted `Debug`, no upstream/raw error or path;
- `MAX_DIVERSIFIER_INDEX` and `MAX_ISSUANCE_SEQUENCE` both exactly `i64::MAX as u64`,
  so every accepted value is representable in SQLite without a signed wrap; and
- `#[doc(hidden)] pub mod test_support` only as the integration-test facade described
  below.

Reject `zec-mainnet` with `NETWORK_DISABLED` before filesystem, database, key derivation,
or operation recording. Wrong account/session binding is `SCHEMA` before state access.
There is no generic network string parser outside the product-network test entry, no
endpoint, socket, mainnet parameter construction, or fallback receiver composition.

## Derivation, issuance, and storage

Consume the existing `vault::SecretBytes` in-process. Require exactly 32 seed bytes,
derive Zcash account zero through official `zcash_keys` APIs, retain only the UFVK/viewing
material needed after bootstrap, and wipe the input/derived spending owner with label
`zec-seed` on every exit. Do not persist seed, mnemonic, USK, derived spending material,
vault plaintext, passphrase, raw PCZT, or authorization state.

Use `UnifiedAddressRequest::ORCHARD` and upstream UA encoding/decoding. For seed
`[0u8; 32]`, account zero, local schedule `(100, 102, 106)`, and diversifier index zero,
the first encoded receiver must equal the committed manifest's
`expected.orchard_only_receiver`. Never hard-code or copy that receiver into production;
the fixture facade must read it from the bounded committed manifest, while production
derives it independently. Decoding must prove exactly one Orchard-protocol receiver and
no P2PKH, P2SH, Sapling, Tex, or unknown receiver.

The first issuance uses index `0` and sequence `1`; each later issuance advances both
with checked arithmetic. Persist the last issued index and sequence atomically in the
real SQLite account state before returning. Calls sharing one account—including the two
test threads—must serialize through one transaction/lock and return distinct increasing
indices, sequences, and receivers. Reopen must use persisted viewing material only,
report no spending authority, and continue after the greatest committed index. At the
exact named maxima return `LIMIT` without mutation; reject any injected value above a
maximum before storage/allocation.

Create the broker-controlled account directory at relative
`zec-local/{account_id}` and prepare `wallet.sqlite3` plus `compact.sqlite3` as regular
Linux files beneath the supplied `TestStateRoot`; directories are `0700`, files `0600`,
and caller paths are never accepted. Validate ancestors/entries without following a
symlink. This slice may build only the schema/viewing/counter foundation needed by
address bootstrap/reopen; it must be a real SQLite/upstream wallet path that the later
store slice can inspect and extend, not JSON, an ad hoc binary, or a memory-only test
store. Preserve account/network/schema binding and never silently replace existing
state.

The three `AddressFault` variants must inject at the real receiver-row,
sequence-row, and commit/durability boundaries. Each fault returns `STATE_CORRUPT` or
`INTERNAL`, returns no receiver, leaves both coupled records unchanged, and allows the
next clear-fault issuance to start at `(0, 1)`. Do not report durability before commit.

## Hidden integration facade

`test_support.rs` is public only because Rust integration tests require visibility. It
must be `#[doc(hidden)]`, expose exactly the address-test symbols/methods, and wrap the
same production address/store/fixture implementations. It may add deterministic state
roots, typed fault ports, wipe observation, and sanitized projections; it may not derive
an address, implement a second counter/store, fabricate a wipe receipt, or hard-code a
fixture expectation.

Implement the exact test-used surface for `AddressFault`, `FrozenFixture`,
`RecordingWipes`, `TestAccount`, `TestStateRoot`, and `decode_unified_address`.
`TestStateRoot::fresh(label)` must allocate a unique disk-backed directory only beneath
`wallet-broker/target/wal006-state`, preserve clones, and record production operations
so the mainnet test proves none occurred. Do not use system temporary directories.

`FrozenFixture::open` must perform a bounded, closed serde parse of the committed
manifest sufficient to return its expected Orchard-only receiver; reject unknown fields,
wrong format/version, path escape/absolute input, oversized input, malformed JSON, and
missing expected data. Do not generate, mutate, or rewrite fixture bytes.

`RecordingWipes` must implement the real `WipeObserver` path and report an event only
after the underlying mutable seed bytes are zero. `exercise_seed_exit` must drive actual
production ownership through success, error, cancellation, replacement, caught unwind,
and drop. It may not directly insert a success event into the observer. The unsupported
composition helper must route through the production composition validator and leave
issuance state unchanged.

All shared state must remain poison/failure safe. Do not use `unsafe`, leak secrets in
`Debug`/`Display`, log, print, spawn a process, open a network connection, accept a raw
path, or claim erasure beyond the observed mutable buffers.

## Source-actor restrictions and report

Use `apply_patch`. Read-only repository/cached-source inspection is permitted. Do not run
Cargo, Rust, rustfmt, Node, npm, tests, linters, builds, policy checkers, scanners,
Electron, wallets, nodes, devices, network, Git, or GitHub. Do not install, delete,
clean, move, stage, commit, or push anything.

Stop and report instead of substituting if the accepted upstream APIs cannot provide the
exact first receiver, real SQLite/viewing-only reopen, or Orchard-only decoding; if a
new dependency/feature/lock mutation is needed; or if an authorized path is insufficient.
Otherwise report every changed path with line count and SHA-256, the upstream APIs used,
storage/concurrency/wipe design, and any concern. Luna—not Sol—will inspect the drop,
run formatting/compile/tests, write evidence, and own all Git operations.
