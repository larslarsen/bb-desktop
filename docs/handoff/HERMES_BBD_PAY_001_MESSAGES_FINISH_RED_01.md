# Hermes — Messages finish behavioral red 01

CLOSED — actual report/logs reviewed in
[review 04](../testing/BBD-PAY-001-MESSAGES-REVIEW-04.md); behavioral red accepted.
No further execution under this handoff. Earlier missing-report notes below are history.
Reviewer: Codex. Owner-relayed Hermes.

Repository target: **bb-desktop**, not its sibling bb-go. Start at the repository root
containing `social/app.js` and `test/paymentInbox.node.js`, and verify the desktop HEAD
below before execution. The owner's pasted sixteen-file BBGO-PAY-003 publication list
belongs to an already accepted/published daemon task; it is not this task or an active
publication authorization. Do not stage, commit or publish that list. Follow only this
BBD-PAY-001 desktop handoff and write the desktop report named below.

Reviewer completion check: the named report and capture directory were absent after
the owner's latest completion notice. This same handoff remains active; no new task is
needed. If commands already ran in another checkout, locate and preserve their actual
logs and source identities, record the actual cwd and copy/link the evidence into the
authorized paths here. Do not rerun merely to replace missing documentation. If no run
occurred, execute the authorized commands below. If blocked, save the exact blocker in
the named report. Chat-only completion is insufficient; do not ask the owner for logs.
Read AGENTS.md, TESTING.md, HERMES_JR_DEV_ROUTING.md, CURRENT_TASK and
[test-source review 03](../testing/BBD-PAY-001-MESSAGES-REVIEW-03.md).
Sol's two-file source phase is complete. No source edits or repairs are authorized.

## Baseline

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index, preserve dirty work.
Verify these exact identities before and after the run:

| Path | SHA-256 |
| --- | --- |
| test/paymentInbox.node.js | 4a14a37fa89ccc5a0f046e9f47518a4a320a836dec3a27ae60cf1eb30bff6931 |
| test/paymentInbox.electron.js | d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf |
| social/app.js | daa737ff2be1b89b6bbf6c74c51c2ffaaeaed2339e28796302b6d787ace08e5e |
| social/payment-inbox.js | 437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64 |
| social/index.html | 6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24 |
| social/styles.css | f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898 |
| wallet-pay/inbox-client.js | a41463f491fdf1aa317d972f69438b62c273ccb59ad8e2a99444fabe563de8e5 |
| social-main.js | 298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6 |
| wallet-preload.js | 6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025 |
| social/core.js | 4d27e4701ff299ef9753c02174bd8868aa01c94509b84d5a8187978fa239fb8f |
| wallet-contract/canonical.js | 32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761 |
| test/fixtures/payment-inbox/records-v1.json | 76f5a86a7337c3733b36696ad212c976ff8a7c2c848b7b1cdda0e1b968ef0650 |

Stop on drift; document it rather than resetting files. No Git mutation, dependencies,
source changes, formatter, real daemon/user data, Electron or sandbox configuration.

## Writable evidence paths

- `docs/testing/BBD-PAY-001-HERMES-FINISH-RED-01.md` only repository report.
- New ignored directory `dist/pay001-finish-red01/` for captures/manifests/version output.
- Existing test code creates unique fixture trees under
  `dist/pay001-grok-correction01/trees/`; permit only its new owned trees. Preserve all
  earlier trees/logs and do not manually delete anything.

Inspect destination filesystem before artifacts; use disk-backed repository dist.
Record actual `hermes --version`, provider/model and `node --version`. Record source
manifest, line counts, exact argv/cwd, timestamps, exit/signal, stdout/stderr file links
and hashes. The capture wrapper may record process results but must not change tests
or alter their exit. Use new capture names and retain every failure.

## Exact commands, cwd bb-desktop

```text
node --check test/paymentInbox.node.js
node --check test/paymentInbox.electron.js
timeout --signal=TERM --kill-after=5s 120s node test/paymentInbox.node.js
```

Run each once, sequentially. Stop if a syntax check fails. The 120-second external
limit prevents an interrupted run from hanging indefinitely; exit 124/137 or a signal
is a timeout/runner failure, not a behavioral test result. Do not insert process.exit
or suppress failing cases. No broader regression/security commands yet.

Expected: syntax passes, suite naturally finishes nonzero because current production
does not update reused text read labels, disambiguate duplicate text, coalesce/abort
read receipts or cancel the typing timer on disposal. Read the actual assertions;
do not assume all listed defects must be the only failures. The source contains 70
registered cases; report observed pass/fail counts from complete logs, plus teardown
failures separately. Existing 38 transport cases should remain green.

If it unexpectedly passes, hangs, has harness errors or breaks unrelated cases, record
that and stop for reviewer diagnosis. Do not repair or rerun unchanged source.

## Completion

Write the named report and before/after source manifests even for failure. Distinguish
expected behavioral red from harness/timeout failures, include the exact failing case
names and assertion messages, and leave unavailable evidence explicit. No acceptance
or successful production behavior is claimed. Owner relays only the saved report pointer.
Codex next reviews these results before authorizing Sol's bounded production changes.
