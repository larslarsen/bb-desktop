# Hermes Handoff — BBD-WAL-007 Expected Red Resume 04

State: AUTHORIZED — NOT STARTED

You are Jr Dev — Hermes. Rerun only the corrected Node policy, complete the preserved
expected-red evidence, and integrate the accepted Phase-A source.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-05.md`

Preserved evidence identities:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md` — 78 lines — SHA-256
  `d321f924f3ed817eb8112b0e503319e949cf01dae2bbb9009b75f6c230159899`;
- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — 88 lines — SHA-256
  `b319588f01e91c17d0574b28d7d7e737f75b8b350d56e588411f4f5356d3bab7`.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`, Review 05, both
evidence records, prior expected-red handoffs, this handoff, and
`docs/handoff/CURRENT_TASK.md`.

## Authorized paths

Do not edit the eleven Review 05 source paths, resolved lockfile, or first evidence
record. You may integrate their exact hashes.

You may edit only:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — preserve every prior stop/result and
  append the final Node result and completed evidence;
- this handoff — state line only; and
- `docs/handoff/CURRENT_TASK.md` — leading task-state/actor/handoff block only.

Every other path and repository is read-only.

## Preconditions and preserved results

Record Hermes version/provider/model, `HEAD`, `origin/master`, full status/index, every
Review 05 hash/line count, both evidence identities, resolved `Cargo.lock` SHA-256
`29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`,
and `git diff --check`. Stop on any mismatch, staged entry, or divergent HEAD.

Preserve and accept the Resume 03 results because no Rust source, manifest, or lock byte
changed afterward:

- Cargo formatting exited zero without mutation;
- `native_surface` exited 101 only on absent `XmrInstallationSelectionPort` and
  `XmrSelectionController`;
- each of `xmr_distribution`, `xmr_process`, `xmr_rpc`, `xmr_account`, `xmr_receiver`,
  and `xmr_hygiene` exited 101 only because `bitbook_wallet_broker::xmr` is absent; and
- the resolved dependency identity and four-part lock delta remain accepted.

Do not rerun Cargo, Rust, formatting, dependency inspection, or any other prior command.

## Node policy and integration

Run exactly:

```text
node test/securityPolicy.node.js
```

It must exit zero with exactly 86 `ok` cases and final line:

```text
BitBook security policy tests passed (86).
```

Any failure, different count, source mutation, or extra output indicating execution of
a real boundary is a stop. Do not run `xmr_local_gate`, Monero binaries, a wallet/node,
Electron, npm, browser, scanner, a full suite, network, or any other test/build/runtime
command.

On exact green, update `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` without erasing any
prior stop. Record the final Node command/exit/count, the preserved formatting and seven
expected-red Rust results, exact source/lock identities, path audit, and proof no wallet
production or real Monero boundary ran.

Set this handoff state to `State: COMPLETE`. Update only the leading current-task block
to `PHASE B EXPECTED RED COMPLETE — REVIEW REQUIRED`, with no authorized actor and this
completed handoff. Run `git diff --check`.

Stage exactly the eleven Review 05 source paths, resolved lockfile, both evidence
records, this handoff, and `CURRENT_TASK.md`. Commit exactly
`test: reserve BBD-WAL-007 Monero adapter`, push `master`, and prove
`HEAD == origin/master` with a clean worktree/index. Do not amend/rebase/merge/force-push
or authorize production.

On any stop, do not commit or push. Preserve accepted bytes, record only within the
authorized evidence/state paths, and return control to the reviewer.
