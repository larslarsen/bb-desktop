# Codex Sol Handoff — BBD-WAL-006 Store Format Correction 02

You are **Principal Dev — Codex Sol**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Store Production Source Review 02,
Store Format Correction Review 01, Store Gate Formatter Review 02, and
`docs/handoff/CURRENT_TASK.md`.

## Sole task

Apply only the three exact rustfmt replacements below to
`wallet-broker/src/zec/store.rs`, using `apply_patch`. Do not edit another path. Do not run a
formatter, compiler, Clippy, test, policy, dependency, Git, or network command. Do not stage,
commit, or push. Do not make a semantic change, import change, rename, warning fix, or cleanup.

Require the starting file to be 1,687 lines with SHA-256
`534e118c4bb34bf9b27d8342bde4da7f3acca255cb440714790f4994c47a6ad4`. The other three source
paths are frozen at these identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 280 | `01c4fa4b83b0b5f4501db966c27e07c2d3f7d88148daac1865bc35e67044c698` |
| `wallet-broker/src/zec/test_support.rs` | 825 | `e37f486b7dceab7d35fc1d0385ca17d82d5082e77732edeaca727e8903592346` |

## Exact replacements

1. Inline the `File::open` assignment; wrap only the following `file.read` chain:

```rust
            let mut file = fs::File::open(&fault.path).map_err(|_| ZecError::state_corrupt())?;
            let mut marker = [0; 64];
            let length = file
                .read(&mut marker)
                .map_err(|_| ZecError::state_corrupt())?;
```

2. Put the opening brace after the binding condition on its own line and inline the receiver
validation chain:

```rust
    ) || ufvk.is_empty()
    {
        return Err(ZecError::state_corrupt());
    }
    address::derive_orchard_receiver(network, &ufvk, 0).map_err(|_| ZecError::state_corrupt())?;
```

3. Wrap the checkpoint conversion after `=` and inline the complete bound condition:

```rust
    let checkpoint_sequence =
        u64::try_from(checkpoint_sequence).map_err(|_| ZecError::state_corrupt())?;
    if scan_tip.is_some_and(|value| value < 0) || checkpoint_sequence > issued_receiver_sequence {
        return Err(ZecError::state_corrupt());
    }
```

Return the resulting `store.rs` line count and SHA-256, confirm all three frozen identities remain
exact, enumerate the replacements, and confirm no ambiguity remains. The reviewer will inspect
and decide whether Hermes may restart the gate.
