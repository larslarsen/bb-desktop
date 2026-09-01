# Hermes Handoff — BBD-WAL-005 Falsification 01

State: COMPLETE

Baseline: `0b51c73f8a875b69b3b57ad0dfb4740c5d96dc12`

## Authority

Jr Dev — Hermes may make one temporary mutation at a time, run only its focused command,
restore the exact baseline bytes immediately, and record evidence. Temporary mutations
must never be staged, committed, or pushed.

Authorized temporary mutation targets:

- `wallet-pay/model.js`
- `social-main.js`
- `wallet-preload.js`

Create or edit only:

- `docs/testing/BBD-WAL-005-FALSIFICATION-01.md`
- `docs/handoff/HERMES_BBD_WAL_005_FALSIFICATION_01.md` (state line only)

Baseline hashes that must be restored:

```text
wallet-pay/model.js acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e
social-main.js b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df
wallet-preload.js 3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df
```

## Four isolated falsifications

Perform exactly these, restoring and verifying the baseline hash after each:

1. In `wallet-pay/model.js`, bypass the `privacy !== "private"` blocker so a
   transparent-only hardware account reaches later eligibility. Run
   `node test/walletPay.node.js`; it must exit nonzero on the transparent-only Zcash
   assertion.
2. In `wallet-pay/model.js`, bypass the ZEC `restored_pool === "orchard"`
   `MIGRATION_REQUIRED` blocker. Run `node test/walletPay.node.js`; it must exit nonzero on
   the Orchard-restored spend assertion.
3. In `wallet-pay/model.js`, copy input `fee_atomic` or `receiver` into the sanitized
   preview result. Run `node test/walletPay.node.js`; it must exit nonzero on preview
   closure/canary assertions.
4. In `wallet-preload.js` or `social-main.js`, add one forbidden Electron
   `confirmIntent` method or `wallet:intent:confirm` channel. Run
   `node test/electronSecurity.node.js`; it must exit nonzero on the exact API/channel
   allowlist.

Any mutation that does not produce the intended failure is a stop. Do not stack
mutations. Do not alter test source.

## Restored gate and integration

After all four source files are restored and their hashes match, run:

```text
node test/walletPay.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
node scripts/security-policy.js
npm test
npm run build
npm audit --audit-level=high
gitleaks git --redact=100 --no-banner .
gitleaks dir --redact=100 --no-banner .
git diff --check
```

Every command must exit zero, npm audit must report zero high-or-higher findings, both
secret scans must report no leaks, and `git status --short` before evidence must show no
source/test change.

Write `docs/testing/BBD-WAL-005-FALSIFICATION-01.md` with each exact mutation, focused
failure, restoration hash, complete restored gate, environment identity, and path audit.
Change only this handoff's state line to `COMPLETE`. Stage only these two documentation
paths, commit exactly `test: falsify BBD-WAL-005 pay gates`, push `master`, and report
commit/push/final status. Do not authorize ticket completion.

On any changed baseline hash, test-source change, unintended failure, audit finding,
leak, hang, resource leak, or unlisted path, do not commit or push; return control to the
reviewer.
