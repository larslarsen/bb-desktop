# Codex Sol Handoff — BBD-WAL-008 Phase-A Test Correction 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. This correction
continues the documented Grok-outage fill-in route.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `docs/handoff/CURRENT_TASK.md`,
`tickets/BBD-WAL-008.md`, the original Phase-A handoff, and
`docs/testing/BBD-WAL-008-PHASE-A-TEST-SOURCE-REVIEW-01.md`.

The manifest edit at 117 lines and SHA-256
`7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530`
is accepted and frozen. Edit exactly one path:

- `wallet-broker/tests/zec_hardware.rs`

Make only the five bounded corrections in Source Review 01:

1. separate device spend/route denial from account view/receive state;
2. assert the full forbidden-authority negative set in the positive synthetic route;
3. make every redaction canary demonstrably non-vacuous and scan persisted bytes
   without UTF-8 assumptions;
4. add the persist-narrow/reopen/no-silent-expansion/fresh-exact-restoration sequence
   and directory-sync fault; and
5. cover the exact allowed fingerprint alphabet `A-Za-z0-9._+-` as well as existing
   rejected characters and bounds.

Preserve all other accepted tests and the synthetic-only positive profile. Do not edit
the manifest, production/test-support source, existing tests, fixtures, lockfile,
Node/Electron, policy, docs, evidence, workflows, packages, or any other path.

Read-only source inspection and final `wc -l`/`sha256sum` over the one authorized path
are allowed. Do not run Git (including read-only Git), a formatter, test, build,
dependency resolver, product binary, GitHub, network, or another actor. Stop after the
edit and report the new line count/SHA plus a direct mapping of each correction.
