# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 3 Correction 02

Status: AUTHORIZED — SECOND BOUNDED REVIEW CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`,
Slice-3 Upstream RPC Decision, Slice-3 Source Reviews 01 and 02, Correction 01, the
complete current three-path mutable drop, and `docs/handoff/CURRENT_TASK.md`.

## Exact path boundary

Edit only:

- `wallet-broker/tests/xmr_rpc.rs` — 614 lines, 15 tests, SHA-256
  `721636f1c4d8851c811193b74fa2085ce6d60f89d2ce540e68132f383de2e336`;
- `wallet-broker/src/xmr/rpc.rs` — 1,733 lines, SHA-256
  `fa8b955d4ce63e07e5e07216c0f665ebb8027e62c18cfa7418447be65ca847ed`;
- `wallet-broker/src/xmr/test_support.rs` — 2,526 lines, SHA-256
  `6894c757ea0a173b6d33d4780df9d2af97f57a64952d458e5e7359828b731f18`.

Freeze:

- `wallet-broker/src/xmr.rs` — 5 lines —
  `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411`;
- `wallet-broker/src/xmr/model.rs` — 151 lines —
  `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9`;
- `wallet-broker/src/xmr/process.rs` — 1,184 lines —
  `7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f`.

Every other path and repository is read-only.

## Test-first correction

Correct the oracle before production:

1. Make the successful `get_info` fixture use a bounded software-version string. Add
   production-core tests that reject numeric/type-confused versions and accept the
   pinned shape.
2. Exercise `get_info` with both block-weight optionals present, each omitted
   independently, and both omitted. When present they must be `u64`; missing required
   or extra unknown members must still fail closed.
3. Exercise exact `hard_fork_info` widths: reject `version`/`voting` above `u8::MAX`,
   reject `window`/`votes`/`threshold`/`state` above `u32::MAX`, and reject state outside
   `0..=2`. Use a valid state in the successful fixture. Retain the documented default
   `{}` request and assert it exactly.
4. Falsify readiness retry classification. A pre-listen connection refusal may retry;
   connect timeout and other non-refusal connect failures must make one attempt only.
   A refusal on the authenticated exchange after a successful challenge must also make
   no new readiness attempt. All these failures remain `UNAVAILABLE`.

Keep all 15 existing test names and every accepted assertion; strengthening in place or
adding focused tests is permitted. Do not run the corrected tests.

## Production correction

1. Parse `get_info.version` as a bounded string. Permit only the pinned required member
   set plus optional `block_weight_limit` and `block_weight_median`; validate either as
   `u64` when present and reject every other missing, unknown, or confused member.
2. Enforce the pinned `uint8_t`/`uint32_t` hard-fork widths and state `0..=2`; preserve
   the already-correct status/untrusted policy and do not add enabled/height policy.
3. Retry readiness only for `std::io::ErrorKind::ConnectionRefused` occurring before
   any successful exchange in that complete wallet readiness attempt. Do not classify
   timeout or any other connect/read/write/protocol/auth failure as not-yet-listening.
4. Retain every accepted Correction-01 behavior and boundary. Do not widen methods,
   endpoints, transports, response output, deadlines, error surfaces, or secret
   observability.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit evidence or
governance, or begin Slice 4, Slice 5, broader acceptance, or the real local-Monero gate.

Stop after the exact three-path correction. Report changed paths, line counts/hashes,
the test-first edit order, and the implementation/test mapping for all four findings.
Reviewer XHigh acceptance is required before Hermes may execute or integrate.
