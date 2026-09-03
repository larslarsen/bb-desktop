# BBD-WAL-008 Phase-A Expected Red 01

Jr Dev — Hermes execution evidence for the Phase-B expected-red gate under
[HERMES_BBD_WAL_008_PHASE_B_EXPECTED_RED_02.md](../../handoff/HERMES_BBD_WAL_008_PHASE_B_EXPECTED_RED_02.md).

## Preflight record

- Hermes Agent v0.18.2 (2026.7.7.2), upstream `63279301`, local `10b6d1a9`
  (+1 carried commit).
- Provider: `nous`. Model: `meituan/longcat-2.0:free`.
- Branch: `master`.
- `HEAD == origin/master == 21cff7810ae763460b3ec251f3a08dbcb47f2aaf`.
- Clean governance index; `git diff --check` clean.
- Single worktree at `/home/lars/OpenBazaar/bb-desktop` on `master`; no other
  worktree paths.
- Worktree changes limited to the authorized manifest and untracked test file.
- `Cargo.lock` unchanged.

Frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/tests/zec_hardware.rs` | 794 | `32959949c9da01834fe10ab1328777ab906fb9f8c7bc3e8ef66945f6961ad7a7` |

## Exact commands and outcomes

1. `/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check`
   - Exit: `0`. No mutation.

2. `/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware`
   - Exit: `101`. No test executed.

## Normalized absent-contract diagnostics

The only diagnostics are the expected absent production/test-support contract:

- `error: couldn't read tests/../src/zec/hardware.rs: No such file or directory`
  — production-inventory assertion at `zec_hardware.rs:751` references the
  intentionally absent `src/zec/hardware.rs`.
- `error[E0432]: unresolved imports ... zec::test_support::{...}` — 16 items
  (`CapabilityFlag`, `ClaimedRoute`, `DeviceFingerprint`, `DeviceVendor`,
  `FingerprintField`, `HardwareCanaries`, `HardwareCanarySlot`,
  `HardwareStateRoot`, `HardwareStoreFault`, `HardwareTestHarness`, `LiveProbe`,
  `PersistedDecisionMutation`, `ProbeMutation`, `ReviewedProfile`, `SigningPool`,
  `VerifiedField`) not found in the absent `zec::test_support` module.
- `error[E0425]: cannot find type HardwareDecision in module zec::test_support`.

No dependency, lockfile, syntax, type error in otherwise self-contained test
source, existing-source error, network attempt, or other cause is present.

## Prohibited-action confirmation

- No production or existing-test source, fixture, `Cargo.lock`, Node, Electron,
  policy, workflow, package, or any other path was edited or executed.
- No Clippy, build, audit, scanner, product binary, network operation, another
  actor, or the Monero gate was run.
- No repair, rerun, evidence edit, staging, commit, push, or later command
  followed the expected-red outcome.
