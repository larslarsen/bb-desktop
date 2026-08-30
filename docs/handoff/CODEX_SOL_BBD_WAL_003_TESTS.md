# Codex Sol Handoff — BBD-WAL-003 Test Source

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Baseline: `d472785ab896bb5d1367c4117ffd659a9a8512ae`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`,
`docs/engineering/WALLET_ROADMAP_ROUTING.md`, `tickets/BBD-WAL-003.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md` §§4.1–4.3 and 5.3–5.4,
`docs/handoff/CURRENT_TASK.md`, and every currently authorized test path.

Your sole task is the test-source phase defined by `tickets/BBD-WAL-003.md`. Author all
required behavioral tests before any production source and stop. Preserve every existing
accepted assertion; convert only the obsolete blanket no-preload/no-IPC expectations into
the ticket's stricter exact allowlist.

Use `apply_patch`. You may use read-only inspection plus `wc -l` and `sha256sum` over the
six authorized paths. Do not execute Node, npm, tests, builds, formatters, scanners, Git,
GitHub, network, Electron, child processes, wallets, nodes, hardware, or devices. Do not
install Rust or any package. Do not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`,
globs, variables/substitutions as destructive targets, or any unlisted path.

Stop after authoring. Report:

- each changed path, line count, and SHA-256;
- every added/changed test name and total tests per suite;
- the CommonJS API that the tests reserve for later production;
- why each group is non-vacuous and the exact expected red cause;
- the pre-existing assertion counts preserved in both amended suites;
- fixture fields and the independent expected session hash;
- confirmation that no command outside the allowed read-only/reporting set ran and no
  unlisted path changed.

Lead Engineer/Reviewer — Codex at XHigh must inspect and accept the test source before
Codex Luna executes anything. You have no production, execution, integration, evidence,
Git, commit, or push authority.
