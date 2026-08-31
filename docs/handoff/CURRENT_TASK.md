# Current Task

Ticket: BBD-WAL-004

State: REVIEW CORRECTION 3 — CODEX SOL TEST SOURCE ONLY

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `abdd2b1980cbe8c5483a26b08b7ae43c82ae420b`

[BBD-WAL-004](../../tickets/BBD-WAL-004.md) is the only authorized implementation
ticket in this repository. The owner resolved architecture Q10 in favor of a minimal
native authorization window running inside the Rust wallet-broker process. It owns
software unlock, backup/restore, and later authoritative payment confirmation. A
broker-invoked native file picker may select backup paths. Electron never owns the
window, sees a passphrase/path/backup, or gains confirm/unlock/export authority;
hardware devices confirm independently when capable.

The owner installed official user-level Rust/Cargo 1.98.0 successfully. Luna's preflight
matched every accepted byte and added only the authorized root `.gitignore` line
`target/`. The first sandboxed resolution failed on DNS; an approved network retry then
reached crates.io and exposed a real manifest contradiction: `secrecy` 0.10.3 has no
`alloc` feature. No `Cargo.lock`, evidence, staging, commit, or push was created.

Only [Correction 3](CODEX_SOL_BBD_WAL_004_TESTS_CORRECTION_3.md) is active. Codex Sol may
edit exactly `wallet-broker/Cargo.toml` and `test/securityPolicy.node.js` to remove the
nonexistent secrecy feature while preserving the exact version/default-off contract and
add the feature-regression mutation. All other accepted test bytes and Luna's uncommitted
`.gitignore` line are frozen. The existing Luna red handoff is paused until reviewer
acceptance and a durable resume handoff. Large build/cache/temp trees may not use `/tmp`.

BBD-WAL-003 is complete and reviewer-accepted at production commit
`584019e9a89022d77b4bbb6710c2b7670e42d95b`, falsification commit
`2e7e1599b6aee9aa5034d8854ac08bd54eadfe1e`, and acceptance commit
`abdd2b1980cbe8c5483a26b08b7ae43c82ae420b`. GitHub Social client run
`33342988248` passed with package jobs skipped. Its secure Electron/future-broker
boundary remains a frozen input; it does not ship a wallet or native broker.

The independent `../bb-go` BBGO-PAY-001 ticket remains queued for Grok Build at or after
2026-08-30 19:53 PDT under its own durable handoff. Codex Spark remains reserved for
later mechanical UI/view-model work. `../go-ipfs` is deprecated and receives no wallet
work.
