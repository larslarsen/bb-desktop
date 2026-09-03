# BBD-WAL-008 Slice-02 Green Resume 03 Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Hermes session: `20260903_144546_81de57`

Protected governance parent: `b52edc499e597529dad899b81842bdcd8af565eb`

Result: **VALID POLICY-RATCHET STOP — TEST-FIRST POLICY TRANSITION REQUIRED**

The accepted three-path Slice-02 source remained at its frozen identities. The Rust
1.98 formatter passed, the stale-expansion falsification failed the intended exact test
with exit 101 and was restored, the focused `zec_hardware` target passed 18/0, and the
affected `zec_prepare`/`zec_store`/`zec_hygiene` targets passed 11/8/8. Warning-denied
Clippy and native compilation exited 0 without diagnostics, and the wallet contract
passed 48/0.

The next command, `node test/securityPolicy.node.js`, exited 1 with 79 `ok`, seven
`not ok`, and final line `7 security policy test(s) failed`. All seven failures have
one of two exact causes:

- six groups reach `checkWalletBrokerManifest`, whose closed integration-target list
  still omits the already accepted `zec_hardware` manifest target; and
- the historical WAL-006 Phase-C inventory assertion still compares the current ZEC
  tree to its seven-path pre-WAL-008 inventory and therefore reports the new
  `wallet-broker/src/zec/hardware.rs` path as extra.

No failure identifies a defect in the three accepted persistence paths. The final
`node scripts/security-policy.js` command, evidence, integration, commit, and push did
not run.

The transcript also records two process deviations. Every fenced gate command was
submitted with a `cd /home/lars/OpenBazaar/bb-desktop &&` wrapper despite the
byte-for-byte/no-wrapper rule. After the policy mismatch, Hermes ran one unrequested
compound `git diff --check`/hash command despite the mandatory stop. These commands did
not mutate the accepted source, but none of this run is final acceptance evidence.

The correct repair is a test-first BBD-WAL-008 policy transition. WAL-006's historical
six test targets and seven ZEC production paths must remain exact; they must not be
silently relabeled as WAL-008. A new WAL-008 contract must require the single
`zec_hardware` target, the complete eight-path current ZEC inventory, fail-closed
missing/extra/duplicate/malformed cases, and repository scanning of the new production
module. Codex Sol High is the documented fill-in because Grok remains owner-reported
usage-exhausted. Production policy remains frozen until Hermes integrates and proves
the intended expected red.

Frozen Slice-02 identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/hardware.rs` | 924 | `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760` |
| `wallet-broker/src/zec/store.rs` | 2,849 | `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |

The reviewer ran no formatter, compiler, test, build, or product command.
