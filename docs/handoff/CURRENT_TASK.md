# Current Task

Ticket: BBD-WAL-003

State: PRODUCTION SOURCE AUTHORIZED — EXECUTION NOT AUTHORIZED

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
[the Sol production handoff](CODEX_SOL_BBD_WAL_003_PRODUCTION.md) may now author the
eight named production/package/workflow/policy paths. Tests, execution, Git, native
binary, Rust install, wallet, node, network service, hardware, and device work remain
unauthorized for the source actor. Codex Luna may execute only after reviewer source
acceptance and a separate durable handoff.

BBD-WAL-002 is complete and reviewer-accepted at implementation commit
`62ad0ceb90a96805cabff0296fd6980a739554fd` and acceptance commit
`d472785ab896bb5d1367c4117ffd659a9a8512ae`; GitHub Social client run
`33338667462` passed. Its accepted contract and fixtures are protected inputs.

The wallet roadmap routes this trust-boundary ticket to Sol, the independent bb-go
payment protocol ticket to Grok Build when available, and later mechanical UI work to
Codex Spark. `go-ipfs` is deprecated and receives no wallet work.
