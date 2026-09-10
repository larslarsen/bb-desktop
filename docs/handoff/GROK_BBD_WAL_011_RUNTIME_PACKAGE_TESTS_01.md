# WAL-011 packaged JavaScript runtime inventory tests 01

Actor: Grok Build, grok-4.6 High, test source only; no subagents.
Parent: reviewer publication following dd88cfc2 plus CURRENT-only launch record.
Read AGENTS.md, TESTING.md, ticket, this handoff, CURRENT active prefix only.
Only other reads: the files pinned below and test/walletBrokerLaunchConfig.node.js
for owned-temp cleanup and runner conventions. No history or home skills/config.
Only write NEW test/walletRuntimePackage.node.js; confirm it is absent first.
Production scripts/stage-wallet-runtime.js must remain absent. No stubs.

## Fixed packaging contract

Current Linux/macOS/Windows packagers omit wallet-preload and the local JavaScript
modules imported by main/supervisor. Fix this concrete inventory before adding
native artifacts. This slice stages JavaScript only; no native pin, Cargo build,
startup wiring, package-policy expansion or release completion. The current main
still creates an unstarted supervisor. Existing dependency/security findings stand.

Later production paths ONLY: new scripts/stage-wallet-runtime.js and the three
existing build scripts. CommonJS export stageWalletRuntime(sourceRoot, appRoot),
synchronous, only fs/path imports. CLI takes exactly one absolute appRoot argument;
sourceRoot is path.resolve(__dirname, '..'). No environment-selected source or
subprocess, dependency, glob or discovery mechanism. Throw on errors; CLI prints
fixed 'Unable to stage wallet runtime' to stderr and exits 1, success exits 0.

Exact relative inventory (copy bytes, mode 0644; no other files):
- wallet-preload.js
- wallet-broker/protocol.js
- wallet-broker/supervisor.js
- wallet-broker/launch-config.js
- wallet-pay/model.js

API arguments must be nonempty absolute strings. Both roots must already be real
non-symlink directories. Preflight ALL sources and destinations before writing:
source parents wallet-broker and wallet-pay must be real directories; each source
must be a regular non-symlink file. Destination parents, if present, must be real
non-symlink directories. Every final destination must be absent, including dangling
symlinks. Failure during preflight leaves the destination exactly unchanged.
After preflight create only missing two parent directories, copy each file with
COPYFILE_EXCL and set copied mode 0644. Never overwrite existing entries. Actual
I/O failure during copying propagates and prevents packaging; rollback is not
required. Do not delete anything. AppRoot may already contain package.json,
social, imgs and social-main; preserve these. Build roots are trusted, so ancestor
race attacks and atomic multi-file commit are outside this bounded contract.
Return value has no public contract. Inputs/source bytes remain unchanged.

All three packagers call the same helper once after establishing APP_SOURCE or
AppSource and before archive/signing. Exact calls: Bash scripts use
node scripts/stage-wallet-runtime.js "$APP_SOURCE"; PowerShell uses
& node scripts/stage-wallet-runtime.js $AppSource followed immediately by a
LASTEXITCODE check that throws on nonzero, before Compress-Archive. Existing
runtime-package template/main/social/icon copying remains unchanged.
Linux staging additionally moves from TMPDIR/tmp to a fresh mktemp directory
under existing PROJECT_ROOT/dist: create dist first, then
BUILD_ROOT="$(mktemp -d "$PROJECT_ROOT/dist/bitbook-deb.XXXXXX")".
Remove its variable-based recursive-deletion EXIT trap. Retain this staging tree
for inspection and print its path. Do not add recursive cleanup. This enables the
later actual Linux package proof on verified disk-backed storage. Mac/Windows
native execution and their inherited cleanup need separate platform review.

## Exactly seven registered test groups

