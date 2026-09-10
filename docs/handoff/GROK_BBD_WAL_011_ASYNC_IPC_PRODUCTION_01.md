# WAL-011 async Electron IPC production 01

Actor: Grok Build, grok-4.6 High; source only, no subagents.
Protected parent: reviewer publication following 34c297b0; one CURRENT-only launch
commit may follow. Read CURRENT lines 1–45 only, this handoff, AGENTS.md and
TESTING.md. Additional reads are confined to social-main.js and the three test
groups at test/electronSecurity.node.js lines 771–1024. No history or other searches.

The expected-red execution is accepted: Node exit 1, 21 passing groups and the two
delayed groups failing at the missing-thenable assertion. Do not rerun anything.

Verify these identities with a named sha256sum read before editing:

| Path | SHA-256 |
| --- | --- |
| social-main.js | b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df |
| test/electronSecurity.node.js | df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3 |

Only writable path: social-main.js. Inside walletHandler replace exactly this one
line, preserving every other byte and the 174-line count:

```javascript
    return cloneBoundary(result);
```

with:

```javascript
    return Promise.resolve(result).then(cloneBoundary);
```

Do not make the enclosing handler async. Sender/payload validation and dispatch
remain synchronous, so their existing throw behavior is preserved. Promise
resolution now precedes cloning, and rejection propagates unchanged in-process.
No catch, success fallback, retry, schema change, new IPC channel, native authority,
binary pinning, startup/shutdown wiring, refactor or formatting is authorized.
This change handles the existing dispatch contract only; it does not start a
broker or make native wallet operations available.

Allowed commands: separate read-only HEAD/status, named source hashes and line
counts, and bounded reads/searches of the named source. No chained shell commands,
broader repository searches, config/home/credential discovery, history/logs,
network, actors, Node/syntax/test/build commands, docs/evidence edits, Git mutation
or integration. Preserve all pending test, npm/policy and evidence files.

Report the exact replacement, resulting SHA-256 and line count; stop. Reviewer
will independently verify all other bytes are unchanged. Hermes validation follows
source acceptance: node test/electronSecurity.node.js (all 23 groups),
node test/walletPreload.node.js, then exact one-line reversal falsification and
restored targeted green, with no accepted transport/Rust proof replay. A future
Hermes handoff will own the exact commands, restoration and evidence; none may be
executed by this actor.


## Collected source acceptance — 2026-09-09

Accept the production drop for focused validation. Outer 59353 collected on owner
done with exit 0. social-main.js has SHA-256
2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4,
174 lines. Reviewer reversed the one literal replacement in memory and recovered
the exact baseline b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df.
Every other byte is unchanged. The accepted test, supervisor and protocol retain
their frozen hashes. The Promise chain preserves synchronous validation/dispatch
and applies cloning only to fulfillment, with no catch or new authority.

Exact-ID Markdown export of 7c3d38bd-4d47-4620-bb76-4b6ce045eb62 shows named
reads, separate read-only identity/status commands, one social-main.js edit and
final report. No tests, syntax/build commands or Git mutation appear. Displayed
source/CURRENT ranges are shifted by one line but remain in the bounded relevant
sections, without historical reload. The final phrase “without Git” means without
Git mutations; read-only Git commands are present. No raw tool-result audit is
claimed. Reviewer ran no acceptance commands.

Grok is closed. Hermes alone may run the
[focused green/falsification driver](HERMES_BBD_WAL_011_ASYNC_IPC_GREEN_01.md).
No integration, broader tests, startup or native-flow implementation is authorized.
