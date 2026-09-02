# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 3 RPC and Local Node

Status: AUTHORIZED — PRODUCTION SLICE 3 ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Accepted source baseline: `d0a14dd51d7e210ee7f1e2e96c9423447307b484`

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`tickets/BBD-WAL-007.md`, `wallet-broker/tests/xmr_rpc.rs`, Slice-2 Acceptance 01,
the complete accepted XMR production source, and `docs/handoff/CURRENT_TASK.md`.

## Objective and authorized paths

Implement only the bounded synchronous loopback HTTP/JSON-RPC transport, wallet Digest
authentication, closed typed RPC authority, process-control adapter, and local-node
policy proven by the frozen `xmr_rpc` test. Edit only:

- `wallet-broker/src/xmr.rs` — 4 lines, SHA-256
  `6c199bc807b16d80d19141dbe726d630dc6c2b89662253947ec280692af60fc6`;
- `wallet-broker/src/xmr/model.rs` — 143 lines, SHA-256
  `704540f27a1d92c6e82281db01e8689b3f0dbc99c3f066311a806e0ecab437b7`;
- `wallet-broker/src/xmr/rpc.rs` — new;
- `wallet-broker/src/xmr/test_support.rs` — 1,151 lines, SHA-256
  `5946ce53e5ddf0c1dbb64217019b90e0ba982b35c1b0a245ff45aa7079f39526`.

Every other path and repository is read-only. In particular do not edit `process.rs`,
distribution/native/UI, manifest, lockfile, tests, policy, evidence, governance, ZEC,
vault/session/store, Electron, protocol, supervisor, renderer, workflows, packages,
`bb-go`, or `go-ipfs`. Do not declare placeholder account, store, or receiver modules.
If the accepted `WalletRpcControl` boundary cannot be implemented without changing
`process.rs`, stop and report the exact architecture blocker; do not widen the drop.

## Module and process integration boundary

- `xmr.rs` adds only `pub mod rpc;` to the accepted module inventory.
- Preserve accepted distribution and process behavior byte-identically.
- Implement a production RPC control that satisfies the accepted
  `process::WalletRpcControl`: readiness performs authenticated `get_version` against
  the supplied random loopback port and credentials; later `close_wallet` and
  `stop_wallet` use that same authenticated owned-child session; `close_sockets` drops
  every connection and wipes retained session authentication state.
- The RPC layer may retain only the state required to address the exact broker-owned
  child. It must not accept a URL, hostname, path, proxy, remote endpoint, arbitrary
  method string, caller-supplied Authorization value, or caller-selected request ID.
- Public types and errors expose no endpoint, port, credential, challenge, request or
  response bytes, raw upstream text, or other sensitive transport state through
  `Debug`, `Display`, error chains, or logs.

## Closed synchronous transport

Use safe Rust and the existing dependencies only. Production opens a fresh synchronous
`std::net::TcpStream` to numeric IPv4 `127.0.0.1` for each request, with exactly a
two-second connect timeout and five-second read and write timeouts. There is no URL
parser, DNS, proxy, TLS, redirect, connection pool, chunking, keep-alive, generic request
method, or alternate endpoint.

Request JSON bodies are at most 16 KiB. The complete response headers plus body are at
most 64 KiB. Emit fixed `POST /json_rpc HTTP/1.1`, a numeric loopback Host value,
`Content-Type: application/json`, exact `Content-Length`, and `Connection: close`.
Accept only HTTP/1.1, one canonical valid Content-Length, and one exact
`Connection: close`; reject missing, duplicate, conflicting, folded, control-bearing,
or unsupported headers, every Transfer-Encoding, trailing bytes, 3xx, HTTP/1.0, and
unrecognized status before returning data. Close the socket on every success, error,
timeout, and unwind path.

JSON must be UTF-8 without BOM, have maximum nesting 16 and no trailing bytes, and reject
duplicate object keys at every depth. Requests use JSON-RPC `2.0`, an internally fixed
exact request ID, and one typed method. Responses require exact `jsonrpc` and ID and
closed typed result/error shapes; reject missing, duplicate, type-confused, overflowing,
unsupported, or simultaneous result/error shapes. Never return or log raw bytes or raw
upstream error text.

## Digest authentication and secret lifetime

