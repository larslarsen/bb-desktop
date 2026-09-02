# BBD-WAL-007 Slice-1 Source Review 01

Decision: CORRECTION REQUIRED — NO EXECUTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance HEAD: `6b0babe5`

## Reviewed source identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 12 | `08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |
| `wallet-broker/src/native_ui.rs` | 149 | `34fda529c4ac6035bb5147720f456a271145deb43878082fbdfe464d320a7bdf` |
| `wallet-broker/src/xmr.rs` | 3 | `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b` |
| `wallet-broker/src/xmr/distribution.rs` | 864 | `6ff5b1d37f2dd8d073948eae61e5747f9d95f5b08ce09231c25171f100fa2e80` |
| `wallet-broker/src/xmr/model.rs` | 93 | `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47` |
| `wallet-broker/src/xmr/test_support.rs` | 370 | `e2fb6496ddb731ad60753c169a31958f692be235bd0bf7b1a6c5e000872ad722` |

## Accepted design shape

The drop stays within the seven authorized paths. `DistributionManager` centralizes the
validation/order/state machine; the recording port injects observations and named
faults, and `DistributionRig` delegates enroll, non-UTF-8 rejection, launch
authorization, and polling to that production manager. The Linux port implements real
`lstat`, effective UID/GID/group execute checks, bounded streaming SHA-256, bounded exact
version probing under an empty environment, private binary selection records, and file
identity polling. Native origin rejection happens before every selection side effect.

## Blocking findings

1. The accepted real local-gate source imports and calls
   `InstallationVerifier::linux_x86_64().verify_selected(&path)`, but Slice 1 does not
   define that API. Distribution is the owning slice; postponing it would leave the
   public production boundary incomplete. It must delegate to the same production path
   validation/hash/version logic without persistence or a copied verifier.
2. `VerifiedExecutable::selected_path` is public even though only code inside the broker
   needs it. The personal selected path must not become external public API; make that
   accessor crate-private.
3. The Node inventory test permits either zero XMR files or all nine final files. The
   ticket separately mandates five integrated production slices and forbids placeholder
   future files, so no compliant Slice-1 tree can pass policy. Freeze and accept only
   the five exact cumulative inventories: distribution; +process; +RPC; +account/store;
   +receiver. The committed repository checker must likewise accept the exact top-level
   `xmr.rs` addition while retaining every prior inventory rejection.

No reviewer execution occurred. Formatting/tests/builds remain Hermes-only. No other
source repair or Slice 2 work is authorized.
