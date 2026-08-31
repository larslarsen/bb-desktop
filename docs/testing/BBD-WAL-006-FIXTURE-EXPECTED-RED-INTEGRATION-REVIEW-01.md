# BBD-WAL-006 Fixture and Expected-Red Integration Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Reviewed integration commit: `accac4407041f14079211a1e9eeb7047d862922a`

Result: **ACCEPTED — PHASE-C TEST-POLICY TRANSITION AUTHORIZED**

The accepted fixture, tests, and expected-red record are integrated on `master` and
`HEAD == origin/master` at the reviewed commit. The tracked worktree and index are clean.
The evidence is 114 lines with SHA-256
`89570813add0d83c2f988db4a5f4262d086f7e2543233ef10516dd1c15381f17`.

The record proves two deterministic locked/offline fixture runs with four tests passing
each time, exact generated/frozen equality, one closed manifest plus 15 compact-block
files, and the accepted file lengths and SHA-256 values. It also records exactly 66 Node
policy successes and seven Phase-A policy failures. The focused Rust target stopped with
zero tests solely because the production `zec` module was absent (`E0433` for the nested
`zec::test_support` import and `E0432` for the direct `zec` import). Both diagnostics are
the same intended missing-production boundary. No test, fixture, manifest, lockfile,
policy implementation, or existing production source changed during evidence
integration.

## Phase-C architecture decision

The bounded complete ZEC production inventory is:

- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/address.rs`
- `wallet-broker/src/zec/fixture.rs`
- `wallet-broker/src/zec/prepare.rs`
- `wallet-broker/src/zec/scan.rs`
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`

`wallet-broker/src/lib.rs` may expose only `pub mod zec;`. The existing
`scripts/security-policy.js` will eventually enforce the inventory and negative
capability contract. No `zec_network`, raw-PCZT, signer, prover, finalizer, extractor,
broadcast, endpoint, transport, mainnet, Electron, or daemon source path is allowed.

The first independently reviewable production slice will be address issuance and its
real storage/fixture foundation. It will use only `lib.rs`, `zec.rs`, `zec/address.rs`,
`zec/fixture.rs`, `zec/store.rs`, and `zec/test_support.rs`, and it must make the complete
committed `zec_address` target green without weakening the test. The hidden integration
facade may inject deterministic ports, faults, and wipe observers, but it must delegate
derivation, decoding, persistence, and issuance to the same production implementations.
It may not become an alternate wallet state machine.

No manifest promotion is required: all six accepted Zcash crates are already exact,
defaults-off normal dependencies in `[dependencies]`. A newly required direct crate,
feature, lockfile mutation, handwritten compact-block decoder, or wider authority is a
stop requiring separate reviewer action.

## Required test-first transition

The committed Node inventory test still deliberately asserts that the Phase-A ZEC
production inventory is empty. Authoring source while leaving that assertion frozen
would make the policy contract knowingly stale. Before any production byte is
authorized, Sol must change only that test expectation to the exact seven-path inventory
above. Luna must then capture the focused expected red against the still-absent source.

Production source, policy implementation, fixture changes, dependency changes, lockfile
changes, execution, and Git remain unauthorized until that narrow test-source drop and
its red evidence are separately accepted.
