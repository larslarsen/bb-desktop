# Current Task

Ticket: BBD-WAL-003

State: TEST SOURCE REVIEWER ACCEPTED — EXPECTED RED RESUME 01 AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `d472785ab896bb5d1367c4117ffd659a9a8512ae`

[BBD-WAL-003](../../tickets/BBD-WAL-003.md) is the only authorized implementation task
in this repository. Its first test drop and Corrections 01–02 were reviewer-rejected
before execution. Correction 03 is accepted at the exact hashes in the
[Codex Luna red handoff](CODEX_LUNA_BBD_WAL_003_RED.md). Only that expected-red
integration, evidence, exact test-source commit, and push are authorized. The first three
missing-module commands matched; the Electron raw inventory is accepted at 13 pass / 6
expected fail despite an initial reporting miscount. Only
[Resume 01](CODEX_LUNA_BBD_WAL_003_RED_RESUME_01.md) may run the remaining policy red and
finish the record sequence. Production, package/workflow/policy implementation, native
binary, Rust install, wallet, node, network service, hardware, and device work remain
unauthorized.

BBD-WAL-002 is complete and reviewer-accepted at implementation commit
`62ad0ceb90a96805cabff0296fd6980a739554fd` and acceptance commit
`d472785ab896bb5d1367c4117ffd659a9a8512ae`; GitHub Social client run
`33338667462` passed. Its accepted contract and fixtures are protected inputs.

The wallet roadmap routes this trust-boundary ticket to Sol, the independent bb-go
payment protocol ticket to Grok Build when available, and later mechanical UI work to
Codex Spark. `go-ipfs` is deprecated and receives no wallet work.
