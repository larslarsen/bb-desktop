# Codex Sol Handoff — BBD-WAL-002 Test Source Correction 03

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the
complete durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, and this file.

Correction 02 source under review is:

- `test/fixtures/wallet-contract/golden-v1.json`: 231 lines,
  SHA-256 `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/walletContract.node.js`: 1,344 lines,
  SHA-256 `a814bf327345dbdde276343fc40ff6fd8ca770569b12afc0860c664a8c99b7d9`
- `test/electronSecurity.node.js`: 608 lines,
  SHA-256 `e225880b1ab432beee08bc56f9b3fab7972e1775d687372a3e2e700d5119bf11`
- `test/securityPolicy.node.js`: 1,378 lines,
  SHA-256 `482aeb3a61402c1a2cf745f6841b22acffac0028da98ff9adb3c5d5288c34f79`

The reviewer introduced one over-constraint in Correction 02. Correct it without changing
any other behavior:

1. Leave `test/fixtures/wallet-contract/golden-v1.json` and
   `test/walletContract.node.js` byte-identical at the hashes above.
2. In both appended security-test layers, extend the literal module-specifier allowlist
   to permit the exact pure sibling modules with and without `.js`:
   `./canonical`, `./canonical.js`, `./framing`, `./framing.js`, `./model`,
   `./model.js`, `./state-machine`, `./state-machine.js`, `./fakes`, `./fakes.js`,
   `./index`, and `./index.js`.
3. Keep `crypto`, `node:crypto`, `buffer`, and `node:buffer` allowed. Continue rejecting
   `../canonical`, `../wallet-contract/canonical`, absolute paths, `./other`, bare package
   or non-allowlisted built-in names, computed/concatenated/template specifiers, fetch,
   and WebSocket. Add positive and negative mutation assertions for these cases.
4. Keep every inherited security test byte-identical and keep all other Correction 02
   tests and expected-red ordering unchanged.

Authorization is limited to:

- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

Read-only shell commands are authorized solely to read the exact required repository
documents and the four current test paths, verify the two required byte-identical hashes,
and report `wc -l`/`sha256sum` for the two edited paths. Use `apply_patch` for edits. Do
not execute tests, Node, npm, builds, installs, formatters, scanners, Git, GitHub,
network, wallet, node, subprocess, hardware, USB/HID, or device commands. Do not use
`/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable targets, or
unresolved paths.

Stop after correction. Report all four hashes (proving the first two unchanged), the two
new line counts, exact test counts, the positive/negative import cases, expected red
causes, and confirmation that no command or out-of-scope action ran. Reviewer Codex at
XHigh must accept the source before Luna executes anything.
