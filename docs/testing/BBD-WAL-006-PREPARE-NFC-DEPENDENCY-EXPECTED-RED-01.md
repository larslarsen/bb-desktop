# BBD-WAL-006 Prepare NFC Dependency Expected-Red Evidence 01

Protected governance parent: `c4d3ae19`

## Execution

Accepted test source: `test/securityPolicy.node.js`, 2,525 lines, SHA-256
`a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba`, 75
named tests. The mandated command was run once from the repository root:

```text
node test/securityPolicy.node.js
```

Exit status was `1`. The command output retained exactly 68 `ok` lines, exactly
seven `not ok` lines, and final line `7 security policy test(s) failed`. The
Node runtime version was not emitted by the mandated command; no additional
Node command was run under this exact-red contract.

Hermes Agent v0.18.2 (2026.7.7.2) · provider `nous` · model
`meituan/longcat-2.0:free`.

## Exact red

The seven failing groups were exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy`
4. `WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin`
5. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
6. `WAL-006 requires the exact bounded Phase-C ZEC production inventory`
7. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The first three are the accepted prior workflow/WAL-004 integration failures.
The next three (5-7) are the accepted frozen Phase-C partial-red failures. The
fourth is the new exact-red failure: the manifest does not contain the exact
`unicode-normalization = { version = "=0.1.25", default-features = false, features = ["std"] }`
declaration, so the prerequisite unique-exact-line assertion fails before any
policy export is consulted. No previous `ok` case regressed; no syntax,
fixture, module-resolution, exception, or unrelated failure occurred.

The specific missing-manifest assertion reported:
`manifest prepare dependency line must be unique and exact: unicode-normalization = { version = "=0.1.25", default-features = false, features = ["std"] }`
with `0 !== 1`.

## Negative capability record

The test does not implement, fetch, resolve, or install any dependency. It
reads only the committed manifest text and asserts the exact declaration is
present exactly once. No manifest, lockfile, policy implementation, Rust
source/test, fixture, workflow, or production path was changed. No network
access, process spawn, or filesystem mutation occurred beyond reading the
committed manifest.

## Final state

Only the accepted test source, this evidence file, and the authorized
`CURRENT_TASK.md` update are integrated. No canary or secret values are recorded.
Prepare dependency production and all Rust prepare production remain frozen.
