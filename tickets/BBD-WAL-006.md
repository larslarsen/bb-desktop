# BBD-WAL-006 — Offline Zcash Viewing and Unsigned Ironwood PCZT Adapter

Status: SUPPORT-DEPENDENCY TEST ACCEPTED — FOCUSED RED AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Test and production source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `e8894a442b970a856bee3f92de9de1e94aa0ee7c`

Architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` and
`docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`

Cross-repository baseline: `../bb-go` accepted BBGO-PAY-001 production at
`6bbb0629cb0dfaca9958f9cac7d57216760630ae` and final evidence at
`801f5d55d80fe02c6eb512ff35f8c09acfd679af`. This ticket does not edit that
repository. `../go-ipfs` remains deprecated.

## Objective

Add the first maintained Zcash dependency and behavioral contract to the Rust wallet
broker. On deterministic local-consensus compact blocks it must:

1. derive and durably advance Orchard-protocol-only Unified Addresses;
2. initialize and reopen account-bound SQLite viewing state without storing spend
   secrets;
3. scan recognized Orchard and Ironwood notes, continuity, rollback, and corruption;
4. keep restored Orchard value migration-required and transparent/Sapling value
   ineligible; and
5. prepare and inspect a broker-owned, unsigned v6 PCZT with an Ironwood bundle when
   sufficient Ironwood value exists.

It remains an offline adapter below the broker protocol. It creates no usable product
wallet, signs nothing, and cannot move funds.

## Fixed upstream and feature contract

The manifest uses exact direct pins only:

```text
zcash_client_backend = 0.24.0
zcash_client_sqlite  = 0.22.0
pczt                 = 0.9.3
zcash_primitives     = 0.30.1
zcash_protocol       = 0.10.5
zcash_keys           = 0.16.1
```

All six use `default-features = false`; the source actor enables only the smallest
published Orchard/PCZT/SQLite/local-consensus feature union needed by the tests. It must
report the exact resolved feature union. Enabling backend PCZT is known to compile
upstream transparent-input, signer, prover, finalizer, extractor, and related components;
tests enforce that BitBook exposes and invokes none of them. Compiled capability is not
product authority.

Forbidden direct or enabled functionality includes backend live sync, lightwalletd tonic
transport, Tor, zcashd compatibility, ZEWIF, non-standard fee rules, a general HTTP/TLS
client, async network runtime added for transport, git dependencies, wildcard/loose
requirements, `[patch]`, vendored binary, build-time downloader, OpenSSL, FFI, and new
first-party `unsafe` code. If an exact version/API/feature is incompatible, Sol stops and
reports the published contradiction; it does not substitute.

## Closed input and output contract

The adapter is crate-internal product behavior even if integration tests require public
Rust visibility. It receives typed values only. It does not parse Electron IPC or accept
generic JSON/method names.

### Account bootstrap

- Account ID is exactly 32 lowercase hex, asset is exactly ZEC, and network is
  `zec-testnet` or injected local consensus. `zec-mainnet` returns `NETWORK_DISABLED`
  before side effects.
- Tests use one fixed synthetic 32-byte seed. No mnemonic, valid production backup,
  mainnet key, or random test secret is committed.
- The product entry accepts a WAL-004 `SecretBytes` value only while its matching
  account/asset/network session is unlocked. It derives account zero's spending and full
  viewing material in-process. Seed bytes are wiped on every exit path.
- The wallet database stores only viewing/state material allowed by the architecture.
  A scanner opened while locked cannot obtain or reconstruct spending authority.

### `FreshReceiverV1`

Successful fresh receive returns exactly:

```text
account_id, network, receiver, diversifier_index, issued_at_sequence
```

The receiver is a testnet/local Unified Address whose decoded receiver set is exactly
one Orchard-protocol receiver. It contains no P2PKH, P2SH, Sapling, Tex, or unknown item.
Ironwood does not appear as an address kind. Index and issuance sequence are canonical
nonnegative decimal strings with implementation bounds named by the tests.

Issuance is serialized per account. Two concurrent calls return two distinct increasing
indices. Reopen continues after the greatest durably issued index. A persistence failure
returns `STATE_CORRUPT`/`INTERNAL` and must not return a receiver or advance only one of
the coupled state records.

### Recorded chain state

The committed fixture set has a manifest with format/version, generator compatibility
set, local activation heights, birthday/checkpoint, ordered file names, byte lengths,
SHA-256 values, block heights/hashes/previous hashes, scenario labels, and only synthetic
expected public values. Unknown fields, duplicate entries, path traversal, absolute
paths, wrong hash/length, wrong network, and unsupported manifest version fail before a
scan.

Fixture generation is a test-only upstream oracle. It writes only beneath the explicit
disk-backed `wallet-broker/target/wal006-fixture-build` directory. Codex Luna freezes the
result into the exact committed fixture paths only after hash/manifest review. Production
source cannot call the generator.

The accepted scan behaviors are:

- start at the exact birthday/checkpoint and apply contiguous blocks once;
- recognize an incoming Ironwood note and report it as spendable-for-prepare only after
  the fixture's confirmation boundary;
- recognize an older Orchard note but report its value as `MIGRATION_REQUIRED` for Pay;
- ignore an unrelated shielded output without inventing value;
- replay is idempotent;
- truncated/malformed bytes, wrong previous hash, height gap, wrong branch/network,
  impossible tree state, and SQLite corruption fail closed without advancing the
  committed scan tip;
- a supported one-block reorg rolls back the exact prior effects and applies the
  replacement; deeper-than-supported rollback returns a stable failure and does not
  partially mutate state; and
- closing/reopening produces the same balances, pool classification, receiver sequence,
  tree state, and tip.

Balances are exact checked u64 zatoshis represented at the adapter boundary as canonical
decimal strings. Overflow is failure. `total` is never substituted for `spendable`.

### `PrepareZecV1`

The typed input is closed to:

```text
account_id
network
request_id              32 lowercase hex
intent_hash             64 lowercase hex
receiver                decoded Orchard-protocol-only UA
amount_zat              canonical positive u64 decimal string
fee_bound_zat           canonical positive u64 decimal string
memo                     valid WAL-002 memo or empty
expires_at               valid WAL-002 TimestampV1
```

The adapter validates everything before accessing spend material. It rejects unknown
receiver components, wrong HRP/network, zero/overflow/leading-zero/scientific amounts,
expired input, mismatched account/session, absent unlocked session, and memo/control
violations. Fiat, exchange rate, caller-selected fee, floating point, change receiver,
raw proposal/transaction/PCZT component, and endpoint are not inputs.

The adapter uses the upstream standard fee rule and refuses a proposal whose exact fee
exceeds `fee_bound_zat`. It prepares only if the Ironwood pool alone funds amount plus
fee. Pool outcomes are fixed:

| Available recognized value | Result |
| --- | --- |
| sufficient confirmed Ironwood | unsigned v6 PCZT with Ironwood bundle |
| insufficient Ironwood but enough only by adding Orchard | `MIGRATION_REQUIRED` |
| Orchard only | `MIGRATION_REQUIRED` |
| transparent or Sapling only | `CAPABILITY_MISSING` |
| mixed old pools plus sufficient Ironwood | use Ironwood only; ignore old pools |
| wrong/unconfirmed/locked notes | `INSUFFICIENT_FUNDS` or `LOCKED` as applicable |

The PCZT is decoded through upstream types and independently inspected for exact network,
branch `0x37a5165b`, transaction version 6, Ironwood input/output bundle, destination,
amount, memo commitment, standard fee, fee bound, and absence of transparent, Sapling,
and Orchard output bundles. It has no signatures/proofs and is not finalized/extracted.

The only returned value is `PreparedZecV1`:

```text
handle                   32 lowercase hex, opaque and single-session
account_id
network
request_id
intent_hash
receiver
amount_zat
fee_zat
fee_bound_zat
expires_at
tx_version               "6"
consensus_branch         "37a5165b"
spend_pool               "ironwood"
output_pool              "ironwood"
signed                   false
extractable              false
broadcastable            false
```

No raw PCZT/transaction/proof/key bytes, txid, endpoint, path, diagnostic, or rate is
returned. Handle lookup is account/session/request/intent-bound, constant-shape on miss,
and invalidated on lock, timeout, cancel, expiry, account replacement, database rollback,
error, panic unwind, and broker exit. Raw prepared state is memory-only and has redacted
`Debug`; it never reaches logs or disk.

## Error and log contract

Use the existing stable broker meanings where applicable:

```text
SCHEMA, NETWORK_DISABLED, LOCKED, WATCH_ONLY, CAPABILITY_MISSING,
PROTOCOL_INCOMPATIBLE, TRANSPARENT_DOWNGRADE, MIGRATION_REQUIRED,
INSUFFICIENT_FUNDS, FEE_BOUND, EXPIRED, STATE_CORRUPT, LIMIT, INTERNAL
```

Public diagnostics reveal operation, account ID, network, and stable code only. Logs and
errors never contain seed/key material, vault plaintext, UFVK, raw receiver internals,
memo text, note plaintext, nullifier, raw compact block, raw SQLite row, raw PCZT,
transaction bytes, or user paths. Tests use canaries for each prohibited class.

## Required test groups

1. **Dependency and feature policy:** exact pins/checksums after lock resolution; no loose,
   git, patched, downloader, live-network, or forbidden direct feature; document the
   unavoidable upstream compiled PCZT/Sapling/transparent capability without exposing it.
2. **Independent fixture producer:** official upstream-only local-consensus construction,
   deterministic repeat output, closed manifest, fixed checksums, no production adapter
   import, no network, and no secret/mnemonic/mainnet material.
3. **Account and receiver:** seed lifecycle/wipe observation, viewing-only reopen,
   `UnifiedAddressRequest::ORCHARD`, receiver-set decoding, monotonic durable issuance,
   concurrency, exhaustion, wrong account/network, persistence failure, and no fallback.
4. **SQLite boundary:** exact path derivation, Linux permissions, account/network binding,
   initialization/reopen, migration transactionality, symlink/non-regular rejection,
   corrupt/wrong schema, failed write/sync, and no secret columns/rows.
5. **Compact-block scanning:** birthday, continuity, confirmation, unrelated output,
   Orchard-versus-Ironwood classification, idempotence, truncation/corruption, rollback,
   reorg, compound recovery failure, overflow, and durable reopen.
6. **PCZT preparation:** exact typed validation, pool table above, standard fee/bound,
   v6/NU6.3/Ironwood decoded fields, exact intent values, unsigned/unproved/unextracted
   state, absence of legacy output pools, and no generic/raw mutation interface.
7. **Hygiene and negative capability:** prepared-handle binding/invalidation, wipe/drop/
   unwind canaries, redacted formatting/logging, no sign/prove/finalize/extract/broadcast/
   network/mainnet entry point, bounded allocations, and deterministic malformed-input
   tables immediately below/at/above every limit.
8. **Regression and policy:** every accepted WAL-002/003/004 Rust/Node/security test stays
   green; no Electron, supervisor, preload, renderer, daemon, workflow, package, or SBOM
   behavior changes.

Tests must assert decoded upstream behavior, persisted state, call ordering, and negative
capability. Source-text assertions are reserved for repository dependency/capability
policy and cannot substitute for runtime behavior.

## Test-first phases and current authorization

### Phase A — test source only (complete)

Sol may create or edit exactly:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_fixture_builder.rs`
- `wallet-broker/tests/zec_address.rs`
- `wallet-broker/tests/zec_store.rs`
- `wallet-broker/tests/zec_scan.rs`
- `wallet-broker/tests/zec_prepare.rs`
- `wallet-broker/tests/zec_hygiene.rs`
- `test/securityPolicy.node.js`

