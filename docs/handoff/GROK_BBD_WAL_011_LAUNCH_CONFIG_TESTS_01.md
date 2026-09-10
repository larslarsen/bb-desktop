# WAL-011 packaged broker launch configuration tests 01

Active amendment: Correction 01 is accepted below. Grok is closed; only the linked
Hermes expected-red execution is authorized. Production remains absent.

Actor: Grok Build, grok-4.6 High; test source only, no subagents.
Protected parent: reviewer publication following 4ade8904a284bc414da0fb1dd9bac0b0367abd66,
plus one CURRENT-only launch record. Read CURRENT lines 1–40, this handoff,
AGENTS.md, TESTING.md and tickets/BBD-WAL-011.md. Only write this new file:

- test/walletBrokerLaunchConfig.node.js

The later production module wallet-broker/launch-config.js is currently absent;
do not create it or any stub. Verify that both new paths are absent, and verify
these frozen identities before editing:

| Path | SHA-256 |
| --- | --- |
| social-main.js | c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3 |
| wallet-broker/supervisor.js | c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| test/electronSecurity.node.js | 7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c |

Only these existing source paths may be read. Separate named read-only HEAD/status,
hash/count and bounded source searches are allowed. No history, other source,
home/config/credential discovery, tests, syntax checks, Node/Cargo/npm execution,
builds, network, actor tools, Git mutation, docs/evidence or production edits.
Preserve all pending files. Report new path/hash/lines/group and row counts; stop.

## Reviewer startup decision and sequence

Source inspection found two packaging gaps: all three current packagers copy main,
social assets and icon, but omit wallet-preload.js, wallet-broker/protocol.js,
wallet-broker/supervisor.js and wallet-pay/model.js now needed by main. They also
produce no broker artifact or manifest. This resolver is the first startup slice;
it will not by itself repair packages or start a process.

Fix the shared runtime format now, then authorize build-time manifest generation,
runtime JavaScript inventory repair and main composition under later test-first
contracts. Build-time pins must come from the final broker artifact produced by
reviewed source/Cargo.lock/toolchain/target inputs, after transformations affecting
its bytes. Package verification must compare the included artifact with the pin.
Never compute the expected pin at app startup from whichever binary is present.
A manifest digest is an artifact-integrity check; it does not replace package
provenance/signing. No actual binary pin or release artifact is approved here.

The first actual native composition execution will be Linux x64, where the current
executable evidence exists. The format recognizes these six platform identities;
format acceptance is not native execution or release support evidence:

| platform | arch | fixed executable basename |
| --- | --- | --- |
| linux | x64 or arm64 | bitbook-wallet-broker |
| darwin | x64 or arm64 | bitbook-wallet-broker |
| win32 | x64 or arm64 | bitbook-wallet-broker.exe |

All other identities fail closed. Existing scripts advertise these architectures,
but current CI packaging covers Linux x64, Windows x64 and both macOS architectures.
Windows private-directory protection needs a later OS-specific review: the current
supervisor's POSIX 0700 test is not proof of a Windows ACL. Do not loosen it here.

