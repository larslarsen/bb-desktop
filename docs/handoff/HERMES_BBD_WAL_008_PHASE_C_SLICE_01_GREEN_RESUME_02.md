# Hermes Handoff — BBD-WAL-008 Slice-01 Green Resume 02

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, the Slice-01 source and format-correction reviews,
Green Stop Review 01, Green 01 handoff, the complete test, and all three source paths.

Frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 253 | `6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2` |
| `wallet-broker/src/zec/hardware.rs` | 864 | `48233ec9ceea26e4b8ac499c00ee5d8c00ca546f2e036dfd16d1deb59d565285` |
| `wallet-broker/src/zec/test_support.rs` | 2,385 | `7d73c8ce55073198a345a71eadf2ac47589665dc1580dc8f485ab3d0f38a5a55` |

Preflight records Hermes identity/provider/model, branch, exact parent at HEAD/origin,
clean index, only these three worktree paths, exact hashes, unchanged lockfile, and
clean `git diff --check`. Stop on any mismatch.

Submit each command byte-for-byte, alone, once, sequentially:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 and no mutation. Then patch only
`reviewed.contains(flag) && live.contains(flag)` to the deliberate falsification
`reviewed.contains(flag) || live.contains(flag)` in `zec/hardware.rs` and run:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware live_claims_cannot_expand_reviewed_booleans_pools_routes_or_fields -- --exact
```

Require exit 101 with exactly that test failing because live authority expanded.
Regardless of outcome, restore the exact `&&` expression immediately and verify all
three frozen hashes plus `git diff --check`. Stop if failure or restoration differs.

Then run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware -- --skip persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration --skip write_file_sync_directory_sync_and_commit_faults_publish_nothing_and_preserve_prior_bytes --skip invalid_records_and_reopen_drift_fail_closed_without_ready_publication --skip verified_fields_are_intersected_and_every_omission_is_host_trusting --skip success_error_debug_panic_and_persistence_representations_are_redacted
```

Require exit 0 with exactly 12 passed, 0 failed, and 5 filtered out.

On exact success only, create `docs/testing/BBD-WAL-008-SLICE-01-GREEN-01.md`, update
`docs/handoff/CURRENT_TASK.md` to await reviewer acceptance, and record all identities,
commands/exits/counts, falsification/restoration, unchanged lockfile, and scope. Stage
exactly the three source paths and those two records. Commit exactly
`feat: add Zcash hardware capability gate`, push `master`, prove a clean
`HEAD == origin/master`, and stop.

Stop on any mismatch without evidence, integration, rerun, or repair. Do not implement
persistence or run any other test, regression, Clippy, build, product/device/network,
other actor, or WAL-007 command.
