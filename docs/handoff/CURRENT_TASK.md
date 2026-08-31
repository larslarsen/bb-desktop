# Current Task

Ticket: BBD-WAL-004

State: PRODUCTION GREEN INTEGRATED — FALSIFICATION AND CI SECURITY/SBOM GATES PENDING

Evidence: [BBD-WAL-004-GREEN.md](../testing/BBD-WAL-004-GREEN.md)

Active handoff: [CODEX_LUNA_BBD_WAL_004_AUDIT_RESUME.md](CODEX_LUNA_BBD_WAL_004_AUDIT_RESUME.md)

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

Sol completed the exact 15-path production source/policy drop, but it remains uncommitted
and is rejected pending test-first correction. The reviewer findings are frozen in
[production source review 01](../testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-01.md):
late authorization can revive expiry, native/session validation is incomplete,
diagnostics are not closed, direct Linux port operations follow symlinks, rfd converts
paths lossily, reviewed secrecy/Base64 dependencies are unused, and relevant policy
files/source inventory can evade the intended checks.

Correction 1 expected red remains accepted, but Sol's nine-path production correction is
rejected in
[production source review 02](../testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-02.md).
Static review found global lock events suppressible by malformed account input, missing
unlock-input wipe observation on clock failure, invalid UTF-8 native passphrases reaching
custody, wrong-mode descriptor operations, and filesystem-order-dependent source policy.
Correction 3 is reviewer-accepted in
[production source review 04](../testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-04.md).
Green Run 02 then passed formatting, build, all Node/security suites, and all 78 Rust
tests. The all-features Clippy gate exposed only pinned `eframe` and RustCrypto API
drift in two production files. Sol's exact compatibility correction is accepted in
[production source review 05](../testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-05.md).
All production and formatter-only test state is frozen by hash. GREEN_4's exact
formatter contingency, build, Node/security suites, zero-vulnerability npm audit, and
all 78 Rust tests passed. All-features Clippy then identified exactly three denied
behavior-equivalent idiom warnings in `store.rs` and `vault.rs`; Luna stopped before the
native compile and RustSec audit. Sol's exact three corrections are accepted in source
review 06. GREEN_5 repeated all functional gates successfully and production passed
Clippy; Clippy then found four compatibility warnings only in the independent-vector
test. Sol's one-test correction is reviewer-accepted with every vector byte and
assertion unchanged. GREEN_6 passed every local functional, lint, and native compile
gate. Only RustSec's advisory-database refresh was denied outbound network. Luna may
resume that exact audit with network access and integrate only on a clean result.
Post-commit falsification and CI security/SBOM results remain required.

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
