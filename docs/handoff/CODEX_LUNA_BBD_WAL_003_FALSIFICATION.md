# Codex Luna Handoff — BBD-WAL-003 Falsification and Restoration

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable
falsification prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Accepted implementation commit: `584019e9a89022d77b4bbb6710c2b7670e42d95b`

GitHub Social client run `33342988248` completed successfully for that exact commit. Its
routine `check` job passed and package-linux, package-macos, and package-windows were
skipped. Do not rerun or dispatch a workflow.

Read the ticket, current task, local-green evidence, and the three production/test pairs
below. Verify a clean worktree with `HEAD == origin/master`, implementation commit
`584019e9a89022d77b4bbb6710c2b7670e42d95b` in current history, and these committed
production hashes:

- `wallet-broker/protocol.js` — `79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4`
- `wallet-broker/supervisor.js` — `773a4815e9ca89752b28fcdbaaf19dcc347e648e4226314f7da78241f23d5520`
- `social-main.js` — `ef3c12eb00fe5ea990399bc8f4821d5574aa7bc79c353554e42851d8407e8397`

Use `apply_patch` for each mutation and its exact inverse. Apply only one mutation at a
time. Never use Git checkout/reset, deletion, cleanup, `rm`, a temporary copy, or `/tmp`.
After every negative run, restore immediately, verify the committed SHA-256, and run the
same suite green before proceeding. Stop without starting the next mutation on any wrong
failure inventory, restoration mismatch, or unexpected green.

## Falsification 1 — transcript order

In `computeSessionId()`, swap only `parentPid` and `childPid` in the exact preimage order;
leave the domain and nonces unchanged. Run:

```text
node test/walletBrokerProtocol.node.js
```

Expected negative: exit 1, exactly 10 pass / 1 fail, only
`transcript: independent fixture preimage and implementation session ID are exact`.
Restore the original parent-then-child order, verify the protocol hash above, and rerun
for 11/11 green.

## Falsification 2 — spawn before verification

In `createWalletSupervisor().start()`, immediately after the accepted `system.access(...)`
and before `system.sha256(...)`, add one deliberately unsafe call to the injected fake
`system.spawn` using the same empty argv and clean-env/no-shell/three-pipe options as the
real call. Guard it with `typeof system.spawn === 'function'`, and discard its return.
The supervisor test harness always injects this fake boundary; no real child process may
run. Run:

```text
node test/walletSupervisor.node.js
```

Expected negative: exit 1, exactly 9 pass / 2 fail, only:

- `launch: private data directory and regular readable pinned binary precede one inert spawn`
- `launch: missing, non-file, symlink, unreadable, and hash mismatch never spawn`

Restore by removing only that inserted pre-verification block, verify the supervisor hash
above, and rerun for 11/11 green.

## Falsification 3 — Electron confirmation channel

In the existing app-ready IPC registration block of `social-main.js`, add exactly one
`ipcMain.handle` registration for `wallet:intent:confirm` mapped through `walletHandler`
to broker method `intent.confirm`. Do not invoke the handler. Run:

```text
node test/electronSecurity.node.js
```

Expected negative: exit 1, exactly 17 pass / 2 fail, only:

- `wallet IPC registers only the exact renderer channel allowlist`
- `wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface`

Restore by removing only that registration, verify the social-main hash above, and rerun
for 19/19 green.

After all three restorations, verify a clean worktree and run `npm test` once; it must exit
zero. No source or test path may differ from commit `584019e9...`.

If and only if every negative inventory, exact restoration, positive rerun, and final
`npm test` matches:

1. Create `docs/testing/BBD-WAL-003-FALSIFICATION.md` recording timestamp, the three
   mutations, exact negative/positive counts, restored hashes, final `npm test`, no-canary
   statement, no-real-process statement, and GitHub run/job results above.
2. Update `docs/handoff/CURRENT_TASK.md` and `tickets/BBD-WAL-003.md` to
   `FALSIFICATION AND CI PASSED — REVIEWER ACCEPTANCE PENDING`, linking both local-green
   and falsification evidence.
3. Stage exactly those three documentation paths, commit
   `test: falsify wallet broker boundary`, push, and prove a clean worktree with
   `HEAD == origin/master`.

Do not edit tests, fixtures, lockfile, other source, prior evidence, architecture, package,
workflow, or any other repository. Do not run npm audit/build again, Electron, packaging,
scanners, installs, network, wallets, nodes, hardware, or devices. Git push is the only
authorized network action.
