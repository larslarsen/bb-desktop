# Codex Sol Handoff — BBD-WAL-007 Phase A Test Source

Status: AUTHORIZED — TEST SOURCE ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at High

Product/test source baseline: `657074b20b2a9233ae9247a2437ba237c7f99b73`

Ticket: `../../tickets/BBD-WAL-007.md`

## Objective

Author the complete test-first contract for BBD-WAL-007. Cover the frozen Linux release
pin and native selection boundary, exact wallet-RPC process plan/lifecycle, bounded HTTP
Digest and JSON-RPC transport, local-node/bootstrap policy, software/watch-only custody
and recovery, viewing state, durable fresh subaddresses, secret hygiene, and the
feature-gated real offline local-Monero gate.

This drop must contain no production implementation and must not be executed by Sol.

## Authorized paths

Sol may create or edit only:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/native_surface.rs`
- `wallet-broker/tests/xmr_distribution.rs`
- `wallet-broker/tests/xmr_process.rs`
- `wallet-broker/tests/xmr_rpc.rs`
- `wallet-broker/tests/xmr_account.rs`
- `wallet-broker/tests/xmr_receiver.rs`
- `wallet-broker/tests/xmr_hygiene.rs`
- `wallet-broker/tests/xmr_local_gate.rs`
- `test/securityPolicy.node.js`

Every other path is read-only.

## Frozen manifest delta

`wallet-broker/Cargo.toml` may add only:

```toml
xmr-local-gate = []
md-5 = { version = "=0.11.0", default-features = false, features = ["zeroize"] }
```

and explicit test targets for the seven new XMR tests. The `xmr_local_gate` target must
have `required-features = ["xmr-local-gate"]`. Do not change package identity, Rust
version, default/native UI features, an existing dependency, or an existing test target.
Do not edit `Cargo.lock`.

## Required test-source shape

- Tests import the future `bitbook_wallet_broker::xmr` production surface and fail for
  its absence after lock resolution. They may not pass through a test-local substitute.
- Fakes must record exact typed calls, bytes, state transitions, persistence, cleanup,
  and process ordering. A fake RPC endpoint fails immediately on an unlisted method.
- Boundary tables include immediately below, at, and above every ticket limit. Failure
  tests assert non-vacuous state and the absence of returned addresses, secrets, child
  processes, handles, or alternate endpoints.
- `native_surface.rs` proves only the new native-only installation-selection authority;
  every nonnative origin is rejected before a dialog/path/hash/process action.
- `test/securityPolicy.node.js` must replace the current empty XMR production expectation
  with the exact Phase-C inventory while continuing to require that the inventory is
  absent during Phase A. It freezes the single direct dependency, local-gate feature,
  forbidden generic/spend/mainnet/remote capabilities, and no Electron/Node inventory
  expansion. Source-text checks do not replace behavior tests.
- `xmr_local_gate.rs` is complete test source but cannot execute in ordinary tests. It
  accepts only `BITBOOK_MONERO_TEST_ROOT`, verifies the ticket's exact reviewed inner
  binary hashes, uses an exact disk-backed scratch path supplied later by the Hermes
  handoff, launches test-owned stagenet `monerod` with offline/loopback/random-port
  flags, and never touches a running user node, existing wallet, mainnet, or public
  network. It emits no path, port, address, password, mnemonic, raw RPC, or wallet data.
- No ignored tests, conditional success, placeholder assertions, compile-error stubs,
  commented future tests, source copies of production algorithms, real user secrets,
  mainnet wallet material, or machine-specific paths are allowed.

The exact semantics, release values, RPC allowlist, secret framing, SQLite binding,
teardown order, errors, and required test groups are frozen by the ticket. Stop and
report a contradiction rather than inventing or widening them.

## Prohibited actions

Sol must not run tests, Cargo resolution/metadata, formatters, builds, binaries, package
managers, security tools, network, Git, or GitHub. It must not edit production source,
`Cargo.lock`, fixtures, policy implementation, documentation, evidence, workflows,
packages, Electron/Node production, ZEC/rate source, or create scratch state.

## Delivery

Stop after the test-source drop and report:

- exact changed paths;
- line count and SHA-256 for each changed file;
- named test count per Rust/Node file;
- which required test group each file proves;
- any frozen contract contradiction; and
- confirmation that no command prohibited above was run.

Do not stage, commit, push, or ask Hermes to execute. Reviewer acceptance is required
before any integration or production authorization.
