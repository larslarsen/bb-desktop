# WAL-011 packaged broker launch configuration production 01

Actor: Grok Build, grok-4.6 High; production source only, no subagents.
Protected parent: reviewer publication following 118e6120f5d17f5702d2636cc772bf3663947553,
plus one CURRENT-only launch record. Read AGENTS.md, TESTING.md, ticket, this
handoff and CURRENT using sed -n '1,40p' docs/handoff/CURRENT_TASK.md.
Only write new wallet-broker/launch-config.js. It must be absent before editing.

Verify these frozen SHA-256 identities first:

| Path | SHA-256 |
| --- | --- |
| test/walletBrokerLaunchConfig.node.js | 1b6a7d8c102360efdbd0920b6211e7b55999fbdfff18aff4eee23cd311d750f8 |
| social-main.js | c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3 |
| wallet-broker/supervisor.js | c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| test/electronSecurity.node.js | 7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c |
| docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md | 1c98fde59cd882a406b6e73082f5fd5a1d8b7328cadc2a71e3d46941a3ae5485 |

Only the named source files are readable, plus the fixed resolver API section in
GROK_BBD_WAL_011_LAUNCH_CONFIG_TESTS_01.md and collected acceptance section in
HERMES_BBD_WAL_011_LAUNCH_CONFIG_RED_01.md. No historical fixture discovery, home
skills/config/credentials, graph tools or unrelated files. Separate read-only
HEAD/status/hash/count/existence commands and bounded named reads/searches only;
do not chain commands or inspect Git history/publication blobs.

Expected red is accepted: Node exited 1, zero ok/five explicit missing-resolver
assertions with unchanged inputs and absent production. Implement the fixed API
against the frozen test source; do not modify tests or create fixture artifacts.

## Exact implementation contract

CommonJS export: resolveWalletBrokerLaunch(options), synchronous. Use only Node
fs/path imports. No Electron, crypto, child_process, environment, discovery, network,
timer/listener, logging or side effects other than the specified filesystem reads.

1. Require a plain Object.prototype object with exactly four own data properties:
   resourcesPath, userDataPath, platform, arch. Reject arrays, null, other prototypes,
   accessors and extra/missing keys without evaluating accessors. Count all own keys,
   including symbols, when enforcing the closed options shape. Paths are nonempty
   absolute strings under the host path API, without coercion or input mutation.
2. Supported identities are linux/darwin/win32, each with x64/arm64 only. No alias
   conversion. This is format recognition, not a native-platform readiness claim.
   Fixed basename is bitbook-wallet-broker.exe for win32, bitbook-wallet-broker for
   linux/darwin. Never use a basename, relative path or launch settings from JSON.
3. The only layout is path.resolve(resourcesPath, 'wallet-broker'), containing
   manifest.json and that fixed executable. lstat the broker directory: require a
   directory and reject symlink. Require regular, non-symlink manifest and binary.
   Missing/type/I/O errors fail closed. The app-supplied resources root is trusted;
   do not expand this into an ancestor traversal or same-user race defense.
4. Manifest length is 1–4096 bytes inclusive. Check stat size before reading and
   actual Buffer byte length afterward. Read/parse UTF-8 JSON from manifest.json
   only. Require a JSON object with exactly v, platform, arch, sha256; v must be
   numeric 1, target fields exactly equal the validated input pair, digest a
   string of exactly 64 lowercase hexadecimal characters. Check length as well
   as character pattern so a trailing newline cannot pass a regex end anchor.
   No custom JSON parser or duplicate-member detection requirement. Reject every
   extra field, including executable/path/argv/env/dataDir overrides.
5. Return a frozen plain object with exactly brokerPath (resolved fixed package
   file), expectedSha256 (unchanged manifest string), and dataDir
   (path.resolve(userDataPath, 'wallet-broker')). Do not create/chmod data storage,
   modify any file or inspect/hash/read executable contents. Existing supervisor
   remains responsible for verifying bytes before spawning.
6. Every rejection or filesystem/parse error becomes Error with code UNAVAILABLE
   and message `Wallet broker is unavailable`, no cause, raw diagnostics, manifest
   contents, input paths or partial configuration attached. Standard source stack
   locations need not be stripped; do not attach the underlying I/O exception.

Keep this one module small and disconnected: do not import it from main, adjust
supervisor, package scripts/policy, generate real manifests or launch a process.
Preserve all pending npm/policy and evidence files. Future build-time pins must
come from final reviewed artifacts; no runtime self-pinning or target/debug fallback.

## Stop and later validation

No tests, syntax checks, Node/Cargo/npm execution, builds, dependencies, docs/evidence
changes, Git mutations, network tools or additional actors. Report path, SHA-256,
line count and implementation summary; then stop for source review.

Hermes validation follows separate authorization: node test/walletBrokerLaunchConfig.node.js
(five groups/58 rows), then node test/walletSupervisor.node.js (13 groups).
Falsification will suppress manifest/input target equality, require the mismatched
manifest assertion to fail, restore exact production and repeat targeted green.
No other suite replay or integration is authorized now. Packaging inventory repair,
build-time pin creation and main startup/status/quit composition remain later steps;
existing security failures are not waived and whole-ticket completion is not claimed.

## Collected source review — 2026-09-09

Outer 20952 completed exit 0, collected after the owner asked about the background
session. Grok session bd2da903-141e-486a-92a8-403ec942dbfb produced only
wallet-broker/launch-config.js: 132 lines, SHA-256
4a639b1644ff0ea618fc53fd525cbd5c0fde84fbea5905f3f3eb56a5520dc01a.
All six frozen input hashes match at collection. No tests were run by reviewer.

Source review holds acceptance: closedData at lines 24–30 performs Array.isArray,
Object.getPrototypeOf, Reflect.ownKeys and Object.getOwnPropertyDescriptors without
a containing normalization catch. A revoked Proxy throws TypeError, and reflection
traps can throw an arbitrary Error with raw diagnostics. resolveWalletBrokerLaunch
line 88 lets those escape without the required UNAVAILABLE code and fixed message.
This violates contract item 6; it is not a claim of renderer reachability or exploit.
Ordinary options, inventory, size, digest, target match and immutable output paths
otherwise follow the fixed contract. Keep this production drop frozen for a focused
regression red before correcting the public error boundary.

The export shows named reads, hash/count/status checks and one source edit. It also
shows one prohibited chained existence check (test -e followed by echo), and an
extra read of the named frozen evidence contents. No execution, Git mutation,
home-skill discovery or unrelated source write appears. Do not repeat those scope
deviations. The final report's “no Git operations” means no mutations; read-only
Git checks are visible. Grok production authorization is closed. Only the
[reflection regression tests](GROK_BBD_WAL_011_LAUNCH_CONFIG_REFLECTION_TESTS_01.md)
are now authorized.
