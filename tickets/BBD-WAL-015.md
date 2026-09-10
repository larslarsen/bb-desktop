# WAL-015 — Native live Zcash testnet balance synchronization

Status: COMPLETE. Source edda35308913d59683a08be58052dc7c2d95dbaf; evidence 105a2e36af03d26dbc15a3d052901d836b2a63b5.
Reviewed baseline e85dfdc8b6e06fdd09f84cee3c6ac37037448437.

Owner delegated server/privacy choices after asking to continue. The reviewer chooses
an editable suggested testnet HTTPS endpoint, direct TLS with native IP disclosure,
and explicit Sync/Cancel actions. Do not ask the owner to choose implementation details.
Deliver actual live transport, upstream scanning, durable progress and native balances.
Mainnet, broadcast, signing, social/daemon and packaging changes remain outside scope.

## Fixed design

Use the maintained pinned upstream compact-block scanner and WalletDb. Keep the
existing receive wallet.sqlite3 and compact.sqlite3 byte-compatible. Their fixture
recovery invariant cannot safely accommodate a live rolling cache. Add one separate
viewing cache file live.sqlite3 in the same validated network/account directory,
containing the official viewing wallet and versioned live checkpoint metadata. This
is derived viewing state for the existing account, not a new account or custody store.
It must never be used implicitly by the older payment/fixture APIs. No vault migration.

Validate existing receive storage/UFVK using the authenticated session seed, reuse
account initialization without issuing an address, then give the worker only the
bound UFVK, account ID and validated cache location. No session seed or passphrase
copy survives preparation. Never extend the idle lock deadline. Only NativeSurface
may start a sync; locked/unknown/expired accounts fail before any source request.

One owned worker at a time per manager; network/scanning must not hold its mutex.
Worker owns its SQLite connection and bounded source. UI polls a bounded latest-state
slot, not an unbounded event queue. Cancel on lock/expiry/hide/quit/drop, suppress old
job results, join owned worker before manager root guard is released. Cancel must
interrupt transport waits; do not detach threads or introduce an independently
listening wallet service. Limit individual scan batches so cancellation is checked
between bounded CPU operations. Cancel never deletes committed viewing data.

Suggested endpoint remains editable and is researched by reviewer. Native endpoint
validation: HTTPS only, <=2048 ASCII bytes, nonempty DNS/IP host and valid nonzero
port, no userinfo/query/fragment or non-root path. No environment proxy/CA override,
certificate bypass, redirect or silent endpoint fallback. Use tonic transport with
WebPKI trust roots and standard hostname verification. Connection/request timeout
15 seconds; cancel-aware stream deadline, not merely time to response headers.
Only GetLightdInfo, GetLatestBlock, GetTreeState, GetBlockRange are authorized RPCs.
No address/transaction/UFVK request or generic upstream sync::run.

Scan shielded receive pools Orchard + Ironwood from upstream Testnet NU5 activation;
the product issues only Orchard receivers, so this covers their entire possible
history, including restored accounts. Do not default to today's tip. Label balances
as received shielded funds; do not claim a complete Sapling/transparent total. Use
upstream three-confirmation summary and distinguish confirmed/pending; never label
an incompletely scanned or failed request as a fresh zero or as sendable funds.

Capture a target tip for each run. Validate server Testnet chain and the pinned
consensus schedule at its tip, checked u32 heights/hash lengths, requested TreeState
network/height/hash and all block heights/prev_hash links. GetTreeState uses the
upstream conversion (RPC hashes are reversed hex); no handwritten tree or crypto.
Server reports are light-client observations, not full consensus verification.

Default batch 100 blocks, each <=2 MiB and aggregate <=32 MiB, exact requested count,
no duplicates/gaps/extra blocks or unbounded collection. TreeState strings <=4096
bytes each before decoding. Existing live db is opened and validated, never reset
or reinitialized on corrupt/partial state. Validate ancestors before descendants,
owner/mode 0700 dirs /0600 regular files, no links/hard links or hostile sidecars.
Create only the fixed cache filename; retain upstream SQLite recovery semantics.

