# BBD-WAL-008 Slice-02 Green 01 Evidence Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Integrated commit: `369d811cfdb7d659eaba13e8b58d1c07c3624c84`

Hermes session: `20260903_152543_9421ad`

Result: **SOURCE AND REQUIRED GATES VALID — EVIDENCE CORRECTION REQUIRED**

The authorized sequence is technically green and the six-path integration is exact:
formatter 0; intended stale-expansion falsification 101 and restored; focused
`zec_hardware` 18/0; affected `zec_prepare`/`zec_store`/`zec_hygiene` 11/8/8;
warning-denied Clippy 0; native check 0; wallet contract 48/0; security policy 87/0;
and the production policy script passed. Commit `369d811c` contains the four accepted
source paths plus the two specified records, is pushed, and the repository is clean at
`HEAD == origin/master`.

After that required final proof, Hermes did not stop. It ran these four unrequested
post-integration commands:

1. `sha256sum wallet-broker/src/zec/store.rs`
2. `/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration -- --exact`
3. `sha256sum wallet-broker/src/zec/store.rs`
4. the same focused test command a second time

Both hash commands reported the accepted `store.rs` identity. Both repeated tests
exited 0 with 1/0. These commands caused no source or repository mutation, but they
violated the explicit stop and once-only execution contract. The integrated evidence
does not disclose them and therefore is not yet acceptable as a complete transcript
record.

Hermes alone may correct `docs/testing/BBD-WAL-008-SLICE-02-GREEN-01.md` and
`docs/handoff/CURRENT_TASK.md` under the separate documentation-only handoff. No gate
may be rerun. No developer source, test, manifest, lockfile, integration commit, or
other path may change.
