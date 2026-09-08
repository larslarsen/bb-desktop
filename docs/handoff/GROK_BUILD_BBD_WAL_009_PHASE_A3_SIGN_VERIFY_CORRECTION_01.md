# Grok Build Handoff — BBD-WAL-009 Phase A3 Sign/Verify Correction 01

You are **Sr Dev — Grok Build** at High. Repository:
`/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

The owner reports Grok usage is available again. This handoff supersedes the stopped
Sol production authorization. Read completely: `AGENTS.md`, `TESTING.md`, both
role/routing policies, `docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-009.md`, the
Phase-A1 test/format/expected-red/acceptance records, the Phase-A3 Sol handoff and usage
stop rejection, the accepted `zec_sign_verify` test, all seven partial source paths,
the WAL-006 and WAL-008 acceptance records, and the pinned local librustzcash/PCZT
sources.

## Frozen start and protected concurrent work

The seven rejected partial paths are exactly:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` |
| `wallet-broker/src/zec/prepare.rs` | 1,171 | `1a97332e9c4e714833dcda1f487d0da20c371f21ee5375fb4b100eaa0947f642` |
| `wallet-broker/src/zec/spend.rs` | 530 | `0b9a7e4c30afb85558f480d7c177f2732e53755b260ad78c8ee51763948c03cb` |
| `wallet-broker/src/zec/store.rs` | 2,872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` |
| `wallet-broker/src/zec/test_support.rs` | 4,136 | `eac4d2ec63a140267bb23706fc6fc675a0fb820106ffebec042bc156f5dc2627` |
| `wallet-broker/src/native.rs` | 435 | `7f64822feae9eb3cb066ac834a769afff2b2dac9644542f4cb94f543ba3df029` |
| `wallet-broker/src/native_ui.rs` | 178 | `a8a8d2486e453ccd90d5d868a45e2778fb93896e4e0275277611019c81462ae2` |

Preserve these unrelated worktree files byte-for-byte and do not stage them:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `package.json` | 42 | `84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780` |
| `package-lock.json` | 396 | `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc` |

Also preserve the accepted manifest and test exactly:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,115 | `80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd` |

## Authorized correction paths

Edit only the same seven Rust source paths above. You may rewrite or remove rejected
Sol code within them. Do not edit packages, manifest, lockfile, tests, fixtures,
governance, policy, workflows, Node/Electron source, or any other path.

Implement the full Phase-A3 boundary and all 14 accepted tests exactly as specified by
BBD-WAL-009 and the original Sol handoff, with these mandatory corrections:

- Every prerequisite, mutation, concurrency, cancellation/expiry, fault, panic,
  replacement, broker-exit, hardware contribution, and cleanup helper must drive the
  real production state and operation boundary. Do not map an enum directly to an
  expected error, run an unmodified pipeline and return the desired code afterward, or
  increment a detached observation merely to satisfy an assertion.
- Mutation hooks alter one isolated authoritative input/artifact/signature/proof field
  before the real independent verifier. The verifier—not the hook—must select
  `INTENT_MISMATCH`, `SIGNATURE_INVALID`, `STATE_CORRUPT`, or `SCHEMA` according to the
  frozen precedence.
- Do not emit literal or self-reported proof/signature validity. Verify the Ironwood
  proof, spend-authorization signatures, and binding signature cryptographically using
  pinned library primitives independent of the signing path. If the pinned APIs cannot
  support an honest independent check, stop and report that exact blocker instead of
  claiming success.
- `TransactionExtractor` is an extractor, not proof that cryptographic verification
  occurred. Decode the extracted transaction independently, derive its real txid, and
  compare actual transaction/bundle effects to the frozen intent. Destroy the raw
  transaction buffer immediately afterward; retain only the bounded structured
  observation and txid. No verified/public structure may own or clone raw bytes.
- The production native UI must actually render the complete immutable review and mint
  its non-cloneable, non-serializable, one-shot capability only from the affirmative UI
  event bound to that exact review. An unused tuple and pre-set boolean are not a
  production confirmation surface. Synthetic minting stays isolated behind fixture
  types and cannot accept caller-provided origin strings.
- Production hardware remains empty and fails before any signer view. Synthetic
  Keystone-v2 contributions are cryptographically verified for route, batch, intent,
  pool, index, randomized key, ordering, replay, and signature before application to
  the retained PCZT. Never hand a PCZT to or accept one from the external signer.
- Actual resource owners generate wipe/drop observations. Canary values must enter
  those owners before the exercised path, while captured errors, panic text, logs,
  diagnostics, persistence, and JSON come from the real path. Constant empty or
  redacted stand-ins do not prove non-disclosure.
- The per-account gate must surround the real prepare/sign/verify operation and release
  by RAII across every terminal edge. Pre- and post-sign cancellation, exact expiry,
  capability/device/session/account ownership, and review bindings must be reread at
  the stated barriers.
- Capability and operation inventories must be derived from the actual reachable
  registry, not hard-coded solely for the test. Preserve zero network, broadcast,
  mainnet, XMR, real-device, export, and Electron authority.

Use no custom transaction/signature codec and add no dependency or feature. Preserve
all WAL-006/WAL-008 behavior. Keep the implementation as small as an honest solution
allows; do not add parallel test-only business logic.

## Stop boundary

Use read-only inspection only. Do not run Rustfmt, Cargo, compiler, tests, Clippy,
audit, scanner, dependency/product command, Git, network, wallet/node process,
hardware/device action, or another actor. Stop after writing only the seven authorized
paths. Report their exact line counts and SHA-256 identities, the four protected
manifest/test/package identities, and confirmation that nothing executed. Reviewer
source inspection precedes any formatter or test command.