Wallet calls perform exactly one unauthenticated request followed by at most one
authenticated retry. Parse one bounded `WWW-Authenticate: Digest` challenge containing
exactly one quoted realm, one quoted nonce, `qop=auth`, absent or exact `MD5` algorithm,
and an optional bounded quoted opaque. Reject duplicate keys, unknown keys, bad quoting,
lists containing unsupported qop values, `auth-int`, unknown algorithms, stale loops,
multiple challenges, and a second 401.

Use the exact `md-5 0.11.0-pre.4` dependency only for RFC Digest interoperability.
Calculation is fixed to the typed request's `POST` method and `/json_rpc` URI; nonce
count starts at `00000001`. Generate every cnonce from 16 fresh bytes of OS entropy and
encode it as 32 lowercase hexadecimal characters. HA1/intermediate digest bytes,
password, cnonce source and text, realm, nonce, opaque, challenge storage, and complete
Authorization bytes are non-`Debug` and zeroized on success, error, and unwind. Do not
clone secrets into ordinary strings beyond an unavoidable bounded construction whose
owner is immediately zeroized.

Node requests never send Authorization. Wallet requests never accept an authenticated
success without the required two-request Digest exchange.

## Closed RPC and local-node policy

The production authority is a typed enum or typed methods with exactly these wallet
operations: `get_version`, `create_wallet`, `restore_deterministic_wallet`,
`generate_from_keys`, `open_wallet`, `close_wallet`, `stop_wallet`, `query_key`,
`refresh`, `get_height`, `get_balance`, `get_address`, `create_address`, and
`validate_address`. Node authority is exactly `get_info` and `hard_fork_info`. No public
or crate-visible raw method-string/JSON entry point may bypass phase-bound typed calls.
There is no transfer, sweep, describe, sign, submit, relay, proof, key-image,
multisig, address-book, mining, daemon-switch, or future-method route.

Construct the node endpoint internally from the accepted `XmrNetwork`: stagenet uses
only `127.0.0.1:38081`, testnet only `127.0.0.1:28081`, and `xmr-mainnet` returns
`NETWORK_DISABLED` before socket or other side effects. Probe exactly `get_info` and
`hard_fork_info`; never try another endpoint.

Accept `get_info` only when status is exactly `OK`, `nettype` and mainnet/stagenet/
testnet booleans are mutually consistent with the selected network, `offline=false`,
`untrusted=false`, and `bootstrap_daemon_address` is empty. Historical
`was_bootstrap_ever_used=true` alone is accepted. Require nonnegative, internally
consistent bounded heights across both typed node responses. A valid local node with
`synchronized=false` returns `NODE_SYNCING`. Authentication demand, redirect,
bootstrap/remote state, wrong network, malformed/oversized response, inconsistent
heights, or connection failure returns `NODE_UNAVAILABLE` without fallback or partial
state.

## Test-support constraint

Extend `xmr/test_support.rs` only with the recording stream/entropy ports, typed fixtures,
fault controls, sanitized observations, `RpcTransportRig`, and `NodeProbeRig` consumed by
the frozen test. Both rigs must drive the same production framing, parsing, Digest,
typed dispatch, and node-policy code. Fakes may inject byte streams, elapsed boundaries,
entropy, and the exact named faults, but may not duplicate acceptance logic, precompute
production decisions, branch on test names, hard-code success flags, open real sockets,
sleep, or substitute a second transport implementation.

The independent RFC response value and node network matrix remain test-owned oracles.
Test-only accessors may expose sanitized copied observations needed by assertions; they
must not widen production authority or make secret-bearing production values `Debug`.
Panic-unwind exercise must traverse the same secret owners and cleanup mechanism as
success and error.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, push, edit evidence, or
begin Slice 4, Slice 5, broader acceptance, or the real local-Monero gate.

Stop after the exact four-path source drop. Report changed paths, line counts and
SHA-256 hashes; the production-to-recording-port delegation shape; `WalletRpcControl`
integration; typed method ownership; socket/framing/parser bounds; Digest secret
ownership and wipe points; node-policy mapping; and confirmation that no prohibited path
or command was used. Reviewer XHigh source acceptance is required before Hermes may run
the focused formatter, falsification, and green gate.
