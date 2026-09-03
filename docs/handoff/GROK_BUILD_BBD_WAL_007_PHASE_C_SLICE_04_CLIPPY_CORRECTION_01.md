# Grok Build Handoff — BBD-WAL-007 Slice 4 Clippy Correction 01

Status: AUTHORIZED — SIX-PATH WARNING-ONLY SOURCE CORRECTION

Source actor: Sr Dev — Grok Build using Grok 4.6 High

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`, Green Resume 05 Rejection
01, the complete six editable sources, and `CURRENT_TASK.md`.

Edit only these exact starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/account.rs` | 3,034 | `318ca5ce58f0ced19d974155bdb66f3ecce915f7a600f99138b6f853d72348d8` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/process.rs` | 1,748 | `8b373c6a984608f4689c7d8a210dd68a586d64c8bd470f05c2104641050944a0` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `95b6795969967d608efae322fce17fa81ac805830307170c7c6e69196f5cdf47` |
| `wallet-broker/src/xmr/store.rs` | 1,329 | `19ac8891fb4deaf3cc323bb74647a5490c4684794171c0a262e9378ff51ecaea` |
| `wallet-broker/src/xmr/test_support.rs` | 4,767 | `20ae80415859cafc56ad0a2c80b770c02e2d28e8e4ea9a525ad67720378452ef` |

Freeze every other path, especially `vault.rs`, `xmr.rs`, all tests, governance,
manifests, and lockfiles.

Eliminate exactly the 23 Rust 1.98 Clippy diagnostics without `allow`, expectation
lowering, dummy use, test branch, visibility widening, or behavior change:

- `account.rs`: three `question_mark` at 1050/1071/1088; `collapsible_if` at 1348
  and 2724; `too_many_arguments` at `verify_open_identity`; `redundant_closure` at
  2489; `chunks_exact_to_as_chunks` at 2662.
- `distribution.rs`: `needless_as_bytes` at 197 and `chunks_exact_to_as_chunks` at 412.
- `process.rs`: `too_many_arguments` at `WalletRpcProcessPlan::build` and
  `useless_conversion` at 1220.
- `rpc.rs`: `identity_op` at constant line 29 and `too_many_arguments` at
  `digest_response_for_test`.
- `store.rs`: `redundant_closure` at 216.
- `test_support.rs`: `useless_borrows_in_formatting` at 1611 and 1937;
  `useless_concat` at 1905 and 1908; `manual_repeat_n` at 2231;
  `chunks_exact_to_as_chunks` at 3203; `redundant_closure` at 3471; and
  `needless_return` at 4579.

Use small private typed parameter records for the three over-argument functions and
update only their existing callers. Preserve evaluation order, secret ownership and
zeroization, fail-closed identity checks, process-plan derivation, digest inputs, exact
RPC output, and frozen public test-support APIs. For fixed-size byte chunks, keep the
existing validated-even-input invariant and do not discard a possible remainder on an
unvalidated path. All other fixes should follow the diagnostic's semantics-preserving
form. Keep Rust 1.98 formatting manually.

Do not run rustfmt, Cargo, compiler, tests, Clippy, binaries, Node/npm, network, Git, or
GitHub. Do not edit tests/governance/evidence or invoke another actor. Report changed
paths with line counts/SHA-256, the 23-fix inventory, structural-record design, and
prohibited-action compliance, then stop for reviewer inspection.
