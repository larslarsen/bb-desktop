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

## Collected review — 2026-09-09

ACCEPT the initial expected-red execution; no repeat is needed. Outer 44012 was
collected once on owner done, exit 0. Runtime session 20260909_135906_d3d5a1 has
provider nous, model poolside/laguna-s-2.1:free; both session metadata and the actual
status output support that model. The unintegrated evidence incorrectly copied
meituan/longcat-2.0:free from historical material. Correct it during the next Hermes
integration, not through a separate report-only actor.

Actual command wrapper: `timeout 45 node test/walletSupervisorTransport.node.js
2>&1; echo "EXIT_CODE=$?"`. The wrapper returned 0; its captured EXIT_CODE=1 is the
test exit. Six not-ok groups, no ok groups. Groups 1–3 report missing real framed
hello/ack/bootstrap connection. Group 4 reports no down publication after the child
exits. Groups 5–6 report object writes where framed Buffers are required. These are
expected missing-production failures, NOT the separate harness cleanup gaps noted
in source review. In particular the fake correctly captures what production writes.
The evidence's contrary explanation must be corrected during later integration.

All five hashes still match. No production/test edits or Git mutation occurred.
Transcript shows extra read-only Git log/show, directory listing, historical evidence
read and final status. Status output included a masked credential indicator and is
not suitable for future runtime metadata collection; use narrow session metadata.
No independent process/directory inspection supports the evidence's clean-process
claim; final cleanup acceptance still awaits the strengthened tests. These reporting
issues do not invalidate the observed missing-transport red. Preserve the raw record
uncommitted until Hermes corrects it alongside source integration.

Hermes execution authorization is closed. Grok may implement the fixed transport
contract and bounded harness corrections under the production handoff.
