# BBD-PAY-001 — Hermes finish behavioral red 01

Date: 2026-09-16. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and versions

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 110baa09 · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Node: v24.14.0

## Baseline

HEAD: `7a31c41cb29692a94acf1f24adb379f3a829d237` (verified before/after).
Empty index. Unrelated dirty work preserved.

## Source manifest (verified before/after)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.node.js | 2703 | `4a14a37fa89ccc5a0f046e9f47518a4a320a836dec3a27ae60cf1eb30bff6931` |
| test/paymentInbox.electron.js | 903 | `d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf` |
| social/app.js | 1160 | `daa737ff2be1b89b6bbf6c74c51c2ffaaeaed2339e28796302b6d787ace08e5e` |
| social/payment-inbox.js | 340 | `437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64` |
| social/index.html | 189 | `6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24` |
| social/styles.css | 988 | `f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898` |
| wallet-pay/inbox-client.js | 609 | `a41463f491fdf1aa317d972f69438b62c273ccb59ad8e2a99444fabe563de8e5` |
| social-main.js | 322 | `298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6` |
| wallet-preload.js | 72 | `6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025` |
| social/core.js | 64 | `4d27e4701ff299ef9753c02174bd8868aa01c94509b84d5a8187978fa239fb8f` |
| wallet-contract/canonical.js | 373 | `32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761` |
| test/fixtures/payment-inbox/records-v1.json | 81 | `76f5a86a7337c3733b36696ad212c976ff8a7c2c848b7b1cdda0e1b968ef0650` |

All 12 pins matched before and after the run. No source changes.

## Commands executed

### run-01-syntax-inbox

Command: `node --check test/paymentInbox.node.js`
Exit: 0

### run-02-syntax-electron

Command: `node --check test/paymentInbox.electron.js`
Exit: 0

### run-03-behavioral-red

Command: `timeout --signal=TERM --kill-after=5s 120s node test/paymentInbox.node.js`
Cwd: bb-desktop
Started: 2026-09-16T01:10:00Z (approx). Ended: ~12s elapsed.
Exit: 1
Stdout: `dist/pay001-finish-red01/run-03-behavioral-red.stdout.log`
Stderr: `dist/pay001-finish-red01/run-03-behavioral-red.stderr.log`

## Behavioral red results

**Result: 65 passed, 5 failed (5/70 failing).**

The 38 transport cases remain green. The 5 failures are all behavioral red —
current production does not yet:

1. **outgoing read labels update in place after a live event and fallback poll**
   - Assertion: `live-read-transitionAug 30, 5:00 AM`
   - Production does not update reused text read labels after live events.

2. **message IDs preserve identical same-time text through unrelated request updates**
   - Assertion: `'t:2026-08-30T12:00:00Z:0:identical-body'` should be unique per message
   - Production does not disambiguate duplicate text with identical timestamps.

3. **read receipts coalesce repeated history and acknowledge only later unread IDs**
   - Assertion: `3 !== 1` (duplicate receipt started while one was in flight)
   - Production does not coalesce/abort read receipts.

4. **peer session and disposal abort receipt ownership and ignore late completion**
   - Assertion: `false !== true` (peer switch retained old receipt ownership)
   - Production does not cancel receipt ownership on peer/session changes.

5. **typing reset timer is owned and cancelled by disposal**
   - Assertion: `2 !== 0` (disposal left a typing or reconciliation timer)
   - Production does not cancel the typing timer on disposal.

## Assessment

The behavioral red confirms the expected gaps in current production:
- Read label updates in place
- Duplicate text disambiguation
- Read receipt coalescing
- Receipt ownership abort on peer/session change
- Typing timer cancellation on disposal

These are the bounded production changes Sol is authorized to make next.

## Evidence paths

- `dist/pay001-finish-red01/run-03-behavioral-red.stdout.log`
- `dist/pay001-finish-red01/run-03-behavioral-red.stderr.log`

## Verdict

BEHAVIORAL RED CAPTURED — 5/70 failing, all expected gaps. No source edits.
Codex reviews before authorizing Sol's bounded production changes.