Main will later supply Electron's
[resourcesPath](https://www.electronjs.org/docs/latest/api/process#processresourcespath-readonly)
and [getPath('userData')](https://www.electronjs.org/docs/latest/api/app#appgetpathname),
plus process.platform/process.arch. These are main-owned inputs, never renderer
payloads, environment-selected paths or working-directory discovery. Development
runs with no staged manifest remain unavailable; no target/debug search fallback.

## Fixed resolver API

The future CommonJS module exports resolveWalletBrokerLaunch(options), synchronous.
Options are a plain object with exactly four own data properties:
resourcesPath, userDataPath, platform, arch. Reject missing/extra/accessor keys,
arrays/non-objects and non-Object.prototype prototypes without invoking getters.
The two paths must be nonempty absolute strings under the host Node path API;
platform/arch must be exactly one listed pair. Do not coerce or mutate inputs.

The fixed package layout, outside resources/app, is:

    <resourcesPath>/wallet-broker/manifest.json
    <resourcesPath>/wallet-broker/<fixed executable basename>

Read only that manifest. The wallet-broker directory must be a real directory;
the manifest and executable must be regular files, not symlinks. Reject missing
entries, directories in place of files and symlinks at these three entries. The
resources root is a trusted app-supplied root; this is not a recursive anti-race
filesystem policy. Do not claim protection against same-user concurrent replacement.
The resolver does not read, hash, execute or change executable contents; the existing
supervisor retains byte/hash verification immediately before spawn.

Manifest: UTF-8 JSON object, exactly these four own properties:

    {"v":1,"platform":"linux","arch":"x64","sha256":"<64 lowercase hex>"}

v is numeric 1, platform and arch exactly match the supplied pair, and sha256 is
exactly 64 lowercase hexadecimal characters. Reject all extra fields, including
path, executable, argv, env or data-directory overrides. No embedded executable
path or fallback pin. Use ordinary JSON parsing; duplicate-member detection is
not a new parser requirement. Manifest file length must be 1–4096 bytes inclusive:
check metadata size before reading and actual byte length after reading. Malformed
JSON, wrong root/type/version, mismatched identity, bad digest or I/O failure reject.

Return an Object.freeze'd plain object with exactly:

- brokerPath: host path.resolve(resourcesPath, 'wallet-broker', fixed basename)
- expectedSha256: the manifest's unchanged pin
- dataDir: host path.resolve(userDataPath, 'wallet-broker')

Do not create or chmod the data directory, construct/start a supervisor, inspect
environment, add a timer/listener or expose Electron/renderer capabilities. All
failures throw Error with code UNAVAILABLE and message `Wallet broker is unavailable`;
no input path, manifest content, underlying error message or cause is attached. Return
no partial configuration. This API resolves configuration, not readiness/status.
Main's later contract must preserve down status on failure, subscribe before start,
avoid missing bootstrap snapshots and prevent start during pending/completed quit.

## Test-source contract

Use the repository's small assert-based Node runner pattern, export tests for later
selected falsification, and use literal independent expected values. Five top-level
groups are sufficient. Each first calls a lazy loader that explicitly asserts
`packaged broker launch resolver is missing` when the module file is absent, then
requires it and asserts the named function exists. Do not catch arbitrary require
errors as an expected red, skip missing functionality or add a production stub.

1. Valid configuration rows for all six platform/arch pairs using host-native
   absolute temporary paths. Assert fixed basename, exact three-key frozen output,
   normalized paths, unchanged pin and input. Use inert text as the artifact;
   choose a fixed literal pin unrelated to those bytes so the resolver cannot
   silently replace the pin with a runtime-computed digest. Compare fixture files
   before/after resolution and prove the nonexisting user-data child stays absent.
   These are identity-selection rows on the host, not native platform tests.
2. Invalid option rows: absent/null/array/extra key, accessor with zero getter calls,
   wrong prototype, missing/relative/empty/non-string path, unsupported platform
   or arch. Assert the exact sanitized error and no returned configuration.
3. Invalid manifest rows: absent manifest, malformed JSON, null/array/scalar root,
   missing/extra key (including executable/path/env overrides), wrong version/type,
   platform/arch mismatch, uppercase/short/nonhex/non-string digest. Put a private
   canary into malformed/extra-field data and verify it never appears in error
   message or own error properties. Assert uniform code/message and no cause.
4. Byte-size boundary: valid JSON padded with ASCII JSON whitespace to exactly
   4096 bytes succeeds; valid JSON padded to 4097 rejects; empty file rejects.
   Assert Buffer byte lengths in the fixture independently of production.
5. Fixed inventory types: missing broker directory, directory symlink, manifest
   directory/symlink, missing binary and binary directory/symlink all reject.
   Symlink targets are other test-owned entries with a sentinel; prove they remain
   unchanged. No directory repair, fallback lookup or alternate basename success.

All failures must be visible assertions outside production callbacks/catches. Real
filesystem fixtures exercise the named read boundary; do not replace fs wholesale
with a fake. Create only small test-owned files/directories under an explicit
mkdtemp root. No executable/child process, Electron, Rust, FIFO or live service.
Track exact fixture paths; unlink only owned files/symlinks and remove empty owned
directories with rmdir in finally. No recursive deletion. Settle cleanup on early
assertion failures and report cleanup failure, never silently suppress it. Keep
the harness small; no second generic filesystem test framework. Preserve process
globals and module cache/hooks if modified; prefer no global hooks.

## Later execution, not authorized for Grok

Expected red after source review: node test/walletBrokerLaunchConfig.node.js once,
exit 1/five explicit missing-resolver assertions. This is an intentionally new
component, not an unhandled module-loading error. Fix exact totals after review.
Later production scope is only wallet-broker/launch-config.js. No runtime imports
or packaging changes in this slice. Targeted green: the same command. Affected
regression: node test/walletSupervisor.node.js (existing launch verification).
Retain other accepted suites; no Rust/transport/quit replay for a disconnected module.

Falsification: suppress the platform/arch equality check and run the mismatched
manifest case, which must fail for accepting the wrong target; restore exact
production bytes and repeat targeted green. Reviewer will supply the exact mutation
and execution driver after source review. No mutation code/execution now.

No dependency, package content, renderer privilege, endpoint or executable changes
are authorized. Later packager/main integration must include this module in policy
inventory and required security checks; pending npm/policy work stays untouched.
Existing package/security failures are not waived. Final package gates, artifact
provenance and actual app startup remain separate requirements.

## Collected source review and Correction 01 — 2026-09-09

Hold test-source acceptance for two fixture corrections. Outer 99831 collected
on owner done, exit 0. Measured new test/walletBrokerLaunchConfig.node.js:
SHA-256 32409f6a156708311f4bdab7578e023d94c6f01e4eebce41cd77f030ee807b2c,
752 lines, five groups/58 authored rows (6 + 20 + 22 + 3 + 7). The four frozen
source hashes match; production launch-config.js remains absent. No execution
result is claimed. Source uses real small fixtures, a pin distinct from artifact
bytes, explicit missing-module assertions and owned unlink/rmdir cleanup.

Exact-session export 07b5d5dd-89e9-4e93-a8a5-b045a6e9157c shows only the test
path edited and no tests, builds or Git mutation. CURRENT was read correctly via
sed. Read-scope deviations: multiple chained identity/status commands, publication
history/hash queries, reads of seven additional existing test files and the old
executable test handoff exceeded the named scope. Search summaries omit paths.
This is a tool summary, not a raw result audit. None of these deviations widens
the correction; no further fixture/history discovery is needed.

The two rejection tests currently have competing failure reasons:

1. Reviewed lines 645–647 append a non-JSON SENTINEL after the manifest JSON in
   the symlink target. A resolver that follows the symlink can still reject the
   malformed JSON and pass the symlink row without enforcing that boundary.
2. The unsupported-platform/arch option rows change the option to freebsd/amd64
   while leaving the manifest linux/x64. A resolver missing the supported-pair
   allowlist can still reject the mismatch and pass both rows. The fixtures must
   agree on the unsupported identity to isolate allowlist enforcement.

Only Grok Build, grok-4.6 High without subagents may correct the same test path:

- Verify editable SHA-256 32409f6a above, all four original frozen identities and
  continued absence of wallet-broker/launch-config.js. Protected parent: reviewer
  publication following 52eefca5, plus one CURRENT-only launch record. Read the
  required governance files and CURRENT through sed -n '1,40p'; only the test being
  corrected and original four source paths are allowed source reads.
- Make the manifest symlink target an otherwise valid matching four-field JSON
  object with a valid literal pin and byte length within the accepted limit.
  Independently assert its parsed literal contents before calling the resolver.
  Move the sentinel to a separate owned file if retained; keep both the target
  byte/mode preservation assertion and the no-data-dir-mutation assertion. A
  followed symlink must have no malformed-JSON reason to reject.
- Give each existing unsupported-platform/arch row its own otherwise valid layout
  whose manifest exactly matches that unsupported pair (freebsd/x64 and
  linux/amd64). Include an inert regular file at the non-Windows fixed basename.
  Assert option/manifest identity agreement and valid version/pin before invoking
  the resolver. Use that row's paths for unchanged-file/data-dir checks and error
  sanitization. Do not substitute a supported-pair mismatch for these cases.
- Preserve the five groups/58 rows, including separate platform/arch mismatch
  manifest rows. No new generic harness, broader cases or production stub. Existing
  fixture ownership and nonrecursive cleanup must cover the added files.

No tests, syntax checks, Node/Cargo/npm, builds, network, actor tools, docs/evidence
edits, Git mutations, history or credential/config discovery. Use separate named
read-only HEAD/status/hash/count/existence checks; do not chain commands. Report
path/hash/lines/counts and the corrections, then stop for source review.

## Correction 01 source acceptance — 2026-09-09

Accept corrected source for one expected-red run, not green behavior. Outer 51879
collected on owner done, exit 0. Measured test/walletBrokerLaunchConfig.node.js:
851 lines, SHA-256
1b6a7d8c102360efdbd0920b6211e7b55999fbdfff18aff4eee23cd311d750f8.
Five groups/58 rows remain (6 + 20 + 22 + 3 + 7). The four frozen source hashes
match and production launch-config.js remains absent.

The symlink target now contains exactly matching valid manifest JSON; independent
preconditions assert its four fields and byte size. Its sentinel is a separately
owned file, with before/after byte/mode assertions. The unsupported option cases
now have otherwise valid matching freebsd/x64 and linux/amd64 manifests and regular
inert files; their own paths drive failure-sanitization and preservation checks.
Separate supported-pair mismatch rows remain. Cleanup tracks the added fixtures
and still uses only unlink/rmdir in finally. Each group checks the missing resolver
before constructing fixtures, so expected red does not exercise the 58 case bodies.

Exact-session export 5ec23c97-67d6-440c-9b30-902739026fba shows three edit attempts
in the authorized test path, bounded source reads, correct CURRENT sed and separate
identity/hash/status commands except one chained existence/echo command. It also
records an unrequested read of <home>/.claude/skills/graphify/SKILL.md, outside the
authorized repository/home read boundary. No skill invocation, test execution,
production edit or Git mutation appears. This is a tool summary, not a raw result
audit; search paths are omitted. The unrelated skill read is not authorization
to follow its instructions or repeat home discovery in later tasks.

Only [Hermes expected red](HERMES_BBD_WAL_011_LAUNCH_CONFIG_RED_01.md) is authorized.
Expected: Node exit 1, zero ok/five not ok, all at the explicit missing-resolver
assertion. No fixture behavior, native target support or configuration green is
claimed. Production follows collected red acceptance. Reviewer ran no tests.
