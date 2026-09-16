# BBD-PAY-001 — Hermes correction record recovery

Date: 2026-09-16. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Authorization

This is record recovery only. Grok's execution authority was paused after weekly
usage exhaustion. No Grok completion report exists. This document reconstructs
evidence from retained source and command captures. No source edits, Git mutations,
test execution, or implementation advancement are authorized.

## Actor identity

Hermes (recovery actor): meituan/longcat-2.0:free via Nous Portal.
Grok (original actor, paused): Grok Build 4.6 High — completion report unavailable.

## Baseline

HEAD: `7a31c41cb29692a94acf1f24adb379f3a829d237` (verified at reviewer inspection).
Empty index. Unrelated dirty work preserved.

## Recovered source inventory

| Path | Lines | SHA-256 | Role |
| --- | ---: | --- | --- |
| test/paymentInbox.node.js | 2404 | `1fee6ac398dd35de7ebfaa185e85b8aba608dd73ff850381846ecfdd7c84750c` | Test harness (corrected) |
| test/paymentInbox.electron.js | 661 | `932257736999d2c5d04deb55e7bba26806ce190d31503b5c21a362d41970d492` | Electron test harness (corrected) |
| social/app.js | 1160 | `daa737ff2be1b89b6bbf6c74c51c2ffaaeaed2339e28796302b6d787ace08e5e` | Production (corrected) |
| social/payment-inbox.js | 340 | `437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64` | Production (corrected) |
| social/index.html | 189 | `6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24` | Frozen (unchanged) |
| social/styles.css | 988 | `f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898` | Frozen (unchanged) |

The four corrected paths differ from their starting SHA-256 values in the
correction handoff. Exact before/mutated/restored hashes were not retained.

## Retained command captures

All captures in `dist/pay001-messages-correction01/`. Cwd: `bb-desktop`.

### run-01-red — pre-fix red

- Command: `node test/paymentInbox.node.js`
- Started: 2026-09-16T00:44:29Z. Ended: 2026-09-16T00:44:34Z.
- Exit: 1
- Stdout: 7 passing tests, then 8 failures.
- Actual failures (from stderr):
  1. `A to B to A keeps Alice and Bob transcripts on the correct peer` — missing pending chatmessages for Bob
  2. `old send completion does not erase a newer conversation draft` — `'' !== 'bob-draft'`
  3. `old session config cannot restore prior conversation after a switch` — stale payment card rendered
  4. `fast-failing conversations still allow hanging history to finish its own deadline` — "Messages could not be loaded"
  5. `conversation list survives failed active history and history survives failed conversations` — "Messages could not be loaded"
  6. `disposal during inflight chat and payment work ignores late completions` — `10 !== 8`
  7. `proto peer ids do not corrupt the conversation map and outgoing previews keep You` — `__proto__` peer id missing from rows
  8. `old Alice history does not appear after Bob is open` — stale alice history visible

### run-02-green — post-fix green

- Command: `node test/paymentInbox.node.js`
- Started: 2026-09-16T00:57:10Z. Ended: 2026-09-16T00:57:15Z.
- Exit: 0
- Tests: 63 passing (63 including pre-fix passing tests)
- Stdout ends with: `BitBook payment inbox tests passed (63).`

### run-02-security — electron security

- Command: `node test/electronSecurity.node.js`
- Started: 2026-09-16T00:57:15Z. Ended: 2026-09-16T00:57:15Z.
- Exit: 0
- Tests: 32 passing

### run-02-socialcore — social core

- Command: `node test/socialCore.node.js`
- Started: 2026-09-16T00:57:15Z. Ended: 2026-09-16T00:57:15Z.
- Exit: 0

### run-02-check-app — syntax check

- Command: `node --check social/app.js`
- Exit: 0

### run-02-check-inbox — syntax check

- Command: `node --check social/payment-inbox.js`
- Exit: 0

### run-02-check-electron — syntax check

- Command: `node --check test/paymentInbox.electron.js`
- Exit: 0

### run-03-falsify — ineffective falsification

- Command: `node test/paymentInbox.node.js`
- Started: 2026-09-16T00:58:00Z. Ended: 2026-09-16T00:58:05Z.
- Exit: 0
- Tests: 63 passing
- **Assessment:** Falsification did not break any test. Not a successful proof.

### run-04-green — re-run green

- Command: `node test/paymentInbox.node.js`
- Started: 2026-09-16T00:59:48Z. Ended: 2026-09-16T00:59:53Z.
- Exit: 0
- Tests: 63 passing

### run-05-falsify — effective falsification

- Command: `node test/paymentInbox.node.js`
- Started: 2026-09-16T01:00:13Z. Ended: 2026-09-16T01:00:19Z.
- Exit: 1
- Tests: 62 passing, 1 failing
- Failure: `old Alice history does not appear after Bob is open` — stale-alice-history visible after Bob is open
- **Assessment:** This falsification was effective. The test caught stale state.

### run-06-restored — post-restore green

- Command: `node test/paymentInbox.node.js`
- Started: 2026-09-16T01:00:51Z. Ended: 2026-09-16T01:00:56Z.
- Exit: 0
- Tests: 63 passing

## Evidence gaps

The following were not retained and cannot be recovered:

- Grok's completion report (`docs/testing/BBD-PAY-001-GROK-MESSAGES-CORRECTION-01.md`)
- Before/mutated/restored source hashes for each correction iteration
- Exact mutation applied during each falsification attempt
- Test-first chronology (run labels alone do not establish which tests were added before/after production edits)
- Grok's actor narrative and restoration evidence
- Electron execution record (Electron tests were not run)

## Assessment

The correction achieved green (63/63 tests passing) and one falsification was
effective (run-05-falsify caught stale old-Alice-history state). However:

1. run-03-falsify was ineffective — it did not prove the test binds to the mechanism.
2. Only one of eight original red failures was explicitly caught by falsification.
3. The other seven red failures may have been fixed by the production edits, but
   without before/source manifests, this cannot be verified from retained evidence.
4. No source manifests tie the tested bytes to specific production edits.

## Routing after recovery

Codex reviews this recovered evidence before selecting further source work.
If corrections remain, the documented fallback is Codex Sol (`gpt-5.6-sol`, High),
because Grok is unavailable and its correction attempt stopped without a completed
reviewable report. Sol retains its source-only role; Hermes owns test execution
and evidence.

Reviewer governance paths: this recovery, CURRENT_TASK,
GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md, tickets/BBD-PAY-001.md and
docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md.
