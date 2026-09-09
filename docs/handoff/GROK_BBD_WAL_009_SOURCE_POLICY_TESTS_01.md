# WAL-009 current Zcash inventory and exact extractor policy tests 01

Actor: Sr Dev — Grok Build, grok-4.6 High; no subagents.
Reviewer: Codex; High is sufficient. Source tests only, no execution/integration.
Protected parent: publishing reviewer commit directly following 6613707e, with one
subsequent CURRENT-only launch allowed. Read AGENTS.md, TESTING.md, this task and
active CURRENT/ticket prefixes; no historical reload.

## Fixed contract and baseline

The prior two-file mapping/manifest correction is accepted and frozen. The actual
repository policy still rejects the reviewed WAL-009 source layout and two exact
transaction extraction statements. Add tests for both in one bounded file now.
Production source and lock synchronization remain for later phases after observed
red. No new wallet behavior, cryptographic design or broad authority exception.

Only writable path: test/securityPolicy.node.js. Current baseline is 3643 lines,
SHA-256 fe31a4cbab5c3c179a02812dd892542dcc5bbe9d604435b3609242fa6b7f58c7.
Verify before editing. Useful frozen reads: scripts/security-policy.js (2738 lines,
fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078), reviewed Rust
sources below, package/lock. The modified package/policy and untracked red evidence
belong to accepted earlier drops; do not touch them. No docs/evidence/Git/npm/Cargo,
syntax/test/formatter/scanner command, network or actor. Read/search and read-only
file identity measurements only.

## Exact current inventory, preserving all negative checks

Extend the existing test-side WAL008_ZEC_RUST_SOURCE_PATHS literal to the following
14 current sorted paths. Retain its existing name and checker API for compatibility;
update the existing test title/text and expected count from eight to fourteen to
state the current WAL-008/WAL-009 inventory. Keep historical WAL006 constants intact.
The production export/checker will later be updated to the same independently fixed
current list, not changed in this phase.

- wallet-broker/src/zec.rs
- wallet-broker/src/zec/address.rs
- wallet-broker/src/zec/fixture.rs
- wallet-broker/src/zec/hardware.rs
- wallet-broker/src/zec/prepare.rs
- wallet-broker/src/zec/scan.rs
- wallet-broker/src/zec/spend.rs
- wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs
- wallet-broker/src/zec/spend/effects.rs
- wallet-broker/src/zec/spend/external_binding_tests.rs
- wallet-broker/src/zec/spend/verification_context_tests.rs
- wallet-broker/src/zec/store.rs
- wallet-broker/src/zec/test_support.rs
- wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs

Preserve real recursive discovery and exact comparison to this literal list, order,
duplicate, malformed, extra/unlisted, hardware and missing checks. Extend omission
coverage to each of the fourteen paths, not just hardware. Keep all actual source
files unchanged. Do not weaken discovery to a subset or ignore nested test modules.

## Exact extraction statements and non-vacuous mutation tests

Add two separately named test groups using checkRustWalletSource on actual files:
production wallet-broker/src/zec/spend.rs and test-only
wallet-broker/src/zec/spend/verification_context_tests.rs. The future checker may
recognize only one occurrence of its exact reviewed statement at each exact path,
mask that exact statement only for the legacy extraction-pattern check, and still
apply every general unsafe/network/authority/mainnet check to the original source.
No whole-file whitelist, disabled scan or generic per-path extraction allowance.

Independent expected production statement (including the reviewed error mapping):

    let transaction = TransactionExtractor::new(finalized)
        .extract()
        .map_err(|_| ZecError::signature_invalid())?;

Independent expected verification-fixture statement:

    let transaction = TransactionExtractor::new(finalized)
        .extract()
        .expect("extract transaction");

For each file, first assert that the literal reviewed statement occurs exactly once
and that the actual source is accepted by the production checker. Then use fresh
mutations of that source to assert rejection, with meaningful authority/extract/
network/mainnet/unsafe diagnostics (never a missing fixture or unrelated error):

- append a second copy of its reviewed statement;
- replace TransactionExtractor::new(finalized) in that statement with an unreviewed
  constructor while retaining .extract();
- change its reviewed error mapping/expect text while retaining .extract();
- swap in the other file's reviewed extraction statement;
- append each independent pczt.sign(key), pczt.prove(), pczt.extract(),
  pczt.finalize(), broadcast(raw_transaction), std::net::TcpStream import,
  Network::MainNetwork assignment, and unsafe {} mutation;
- present the unchanged reviewed source as wallet-broker/src/zec/unlisted.rs,
  proving the exception cannot move to another path.

Assert mutation targets are unique and bytes really change. Preserve positive-
before-negative order and all existing tests/harness behavior. Literals must not
come from production exports. No new test-support mock or cryptographic execution.
The existing generic forbidden-operation tests remain strict on other paths.

## Stop and next validation

Report exact test source hash/lines and new group names; stop. No execution or Git.
After source review Hermes will synchronize only the accepted npm graph and collect
this suite's expected red and CLI findings; exact commands will be in its handoff.
The remaining source-policy production change follows understood red. All previous
verified Rust gates remain retained and will not be repeated. No report-only task.