Import exactly one upstream viewing account, bound to UFVK/network/account/birthday.
Persist scan range/checkpoint and official scan writes in one SQLite transaction;
on restart continue from the committed height. All progress is provisional until
commit. Supply in-memory BlockSource to scan_cached_blocks, discard each batch after
commit. Revalidate saved tip against server before extending it. For a fork, find
a common stored checkpoint within 100 blocks and use upstream truncation in the
same transaction as replacement scanning; if none exists, stop with a fixed rescan
required status. Preserve receive issuance and vault bytes across every sync path.

Native flow: selected unlocked account -> Sync balance screen -> editable server,
short connection disclosure, Sync, visible scanned/target heights, Cancel, and result.
UI remains responsive. Show unsynced before first completed scan; partial/stale state
clearly labeled and not painted as current. Lock/selection change/error/hide clears
private scene and invalidates results. Existing Receive and Copy continue to work.
One suggested service plus Custom satisfies the selectable endpoint decision.
Snapshots carry account ID, network and job ID; validate all three before painting a
result. Display exact eight-decimal ZEC amounts using integer division/remainder,
such as “Confirmed: 1.90000000 ZEC”, rather than raw internal units or floating point.

## Ownership and authorized paths

Escalation: prior WAL-014 Grok grok-4.6 High session
01a08cf5-b5e8-7f21-b00f-a0f774282253 stopped with no usable test drop. This task adds
concurrency, adversarial transport and durable scanning to that work; reviewer judges
the default actor insufficiently reliable for this bounded task. Route Principal Dev
Sol gpt-5.6-sol High. No actor may expand roles or spawn another actor.

Tests first: wallet-broker/tests/zec_live_sync.rs (new),
wallet-broker/tests/account_management.rs, wallet-broker/tests/account_native_ui.rs.
Additional parallel transport test-source: wallet-broker/tests/zec_live_transport.rs
and wallet-broker/tests/fixtures/live-grpc-server.js, per the dedicated handoff.
Additional compact preflight regression source: wallet-broker/tests/zec_live_validation.rs
under SOL_BBD_WAL_015_VALIDATION_TESTS_01.md.
Actual hot-journal recovery regression: wallet-broker/tests/zec_live_recovery.rs
under SOL_BBD_WAL_015_RECOVERY_TEST_01.md.
Production later: wallet-broker/src/zec/live.rs (new),
wallet-broker/src/zec/live_transport.rs (new), wallet-broker/src/zec.rs,
wallet-broker/src/zec/test_support.rs, wallet-broker/src/accounts.rs,
wallet-broker/src/account_ui.rs, wallet-broker/Cargo.toml. Minimal crate-private
visibility changes only in wallet-broker/src/zec/store.rs if needed to reuse validators.
The one-line crate-private visibility change to scan.rs::derive_chain_states is
authorized to reuse upstream fixture tree derivation. The existing orphan_projection helper/struct and Orchard/Ironwood fields may also
be exposed crate-private under SOL_BBD_WAL_015_REORG_CORRECTION_01.md. No other
scan.rs behavior changes.
Hermes alone resolves Cargo.lock and runs formatters/builds/tests/security/integration.
No other source, fixture, npm or policy edits. Artifacts stay in wallet-broker/target;
reviewer confirmed its filesystem ext4. User profile and running user app are untouched.

## Verification contract

Targeted RED and GREEN (Hermes only):
`rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_live_sync`
and the same command with `--test account_management wal015_`, and with
`--features native-ui --test account_native_ui wal015_`.
Expected initial red is missing live-sync APIs, not unrelated errors. Source actor
may propose Rust API names/types in tests; reviewer fixes them before production.

Real recorded block bytes must enter the production scan loop with upstream SQLite
and an independently known nonzero balance, resume/idempotence and reorg results.
Use deterministic local-consensus fixture only through an explicit test seam that
calls the same generic scan engine; production manager always fixes Testnet.
Test malformed responses, cancellation, foreign UFVK/corrupt cache, no source calls
before native/unlock gates, no address issuance and unchanged vault/receive files.
Native tests use actual pointer/paint events for start/progress/cancel/stale results.
Transport needs deterministic local gRPC boundary proof once dependencies are added;
no ordinary test may require a public server. No unbounded waits or recursive cleanup.

