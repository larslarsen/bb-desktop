# WAL-009 signature-context falsification order 01

Actor: Grok Build, CLI model `grok-4.6`, reasoning High.
Governance parent: the commit containing this handoff.
One existing test assertion moves; no other source change.

Read AGENTS.md, TESTING.md, this handoff, only the first 75 lines of
docs/handoff/CURRENT_TASK.md, and
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-PRODUCTION-01-REVIEW.md.
Inspect only lines 328-390 of the test module below. No dependency/API lookup,
repository-wide search, home-directory search, historical task read, or other actor.

## Exact baseline and scope

Only writable path: wallet-broker/src/zec/spend/verification_context_tests.rs,
528 lines, SHA-256
`78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec`.

Read-only anchors:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 929 | bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361 |
| wallet-broker/src/zec/test_support.rs | 4371 | fea8f65ed6637033506902688c8f547952cea848af51d05a49969cae81f48920 |

Verify all three identities before edits; stop on mismatch. All production source,
test helpers, other tests, module declarations/accessors, fixtures, dependencies,
manifests, locks, package files, evidence, and governance are frozen.

## Single exact correction

Inside uses_real_spend_and_binding_signatures, move the existing line

`    assert_eq!(message, fixture.pczt_sighash);`

from immediately after `let message = context.shielded_sighash();` to immediately
after the closing `);` of the valid binding-signature assertion, and before the
existing blank line followed by `let mut altered = message;`.

Keep the line's text and indentation exactly. Do not add/remove blank lines,
rewrite assertions, change either verifier's message, alter the action count,
move any other statement, change the PCZT oracle, add a test/helper, or touch the
later mutation controls. Keep all four test names and every other byte unchanged.

The expected final module is 528 lines, SHA-256
`45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a`.
The reviewer computed this proposed identity in memory without writing test source.
If the final hash differs, stop and report the mismatch; do not make speculative
extra changes.

## Purpose and reserved execution

The current early oracle equality would stop a deliberately corrupted context
message before the test's actual signature-positive assertion. Moving the existing
equality after both valid-signature controls preserves its independent check and
allows the required signature falsification to reach the real verifier. The first
test continues to supply the separate early PCZT-oracle failure.

No execution or falsification is authorized now. The existing focused command,
transparent-rejection and message-corruption falsifications with exact restoration,
and broader zec_prepare/zec_sign_verify/library commands remain reserved to a later
Hermes handoff after reviewer inspection. Four tests are declared; no test result
is claimed. The accepted production context stays frozen.

Read-only measurements and this single test-source reorder only. No compiler,
formatter, test, Cargo/npm, Git, network, native launch, evidence, integration,
dependency operation, or subagent. No build artifacts are needed. All pending
work stays uncommitted.

Report the final module hash/line count, the exact moved assertion, and both
unchanged anchor identities. Then stop. The reviewer collects once only after
the owner reports done; do not poll or launch another actor.
