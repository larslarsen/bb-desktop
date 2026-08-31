# BBD-WAL-006 Store Production Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `547a396b`

Result: **SOURCE ACCEPTED — HERMES STORE GATE AUTHORIZED**

Sol corrected only the two authorized source paths. `zec.rs`, `fixture.rs`, the complete
`zec_store` test, `lib.rs`, and `address.rs` retain their protected hashes. The complete
production worktree remains exactly four ZEC source paths, and `git diff --check` passes. No
formatter, compiler, test, policy, Git, or network command was executed by the source actor.

## Accepted source inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 214 | `800111bf587265b49764de71c6d2beb054d1906932005bb3513b2d4fc132d90e` |
| `wallet-broker/src/zec/fixture.rs` | 279 | `5b1e91a730cddd82d0321383ec86f68dd781bf441a4b3e1db7e0514c5b9d5229` |
| `wallet-broker/src/zec/store.rs` | 1,720 | `9da2d00d7ed2fa4d942cf33cd8fbfe9bc28a02dd33174154bd6c2d54b4d81822` |
| `wallet-broker/src/zec/test_support.rs` | 824 | `0b4700eb776b01f9ab8cadfce44a916afe8c7eb01aa0be1f599c453986d097c4` |

Total: 3,037 lines.

## Correction acceptance

- The filesystem-facts seam accepts only a validated account ID and typed hostile kind, then
  internally derives the fixed local wallet path.
- Invalid schedules read from SQLite map to `STATE_CORRUPT`; UFVK decoding/Orchard derivation is
  proven for the exact bound network before viewing state is accepted.
- V1 rejects checkpoint receiver sequence greater than current issued sequence.
- Wallet migration state requires the 71-row/distinct/16-byte aggregate and every published
  current upstream leaf exactly once. Cache state requires exactly one pinned migration ID.
- Writable migration/checkpoint connections configure and verify SQLite `synchronous=FULL` before
  an immediate transaction. Typed faults return before commit and rollback. Success ends at the
  SQLite commit, with no fallible post-commit step that could report failure after advance.
- Bootstrap synchronizes both initialized files and the account/network/root directory chain
  before returning success.
- Hidden SQLite inspection checks per-cell and aggregate cell/byte limits before cloning borrowed
  text/blob bytes.

This is source acceptance, not runtime acceptance. Jr Dev — Hermes is authorized only for Store
Gate 01. The reviewer retains acceptance authority. Scan, PCZT preparation, handle hygiene,
broader policy transition, mainnet, network, signing, proving, extraction, broadcast, Electron,
and other-repository work remain frozen.
