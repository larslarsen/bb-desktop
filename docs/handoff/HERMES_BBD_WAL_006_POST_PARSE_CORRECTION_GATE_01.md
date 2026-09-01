# Hermes Handoff — BBD-WAL-006 Post-Parse Correction Gate 01

Use only free Hermes/Nous. Validate the accepted one-path source correction with one focused test
command. Make no source, evidence, or Git mutation.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Require `HEAD == origin/master`, exactly the eight expected dirty ticket paths, clean
`git diff --check`, ext4 Cargo work directories, and:

- `wallet-broker/src/zec/store.rs`: 2,105 lines,
  SHA-256 `5d05ce63a3da21d59ec3493624cd586a6d7de9e37bfaefba2ba91f697efa4ae1`;
- `wallet-broker/tests/zec_prepare.rs`: 416 lines,
  SHA-256 `c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa`.

Record Hermes version/provider/model. Run exactly once, unfiltered, from the repository root:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact --nocapture
```

Report the exact test result and whether any `BBD-WAL-006-DIAGNOSTIC` output appeared. Stop without
a second Cargo command, evidence, edit, stage, commit, push, cleanup, deletion, network, or device
access. A pass authorizes reviewer consideration of the full gate; any other result stops.