The existing lockfile is read-only to Sol. The manifest adds exact dependency declarations
and explicit test targets; it does not change package identity, Rust version, or native
UI features. Lock resolution proved that the accepted WAL-004 `hkdf = 0.13.0` and
`sha2 = 0.11.0` custody pins cannot coexist with the exact prerelease HMAC/SHA-2 lines
required by the fixed Zcash graph. The separately reviewed, test-first correction may
move only those two direct custody dependencies to exact, defaults-off stable
`hkdf = 0.12.4` and `sha2 = 0.10.9`; the existing RFC 5869 and deterministic envelope
vectors must remain byte-exact. Resolution then proved direct `argon2 = 0.6.0` cannot
coexist with Zcash's exact prerelease digest line; the same test-first exception may move
only Argon2 to exact, defaults-off stable `argon2 = 0.5.3`, with the RFC 9106 and envelope
vectors byte-exact. Resolution then proved direct `chacha20poly1305 = 0.11.0` cannot
coexist with Zcash's exact prerelease `crypto-common` line; the same test-first exception
may move only that direct AEAD to exact, defaults-off stable
`chacha20poly1305 = 0.10.1`, with the XChaCha20-Poly1305 and envelope vectors byte-exact.
See
`docs/testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-01.md` and
`docs/testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-02.md` and
`docs/testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-03.md`. The fixture builder may
refer to future generated paths, but Sol does not generate or commit fixture output.

