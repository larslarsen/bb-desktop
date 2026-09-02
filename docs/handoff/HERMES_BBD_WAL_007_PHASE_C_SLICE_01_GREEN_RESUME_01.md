# Hermes Handoff — BBD-WAL-007 Phase-C Slice 1 Green Resume 01

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/engineering/HERMES_JR_DEV_ROUTING.md`,
`tickets/BBD-WAL-007.md`, `docs/testing/BBD-WAL-007-PHASE-B-ACCEPTANCE-01.md`, both
Slice-1 source reviews, Slice-1 Green Stop Review 01, the original Slice-1 Green 01
handoff, the complete provisional stop evidence, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and precedence

Resume Slice-1 Green 01 with the one mechanical formatter mutation below, then restart
the original gate from its formatter check. This handoff replaces the original
prohibition on formatting only for the exact `cargo fmt` mutation and postconditions
below. Every other protected identity, command, order, count, stop rule, evidence field,
Git scope, commit message, push requirement, and closed boundary from Green 01 remains
mandatory.

You may not hand-edit source or tests, repair logic, change a command, begin Slice 2, or
run the real local-Monero gate. Stop on the first mismatch or failure; do not improvise.

## Resume preconditions

Require `HEAD == origin/master ==` the protected governance parent, a clean index,
`git diff --check` clean, and exactly the original nine accepted source/policy worktree
paths plus this provisional untracked record:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md` | 73 | `55a5361079eafe32bfe6d6d07d5bbaf68ad9d8ccaf886939cf99d2b39c6cad60` |

Re-prove all nine accepted identities and all four frozen Phase-A identities from Green
01. In particular, before formatting require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/distribution.rs` | 910 | `f5e3b43c11f1a4a1b0389738b9729621fe80ea6f87bb5216640c47f213903ebb` |
| `wallet-broker/src/xmr/test_support.rs` | 370 | `e2fb6496ddb731ad60753c169a31958f692be235bd0bf7b1a6c5e000872ad722` |

Record the actual resolved identity again and preserve the three command outputs
separately as version, provider, and model:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

## Mechanical formatting authorization

Run exactly once from the repository root:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml
```

This is a mechanical integration exception, not source-authoring authority. Require exit
0. Compare every protected hash immediately afterward. Only
`wallet-broker/src/xmr/distribution.rs` and
`wallet-broker/src/xmr/test_support.rs` may have changed bytes, line counts, or hashes;
every other accepted source/policy path, manifest, lockfile, and test must retain its
Green 01 identity. Require `git diff --check` clean. If any other file changes, stop
without running another command or staging anything.

Record the two post-format identities and the exact formatter-induced diff. Do not make
any manual edit.

## Restarted exact green gate

Only after the mechanical postconditions pass, run once each from the repository root,
in this order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Apply the exact six acceptance requirements from Green 01: format exit 0 without further
mutation; 17 native tests; 12 distribution tests; native-feature check exit 0 without
warning; 86 Node policy tests; and the repository-policy success line. Run no other
command, test, build, binary, package manager, security tool, network operation, or real
gate.

## Evidence corrections and exact-success integration

On exact success, update the existing provisional
`docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md`; do not create a second green evidence
file and do not erase the first-attempt formatting stop. Correct these two transcription
errors in its frozen Phase-A table:

- `wallet-broker/Cargo.lock` SHA-256 is
  `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`;
- `wallet-broker/tests/xmr_distribution.rs` SHA-256 is
  `481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8`.

Record provider and model separately from their actual commands. Append the mechanical
format command, the two post-format identities, the restarted gate results, exact test
counts, no-further-mutation proof, and final source/policy hashes. Change the evidence
state to complete while preserving the stop chronology. Do not describe the original
handoff as contradictory; it correctly stopped an unformatted source drop, and this
resume adds the explicit mechanical exception.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 1 GREEN COMPLETE — REVIEW REQUIRED`, link the completed evidence, and
retain the ticket, architecture decision, routing, and prior-ticket records.

Stage explicitly only the final nine accepted source/policy paths, the completed Green
01 evidence, and `CURRENT_TASK.md`. Inspect the staged names and diff. Commit exactly:

```text
feat: add BBD-WAL-007 Monero distribution boundary
```

Push `master`, then prove `HEAD == origin/master`, clean index, and clean tracked and
untracked worktree. Stop for reviewer acceptance. Every other path, task, and repository
remains unauthorized.
