# BBD-WAL-007 Phase-D Root Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Repository baseline: `48b5a7565478099eb78e9a112cd2f78fa76cac49`

Result: **REVIEWED ROOT ACCEPTED; PHASE D PARKED**

The owner supplied one extracted Monero GUI `v0.18.5.2` root outside source. Its
canonical root and every member-path component are non-symlinks. The personal root is
intentionally not recorded.

The two required archive members exactly match the ticket pins:

| Member | Bytes | SHA-256 |
| --- | ---: | --- |
| `monero-gui-v0.18.5.2/extras/monero-wallet-rpc` | 29,026,368 | `c1e3aff7c72837e6f29045c439b772a82b5cd7324c8b831fa825a6ce2019a656` |
| `monero-gui-v0.18.5.2/monerod` | 24,112,840 | `9b3b2676ea7868c1a7186feea9569c2cf7683ae79d2fcc769c846a91c810a1f5` |

Both are regular executable files owned by the repository owner. The gate target is on
disk-backed `ext4`, and the exact `wallet-broker/target/wal007-local-gate` scratch leaf
was absent during review.

No Monero binary or acceptance command was executed. The owner elected to move to
Zcash work while a separate user node syncs. The Phase-D gate itself does not depend on
that user node: by contract it launches a fresh test-owned stagenet `monerod --offline`
with ephemeral state and never touches a running user node. The gate remains parked
until the owner asks to resume it and a new Hermes handoff is committed.
