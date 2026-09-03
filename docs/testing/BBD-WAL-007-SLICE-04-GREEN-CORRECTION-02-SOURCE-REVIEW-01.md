# BBD-WAL-007 Slice-4 Green Correction 02 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance parent: `624b6adf`

Result: **ACCEPTED FOR FRESH HERMES EXECUTION — NOT YET GREEN**

Grok 4.6 High changed only the five authorized paths. `git diff --check` is clean,
all reported identities match, and frozen `vault.rs`, `xmr.rs`, `xmr_account.rs`, and
`xmr_hygiene.rs` remain byte-exact.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `318ca5ce58f0ced19d974155bdb66f3ecce915f7a600f99138b6f853d72348d8` |
| `wallet-broker/src/xmr/store.rs` | 1,329 | `19ac8891fb4deaf3cc323bb74647a5490c4684794171c0a262e9378ff51ecaea` |
| `wallet-broker/src/xmr/process.rs` | 1,748 | `8b373c6a984608f4689c7d8a210dd68a586d64c8bd470f05c2104641050944a0` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `95b6795969967d608efae322fce17fa81ac805830307170c7c6e69196f5cdf47` |
| `wallet-broker/src/xmr/test_support.rs` | 4,782 | `e422ed545d8c96127c240e64d899ca536f7bd9a454d5da03ea980a32013cb3b6` |

The review confirms that `RpcSecret` is crate-visible rather than public; unused
forwarders and test-only stored fields were removed without removing live teardown;
and retained identity accessors now participate in SQL binding. No lint suppression
was added.

`restore_height` is now an eight-byte big-endian SQLite BLOB with an exact-length
schema constraint, strict BLOB/type/length decoding, and `STATE_CORRUPT` failure. This
round-trips the ticket's complete `u64` domain without changing receiver bounds.

The new test-support authority and hygiene surface matches every call made by the
frozen nine-test suite. Typed calls traverse the production request allowlist;
mnemonic and mainnet gates are phase/network checked; canary receipts use SHA-256;
diagnostic and Debug surfaces are closed; exit owners use zeroizing secret storage;
teardown exercises the owned process manager; and the isolation path uses the process
coordinator. The support is observable rather than an unconditional-pass constant.

This was a source inspection only. No formatter, compiler, test, Clippy, Node, binary,
network, staging, or source Git action was performed by the reviewer. Fresh Hermes
execution must establish formatting, compilation, warning freedom, falsification,
green tests, policies, and integration. Sol is neither needed nor authorized.
