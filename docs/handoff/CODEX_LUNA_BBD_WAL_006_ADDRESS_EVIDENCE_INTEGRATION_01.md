# Codex Luna Handoff — BBD-WAL-006 Address Evidence Integration 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Address Gate 01 and all resumes, stop/correction
reviews, `BBD-WAL-006-ADDRESS-GATE-RESULT-REVIEW-01.md`, Source Review 03, and `CURRENT_TASK.md`.

## Sole task

Do not rerun formatter, Clippy, Rust tests, Node, or any other acceptance command. The retained
results are reviewer-accepted without rerun.

Require `HEAD == origin/master ==` the protected governance parent, clean index, exact six-path
source worktree, the 1,854-line source inventory/hashes from Resume 03, all protected input hashes
from Address Gate 01, and clean source/whole-worktree diff checks. Stop on mismatch.

Create only `docs/testing/BBD-WAL-006-ADDRESS-GATE-01.md` and update
`docs/handoff/CURRENT_TASK.md` to `PHASE-C ADDRESS GATE COMPLETE — REVIEW REQUIRED`. Record:

- timestamp/timezone and execution parent `78a488ff76b59291419e33b0e3fec0ed03425575`;
- `ext2/ext3` and the two exact disk-backed ignored target paths;
- all protected precondition/hash/scope checks;
- formatter 0, warnings-denied Clippy 0/no diagnostics, and exact 8/0 address result;
- exact Node 69/6 partial red, all six names, and the accepted inventory-transition explanation;
- all three safe stops/corrections required by Result Review 01;
- only the observed owned seed buffer is claimed zeroed; upstream derived key memory receives no
  allocator/register/stack/copy erasure claim;
- no network, real wallet/seed, node, device, secret, mainnet, signing, proving, extraction, or
  broadcast; and
- exact final source paths, lines, hashes, integration commit, and repository state.

Do not include a seed, receiver/address, UFVK, canary, user-data path, or raw upstream error.

Recheck hashes, exact scope, and `git diff --check`. Stage explicitly only:

- `wallet-broker/src/lib.rs`
- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/address.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`
- `docs/testing/BBD-WAL-006-ADDRESS-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

Inspect the staged list/diff, commit as `feat: add WAL-006 viewing address foundation`, push
`master`, and prove `HEAD == origin/master`, clean index, and clean tracked worktree. Do not stage
ignored target artifacts. No source/test/policy/fixture/manifest/lock/workflow/package/ticket or
other-repository edit is authorized. Do not clean or delete artifacts. Stop on any mismatch.
