# BBD-WAL-007 Slice-2 Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **ACCEPTED FOR HERMES FOCUSED GREEN**

No test, build, formatter, or product binary was run by the reviewer. The worktree
contains exactly the accepted five-path Slice-2 test/production drop, `git diff --check`
is clean for tracked changes, and no accepted Slice-1 source outside the opened XMR
module/model/test-support paths changed.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 4 | `6c199bc807b16d80d19141dbe726d630dc6c2b89662253947ec280692af60fc6` |
| `wallet-broker/src/xmr/model.rs` | 143 | `704540f27a1d92c6e82281db01e8689b3f0dbc99c3f066311a806e0ecab437b7` |
| `wallet-broker/src/xmr/process.rs` | 1,191 | `b91a18f13568a8288b787c065ce72e165f81ed935c2fe2e508aa68a061ddaeee` |
| `wallet-broker/src/xmr/test_support.rs` | 1,173 | `aa737decda7cae13cd15c3f6b0de05ff15f88f96703fddece0f184bc696268d2` |
| `wallet-broker/tests/xmr_process.rs` | 452 | `0e4a3e7823e987da982fed572f1bd79e914ce730ca49aa3fb4c2260e6f7d962a` |

The corrected test retains all 12 named tests. It now checks exact option names while
inspecting the real required `untrusted-daemon=1`, separately rejects IPv6 values,
observes entropy and reservation calls, proves polling does not respawn, checks exact-
child liveness around readiness, exercises the production coordinator, and rejects
control-bearing config roots before effects.

The production path now owns per-account/four-child admission in
`ProcessCoordinator`; `ProcessRig` delegates rather than maintaining a test-only
registry. Start and health polling are separate, and post-child startup, health, and
broker-exit failures enter the same closed teardown. Linux-only OS operations are gated
while unsupported targets retain an `UNAVAILABLE` public pool. Transient entropy,
combined-login, config-entry, and completed-plan secret copies have unwind/drop wiping.
Paths are derived and config-safe before effects. The listener is retained through
config synchronization and released only at the immediate spawn boundary; exact-child
liveness is checked before and after authenticated exact-version readiness.

The listener behavior is accepted under
`BBD-WAL-007-SLICE-02-PORT-PREFLIGHT-DECISION.md`. It is an availability preflight, not
an atomic socket transfer. Exact-owned-child kill/reap remains the accepted stable-safe
teardown boundary. No dependency, manifest, lockfile, RPC transport, account, store,
receiver, hygiene, local-gate, Electron, Node, ZEC, or cross-repository work is accepted
by this review.
