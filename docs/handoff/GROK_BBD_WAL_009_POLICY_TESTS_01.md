# WAL-009 exact mapping and wallet policy regression tests 01

Actor: Sr Dev — Grok Build, grok-4.6 High, no subagents.
Reviewer: Codex; High is sufficient. Execution/integration actor: none in this phase.
Protected parent: reviewer commit publishing this task, directly following
43e3357cc93507f1ee5f8f40c913c418f876bc46; one subsequent CURRENT-only launch allowed.

## Fixed contract

The owner chose MapLibre for mapping and removal of Leaflet. Keep the currently
locked MapLibre version 6.8.0 and pin the direct declaration exactly to 6.8.0 in the
later production drop. This is dependency policy only; no map UI, map server,
location collection or renderer/sandbox authority is added. No general dependency
allowance: the sole runtime entry must be maplibre-gl at exactly 6.8.0. Electron and
all existing script/security requirements stay fixed. Package/lock updates and npm
security checks are a later Hermes integration step after reviewed source and red.

The Rust source checkpoint is reviewed. Its manifest already contains the exact
orchard = { version = "=0.15.5", default-features = false, features = ["circuit"] }
entry and zec_sign_verify:tests/zec_sign_verify.rs immediately after zec_hardware.
The policy's exact manifest inventory is stale. Correct that inventory later, not
the reviewed Rust manifest. Existing CI identified 11 policy failures: six package,
four dependency inventory and one stale WAL-008 target-order assertion.

## Exact source scope and baseline

Read AGENTS.md, TESTING.md, this handoff, and only active CURRENT/ticket prefixes.
Only writable path: test/securityPolicy.node.js.
Baseline: 3358 logical lines; SHA-256
464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07.
Verify it with a read-only measurement before editing. Useful frozen reads:

- scripts/security-policy.js: 2733 lines, SHA-256
  bd8202bbc39760abdef8cecd394e2ac3d7bc8b97533781e7ff91fb18b1f4b943.
- package.json: 42 lines, SHA-256
  84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780.
- package-lock.json: 396 lines, SHA-256
  5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc.
- wallet-broker/Cargo.toml and the existing test helpers/inventory assertions.

No production policy, package, lockfile, Rust, documentation or evidence edits.
No test/syntax/formatter/scanner/build execution, Git, downloads, other actor or
CI queries. Read/search tools and read-only identity measurements are allowed.

## Author tests only, preserving independent oracles

Use the existing custom test harness, loadPolicy, assertRejects and replaceOnce.
Do not modify the harness, weaken unrelated assertions, skip tests, derive expected
values from policy exports, or globally sanitize the real package fixture. Existing
real-repository tests must keep detecting Leaflet until the authorized removal.

1. Add a WAL-009 package regression group using a fresh in-memory copy of the actual
   package with only dependencies replaced by the literal reviewed map. Assert
   acceptance of {"maplibre-gl":"6.8.0"} through checkPackageJson. From independent
   fresh copies assert rejection of: missing dependencies, empty map, Leaflet alone,
   Leaflet alongside MapLibre, an arbitrary extra dependency, ^6.8.0, ~6.8.0, another
   version, npm alias, git/file URL and a non-string version. Require real mutations
   and a meaningful package/dependency/version/pin rejection; do not accept unrelated
   script errors as the oracle. Keep the positive assertion before negative cases so
   the current reject-everything implementation cannot masquerade as valid coverage.
2. Add a WAL-009 manifest regression group accepting the actual reviewed manifest,
   then rejecting independent mutations of the literal Orchard declaration: omitted,
   unpinned/wrong version, enabled defaults, absent/extra circuit feature, duplicate,
   and displaced into a dev-dependencies section. Preserve exact other dependencies.
   Assert the mutation matched once before calling the production validator. Do not
   remove Orchard to make older tests pass or extend historical WAL-006 constants.
3. Add WAL-009 signing-target coverage: accept the actual reviewed manifest and reject
   removing, renaming, duplicating, changing the path of, or moving zec_sign_verify
   outside its exact hardware/signing/xmr neighborhood. Retain existing target checks.
4. Correct only the stale WAL-008 neighborhood oracle: require the exact four entries
   zec_hygiene, zec_hardware, zec_sign_verify, xmr_distribution, with their exact test
   paths. Update the corresponding slice upper bound and explanatory text; keep all
   WAL-008 hardware/source-inventory negatives. This is a reviewed inventory change,
   not a blanket weakening of ordering.

