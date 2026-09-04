# Codex Sol Handoff — BBD-WAL-009 Phase A3 Sign/Verify Production 01

You are **Principal Dev — Codex Sol** using `gpt-5.6-sol` at High. Repository:
`/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Grok remains owner-reported weekly-usage exhausted. This is the documented Sol
fill-in. Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-009.md`, the Phase-A1 source,
formatting, expected-red, and acceptance records, all seven authorized source files,
the accepted `zec_sign_verify` test, the WAL-006 prepare/store and WAL-008 hardware
acceptance records, and the pinned local `pczt 0.9.3`, `zcash_client_backend 0.24.0`,
`zcash_primitives 0.30.1`, and `zcash_keys 0.16.1` sources.

## Frozen accepted inputs

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,115 | `80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd` |
| `wallet-broker/src/zec.rs` | 253 | `6dd71f9f70d7b5b8aaddd2e3d4df2b9b2b232b45182ca1db4d32078146751fa2` |
| `wallet-broker/src/zec/prepare.rs` | 964 | `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e` |
| `wallet-broker/src/zec/store.rs` | 2,849 | `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |
| `wallet-broker/src/native_ui.rs` | 149 | `34fda529c4ac6035bb5147720f456a271145deb43878082fbdfe464d320a7bdf` |

## Authorized paths

Edit only:

- `wallet-broker/src/zec.rs`
- `wallet-broker/src/zec/prepare.rs`
- `wallet-broker/src/zec/spend.rs` (new)
- `wallet-broker/src/zec/store.rs`
- `wallet-broker/src/zec/test_support.rs`
- `wallet-broker/src/native.rs`
- `wallet-broker/src/native_ui.rs`

Do not edit the accepted manifest, test, lockfile, another test, dependency, fixture,
policy, workflow, Node/Electron source, documentation, or any other path.

## Implementation boundary

Implement the smallest production-facing core and typed test harness required by all
14 accepted tests. Do not satisfy them with a parallel fake state machine, fabricated
decoded transaction, self-reported cryptographic booleans, or counters detached from
the operations they claim to observe.

- `native.rs` owns an opaque process-local confirmation capability. Its constructor
  and fields are private, it is consumed by value, and it implements neither `Clone`
  nor serialization. Caller-supplied `ActionOrigin`, Electron/protocol strings, and
  reconstructed values cannot mint it. `native_ui.rs` is the only production minting
  path after displaying the exact frozen review; test support may invoke a separately
  unmistakable synthetic native-surface adapter.
- `prepare.rs` remains the sole owner of prepared handles and raw PCZT bytes. Add
  crate-private consume/inspect operations that revalidate the full handle, session,
  account, network, request, intent, review, fee, fee-bound, memo-hash, and expiry
  binding before exposing an artifact to `spend.rs`. Never clone raw PCZT or retain it
  after a terminal result. Wrong binding and exact-time expiry fail before PCZT access.
- Retain real unlocked software custody only in observed secret wrappers. Derive the
  unified spending authority in broker memory, derive its viewing key, and compare it
  to the account's stored viewing binding before parsing or signing the PCZT. Wrong
  seed/account/session/network and watch-only state fail closed.
- `spend.rs` owns the per-account authorization gate and the authorize/prove/finalize/
  extract/verify pipeline. Use only the pinned librustzcash PCZT roles and the real
  accepted local v6/Ironwood fixture: `Signer`, prover, spend finalizer, transaction
  extractor, and the canonical transaction decoder/txid implementation. There is no
  custom transaction or signature codec.
- The software signer receives authority for exactly the retained Ironwood action.
  No transparent, Sapling, Orchard, extra action, destination, value, fee, change,
  account, request, or intent substitution may reach signing.
- Independently decode the extracted authoritative transaction after signing. Compare
  actual v6/`37a5165b` network and bundle effects, exact external receiver bytes and
  amount, fee/bound, memo hash, expected internal change, pools, proof/signature
  validity, request and intent bindings, and a txid derived from the decoded
  transaction. Only then publish the bounded verified record. Do not retain or expose
  raw transaction bytes through the result or inspection oracle.
- Keep the WAL-008 positive production hardware table empty. Production hardware
  denial occurs before signer-view/PCZT export. The synthetic Keystone-v2 adapter is
  reachable only through the test fixture types, receives a bounded signer/batch view,
  and returns tagged Ironwood signatures. Apply contributions to the retained PCZT
  only after exact route, batch, intent, pool, action index, randomized key, cardinality,
  ordering, replay, and cryptographic signature checks. Never accept a returned PCZT.
- Read cancellation, expiry, capability/device/session/account ownership before secret
  work and again after signing/proving immediately before verified publication. The
  account gate blocks only the same account and releases on every success, error,
  cancellation, expiry, lock, panic unwind, replacement, and broker exit.
- Fault hooks and counters must wrap the real operation boundary. Secret observations
  must come from the actual `SecretBytes`/zeroizing owners. Positively wipe every
  mutable secret byte buffer and honestly distinguish third-party parsed objects that
  can only be promptly dropped. No secret, receiver, memo, PCZT, transaction, proof,
  key, or signature value may enter Debug/Display/error/panic/log/diagnostic/persistence
  or public JSON.
- Stable precedence is schema/state corruption first, then native/capability and
  ownership/session binding, then cancel/expiry, then cryptographic failure. Do not
  publish a verified handle on any failure.

The public operation and capability inventories remain exactly those asserted by the
accepted test. There is no socket, HTTP/gRPC client, endpoint, submit, confirmation
polling, broadcast, retry, filesystem export, real device transport, mainnet, XMR,
Electron method, or new dependency. Preserve every WAL-006/WAL-008 API and invariant.

## Stop boundary

Use read-only inspection only. Do not run Rustfmt, Cargo, compiler, tests, Clippy,
audit, scanner, dependency/product command, Git, network, wallet/node process,
hardware/device action, or another actor. Stop after the seven authorized paths are
written. Report every changed path with exact line count and SHA-256, every unchanged
frozen input, and confirmation that nothing executed. Reviewer inspection precedes
any formatter or green command.
