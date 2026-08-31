# Codex Sol Handoff — BBD-WAL-004 Test Source Correction 3

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-004.md`,
the original test-source handoff and Corrections 1–2, `docs/handoff/CURRENT_TASK.md`,
`docs/testing/BBD-WAL-004-TEST-SOURCE-REVIEW.md`, `wallet-broker/Cargo.toml`, and the
complete `test/securityPolicy.node.js`.

The owner installed Rust/Cargo 1.98.0. Luna's sandboxed resolution first failed DNS. The
reviewer's approved network retry reached crates.io and produced this authoritative
resolver error before any lockfile was created:

```text
failed to select a version for `secrecy`
package `bitbook-wallet-broker` depends on `secrecy` with feature `alloc`
but `secrecy` does not have that feature
available features: serde
```

Official crate metadata confirms secrecy 0.10.3 has only optional `serde`; its
allocation-backed secret wrappers do not require an `alloc` feature. The version pin is
not changing.

Your sole task is to edit exactly:

- `wallet-broker/Cargo.toml`
- `test/securityPolicy.node.js`

Use `apply_patch`. In the manifest, change only secrecy to:

```toml
secrecy = { version = "=0.10.3", default-features = false }
```

In the Node `WAL004_DIRECT_DEPENDENCIES` expectation, change only secrecy's features from
`['alloc']` to `[]`. Add a non-vacuous manifest mutation in the existing WAL-004 manifest
test that transforms the exact corrected secrecy line into one enabling `features =
["serde"]` and requires rejection. Preserve every other manifest pin/feature, Node test,
assertion, name, constant, and byte outside the minimal changes. Do not enable defaults,
serde, another secrecy version, another crate, or a loose requirement.

You may perform read-only inspection and final `wc -l`/`sha256sum` reporting over only
the two authorized paths. Do not execute Rust, Cargo, Node, npm, tests, builds,
formatters, scanners, Git, GitHub, network, install, child processes, Electron, native
windows, wallets, nodes, hardware, or devices. Do not create `Cargo.lock` or edit
`.gitignore`, production, evidence, documentation, or any unlisted path. Do not use root,
`sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or unresolved destructive targets.

Stop and report both final line counts/SHA-256 values, exact changed assertion/mutation,
confirmation that the secrecy pin remains exact/default-off/no-feature, and confirmation
that no unlisted path changed and no prohibited command ran. Codex XHigh will inspect the
correction and authorize a separate Luna resume before any further resolution or Git.
