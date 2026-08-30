# Codex Luna Handoff — BBD-WAL-002 Acceptance, Falsification, and Integration

You are **Jr Dev — Codex Luna**. This is the complete durable handoff; ephemeral chat is
not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely: `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`tickets/BBD-WAL-002.md`, `docs/handoff/CURRENT_TASK.md`, every WAL-002 handoff/evidence,
and all current implementation/test/wiring paths.

Targeted green is reviewer-accepted at baseline `ff49a78519920aed51553ee8fabe6fe810a5ddc5`:
48 wallet, 14 Electron-security, and 54 policy tests passed. Use the exact accepted hashes
in `CODEX_LUNA_BBD_WAL_002_TARGETED_GREEN.md`; the lockfile remains
`7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`.

## 1. Broader foreground acceptance

Verify `HEAD == origin/master`, exact status/hashes, and `git diff --check`. Run in order:

```text
npm run build
npm test
node scripts/security-policy.js
npm audit --audit-level=low
```

All must exit 0. `npm test` must report 48 wallet, 14 Electron-security, 54 policy, and
the existing social suite green. Audit must report no vulnerability at the requested
threshold. These commands must not change `package-lock.json` or create/stage artifacts.
Stop on any failure, canary exposure, resource leak, hash drift, or scope change.

## 2. Required temporary falsifications

Use `apply_patch` only. Run one mutation at a time, run only
`node test/walletContract.node.js`, require a nonzero exit for the named reason, then
restore the exact original line with `apply_patch` before continuing. After every restore,
verify the file's accepted SHA-256 and `git diff --check`.

1. In `wallet-contract/fakes.js`, change the intent mismatch branch inside `verify` from
   returning `failure('INTENT_MISMATCH')` to returning `{ ok: true }`. The authoritative
   post-sign mutation test must fail because the mismatch is no longer rejected. Restore
   `fakes.js` to SHA-256
   `3f3ad73a9b051831406ae37dd8db7325340c4763000abf09d03eb885c62c6b74`.
2. In `wallet-contract/model.js`, immediately after the local `capabilities` declaration,
   temporarily add a vendor shortcut that returns spendable/private success for
   `account.vendor === 'trezor'`. The transparent-Trezor capability test must fail,
   proving vendor names cannot replace capabilities. Remove the line and restore
   `model.js` to SHA-256
   `08c6528cc29271bb4b939c2de890cb53558f94c2c32d1b74e1a5e91735069d53`.
3. In `wallet-contract/fakes.js`, change the final fake-adapter `broadcast` return from
   `failure(code)` to `{ ok: true, funds_moved: true }`. The inert-fake broadcast test
   must fail because success/funds movement is reported. Restore `fakes.js` to its exact
   accepted hash above.

No mutation may be committed. If a falsification passes, fails for syntax/setup, affects
an old unrelated test first, prints a canary, or cannot be exactly restored, stop.

## 3. Evidence and integration

After all green commands and falsifications/restores succeed, create only
`docs/testing/BBD-WAL-002-IMPLEMENTATION-EVIDENCE.md` using `apply_patch`. Record:

- baseline and every final protected/source/test hash and line count;
- exact command order, exit 0, and pass totals;
- audit result;
- each temporary mutation, expected failing test/reason, nonzero exit, restoration hash;
- no canary exposure, resource/artifact creation, dependency/lockfile change, package
  build, real wallet/network/device activity, or out-of-scope change; and
- exact final staged/commit scope.

Run `git diff --check`. Stage exactly these 15 paths:

- `.github/workflows/security.yml`
- `.github/workflows/social.yml`
- `package.json`
- `scripts/security-policy.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/wallet-contract/golden-v1.json`
- `test/walletContract.node.js`
- all six files under `wallet-contract/`
- `docs/testing/BBD-WAL-002-IMPLEMENTATION-EVIDENCE.md`

Verify the cached path list and hashes; `package-lock.json` must not be staged or changed.
Commit with `Implement WAL-002 offline wallet contract`, push `master`, and verify the
commit contains exactly those paths and `HEAD == origin/master`.

## Exclusions

Do not edit governance, packaging, SBOM, Electron/main/UI, dependencies, lockfile, or any
other path. Do not run installs, package:* commands, platform/native packaging, SBOM
generation, external scanners, Electron, network/wallet/daemon/hardware/device commands,
or any command beyond those above and read-only verification/Git integration. No
background execution, root, `sudo`, `/tmp`, deletion, cleanup, `rm`, checkout/reset,
globs as destructive targets, unresolved paths, or environment-variable targets.

Stop after push. Report every command/falsification result, evidence path line count/hash,
commit full hash/push/final baseline, exact commit paths, protected hashes, final clean
status, and all exclusions. Reviewer XHigh must accept the implementation and CI before
another ticket.
