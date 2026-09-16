# Hermes — recover the interrupted Messages correction record

CLOSED — recovered record reviewed; see
[review 02](../testing/BBD-PAY-001-MESSAGES-REVIEW-02.md) for corrections and gaps.
No further recovery execution authorized. Historical scope follows.
Reviewer: Codex. Owner reports Grok exhausted its weekly
usage during Messages correction 01. No Grok completion report exists, but four source
files changed and eleven command captures were retained. Preserve this work.

Read AGENTS.md, `docs/engineering/HERMES_JR_DEV_ROUTING.md`, CURRENT_TASK,
[the correction contract](GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md) and
[the preceding review](../testing/BBD-PAY-001-MESSAGES-REVIEW-01.md).
Grok's execution authority is paused. This handoff authorizes recovering the evidence
record, not advancing source acceptance or starting another implementation.

## Exact scope

Only writable repository path:
`docs/testing/BBD-PAY-001-HERMES-CORRECTION-RECOVERY-01.md`.

Read source, existing reports and `dist/pay001-messages-correction01/` captures.
Read-only version/provider/model identification, Git status/HEAD, hashing and line
counting are authorized. No tests, syntax checks, Electron, builds, installs, source
edits, Git mutation or rewriting original captures. Do not create the missing report
under Grok's name. Record actual Hermes identity separately from recovered Grok evidence.

Reviewer observed HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index and:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.node.js | 2404 | 1fee6ac398dd35de7ebfaa185e85b8aba608dd73ff850381846ecfdd7c84750c |
| test/paymentInbox.electron.js | 661 | 932257736999d2c5d04deb55e7bba26806ce190d31503b5c21a362d41970d492 |
| social/app.js | 1160 | daa737ff2be1b89b6bbf6c74c51c2ffaaeaed2339e28796302b6d787ace08e5e |
| social/payment-inbox.js | 340 | 437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64 |
| social/index.html | 189 | 6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24 |
| social/styles.css | 988 | f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898 |

## Recover facts, retain gaps

For each retained capture, record exact argv/cwd/timestamps/exit/count, metadata/log
links and SHA-256. Verify the logs against their metadata. Reviewer already observed
matching log hashes for all eleven captures:

- run-01-red: inbox exit 1;
- run-02-green: inbox exit 0, 63 tests;
- run-02-security: exit 0, 32 tests; run-02-socialcore: exit 0;
- run-02-check-app, check-inbox, check-electron: syntax exit 0;
- run-03-falsify: exit 0, 63 tests — ineffective falsification, not a successful proof;
- run-04-green: exit 0, 63 tests;
- run-05-falsify: exit 1, old-Alice-history assertion failed;
- run-06-restored: exit 0, 63 tests.

Read stderr to identify actual red/falsification failures. Inventory any retained
before/mutated/restored source identities; do not infer exact mutation/restoration or
test-first chronology from run labels alone. Distinguish current file hashes from the
source bytes tested by earlier commands when those commands lack source manifests.
Record unavailable actor narrative and restoration evidence as gaps. No claim that
63 passing tests resolves all review findings. Electron remains unexecuted unless an
actual retained execution record proves otherwise.

Provide a concise source/test change inventory based on the files, clearly marked as
Hermes's reconstruction. Preserve unrelated dirty work. If baseline drift or conflicting
evidence is found, document it without repairing files. Stop for Codex review.

## Routing after recovery

Codex reviews the recovered drop before selecting more source work. If corrections
remain, the documented fallback is Codex Sol (`gpt-5.6-sol`, High), because Grok is
unavailable and its correction attempt stopped without a completed reviewable report.
This records an availability interruption, not a claim of model inadequacy. Sol retains
its source-only role; Hermes owns test execution and evidence. No Sol task or actor
launch is authorized here. No owner transcription or further manual UI test is needed.

Reviewer governance paths for this routing: this handoff, CURRENT_TASK,
GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md, tickets/BBD-PAY-001.md and
docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md.
