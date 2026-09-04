# BBD-WAL-009 — Zcash PCZT Authorization, Verification, and Durable Send Pipeline

Status: PHASE A1 FORMAT CORRECTION AUTHORIZED — SPARK HIGH ONLY

Reviewer: Lead Engineer/Reviewer — Codex at High

Completed test-source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High).
Grok Build remains the default senior route, but the owner reports its weekly usage
exhausted; this is the documented fill-in condition.

Authorized formatting actor: Implementation Dev — Codex Spark, High, under the exact
one-path mechanical handoff.

Planned integration actor: Jr Dev — Hermes

Source baseline: `97d407e6a71ac8446933d5e69530b3566abdd42a`

Dependencies: reviewer-accepted BBD-WAL-002 intent contract, BBD-WAL-004 custody and
native authority, BBD-WAL-006 Zcash fixture/scan/prepare adapter, and BBD-WAL-008
hardware capability gate. BBD-WAL-007/Monero is parked and is not a dependency for the
Zcash-first phases of this ticket.

## Objective and sequencing

Implement the Zcash side of the native send pipeline in independently reviewed slices:

1. authorize a prepared v6/Ironwood PCZT through either unlocked software custody or a
   synthetic test-only external-signature contribution;
2. prove, finalize, extract, and independently verify the signed transaction against
   the confirmed intent before it can become `verified`;
3. add durable intent states, per-account concurrency, cancellation/expiry barriers,
   and crash-recovery reconfirmation;
4. add internal testnet submission and confirmation behind an injected transport; and
5. run a separate live testnet gate only after the owner resolves the default Zcash
   endpoint and compact-block IP-privacy/Tor policy.

Phase A1 authorizes test source for item 1 plus the item-2 verification boundary only.
It does not authorize production source or execution. Later phases require separate
reviewer handoffs. XMR prepare/sign/verify and cross-coin integration remain deferred
while the owner's Monero node syncs.

## Security invariant

Signer success is never payment success. Only a broker-native confirmation bound to an
exact prepared handle and `intent_hash` may authorize signing. A signed artifact becomes
`verified` only after independent decoding and exact comparison of every authoritative
effect. Cancellation, expiry, capability, device, session, and account ownership are
rechecked after signing. No raw PCZT, transaction, key, seed, memo, receiver internals,
or signature crosses the broker's public/status boundary.

## Fixed Phase-A1 contract

### Authority and bindings

- Input is an existing opaque prepared handle. Callers cannot supply or replace raw
  PCZT bytes.
- The handle remains exactly bound to account, network, unlock session, request ID,
  intent hash, receiver, amount, exact fee and bound, memo hash, and expiry.
- A confirmation capability is minted only by the broker-native authority path. It is
  one-shot, non-serializable, non-cloneable, process-local, and bound to the same handle
  and review hash. Electron, supervisor protocol, HTTP, strings such as
  `intent.confirm`, and reconstructed values cannot mint or replay it.
- Confirmation is rejected unless the prepared review already contains the exact fee
  and fee bound. A mismatch fails before seed, signer, prover, or transaction access.
- One account may own at most one prepare/sign/verify operation. A second operation for
  the same account returns `ACCOUNT_BUSY`; another account remains independent.

### Software authorization

- Software signing requires a currently unlocked session and derives the NU6.3-era
  unified spending authority only inside broker memory from the confirmed account's
  custody material.
- The derived full viewing key must exactly match the stored account binding before the
  PCZT is touched. Wrong seed/account/session/network fails closed.
- The signer may authorize only the single real Ironwood spend in the accepted fixture.
  It cannot add an Orchard, Sapling, or transparent spend/output, replace a destination,
  change an amount or fee, or sign another account's action.
- Proof generation, spend authorization, finalization, and extraction use the pinned
  librustzcash/PCZT stack and transaction v6 branch `37a5165b`. No custom transaction or
  signature codec is allowed.

### Synthetic hardware contribution

- The production reviewed hardware table remains empty. Production hardware signing
  must therefore return `CAPABILITY_MISSING` without exporting a PCZT.
- Tests may inject only BBD-WAL-008's unmistakably synthetic Keystone-like exact
  profile. Its route is `keystone_pczt_v2`; no Trezor, Ledger, vendor-name shortcut,
  fallback signer, or PCZT-v1 path is accepted.
- The broker retains the authoritative PCZT. A synthetic external signer receives only
  the reviewed v2 signer/batch view and returns tagged Ironwood spend-authorization
  contributions. The broker applies them to the retained authoritative PCZT and
  cryptographically verifies pool, action index, randomized key, and signature.
- Missing, duplicate, extra, wrong-pool, wrong-index, invalid, reordered, replayed, or
  cross-intent contributions fail closed. A device-returned PCZT is never accepted as
  the new authoritative transaction merely because it parses.

### Independent post-sign verification

Before publishing `verified`, decode the authoritative signed artifact independently
and compare it to the frozen prepared binding and confirmed review:

- network `zec-local`/testnet discriminator and no mainnet;
- transaction version `6` and consensus branch `37a5165b`;
- exact external receiver bytes and exact amount;
- exact fee and `fee <= fee_bound`, with no caller fee substitution;
- memo SHA-256, request ID, and intent-hash binding;
- Ironwood-only real spend/output effects, with expected shielded internal change;
- required proof and signatures present and valid; and
- extracted transaction ID derived from the verified transaction, not signer metadata.

Every one-field mutation returns `INTENT_MISMATCH`, invalid authorization material
returns `SIGNATURE_INVALID`, and malformed state returns `STATE_CORRUPT` or `SCHEMA`
according to the existing precedence. No failure publishes a transaction handle or
invokes broadcast.

