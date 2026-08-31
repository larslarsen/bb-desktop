# BBD-WAL-006 Address Gate Test Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `9cd07204`

Result: **SAFE STOP — PARALLEL TEST-ROOT CORRECTION REQUIRED**

Luna restarted the complete gate. The formatter exited 0. Warnings-denied production-library
Clippy exited 0 with no diagnostic. The full `zec_address` target then exited 101 with 4 passed
and 4 failed, so Luna stopped before Node, evidence, staging, commit, or push.

All four failures were the same deterministic concurrency defect:

```text
WAL-006 state ancestor creation failed: AlreadyExists
wallet-broker/src/zec/test_support.rs:76:34
```

Rust integration tests run in parallel. Multiple `TestStateRoot::fresh` calls can observe the
shared `target/wal006-state` ancestor as absent; one creates it and the others currently panic on
`AlreadyExists`. This is test-support plumbing, not an address/store semantic failure.

The correction must make absent-ancestor creation atomic and race-safe with a Unix directory
builder that applies mode `0700` at creation, accept `AlreadyExists` only after fresh
`symlink_metadata` proves the result is a real nonsymlink directory, and preserve failure for every
other error/type. It must not change permissions on the repository root or an already-existing
shared build directory. The unique per-test state root and production path checks remain unchanged.

Only `wallet-broker/src/zec/test_support.rs` is writable. No test, production store/address logic,
fixture, dependency, policy, evidence, or integration change is authorized. All execution remains
frozen until source review and a fresh full gate resume.