No production source, existing WAL-004 Rust test, fixture output, `Cargo.lock`, `deny.toml`,
workflow, package file, Electron/Node production, policy implementation, evidence,
documentation, scratch data, execution, install, dependency resolution, network, Git,
GitHub, cleanup, wallet/node/device, or unlisted path is authorized for Sol.

### Phase B — fixture generation and expected red (complete and accepted)

After XHigh accepts exact Phase-A hashes, a Luna handoff will resolve the graph, review
the lock/feature/license/build-script diff, run the upstream-only fixture builder into
`wallet-broker/target/wal006-fixture-build`, freeze exact fixture bytes under
`wallet-broker/tests/fixtures/zec/`, record their hashes, then run focused tests. The
first focused adapter test must fail for an absent `bitbook_wallet_broker::zec` behavior,
not because the fixture is missing, the manifest is invalid, the test does not compile
against its documented upstream APIs, or an assertion is tautological.

The fixture and expected-red record are accepted and integrated at `accac440`. No
production source was authorized by that red.

### Phase C0 — production-inventory policy test (authorized now)

The committed Node test still encodes the completed Phase-A requirement that ZEC
production source remain empty. Sol may edit only `test/securityPolicy.node.js` under
`docs/handoff/CODEX_SOL_BBD_WAL_006_PHASE_C_POLICY_TESTS.md` to replace that empty
expectation with the exact bounded Phase-C inventory. Luna will capture a new focused
expected red before any production byte is authorized. Policy implementation and all
production source remain frozen.

