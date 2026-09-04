# BBD-WAL-009 Phase-A1 Expected-Red 01

Hermes Jr Dev integration evidence for the completed Phase-A2 expected-red run.

## Identity

- Hermes: `v0.18.2 (2026.7.7.2)`
- Provider/model: `meituan/longcat-2.0:free`
- Execution parent: `efd1210dbbf6fbf942b5b00ce2e56def7027703a`
- Branch: `master`
- `HEAD` and `origin/master` confirmed equal at `7191541e`

## Preflight

Governance index clean. Worktree contains only the two expected source identities;
no other path is modified or untracked. `wallet-broker/Cargo.lock` is unchanged.

Frozen source identities confirmed:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,115 | `80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd` |

## Formatter

Command:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
```

Result: exit `0`, no mutation.

## Focused expected-red

Command:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify
```

Result: exit `101`, zero tests executed.

## Normalized absent-contract diagnostics

The three diagnostic groups accepted by the stop review:

1. absent `src/zec/spend.rs` from the permitted production-inventory assertion;
2. unresolved new sign/verify types imported from `zec::test_support`; and
3. absent new `FrozenFixture::expected_destination_receiver_bytes` test-support method.

No dependency, lockfile, syntax, existing-source, fixture, network, or unrelated
failure was present.

## Conservative stop

Hermes stopped without editing, staging, committing, pushing, rerunning, or running
another gate. No command followed the accepted stop.

## Scope

This is expected-red documentation only. It claims no green, production, hardware,
network, or broadcast support.
