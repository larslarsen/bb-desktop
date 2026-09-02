# BBD-WAL-007 Slice-2 Owned-Child Expected Red 01

State: COMPLETE

Integration actor: Jr Dev — Hermes

## Environment identity

Hermes Agent v0.18.2 (2026.7.7.2) · upstream c5c9aa8d · local 10b6d1a9
Provider: nous
Model: meituan/longcat-2.0:free
Node.js v22.23.1
Branch: master
HEAD: d1633b04d606aed3dc7fc6e65ed225665ff3a8c0
origin/master: d1633b04d606aed3dc7fc6e65ed225665ff3a8c0

## Protected preconditions (verified)

| Path | Lines | Tests | SHA-256 | Status |
| --- | ---: | ---: | --- | --- |
| `wallet-broker/tests/xmr_process.rs` | 374 | 12 | `12cb52a5efca6a5ebfa53b1e856fc816c5ae7e8e01849b9034bd11d5a74d6f06` | OK |

Frozen Slice-1 production: commit `c139641a`. Cargo.toml and Cargo.lock retain frozen hashes.

`git diff --check`: clean. Index clean. HEAD == origin/master.

## Gate results

### 1. Formatting

```
cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Exit: 0. No source or test mutation.

### 2. Expected red

```
cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
```

Exit: 101 (compile failure).

Diagnostics limited to the exact absent Slice-2 production API:

```
error[E0432]: unresolved import `bitbook_wallet_broker::xmr::process`
 --> tests/xmr_process.rs:1:33

error[E0432]: unresolved imports `bitbook_wallet_broker::xmr::test_support::ChildExit`,
    `ProcessFault`, `ProcessRig`, `TeardownCause`, `XmrNetwork`
 --> tests/xmr_process.rs:6:5
```

Zero tests executed — the target's production imports cannot compile. No dependency, lock, syntax, formatting, toolchain, network, linker, unrelated source, or runtime-test failure.

## Architecture-decision reference

Slice 2 (owned-child process lifecycle) requires production code that can signal and reap a child process group. The XHigh decision ([BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md](../architecture/BBD-WAL-007-SLICE-02-OWNED-CHILD-DECISION.md)) documents that stable safe Rust cannot signal a process group, which is why Sol stopped before editing and the production boundary remains absent. This expected-red result proves the test is non-vacuous: it fails to compile because the production API it tests does not yet exist.

## Scope

Only the corrected test file `wallet-broker/tests/xmr_process.rs` was modified. No production source, manifest, lockfile, or other test changed.

## Prohibited-action confirmation

No production source was added. No `xmr_local_gate`, Monero binaries, wallet, node, Electron, npm, browser, scanner, full suite, or network operation was run. No product or Monero binary was started.
