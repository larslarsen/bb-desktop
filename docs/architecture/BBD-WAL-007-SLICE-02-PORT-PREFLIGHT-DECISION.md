# BBD-WAL-007 Slice-2 Port Preflight Decision

Decision: **PREFLIGHT THE RANDOM PORT, SPAWN IMMEDIATELY, AND DO NOT CLAIM AN
ATOMIC LISTENER HANDOFF**

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Reviewed source drop:

- `wallet-broker/src/xmr/process.rs` — 980 lines — SHA-256
  `d5069097b835d5a69f19da22ac5b5ec0af85c8202844db5fc449d575ccc64673`

## Finding

The original Slice-2 handoff required the broker's reserved `TcpListener` to remain
owned until a safe child-bind handoff. The pinned `monero-wallet-rpc` interface cannot
accept an inherited/pre-bound listener. It accepts `rpc-bind-port` as text and passes
that value into its own HTTP server initialization:

- <https://github.com/monero-project/monero/blob/v0.18.5.1/src/wallet/wallet_rpc_server.cpp#L2434>
- <https://github.com/monero-project/monero/blob/v0.18.5.1/src/wallet/wallet_rpc_server.cpp#L2904-L2919>

Keeping the broker listener open prevents the child from binding it; closing the
listener creates an unavoidable narrow race before the child owns the port. No allowed
safe-Rust API can atomically transfer this socket to the pinned external program. The
ticket itself requires random collision-bounded selection but does not claim socket
activation or an atomic transfer.

## Decision

The broker performs a real numeric-IPv4-loopback bind as an availability preflight for
each OS-entropy candidate. It retains that listener through all planning, private-file
write, and config synchronization work. At the typed spawn boundary it releases the
listener immediately before `Command::spawn`, with no intervening application work.

Readiness must check that the exact owned child is still alive both before and after the
authenticated exact-version response. Any bind loss, child exit, authentication error,
wrong version, malformed response, or timeout performs the full closed teardown. The
random port and credentials remain undisclosed. A narrow local race remains and is
recorded as fail-closed availability risk; it is not described as atomic reservation or
socket ownership transfer.

No helper, shell, inherited descriptor protocol, FFI, libc, `unsafe`, dependency, or
nightly feature is authorized.
