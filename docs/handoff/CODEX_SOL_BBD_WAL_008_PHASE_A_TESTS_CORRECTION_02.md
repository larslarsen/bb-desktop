# Codex Sol Handoff — BBD-WAL-008 Phase-A Test Correction 02

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `docs/handoff/CURRENT_TASK.md`,
`tickets/BBD-WAL-008.md`, and
`docs/testing/BBD-WAL-008-PHASE-A-TEST-SOURCE-REVIEW-02.md`.

Edit exactly one path:

- `wallet-broker/tests/zec_hardware.rs`

At the redaction canary test, immediately after
`install_observable_canaries_for_test`, assert for every `CANARY_INPUTS` entry that the
slot contains the exact canary and its touch count is zero. Preserve the existing
post-exercise assertions that every touch count is positive. Make no other semantic
change.

The manifest remains frozen at 117 lines and SHA-256
`7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530`.
Do not edit any other path. Read-only source inspection and final `wc -l`/`sha256sum`
over the authorized test file are allowed. Do not run Git, formatter, test, build,
dependency resolver, product binary, GitHub, network, or another actor. Stop with the
new line count and SHA-256.
