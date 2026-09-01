# Codex Sol Handoff — BBD-WAL-005 Test Syntax Correction 01

State: ACTIVE

Production baseline: `14a6818740a1da0641f88a1f91dea3346554dddd`

Hermes stopped before production evaluation because accepted test source did not parse at
`test/walletPay.node.js:310`.

Edit only `test/walletPay.node.js`. Change only the assertion at line 310 so
`Buffer.byteLength` receives the sanitized label and `assert.strictEqual` receives the
result plus `128`:

```text
assert.strictEqual(
  Buffer.byteLength(sanitize(snapshot([account(...)] )).accounts[0].label),
  128
);
```

Preserve the test input and behavior exactly. Do not edit any other assertion or path. Do
not run tests, builds, formatters, package-manager, Git mutation, network, browser, device,
Rust, or cleanup commands. Read-only inspection, `git diff --check`, line count, and
SHA-256 are permitted.

Stop with the corrected path line count/hash and exact diff accounting.
