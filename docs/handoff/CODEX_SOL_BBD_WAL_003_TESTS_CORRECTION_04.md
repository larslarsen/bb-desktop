# Codex Sol Handoff — BBD-WAL-003 Post-Red Test Contract Correction 04

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `a8a70a520ad0a02696086f1f8e79d198ef98f72e`

Read the ticket, current task, accepted expected-red evidence, all prior Sol handoffs, and
`test/securityPolicy.node.js`. The reviewer found one post-red contradiction before any
production handoff: the inherited wallet-contract test requires `pkg.scripts.test` to
equal the old three-suite `TOP_LEVEL_TEST_CMD`, while the new broker-boundary test
requires that same command to contain `npm run test:wallet-broker`. No green production
state can satisfy both assertions.

Modify only `test/securityPolicy.node.js`. Change the shared `TOP_LEVEL_TEST_CMD` test
constant to exactly:

```text
npm run test:social && npm run test:security && npm run test:wallet && npm run test:wallet-broker
```

Do not weaken either assertion, make the check order-insensitive, alter production, or
change any other test, fixture, evidence, package/workflow/policy file, documentation, or
path. Inspect the surrounding package-contract tests after the edit and report whether
you find any other mutually incompatible green requirements; do not independently
expand scope to fix one.

Use `apply_patch`. Only read-only inspection and final `wc -l`/`sha256sum` for
`test/securityPolicy.node.js` are allowed. Do not execute Node, npm, tests, builds,
formatters, scanners, Git, GitHub, network, Electron, child processes, wallets, nodes,
hardware, or devices. Do not install anything or use root, `sudo`, `/tmp`, deletion,
cleanup, `rm`, globs, or unresolved destructive targets.

Stop after authoring and report the exact change, line count, SHA-256, compatibility
review, expected policy red change, and confirmation that nothing ran. Reviewer XHigh
must accept this corrected source before Codex Luna executes the policy suite.
