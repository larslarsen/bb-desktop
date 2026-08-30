# Grok Build Handoff — BBD-WAL-001

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This file is the complete durable
prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: recorded in `tickets/BBD-WAL-001.md`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-001.md`, and
`docs/handoff/CURRENT_TASK.md`.

Perform the architecture review required by BBD-WAL-001 and author only:

- `docs/architecture/BBD-WAL-001-REVIEW.md`

Make concrete recommendations and rejected-alternative decisions. Treat the July 2026
Zcash Ironwood transition and hardware capability drift as explicit compatibility risks;
do not equate generic ZEC support with shielded signing. Treat Monero's node and wallet
processes separately. Preserve a private wallet-broker boundary, device-neutral signing,
and a wallet-free social daemon.

The final document must contain: context and fixed owner decisions; trust boundaries and
data-flow diagram; threat table; process/repository/IPC decision; versioned message and
state-machine contracts; account/signer capability model; ZEC and XMR adapter designs;
hardware support policy; key/recovery policy; payment-request design; UX states; offline
test/falsification plan; security/SBOM/release gates; ordered ticket decomposition; open
questions that genuinely require owner choice; and a clear first implementation slice
that cannot create or move funds.

Do not execute tests, builds, installs, formatters, scanners, Git, GitHub, wallet, node,
network, hardware, or package commands. Do not touch a real device or secret. Do not use
`/tmp`, root, deletion, cleanup, `rm`, globs, variables as targets, or unresolved paths.
Do not edit any other file.

In your terminal response report only the authored path, line count, SHA-256, principal
decisions, unresolved owner choices, and confirmation that no out-of-scope action ran.
Codex reviews the source; Codex Luna owns integration, evidence, Git, and push.

