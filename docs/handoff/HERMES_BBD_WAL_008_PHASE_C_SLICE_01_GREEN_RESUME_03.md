# Hermes Handoff — BBD-WAL-008 Slice-01 Green Resume 03

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, Slice-01 Source Review 01, Format-Correction Source
Review 01, Resume-02 Rejection 01, the complete test, and all three source paths.

Frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 253 | `6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2` |
| `wallet-broker/src/zec/hardware.rs` | 864 | `48233ec9ceea26e4b8ac499c00ee5d8c00ca546f2e036dfd16d1deb59d565285` |
| `wallet-broker/src/zec/test_support.rs` | 2,385 | `7d73c8ce55073198a345a71eadf2ac47589665dc1580dc8f485ab3d0f38a5a55` |

Preflight records Hermes identity/provider/model, exact HEAD/origin parent, clean index,
only these three worktree paths, exact hashes, unchanged lockfile, and clean whitespace.
Stop on mismatch.

The exact Resume-02 AND-to-OR falsification and restoration are reviewer-accepted; do
not repeat or mutate source. Submit each command below byte-for-byte, alone, once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Require exit 0 and no mutation.

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware -- --skip persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration --skip write_file_sync_directory_sync_and_commit_faults_publish_nothing_and_preserve_prior_bytes --skip invalid_records_and_reopen_drift_fail_closed_without_ready_publication --skip verified_fields_are_intersected_and_every_omission_is_host_trusting --skip success_error_debug_panic_and_persistence_representations_are_redacted
```

Require exit 0 with exactly 13 passed, 0 failed, and 5 filtered out.

On exact success only, create `docs/testing/BBD-WAL-008-SLICE-01-GREEN-01.md` and
update `docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. Record the accepted
Resume-02 falsification/restoration separately from these two fresh commands, actual
identities/counts, unchanged lockfile, and scope. Stage exactly the three source paths
and those two records. Commit exactly `feat: add Zcash hardware capability gate`, push
`master`, prove clean `HEAD == origin/master`, and stop.

Stop on any mismatch without evidence, integration, rerun, repair, or extra command.
Do not run another test/regression/Clippy/build, implement persistence, invoke a product,
device, network, other actor, or WAL-007 command.
