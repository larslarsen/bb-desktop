# BBD-WAL-006 Scan Format Capture Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `1c74f0af29447f0d5f462bba7cede68c8c6393ad`

Result: **CAPTURE ACCEPTED — EXACT SOL FORMAT CORRECTION AUTHORIZED**

Hermes v0.18.2 (`nous`, `meituan/longcat-2.0:free`) re-proved the protected parent,
clean index, exact five-path source worktree, absent `prepare.rs`, ext4 storage, and the two
disk-backed target paths. It then ran the sole authorized Rust 1.98.0 formatter check exactly
once. The command exited 1, wrote 577 lines to stdout and zero lines to stderr, and did not mutate
source or tracked state.

The reviewer read the complete retained stdout capture. Its SHA-256 is
`261480b0560df8ddbfb1a71d33c068fffdca2b31fd4c9d58801cbe061ee444a3`; empty stderr is
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. The output contains
exactly 34 rustfmt hunks: five in `fixture.rs`, 22 in `scan.rs`, and seven in
`test_support.rs`. Every hunk changes whitespace and line wrapping only. No semantic token,
identifier, type, literal, import, SQL statement, visibility, or control-flow decision changes.
`zec.rs` and `store.rs` have no formatter hunk and remain frozen.

Hermes's prose summary misstated those per-file hunk counts as 6/26/6 and did not reproduce the
capture verbatim. Those narrative defects are superseded by the exact locally retained artifact
and this reviewer count; they do not authorize broader execution or correction. The source actor
must consume the captured artifact itself, not the Hermes summary.

## Protected source inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 600 | `708ebba85b215b873bacf580156dace9cd68e3d6ed6feb164719c1ff7c9776ee` |
| `wallet-broker/src/zec/scan.rs` | 1,368 | `6f7ef21d8bd951e071ed6b4454ffad0a27ad334cdd4b4c671d1a11e042406e9e` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,231 | `10f453de6e41de698c60255881715b9211a14a8642ffb59ce307eeddadb3ca6c` |

Total: 5,231 lines.

This is formatter-capture acceptance, not source or runtime acceptance. Sol may apply only the
captured mechanical replacements. Hermes may not resume the scan gate until the reviewer accepts
the resulting source identities and publishes a new gate handoff.
