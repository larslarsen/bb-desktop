# Current Task

Ticket: BBD-WAL-004

State: TEST SOURCE ACCEPTED — OWNER RUST TOOLCHAIN INSTALL REQUIRED

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

The initial Sol drop plus Corrections 1 and 2 are reviewer-accepted at the exact hashes in
[the test-source review](../testing/BBD-WAL-004-TEST-SOURCE-REVIEW.md). The nine accepted
paths remain unstaged and uncommitted. No production source is authorized.

Rust and Cargo are not installed. The owner must first install official rustup with the
minimal pinned Rust 1.98.0 toolchain under `/home/lars`, using a disk-backed temporary
directory under `/home/lars/.cache` and no root. After that explicit owner action,
[the Luna expected-red handoff](CODEX_LUNA_BBD_WAL_004_RED.md) authorizes lockfile
resolution, exact red execution/evidence, and Git integration. Until the owner reports
the installation complete, Luna must not act. Large build/cache/temp trees may not use
`/tmp`.

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
