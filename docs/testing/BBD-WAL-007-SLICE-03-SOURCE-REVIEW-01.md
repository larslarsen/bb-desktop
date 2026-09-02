# BBD-WAL-007 Slice-3 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **REJECTED — TEST/PRODUCTION CORRECTION REQUIRED**

No formatter, test, build, policy command, or product binary was run. `git diff --check`
was clean, the pre-drop mutable hashes matched the handoff, and the worktree contained
exactly the four authorized source paths.

Reviewed source identities:

- `wallet-broker/src/xmr.rs` — 5 lines —
  `92af416c0ed6c5f2f8a6ff7011facb622da5e3f1ba8adaa2e96f600c09f31411`;
- `wallet-broker/src/xmr/model.rs` — 151 lines —
  `23a85dc216839cb936161845b52cacdca68a24181e5d31afeaaba30e62c77ca9`;
- `wallet-broker/src/xmr/rpc.rs` — 1,441 lines —
  `7f4b8a194cd7b351883369192589ef68a15758ff7849c7b1dbbb4b311f009aa7`;
- `wallet-broker/src/xmr/test_support.rs` — 2,154 lines —
  `a019c4800df0a4c819619c6f41f66cf56d86011c85b3c4bbac64dae10ae1eba9`.

The frozen `xmr_rpc` test remained 422 lines, 15 tests, SHA-256
`0046a94d8a3f7932c02e872f90afdcd8e0a79641f3b87db6cac4e2db25311b86`.

## Blocking findings

1. Production and the fixture require wallet RPC version `196610`. Pinned Monero
   v0.18.5.1 source defines major 1, minor 31, hence exact value `65567`. The current
   code rejects the reviewed executable it is intended to launch.
2. The closed daemon parsers omit real pinned members including `credits` and `top_hash`.
   The fake omits those same members, so it does not exercise the real upstream shape.
   Wallet `get_balance` and `create_address` similarly reject documented serialized
   members beyond the synthetic reduced results.
3. The node policy rejects valid local nodes by requiring `height <= target_height` and
   `hard_fork.enabled=true` with `earliest_height <= height`. Official semantics permit
   `target_height=0` when synchronized, a target below current height, a future earliest
   fork height, and a not-yet-enabled fork. Conversely, hard-fork `untrusted` is parsed
   neither as required nor as a rejection condition.
4. System readiness makes one immediate connection attempt, so a normally starting
   child can be killed before it binds. It does not own the required ten-second startup
   retry/deadline behavior and it converts all connection/auth/protocol errors into
   `PROTOCOL_INCOMPATIBLE`.
5. `ReadinessStatus.version` is filled with the CLI-version constant after parsing the
   wrong numeric RPC value. The upstream method cannot return that string. The correction
   must use the explicit two-proof bridge in the accompanying architecture decision and
   must not describe the string as an upstream result.
6. Every RPC request currently serializes empty `{}` params. Crate-visible enum dispatch
   therefore purports to implement account/receiver methods without their required typed
   phase inputs. Define closed typed request payloads, or keep a method unreachable until
   its typed payload exists; no generic method-plus-empty-params route is accepted.
7. Several negative/hygiene observations are hard-coded (`invoke_unlisted_for_test`,
   `returned_bytes`, DNS/proxy answers), and the wipe audit counts only selected
   `SecretBytes` drops while authorization observations retain ordinary strings. Replace
   claims with observations derived from the production core/recording port and clear or
   zeroize retained secret observations before reporting hygiene success.

The numeric-loopback socket type, bounded body/response framing, strict JSON pre-parser,
one-retry Digest calculation, zeroize-enabled MD5 core, and shared generic
production/recording `RpcCore` are retained. A full rewrite is not required.
