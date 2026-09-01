# BBD-WAL-006 Prepare NFC Dependency Gate 01 — Evidence

Jr Dev — Hermes integration gate evidence for the WAL-006 prepare NFC
unicode-normalization dependency.

## Hermes identity

- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream b20cc5f7 · local 10b6d1a9 (+1 carried commit)
- Provider: meituan/longcat-2.0:free
- Model: meituan/longcat-2.0:free

## Protected parent

`1d5baf4c` — `docs: authorize WAL-006 prepare NFC gate`. Verified
`HEAD == origin/master == 1d5baf4c` before any command.

## Source baseline verification

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 82 | `dcc4a9d7285aca962f1ea80d69ac3df9f276ffb735680b89c8e9c05ea15ffaf1` |
| `scripts/security-policy.js` | 2,306 | `1273868a1667aafc723d263bbb564ef3a9940a27d68e119deaee0308425e25dc` |
| `test/securityPolicy.node.js` | 2,525 | `a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba` |

Two-path working-tree diff: only `scripts/security-policy.js` and
`wallet-broker/Cargo.toml`. `git diff --check` clean.

## Frozen lock (pre-command)

5,369 lines — `ac454889b0796afea3f2a2ddfaf8c585bfff1080bec6d1428d0c227534ffb9dd`.

## Disk-backed target directories

`wallet-broker/target/wal006-tmp` and `wallet-broker/target/wal006-cargo`
exist on ext4 (`/dev/mapper/ubuntu--vg-ubuntu--lv`), not tmpfs.

## Command results

### 1 — Lock mutation (sole authorized, not `--locked`)

```
TMPDIR=.../wal006-tmp CARGO_TARGET_DIR=.../wal006-cargo \
  rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml \
  --offline --no-default-features --test vault_crypto
```

Exit 0. Added `unicode-normalization v0.1.25`.

### 2 — Feature tree

```
rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml \
  --locked --offline -e features -i unicode-normalization@0.1.25
```

Exit 0. Clean tree: broker direct defaults-off `std` feature, no extra
features or packages.

### 3 — Node policy test

```
node test/securityPolicy.node.js
```

Exit 1. 69 `ok`, exactly 6 `not ok`, final line
`6 security policy test(s) failed`. NFC dependency test
(`WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin`)
is `ok`.

The six frozen Phase-C failures:
- committed workflows satisfy the fail-closed checker
- strict nine-line reviewed Gitleaks ratchet bytes and content are enforced
- WAL-004 Rust source inventory is exported closed and enumerated by repository policy
- WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority
- WAL-006 requires the exact bounded Phase-C ZEC production inventory
- WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives

### 4 — Custody

```
rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml \
  --locked --offline --no-default-features --test vault_crypto
```

Exit 0. All 11 frozen `vault_crypto` tests passed.

## Lock diff (post-command)

5,379 lines — `9a6166ef2b39b47aa41b7a77cc3054dd8aee481f5a198a1ad4e4882111f97f59`.

Diff is exactly:
- `"unicode-normalization",` added to the `bitbook-wallet-broker` dependency array.
- One new package block:

```
[[package]]
name = "unicode-normalization"
version = "0.1.25"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8"
dependencies = [
 "tinyvec",
]
```

No existing package/version/checksum/source/dependency edge or other lock byte
changed.

## Negative capability

No network access. No formatter, Clippy, audit, scanner, Electron, wallet,
fixture generation, cleanup, or deletion. No amend, rebase, merge, or
force-push.
