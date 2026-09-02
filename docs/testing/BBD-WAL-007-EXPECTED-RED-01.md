# BBD-WAL-007 Expected Red 01 — STOP

State: STOP — LOCK CONFLICT

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9
Provider: meituan/longcat-2.0:free
Node.js v22.23.1
Branch: master
HEAD: 1ceea8cb78e1fd42fb35ca9a980512ae9d93872
origin/master: 1ceea8cb78e1fd42fb35ca9a980512ae9d93872

## Preconditions (verified)

| Path | Lines | SHA-256 | Status |
| --- | ---: | --- | --- |
| `wallet-broker/Cargo.toml` | 113 | `84e0e4eac1d64d10128334163b0ddbeaf2721aff429828929425e342d4573456` | OK |
| `wallet-broker/tests/native_surface.rs` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` | OK |
| `wallet-broker/tests/xmr_distribution.rs` | 257 | `b17603919a4db88ff585e96ae590cbc7101687d787d9604de6a16599607d3e46` | OK |
| `wallet-broker/tests/xmr_process.rs` | 336 | `0a2ed9cb452015861bf0b66a13a788c8221609be5de7880b3a454a37a3c97f17` | OK |
| `wallet-broker/tests/xmr_rpc.rs` | 398 | `bd355d31d0ae64736e14f412293d54190d1c701f1cff2252b16d7ca03001ee18` | OK |
| `wallet-broker/tests/xmr_account.rs` | 537 | `049eabad90979fcdbe3555460c047d8237900f9d8dc1d7ade7049996654afc3e` | OK |
| `wallet-broker/tests/xmr_receiver.rs` | 553 | `e880fd2b3dfeadf412c2e44b85c17f9d7fd4d67ed691f07420bd09035b4d07cd` | OK |
| `wallet-broker/tests/xmr_hygiene.rs` | 281 | `5c94c9452fea5229fdefc0568088ad4f95ee4ac61111fface59109540da87374` | OK |
| `wallet-broker/tests/xmr_local_gate.rs` | 477 | `b3d558421cb0eb81aa13e525a9ea2cbb85e19c41425f927da050b6cc48935f1e` | OK |
| `test/securityPolicy.node.js` | 3,067 | `41988b598fb73afd10eade38dd97527fd1db31ca430a6760bdde701a400da0fb` | OK |

Original `wallet-broker/Cargo.lock` SHA-256: `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01`

`git diff --check`: clean.

## Stop reason

The lock resolution command failed:

```
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --no-default-features --lib
```

Exit code: 101

Error:
```
error: failed to select a version for `digest`.
    ... required by package `md-5 v0.11.0`
    ... which satisfies dependency `md-5 = "=0.11.0"` of package `bitbook-wallet-broker v0.1.0`
versions that meet the requirements `^0.11` are: 0.11.3, 0.11.2

all possible versions conflict with previously selected packages

  previously selected package `digest v0.11.0-pre.9`
    ... which satisfies dependency `digest = "=0.11.0-pre.9"` of package `hmac v0.13.0-pre.4`
    ... which satisfies dependency `hmac = "=0.13.0-pre.4"` of package `bip32 v0.6.0-pre.1`
    ... which satisfies dependency `bip32 = "=0.6.0-pre.1"` of package `zcash_client_backend v0.24.0`
    ... which satisfies dependency `zcash_client_backend = "=0.24.0"` of package `bitbook-wallet-broker v0.1.0`

failed to select a version for `digest` which could resolve this conflict
```

## Analysis

The accepted `Cargo.toml` pins `md-5 = "=0.11.0"` which requires `digest ^0.11` (i.e., `digest` 0.11.x). However, the existing ZEC dependency chain locks `digest` to `0.11.0-pre.9`:

- `zcash_client_backend = "=0.24.0"` → `bip32 = "=0.6.0-pre.1"` → `hmac = "=0.13.0-pre.4"` → `digest = "=0.11.0-pre.9"`

Cargo cannot unify `digest 0.11.x` (a release) with `digest 0.11.0-pre.9` (a pre-release). This is a genuine version conflict that cannot be resolved without either:
1. Changing the `md-5` pin to a version compatible with `digest 0.11.0-pre.9`, or
2. Patching the ZEC dependency chain to use a compatible `digest` version, or
3. Using `[patch]` to override the `digest` version (not permitted by the ticket).

The handoff states: "Stop on a git dependency, wildcard, patch, vendored source, downloader, native code, network/runtime/TLS/URL dependency, or any build script in the new closure." This conflict is a version resolution failure, which falls under the general stop condition: "On any hash/count mismatch, unexpected failure, leak, audit finding, syntax error, hang, resource leak, network/provider access, or unlisted change: do not stage, commit, or push."

## No changes made

No source bytes were modified. No lockfile was written. No commit or push was performed. Control returned to reviewer.
