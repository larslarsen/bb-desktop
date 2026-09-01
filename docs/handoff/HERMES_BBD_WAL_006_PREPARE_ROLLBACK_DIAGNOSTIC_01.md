# Hermes Handoff — BBD-WAL-006 Prepare Rollback Diagnostic 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Run one focused diagnostic
and make no file or Git mutation.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Verify `HEAD == origin/master`, the exact eight dirty paths and identities from Prepare Rollback
Gate 01, clean `git diff --check`, and ext4 Cargo work directories. Record Hermes version,
provider, and model. Stop on mismatch.

Run exactly once from the repository root, without a pipe, output filter, redirection, or wrapper:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact --nocapture
```

Report the complete panic line and exact public `ZecError` code, or the exact passing result and
whether the byte-equality assertion was reached. Do not run another command, edit evidence or
`CURRENT_TASK.md`, stage, commit, push, clean, delete, or use network/wallet/node/device access.
