# BBD-WAL-007 Slice-1 Native Policy Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `503a0204`

Result: **SOURCE ACCEPTED — HERMES NODE-ONLY RESUME AUTHORIZED**

Sol edited only the two authorized policy paths. `HEAD == origin/master == 503a0204`,
the index is clean, the worktree contains only the accepted Slice-1 paths plus the
provisional evidence, and `git diff --check` is clean. The Node test count remains 86.
The reviewer ran no Node, test, Cargo, formatter, build, product binary, network, or
acceptance command.

## Accepted final Slice-1 identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/lib.rs` | 12 | `08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925` |
| `wallet-broker/src/native.rs` | 320 | `228c007856097c8433bc6e1c7132921f69be0b6f10543c3d999fabd9b14487f8` |
| `wallet-broker/src/native_ui.rs` | 149 | `34fda529c4ac6035bb5147720f456a271145deb43878082fbdfe464d320a7bdf` |
| `wallet-broker/src/xmr.rs` | 3 | `b39ea4228dd25951701cda9595a2b521e3391a92a41904c9e61b68ea1368695b` |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/model.rs` | 93 | `acb9391d22df344d7d72cbe139a0b87366be30d8c8f464e82e7db6e97ebddc47` |
| `wallet-broker/src/xmr/test_support.rs` | 368 | `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |

The two formatted Rust hashes reported by Hermes were exact, but the provisional
evidence and Node Stop Review 01 retained their pre-format line counts. The authoritative
post-format counts are 914 for `distribution.rs` and 368 for `test_support.rs`. This
review corrects that documentation error; the bytes and hashes did not change after the
green Rust commands.

## Policy correction acceptance

- The generic case-insensitive Monero authority ban remains active on every checked Rust
  source path.
- Only `wallet-broker/src/native_ui.rs` may contain zero or one occurrence of the exact
  reviewed literal `"Select monero-wallet-rpc"`; only that literal is removed from the
  generic screening text.
- A duplicate exact title is rejected before general screening. The exact title on a
  different path, alternate case/text, executable spelling, and any additional Monero
  token remain rejected.
- Existing unsafe, FFI, process, network, temporary-directory, lossy-path, WAL-006,
  inventory, dependency, and WAL-007 checks are unchanged.
- The existing WAL-004 native-policy test now contains the four required positive and
  negative regression groups without increasing the 86-test inventory.

The already completed formatter check, 17 native tests, 12 distribution tests, and
native-feature compile check remain valid because Sol changed only JavaScript policy and
its Node test. Only the two Node commands in the linked Hermes handoff need execution.
Slice 2 and the real local-Monero gate remain closed.