### Cancellation, expiry, cleanup, and public result

- Cancellation/request status and the injected clock are read before signing and again
  after signing/proving immediately before `verified`. At the exact expiry instant,
  expiry wins. A cancelled or expired result destroys the pending authorization and
  releases the account lock.
- Seed, unified spending authority, derived authorizing keys, signer view, signature
  contributions, authoritative PCZT, proof workspace, and extracted transaction bytes
  are wiped or dropped through observed secret wrappers on success, error, cancel,
  expiry, lock, panic unwind, account replacement, and broker exit.
- The public result contains only an opaque verified handle, transaction ID, stable
  state/code, account/request identifiers already allowed by the broker status model,
  and boolean `broadcastable=false`. It exposes no raw artifact or secret-derived data.
- Phase A1 has no network client, endpoint, socket, gRPC, HTTP, submit, confirmation
  polling, retry, broadcast, Electron method, filesystem export, mainnet, or XMR action.

## Required Phase-A1 tests

The new Rust target must contain non-vacuous tests covering at least:

1. an unlocked fixture account signs/proves/finalizes/extracts one v6 Ironwood PCZT and
   an independent decoded inspection matches every frozen authoritative field;
2. the public verified result is bounded/redacted, uses a real derived txid, and remains
   non-broadcastable;
3. native-only, one-shot confirmation rejects Electron/protocol/HTTP/string/replay and
   every handle/session/account/request/intent/review mismatch before secret access;
4. wrong seed/full-viewing-key binding, locked, watch-only, wrong-network, mainnet, and
   stale-session paths fail before signing or proof generation;
5. production hardware has no positive route and exports zero bytes;
6. the exact synthetic Keystone v2 route returns only tagged Ironwood signature
   contributions that are applied to the retained authoritative PCZT;
7. missing, duplicate, extra, invalid, wrong-pool, wrong-index, replayed, and
   cross-intent external contributions all fail with zero verified/broadcast outputs;
8. receiver, amount, network, fee, fee bound, memo, request, intent, pool, version,
   branch, change, proof, signature, and transaction-ID mutations independently fail;
9. cancel and expiry are re-read after signing/proving; immediately-before, exact, and
   immediately-after expiry boundaries are covered;
10. same-account concurrent authorization is `ACCOUNT_BUSY`, while another account is
    not blocked and every terminal/error/panic exit releases the lock;
11. injected signer, prover, finalizer, extractor, verifier, and cleanup failures return
    stable closed errors without a verified result;
12. success, each failure class, cancellation, expiry, lock, panic, replacement, and
    broker exit produce positive wipe observations for every sensitive class touched;
13. seed/key/PCZT/transaction/signature/receiver/memo canaries are absent from `Debug`,
    display, errors, diagnostics, panic text, persisted bytes, and public JSON; and
14. operation/capability inventories prove zero broadcast/network/mainnet/XMR authority
    and preserve the empty positive production hardware table.

Tests must exercise the real accepted fixture and typed production-facing test harness.
Source-text assertions are allowed only for production inventory/forbidden-authority
checks. Behavioral outcomes cannot be proved by self-reported flags or a fake decoded
transaction shaped by the implementation under test.

## Phase A1 test-source authorization (completed)

Codex Sol may edit only:

- `wallet-broker/Cargo.toml` — add one explicit `zec_sign_verify` test target; and
- `wallet-broker/tests/zec_sign_verify.rs` — new test source.

No dependency, feature, lockfile, fixture, production source, Node/Electron source,
policy, workflow, documentation, evidence, Git, network, wallet process, device, or
other path may change. Sol may use read-only inspection and must stop with exact
path/line/SHA-256 reporting. Sol runs no formatter, test, build, lint, audit, scanner,
product, Git, network, node, hardware, or actor command.

## Expected red, falsification, and future gates

After reviewer source acceptance, Hermes will first run:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify
```

The expected red is unresolved new `zec::test_support` sign/verify contract and absent
production `zec/spend.rs`, never a syntax, formatting, dependency, fixture, or unrelated
failure. Formatter mismatch stops before the red test.

Future green must falsify at least native-origin enforcement, one post-sign receiver
mutation, the after-sign cancellation barrier, and tagged hardware contribution
pool/index verification, restoring the exact source after each intended failure. Green
also includes the focused target; all Zcash tests; native-surface, vault, and wallet
contract regressions; warning-denied Clippy; native compilation; security policy;
dependency audits; secret scans; and GitHub CI. Exact commands are frozen only in later
Hermes handoffs.

## Reserved future production paths

Separate reviewed slices may authorize only the minimum needed subset of:

- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/prepare.rs`
- `wallet-broker/src/zec/spend.rs` (new)
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`
- `wallet-broker/src/native.rs`
- `wallet-broker/src/native_ui.rs`
- `wallet-broker/tests/native_surface.rs`
- `test/securityPolicy.node.js`
- `scripts/security-policy.js`

Listing does not authorize an edit. Raw network submission and confirmation transport,
real hardware I/O, default endpoints, Tor policy, XMR, Electron wiring, packaging, and
mainnet remain outside Phase A1.

## Acceptance boundary

Completing Phase A1/C1 proves local cryptographic authorization and independent
post-sign verification on the synthetic Zcash fixture. It does not move funds or prove
live testnet behavior. Completing the whole ticket still cannot enable mainnet, an
Electron broadcast method, auto-broadcast after crash, or a real hardware device
without separate positive capability and transport evidence.
