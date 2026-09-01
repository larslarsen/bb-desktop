# BBD-WAL-006 Scan Truth Correction Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `cecbe0f2`

Result: **SOURCE ACCEPTED FOR HERMES EXECUTION**

Sol corrected only `scan.rs`, `store.rs`, and `test_support.rs`. The accepted complete-candidate,
wallet-transaction, recovery, fault, confirmation, and promotion design remains unchanged.

The prior three findings are resolved:

- `tree_root` is now a domain-separated commitment over canonical bytes from the official
  Sapling, Orchard, and Ironwood checkpoint roots. It uses stable public
  `WalletCommitmentTrees`, `root_at_checkpoint_id`, and each root type's `to_bytes`; tip/hash and
  tree-size metadata are not proxies.
- pool classification is deterministically derived from nonzero official transparent, Sapling,
  Orchard migration-required, Ironwood pending, and Ironwood spendable components, with `empty`
  for no official pool value;
- fallible postcommit metrics are `Option`-backed, reset for every valid attempt, and cannot expose
  stale/default zero. A postcommit observation failure does not falsely fail a committed scan, but
  the hidden test accessor fails loudly if the requested observation is unavailable.

Accepted source inventory:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 600 | `708ebba85b215b873bacf580156dace9cd68e3d6ed6feb164719c1ff7c9776ee` |
| `wallet-broker/src/zec/scan.rs` | 1,368 | `6f7ef21d8bd951e071ed6b4454ffad0a27ad334cdd4b4c671d1a11e042406e9e` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,231 | `10f453de6e41de698c60255881715b9211a14a8642ffb59ce307eeddadb3ca6c` |

Total accepted production source: 5,231 lines. `git diff --check` passes. Frozen scan/store/address/
prepare/hygiene tests and fixture manifest retain their accepted hashes. No formatter, compiler,
test, Node, policy, Git, network, fixture, wallet, node, or device command has run against this
drop. Only the active Hermes handoff authorizes execution and integration.
