# WAL-011 finish application startup — 2026-09-10

Reviewer scope: owner asked to finish the app-to-broker connection; prioritize usable
local startup, no further installer work. Actor Grok grok-4.6 High, no subagents.
Read AGENTS.md, TESTING.md, this contract and CURRENT first 24 lines only. Source
reads limited to social-main.js, wallet-broker/{supervisor,launch-config,protocol}.js,
wallet-pay/model.js, test/{electronSecurity,walletBrokerRuntime,walletBrokerLaunchConfig}.node.js,
wallet-broker/{Cargo.toml,src/main.rs}, and the fixture those tests use. No history.
No tests, syntax checks, builds, execution, evidence, Git mutations or extra actors.

TESTS FIRST: only write new test/walletStartup.node.js, new test/walletBrokerBuild.node.js,
and minimal mock compatibility in test/electronSecurity.node.js. Production stays
unchanged/absent until reviewer accepts observed red. Stop and report hashes/counts.
Later production ONLY social-main.js and new scripts/build-wallet-broker.js.

## Main semantics fixed by reviewer

Preserve existing sandbox, IPC validation/clone boundary and normal quit gate.
Keep initial unstarted supervisor available for pre-ready shutdown, replace it with
configured supervisor on ready only while quitState is idle. One startup attempt
per application lifetime; no retry on activation or repeated ready. If quit began
before ready, ready does nothing (no window or broker); activate also gated by idle.
Resolver input exactly resourcesPath, userDataPath=app.getPath('userData'),
platform=process.platform, arch=process.arch. Packaged resourcesPath is strictly
process.resourcesPath; development (app.isPackaged !== true) uses fixed
path.join(__dirname,'wallet-broker','target','app-resources'). No environment binary,
pin or path selection, no runtime hashing/pin generation. Configured supervisor
receives resolver's three fields; use empty environment (supervisor defaults), no
secrets or inherited process.env. Subscribe BEFORE calling start(), after IPC and
window exist. Catch resolver/factory/start failures; social window remains usable,
wallet status stays sanitized {v:1,broker:'down',accounts:[]}. Never raw diagnostics.
Cache sanitized snapshots before checking for a window; subscription sends clones.
wallet:snapshot:get validates sender and payload first; while supervisor.bound is
false return a Promise of a fresh clone of cached sanitized status, initially down.
When bound is true retain real async status.get dispatch and clone/error behavior.
Other channels retain dispatch and fail closed while unbound. start().ok is spawn
attempt only and must never fabricate ready status. Handshake snapshot comes from
supervisor subscription. Retain awaitable shutdown on configured instance. Startup
must be gated again after resolver/factory before spawning if reentry requested quit.
No production supervisor/resolver/protocol/Rust changes.

## Build command fixed by reviewer

New scripts/build-wallet-broker.js exports synchronous
stageWalletBroker(sourceBinary, resourcesRoot, platform, arch). Both paths nonempty
absolute, resourcesRoot existing real non-symlink directory, source regular non-link;
platform/arch supported linux/darwin/win32 x64/arm64. Preflight existing destination
wallet-broker directory and binary/manifest entries: directory real non-link, final
entries absent or real regular non-link (including reject dangling links). All
preflight before writes. Trusted build roots; concurrent same-user races not claimed.
Stage exact copied binary (0755), manifest (0644) {v:1,platform,arch,sha256}; hash
COPIED bytes, manifest last, SHA-256 lowercase. Fixed platform basename matches
resolver. Use fresh mkdtemp directory beneath resourcesRoot, write there, rename
binary then manifest to final directory; preserve unrelated files, retain staging
directory, no recursive deletion. Repeated builds may replace regular files safely;
failure between renames leaves old pin mismatch and runtime fails closed.
CLI accepts no args. It invokes rustup run 1.98.0 cargo build --manifest-path
<repo>/wallet-broker/Cargo.toml --target-dir <repo>/wallet-broker/target --locked
--offline --no-default-features --bin bitbook-wallet-broker with cwd=<repo>, shell=false
and inherited stdio. Nonzero build prevents staging. Repo from __dirname, binary
fixed target/debug basename, resources fixed target/app-resources; create real
resources directory if absent, reject symlink. Use host process.platform/arch.
CLI fixed error 'Unable to build wallet broker', nonzero exit; no secret/raw error.
This composes accepted degraded Rust executable; no dependencies or native account
features change. No installer invocation or modification. Native-platform release
and inherited security blockers stay explicit, without blocking this local fix.

## Meaningful tests

Main suite exercises real registered social-main handlers, mocking Electron and
controlled resolver/supervisor only; restore require hooks/cache on every outcome.
Cover ready-only fixed dev and packaged paths, exact options and one spawn,
subscribe-before-start, cached down before async handshake then live status reply,
resolver/start failure leaving social usable, sender/payload denial on fallback,
cache/clone isolation and updates while window closed, no restart on activation,
pre-ready quit and reentrant quit during resolver (no spawn), configured shutdown
wait. Use existing mock style. Keep tests compact and grouped; no giant hostile
matrix duplicating resolver. Existing electronSecurity mock needs app.getPath,
isPackaged, resolver stub, supervisor start and bound=true so its existing 30 groups
retain assertions and actual async dispatch coverage. No removed/weakened assertions.
Main suite export {tests} and main runner supports selected groups via require,
print ok/not ok plus count; bounded async waits cleared and no unhandled rejection.

Build suite: explicit missing module assertion for red; real owned temporary files
and real resolver, stage bytes vs independent crypto hash, mode/identity, repeat
replacement and unrelated-file preservation; invalid source/destination symlinks
and pin destination links preflight leaves bytes unchanged. CLI via vm with mocked
child_process.spawnSync to assert exact fixed command, nonzero stops before staging;
no Cargo build in unit tests. Fixtures remove ONLY individually tracked/enumerated
owned files/links and directories without following symlinks, no recursive deletion.
Export {tests}, clear per-group results. Avoid implementation-mirroring assertions.

Later Hermes red: node test/walletStartup.node.js and node test/walletBrokerBuild.node.js.
Green: same plus node test/electronSecurity.node.js, node test/walletPreload.node.js,
node test/walletBrokerLaunchConfig.node.js, node test/walletSupervisor.node.js.
Falsify main by suppressing configured start; selected startup assertion must fail,
restore exact main. Falsify build copied-byte hash pin; staging resolver/hash assertion
must fail, restore. Then repeat targeted suites. Hermes additionally runs actual
node scripts/build-wallet-broker.js and a reviewed real-main/real-Rust smoke test
(to be authored after main tests if appropriate; no fake-wallet completion claim).
Security exact commands: npm audit --audit-level=low; node test/securityPolicy.node.js;
node scripts/security-policy.js; target/security-tools/gitleaks-v8.30.1/gitleaks git
--redact=100 --no-banner . and ... dir --redact=100 --no-banner .; zero new findings,
existing six policy groups/inventory failure remain recorded release blockers.

## Baseline

- social-main.js: c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3
- wallet-broker/supervisor.js: c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a
- wallet-broker/launch-config.js: 68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8
- wallet-pay/model.js: acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e
- test/electronSecurity.node.js: 7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c
- wallet-broker/Cargo.toml: 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503
- wallet-broker/Cargo.lock: b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71
- wallet-broker/src/main.rs: 19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d
