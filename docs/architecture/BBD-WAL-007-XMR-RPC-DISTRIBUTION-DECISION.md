# BBD-WAL-007 XMR Wallet-RPC Distribution Decision

Date: 2026-09-01

Owner decision: **use the user's installed official Monero distribution and let the
BitBook wallet broker manage `monero-wallet-rpc`.**

## Product behavior

- BitBook does not bundle or silently download `monero-wallet-rpc` in v1.
- The user explicitly selects an official Monero installation or its
  `monero-wallet-rpc` executable. Normal and Monero GUI portable installations are both
  supported.
- BitBook verifies the selected executable against a reviewed platform/version/hash pin
  before every launch. Unknown, changed, missing, non-executable, or unmounted portable
  installations fail closed as XMR `UNAVAILABLE`; there is no PATH substitution,
  automatic download, or remote fallback.
- The wallet broker starts, authenticates, monitors, and stops the verified full
  `monero-wallet-rpc`. The user does not configure or start wallet RPC manually.
- Wallet RPC binds only to loopback with broker-controlled authentication and is never
  passed `--restricted-rpc` for a spending wallet. Its endpoint and credentials remain
  inside the broker boundary.
- The user separately owns and runs local `monerod`. BitBook does not bundle it, start a
  public node, or use Monero Simple/Bootstrap mode. v1 accepts no remote/bootstrap node
  fallback.
- Portable mode is treated only as an alternate user-selected executable/settings
  location; it does not change the local-only network policy. If its drive is absent, XMR
  is unavailable without affecting ZEC, social features, or stored BitBook state.

## Ticket consequence

The owner gate for BBD-WAL-007 is resolved. The implementation ticket may now specify
explicit selection, release/hash pins, executable validation, broker-only lifecycle and
authentication, local-node liveness, portable-drive removal, and fail-closed tests. Exact
supported Monero release hashes remain review artifacts for that ticket; no personal path
or machine-specific installation location belongs in product source.

The resulting implementation contract is `../../tickets/BBD-WAL-007.md`. It is ready
for a later bounded authorization but does not itself authorize source or execution.
