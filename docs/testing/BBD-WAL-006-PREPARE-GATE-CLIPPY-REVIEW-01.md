# BBD-WAL-006 Prepare Gate Clippy Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected parent: `6624c6a9`

Result: **GATE STOPPED AT COMMAND 2 — TEST-FIRST FEATURE/API CORRECTION REQUIRED**

Jr Dev — Hermes passed command 1: Rust 1.98.0 `cargo fmt --check` exited 0 with no diff. Command 2,
the exact warnings-denied Clippy gate, exited 101 on three compile errors and Hermes stopped without
running commands 3 through 6 or changing/staging/committing/pushing any file. All protected source,
test, Git, diff, and ext4 preconditions passed. Hermes Agent v0.18.2 used provider `nous`, model
`meituan/longcat-2.0:free`.

## Findings

1. `zcash_client_backend::create_pczt_from_proposal` requires
   `<DbT as WalletRead>::AccountId: serde::Serialize`. The accepted read-only wallet is
   `WalletDb<Connection, ...>` whose account ID is `zcash_client_sqlite::AccountUuid`.
   `AccountUuid` derives `Serialize` only when the pinned SQLite crate's narrow `serde` feature is
   enabled. The current direct feature union omits it, and the repository policy currently fixes
   that omission as the only accepted manifest line. A test-first feature/policy correction is
   therefore required; bypassing the official PCZT builder or implementing a parallel wallet
   adapter is rejected.
2. `pczt::orchard::Spend::value` is deliberately not a public getter in `pczt 0.9.3`. The two
   calls in `store.rs` cannot compile. The real selected spend is publicly distinguishable from
   padded dummy spends through `Spend::dummy_sk()`: real spends have no dummy key. Count real
   inputs with that marker. Signature absence must inspect every action's public
   `spend_auth_sig()`, which is stricter than filtering only real inputs.

The direct `zcash_client_sqlite` `serde` feature adds no live-network, signing, proving,
finalizing, extracting, broadcasting, or endpoint authority. It only activates serde support for
the opaque upstream account UUID required by the already accepted official PCZT API. The resolved
lock delta must be captured offline and reviewed before the gate resumes.

The first authorized correction action is test source only. Production source, manifest, lock,
policy implementation, evidence, and execution remain frozen until the new policy expectation is
accepted and captured red.
