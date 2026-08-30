# Codex Luna Handoff — BBD-WAL-001 Integration

You are **Jr Dev — Codex Luna** (`gpt-5.6-luna`). This is the complete durable integration
contract; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-001.md`,
`docs/handoff/CURRENT_TASK.md`, `docs/handoff/GROK_BUILD_BBD_WAL_001.md`, both correction
handoffs, and `docs/architecture/BBD-WAL-001-REVIEW.md`.

Parent governance baseline: `aa1acba9`. Expected `HEAD` is the reviewer-acceptance commit
that contains this handoff, has `aa1acba9` as its first parent, and equals `origin/master`.

Reviewer-accepted source guard:

- path: `docs/architecture/BBD-WAL-001-REVIEW.md`
- line count: `2271`
- SHA-256: `aae487b169689f310b222640427c1cdae62850d39ebb0243e29f10568d6fcb3f`

## Authorized work

1. Verify `HEAD` contains this handoff, its first parent is the governance baseline above,
   and `origin/master` equals `HEAD`.
2. Verify the worktree contains only the expected untracked architecture path before any
   integration action. Any additional change stops the ticket and returns to Codex.
3. Verify the exact path, line count, SHA-256, and these fail-closed assertions:
   wallet-free `bb-go`; no Electron confirm/unlock/broadcast; NU6.3/Ironwood; full
   authenticated XMR wallet RPC rather than `--restricted-rpc`; exact atomic amounts;
   rates optional; separate quote worker; no `/ob/exchangerates`; no OB1 ticker path;
   BBD-WAL-002 cannot construct or move funds.
4. Run documentation-only whitespace validation. No application test, build, install,
   audit, scanner, formatter, package, wallet, node, provider, network, or device command
   is required or authorized.
5. Stage only `docs/architecture/BBD-WAL-001-REVIEW.md`. Verify the staged diff contains
   only that path and passes `git diff --cached --check`.
6. Commit with message `docs: add dual-coin wallet architecture` and push `master` to
   `origin`.
7. Return the commit SHA, remote result, final clean status, exact document hash/count,
   and confirmation that no out-of-scope command or file ran.

Do not edit the accepted document or any governance file. Do not use `/tmp`, root,
`sudo`, deletion, cleanup, `rm`, globs, variables as targets, or unresolved paths. Do not
touch a wallet, node, provider, network, hardware device, secret, package, dependency,
workflow, GitHub setting, or another repository. Any guard mismatch is a hard stop before
staging or Git.
