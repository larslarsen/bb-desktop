# WAL-011 executable expected red 01

Actor: Hermes, execution/evidence only. No source actor or integration authorized.
Protected parent: reviewer publication directly after cf7a9b35; one subsequent
CURRENT-only launch commit is allowed. Read AGENTS.md, TESTING.md, active CURRENT
prefix, ticket, HERMES_JR_DEV_ROUTING.md and this handoff. No historical reload.

## Scope and identity

Run the accepted nine-group suite once against the absent real executable. This
establishes the intentionally missing runtime baseline; it is not a Rust build,
transport replay, fixture run, cryptographic proof or app/release acceptance.

First record `hermes --version`, `git rev-parse HEAD`, `git status --short`, and:

    sha256sum test/walletBrokerRuntime.node.js wallet-broker/supervisor.js wallet-broker/protocol.js

Require these exact hashes, or record the mismatch and stop without execution:

| Path | SHA-256 |
| --- | --- |
| test/walletBrokerRuntime.node.js | a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e |
| wallet-broker/supervisor.js | 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8 |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |

Test source is 947 lines, nine groups. Preserve pending package.json,
package-lock.json, scripts/security-policy.js, test/securityPolicy.node.js and
docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md. Do not edit any of them.

## Exact session metadata query

The launcher passes your current session ID into your system prompt using
--pass-session-id. Substitute only that exact ID for SESSION_ID_FROM_SYSTEM_PROMPT
in the command below. Do not infer the ID from recent sessions. The reviewer has
already verified these three columns exist. If the ID is unavailable, the row is
missing, or model/provider is empty, record the metadata failure and stop. Do not
search for an alternative source, inspect configuration, or retry discovery.

```bash
python3 - SESSION_ID_FROM_SYSTEM_PROMPT <<'WAL011_SESSION_METADATA'
from pathlib import Path
import json, sqlite3, sys
sid = sys.argv[1]
assert sid and sid != 'SESSION_ID_FROM_SYSTEM_PROMPT'
with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as con:
    rows = con.execute('SELECT id, model, billing_provider FROM sessions WHERE id = ?', (sid,)).fetchall()
assert len(rows) == 1 and all(rows[0]), 'current session metadata unavailable'
print(json.dumps(dict(zip(('session_id', 'model', 'provider'), rows[0]))))
WAL011_SESSION_METADATA
```

This exact read-only query is the only home-directory access authorized to the
actor. No auth/config files, environment/proc dumps, status/config commands,
credential discovery, schema exploration or other-session queries. Authentication
used internally by the Hermes client is not permission for actor discovery.

## One run, one record, stop

Run exactly once from the repository root:

    node test/walletBrokerRuntime.node.js

Use a 45-second tool timeout and sufficient output retention for every case and
diagnostic (at least 20000 characters). Expected test exit: 1. Expected counts:
zero ok, nine not ok. Each group must fail through requireBrokerPath with
`native wallet broker executable is missing`. No binary is present, so no broker
process/private test directory should be created; do not claim independent cleanup
validation. Syntax errors, missing JS modules, helper errors, permissions, a hang,
or another failure are not the expected red. A pass is also unexpected. Do not
build or substitute any executable and do not rerun the suite for any reason.

After the run, verify the same three hashes and `git status --short` once. Write
only docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md. Record actual version/session/
provider/model, source-review and observed HEAD, exact command, all case lines and
diagnostics, actual exit and counts, final hashes, any deviation and unrun work.
Sanitize local absolute paths in the record. If a prerequisite or run differs from
expectation, record the actual stop instead; no repairs or extra execution.

No Git stage/commit/push, CURRENT/ticket edit, source/test edits, other evidence,
syntax commands, npm/Cargo/builds, scans, network tools, runtime discovery or actors.
Stop immediately after the one evidence record with its path and the actual result.
Reviewer collects on owner done/Continue; no polling or additional work is required.