Use existing synchronous test-array export {tests} and a main-only runner, with
per-group ok/not ok lines and final 'BitBook wallet runtime package tests passed (7).'
Each group first asserts the new helper exists with exact diagnostic
'wallet runtime staging helper is missing', then requires its exported function.
No production-source pattern checks except explicitly named build-script wiring.

1. 'staging: exact five-file inventory preserves bytes and excludes native data'
   Use tiny owned source fixtures with five distinct contents, extra Rust/target/
   native-binary/sentinel decoys, and existing unrelated app entries. Compare exact
   staged inventory/bytes (no extras), source bytes and untouched app sentinels.
   On POSIX assert copied file modes 0644 even with executable source fixtures.
2. 'preflight: missing and hostile source entries leave destination unchanged'
   Independently cover each of five missing sources, one source as directory,
   file symlink, and each of the two source-parent directory symlinks. Other sources
   must be valid; use late-inventory defects too. Verify unchanged destination and
   real symlink-target sentinel bytes. No recursive-copy fallback is allowed.
3. 'preflight: existing and hostile destinations are never overwritten'
   Cover each of five existing final files, dangling final symlink, source/root
   symlink and appRoot symlink, and each destination-parent symlink or regular file.
   Verify all existing contents, symlinks, external sentinels and source fixtures
   unchanged; no new entries on rejected preflight.
4. 'arguments: invalid roots fail before staging'
   Missing/null/non-string/empty/relative arguments for both roots, missing root and
   root-as-file cases. Independent fixtures or snapshots as needed; no writes.
5. 'cli: real checkout stages modules whose nested imports resolve'
   Spawn process.execPath on actual helper with owned temp absolute appRoot, require
   exit 0, compare each staged byte against actual checkout. Spawn a separate Node
   child to require staged supervisor/protocol/launch-config/model; assert exported
   expected functions, without starting a supervisor or loading Electron main.
   Both children synchronous bounded timeout 15 seconds; no shells or network.
6. 'cli: missing and extra arguments fail without staging'
   Spawn actual CLI with no args and with appRoot plus extra arg; require exit 1,
   fixed error diagnostic, no destination writes. Timeout 15 seconds, no shell.
7. 'packagers: shared staging precedes signing and archive with Linux disk storage'
   Read only the three real build scripts. Assert exact helper call once in each,
   after app destination definition and before dpkg-deb/codesign/Compress-Archive;
   assert PowerShell failure guard immediately after its call. Check Linux staging
   uses PROJECT_ROOT/dist and has no recursive EXIT cleanup. These static checks
   enforce script wiring only; groups 1–6 exercise the actual helper, and a later
   Hermes real Linux package/extraction check supplies the package boundary proof.

Tests may create only small os.tmpdir-owned files/dirs/symlinks, track them and
unlink files/symlinks then rmdir directories in finally, without recursive removal.
Do not execute large staging/native builds from unit tests, mutate production,
read user data, monkey-patch globals or introduce test-only production switches.
Expected red: seven exact helper-missing assertions, no fixture bodies executed.
Green: same suite plus nine resolver groups; falsification removes preload from
inventory so selected exact-inventory group fails, restore and repeat full suite.
Hermes alone owns execution/evidence and later package proof/integration.

## Frozen baseline

| Path | SHA-256 |
| --- | --- |
| scripts/build-deb.sh | 3ea4d0eba122eedcf17e8044e25dcb33dba998609c6213c1138233da3535b396 |
| scripts/build-macos.sh | 74ea1c30684722dff2446bc6818e7207acc384e7ff7c56e4311a4e9ad47fb285 |
| scripts/build-windows.ps1 | 49619fae88bc01a3f16728e3f595cd70ff8bee35d43971c5a9b43b8ccae4ac28 |
| wallet-preload.js | 3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| wallet-broker/supervisor.js | c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a |
| wallet-broker/launch-config.js | 68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8 |
| wallet-pay/model.js | acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e |
| social-main.js | c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3 |
| packaging/runtime-package.json.in | 1660c63cf7d36abdfacf47333a69496c3a03121819100cb64a55e2c11a658d9e |

