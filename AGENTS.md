# BitBook Desktop Agent Workflow

This file governs agent work in the `bb-desktop` repository.

## Repository Boundary

- This repository owns the Electron client, user interface, and native packaging.
- `../bb-go` owns daemon and protocol behavior. `../go-ipfs` owns the legacy network
  substrate. Cross-repository work requires an explicit baseline, authorized paths,
  validation, and commit in every affected repository.
- BitBook is a barebones distributed social network, not an OpenBazaar marketplace. Do
  not restore inherited marketplace or payment behavior without a separate owner-approved
  architecture ticket.
- The client must not make a centralized search service mandatory when distributed peer
  search is available from the daemon.
- Packaged applications must retain Chromium's sandbox. Development-only workarounds must
  remain clearly labeled and must not become product installation instructions.
- Never commit signing credentials, secrets, private keys, user data, or local absolute
  paths.

## Roles

- **Lead Engineer/Reviewer — Codex:** owns architecture, task contracts, source review,
  independent validation, acceptance or rejection, commits, and pushes. After this
  governance baseline, the reviewer does not author production or test implementation
  for work delegated to the senior developer. The reviewer may author and publish
  governance, architecture, task, and review documents.
- **Sr Dev — Grok Build:** agentic, using Grok 4.6 High. Authors only the production and
  test source bounded by the active ticket. It may run only commands explicitly listed
  in that ticket. It does not edit governance or review records, change architecture,
  use Git, commit, push, access secrets, or widen scope.
- **Owner:** makes product decisions and relays task prompts and completion reports. The
  owner is not the engineering acceptance authority.

Only the reviewer accepts a developer drop or authorizes another implementation task.

## Workflow

1. Read `docs/handoff/CURRENT_TASK.md` and the referenced ticket.
2. Verify the exact source baseline before editing.
3. Modify only the ticket's authorized paths.
4. Run only its explicitly authorized commands.
5. Report changed paths, hashes, line counts, test counts, and exact command results.
6. Stop for reviewer inspection without Git operations.

If `CURRENT_TASK.md` says no implementation is authorized, inspect or discuss only; do
not edit production or test source.
