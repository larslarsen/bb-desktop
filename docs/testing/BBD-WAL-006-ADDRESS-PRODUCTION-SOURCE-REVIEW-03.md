# BBD-WAL-006 Address Production Source Review 03

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `a9bd4a4a`

Result: **SOURCE ACCEPTED — EXECUTION GATE AUTHORIZED**

Sol completed the bounded three-path correction without changing any frozen path. Read-only
inspection confirms that all four Source Review 02 findings are corrected and the complete
six-path address vertical remains within the original production contract.

## Accepted inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 11 | `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617` |
| `wallet-broker/src/zec.rs` | 203 | `c86c030245e3caaec5182e4138f199a5bab08223c5c95ecb25b87745bbfa5e80` |
| `wallet-broker/src/zec/address.rs` | 206 | `16ebba57e1503bc8fecbc8727c676a19ff944633e254137de31744901a97fdce` |
| `wallet-broker/src/zec/fixture.rs` | 257 | `6c3a5368617dc0039c6d1da970a489f9e4fb4f4235bc39b32866fd085a33a715` |
| `wallet-broker/src/zec/store.rs` | 802 | `946fd7531bd34bfc2ac411d35d582c0333375c0f712de8847fb729a8bf6d8fc6` |
| `wallet-broker/src/zec/test_support.rs` | 378 | `9be9d676b5764ace0814786fdb7cc7fcb782bb365d21937a7d472f0efd69a3cc` |

Total: 1,857 lines. Source-only `git diff --check` is clean.

## Correction acceptance

- `LocalNetwork` now derives exact field equality and hash. The outer `Network` documents its
  discriminator-only comparison for hidden `uregtest` decoding, while every persisted local
  binding independently checks birthday, NU6.3, and confirmation heights.
- Extension discovery closes over objects named with, attached to, or SQL-referencing the
  reserved namespace. Its exact inventory is three reviewed tables plus their SQLite primary-key
  autoindexes. Normalized table SQL and independent column/nullability/key checks reject changed
  definitions, triggers, views, and extra indexes.
- Receiver issuance validates schema, exact row cardinality, account, network, activation tuple,
  nonempty UFVK, and state inside the same `BEGIN IMMEDIATE` transaction before derivation or
  mutation. The coupled test mutation receives the same protection.
- The production account owner retains its fixed paths. Every later inspection or mutation
  revalidates root, network directory, account directory, wallet database, and compact cache with
  exact modes and symlink/type rejection.
- The closed fixture model structurally consumes every declared generator, network, expected,
  file, block-metadata, label, and scenario field. It adds no lint suppression.

The remaining design is accepted for execution: official account-zero USK/UFVK derivation,
Orchard-only address search and encoding, observed seed wipe paths, official wallet/cache
initialization, durable monotonic issuance, viewing-only reopen, rollback fault injection,
mainnet rejection before effects, bounded fixture parsing, and the thin hidden test facade.

No Cargo, Rust, formatter, Node, test, build, linter, policy, wallet, network, or acceptance
command was executed by the reviewer. Acceptance is source-only; Luna must stop on any formatter,
compile, lint, test, expected-policy-red, scope, or hash mismatch.
