# Codex Luna Handoff — BBD-WAL-006 AEAD API Expected Red 02

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, dependency-resolution review 04, and
AEAD API test-source review 02. Require protected `HEAD == origin/master`, clean index,
`git diff --check`, and exactly ten uncommitted paths: manifest, policy, lockfile,
corrected `vault_crypto`, and six ZEC tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/src/vault.rs` | 760 | `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |
| `wallet-broker/tests/zec_fixture_builder.rs` | 890 | `efb104bedeaf48f5e3a0850f84a6b504651bad2267eb3fc4a443864ae2fd3c81` |
| `wallet-broker/tests/zec_address.rs` | 277 | `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3` |
| `wallet-broker/tests/zec_store.rs` | 334 | `492e4e6934f8cd9589de22cc338fd5e93131f3f3d3fcca5f79b44455b297e1ca` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

Revalidate that `/home/lars/OpenBazaar/bb-desktop/wallet-broker/target` is disk-backed,
and use only the already populated `wal006-cargo` target and `wal006-tmp` temp paths
under it. Do not resolve, fetch, generate, or mutate the lockfile.

Run exactly once:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

Expected exit is 101 during library compilation with zero executed tests. Every compiler
error must arise only from these frozen production constructs in
`wallet-broker/src/vault.rs`:

- unresolved import `AeadInOut` at line 5;
- `XNonce::try_from` at lines 482 and 538;
- `Tag::try_from` at line 539;
- missing `encrypt_inout_detached` at line 484; and
- missing `decrypt_inout_detached` at line 541.

The concrete Rust error set may include `E0432`, `E0277`, and `E0599` plus their compiler
help/note context, but must name no source path other than frozen production
`wallet-broker/src/vault.rs`. The corrected test must produce no API error. Any manifest,
policy, lock, ZEC-test, dependency, build-script, linker, runtime, warning-as-error,
unexpected pass, executed test, canary, secret, or other source-path failure is unintended;
stop without evidence or Git.

Run no other Cargo/Rust/Node/npm/build/scanner/resolution/network/fixture/wallet/node/
device command and edit no source/test/manifest/policy/lock/fixture path.

On exact red create only
`docs/testing/BBD-WAL-006-AEAD-API-CORRECTION-EXPECTED-RED-02.md` with timestamp/timezone,
parent, exact command/status, complete error-code/site inventory, zero-test result,
no-canary result, and pre/post hashes. Update only `CURRENT_TASK.md` to
`AEAD API EXPECTED RED 02 RECORDED — REVIEW REQUIRED` and link the evidence.

Run `git diff --check`; stage only corrected `vault_crypto`, evidence, and
`CURRENT_TASK.md`; inspect. Commit `test: record WAL-006 AEAD API correction red two` and
push. Leave exactly nine hash-exact uncommitted paths: manifest, policy, lock, and six ZEC
tests. Require final `HEAD == origin/master`, clean index, and report exact results.
Stop before production correction.
