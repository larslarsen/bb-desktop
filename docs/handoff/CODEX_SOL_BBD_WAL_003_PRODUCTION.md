# Codex Sol Handoff — BBD-WAL-003 Production Boundary

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Accepted red baseline: `7a1bde2372fbd94a85efc42ec1919ac451e6797a`

Read the ticket, current task, architecture §§4.2–4.3 and 5.3–5.4, all BBD-WAL-003
test-source handoffs/evidence, and all five accepted suites before editing. Implement the
smallest dependency-free production boundary that makes the accepted behavior green.

You may create or edit only:

- `wallet-broker/protocol.js`
- `wallet-broker/supervisor.js`
- `wallet-preload.js`
- `social-main.js`
- `package.json`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`
- `scripts/security-policy.js`

All tests, fixtures, package lock, renderer source, architecture, evidence, documentation,
other workflows, build/package scripts, inherited OpenBazaar source, and other repository
paths are frozen.

## Required implementation

1. **Protocol module.** Export exactly the accepted public names. Implement strict
   four-byte big-endian UTF-8 JSON framing, fatal terminal decoders, 64-KiB control and
   1-MiB absolute limits, duplicate-key/trailing/malformed rejection, closed v1 hello and
   ack validation, exact session transcript hashing, bidirectional post-hello sequence,
   unique-ID, correlation, cancellation, deadline, direction, and session binding, plus
   safe stable error normalization. A rejected frame must not partially advance state.
2. **Supervisor module.** Export the exact broker-method list and accepted public names.
   Keep all OS boundaries injectable. Verify the private 0700 data directory, regular
   non-symlink readable broker file, and exact lowercase SHA-256 pin before one inert
   `spawn(path, [], { cwd, env, shell: false, stdio: ['pipe','pipe','pipe'] })`. Clean the
   environment to the accepted non-secret allowlist. Keep protocol and diagnostics
   separate, require child-first binding within two seconds, validate child PID/session,
   enforce the closed method/parameter schemas and 64-KiB limit before send, never buffer
   spend work, sanitize snapshots without invoking accessors, use bounded restart delays,
   and cancel tracked intents before termination. Provide bounded snapshot subscription
   for Electron main without an eager callback. A missing/unconfigured broker is only a
   sanitized down snapshot; it must not cause a real spawn in this slice.
3. **Preload.** Expose one null-prototype frozen `bitbookWallet` object containing exactly
   six frozen own functions in the tested order. Use only the six fixed channels. Deep
   clone arguments, results, and subscription values synchronously at the boundary; strip
   Electron event objects; reject invalid callbacks before listener registration; make
   unsubscribe idempotent and bounded; swallow hostile callback exceptions. Expose no
   generic IPC, Node/Electron/process handle, confirm, unlock, backup, sign, or broadcast.
4. **Electron main.** Preserve every sandbox, navigation, permission, CSP, local-page, and
   single-window invariant. Add the explicit local preload and only the five invoke
   handlers plus one outbound snapshot channel. Check exact main frame and local origin,
   own data-property/closed shapes, nesting, and byte limits before dispatch without
   invoking accessors. Deep-clone each input and result, map channels to fixed supervisor
   methods exactly once, and send only sanitized cloned snapshots to the maintained main
   frame. No confirmation window/channel/callback exists. Construct the real supervisor
   as inert/down when no later packaging pin exists; do not invent a pin, binary, or
   dependency and do not make the social app require a wallet to start.
5. **Package and CI.** Add exact `test:wallet-broker` commands, append it to the exact
   top-level test order, add syntax checks for the three boundary sources, add
   `wallet-broker/**` and `wallet-preload.js` to every routine Social/Security path filter,
   and run `npm run test:wallet-broker` in the package-free Social check. Do not alter the
   lockfile, Electron version, audit/SBOM/scanner pins, manual-only packaging, or add a
   dependency/artifact/build matrix.
6. **Fail-closed policy.** Mirror the exact new path, command, build, import, channel, and
   workflow constants from the accepted policy tests. Extend package/workflow/repository
   checks and exports. Scan all three maintained boundary sources with path-specific
   literal import allowlists. Forbid computed/dynamic imports, network/listener creation,
   shell or synchronous/exec/fork process APIs, inherited stdio/environment, secret argv,
   generic or authority-bearing IPC/method strings, and unknown boundary paths. Preserve
   all 54 inherited security-policy behaviors and the existing wallet-contract policy.

Use Node built-ins only and `apply_patch` for all source edits. Do not implement a wallet,
native-confirm UI, Rust crate, coin adapter, rate worker/provider, transaction, custody,
key, node, hardware/device integration, TCP/HTTP/UDS/named-pipe listener, real broker
binary, packaged pin, or fallback to inherited marketplace wallet code.

Do not execute Node, npm, tests, builds, formatters, scanners, Electron, Git, GitHub,
network, child processes, wallets, nodes, hardware, or devices. Do not install anything or
use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or unresolved destructive
targets. Stop after authoring. Report the eight changed paths with line counts and
SHA-256, the implemented boundary, any residual uncertainty, and confirmation that
nothing ran. Reviewer XHigh must accept the source before Codex Luna executes it.
