# BBD-WAL-007 Slice-2 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **REJECTED — BOUNDED CORRECTION REQUIRED**

No execution gate was run. `git diff --check` was clean, the worktree contained exactly
the four authorized source paths, and the frozen test remained 374 lines with SHA-256
`12cb52a5efca6a5ebfa53b1e856fc816c5ae7e8e01849b9034bd11d5a74d6f06`.

Reviewed source identities:

- `wallet-broker/src/xmr.rs` — 4 lines —
  `6c199bc807b16d80d19141dbe726d630dc6c2b89662253947ec280692af60fc6`;
- `wallet-broker/src/xmr/model.rs` — 143 lines —
  `704540f27a1d92c6e82281db01e8689b3f0dbc99c3f066311a806e0ecab437b7`;
- `wallet-broker/src/xmr/process.rs` — 980 lines —
  `d5069097b835d5a69f19da22ac5b5ec0af85c8202844db5fc449d575ccc64673`;
- `wallet-broker/src/xmr/test_support.rs` — 1,045 lines —
  `e93ea3f4275c7eb816251efb6cc713c8a95c00c596af5d0c6d8290d30972ba9b`.

## Blocking findings

1. The four-child/per-account limit is not a production boundary. `ProcessRegistry` is
   used only by `ProcessRig`; independently constructed public `WalletRpcProcess`
   instances bypass it. The test therefore proves test-support orchestration rather
   than a production process coordinator.
2. Post-start executable/child health is not exposed by `WalletRpcProcess`.
   `ProcessManager::start_or_poll` always calls `start` and is used only by test support,
   so it is not a valid ongoing poll path. Startup failures after spawn also use a
   kill/reap shortcut instead of the ticket's one closed teardown sequence.
3. `process.rs` imports Unix-only traits unconditionally. The ticket requires non-Linux
   platforms to compile and return XMR `UNAVAILABLE`; Windows instead fails compilation.
4. `argv_and_config_text_for_test` deliberately omits the required
   `untrusted-daemon` key because the frozen assertion searches for the substring
   `trusted-daemon`. This is a test-only concealment, not evidence of the actual config.
   `private_paths_accept_caller_input` is also a hard-coded `false`, and entropy-origin
   booleans are asserted without recording the boundary calls they claim.
5. Secret zeroization is incomplete. A partially filled entropy buffer is not wiped
   when `fill_entropy` returns an error, and the temporary combined RPC login `String`
   is dropped without zeroization. Secret-bearing config values need their own drop
   protection rather than relying only on the completed plan's outer drop.
6. The caller-supplied UTF-8 private root is inserted into config paths without rejecting
   newline/control characters, allowing a malformed root to inject config lines.
7. The retained-listener clause is impossible for the pinned upstream interface. The
   accompanying port-preflight decision narrows it to the behavior the product can
   truthfully implement and requires exact-child liveness around readiness.

The verified-capability spawn, exact argv/environment/config inventory, random bounded
candidate selection, typed exact-child kill/reap, private layout, sanitized public
errors, and basic manager/recording-port split are retained. No rewrite of the accepted
Slice-1 implementation is required.
