# Hermes Handoff — BBD-WAL-006 Prepare Stage Diagnostic 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Own only one diagnostic
test execution. Do not modify any file or use Git beyond read-only precondition checks.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Targeted Review 01, Prepare Stage
Diagnostic Source Review 01, this handoff, the complete temporary `store.rs`, the frozen
`zec_prepare.rs` test, and `CURRENT_TASK.md`.

Verify free Hermes version/provider/model, `HEAD == origin/master`, exact seven-path inventory,
temporary `store.rs` at 2,060 lines and SHA-256
`a3c36fcb920a1cf2e5c228a75ef3a0f87cffa4dcfdb191b13e20ba9e32c71852`, all other accepted
Resume-06 identities, clean `git diff --check`, and ext4 Cargo work directories. Stop on mismatch.

Run exactly once from repository root:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact --nocapture
```

Expected: exit 101, exactly one failed and zero passed, and exactly one of the four fixed diagnostic
markers appears. Record which marker and the public `INTERNAL` result. Any compilation/setup
failure, no marker, multiple markers, different public error, file/lock mutation, warning, or
network attempt is a stop.

Do not run a second command, formatter, Clippy, another test, Node, npm, audit, scanner, dependency
resolution, wallet/node/device, or network client. Do not create evidence, edit `CURRENT_TASK.md`,
stage, commit, push, clean, or delete. Report the exact result and leave the temporary source intact
for reviewer-supervised restoration.
