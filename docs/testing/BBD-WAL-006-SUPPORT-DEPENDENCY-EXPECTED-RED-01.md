# BBD-WAL-006 Support-Dependency Expected-Red Evidence 01

Protected governance parent: `67be9a74`

## Execution

Accepted test source: `test/securityPolicy.node.js`, 2,454 lines, 74 named tests,
SHA-256 `f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`.
The exact command was run once from the repository root:

```text
node test/securityPolicy.node.js
```

It exited `1`, with exactly 66 `ok` lines and 8 `not ok` lines. The final line was
`8 security policy test(s) failed`.

## Exact red

The eight failing groups, in output order, were exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`
5. `WAL-006 manifest pins the exact production support APIs for RNG and SQLite`
6. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
7. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
8. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The new fifth failure occurs at the exact support-line assertion before
policy-export validation: the real manifest contains neither required exact
support line. The unique `rand_core` line count is `0` for
`rand_core = { version = "=0.6.4", default-features = false, features = ["std"] }`,
and the corresponding exact `rusqlite` support line is also absent. The other
seven failures retain their accepted causes. No prior `ok` case regressed; there
was no syntax, exception-outside-test, fixture, Rust, module-resolution, or
other-path failure.

## Frozen inputs and source boundary

The manifest and lockfile were unchanged:

| Path | SHA-256 |
| --- | --- |
| `wallet-broker/Cargo.toml` | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `wallet-broker/Cargo.lock` | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The manifest still omits direct `rand_core` and `rusqlite` dependency lines, and
no `wallet-broker/src/zec*` production path exists. Production Rust, manifest,
lockfile, policy implementation, fixtures, and all other source remained frozen.
No secret or canary value is recorded.

## Final state

Only the accepted Node test, this evidence, and the authorized CURRENT_TASK state
update are integrated. The support dependencies and all Phase-C implementation
remain unauthorized pending review.
