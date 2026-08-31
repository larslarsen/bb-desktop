# Codex Luna Handoff — BBD-WAL-006 Dependency Correction Gate 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, dependency-resolution reviews 01–05,
AEAD API test-source/red/production-source reviews 02/02/01, accepted evidence 02, and
every current path. Require protected `HEAD == origin/master`, clean index,
`git diff --check`, and exactly ten uncommitted paths: manifest, policy, resolved lock,
corrected `vault.rs`, and six ZEC tests. Require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 759 | `89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The six ZEC tests must remain at the exact line counts and hashes in format-correction
review 01.

Revalidate that `wallet-broker/target` is ignored, non-symlink, disk-backed, and has safe
space. Use only these existing paths for every Cargo command:

```text
TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo
```

Do not generate or update the lockfile, fetch, access the network, or create fixtures.
Run the following commands separately and in order.

First run the focused production-policy progression once:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

It must exit 1 only because the dependency map now matches and frozen
`checkWalletBrokerManifest` rejects the six WAL-006 manifest additions at
`scripts/security-policy.js:1840`. Any map mismatch, other checker result, canary, secret,
or unexpected pass is unintended; stop.

Then run:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features -e features
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --offline --all-features -e features
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --offline --all-features -d
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 metadata --manifest-path wallet-broker/Cargo.toml --locked --offline --format-version 1
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

Formatting, all three trees, and metadata must exit 0 without changing a byte. The final
target must exit 0 with exactly 11 passed, zero failed/ignored/measured/filtered. It must
include the independent Argon2id RFC 9106, HKDF-SHA256, XChaCha20-Poly1305, and
deterministic envelope vectors. No warning, canary, or secret may appear.

Inventory the immutable graph from command output and lock/metadata: exact direct pins,
checksums, feature union, duplicate crypto primitive versions, crates.io-only sources,
licenses, build scripts, and proc macros. Retain all requirements of dependency-resolution
gate Resume 03, including exact `chacha20poly1305 0.10.1`, `aead 0.5.2`,
`chacha20 0.9.1`, `cipher 0.4.4`, stable `crypto-common 0.1.7`, Zcash exact
`crypto-common 0.2.0-rc.1`, both required digest/HMAC/SHA-2 lines, and absence of the
rejected stable versions. Any lock/hash mutation, missing crate, git/path/patched source,
unreviewed feature, network capability, build-script contradiction, or source/test error
stops the slice without evidence, integration, Git, fixture, or repair.

On full success create only
`docs/testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-01.md` with timestamp/timezone,
protected parent, disk boundary, exact commands/statuses, policy expected-red cause,
graph inventories, 11-test result, no-canary result, and pre/post hashes. Update only
`docs/handoff/CURRENT_TASK.md` to
`DEPENDENCY CORRECTION GATE PASSED — REVIEW REQUIRED` and link the evidence.

Run `git diff --check`; stage exactly these six paths with an explicit list:

- `scripts/security-policy.js`
- `wallet-broker/Cargo.toml`
- `wallet-broker/Cargo.lock`
- `wallet-broker/src/vault.rs`
- `docs/testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-01.md`
- `docs/handoff/CURRENT_TASK.md`

Inspect staged names/diff. Commit `fix: resolve WAL-006 dependency graph` and push.
Require `HEAD == origin/master`, a clean index, and exactly the six accepted untracked ZEC
tests remaining. Do not amend, clean, generate fixtures, run any ZEC target/full Node/npm/
security suite, edit another path, build/package Electron, access a wallet/node/device or
live endpoint, or touch `../bb-go`/`../go-ipfs`. Stop and report exact results.
