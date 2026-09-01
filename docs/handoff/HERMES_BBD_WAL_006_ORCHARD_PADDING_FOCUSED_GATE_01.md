# Hermes Handoff — BBD-WAL-006 Orchard Padding Focused Gate 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Run one focused gate and
make no file or Git mutation.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Verify `HEAD == origin/master`, exactly the eight dirty paths from Prepare Rollback Gate 01, clean
`git diff --check`, and ext4 Cargo work directories. All Gate-01 identities remain exact except:

- `wallet-broker/src/zec/store.rs`: 2,095 lines,
  SHA-256 `918cd4d946273b5676c558701f89e69fc2b9bf95fba46c05ea2a5dd21f2749bd`.

Record Hermes version/provider/model. Stop on mismatch.

Run exactly once from the repository root, without a pipe, output filter, redirection, or wrapper:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact --nocapture
```

Expected: exit 0, one passed and zero failed, including the byte-exact SQLite rollback assertion.
Report the exact result. Do not run a second command, edit evidence or `CURRENT_TASK.md`, stage,
commit, push, clean, delete, or use network/wallet/node/device access.
