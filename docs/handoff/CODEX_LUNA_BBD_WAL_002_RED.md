# Codex Luna Handoff — BBD-WAL-002 Expected Red

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This file is the complete durable
prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`, and
`docs/handoff/CURRENT_TASK.md`.

## Accepted uncommitted test source

Before execution, verify that the only source/test changes are exactly these paths and
values:

- `test/fixtures/wallet-contract/golden-v1.json`: 231 lines,
  SHA-256 `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/walletContract.node.js`: 1,344 lines,
  SHA-256 `a814bf327345dbdde276343fc40ff6fd8ca770569b12afc0860c664a8c99b7d9`
- `test/electronSecurity.node.js`: 639 lines,
  SHA-256 `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- `test/securityPolicy.node.js`: 1,396 lines,
  SHA-256 `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`

Use `git status --short --untracked-files=all`, `wc -l`, `sha256sum`, and
`git diff --check` for this verification. If any value or path differs, stop without
editing, staging, committing, or running tests and report the mismatch.

## Authorized execution

Run these commands separately and in this exact order, recording stdout, stderr, and exit
status:

```text
node test/walletContract.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

Expected results:

1. Wallet test: nonzero only because `../wallet-contract` is absent, with
   `MODULE_NOT_FOUND` originating at the deliberately late implementation import after
   the fixture was read, parsed, inventoried, byte-checked, and independently hashed.
   Syntax, fixture JSON, preflight assertion, dependency, or any other failure is a
   rejection.
2. Electron security: exactly 13 inherited tests report `ok`; the one appended wallet
   test reports `not ok` only because the six maintained `wallet-contract/` files are
   absent. An inherited failure or another cause is a rejection.
3. Security policy: exactly 50 inherited tests report `ok`; the four appended wallet
   tests fail only on absent future package script/build contract, policy exports/source
   checker, workflow filters, and routine CI command. An inherited failure, parser error,
   missing existing file, or other cause is a rejection.

Run no npm command, build, formatter, scanner, audit, install, production code, workflow,
network, wallet, node daemon, device, USB/HID, or other repository action.

## Evidence and Git boundary

If and only if all three red results match, create only:

- `docs/testing/BBD-WAL-002-RED-EVIDENCE.md`

Record timestamp/timezone, governance `HEAD`, the four hashes/line counts, each exact
command and exit status, concise non-secret output proving the expected cause and prior
assertion counts, and confirmation that no production/dependency/lockfile/network/device
action occurred. Do not paste an irrelevant full stack trace.

Then stage only that evidence path with an explicit `git add --` command, verify the
staged diff contains only it, commit with message
`test: record wallet contract expected red`, and push `master`. Do not stage, commit, or
push any of the four failing test paths. After the push, report `HEAD`, `origin/master`,
the evidence hash/line count, the still-uncommitted exact test paths, and every command
result.

If a red result differs, do not create evidence or perform any Git operation. Report the
unexpected result to the reviewer. Production remains unauthorized in all cases.
