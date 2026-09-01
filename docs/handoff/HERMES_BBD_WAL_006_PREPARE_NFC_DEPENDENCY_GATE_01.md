# Hermes Handoff — BBD-WAL-006 Prepare NFC Dependency Gate 01

You are **Jr Dev — Hermes**. Own only this dependency execution/evidence/integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 82 | `dcc4a9d7285aca962f1ea80d69ac3df9f276ffb735680b89c8e9c05ea15ffaf1` |
| `scripts/security-policy.js` | 2,306 | `1273868a1667aafc723d263bbb564ef3a9940a27d68e119deaee0308425e25dc` |
| `test/securityPolicy.node.js` | 2,525 | `a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba` |

Frozen lock: 5,369 lines,
`ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd`.

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Design Review 01, the dependency test/red and
production source reviews/evidence, both Sol handoffs, complete changed source/test, manifest, and
lockfile.

## Preconditions

Record actual Hermes version/provider/model, protected `HEAD == origin/master`, status, exact
two-path source diff/hashes, frozen test/lock identities, `git diff --check`, disk-backed
`wallet-broker`, and existing explicit `wallet-broker/target/wal006-{tmp,cargo}` directories. Stop
on any mismatch or other tracked/untracked source change.

## Exact sequence

Use Rust/Cargo 1.98.0. Run only these commands, separately, from repository root with
`TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp` and
`CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo` on Cargo
commands:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --offline --no-default-features --test vault_crypto
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline -e features -i unicode-normalization@0.1.25
node test/securityPolicy.node.js
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

The first command is intentionally not `--locked` and is the sole authorized lock mutation. It
must exit 0. Immediately inspect the entire lock diff before continuing. It must add exactly:

- `"unicode-normalization",` to the `bitbook-wallet-broker` dependency array; and
- one crates.io package block for `unicode-normalization 0.1.25`, checksum
  `5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8`, depending only on the
  already locked `tinyvec`.

No existing package/version/checksum/source/dependency edge or other lock byte may change. Stop
before command two on any wider or absent diff.

The feature tree must show the broker's direct defaults-off `std` feature and only the expected
existing `tinyvec` transitive family. Node must exit 1 with exactly 69 `ok`, exactly six `not ok`,
the same six frozen Phase-C failure names, and final line `6 security policy test(s) failed`; the
new NFC dependency test must be `ok`. Custody must pass all 11 frozen `vault_crypto` tests. Stop on
any warning/failure/extra feature/package/build script/source/license/authority or mutated source.

## Evidence and integration

On exact success, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-NFC-DEPENDENCY-GATE-01.md` and update only the leading
state/actor/active-handoff block in `docs/handoff/CURRENT_TASK.md`. Record commands/statuses, exact
lock diff, checksum/source/features/license, counts/names, hashes/lines, negative capability, and
Hermes identity.

Stage exactly the manifest, lockfile, policy implementation, evidence, and `CURRENT_TASK.md`.
Commit exactly `build: add WAL-006 prepare NFC dependency`, push `master`, and prove a clean
worktree/index with `HEAD == origin/master`.

Do not edit source/test, fixture, ticket, workflow, package, deny policy, or another path. Do not
run formatter, Clippy, another Cargo/Node/npm command, audit/scanner, network, Electron,
wallet/node/device, fixture generation, cleanup, or deletion. Do not amend, rebase, merge, or
force-push. Stop and report exact evidence on any mismatch.

