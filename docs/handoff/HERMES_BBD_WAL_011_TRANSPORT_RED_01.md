# WAL-011 live transport expected red 01

Actor: Hermes, execution/evidence only. Source actor is closed.
Protected parent: reviewer publication directly after c0ce49a4, with one subsequent
CURRENT-only launch record allowed. Read AGENTS.md, TESTING.md, active CURRENT and
ticket prefixes, HERMES_JR_DEV_ROUTING.md and this handoff; no historical reload.

Verify HEAD/status and the five hashes below before execution. Stop on a mismatch.
The pending npm/policy files and prior WAL-009 evidence are unrelated; preserve
them exactly. The three test paths are an unintegrated source drop. Do not edit them.

| Path | SHA-256 |
| --- | --- |
| wallet-broker/supervisor.js | 2634fd116f476998db2cf4a4e948fcd864397d0d26d3fd98e4c2a5e41d2f0430 |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| test/walletSupervisorTransport.node.js | 284546977bd0a1a11ddc92a9fa743c06884e5157029759dd5003001891510b8b |
| test/fixtures/wallet-broker/transport-child.js | 17cdea3eaec6c01f839eb34c86e25d1fd93f8cad165cd8e9fb7f108bb9fa7fef |
| test/walletSupervisor.node.js | db02281f9314e382255945d569de4f83be8bc0edbc3ff62e5485f11e3d4e4992 |

Record `hermes --version` and the actual resolved provider/model/session from this
run's runtime metadata. Do not dump credentials or full configuration.

Run exactly once from the repository root:

    node test/walletSupervisorTransport.node.js

Expected: nonzero exit, six failing groups on missing framed handshake/listeners/
bootstrap and reply handling. The primary live success case must report
`missing real framed hello/ack/bootstrap connection`. The suite owns its small
temporary fixture directories and child cleanup. No Rust builds or large artifacts.
If the process hangs beyond 45 seconds, interrupt it and report the hang; do not
rerun or fix anything. Missing imports, syntax errors, spawn permissions, fixture
startup failures or a different primary failure are not accepted expected red.
Capture every case line, diagnostics and the actual exit code. No retries, other
suites, syntax commands, npm/policy/audit/scanners, Rust commands or network.

After this one run, verify the same five hashes and git status once. Write only
docs/testing/BBD-WAL-011-TRANSPORT-RED-01.md, recording runtime identity, baseline,
command, counts, exact exit and failures, unchanged hashes and any observed cleanup
or fixture issue. Sanitize local absolute paths in the record. If any prerequisite
or run fails unexpectedly, record that result instead and stop. Known harness gaps
in the source review are not permission to edit tests or assert green readiness.

No Git stage/commit/push, CURRENT edit, source changes or other evidence edits.
Stop after the one evidence record and report it to the reviewer.
