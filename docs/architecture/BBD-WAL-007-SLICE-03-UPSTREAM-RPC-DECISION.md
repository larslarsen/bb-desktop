# BBD-WAL-007 Slice-3 Upstream RPC Decision

Date: 2026-09-02

Decision: the pinned v0.18.5.1 wallet-RPC and daemon serialization definitions, not a
synthetic fixture, define Slice 3's accepted wire shapes.

Authoritative pinned source:

- <https://github.com/monero-project/monero/blob/v0.18.5.1/src/wallet/wallet_rpc_server_commands_defs.h>
- <https://github.com/monero-project/monero/blob/v0.18.5.1/src/wallet/wallet_rpc_server.cpp>
- <https://github.com/monero-project/monero/blob/v0.18.5.1/src/rpc/core_rpc_server_commands_defs.h>
- <https://docs.getmonero.org/rpc-library/monerod-rpc/>

## Wallet readiness identity

The pinned wallet source defines `WALLET_RPC_VERSION_MAJOR=1` and
`WALLET_RPC_VERSION_MINOR=31`; the exact numeric `get_version` value is therefore
`65567`, and `release` must be `true`. `196610` is not the pinned wallet-RPC version.

The authenticated `get_version` response does not return the CLI product-version text.
Distribution verification already proves the exact selected executable bytes and exact
bounded `--version` output before launch. Process readiness may translate an
authenticated exact `(version=65567, release=true)` result into the accepted process
trait's `VERIFIED_VERSION` sentinel only because the process consumes that verified
executable capability. The code and evidence must describe this as a conjunction of the
pre-launch distribution proof and post-launch RPC-protocol proof; it must not claim that
wallet RPC returned the CLI string.

Readiness owns the ticket's ten-second startup window. A not-yet-listening child is
retried within that deadline with the fixed two-second per-connect maximum. The deadline
must include complete attempts and never expand because one connect/read/write timeout
started near its end. Connection failure remains `UNAVAILABLE`, malformed protocol
remains `PROTOCOL_INCOMPATIBLE`, and Digest rejection remains `UNAUTH`.

## Pinned response shapes

Closed typed parsing means the reviewed v0.18.5.1 serialized member inventory is
enumerated and type-checked, including members the broker does not return. It does not
mean deleting documented members from fixtures or rejecting every real response.
In particular:

- wallet `get_balance` includes the pinned top-level balance metadata and
  `per_subaddress` members, even though the broker returns only checked total and
  unlocked atomic values;
- wallet `create_address` includes both scalar and vector address/index members;
- daemon `get_info` includes `credits` and `top_hash`; and
- daemon `hard_fork_info` includes `credits`, `top_hash`, `untrusted`, and the other
  pinned voting/version members.

Every retained string or nested value still follows the ticket's no-raw-output and
secret-wipe rules. Unknown members outside the reviewed pinned inventory fail closed.

## Node policy

The ticket's explicit `get_info` rules govern acceptance: exact `OK`, exact network
booleans/nettype, `offline=false`, `untrusted=false`, empty current bootstrap address,
and internally valid unsigned heights. `height_without_bootstrap` may not exceed
`height`.

`target_height` is not ordered against `height`: official behavior permits zero for a
fully synchronized node and published examples contain a target below current height.
Likewise, `hard_fork_info.earliest_height` may describe a future fork and `enabled=false`
is not by itself a remote-node signal. Require an exact typed response, `status=OK`, and
`untrusted=false`; do not invent an earliest-height or enabled acceptance gate.

This decision changes no mainnet, remote/bootstrap, endpoint, method-authority, or error
policy. It corrects the fake oracle and production compatibility before any execution.
