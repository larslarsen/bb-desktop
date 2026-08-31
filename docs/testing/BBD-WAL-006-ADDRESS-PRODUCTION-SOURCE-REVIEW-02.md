# BBD-WAL-006 Address Production Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `33e457d2`

Result: **CORRECTION REQUIRED — EXECUTION NOT AUTHORIZED**

The resumed six-path production drop is present and remains uncommitted. Read-only source,
diff, cached-upstream, line-count, and hash inspection found no unauthorized path change and
`git diff --check` is clean. The drop uses the accepted upstream derivation, wallet migration,
compact-cache initialization, and direct SQLite APIs, but it is not accepted for Luna execution.

## Reviewed inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 11 | `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617` |
| `wallet-broker/src/zec.rs` | 206 | `8465119f5b726ebd7793e2cf95539bd4d04106735b446be13dd55ab557c57b3e` |
| `wallet-broker/src/zec/address.rs` | 206 | `16ebba57e1503bc8fecbc8727c676a19ff944633e254137de31744901a97fdce` |
| `wallet-broker/src/zec/fixture.rs` | 183 | `7d2cf5075fa024d6d618ea1d45b3847be5059cdc971279c267d28433c45d6849` |
| `wallet-broker/src/zec/store.rs` | 688 | `739a7aa7fc20d3299e39b97e217bbc6f11e9eedd36f8550b128f9dc467bbc471` |
| `wallet-broker/src/zec/test_support.rs` | 378 | `9be9d676b5764ace0814786fdb7cc7fcb782bb365d21937a7d472f0efd69a3cc` |

Total: 1,672 lines.

## Blocking findings

1. `LocalNetwork` derives `Hash` from all three heights but its manual `PartialEq` returns
   `true` for every pair. Equal values can therefore produce different hashes, violating the
   Rust `Eq`/`Hash` contract. The schedule type must use exact field equality. The outer
   `Network` may retain discriminator equality because a `uregtest` Unified Address encodes the
   regtest discriminator but cannot recover activation heights; that narrower behavior must be
   explicit and must not replace exact schedule comparison or SQLite binding checks.

2. The broker extension is not yet closed strongly enough for the accepted corruption boundary.
   `extension_objects` sees only object names beginning `ext_bitbook_`, while column-only PRAGMA
   checks can accept an unreviewed trigger, index, view, changed table SQL, or changed constraints
   attached under another object name. In addition, receiver issuance rechecks only the stored
   network discriminator and does not recheck the stored local activation tuple in its immediate
   transaction. Reopen performs a fuller check, but mutation must not rely on an earlier check.

3. Full ancestor and companion-file validation occurs at bootstrap/reopen, but later receiver
   issuance, inspection, and test-state mutation call `open_connection` with only a final-file
   check. Replacing an account/network ancestor with a symlink after open can therefore redirect a
   later path resolution outside the reviewed state root. The fixed account paths must remain
   available and the complete root/network/account/file chain must be revalidated before each
   later database operation. No caller-provided path or follow-on dependency is permitted.

4. The closed manifest model declares multiple fields that the address slice never reads. The
   final accepted gate includes Clippy with warnings denied, so the correction must make this
   intentional at source level: structurally validate the declared frozen-manifest fields where
   useful, or use only narrowly scoped, reasoned dead-code allowances for fields retained solely
   to enforce the closed serde shape. A module/crate-wide lint suppression is not accepted.

## Preserved design

The correction must preserve the sound parts of the drop: exact account parsing and schedule
mapping, mainnet rejection before effects, account-zero USK-to-UFVK derivation, Orchard-only
`find_address`, observed seed wiping, official wallet/cache initialization, `BEGIN IMMEDIATE`
coupled state updates, rollback fault ports, viewing-only reopen, private modes, bounded closed
fixture parsing, and the thin hidden integration facade. No test, fixture, dependency, lock,
policy, documentation outside the reviewer handoff, scan/preparation source, or broader feature
work is authorized.

No Cargo, Rust, formatter, Node, test, build, policy, wallet, network, or acceptance command was
executed by the reviewer. Luna remains unauthorized until the corrected source passes a new
review.
