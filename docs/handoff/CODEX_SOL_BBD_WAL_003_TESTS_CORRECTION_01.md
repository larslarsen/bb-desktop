# Codex Sol Handoff — BBD-WAL-003 Test Source Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `dc898d4640a795f927fa84341b4bf4b5470851db`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`tickets/BBD-WAL-003.md`, `docs/architecture/BBD-WAL-001-REVIEW.md` §§4.2–4.3 and
5.3–5.4, `docs/handoff/CURRENT_TASK.md`, the original Sol handoff, and all six current
test-drop paths.

The first test drop is reviewer-rejected before execution. Preserve its valuable
coverage and correct only the following defects. Modify only the same six authorized
paths:

- `test/fixtures/wallet-broker/transcript-v1.json`
- `test/walletBrokerProtocol.node.js`
- `test/walletSupervisor.node.js`
- `test/walletPreload.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

## Required corrections

1. **Version overlap, not exact range equality.** Child hello `{min:1,max:2}` has a v1
   overlap and must validate with negotiated version 1. `min:2,max:2`, reversed ranges,
   zero/negative/non-integer bounds, and no-overlap ranges fail. `hello_ack.version`
   remains exact integer 1. Do not reserve a protocol that rejects a future-capable child
   merely because its maximum is above 1.
2. **Real supervisor transcript binding.** Replace the synthetic
   `{kind:'hello',payload:'fixture'}` acceptance. The harness child has an injected PID,
   the supervisor has an injected parent PID and deterministic cryptographic nonce source,
   and it receives the complete fixture `hello`. Prove the child PID in hello equals the
   spawned child PID, the emitted `hello_ack` contains the actual parent PID and injected
   parent nonce, the computed session ID equals the fixture, and application dispatch is
   unavailable until the required first post-hello session binding completes. Wrong
   child PID, wrong nonce/session, hello on diagnostics, timeout, and child exit before
   binding all fail closed without dispatch. Include successful parent and child binding,
   not only a mismatch.
3. **Data-directory and launch order.** Prove the exact data directory is created or
   verified as a non-symlink directory with mode `0700` before broker verification;
   broker `lstat`, readability, SHA-256, and only then one spawn follow. Missing/wrong-mode
   or symlinked data directories and every existing binary failure cause zero spawn.
   Spawn still has empty argv, cleaned environment, distinct protocol/diagnostic pipes,
   no shell/listener, and no secret.
4. **Closed method-specific parameter schemas.** Do not assert that all broker methods
   accept `{}`. Table-drive positive and negative shapes. Empty params are valid only for
   `status.get`, `account.list`, and `sync.subscribe`; `account.lock` requires one 32-hex
   `account_id`; `receiver.fresh` requires exactly `account_id`, `asset`, `network`, and
   32-hex `request_id` with consistent asset/network; `intent.begin` requires exactly one
   non-null plain-object `payment_request` within the control limit; and `intent.cancel`
   requires one 32-hex `intent_id`. Missing, extra, inherited, accessor, wrong-type,
   invalid-enum/ID, array, and oversize inputs fail before send. The inner signed-request
   cryptographic schema remains broker-owned; Electron must not reimplement JCS.
5. **Non-vacuous lifecycle and sanitization.** Quit must be observed on the injected
   transport: exact `intent.cancel` sends precede the child termination call. Do not prove
   this solely through an implementation-owned lifecycle log. Snapshot tests must retain
   one fully valid public account/sync/device/capability shape while removing unknown and
   secret-bearing top-level and nested fields, functions, accessors, receivers, backup,
   RPC, and raw data. Inputs must be cloned and must not share nested references.
6. **Fixed safe error messages and stricter JSON.** A recognized error code paired with a
   canary/path/backtrace message must return that code with a fixed reviewer-known safe
   message, not echo the input. Unknown code still normalizes to `INTERNAL`. Add duplicate
   JSON-name rejection before ordinary parsing and prove the decoder stays terminal.
7. **Preload positive and negative boundary.** Align valid payloads with item 4. Prove
   no-argument methods do not smuggle a payload, every argument method clones one fixed
   shape, invalid callback types register no listener, repeated unsubscribe is inert, and
   the exposed object has no inherited bridge properties (use a null prototype or an
   equivalently tested own-property-only boundary). Keep hostile callback containment and
   event stripping.
8. **Electron positive dispatch and subscription.** The current test only proves invalid
   calls reject, so an implementation that rejects everything could pass. Inject/mock the
   supervisor boundary and prove one valid call for each of the five invoke channels
   reaches exactly its fixed supervisor method with cloned validated params. Prove a
   sanitized snapshot subscription uses only `wallet:snapshot:subscribe`, targets only
   the maintained main frame, and carries no Electron event/secret. Keep wrong frame,
   origin, shape, size, and every authority-bearing channel rejection.
9. **Exact package/CI mutation tests.** Follow the existing wallet-contract convention:
   require one named `test:wallet-broker` script containing the three exact Node commands,
   require the top-level test and routine Social workflow to invoke that named script,
   and mutate/remove the script, every syntax command, top-level inclusion, workflow
   filter, and workflow invocation to prove the production checker rejects each case.
   Do not force three redundant public npm script names or three unrelated CI steps.
10. **Path-specific least privilege.** The source-policy test must use separate exact
    allowlists: protocol may load only its reviewed pure built-ins/siblings; supervisor
    may additionally load `fs`, `path`, and `child_process` for exact `spawn`; preload may
    load only `electron`. Prove `child_process`/`fs`/`electron` are rejected from the wrong
    boundary path, computed/dynamic imports fail, `exec`/`execFile`/`fork`, `shell:true`,
    inherited secret environment, generic argv tokens, listeners (`net`, `http`, UDS,
    named pipe, TCP), generic IPC channels/method strings, provider fetch/WebSocket,
    worker/device modules, and quote/wallet raw proxies are rejected. A source checker
    that merely exports constants or is a no-op must not pass.

Keep the independent fixture digest and every pre-existing accepted Electron/policy
assertion. Tests must remain deterministic, Node-built-in-only, and behavior-oriented
except for the repository-policy assertions. Do not add production or broaden the ticket.

Use `apply_patch`. Read-only inspection and final `wc -l`/`sha256sum` over the six paths
are allowed. Do not execute Node, npm, tests, builds, formatters, scanners, Git, GitHub,
network, Electron, child processes, wallets, nodes, hardware, or devices. Do not install
anything or use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or unresolved
destructive targets.

Stop after authoring. Report corrected/added test names, totals, reserved CommonJS API,
non-vacuity, expected red causes, all six line counts/hashes, preserved assertion counts,
and confirmation that nothing ran. Reviewer XHigh must accept the corrected source before
Codex Luna executes it.
