# BBD-WAL-009 Phase-A1 Test-Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Protected governance parent: `1bccf869`

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,105 | `670b6d0938bf061b774bc7126b4971105208f5385b395baed69fb967c00cb4b7` |

Result: **PHASE-A1 TEST SOURCE ACCEPTED**

The manifest adds only the explicit `zec_sign_verify` test target. The new target has
14 non-vacuous tests mapping to all 14 required Phase-A1 behavior groups: real local
v6/Ironwood software authorization and independently decoded effects; bounded public
result and independently derived transaction ID; native-only one-shot confirmation
and exact bindings; custody/network/session negatives; empty production hardware;
synthetic Keystone PCZT-v2 tagged contributions; malformed/replayed/cross-intent
contribution rejection; one-field post-sign mutation rejection; post-proof
cancellation and exact expiry boundaries; per-account locking and terminal release;
component/cleanup failures; positive wipe observations; canary non-disclosure; and
forbidden authority/capability inventories.

The source keeps the authoritative PCZT inside the broker-facing harness, requires the
synthetic signer to return only tagged Ironwood contributions, distinguishes signer
metadata from the independently derived transaction ID, and allows source-text checks
only for the explicitly permitted forbidden-authority inventories. No production
hardware route, raw broadcast/network/mainnet/XMR authority, dependency, feature,
lockfile, fixture, production source, Node/Electron source, policy, or workflow change
is present.

Sol session `01a06a1a-9e31-7a30-a6df-31bd8dea2130` used `gpt-5.6-sol` at High and
stopped at the two-path boundary. Transcript audit found only read-only inspection and
measurement commands; no formatter, test, build, lint, audit, scanner, dependency,
product, Git, network, wallet/node, hardware/device, or actor command ran. Execution
and integration remain unproven. Only the separately committed Hermes handoff may run
the formatter and expected-red target.
