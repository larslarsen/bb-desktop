# Current Task

Ticket: BBD-WAL-003

State: LOCAL GREEN STOPPED — PRODUCTION CORRECTION 02 AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `d472785ab896bb5d1367c4117ffd659a9a8512ae`

[BBD-WAL-003](../../tickets/BBD-WAL-003.md) is the only authorized implementation task
in this repository. Its accepted test source and expected-red evidence are preserved at
commit `a8a70a520ad0a02696086f1f8e79d198ef98f72e`. Before issuing any production handoff,
the reviewer found that the inherited exact three-suite `npm test` assertion conflicts
with the new requirement to include `npm run test:wallet-broker`. Sol Correction 04 is
reviewer-accepted at 1,574 lines and SHA-256
`1414a32cb114b1467c9d39bbcbf02228aa185857b0ccc304cd4356be9a02507b`.
The Luna Correction 04 policy-red handoff executed the corrected policy suite and
recorded its exact 53-pass/5-fail result in
[the correction evidence](../testing/BBD-WAL-003-CORRECTION-04-EXPECTED-RED.md). The
reviewer accepts that gate. Only
[the Sol production handoff](CODEX_SOL_BBD_WAL_003_PRODUCTION.md) authored the eight named
production/package/workflow/policy paths without execution. The reviewer rejected that
drop before execution because the real supervisor does not normalize the two accepted
no-payload Electron calls to the broker's required empty parameter object. Only
[Production Correction 01](CODEX_SOL_BBD_WAL_003_PRODUCTION_CORRECTION_01.md) edited only
`wallet-broker/supervisor.js` to close that mismatch. The reviewer accepts the exact
eight-path source hash set recorded in
[the Luna green handoff](CODEX_LUNA_BBD_WAL_003_GREEN.md). Luna stopped on its first
targeted command at 10 protocol passes / 1 handshake failure; no later command ran. The
numeric child PID was accepted through regex string coercion. Only
[Production Correction 02](CODEX_SOL_BBD_WAL_003_PRODUCTION_CORRECTION_02.md) may add
explicit string guards for the same validation defect family in protocol, supervisor, and
Electron main. Execution, native binary, Rust install, wallet, node, network service,
hardware, and device work remain unauthorized pending renewed reviewer acceptance.

BBD-WAL-002 is complete and reviewer-accepted at implementation commit
`62ad0ceb90a96805cabff0296fd6980a739554fd` and acceptance commit
`d472785ab896bb5d1367c4117ffd659a9a8512ae`; GitHub Social client run
`33338667462` passed. Its accepted contract and fixtures are protected inputs.

The wallet roadmap routes this trust-boundary ticket to Sol, the independent bb-go
payment protocol ticket to Grok Build when available, and later mechanical UI work to
Codex Spark. `go-ipfs` is deprecated and receives no wallet work.
