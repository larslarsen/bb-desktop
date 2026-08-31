# BBD-WAL-006 Phase-C Policy Expected-Red Evidence 01

Protected governance parent: `9df3aa41`

## Execution

Accepted test source: `test/securityPolicy.node.js`, 2,401 lines, SHA-256
`19b7948bfa2c7f9b29426133bdda1630abfade5f1c438c7367e5c6dacd32688b`.
The mandated command was run once from the repository root:

```text
node test/securityPolicy.node.js
```

Exit status was `1`. The command output retained exactly 66 `ok` lines, seven
`not ok` lines, and final line `7 security policy test(s) failed`. The Node
runtime version was not emitted by the mandated command; no additional Node
command was run under this exact-red contract.

## Exact red

The seven failing groups were exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`
5. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
6. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
7. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The first three are the accepted prior workflow/WAL-004 integration failures.
The fourth and fifth report absent policy exports for the reviewed six exact
defaults-off dependency pins/direct feature union and the upstream PCZT versus
BitBook authority feature policy. The sixth reports the still-absent exact
bounded Phase-C inventory export: actual `undefined` versus the expected seven
paths (`wallet-broker/src/zec.rs`, `address.rs`, `fixture.rs`, `prepare.rs`,
`scan.rs`, `store.rs`, and `test_support.rs` under `wallet-broker/src/zec/`).
The seventh reports the intentionally unimplemented rejection of a mutated
workflow containing live-network and authority-bearing Rust snippets. No
previous `ok` case regressed; no syntax, fixture, module-resolution, exception,
or unrelated failure occurred.

No `wallet-broker/src/zec*` production path exists. Production and policy
implementation remained frozen; no Rust source/test, fixture, manifest,
lockfile, or production path was changed.

## Final state

Only the accepted test source, this evidence file, and the authorized
`CURRENT_TASK.md` update are integrated. No canary or secret values are recorded.
Phase-C implementation remains unauthorized pending review.
