# Codex Luna Handoff — BBD-WAL-003 Expected Red

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This file is the complete durable
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected pre-handoff governance parent: `886fd06e33bbc43cad20b13c54fdaa69231450a3`

The reviewer accepts the BBD-WAL-003 test source at exactly these paths and hashes:

- `test/fixtures/wallet-broker/transcript-v1.json` — 22 lines —
  `92702c7f8ae18a383b194142986992888e96e7211e0e6a974945b658e854c3f1`
- `test/walletBrokerProtocol.node.js` — 257 lines —
  `1397fb5e0833c2b58a53d6a8332914b47e4f00102bab4371ee43fc77d1960fd0`
- `test/walletSupervisor.node.js` — 340 lines —
  `ee17cf2ecd39c65a4c37821c78e04bfdcb2797d3e2cfe846580961840ce925e7`
- `test/walletPreload.node.js` — 134 lines —
  `60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e`
- `test/electronSecurity.node.js` — 797 lines —
  `135479b319bfca2d97ce7ca412da04afe79332ebb92e39fcb4b00ef3702b0d55`
- `test/securityPolicy.node.js` — 1,574 lines —
  `5ebb4cfa7fe91b073cdf8f6769c9e887e2bdcae17ef6b29dd3ef059c8daf9a83`

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-003.md`,
`docs/handoff/CURRENT_TASK.md`, the original Sol handoff, Corrections 01–03, and all six
accepted paths.

## Authority and sequence

1. Verify the named governance parent is an ancestor of `HEAD`, `origin/master` matches
   `HEAD`, the committed delta after that parent contains only this red handoff, ticket,
   and current-task governance, and the worktree contains only the six accepted unstaged
   paths. Verify every line count and SHA-256. Stop on any mismatch.
2. Run these five commands independently, in order, from the repository root:

   ```text
   node test/walletBrokerProtocol.node.js
   node test/walletSupervisor.node.js
   node test/walletPreload.node.js
   node test/electronSecurity.node.js
   node test/securityPolicy.node.js
   ```

3. Expected red:
   - protocol exits 1 only for missing `../wallet-broker/protocol`, after its independent
     fixture preflight;
   - supervisor exits 1 only for missing `../wallet-broker/supervisor`;
   - preload exits 1 only for missing `../wallet-preload.js`;
   - Electron exits 1 with 13 passing and 6 failing tests, all six failures caused by the
     missing explicit preload/exact IPC/supervisor integration; no inherited sandbox,
     navigation, permission, CSP, injection, or wallet-contract assertion may fail;
   - policy exits 1 with exactly 54 passing and 4 failing tests, all four new boundary
     tests failing only because package, workflow, and policy production are absent.
4. Stop immediately if a test unexpectedly passes, a count differs, a canary appears in
   output, or a failure has any other cause. Do not run broader tests or production.
5. If and only if red matches, create
   `docs/testing/BBD-WAL-003-EXPECTED-RED.md` with exact commands, exit codes, concise
   failure causes/counts, verified hashes, no-canary result, and confirmation that no
   production path ran or changed. Update only `docs/handoff/CURRENT_TASK.md` to state
   `EXPECTED RED RECORDED — PRODUCTION NOT AUTHORIZED` and link the evidence.
6. Stage exactly the accepted six paths plus those two record paths. Commit with message
   `test: define wallet broker boundary`, push `HEAD` to `origin/master`, then prove
   `HEAD == origin/master` and the worktree is clean.

Do not edit accepted test bytes, production, package/lock files, workflows, policy
implementation, any other documentation, or another repository. Do not run npm, builds,
formatters, scanners, Electron, a child process other than the five Node test commands,
wallets, nodes, network services, hardware, or devices. Git push is the only authorized
network action. Do not use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or
unresolved destructive targets.

Report exact command exits/counts, changed paths/hashes, evidence hash/line count, commit,
push result, final repository state, and confirmation of no out-of-scope action. Stop;
the reviewer must inspect the red evidence before authorizing production.
