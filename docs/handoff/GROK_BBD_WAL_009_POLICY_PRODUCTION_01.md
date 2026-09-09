# WAL-009 exact mapping and wallet production policy correction 01

Actor: Sr Dev — Grok Build, grok-4.6 High, no subagents.
Reviewer: Codex; High is sufficient. No execution/integration actor in this phase.
Protected parent: reviewer commit publishing this handoff, directly following
58423f75a9caf7e3d538772e79733713acd6aeef; one CURRENT-only launch allowed.

## Accepted red and fixed behavior

The corrected 90-test policy suite exited 1 with 14 failures. All three new positive
contract groups visibly fail at the old runtime/dependency inventory guards; the
reviewed acceptance and capture limits are in
HERMES_BBD_WAL_009_POLICY_RED_AND_NATIVE_LINT_01.md, collected acceptance appendix.
Native-ui Clippy now passed; Rust source/formatter/library lint work is retained.
No further tests or proofs are needed for this source-writing phase.

Owner decision: keep MapLibre and remove Leaflet. Retain the existing locked
MapLibre 6.8.0 and pin its direct declaration to exactly 6.8.0. The only allowed npm
runtime dependency is that exact mapping entry. This adds no map UI, map endpoint,
location collection, renderer/sandbox permission or wallet authority.

The reviewed Rust manifest already contains Orchard and the signing test target.
The production validator's exact inventories must match those reviewed inputs.
Do not alter the Rust manifest, lockfile, source or historical feature constants.

## Exact writable scope and baseline

Read AGENTS.md, TESTING.md, this task and only active CURRENT/ticket prefixes. Do
not reload old evidence or directories. Verify these source identities using only
read-only file measurements before editing:

| Writable path | Logical lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2733 | bd8202bbc39760abdef8cecd394e2ac3d7bc8b97533781e7ff91fb18b1f4b943 |
| package.json | 42 | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |

Useful frozen reads: test/securityPolicy.node.js (3643 lines, SHA-256
fe31a4cbab5c3c179a02812dd892542dcc5bbe9d604435b3609242fa6b7f58c7),
wallet-broker/Cargo.toml, and package-lock.json. The modified test file and untracked
policy-red evidence are accepted inputs owned by previous actors; do not touch them.
No test, manifest/lock beyond package.json, Rust, documentation or evidence edits.

## Four bounded edits, no general refactor

1. In checkPackageJson replace only the old runtime-dependency rejection block.
   Require dependencies to be a non-null, non-array object with exactly one own
   enumerable key, maplibre-gl, whose value is the string 6.8.0. Reject missing,
   empty, malformed, Leaflet/extra entries, range/alias/URL declarations and all
   other versions. Use a single local guard and a clear PolicyError message containing
   runtime dependencies and maplibre-gl. Do not add fallbacks or skip any later
   script/build/security checks. Electron/devDependency checks stay exact.
2. In checkWalletBrokerManifest's expectedDependencies array, insert exactly this
   literal after the existing pczt line, matching the reviewed Cargo manifest:
   orchard = { version = "=0.15.5", default-features = false, features = ["circuit"] }
   Retain exact ordered dependency comparison and duplicate/displaced-declaration
   detection. Do not relax validation, change WAL-006 historical exported constants,
   allow arbitrary crates/features, or edit other inventories.
3. In that function's expectedTests array insert exactly
   zec_sign_verify:tests/zec_sign_verify.rs immediately after
   zec_hardware:tests/zec_hardware.rs and before xmr_distribution. Preserve exact
   ordered comparison and every other target.
4. In package.json only, remove leaflet and change maplibre-gl from ^6.8.0 to 6.8.0.
   Preserve every other field, script, dependency and formatting convention.
   Do not update package-lock.json by hand or with npm. The next Hermes task will
   synchronize that lock from reviewed package source without upgrading MapLibre
   or unrelated locked resolutions. Temporary package/lock mismatch is intentional
   until that bounded integration step, not permission to touch another file.

No test/syntax/formatter/build/scanner execution, Cargo/npm command, Git command,
network, delegation, tool installation or extra source change. File reads/searches
and read-only before/after identity measurements are allowed. Source only.

## Stop and next validation

Report the two exact changed paths, logical line counts and SHA-256, the four edits
and any unresolved concern. State whether any unauthorized command occurred. Stop
for reviewer source acceptance; do not author evidence, integrate or continue.

After review Hermes will synchronize package-lock.json, run the same policy suite
and policy CLI, execute a bounded falsification of the new runtime guard, restore
it exactly, and perform package/lock security validation and source integration.
Those exact commands and paths will be authorized in the later handoff. No accepted
Rust gate is repeated for this npm/policy-only correction. Full typed-secret erasure
and final release/OS/security remain separate open requirements. High is sufficient.
