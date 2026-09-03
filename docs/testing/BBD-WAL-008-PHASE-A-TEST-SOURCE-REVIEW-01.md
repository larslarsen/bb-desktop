# BBD-WAL-008 Phase-A Test-Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Protected governance parent: `2dfdb92d23fca4166336fb9a6ece164b516865af`

Reviewed drop:

| Path | Lines | SHA-256 | Decision |
| --- | ---: | --- | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` | accepted test-target-only edit |
| `wallet-broker/tests/zec_hardware.rs` | 614 | `ec3037616e71263a379c36f35c5b2689eb91fa98f68e5c368d16815a0cbe4755` | correction required |

Result: **TEST SOURCE NOT YET ACCEPTED**

The drop has the exact two-path scope, the manifest adds only the named test target,
and no dependency, feature, production path, or lockfile changed. The test structure is
substantial and correctly covers the empty production table, exact synthetic Keystone
route, reviewed/live reduction, protocol-pin mismatch, Trezor/Ledger negatives,
persistence faults, input bounds, and production inventory. Four oracle corrections
are required before expected red:

1. `assert_no_private_spend_authority` also rejects `can_receive_private`. Device
   disconnection must deny device spend/route authority, but this hardware boundary may
   not decide whether already broker-held viewing material can still view or derive a
   receiver. Remove view/receive assertions from the shared spend-only helper. Tests
   that require a wholly empty decision must assert that separately.
2. The exact positive case does not explicitly prove all forbidden authority remains
   false. Assert at least no broadcast, transparent/Orchard signing, migration,
   software fallback, other-device fallback, artifact input/output, mutation, proof,
   finalization, extraction, or signing call.
3. The redaction test can pass if `install_observable_canaries_for_test` is a no-op. It
   must prove every distinct canary was actually installed/touched in its intended
   sensitive input before asserting absence from outputs. Scan persisted bytes as
   bytes; do not require the SQLite-backed representation to be valid UTF-8.
4. The contract requires durable narrowing. Add a sequence that persists a narrowed
   decision, reopens it without silent re-expansion, rejects restoration without a
   fresh exact reviewed/live decision, and permits restoration only after that explicit
   fresh match. Include the existing Zcash store's directory-sync fault alongside
   write/file-sync/commit coverage.

Also pin and test the allowed fingerprint alphabet (`A-Z`, `a-z`, `0-9`, `.`, `_`,
`+`, `-`) rather than leaving "separator" behavior implicit.

The Sol transcript includes an unauthorized read-only Git baseline check. It did not
mutate Git or widen the source drop, so it does not invalidate the accepted manifest
edit, but it is a process deviation and must not recur. No test, formatter, build, or
acceptance result exists or is authorized.
