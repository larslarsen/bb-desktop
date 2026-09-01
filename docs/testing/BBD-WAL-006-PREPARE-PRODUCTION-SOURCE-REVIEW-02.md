# BBD-WAL-006 Prepare Production Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `99a2116e`

Result: **STATICALLY ACCEPTED — HERMES GATE AUTHORIZED**

Principal Dev — Codex Sol corrected exactly the four authorized production paths and did not
execute or use Git. The corrected drop closes all seven findings from Prepare Production Source
Review 01. The two frozen tests retain their accepted identities, no manifest/lock/fixture/policy
path changed, and `git diff --check` is clean. This is source acceptance for bounded execution;
final integration acceptance still depends on the exact Hermes gate.

## Accepted source inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/prepare.rs` | 966 | `45d2472ef8686331aad4082008d4b0caa24f6e82cd1594bcfd84b09c2a16081c` |
| `wallet-broker/src/zec.rs` | 254 | `72612217b6543bb356541e1d470405375fdcffdd0b2b3ce6006617da82b6e78e` |
| `wallet-broker/src/zec/store.rs` | 2,042 | `b99f901f11b88535831bdd6e79964825ae10caa82ace8ed1a02a3f2983d03701` |
| `wallet-broker/src/zec/test_support.rs` | 1,834 | `bebc6d691f4bbe1416dfeafff06ad3744741714f401cdb2f56c5d39ed03b6974` |

Frozen identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

## Correction acceptance

1. Canonical decimal overflow now closes as `SCHEMA`; persisted inventory parsing remaps any
   invalid stored value to `STATE_CORRUPT`.
2. The three negative receiver helpers now decode pinned, genuine ZIP-316 upstream vectors and
   re-encode their Orchard+P2PKH, Orchard+Sapling, and Orchard+unknown compositions under the
   fixture `LocalNetwork`. No malformed string suffix remains.
3. Prepare directly decodes against the account parameters, requires `Address::Unified`, rejects
   transparent composition first as `TRANSPARENT_DOWNGRADE`, and otherwise permits only Orchard
   with neither Sapling nor unknown receivers. Wrong-network decode closes as `SCHEMA`.
4. Viewing-only unlock now returns `WATCH_ONLY` before digest derivation, session allocation, or
   state mutation.
5. The spend-access counters advance immediately before one guarded operation region. Every
   returned error from that region wipes an unowned newly built PCZT when present, invalidates all
   prior handles with `operation-error`, destroys derived spend state, and clears the session.
6. The complete post-access region is inside `catch_unwind`; the unwind branch applies the same
   cleanup with `panic-unwind` and then resumes the original panic. The hidden panic helper arms
   this production boundary instead of substituting manual test-only invalidation.
7. The store bridge holds the account gate, validates the wallet/cache boundary, opens SQLite
   read-only, and constructs `WalletDb::from_connection(&mut connection, params, SystemClock,
   OsRng)`. It adds no whole-database snapshot, transaction, input lock, schema, or authority.
8. Prepare wipe observations are capped at `MAX_PREPARED_HANDLES + 1` (65), preserving one full
   maximum-handle invalidation plus derived-material observation while discarding the oldest
   observation on overflow.

The accepted path still uses the real Ironwood-only proposal, v6 PCZT creation, redaction,
serialization, parse, and decoded component inspection. The proposal's actual required fee is the
only returned fee and is compared with the caller bound after construction. The hidden inventory
override selects only a frozen outcome row and does not construct the PCZT or provide authority.

## Gate boundary

No formatter, compiler, Clippy, Rust test, Node test, wallet, fixture, network, or mutation command
has been run by the reviewer or Sol for this corrected drop. Jr Dev — Hermes is authorized only by
`docs/handoff/HERMES_BBD_WAL_006_PREPARE_GATE_01.md`. Any source mutation, identity mismatch,
warning, unexpected test count, lock change, or additional policy failure stops integration.

The earlier bounded Grok second-opinion attempt produced no verdict and is not relied upon. The
reviewer's acceptance is based on direct inspection of the corrected source and pinned APIs.