The deterministic ordering correction is accepted at SHA-256
`19b7948bfa2c7f9b29426133bdda1630abfade5f1c438c7367e5c6dacd32688b`. Luna may run
only the focused Node expected-red handoff. Production and policy implementation remain
frozen until XHigh accepts its evidence.

### Phase C1 — support-dependency correction (test source authorized now)

The first address source attempt changed no file. It proved that official wallet schema
initialization requires a named `rand_core::RngCore` implementation and the broker-owned
atomic extension schema requires direct SQLite APIs. The exact test-first correction is
`rand_core = 0.6.4` with defaults off/`std` only and `rusqlite = 0.37.0` with defaults
off/no direct features. Both are already locked transitively. Sol may edit only
`test/securityPolicy.node.js` under
`docs/handoff/CODEX_SOL_BBD_WAL_006_SUPPORT_DEPENDENCY_TESTS_01.md`; manifest, lockfile,
policy implementation, and ZEC production remain frozen.

The test source is accepted at SHA-256
`f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`.
Luna may run only the focused expected-red handoff before any manifest change.

### Phase C2 — address production source (future, separate handoff)

The exact complete bounded production inventory is `wallet-broker/src/zec.rs` plus
`wallet-broker/src/zec/{address,fixture,prepare,scan,store,test_support}.rs`, with
`wallet-broker/src/lib.rs` limited to exposing `pub mod zec;`. The first production
slice is the address target and only its real fixture/store foundation, under
`docs/handoff/CODEX_SOL_BBD_WAL_006_ADDRESS_PRODUCTION_01.md`.
The test actor may not pre-create stubs, mocks, `compile_error!`, ignored tests,
conditional skips, or test-only alternate production.

## Planned commands and acceptance

Sol executes none of these. The Luna red handoff will name exact focused tests after test
review. The eventual green gate includes, from repository root, using the existing
disk-backed Cargo target and Rust 1.98.0:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --all-targets --all-features -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
npm run build
npm test
node scripts/security-policy.js
npm audit --audit-level=low
/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock
/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

The lock/feature graph, duplicate crypto primitives, build scripts, sources, licenses,
RustSec advisories, and scanner findings require reviewer inspection. A new advisory,
license denial, source violation, secret finding, live-network capability, or unreviewed
build script blocks acceptance; it is not waived by the importance of upstream Zcash.

At least five isolated falsifications are required with exact restoration:

1. request a UA that also contains P2PKH/Sapling and prove the composition test fails;
2. bypass previous-hash continuity and prove the discontinuity test fails;
3. mark Orchard value spendable and prove the migration test fails;
4. accept a non-v6/non-Ironwood prepared artifact and prove inspection fails; and
5. retain a prepared handle across lock and prove the hygiene test fails.

No platform package build, Electron launch, product binary, release artifact, or SBOM is
required. The existing manually triggered SBOM workflow remains configured but is not run
for this source ticket.

## Acceptance boundary

BBD-WAL-006 is complete only after accepted tests, fixture provenance, expected red,
production source, targeted/full/security green, falsifications, exact Git state, and
successful applicable GitHub checks. Completion proves offline ZEC viewing and unsigned
Ironwood PCZT preparation on synthetic non-mainnet data only. BBD-WAL-008 remains the
hardware-capability ticket, BBD-WAL-009 owns signing/verification/broadcast/recovery,
BBD-WAL-011 owns packaged native component/SBOM release evidence, and BBD-WAL-012 owns
mainnet authorization.