Falsify account-binding/continuity or transaction rollback/recovery in the actual engine;
meaningful test must fail, restore exact bytes and rerun targeted tests. Broader green:
full account_management, account_native_ui, vault_session, native_surface, zec_scan,
zec_address, zec_store, zec_hygiene; production Clippy (--lib --bin
bitbook-wallet-broker --features native-ui -- -D warnings); existing 28 distinct JS
wallet groups, staged native rebuild and Xvfb/actual-host window proof. Exact pinned
commands and hashes go in reviewer execution handoff after source review.

Fresh Cargo dependency audit required for changed graph (pinned cargo-audit, no new
vulnerabilities); preserve prior unmaintained atomic-polyfill warning baseline.
Pinned Gitleaks dir/git must pass, and security-policy checks must retain only the
six inherited failures. Prior test-helper Clippy warnings are not production failures.
No installer/SBOM rebuild. Actual public metadata-only smoke may be separately
authorized for the suggested service, without any account or user data.

## Manifest policy follow-through
GREEN02 surfaced7additional policy failures from the reviewed new dependency declarations.
SOL_BBD_WAL_015_POLICY_01.md authorizes tests-first exactmanifestpolicy updates in
scripts/security-policy.js, test/securityPolicy.node.js and new
test/walletLiveSyncPolicy.node.js. Preserve unrelated prior edits by reversible exact
deltas and partial index integration only; no npm changes. Restore inherited6failure
ratchet with independent manifest authority mutation tests before acceptance.

## Exact non-secret scanner finding
Owner: Lead Engineer/Reviewer, WAL015. Pinned Gitleaks8.30.1 generic-api-key falsely
classifies the SHA256 of the unchanged public hygiene-test source when repeated in
WAL015 execution-record hashmaps. Gitleaks rules/config/ignore file stay unchanged.
Acceptance requires zero actual secrets: every retainedfinding must have the exact
recomputed publictest-file checksum as capturedSecret and a WAL015 generatedrecord
path. Any different value/rule/path blocks. Directory and committed scans must undergo
this exact-value check; never describe nonzero scannerexit as clean. Remove this
classification when those archived checksum records no longer trigger the scanner;
it grants no exception to changed filechecksums or other records. A lossless filename
serialization attempt did not remove the inference and was reverted byte-for-byte.

The exact captured-value triage also identified the three public diagnostic-record
checksums pinned in HERMES_BBD_WAL_015_INTEGRATION_01.py. Each was independently
recomputed from its unchanged file. Only these four exact public file-checksums in
WAL015 generated-record paths are classified as non-secret; every other finding fails.
The scanner still exits1 for these matches; this is explicit reviewer triage, not a
claim that its default scan returned0. No scanner config/rule/ignore changes.

## Acceptance
111distinctRusttests,28appJSgroups,33manifestpolicygroups, productionClippy, real
crash-recoveryfalsification and exactrestore, nativeUIpointerproof, rebuiltbroker and
Xvfb/actualdesktopwindowproof accepted. Cargoaudit found no newvulnerabilities; old
atomic-polyfill unmaintainedwarning remains. Six inheritedpolicyfailures remain.
Integration verified37committedblobs, preserved original unrelatedworkingedits using
partialpolicy staging, and pushedsource/evidence. Exactpublic-checksum triage proved
zero actualsecrets despite nonzeroGitleaksexit. See CURRENT_TASK finalbinarypin and
BBD-WAL-015-LIVE-INTEGRATION-01.md for rawcommands/counts/hashes/actormetadata.
Owner can restart and use Sync balance after unlocking. No userprofilemodified.

Finalgovernance scan found two additional copies of the public diagnostic-handoff
Markdown checksum. Reviewer independently verified both and accepted82totalpublic
checksum matches,0actualcredentials. Fifth exactpublicfile and removalconditions are
recorded in docs/architecture/BBD-WAL-015-FINAL-SCAN-REVIEW.md. No scanner rule/config/
ignore changes and no blanket path exception. All actors stopped; ticketcomplete.
