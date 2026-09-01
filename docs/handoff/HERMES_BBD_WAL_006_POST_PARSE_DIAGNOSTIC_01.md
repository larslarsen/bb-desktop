# Hermes Handoff — BBD-WAL-006 Post-Parse Diagnostic 01

Use only free Hermes/Nous. Run one diagnostic command; make no file or Git mutation.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Require `HEAD == origin/master`, exactly the eight dirty paths from Prepare Rollback Gate 01,
clean `git diff --check`, ext4 Cargo work directories, and:

- `wallet-broker/src/zec/store.rs`: 2,126 lines,
  SHA-256 `c27bc3424e25f5a18d4b31cc0fbd5d510395cdbf8929ec151e645f711cb20134`.

Record Hermes version/provider/model. Run exactly once, unfiltered, from the repository root:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact --nocapture
```

Report the one complete `BBD-WAL-006-DIAGNOSTIC:post-parse` line and exact test result. Stop without
a second command, evidence, edit, stage, commit, push, cleanup, deletion, network, or device access.
