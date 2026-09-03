# BBD-WAL-007 Phase-C Slice 5 Green 01 — COMPLETE GREEN EVIDENCE

State: COMPLETE GREEN — REVIEW REQUIRED

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 63279301 · local 10b6d1a9 (+1 carried commit)
Provider: nous
Model: meituan/longcat-2.0:free
Branch: master
HEAD: 6c12ab678d5cbcd80cb114a4635d5edbeaca35c1
origin/master: 6c12ab678d5cbcd80cb114a4635d5edbeaca35c1

## Filesystem

`wallet-broker/target` resides on ext2/ext3 (disk-backed). No `/tmp`, download, network access, personal Monero path, or product/Monero binary was used.

## Protected preconditions (verified)

| Path | Lines | SHA-256 | Status |
| --- | ---: | --- | --- |
| `wallet-broker/src/xmr.rs` | 8 | `78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce` | OK |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` | OK |
| `wallet-broker/src/xmr/account.rs` | 3,375 | `5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb` | OK |
| `wallet-broker/src/xmr/process.rs` | 1,964 | `66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d` | OK |
| `wallet-broker/src/xmr/rpc.rs` | 2,576 | `1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33` | OK |
| `wallet-broker/src/xmr/store.rs` | 1,904 | `3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4` | OK |
| `wallet-broker/src/xmr/receiver.rs` | 868 | `daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9` | OK |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc` | OK |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e` | OK |
| `docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md` | 59 | `20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637` | OK |

All frozen identities verified. `git diff --check` clean. Index clean. HEAD == origin/master.

## Formatter — no mutation

Command:
```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Result: exit 0, no output, no warning/diagnostic, no source/test mutation.

## Temporary durable-replay falsification

Temporarily replaced `if let Some(existing) = existing {` (receiver.rs:671) with `if let Some(existing) = existing.filter(|_| false) {`.

Command:
```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_receiver exact_replay_returns_durable_binding_without_any_rpc_call
```

Result: exit 101, no warning/compile diagnostic, 0 passed, 1 failed, 0 ignored, 0 measured, 14 filtered out. The selected test failed because suppressing the durable replay return caused the replay attempt to continue instead of returning the stored binding.

Immediately restored the original line. Post-restoration proof: `wallet-broker/src/xmr/receiver.rs` is 868 lines at SHA-256 `daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9`. The temporary mutation was never staged or committed.

## Exact green sequence

Each command run exactly once, sequentially, byte-for-byte, with no wrapper, redirection, or pipeline.

| # | Command (normalized) | Result |
| --- | --- | --- |
| 1 | `cargo test --test xmr_receiver` | 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 2 | `cargo test --test xmr_account` | 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 3 | `cargo test --test xmr_hygiene` | 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 4 | `cargo test --test xmr_rpc` | 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 5 | `cargo test --test xmr_process` | 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 6 | `cargo test --test xmr_distribution` | 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 7 | `cargo test --test native_surface` | 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out |
| 8 | `cargo clippy --lib --test xmr_receiver -- -D warnings` | exit 0, no warning/diagnostic |
| 9 | `cargo check --features native-ui --test native_surface` | exit 0, no warning/diagnostic |
| 10 | `node test/securityPolicy.node.js` | exit 0, exactly 86 `ok`, no `not ok`, final line `BitBook security policy tests passed (86).` |
| 11 | `node scripts/security-policy.js` | exit 0, final line `BitBook desktop security policy checks passed.` |

All Rust commands emitted no warning or diagnostic. No accepted source/test file mutated during the green sequence.

## Scope and prohibitions

- No source/test was designed, authored, or permanently edited.
- No command was wrapped, redirected, repeated, or altered.
- No real local-Monero gate, broader/final acceptance, or other repository was touched.
- No local Monero path, artifact/cache path, environment value, port, credential, process ID, seed, primary address, receiver, or raw sensitive output is recorded here.
