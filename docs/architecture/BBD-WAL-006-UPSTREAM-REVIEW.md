# BBD-WAL-006 — Zcash Adapter Upstream and Trust-Boundary Review

Status: ACCEPTED FOR TEST-SOURCE AUTHORIZATION

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Reviewed: 2026-08-31

Repository baseline: `e8894a442b970a856bee3f92de9de1e94aa0ee7c`

Parent architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` §§1.1, 2, 3 T23/T30,
5.3–5.4, 6, 7, 10, 13, 15, 17, and 19

## Outcome

The first real ZEC slice can proceed without choosing a public lightwalletd endpoint,
without deciding Tor policy, and without enabling mainnet or broadcast. It is an offline
Rust adapter exercised on a reviewer-frozen local-consensus compact-block fixture. It
derives an Orchard-protocol Unified Address, persists viewing and note state in SQLite,
scans Orchard and Ironwood notes, and prepares an unsigned version-6 PCZT containing an
Ironwood bundle. It never signs, proves, extracts, serializes a broadcastable transaction,
submits a transaction, or opens a network connection.

This split is deliberate. It proves the difficult consensus, address, database, and
transaction-construction semantics while leaving the unresolved endpoint and IP-privacy
choices outside the executable product. The adapter is not yet a usable wallet and makes
no claim about mainnet, hardware devices, Orchard-to-Ironwood migration, or live funds.

## Maintained upstream compatibility set

The reviewer checked the official published crate metadata, official librustzcash source
and rustdoc, the Ironwood book, and Zcash's zcashd end-of-life notice on 2026-08-31. The
stable compatibility set is:

| Crate | Exact version | Published | MSRV | crates.io checksum |
| --- | --- | --- | --- | --- |
| `zcash_client_backend` | `0.24.0` | 2026-08-19 | 1.88 | `07ad58d8ca5daacbc089bf82ec09e45cf2a8be40adeb40b9399b16545c81a528` |
| `zcash_client_sqlite` | `0.22.0` | 2026-08-19 | 1.88 | `dd92e4334619b1e3f67019254049318ec0d0240a3c15d853bdf2ac4bba597078` |
| `pczt` | `0.9.3` | 2026-08-07 | 1.88 | `a5592f4f3eba7f9344cc423f45b6e65911e0630c9b89968d5a20792aadd5a0eb` |
| `zcash_primitives` | `0.30.1` | stable | 1.88 | `403d5be1e96339534be098e3377fb8a78d68ca7585b1780133d884b810277418` |
| `zcash_protocol` | `0.10.5` | stable | 1.88 | `314329b91ec4bbb517441840e47d0b2029bf0b946f086980c96c889c2d92dc5d` |
| `zcash_keys` | `0.16.1` | stable | 1.88 | `def800f128e459eedebc900f36f408eaf0687634128dcf64ecfeaeebc3e16c14` |

Rust 1.98.0 already pinned by the broker is compatible. These are exact direct pins, not
caret requirements. `Cargo.lock` remains committed and authoritative for transitives.
The source actor must stop on an incompatible published manifest or API instead of
silently changing a version, enabling a wider feature, or using an RC/git dependency.

Primary sources:

- <https://crates.io/api/v1/crates/zcash_client_backend>
- <https://crates.io/api/v1/crates/zcash_client_sqlite>
- <https://crates.io/api/v1/crates/pczt>
- <https://crates.io/api/v1/crates/zcash_primitives>
- <https://crates.io/api/v1/crates/zcash_protocol>
- <https://crates.io/api/v1/crates/zcash_keys>
- <https://github.com/zcash/librustzcash>
- <https://github.com/zcash/librustzcash/blob/main/zcash_client_backend/CHANGELOG.md>
- <https://zcash.github.io/librustzcash/rustdoc/latest/zcash_client_backend/data_api/index.html>
- <https://zcash.github.io/librustzcash/rustdoc/latest/zcash_client_backend/data_api/chain/index.html>
- <https://zcash.github.io/librustzcash/rustdoc/latest/zcash_client_backend/data_api/wallet/index.html>
- <https://zcash.github.io/librustzcash/rustdoc/latest/zcash_client_sqlite/index.html>
- <https://zcash.github.io/librustzcash/rustdoc/latest/zcash_client_sqlite/struct.WalletDb.html>
- <https://zcash.github.io/librustzcash/rustdoc/latest/zcash_keys/keys/enum.UnifiedAddressRequest.html>
- <https://zcash.github.io/ironwood/concepts.html>
- <https://zcash.github.io/ironwood/design.html>
- <https://zcash.github.io/ironwood/design/transaction-format.html>
- <https://zcash.github.io/zcash/user/end-of-life.html>

## Feature-graph truthfulness

`zcash_client_backend`'s `pczt` feature also enables Orchard, transparent-input support,
and upstream PCZT builder/finalizer/prover/signer/spend-finalizer/extractor components.
`zcash_client_sqlite` and backend dependencies also compile Sapling-related support.
Therefore BitBook cannot honestly claim that transparent, Sapling, signing, proving, or
extraction code is absent from the resolved dependency graph.

The enforceable security claim is narrower and testable:

- the BitBook adapter exposes only Orchard-protocol UA receive, fixture scan, and
  unsigned PCZT preparation;
- no public BitBook method accepts a transparent or Sapling receiver, raw transaction,
  raw PCZT, signing key, proof, signature, or broadcast target;
- no signer, finalizer, prover, extractor, network transport, or submit function is
  invoked;
- a prepared PCZT remains broker-owned behind an opaque, bounded handle and is never
  logged, returned over Electron IPC, or written to an unencrypted export path; and
- feature/lock policy tests fail if a direct live-sync, lightwalletd, Tor, zcashd
  compatibility, ZEWIF, non-standard-fee, git, wildcard, or build-time downloader path
  appears.

No backend `sync`, `lightwalletd-tonic*`, `tor`, `zcashd-compat`, `zewif`, or
`non-standard-fees` feature is enabled in this ticket. The upstream feature names must be
verified against the resolved published manifests before expected red is accepted.

## Fixed adapter boundary

### Networks and consensus

- Production-callable WAL-006 behavior is limited to `zec-testnet` and an injected
  `LocalNetwork` used by deterministic tests. `zec-mainnet` returns `NETWORK_DISABLED`
  before account, database, receiver, scan, proposal, or PCZT work.
- The recorded fixture has an explicit wallet birthday and local activation schedule
  containing pre-NU6.3, NU6.3, and post-activation blocks. Tests assert the consensus
  branch ID `0x37a5165b`, transaction version 6, and Ironwood bundle semantics from the
  upstream decoded object, not from BitBook's own labels.
- No public-testnet service is contacted. “Testnet” here is a network discriminator, not
  permission to open a socket.

### Secret and viewing state

- Test material is a fixed synthetic 32-byte seed that is not words, a mnemonic, user
  data, a mainnet key, or a production backup. Production adapter entry accepts secret
  bytes only from the already-unlocked WAL-004 custody core in the same process.
- Seed bytes derive account zero's Zcash key material, then are wiped on success, error,
  cancellation, replacement, panic unwinding, and drop. The ticket may claim only the
  explicit wipe paths it observes; it must not claim allocator, register, swap, or core
  dump erasure.
- The SQLite wallet database may persist the UFVK, birthday, diversifier state, scanned
  notes, nullifier-related viewing state, and tree state. It must never persist the seed,
  mnemonic, Unified Spending Key, passphrase, vault plaintext, raw PCZT, or authorization
  session.
- Scanning and fresh receiver derivation work while the software account is locked from
  persisted viewing state. PCZT preparation requires an active WAL-004 software session;
  the seed/derived spending material never crosses the Rust process boundary.

### Address contract

- Fresh receive uses `UnifiedAddressRequest::ORCHARD`, whose official stable definition
  requires Orchard and omits Sapling and P2PKH. Ironwood does not have a new receiver
  kind.
- The decoded UA must contain exactly one Orchard-protocol receiver and no Sapling,
  P2PKH/transparent, or unknown receiver. Any other composition is
  `TRANSPARENT_DOWNGRADE` or `PROTOCOL_INCOMPATIBLE`, never a fallback.
- Diversifier/address state is monotonic and durable across close/reopen. A successful
  `receiver.fresh` never repeats a previously issued receiver. A failed durable update
  returns failure and does not claim the receiver was issued.

### SQLite and compact-block contract

- Each validated `(account_id, network)` owns one wallet database and one compact-block
  cache under its broker-controlled account directory. Paths are constructed from closed
  enums and a 32-lowercase-hex account ID; no caller path is accepted.
- The account directory is mode `0700` and state files are mode `0600` on the accepted
  Linux boundary. Symlink, non-regular-file, wrong-owner-access mode, SQLite corruption,
  unsupported schema, wrong network/account binding, migration failure, discontinuous
  chain, wrong previous hash, malformed compact block, and impossible tree state fail
  closed. Existing state is never silently discarded and replaced by an empty wallet.
- The fixture is produced once by test-only code using official upstream APIs, at an
  explicit disk-backed path under `wallet-broker/target/`, then frozen as committed
  bytes with SHA-256 and a human-readable manifest. Production scanning tests consume
  the frozen bytes; the production adapter does not generate its own oracle.
- The fixture covers a recognized incoming Ironwood note, an older Orchard note, an
  unrelated output, a block discontinuity, a one-block rollback/reorg, replay, truncation,
  and corruption. Balance assertions distinguish Ironwood spendable value from Orchard
  migration-required value.

### Unsigned v6 PCZT preparation

- Preparation starts from a closed typed input containing account, network, request ID,
  intent hash, exact destination UA, exact positive zatoshis, memo, expiry, and exact
  fee bound. This slice does not parse Electron messages or social signatures; it is an
  internal coin-adapter boundary beneath WAL-002's intent contract.
- The upstream wallet proposal and standard fee rule are authoritative. BitBook does not
  accept a caller fee, floating-point amount, fiat value, custom fee rule, change address,
  raw transaction component, or raw PCZT mutation.
- Enough Ironwood value prepares an unsigned v6 PCZT with an Ironwood bundle. If spending
  would require Orchard value, return `MIGRATION_REQUIRED`; if only transparent or
  Sapling value is available, return `CAPABILITY_MISSING`. If enough Ironwood exists,
  older-pool value is ignored rather than silently combined.
- The decoded prepared artifact must match the input network, receiver, amount, memo,
  request ID/intent binding, standard fee, and fee bound. It contains no transparent,
  Sapling, or Orchard output bundle. It is unsigned and unextracted.
- The caller receives only `PreparedZecV1`: an opaque random handle plus sanitized
  account/network/request/intent/amount/fee/expiry/version/branch/pool metadata. The raw
  PCZT never implements `Display`, never has a useful `Debug`, and is destroyed on lock,
  timeout, cancellation, replacement, error, panic, or broker exit.

## Explicitly absent capabilities

WAL-006 contains no live lightwalletd client, DNS, HTTP, HTTPS, tonic transport, proxy,
Tor, socket, endpoint preference, public-testnet sync, mainnet, zcashd, zallet, zebrad,
ZIP-318 migration, Sapling/transparent product account, mnemonic generation, backup UI,
native confirm UI, signer, proof generation, signature, hardware device, PCZT export,
transaction extraction/serialization, transaction ID, broadcast, retry queue, Electron
integration, daemon integration, rate fetch, fiat conversion, package build, or release
SBOM generation.

This leaves owner questions Q2 (default light endpoint) and Q3 (compact-block IP privacy)
open without blocking offline adapter work. Q5 remains “no transparent receive/spend,”
Q9 remains non-mainnet until BBD-WAL-012, and the hardware preference remains gated on
verified device capabilities after an applicable adapter exists.

## Test and review consequences

Tests lead production and include deterministic unit/boundary tests, frozen fixtures,
failure injection, corruption/reorg cases, concurrency around receiver issuance and
prepared-handle invalidation, secret/log canaries, and negative capability tests. At
least these mechanisms are falsified before acceptance: Orchard-only UA construction,
chain continuity enforcement, Orchard-pool migration refusal, PCZT v6/Ironwood inspection,
and prepared-handle wiping on lock.

The first developer phase authors test source only. Codex Luna later resolves the pinned
graph into the existing committed lockfile, records the independent fixture, and proves
expected red. Production remains unauthorized until XHigh accepts exact test hashes and
expected-red evidence. Routine pushes run the established Rust/Node/security checks; no
cross-platform package binary or release SBOM is built for this source-only ticket.