All comparisons exercise production validators, with literals independently fixed
here. No new cryptographic tests or Rust execution are involved.

## Execution plan reserved for the next Hermes handoff

After reviewer source acceptance, Hermes will run node test/securityPolicy.node.js
once for the expected red, and the missing native-ui Clippy once on unchanged Rust:

"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib -- -D warnings

Do not run either now. Expected policy red is the exact package/dependency/target
mismatch, including new positive contract tests; no syntax/setup failure is accepted.
The later green uses the same policy suite plus its CLI, package/lock checks and npm
security validation. A later falsification will bypass the new runtime guard and
prove a negative mutation fails, then restore it; manifest list mutations are already
explicit negative coverage. Exact execution/falsification commands will be authorized
only after source review. No replay of accepted Rust tests/formatter/secret scan now.

Stop after the single-file test drop. Report the exact file, logical lines, SHA-256,
new test groups and residual concern. State no execution or Git occurred. No evidence
or next task. Reviewer alone accepts and authorizes production after observed red.

## Collected source review and Correction 01 — 2026-09-09

Outer 66797 closed on owner done, exit 0; Grok session
b0c5ac17-b70d-4f51-a5f4-392a48012d63. The one-file drop is 3649 logical lines,
SHA-256 812ba42bf440044cc58da45e69ded6913c23f3ba1729157b49f55834c853ff5f.
Reviewer reconstructed the exact current file from all four unique transcript
replacements. Frozen production policy/package/lock hashes are unchanged. Transcript
contains 32 reads, 12 searches, four replacements, three todo updates and two
identity command batches. Those batches included read-only Git status/HEAD/log
outside the source actor's no-Git instruction; no Git mutation or execution gate ran.

The three new groups and strict four-entry target neighborhood follow the fixed
contract. Positive assertions precede negatives, literal oracles are independent,
and signing rejections cannot be satisfied by unrelated dependency failures.
Orchard displacement, target move and independent copies are source-reviewed.
Do not execute this drop yet: its package fixture has a deterministic green blocker.
wal009PackageJsonWithDependencies asserts its result differs from the actual package,
including for the valid fixture. Once the package is correctly pinned, that assertion
would fail before checkPackageJson. Additionally, four existing Pay/RATE package and
lock-root assertions still require undefined dependencies. The reviewer previously
froze these as unrelated assertions; the MapLibre decision requires their exact
inventory transition now. Both findings are test-source defects, not production bugs.

Authorize Grok Build High for one bounded Correction 01 in the same file, from the
3649-line hash above. Protected parent is the reviewer commit publishing this
appendix with one subsequent CURRENT-only launch commit allowed. All other source,
execution and Git mutation remain closed. The previous actor authorization is closed;
this is one new launch of the following correction only:

1. In wal009PackageJsonWithDependencies remove only the originalDependencies local
   and its assert.notDeepStrictEqual block. Keep fresh JSON parsing, explicit missing
   dependency deletion, assignment and serialization. A valid fixture may equal the
   real package. All negative fixtures are already distinct literal deviations from
   the independently fixed approved map; keep all their production validator calls,
   rejection needles and positive-before-negative order.
2. In the WAL-005 Pay package test and RATE-001 quote-worker package test, change only
   the two dependencies assertions apiece (package and lock root). Replace strict
   equality to undefined with deep equality to the literal
   { 'maplibre-gl': '6.8.0' }. Use literal objects at these sites, not the later
   WAL009_RUNTIME_DEPENDENCIES const: the custom harness executes Pay before that
   const is initialized. This continues to reject current Leaflet/range declarations.
   Preserve Electron/devDependency, script, build, workflow, import and policy calls.
   Use messages respectively: 'Pay package must retain only the reviewed mapping
   dependency', 'Pay lock root must retain only the reviewed mapping dependency',
   'quote worker package must retain only the reviewed mapping dependency', and
   'quote worker lock root must retain only the reviewed mapping dependency'.
3. No other test or production change. Report the corrected file identity and stop.
   No tests, syntax checks, formatting, compiler, scanner, Git command, documentation,
   network, delegation or integration. Read-only file measurements are sufficient.

The reviewer will collect once on owner done/Continue, then authorize the policy
expected-red plus missing native-ui Clippy together. No accepted gate is replayed.
High is sufficient. Reviewer publication scope: this handoff, CURRENT_TASK.md and
tickets/BBD-WAL-009.md only.
