# Codex Luna Handoff — BBD-WAL-006 Dependency Resolution Gate 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is the complete
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-006.md`, the original and resume
Phase-B handoffs, every BBD-WAL-006 dependency correction review/evidence file, and the
complete accepted manifest, production policy, Node test, `vault_crypto` test, six Rust
tests, architecture review, and current lockfile.

## Preflight and disk boundary

Require `HEAD == origin/master` at the protected governance parent, a clean index, and
exactly eight uncommitted paths: the two accepted correction paths plus six accepted
Rust tests. Require `git diff --check` and these exact hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `da7639af39bbc578936f0d38fcc32317fbacb279e53040cad4b3543c67a54294` |
| `scripts/security-policy.js` | 2,231 | `627eaf04248ab744e8c4300f3cae3a34d114a69bad88dfa4de04492dea537d4d` |
| `test/securityPolicy.node.js` | 2,374 | `f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `26475b2ccddd692b036e5440fdfde66d105f943f6bde912d81391efe7984b76e` |
| `wallet-broker/Cargo.lock` | 3,273 | `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5` |

The six Rust test hashes must match the format-correction review. Stop on an extra path,
staged change, mismatch, or dirty frozen source.

Revalidate that `wallet-broker/target` and the existing exact
`wallet-broker/target/wal006-tmp` and `wallet-broker/target/wal006-cargo` paths are real,
ignored, disk-backed ext4 directories with safe free space. Create only either exact
ignored child if absent. Never use `/tmp`, a symlink-derived path, root/`sudo`, another
cache/target, or cleanup/deletion.

## Focused policy progression

Run the prior focused command exactly once:

```text
node -e 'const name = "WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features"; const matches = require("./test/securityPolicy.node.js").tests.filter((entry) => entry.name === name); if (matches.length !== 1) throw new Error(`expected one test, found ${matches.length}`); matches[0].fn();'
```

It must still exit 1, but only after the dependency-map equality now passes and the
existing production `checkWalletBrokerManifest` rejects the six not-yet-authorized
WAL-006 dependencies with `wallet Rust manifest dependency pins or features differ from
review`. A dependency-map mismatch, syntax/load error, missing/duplicate test, mutation
failure, different reason, or unexpected pass is rejection; stop.

## Resolution, inventory, and custody gate

Run separately in this order with the exact environment:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 generate-lockfile --manifest-path wallet-broker/Cargo.toml
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fetch --manifest-path wallet-broker/Cargo.toml --locked
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features -e features
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --offline --all-features -e features
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --offline --all-features -d
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 metadata --manifest-path wallet-broker/Cargo.toml --locked --offline --format-version 1
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

The network-enabled resolution and fetch may use crates.io only. If the sandbox blocks
crates.io, request the normal network escalation and rerun only that exact failed
command. All later Cargo commands are locked and offline. Stop if Cargo changes any
source/test/manifest/policy byte; selects a wrong direct version;
introduces git/path/patched/non-crates.io source; reports an incompatible MSRV/feature;
or omits any checksum.

Require exact direct `hkdf 0.12.4`, `sha2 0.10.9`, and all six fixed Zcash versions and
architecture-review checksums. Require the resolved custody line to use `hmac 0.12.1`,
while `bip32 0.6.0-pre.1` uses only exact `hmac 0.13.0-pre.4` and
`sha2 0.11.0-pre.4`; stable `hmac 0.13.0` and stable `sha2 0.11.0` must be absent.
Inventory, without accepting, all package counts, additions/removals, features,
duplicate crypto primitives, build scripts, proc macros, sources, and licenses.

The final test must compile and report exactly 11 passed, zero failed/ignored. In
particular, the independent RFC 5869 HKDF-SHA256 vector and deterministic encrypted
envelope must remain byte-exact. Any warning-as-error issue, compile/link/runtime failure,
vector/envelope difference, unexpected test count, canary, or secret is rejection.

Do not run the fixture builder, any ZEC adapter test, full Node/Rust/npm suite, fmt,
clippy, check, audit, deny, SBOM, scanner, Electron, package, wallet, node, device, live
endpoint, mainnet, signing, proving, extraction, broadcast, or unlisted command.

## Evidence and Git

On an exact gate only, create
`docs/testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-01.md` recording timestamp/timezone,
protected parent, policy-progression result, every exact command/status, lockfile
line/hash and package diff, direct versions/checksums, resolved feature/duplicate/build/
proc-macro/source/license inventories, exact 11-test result, custody vector/envelope
result, no-canary result, and pre/post source hashes. Update only
`docs/handoff/CURRENT_TASK.md` to
`DEPENDENCY CORRECTION GATE RECORDED — GRAPH REVIEW REQUIRED` and link the evidence.

Read-only `git status`/`diff`, `wc`, `sha256sum`, `rg`, `sed`, `jq`, `sort`, and `comm`
inspection over the repository, lockfile, Cargo metadata, and exact ignored target paths
is authorized only to perform the required preflight and inventories; it may not mutate
or clean them.

Run `git diff --check`. Stage only the accepted manifest, production policy, resolved
lockfile, evidence, and `CURRENT_TASK.md`; inspect staged paths/diff. Commit once as
`fix: resolve WAL-006 dependency graph` and push `master`. Leave exactly the six Rust
tests unstaged/untracked at their accepted hashes. Require final `HEAD == origin/master`
and a clean index. Report commit, evidence count/hash, lock count/hash, all command
statuses/counts, inventories, exact remaining status, and any blocker. Stop without
fixture generation.
