# WAL-009 source-policy expected red and npm synchronization 01

Actor: Jr Dev — Hermes. Reviewer: Codex; High is sufficient.
Protected parent: publishing reviewer commit directly following a2c7ee0e, with one
subsequent CURRENT-only launch checkpoint allowed. No source actor is active.
Source acceptance: GROK_BBD_WAL_009_SOURCE_POLICY_TESTS_01.md, collected appendix.

## Scope and preflight

Synchronize the already reviewed MapLibre-only graph, collect the remaining source-
policy red, and run package security checks in one pass. Preserve every accepted
Rust result. Read AGENTS.md, TESTING.md, Hermes routing, this task and active CURRENT/
ticket prefixes only; no historical evidence reload or directory/config discovery.
Run hermes --version, git status --short and git rev-parse HEAD once. One
 git show --stat --oneline HEAD is allowed if needed to identify the launch.

Require empty index, exactly three modified paths (package.json,
scripts/security-policy.js, test/securityPolicy.node.js) and one untracked old red
report. The new docs/testing/BBD-WAL-009-SOURCE-POLICY-RED-01.md must be absent.
Verify these frozen inputs in one read-only batch:

| Path | SHA-256 |
| --- | --- |
| package.json | 76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5 |
| scripts/security-policy.js | fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078 |
| test/securityPolicy.node.js | a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c |
| package-lock.json | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md | 1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f |
| wallet-broker/Cargo.toml | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |

Only package-lock.json through the named npm operation, npm's local node_modules
maintenance, and the new evidence path are writable. No production/test source,
package.json, Rust, old report, CURRENT, ticket or handoff changes. No Git mutation,
integration, new actor, CI polling, tool download, retry, formatter, proof or other
Rust gate. Keep existing disk-backed caches/target; no temporary artifacts/scripts.

## Five commands, once each, sequentially

Use explicit repo-root workdir and each exact standalone command without extra cd,
echo, wrappers, redirection or chained commands. Capture actual exits and command/
result IDs. Retrieve complete logs, paging through all lines (the previous 200-of-261
capture was incomplete). For background commands wait only on that process with
waits at most 60 seconds. No inference of execution from a todo or another command.

1. Offline dependency synchronization; require exit 0:

```text
npm install --ignore-scripts --offline --no-audit --no-fund
```

The installed baseline is MapLibre 6.8.0, Leaflet 1.9.4 and Electron 44.0.0; the v3
lock has 38 package entries. Verify after the npm operation, using read-only JSON
comparison against git show HEAD:package-lock.json, that the only semantic changes
are root dependencies becoming {"maplibre-gl":"6.8.0"} and removal of the entire
packages["node_modules/leaflet"] entry. Exactly 37 entries remain; every other
nested lock value, integrity, resolution and version must be unchanged. Whitespace-
only differences may be recorded. Verify package.json is unchanged, installed
MapLibre 6.8.0/Electron 44.0.0 remain, and node_modules/leaflet is absent. No lifecycle
scripts, package download, manual lock edit or unrelated graph update. Any setup/
extra graph drift is a stop without retry or repair.

2. Policy suite expected red; expected exit 1, 89 pass / 3 fail out of 92:

```text
node test/securityPolicy.node.js
```

The expected failing groups are the updated fourteen-path WAL-008/WAL-009 inventory
(old production export still has eight paths), and the two new exact extractor
positive groups (legacy .extract() rejection). The earlier package, Orchard and
signing-target groups must now pass after npm synchronization. Do not claim negative
cases after a failing positive were executed. Unexpected failures are findings to
record, not permission to edit or retry.

3. Production CLI expected red; expected exit 1 with the old WAL-008 unlisted-extra-
path rejection:

```text
node scripts/security-policy.js
```

4. Electron boundary after dependency synchronization; expect exit 0 and 20 passes:

```text
node test/electronSecurity.node.js
```

5. Dependency advisory query, all locked npm dependencies; expect exit 0 and
metadata.vulnerabilities.total = 0:

```text
npm audit --json
```

Advisory network access is authorized for this query only. No audit fix, package
upgrade, findings suppression or new baseline. Network/setup failure or findings
are recorded without retry; do not claim a clean audit from an unavailable response.
The prior reviewer-role accidental audit failed DNS and supplies no audit evidence.

After successful npm synchronization, collect all four remaining outcomes even if
ordinary policy/security findings are nonzero. A tool invocation/setup failure
stops execution. No claim of complete green or integration at this expected-red
checkpoint. Future source-policy changes do not change the npm graph, so retain
successful package audit/Electron evidence for later review rather than replaying it.

## One concise evidence record, then stop

Recheck source/manifest/old-report hashes and git status once. Require exactly the
original three modified files plus the expected modified package-lock.json; old
report stays untracked and unchanged. Record actual new lock hash/lines and exact
structural/installed-package results. Create only
 docs/testing/BBD-WAL-009-SOURCE-POLICY-RED-01.md after execution. Include observed
version/HEAD, actual runtime identity if available (otherwise explicitly unavailable),
all five command/result/log IDs, actual exits/counts and exact failing names/reasons,
complete-output limits, source identities and no-integration statement. Never copy
old runtime IDs or fabricate unavailable metadata. Mark unrun commands UNRUN.
Normalize literal repo/home prefixes to <repo>/<home> before writing. No future
success/commit claim. Measure evidence lines/bytes/SHA-256 as the final tool operation,
then STOP: no extra todo, Git, report revision, gate, actor or new task. No report-only
repair. Reviewer collects once on owner done/Continue; no automatic relaunch.
