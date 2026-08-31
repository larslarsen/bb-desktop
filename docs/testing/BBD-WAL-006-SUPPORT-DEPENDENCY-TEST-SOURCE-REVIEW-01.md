# BBD-WAL-006 Support-Dependency Test Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `152a0b2b`

Result: **ACCEPTED — FOCUSED EXPECTED RED AUTHORIZED**

The changed `test/securityPolicy.node.js` is 2,454 lines with SHA-256
`f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`
and 74 named tests. It is the only modified path.

The drop adds one independently named support-dependency test and leaves the six-crate
`WAL006_DIRECT_DEPENDENCIES` compatibility object unchanged. The new frozen object
requires exactly:

- `rand_core = 0.6.4`, defaults off, `std` only, non-optional; and
- `rusqlite = 0.37.0`, defaults off, no direct features, non-optional.

The test first requires each exact one-line declaration exactly once in the real
manifest, then validates the future policy export and real manifest checker. Its
mutations cover loose versions, enabled defaults, removed/widened RNG features, and
three forbidden direct SQLite feature families including loadable extension and
vendored SQLCipher/OpenSSL authority. The ordering makes the current missing-manifest
red specific and non-tautological.

No manifest, lockfile, policy implementation, Rust source/test, fixture, workflow,
package file, or other path changed. Reviewer source inspection finds no assertion
weakening or contradiction. Luna may run the one focused Node policy file and integrate
only exact-red evidence plus this accepted test/current-task update. Production,
manifest, lockfile, and policy implementation remain frozen.
