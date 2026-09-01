# BBD-WAL-006 Scan Integration Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Integration commit: `4be931150583876fabadf5a6ffb52021c791fdb3`

Result: **SCAN VERTICAL ACCEPTED — PREPARE DESIGN REVIEW AUTHORIZED**

The pushed integration is exact. `HEAD == origin/master ==` the integration commit, the index and
tracked worktree are clean, and the commit contains exactly eight paths: five production source
paths, the exact one-token `zec_scan` lint correction, Scan Gate Evidence 01, and current task.
The corrected source/test hashes match the accepted inventory and `zec/prepare.rs` remains absent.

## Accepted execution

- Rust 1.98.0 formatter: exit 0 without mutation.
- Locked/offline/no-default library Clippy with warnings denied: exit 0 without diagnostics.
- `zec_scan`: exactly 9 passed and 0 otherwise.
- `zec_store`: exactly 8 passed and 0 otherwise.
- `zec_address`: exactly 8 passed and 0 otherwise.
- Complete 74-test Node policy: expected exit 1, exactly 68 `ok`, exactly 6 `not ok`, the six
  frozen Phase-C failures, and no other finding.

The evidence is 124 lines at SHA-256
`1708fe6ee611bc77a109f7f16d60df0f12b5dac99fa67bceb4a0eb395fbf6e55`. It records Hermes Agent
v0.18.2, provider `nous`, model `meituan/longcat-2.0:free`, exact disk-backed paths, commands,
counts, protected identities, negative capability record, and all 25 proven Rust behaviors.

The accepted vertical provides real offline compact-block scanning, durable/recoverable cache and
wallet transaction coordination, current tree inspection, main-chain-only displayed balances,
confirmation/spendability authority, one-block reorg handling, and fail-closed corruption and
limit behavior. It authorizes no signing, proving, finalizing, extraction, broadcast, live
networking, mainnet, Electron surface, or another repository.

The next BBD-WAL-006 slice is unsigned v6 Ironwood PCZT preparation against the already accepted
11-test `zec_prepare` contract. Because pinned PCZT proposal/builder APIs and compiled authority
boundaries are high-risk, Grok receives a bounded read-only design review before Sol may author
source. Hermes remains the later execution/evidence/Git actor.
