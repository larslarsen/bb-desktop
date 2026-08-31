# BBD-WAL-006 Support-Dependency Production Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `d54744fc`

Result: **ACCEPTED — OFFLINE LOCK/POLICY GATE AUTHORIZED**

Accepted source:

- `wallet-broker/Cargo.toml`: 81 lines, SHA-256
  `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632`
- `scripts/security-policy.js`: 2,299 lines, SHA-256
  `60e41a12462d77c6be875f1659e5ef8a86d2b8146bd25d37ec7777297847d767`

The manifest adds only the exact reviewed `rand_core 0.6.4/std` and
`rusqlite 0.37.0/no-direct-features` lines. The policy exposes only the authorized
manifest constants, extends the exact dependency/test inventories, removes only the
obsolete blanket Zcash rejection, and scans the entire manifest to require every
reviewed dependency assignment exactly once in exact order. The existing loose Zcash
assignment appended outside `[dependencies]` is therefore rejected without denying the
accepted Zcash declarations.

No feature-authority/source-inventory export, ZEC source exception, test, fixture,
lockfile, workflow, package file, or Rust source changed. The frozen lock is 5,367 lines,
SHA-256 `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`.
The accepted Node test remains 2,454 lines, SHA-256
`f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`.

Luna may now execute the exact narrow gate, resolve the already-present packages into
the root lock dependency list offline, record evidence, and integrate. Any new package,
version, checksum, source, build script, license, feature beyond the reviewed union, or
unexpected Node/Rust result is a stop. ZEC Rust production remains frozen.
