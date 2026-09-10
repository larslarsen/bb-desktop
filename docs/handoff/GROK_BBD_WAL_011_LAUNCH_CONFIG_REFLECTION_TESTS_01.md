# WAL-011 launch resolver reflection regression tests 01

Actor: Grok Build, grok-4.6 High; test source only, no subagents.
Protected parent: reviewer publication following 9d5d9c457e3b167dee33905e16fb0ef91f6a391d
plus one CURRENT-only launch record. Read AGENTS.md, TESTING.md, this handoff,
tickets/BBD-WAL-011.md and CURRENT with sed -n '1,40p' docs/handoff/CURRENT_TASK.md.
Only other reads: test/walletBrokerLaunchConfig.node.js, wallet-broker/launch-config.js,
and the collected source review section of GROK_BBD_WAL_011_LAUNCH_CONFIG_PRODUCTION_01.md.

Verify before editing:
- test/walletBrokerLaunchConfig.node.js: 851 lines, SHA-256
  1b6a7d8c102360efdbd0920b6211e7b55999fbdfff18aff4eee23cd311d750f8.
- wallet-broker/launch-config.js: 132 lines, SHA-256
  4a639b1644ff0ea618fc53fd525cbd5c0fde84fbea5905f3f3eb56a5520dc01a.

Only write test/walletBrokerLaunchConfig.node.js. Preserve every existing byte
except insertion immediately before function run(). Keep existing five groups,
58 rows, helpers, runner and exports unchanged. Add exactly four independently
registered synchronous groups with the following exact names:

1. reflection errors: revoked proxy is sanitized
2. reflection errors: getPrototypeOf trap is sanitized
3. reflection errors: ownKeys trap is sanitized
4. reflection errors: getOwnPropertyDescriptor trap is sanitized

Each must call loadResolver and then assertUnavailable on the actual public
resolver call. The first uses a Proxy.revocable object revoked before invocation.
The other three use a fresh otherwise-valid plain options target with exactly
resourcesPath, userDataPath, platform: linux and arch: x64, wrapped with precisely
the named throwing trap. Use path.resolve for two inert absolute path strings;
no filesystem fixtures are necessary because reflection fails before file access.
Each throwing trap increments a counter and throws a fresh Error containing CANARY.
After assertUnavailable, assert the trap ran exactly once. Pass canary and path
strings to the existing helper so it checks code, fixed message, absence of cause,
no partial result and diagnostic sanitization. Do not call traps directly to prove
behavior, patch globals, mock the resolver, or introspect production source.
All cases must reach their intended reflection operation against the current
implementation. Existing valid cases already prevent an always-unavailable stub.

This supplements the existing fixed error contract; no new launch behavior. The
reviewer anticipates four assertion failures at the missing UNAVAILABLE code.
After source review, Hermes will execute just the four new exported groups once
to establish red; no test execution is authorized for Grok. A later source
correction will normalize uncaught errors at the public resolver boundary, followed
by nine-group green (62 authored rows), affected supervisor tests and falsification.

Separate read-only HEAD/status/hash/count and bounded named reads only. No command
chaining, history, home skills/config, unrelated discovery, production edits, tests,
syntax checks, Node/npm/Cargo/build execution, evidence/docs edits, Git mutation,
network or additional actors. Preserve all pending files. Report the one path,
SHA-256, line count, four added groups and stop for source review.

## Source acceptance — 2026-09-09

Accept 90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113,
949 lines. Removing the single 98-line insertion reconstructs the exact accepted
851-line hash. Four independent public-boundary calls cover revoked proxy and
three throwing reflection operations; trap counters and the shared diagnostic
assertions are non-vacuous on green. Production remains 4a639b16. Exact Grok export
e8ddbd20-50d2-4fb2-a152-2225ff0b95f8 shows named reads/checks, one test edit,
no execution or Git mutation. Source actor is closed. Only the reflection-red
Hermes handoff authorizes execution; reviewer ran no tests.
