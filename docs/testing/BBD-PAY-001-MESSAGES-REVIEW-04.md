# Messages finish behavioral-red review 04

Reviewer: Codex. Decision: behavioral red accepted for bounded source repair.
No acceptance/test commands were executed by the reviewer.

Read [Hermes finish-red report](BBD-PAY-001-HERMES-FINISH-RED-01.md) and both retained
logs. Independently verified all twelve current source hashes against the handoff and
report, including unchanged production and Sol's exact test-source hashes. HEAD is
`7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

## Verified results and limits

The raw logs contain **65 passing cases and five failing cases**, completing all 70.
All original 38 transport cases pass. Failures reproduce the expected implementation
defects, not harness syntax errors:

1. Reused outgoing bubble lacks the updated read label.
2. Distinct same-time/body messages share one entry key.
3. Repeated history starts three concurrent receipts instead of one.
4. Peer switching leaves the old receipt request un-aborted.
5. Disposal leaves two owned timers instead of zero.

Retained evidence:

- [stdout](../../dist/pay001-finish-red01/run-03-behavioral-red.stdout.log), SHA-256
  `2597be3deabeebddaaabcb4985ac4cf96f109a1a915443064c7cd8cb4c71cb97`.
- [stderr](../../dist/pay001-finish-red01/run-03-behavioral-red.stderr.log), SHA-256
  `114ff65d38deccdedb002eabc15e5320188ebf0c9c42e73ac79b9a5c30333d94`.

Hermes reports exit 1 from the bounded suite command and both syntax checks exit 0.
Only the two raw suite logs were retained: separate command metadata, syntax captures,
version-output files and before/after manifest files are absent. The report's start time
is approximate. These details remain actor-reported, not independently captured facts;
do not invent them or rerun this red merely to reconstruct the record. Complete named
assertion failures in retained logs plus the unchanged source pins establish enough
behavioral red to authorize repair. Future execution must retain the required metadata.

Hermes's sentence saying Sol is authorized to act was premature. This reviewer decision
and the linked source handoff are the authorization; execution actors do not select
the next phase. The actual final verdict correctly deferred to reviewer approval.

## Next source scope

[Sol Messages finish production 01](../handoff/SOL_BBD_PAY_001_MESSAGES_FINISH_PRODUCTION_01.md)
authorizes app.js repairs and narrowly shared Node fixture teardown. Retain all 70
regression assertions. Include the previously reviewed viewport/focus fix; its browser
regression is already authored, but Electron has not run and will remain a distinct
validation gate. No claim of a browser red result is made.

Sol remains source-only. Hermes owns later green, falsification and runtime evidence
after reviewer source inspection. No owner UI check or publication is requested.
