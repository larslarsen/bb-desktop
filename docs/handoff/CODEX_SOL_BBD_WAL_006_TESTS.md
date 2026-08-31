# Codex Sol Handoff — BBD-WAL-006 Test Source

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Baseline: `363c0046`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`,
`docs/engineering/WALLET_ROADMAP_ROUTING.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md` §§1.1, 2, 3 T23/T30, 5.3–5.4, 6, 7, 10,
13, 15, 17, and 19, `docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`,
`tickets/BBD-WAL-006.md`, `docs/handoff/CURRENT_TASK.md`, current
`wallet-broker/Cargo.toml`, all current `wallet-broker/src/*.rs`, all accepted WAL-004
Rust tests, the WAL-002 contract fixtures/tests relevant to amounts, timestamps, memos,
capabilities, intent hashes, and migration errors, `test/securityPolicy.node.js`, and
every path you edit.

Your sole task is BBD-WAL-006 Phase A: author test source and the exact manifest additions
before any ZEC production source. The ticket and upstream review fix the product and
security semantics. Do not weaken, reinterpret, or widen them.

Authorized paths are exactly:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/zec_fixture_builder.rs`
- `wallet-broker/tests/zec_address.rs`
- `wallet-broker/tests/zec_store.rs`
- `wallet-broker/tests/zec_scan.rs`
- `wallet-broker/tests/zec_prepare.rs`
- `wallet-broker/tests/zec_hygiene.rs`
- `test/securityPolicy.node.js`

Use `apply_patch`. You may perform read-only inspection and report `wc -l` and
`sha256sum` over only those eight paths. Do not execute Rust, Cargo, Node, npm, tests,
builds, formatters, scanners, Git, GitHub, network, Electron, native windows, child
processes, fixture generators, wallets, nodes, hardware, or devices. Do not install,
resolve, update the lockfile, create fixture output, create scratch state, or clean up.
Do not use root, `sudo`, `/tmp`, `rm`, deletion, globs, variables/substitutions as
destructive targets, or an unlisted path.

The manifest must preserve package identity, edition, Rust 1.98.0, existing exact custody
pins, and the exact `native-ui` feature. Add the six exact stable Zcash pins from the
ticket with defaults disabled, the minimum explicitly named upstream feature union, and
six explicit integration-test targets. If the published feature/API combination is
incompatible with the fixed versions, stop and report the exact contradiction instead of
guessing, loosening, or substituting. Do not add an RC, git source, patch, wildcard,
transport, test framework, downloader, general network client/runtime, OpenSSL, FFI,
binary, or unrelated dependency.

The fixture builder is test-only upstream-oracle code. It must be independently
invocable after later authorization, import no future BitBook ZEC adapter, open no
network, use the injected local consensus schedule, produce deterministic compact-block
bytes and a closed manifest only beneath the future explicit
`wallet-broker/target/wal006-fixture-build` path, refuse any other output location, and
never delete or recursively clean a path. It must cover every fixture scenario named by
the ticket. Sol does not run it or create the output.

The other five Rust suites reserve the smallest coherent future adapter API. Prefer
typed constructors and opaque owners over strings/generic maps. Test behavior and decoded
upstream structures, not production source text. Do not expose raw PCZT bytes merely to
make assertions; place a narrow read-only test inspector behind a compile-time test seam
that returns only the exact non-secret decoded fields required by the ticket and is not
reachable from broker IPC. Inject clock, fixture source, filesystem/fault boundary, and
wipe observer only where deterministic proof requires them. No injected port becomes a
generic wallet or network port.

The tests must be non-vacuous and must not contain a stub library, mock production ZEC
adapter, `compile_error!`, ignored test, conditional skip, tautological comparison,
production result reimplementation, self-generated expected result, or acceptance based
only on surviving a call. Use fixed expected values from the reviewed upstream fixture
and existing WAL-002 independent vectors. The expected-red adapter failure will be an
absent `bitbook_wallet_broker::zec` API after Luna separately proves the upstream fixture
builder compiles and freezes its output.

`test/securityPolicy.node.js` may add only dependency/capability-policy assertions for
BBD-WAL-006. Preserve every existing assertion and exact ratchet. It must require the six
exact direct versions with defaults disabled; reject loose/git/patched/live-network
dependency declarations and unlisted Rust ZEC source paths; and remain truthful that
upstream PCZT enables compiled transparent/signer/prover/finalizer/extractor capabilities
while BitBook's adapter surface forbids their invocation. Do not claim those transitives
are absent. Do not edit `scripts/security-policy.js` in this phase.

Cover every required group and below/at/above boundary in the ticket. In particular,
prove receiver composition by decoding it; prove durable monotonic receiver issuance and
concurrency; inspect SQLite state for forbidden secret persistence; exercise corruption,
compound failure, discontinuity, replay and one-block reorg; distinguish Ironwood
spendable from Orchard migration-required value; decode and inspect v6 branch/Ironwood
PCZT semantics; prove absence of raw/sign/prove/finalize/extract/broadcast/network/mainnet
authority; and observe handle/secret invalidation on every named lifecycle edge.

Stop after authoring and report:

- each changed path, line count, and SHA-256;
- every test name and total by Rust/Node suite;
- the exact future Rust API reserved by the tests and why each exposed item is necessary;
- every dependency version/feature and the expected upstream feature union;
- the fixture generator inputs, local activation schedule, scenarios, outputs, and why
  its oracle is independent of future production;
- every independent expected value/vector and its provenance;
- why each test group is non-vacuous and the exact expected-red missing behavior;
- the existing Node policy assertion count before/after and proof none was removed;
- every limit and immediate below/at/above case;
- confirmation that no secret, valid mnemonic, user path, mainnet material, live endpoint,
  raw broadcastable transaction, or raw prepared PCZT crosses a public boundary; and
- confirmation that no command outside the allowed read-only/reporting set ran and no
  unlisted path changed.

Lead Engineer/Reviewer — Codex at XHigh must inspect and accept the exact source hashes
before Codex Luna resolves dependencies, touches `Cargo.lock`, runs the fixture builder,
creates fixture bytes, or executes expected red. You have no production, fixture-output,
execution, integration, evidence, Git, commit, push, install, or dependency-resolution
authority.
