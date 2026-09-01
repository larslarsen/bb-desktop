# Codex Sol Handoff — BBD-WAL-006 Scan Runtime Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Scan
Atomicity Review 01, Scan Truth Correction Review 01, Scan Gate Runtime Review 01, Scan Runtime
Design Review 01, the complete frozen `zec_scan` test, `scan.rs`, and the pinned upstream sources
named by those reviews.

## Authorized paths and starting identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/src/zec/scan.rs` | 1,400 | `17d411e4af9e64d1169d6326ae17f4ae13e1283236d690c80984971507dedab9` |

Author the test-source correction first, then production source, using `apply_patch`. Every other
source, test, fixture, manifest, lockfile, policy, document, workflow, package, and repository
path is frozen. In particular require these identities to remain exact:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/store.rs` | 1,816 | `d0b97b802209489804e5d550ef90138a87342e155c519a686b2d900189f6603f` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

Require `wallet-broker/src/zec/prepare.rs` to remain absent.

## Test-source correction

At `zec_scan.rs:302`, replace only the first loop binding `(value, accepted)` with `(value, _)`.
Do not change its two values, loop body, assertions, second below/at/above loop, test count, or any
other test token.

## Production correction

Implement every fixed invariant in Scan Runtime Design Review 01. Specifically:

1. Change `official_tree_digest` to compute Sapling, Orchard, and Ironwood current roots with
   `root_at_checkpoint_depth(None)`. Remove its unused height parameter and update the sole call.
   Preserve the actual `Result<Option<_>>` shape and all fail-closed checks: one missing-root check
   for Sapling and Orchard, and the existing outer optional-tree plus inner missing-root checks for
   Ironwood. Empty trees resolve through upstream to `Some(empty_root)`.
2. Refactor the non-override inspection path so wallet tip, current roots, official
   `get_wallet_summary`, and the read-only orphan query are evaluated in one
   `transactionally_with_extension` snapshot. Do not write through either handle. Preserve UFVK,
   account-purpose, one-account, receiver-sequence, recovery, and balance-override behavior.
3. Query pinned views/tables read-only and in one aggregate row. Match upstream summary inclusion
   exactly for each same-account shielded orphan:
   - `v_received_outputs` pool 2/3/4 joined to its creating `transactions` row;
   - creating transaction `mined_height IS NULL`;
   - creating transaction unexpired at the checked `target_height` by exact branches
     `expiry_height = 0`, `expiry_height >= target_height`, or unknown expiry with
     `min_observed_height + DEFAULT_TX_EXPIRY_DELTA >= target_height`;
   - `value > zip317::MARGINAL_FEE`;
   - no matching `v_received_output_spends` row whose spend transaction satisfies the complete
     pinned unexpired predicate, including its mined-before-target branch.
   Bind the official account UUID and import the upstream constants; add no BitBook copy of an
   upstream numeric constant. Detect any same-account unmined transparent received row and fail
   closed. Reject negative/unknown/unconvertible aggregate results.
4. Adjust only display totals according to the review invariants. Official summary spendable
   values remain authoritative and unchanged. Check pending capacity, every subtraction/addition,
   per-pool/account reconciliation, and Ironwood pending derivation. Continue deriving pool
   classification from the adjusted display components. Return `STATE_CORRUPT` on any mismatch.

Do not mutate wallet-owned tables, delete or reclassify retained transactions, weaken recovery,
change the scan transaction, special-case fixture values, add a lint allowance, or change an
error contract. Do not run rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a compiler, a
linter, Git, network, fixture, wallet, node, device, cleanup, or deletion command. Do not stage,
commit, or push.

After editing, use only read-only file inspection, `wc -l`, and `sha256sum`. Return both changed
path line counts and hashes, re-prove all four frozen source identities and absent `prepare.rs`,
enumerate helper/signature/import/SQL/invariant changes, and confirm no ambiguity or broader edit
remains. The reviewer will inspect and decide whether Hermes may restart the gate.
