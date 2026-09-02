# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 3 Correction 01

Status: AUTHORIZED — REVIEW CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-007.md`,
Slice-3 Source Review 01, Slice-3 Upstream RPC Decision, the original Slice-3 handoff,
the complete current four-path drop, the complete `xmr_rpc` test, and
`docs/handoff/CURRENT_TASK.md`.

## Exact path boundary

Edit only:

- `wallet-broker/tests/xmr_rpc.rs` — 422 lines, 15 tests, SHA-256
  `0046a94d8a3f7932c02e872f90afdcd8e0a79641f3b87db6cac4e2db25311b86`;
- `wallet-broker/src/xmr.rs` — 5 lines, SHA-256
  `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411`;
- `wallet-broker/src/xmr/model.rs` — 151 lines, SHA-256
  `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9`;
- `wallet-broker/src/xmr/rpc.rs` — 1,441 lines, SHA-256
  `7f4b8a194cd7b351883369192589ef68a15758ff7849c7b1dbbb4b311f009aa7`;
- `wallet-broker/src/xmr/test_support.rs` — 2,154 lines, SHA-256
  `a019c4800df0a4c819619c6f41f66cf56d86011c85b3c4bbac64dae10ae1eba9`.

Freeze `wallet-broker/src/xmr/process.rs` at 1,184 lines and SHA-256
`7533a86344c178ed25143d21bc3961a72ea7a1fcf85067f90b57134178928a1f`.
Every other path and repository is read-only.

## Required test correction

Correct the oracle before correcting production:

1. Replace synthetic wallet version `196610` with pinned exact `65567` and assert the
   constant directly.
2. Make successful node fixtures contain and type-check the pinned v0.18.5.1 `get_info`
   and `hard_fork_info` member inventories, including `credits`, `top_hash`, and hard-
   fork `untrusted`.
3. Add non-vacuous accepted cases for a synchronized node with `target_height=0`, a
   published-shape target below current height, and a future/not-yet-enabled hard fork.
   Add a rejecting hard-fork `untrusted=true` case. Retain the full network-boolean,
   bootstrap, offline, status, malformed, oversized, and no-fallback matrix.
4. Exercise the pinned full wallet result shapes used in this slice, including complete
   `get_balance` metadata/nested entries and complete `create_address` scalar/vector
   members. Assert that only checked typed/sanitized values cross the production core.
5. Assert exact typed request params for every method made callable in Slice 3. A method
   may remain listed in the closed authority inventory but must reject before transport
   until its typed, phase-valid request payload exists.
6. Replace hard-coded negative/hygiene answers with recording-port/core observations.
   Unlisted-method rejection must be tied to the absence of a production dispatch value;
   no raw bytes may be observable after typed parsing; DNS/proxy claims must derive from
   the numeric-port transport interface; retained authorization observations must be
   explicitly zeroized/cleared before a wipe result can be true.

Keep all 15 existing test names and strengthen them in place; additional focused tests
are permitted. Do not weaken any accepted boundary, add ignored/conditional passes, or
run the corrected test.

## Required production correction

Retain the valid core and correct every Source Review 01 finding:

1. Use exact wallet RPC version `65567` and `release=true`. Implement the architecture
   decision's explicit distribution-proof plus authenticated-RPC-proof bridge to the
   accepted process readiness trait without claiming the CLI string came from RPC.
2. Give readiness the complete ten-second monotonic deadline. Retry only transient
   not-yet-listening failures; cap each attempt by the remaining deadline and the fixed
   per-operation maxima. Preserve `UNAVAILABLE`, `UNAUTH`, and
   `PROTOCOL_INCOMPATIBLE` distinctions. Do not busy-loop, widen the deadline, or adopt
   another child/endpoint.
3. Enumerate and type-check the exact pinned response members for every callable result,
   including nested members; discard nonreturned values only after validation. Unknown,
   missing, duplicated, confused, overflowing, or inconsistent fields still fail closed.
4. Apply only the ticket's node policy plus the architecture decision. Require hard-fork
   status `OK` and `untrusted=false`; do not gate valid nodes on target-height ordering,
   future earliest height, or enabled state.
5. Replace method-plus-empty-params dispatch with closed typed request values and exact
   serialization. Account index is zero where fixed; `create_address` uses count 1 and
   empty label; `validate_address` disables any-net/openalias; `query_key` can represent
   mnemonic only and never spend/view-key extraction. Secret-bearing params use
   zeroizing owners and never gain `Debug`.
6. Keep production transport numeric IPv4 loopback, synchronous, fresh-connection,
   bounded, no-DNS/proxy/redirect/TLS/generic-method, and safe-Rust only. Keep one
   unauthenticated plus one Digest-authenticated wallet exchange, exact parser bounds,
   and zeroize-enabled MD5 state.
7. Make recording support feed the same production request/framing/parser/policy core.
   Remove the hard-coded claims identified by review and make the wipe result account for
   every retained secret-bearing observation.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit evidence or
governance, or begin Slice 4, Slice 5, broader acceptance, or the real local-Monero gate.

Stop after the exact five-path correction. Report changed paths, line counts/hashes and
map each review finding to the corrected test and production paths. Report the exact
pinned source constants/shapes used, readiness deadline/error behavior, typed request
inventory, production-to-recording delegation, and secret-observation cleanup. Reviewer
XHigh acceptance is required before Hermes may execute or integrate.
