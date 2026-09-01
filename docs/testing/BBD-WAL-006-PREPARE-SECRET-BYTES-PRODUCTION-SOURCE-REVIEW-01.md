# BBD-WAL-006 Prepare Secret-Bytes Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `4237069f`

Result: **ACCEPTED — HERMES GREEN/REGRESSION GATE AUTHORIZED**

Sol changed exactly one authorized production path:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 773 | `500cd2f91ec0a2e0052779ba6b2357053ce0bea1d644fb2c35066f768f363fe0` |

`SecretBytes` now owns only the existing `SecretSlice<u8>` and retains construction, exposure,
replacement wipe, explicit wipe, redacted formatting, and unconditional zeroizing Drop.
`new_observed` returns the new public `ObservedSecretBytes`, which owns the inner secret, static
label, and unchanged unconstrained observer. Its Drop calls `wipe_with` once; that replaces the
inner allocation with an empty secret before notifying the observer, so the later inner Drop
zeroizes only the empty replacement and cannot emit a duplicate observation.

The wrapper's Debug/Display are redacted. It adds no Clone, Deref, exposure, or serialization
surface. There is no unsafe, manual auto-trait implementation, observer bound, thread/channel,
global, alternate container, conditional path, dependency, or test edit. Repository search
confirms the sole `new_observed` caller is the accepted inferred drop-only regression.

The frozen `secret_hygiene` test remains 281 lines at
`dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4`; every other path is
unchanged. Source inspection accepts the correction for Hermes's exact format, Clippy, focused,
custody/native/address regression, and policy gate.
