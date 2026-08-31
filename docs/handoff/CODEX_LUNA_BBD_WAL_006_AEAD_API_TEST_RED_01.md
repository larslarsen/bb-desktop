# Codex Luna Handoff — BBD-WAL-006 AEAD API Expected Red 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, resolution review 04, test-source review,
and complete test/production/manifest/policy/lock paths. Require protected
`HEAD == origin/master`, clean index, `git diff --check`, and exactly ten uncommitted
paths: manifest, policy, lockfile, corrected `vault_crypto`, and six ZEC tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `9bc3fe14718cd5ecdec5da3e8416f46356b88e28bf2f56db9ee9407cd0f71b6d` |
| `wallet-broker/src/vault.rs` | 760 | `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

Six ZEC test hashes remain those in the format-correction review. Revalidate the exact
disk-backed target/temp boundary from the resolution gate.

Run exactly once:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

Expected exit is 101 during library compilation with only `E0432` unresolved import
`chacha20poly1305::aead::AeadInOut` at `wallet-broker/src/vault.rs:5`; zero tests execute.
The corrected test must produce no separate API error. Any different compiler error,
warning-as-error, link/runtime result, executed test, unexpected pass, canary, or secret
is unintended red; stop.

Run no other Cargo/Rust/Node/npm/build/scanner/resolution/network/fixture/wallet/node/
device command and edit no source/test/manifest/policy/lock/fixture path.

On exact red create only
`docs/testing/BBD-WAL-006-AEAD-API-CORRECTION-EXPECTED-RED-01.md` with timestamp/timezone,
parent, exact command/status/error/site, zero-test result, no-canary result, and pre/post
hashes. Update only `CURRENT_TASK.md` to
`AEAD API EXPECTED RED RECORDED — REVIEW REQUIRED` and link evidence.

Run `git diff --check`; stage only corrected `vault_crypto`, evidence, and
`CURRENT_TASK.md`; inspect. Commit `test: record WAL-006 AEAD API correction red` and
push. Leave exactly nine hash-exact uncommitted paths: manifest, policy, lock, and six ZEC
tests. Require final `HEAD == origin/master`, clean index, and report exact results.
Stop before production correction.
