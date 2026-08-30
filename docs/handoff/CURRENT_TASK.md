# Current Task

Ticket: BBD-WAL-003

State: POST-RED TEST CONTRACT CORRECTION 04 AUTHORIZED — PRODUCTION NOT AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `d472785ab896bb5d1367c4117ffd659a9a8512ae`

[BBD-WAL-003](../../tickets/BBD-WAL-003.md) is the only authorized implementation task
in this repository. Its accepted test source and expected-red evidence are preserved at
commit `a8a70a520ad0a02696086f1f8e79d198ef98f72e`. Before issuing any production handoff,
the reviewer found that the inherited exact three-suite `npm test` assertion conflicts
with the new requirement to include `npm run test:wallet-broker`. Only
[Correction 04](CODEX_SOL_BBD_WAL_003_TESTS_CORRECTION_04.md) may edit the shared
`TOP_LEVEL_TEST_CMD` in `test/securityPolicy.node.js` to append that broker command.
Production, package/workflow/policy implementation, evidence revision, native binary,
Rust install, wallet, node, network service, hardware, and device work remain
unauthorized until the reviewer accepts the correction and independently records its
revised policy expected red.

BBD-WAL-002 is complete and reviewer-accepted at implementation commit
`62ad0ceb90a96805cabff0296fd6980a739554fd` and acceptance commit
`d472785ab896bb5d1367c4117ffd659a9a8512ae`; GitHub Social client run
`33338667462` passed. Its accepted contract and fixtures are protected inputs.

The wallet roadmap routes this trust-boundary ticket to Sol, the independent bb-go
payment protocol ticket to Grok Build when available, and later mechanical UI work to
Codex Spark. `go-ipfs` is deprecated and receives no wallet work.
