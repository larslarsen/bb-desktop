# Hermes Handoff — BBD-WAL-008 Phase-C Slice-01 Green 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Slice-01 Source Review 01,
the complete test, and the three source paths.

## Frozen source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 253 | `6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2` |
| `wallet-broker/src/zec/hardware.rs` | 868 | `590199f7ced6ca7389d8536e9a453ff082e1769a4f0b0ae9907d7d1d2c394aaf` |
| `wallet-broker/src/zec/test_support.rs` | 2,385 | `f8778937c22eeabcc5257c2e6458b20433b936c1b323cc4435ddde64f8e50697` |

Preflight records Hermes identity/provider/model, branch, exact
`HEAD == origin/master`, clean index, only these three worktree paths, exact hashes,
unchanged lockfile, and clean `git diff --check`. Stop on any mismatch.

## Exact gate

Submit every fenced command byte-for-byte, alone, once, sequentially, with no wrapper,
redirection, pipeline, or appended shell text.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 and no mutation. Otherwise stop without evidence or integration.

Then use an exact patch to replace only this expression in
`wallet-broker/src/zec/hardware.rs`:

```text
reviewed.contains(flag) && live.contains(flag)
```

with the deliberate falsification:

```text
reviewed.contains(flag) || live.contains(flag)
```

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware live_claims_cannot_expand_reviewed_booleans_pools_routes_or_fields -- --exact
```

Require exit 101 with exactly that test failing because live authority expanded.
Regardless of outcome, immediately restore the exact `&&` expression, then confirm all
three frozen hashes and `git diff --check`. If the failure or restoration is not exact,
stop without another command.

Run the partial green exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware -- --skip persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration --skip write_file_sync_directory_sync_and_commit_faults_publish_nothing_and_preserve_prior_bytes --skip invalid_records_and_reopen_drift_fail_closed_without_ready_publication --skip verified_fields_are_intersected_and_every_omission_is_host_trusting --skip success_error_debug_panic_and_persistence_representations_are_redacted
```

Require exit 0 with 12 passed, 0 failed, and 5 filtered out. Stop on warnings, another
count, lock/source mutation, unexpected diagnostics, or any other mismatch.

## Integration

Only after every exact outcome, create
`docs/testing/BBD-WAL-008-SLICE-01-GREEN-01.md` and update
`docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. Record actual identities,
commands/exits/counts, falsification and restoration, unchanged lockfile, and scope.
Stage exactly the three source paths plus those two records. Commit exactly
`feat: add Zcash hardware capability gate`, push `master`, prove clean
`HEAD == origin/master`, and stop.

Do not implement persistence, repair source, run another test/regression/Clippy/build,
edit another path, run a product/device/network command, invoke another actor, or touch
WAL-007. On any stop, do not create evidence, integrate, commit, push, or rerun.
