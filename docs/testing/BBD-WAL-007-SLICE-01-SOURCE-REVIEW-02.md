# BBD-WAL-007 Slice-1 Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `4aa68e5f`

Result: **SOURCE ACCEPTED — HERMES SLICE-1 GREEN GATE AUTHORIZED**

This review supersedes Slice-1 Source Review 01. Sol changed only the three correction
paths authorized there. The other six Slice-1 paths retain their reviewed identities.
`HEAD == origin/master == 4aa68e5f`, the index is clean, the worktree contains only the
nine accepted source/policy paths below, and `git diff --check` is clean. The reviewer
ran no formatter, compiler, test, build, binary, Node, npm, network, Git mutation, or
acceptance command.

## Accepted source identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 12 | `08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |
| `wallet-broker/src/native_ui.rs` | 149 | `34fda529c4ac6035bb5147720f456a271145deb43878082fbdfe464d320a7bdf` |
| `wallet-broker/src/xmr.rs` | 3 | `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b` |
| `wallet-broker/src/xmr/distribution.rs` | 910 | `f5e3b43c11f1a4a1b0389738b9729621fe80ea6f87bb5216640c47f213903ebb` |
| `wallet-broker/src/xmr/model.rs` | 93 | `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47` |
| `wallet-broker/src/xmr/test_support.rs` | 370 | `e2fb6496ddb731ad60753c169a31958f692be235bd0bf7b1a6c5e000872ad722` |
| `scripts/security-policy.js` | 2,678 | `6dbf22fb3980e424d2bb108ca568612b8cb23f2c7307d45543871486c18eb3f6` |
| `test/securityPolicy.node.js` | 3,162 | `f3464fe3f429c55f66cf1ac18e1a7be70d0d50263433b26068f2f20fa0dc3dad` |

## Correction acceptance

- `InstallationVerifier::linux_x86_64().verify_selected(&Path)` rejects unsupported
  platforms before filesystem or process effects, then delegates to
  `DistributionManager::verify_selected_path`. It reuses the centralized path, lstat,
  file-kind, exact-size, effective-user executable, bounded SHA-256, and exact bounded
  version verification without persisting a selection.
- `VerifiedExecutable::selected_path` is crate-private. Public errors and formatting do
  not disclose the selected path.
- The Node policy keeps the exact final nine-file XMR inventory and accepts only five
  unique, sorted, strictly cumulative implementation inventories: distribution;
  `+process`; `+rpc`; `+account/+store`; and `+receiver`.
- The repository source checker now accepts exactly the existing WAL-006 top-level
  inventory plus `wallet-broker/src/xmr.rs`, while preserving both historical shapes
  and the malformed, duplicate, missing, and extra-path rejections.
- The Node test remains exactly 86 tests. The frozen Phase-A manifest, lockfile, and Rust
  tests are unchanged.

The production design remains bounded to explicit Linux x86-64 selection, the pinned
official executable, native-origin orchestration, private atomic selection state, and
file-change reporting. No process lifecycle, RPC, node transport, account custody,
receiver persistence, Electron authority, download, fallback, PATH search, or remote
node behavior is present.

This is source acceptance, not executable acceptance. Only the focused Hermes gate in
`../handoff/HERMES_BBD_WAL_007_PHASE_C_SLICE_01_GREEN_01.md` is authorized. Slice 2 and
the real local-Monero gate remain closed.
