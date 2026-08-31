# Current Task

Ticket: BBD-WAL-004

State: LOCK GRAPH ACCEPTED — SOL PRODUCTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Production baseline: `fe2fe7e78fab0012a5fa77f128716bb7262aba58`

[BBD-WAL-004](../../tickets/BBD-WAL-004.md) is the only authorized implementation
ticket in this repository. The owner resolved architecture Q10 in favor of a minimal
native authorization window running inside the Rust wallet-broker process. It owns
software unlock, backup/restore, and later authoritative payment confirmation. A
broker-invoked native file picker may select backup paths. Electron never owns the
window, sees a passphrase/path/backup, or gains confirm/unlock/export authority;
hardware devices confirm independently when capable.

The owner installed official user-level Rust/Cargo 1.98.0. Luna integrated the exact
accepted tests, fixture, lockfile, expected-red evidence, and ignored disk-backed target
path at `fe2fe7e78fab0012a5fa77f128716bb7262aba58`; `HEAD == origin/master`. The Node
source-first suite has 57 `ok` and the seven exact expected `not ok` results. The named
Rust vector test reaches compilation and fails solely at the absent future broker crate.

The reviewer accepted the complete lock graph in
[BBD-WAL-004-LOCK-GRAPH-REVIEW.md](../testing/BBD-WAL-004-LOCK-GRAPH-REVIEW.md): 38
headless and 172 Linux all-features package/version pairs, crates.io-only checksummed
sources, no duplicate crypto primitive, no forbidden browser/web/wgpu/network runtime,
reviewed build scripts and licenses, and a clean pinned RustSec scan of all 327 lockfile
records. The new crate must declare the repository's MIT license during production.

Only [the Sol production handoff](CODEX_SOL_BBD_WAL_004_PRODUCTION.md) is active. Sol
authors the bounded production source and policy drop but executes no commands and uses
no Git. The committed tests, fixture, and lockfile remain immutable. Luna integration,
green, broader security/SBOM acceptance, falsification, evidence, commit, and push are
not yet authorized. `/tmp` remains forbidden for substantial Rust state.

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