Verify all hashes before editing. Separate readonly HEAD/status/hash/count/existence
checks and named reads allowed. No tests, syntax checks, Node/npm/Cargo execution,
builds, evidence/docs edits, Git mutation, extra actors or unrelated discovery.
Report new test hash/line count, group/case counts and stop. Reviewer stays with
actor, reviews and routes directly; owner done messages are not required.

## Source review and Correction 01

Outer 1422 completed exit 0; session c2c18a7d-5fe2-41e4-ba81-09db6360dc31.
Drop 67c371b1e70f5282a6c569fdfbb7a2b5106ce146e15188be1c9a263a62f30790,
786 lines, seven groups/44 cases. Helper absent; no execution. Hold acceptance
for exactly these bounded corrections in the SAME test path only:

1. Group 7 indexOnce(deb, 'dpkg-deb') wrongly counts both the preserved explanatory
   comment and actual build command; indexOnce(mac, 'codesign') wrongly rejects
   the required signing PLUS verification commands. Match the complete existing
   command lines instead: dpkg-deb --build --root-owner-group "$PACKAGE_ROOT" "$OUTPUT"
   and codesign --force --deep --sign - "$APP_BUNDLE". Preserve verification and
   comments. No production edits or test accommodations by deleting packager steps.
2. Relative-root argument cases currently point to nonexistent cwd paths, so a
   helper accepting relative paths can fail for missing roots instead. Set each
   relative argument using path.relative(process.cwd(), the corresponding valid
   owned source/app root). Assert it is nonempty and not absolute and resolves
   back to the valid root before invocation. Remove the now-invalid assertions
   that these relative paths resolve to absent directories. Existing source/app
   snapshots must prove unchanged contents. Do not change cwd or write outside
   the owned temporary root. Keep 16 argument cases and all seven groups.
3. trackIfPresent currently lstats inventory children through parent symlinks and
   schedules deletion through aliases after links have been unlinked; it also
   misses unexpected copied files. Replace its inventory-only scan with symlink-
   safe enumeration of the actual owned appRoot tree: lstat each entry, register
   symlinks but never traverse them, register files, register real directories in
   preorder and enumerate only those. No outside-root or arbitrary path discovery.
   All fixture symlink targets are inside owned fixture trees, so enumerating their
   real directory entries tracks unexpected writes by their canonical test paths.
   Keep individual unlink/rmdir finally cleanup, no recursive deletion API.

No helper/packager changes, execution, Git mutations or docs/evidence edits. Read
only this correction section, the test, three packagers, AGENTS/TESTING/ticket and
CURRENT via sed -n '1,18p'. Original ten frozen hashes remain required. Report
corrected test hash/line count then stop; reviewer collects immediately. Initial
export shows named reads/checks and only the test edits; CURRENT was read at
2–31 rather than requested prefix. Use the exact sed command now. No home/config
discovery, acceptance execution, chained commands or Git mutation appeared.

## Correction 01 accepted

Accept 5d8a8bf250a4e664fb15853cfb29deee62130efd72cdcf3acdac1129c2324f25,
794 lines, seven groups/44 cases. Full-command anchors preserve signing verification
and comments; relative arguments resolve to valid owned roots; cleanup enumerates
only real directories and never traverses links. All ten frozen hashes match and
helper remains absent. Outer 63612 completed exit 0; session
bbf66bd6-98c3-4dec-9120-d5a671e7ea0d closed. Export contains only test edits and
read-only checks; one chained existence check and git log -1 for the HEAD hash
exceeded the command restrictions. No acceptance execution, source/Git mutation
or unrelated discovery appears. Report's no-Git claim means no mutations.
Only Hermes red/policy-baseline handoff is authorized. Reviewer ran no tests.
