# BBD-WAL-006 Prepare Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `74474da4`

Result: **REJECTED — BOUNDED PRODUCTION CORRECTION REQUIRED; HERMES PAUSED**

Principal Dev — Codex Sol delivered exactly the four authorized production paths and did not
execute or use Git. The real upstream proposal/PCZT call graph is directionally correct: the drop
uses the Ironwood-only spend policy, ZIP-317 change strategy, v6 proposal, no input lock,
authority-free PCZT creation/redaction/serialization/parsing, positive-value action inspection,
and parsed memo plaintext. The frozen tests, fixture, custody source, Cargo graph, and every other
path remain unchanged. `git diff --check` is clean.

The source is not accepted for execution. Static review found seven bounded defects. At least one
would fail the frozen suite, and the remaining items violate explicit lifecycle, authority, or
bounded-memory requirements.

## Reviewed drop

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/prepare.rs` | 900 | `72f1922b9bb5398f35ed26c7a33aae1f90ed0ddbd62811d4220fe6dbb5b9529b` |
| `wallet-broker/src/zec.rs` | 254 | `72612217b6543bb356541e1d470405375fdcffdd0b2b3ce6006617da82b6e78e` |
| `wallet-broker/src/zec/store.rs` | 2,051 | `0baa6f587dd7dc210463e0104f17f8d81e95e4ad1671bb5abf5c78a3abe41e62` |
| `wallet-broker/src/zec/test_support.rs` | 1,807 | `86537047ff4153ee87ed6a7ea6f3ed45ff983dd98ca14abc6af0a99ba979d678` |

Frozen identities remain:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

## Required corrections

1. `parse_canonical_positive_u64` maps decimal overflow to `LIMIT`. The frozen typed prepare row
   for `18446744073709551616` permits only `SCHEMA` or `NETWORK_DISABLED`; decimal syntax that is
   outside the closed u64 domain must return `SCHEMA` before spend access.
2. The three composite-receiver fixture helpers append `-p2pkh`, `-sapling`, or `-unknown` to a
   valid UA. Those are malformed strings, not valid ZIP-316 addresses, so they do not prove the
   required composition rejection. Use real valid vectors and re-encode them for the fixture's
   local network.
3. `address::decode_unified_address` currently maps `address.has_transparent()` to
   `DecodedReceiver::Unknown`; therefore the prepare branch matching `P2pkh | P2sh` is dead and a
   real Orchard+P2PKH UA cannot return `TRANSPARENT_DOWNGRADE`. Within `prepare.rs`, decode against
   the account's exact parameters, require `Address::Unified`, check `has_transparent()` first,
   then require exactly Orchard with no Sapling or unknown items. Do not broaden this correction
   into the already accepted address vertical.
4. Errors from official scan/inventory parsing, required-value arithmetic, inventory outcome,
   fee-bound rejection, and handle generation can return after spend access without applying the
   `operation-error` lifecycle. Once spend access begins, every returned error must explicitly
   wipe any newly built raw PCZT, invalidate all pre-existing handles, destroy derived spend
   material, and clear the session. Validation/binding/capacity failures that occur before spend
   access retain their existing pre-access behavior.
5. The synthetic panic helper proves only manual invalidation before a synthetic panic. It does
   not protect a real panic from inventory, wallet, proposal, PCZT, inspection, or handle work.
   The production prepare operation must catch/guard the entire post-access region, invalidate
   with `panic-unwind`, and resume the original unwind only after handles and derived material are
   wiped.
6. A viewing-only `PrepareState` can currently call `unlock` and establish a session. Reject that
   call with `WATCH_ONLY` before deriving or retaining spend/session state; viewing-only reopen
   must never become prepare-authorized through the policy-session seam.
7. The new bridge allocates four vectors as large as the complete wallet/cache databases through
   `fs::read`. Replace this with the existing validated read-only connection and
   `WalletDb::from_connection(&mut connection, ...)`, matching the accepted scan pattern. The
   SQLite read-only flag must make a write impossible; the frozen external test remains the exact
   byte-equality oracle. Also cap the new prepare wipe-observation collection with a small ring
   large enough for one maximum-handle invalidation; it may not grow without bound across
   repeated sessions.

The standard proposal fee remains authoritative: the actual `step.balance().fee_required()` is
the returned and bounded fee. The hidden inventory seam may use the frozen 10,000-zatoshi outcome
threshold only to select its frozen table result; it may not replace proposal construction or
become the public fee oracle.

## Independent review note

A bounded Grok CLI second-opinion attempt was made for the UA correction. Grok created an interim
session but returned no verdict; its advertised session could not be restored. No Grok conclusion
is relied on here. The reviewer independently verified `zcash_keys 0.16.1`:
`Address::Unified`, `UnifiedAddress::{has_orchard, has_sapling, has_transparent, unknown}`, and
`Address::{decode, encode}` provide the required no-dependency correction. When Sapling support is
disabled, a decoded Sapling receiver is retained as an unknown typecode and survives re-encoding,
which is sufficient to construct a real protocol-incompatible test address.

No formatter, compiler, test, Clippy, Node, policy, wallet, fixture, network, or Git integration
command is authorized for this source. Hermes remains paused until the corrected four-path source
drop passes reviewer inspection.
