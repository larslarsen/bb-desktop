# Codex Sol Handoff — BBD-WAL-006 Prepare Secret-Bytes Tests 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable handoff is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, Prepare
Production Source Stop Review 01, Prepare Secret-Bytes Design Review 01, the complete current
`wallet-broker/src/vault.rs`, complete `wallet-broker/tests/secret_hygiene.rs`, the concurrent
test in `wallet-broker/tests/zec_address.rs`, and `docs/handoff/CURRENT_TASK.md`.

## Sole test-source task

Edit only `wallet-broker/tests/secret_hygiene.rs`, starting at exactly 274 lines and SHA-256
`3f809e06e96add88a91c232b7824531ddaaf320182e79d9e51cf3c6b61e42323`.

Add one focused regression with a private generic helper equivalent to:

```rust
fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn secret_bytes_can_be_owned_by_synchronized_account_state() {
    assert_send_sync::<SecretBytes>();
}
```

The assertion must be compile-time and non-vacuous. Do not assert a wrapper, reference, pointer,
mock, channel, closure, or a different type. Keep every existing test byte and assertion unchanged.
Do not edit production, ZEC tests/source, Cargo files/lock, policy, docs, fixtures, or another path.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run Cargo, Rust, tests,
formatter, Clippy, Node, policy, dependency, Git, network, wallet/node/device, cleanup, or deletion.
Do not stage, commit, or push.

Return the sole changed path with line count/SHA-256 and the exact added assertion. Hermes will
integrate the test-only drop and run the focused expected-red command. Production remains frozen.
