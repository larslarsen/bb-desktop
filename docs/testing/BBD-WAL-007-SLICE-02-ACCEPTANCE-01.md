# BBD-WAL-007 Slice-2 Acceptance 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **ACCEPTED**

Accepted implementation commit:
`d0a14dd51d7e210ee7f1e2e96c9423447307b484`
(`feat: add BBD-WAL-007 Monero process lifecycle`)

The commit contains exactly the five accepted source/test paths, the complete green
evidence, and `CURRENT_TASK.md`. `HEAD == origin/master`, the index and tracked/untracked
worktree are clean, and the committed source identities match Format-Correction Source
Review 02.

The reviewer did not rerun any formatter, test, build, policy command, or product
binary. The retained Hermes session transcript proves:

- Rust 1.98 `cargo fmt --check`: exit 0 with accepted identities unchanged;
- exact `restricted-rpc` falsification: exit 101, one selected test failed at the
  forbidden-option assertion, then `process.rs` restored to its accepted hash;
- `xmr_process`: 12 passed, no failures or filtered tests;
- `xmr_distribution`: 12 passed, no failures or filtered tests;
- `native_surface`: 17 passed, no failures or filtered tests;
- native-ui check: exit 0 without warning or diagnostic;
- Node security policy suite: exactly 86 `ok`, no `not ok`, correct final line; and
- repository security-policy checker: exit 0 with the correct final line.

Hermes then exceeded the authorized execution scope by inspecting package scripts and
running `npm run build` and `npm run test` after the push. It also queried the Node
version and combined two required config reads. The committed evidence is corrected to
record this. The extra npm commands exited 0 and left the repository clean; they do not
alter the accepted source, commit, or required gate results. Future Hermes handoffs
remain exact-command only.

Slice 2's distribution-selected `monero-wallet-rpc` process plan, exact-owned-child
lifecycle, authenticated exact-version readiness, random loopback-port preflight,
per-account/four-child admission, health polling, and closed teardown are accepted.
This acceptance does not authorize Slice 3, the real local-Monero gate, or other
repositories/paths.
