# Codex Sol Handoff — BBD-WAL-004 Test Source

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable prompt. Ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Baseline: `abdd2b1980cbe8c5483a26b08b7ae43c82ae420b`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`,
`docs/engineering/WALLET_ROADMAP_ROUTING.md`, `tickets/BBD-WAL-004.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md` §§2.2, 3 T01/T08/T09/T17/T20/T21,
5.3–5.4, 6.1–6.2, 10, 13.1, 14, resolved Q10 and the decision register,
`docs/handoff/CURRENT_TASK.md`, all accepted WAL-002/WAL-003 tests, and every currently
authorized path.

Your sole task is the **test-source-only** phase in `tickets/BBD-WAL-004.md`. Author
behavioral Rust integration tests, the closed synthetic fixture, the test-harness Cargo
manifest, and the bounded Node policy-test additions before any production source, then
stop. The ticket fixes all security semantics. Do not substitute algorithms, versions,
costs, limits, error meanings, path behavior, clock rules, UI ownership, or dependency
features. If an exact crate pin or feature is incompatible, stop and report the exact
contradiction; do not select another version.

Authorized paths are exactly:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/vault_crypto.rs`
- `wallet-broker/tests/vault_format.rs`
- `wallet-broker/tests/vault_store.rs`
- `wallet-broker/tests/vault_session.rs`
- `wallet-broker/tests/native_surface.rs`
- `wallet-broker/tests/secret_hygiene.rs`
- `wallet-broker/tests/fixtures/vault-v1.json`
- `test/securityPolicy.node.js`

Use `apply_patch`. You may perform read-only inspection and report with `wc -l` and
`sha256sum` over only the nine authorized paths. Do not execute Rust, Cargo, Node, npm,
tests, builds, formatters, scanners, Git, GitHub, network, Electron, native windows,
child processes, wallets, nodes, hardware, or devices. Do not install anything. Do not
create `Cargo.lock`, production source, a toolchain file, evidence, or scratch data. Do
not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or variable/substitution-
resolved destructive targets.

The manifest is a test harness in this phase. Define explicit integration-test targets
so the expected-red compiler parses every authored test without `src/lib.rs`. Keep the
native GUI dependencies optional and off the expected-red path. Do not create a stub
library, mock production module, `compile_error!`, placeholder implementation, ignored
test, conditional skip, tautological source-text test, or test-only reimplementation of
the expected production result. Fixtures contain ciphertext/synthetic canaries only and
must not contain anything mnemonic-shaped.

Tests reserve a narrow, explicit future Rust API. Inject entropy, monotonic clock,
filesystem/fault boundary, wipe observer, and native surface ports only where the ticket
requires deterministic behavior. No injected boundary may become broker IPC. Assert
observable state, bytes, call ordering, permissions, stable errors, and post-wipe facts.
Use independent published vectors for primitives and a fixed envelope fixture rather
than computing every expected value through the future production helper.

Preserve all accepted Node policy assertions. Add only BBD-WAL-004 expectations; do not
weaken the WAL-003 preload/supervisor allowlists, routine no-packaging rule, sandbox,
Gitleaks ratchet, or current npm SBOM requirements. The later production phase will make
the new policy tests green.

Stop after authoring and report:

- each changed path, line count, and SHA-256;
- every test name and total per Rust/Node suite;
- the exact future Rust public API reserved by the tests;
- every manifest dependency/version/feature and why no looser requirement exists;
- the independent vector sources and fixed fixture fields;
- why each test group is non-vacuous and its exact expected-red compiler cause;
- the existing Node policy assertion count before/after and proof none was removed;
- confirmation that no secret, valid mnemonic, real address, user path, or mainnet
  material appears;
- confirmation that no command outside the allowed read-only/reporting set ran and no
  unlisted path changed.

Lead Engineer/Reviewer — Codex at XHigh must inspect and accept the exact test drop before
Codex Luna installs/resolves anything or executes expected red. You have no production,
execution, integration, evidence, Git, commit, push, install, or dependency-resolution
authority.
