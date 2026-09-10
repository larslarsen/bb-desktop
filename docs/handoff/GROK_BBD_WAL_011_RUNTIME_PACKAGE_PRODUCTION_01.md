# WAL-011 runtime package production 01

Actor: Grok Build, grok-4.6 High, source only; no subagents.
Parent: reviewer publication following 93fef8a4 plus CURRENT-only launch record.
Read AGENTS.md, TESTING.md, ticket, CURRENT via sed -n '1,18p', this handoff and
fixed packaging contract/table in GROK_BBD_WAL_011_RUNTIME_PACKAGE_TESTS_01.md.
Other named reads: accepted test/walletRuntimePackage.node.js, the three packagers,
packaging/runtime-package.json.in and the red acceptance section. Hash-only reads
of the ten frozen table inputs and red evidence are permitted. No other discovery.

Only production paths:
- NEW scripts/stage-wallet-runtime.js (must be absent before edits).
- scripts/build-deb.sh
- scripts/build-macos.sh
- scripts/build-windows.ps1

Verify all ten original table hashes in one sha256sum command before editing.
Test hash 5d8a8bf250a4e664fb15853cfb29deee62130efd72cdcf3acdac1129c2324f25,
794 lines, seven groups/44 cases; evidence SHA-256
 d6c1d5f642b23fa31c13ae933bb661359b123932c6d0541f53b66a9d6b6fa791.
Tests and evidence stay frozen. Expected red is accepted: seven missing-helper
assertions. Two known policy failures are baselined and not waived or editable.

Implement exactly the fixed contract from the test handoff: synchronous exported
stageWalletRuntime(sourceRoot, appRoot), only fs/path imports, validated absolute
existing nonsymlink directory roots, real source parents and five regular source
files, real or absent destination parents and absent final destinations including
rejecting dangling links. Complete ALL preflight before mkdir/copy/chmod. Then
create only absent parents, copy fixed five files exclusively with COPYFILE_EXCL,
chmod copies 0644. No delete, overwrite, recursive copy, native binary, source
mutation or arbitrary inventory. Errors propagate. CLI guard exactly one argument,
sourceRoot from script-relative repository, fixed diagnostic and exit 1 on failure.
Build-time tooling only; no runtime import of this helper or other new behavior.

Add exact shared helper call once after existing app icon copy in each packager,
before signing/archive. Bash: node scripts/stage-wallet-runtime.js "$APP_SOURCE".
PowerShell: & node scripts/stage-wallet-runtime.js $AppSource immediately followed
by if ($LASTEXITCODE -ne 0) { throw 'Staging wallet runtime failed.' }.
Keep existing template/main/social/icon/license/sandbox/sign/archive logic intact.
Linux only: MOVE its existing install -d "$PROJECT_ROOT/dist" before mktemp (one
occurrence), change BUILD_ROOT to the exact project/dist mktemp expression in the
contract, remove recursive EXIT trap, and add echo "Staging retained at $BUILD_ROOT".
Retain staging rather than introducing any cleanup mechanism. Do not change
Mac/Windows inherited cleanup or any other packaging behavior in this slice.

No tests, syntax checks, Node/npm/Cargo/build execution, docs/evidence edits, Git
mutation, home/config/history discovery, chained commands, network or additional
actors. Separate read-only HEAD/status/hash/count/existence and named reads only.
Report each changed path/hash/line count and stop. Reviewer collects directly.

Later Hermes: seven focused groups, nine resolver groups, 30 Electron security
checks, Bash syntax for changed shell scripts; omit preload from inventory to
falsify exact-inventory test, restore then repeat seven. Compare policy baseline,
run npm audit and pinned Gitleaks; local real Linux package/extraction proof must
verify copied bytes and nested imports plus retained chrome-sandbox ownership/mode.
Preserve the existing .deb under an exact no-overwrite backup path first. Linux
dist is verified ext4 with sufficient space; existing Electron 44.0.0 is installed.
No native wallet startup, Cargo build, macOS/Windows native execution, release
publication or policy repair is part of this source contract.

## Source acceptance — 2026-09-10

Outer 55564 completed exit 0, session dd016621-0865-4810-b1ae-13b2176aa9e3.
Accept four paths:
- scripts/stage-wallet-runtime.js: 97d04b4434e498bf4a2dd70d1ec69881221d8466664656e87fbffdd1c0694a70, 113 lines.
- scripts/build-deb.sh: 6ccaeed5fe31c487ed15d1d6424de1d9f2ef82f3b9ef79f90e4c4329e5142aa8, 75 lines.
- scripts/build-macos.sh: 29e6ede69b2db49e780599af5a6a51cc4d263d3083683e6ea13c24321811f171, 75 lines.
- scripts/build-windows.ps1: 8ce8faaff9e9a772540e63165122be9e30f78186f035570bbc63f6dd366d16c1, 47 lines.

All source/destination checks precede writes. The fixed five-file inventory uses
exclusive copies and 0644 modes; CLI rejects arity/errors. Diffs add one helper
call per packager, guard PowerShell failure, move Linux staging to retained dist
and remove its recursive trap. Existing archive/signing/sandbox logic stays intact.
All other red-baseline inputs match, including test 5d8a8bf2 and pending files.
Exact export shows only four source-path edits and read-only checks. It read
lines 2–81 of the named red handoff rather than the acceptance section; no execution,
Git mutation, chained command or home/config discovery appears. Grok closed.
Only the green/package-proof driver is authorized; reviewer AST-parsed it only.
