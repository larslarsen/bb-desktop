# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 2 Process

Status: AUTHORIZED — PRODUCTION SLICE 2 ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Accepted source baseline: `c139641ab59f50d931723e3a8a463d7de17aa1b7`

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`tickets/BBD-WAL-007.md`, `wallet-broker/tests/xmr_process.rs`, Slice-1 Acceptance 01,
the complete accepted Slice-1 source, and `docs/handoff/CURRENT_TASK.md`.

## Objective and authorized paths

Implement only the process-plan and wallet-RPC lifecycle boundary proven by the frozen
`xmr_process` test. Edit only:

- `wallet-broker/src/xmr.rs` — 3 lines, SHA-256
  `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b`;
- `wallet-broker/src/xmr/model.rs` — 93 lines, SHA-256
  `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47`;
- `wallet-broker/src/xmr/process.rs` — new;
- `wallet-broker/src/xmr/test_support.rs` — 368 lines, SHA-256
  `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7`.

Every other path and repository is read-only. In particular do not edit distribution,
native/UI, manifest, lockfile, tests, policy, evidence, governance, ZEC, vault/session/
store, Electron, protocol, supervisor, renderer, workflow, package, `bb-go`, or
`go-ipfs` source. Do not declare placeholder RPC, account, store, or receiver modules.

## Module and model boundary

- `xmr.rs` adds only `pub mod process;` to the accepted Slice-1 inventory.
- Add the closed `XmrNetwork` and stable error constructors needed by this slice without
  mainnet success, caller URLs, raw strings, paths, credentials, or upstream text in
  public errors or `Debug`.
- Keep the accepted distribution API and all Slice-1 behavior byte-identical.

## Production process plan

The plan must consume an already verified executable capability from the distribution
boundary; no production constructor accepts an arbitrary executable path or treats a
boolean as verification. The personal program path may be used only as the OS program.
Fix Unix `argv[0]` to `monero-wallet-rpc`; argv contains exactly
`--config-file=wallet-rpc.conf`. Clear the environment and set only `LANG=C`. The path,
credentials, config text, endpoint/port, and child identity must have no `Debug`, log,
diagnostic, error, argv-vector, or environment disclosure.

Validate the 32-character lowercase-hex account ID and closed stagenet/testnet network
before directory, entropy, listener, config, or process effects. Mainnet fails
`NETWORK_DISABLED`; every other invalid value fails `SCHEMA`.

Derive runtime, wallet, shared-ring, config, and bounded-log paths internally from the
validated account/network beneath a caller-supplied already-private broker root. No
caller path becomes a filename. Directories are `0700`; config/log files are `0600`.
The private config contains exactly the ordered keys and values frozen in
`xmr_process.rs`, including one network flag, numeric IPv4 loopback, full authenticated
wallet RPC, fixed local daemon port, bounded connections/response/logs, and no extra
option. Sync the config before spawn. Do not add `restricted-rpc`, disable login, IPv6,
CORS, trusted/remote daemon, proxy, notification, detach, pidfile, wallet-file,
password-in-argv, hardware, mainnet, or caller-provided config authority.

Generate fresh login username/password and port candidates through real OS entropy using
the existing `getrandom` dependency. Secrets are nonempty, distinct, short-lived,
non-`Debug`, zeroized on every success/error/unwind/teardown path, and never stored in a
copyable public view. Select only ports 49152–65535, validate scripted values before any
bind, and attempt at most 16 collisions. A real reserved listener remains owned until a
safe child-bind handoff; do not claim reservation after dropping it prematurely.

## Lifecycle boundary

Use one centralized production plan/lifecycle state machine behind typed ports for
clock, entropy/listener, private filesystem, verified spawn, authenticated readiness,
wallet close/stop, child wait/kill/reap, socket ownership, and isolation observation.
The future RPC slice may implement its typed readiness/stop adapter; this slice adds no
HTTP, JSON, Digest calculation, raw method, endpoint parsing, DNS, proxy, redirect,
network discovery, or fake success branch.

Enforce one child per account and a global maximum of four before child creation.
Startup accepts only authenticated exact-version readiness at or before 10 seconds;
connect timeout is two seconds. Every config-write/sync, spawn, authentication, version,
malformed-readiness, executable-change/removal, unexpected-exit, and broker-exit path
must close handles, reap the exact owned child, wipe both credentials, and remove
runtime secrets.

Graceful teardown order is exactly: stop new calls; close wallet where possible; request
wallet stop; wait at most two seconds; kill the exact owned child/process group only if
needed; reap; wipe RPC login; wipe wallet password; close sockets; remove runtime
secrets. Use safe Rust and no helper process, shell, FFI, libc, or `unsafe`. Do not claim
process-group semantics that the production port does not enforce; if the safe existing
dependency/toolchain boundary cannot meet the ticket, stop and report that architectural
blocker instead of weakening or faking it. One XMR failure must never stop ZEC or the
social supervisor.

## Test-support constraint

Extend `xmr/test_support.rs` only with the recording ports, typed faults, controlled
clock/entropy/child outcomes, sanitized plan view, and `ProcessRig` consumed by the
frozen test. The rig must delegate validation, plan construction, port-attempt bounds,
startup, pool limits, teardown decisions/order, wipe/cleanup, and isolation to the same
production state machine. It may inspect secrets only through explicit test accessors
that do not add `Debug` or runtime disclosure.

Do not hand-build an expected plan in test support, duplicate lifecycle algorithms,
hard-code success booleans, sleep for real time, spawn a real child, open a real socket,
or make the test pass without exercising production branches. Named faults may only
inject the exact frozen failure points.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit evidence, or
begin Slice 3.

Stop after the exact four-path source drop. Report paths, line counts/hashes, the
production-to-recording-port delegation shape, verified-executable consumption, entropy/
reservation design, teardown ownership, and confirmation that no prohibited path or
command was used. Reviewer XHigh acceptance is required before Hermes may execute the
focused green gate.
