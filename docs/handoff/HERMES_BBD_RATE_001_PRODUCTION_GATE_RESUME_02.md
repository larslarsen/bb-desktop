# Hermes Handoff — BBD-RATE-001 Production Gate Resume 02

State: COMPLETE

You are **Jr Dev — Hermes** using only the free configured Nous route. Run the complete
green/security gate against accepted Source Correction 03, complete the existing evidence,
and integrate only on full success. Do not repeat the five prior falsifications.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-RATE-001.md`,
`docs/testing/BBD-RATE-001-PRODUCTION-SOURCE-REVIEW-03.md`, both production-gate stop
reviews, `docs/testing/BBD-RATE-001-SOURCE-CORRECTION-03-REVIEW-01.md`, both prior gate
handoffs, this handoff, the existing uncommitted gate evidence, all five quote-worker
modules, all four rate test/fixture paths, and `CURRENT_TASK.md`.

## Preconditions

Record Hermes version, actual provider/model, Node/npm versions, branch, HEAD, and
`origin/master`. Require `master`, `HEAD == origin/master`, a clean index, and exactly
these twelve worktree paths: the nine accepted production paths, corrected fixture,
corrected `test/securityPolicy.node.js`, and existing uncommitted gate evidence. No other
path may be dirty. Require clean `git diff --check` and these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `quote-worker/providers.js` | 50 | `e473fb6d32f6dcaa19f8f5825ef47c0a63ca068767f0b75960a2c65d9102470e` |
| `quote-worker/model.js` | 490 | `1f4f674f7e501a3cd69600414f3b6c517d484218d78c7c962f912efa581fa8be` |
| `quote-worker/framing.js` | 460 | `abb27a761e7ba42157ced917ee0da4409c9cd97e5681c83bcb5058ebcf80404e` |
| `quote-worker/worker.js` | 333 | `cbde0dd4242aaf85b3310b149b803e217a34799389edf7b924d0bcb1f7e19674` |
| `quote-worker/supervisor.js` | 286 | `b465c4c5bf3c5226f4e2acf7b555e2b96c90ef043e43ec601423edc831bba825` |
| `package.json` | 38 | `f8b13d53e80c8f91c87a473e3c873999a337078f8eae90779814ac368a10197a` |
| `scripts/security-policy.js` | 2,667 | `f66f6df408d434082b14b8e8a5e1bb61722a7f5bc09c97c7a5e224793b301e7e` |
| `.github/workflows/social.yml` | 153 | `5968dc31bbc72bfc010417381a3b6f83df1f1fa6abf9f71275b007b8254dc9b2` |
| `.github/workflows/security.yml` | 61 | `9b890179bcb5b8ade9503a43ec97c18ed3bca0ab4e2d7e1f0ebcec495225be4e` |
| `test/fixtures/rates/provider-bodies-v1.json` | 263 | `28598e483853b853b08b666f1107772cf8cdb28a6d3bf7a6962cce508c738922` |
| `test/rateWorker.node.js` | — | `242de6e756e5d4db27d21fda16aedd2d7b7b9183d39a218ffe22c55a949736f8` |
| `test/rateSupervisor.node.js` | — | `8f3938cc46a86a893760d92a07da9a30b8d11f713514b89d2f8d77d42c866d80` |
| `test/securityPolicy.node.js` | 2,846 | `6c5851c88cb64c8530f8d4c312b4ef49187d53b2fe233211ec9bddd8905af16f` |

## Full green/security gate

Run in order and stop on the first failure:

```text
node test/rateWorker.node.js
node test/rateSupervisor.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
node test/walletPay.node.js
node test/walletContract.node.js
npm test
npm run build
node scripts/security-policy.js
npm audit --audit-level=low
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
git diff --check
```

Require exact focused counts of 20 rate-worker, 16 rate-supervisor, 20 Electron-security,
82 repository-policy, 20 wallet-Pay, and 48 wallet-contract cases. Record exact `npm test`
and maintained policy totals. Every command must exit zero; npm audit must report zero
findings, and both Gitleaks scans must report no leaks. Run no live provider,
package-build/publish, Rust, device, daemon, or cross-repository command.

## Evidence and integration

Replace the stop state/content in existing
`docs/testing/BBD-RATE-001-PRODUCTION-GATE-01.md` with complete evidence while preserving
the five successful falsification observations. Record both accepted corrections, every
exact command exit/count, scanner/audit results, all final identities, and final path
audit. Correct any prior partial-gate wording; do not report a command as exit zero if its
child suite failed. Change only this handoff's state line to `COMPLETE`.

Stage exactly the nine accepted production paths, corrected fixture, corrected policy test,
completed evidence, and this handoff. Commit exactly
`feat: add BBD-RATE-001 quote worker` and push `master` to `origin`. Report commit, push,
final status, and exact evidence. Do not edit either prior gate handoff, the correction
handoff, ticket, current task, other test/fixture, lockfile, wallet/Pay/broker/preload/
renderer/Rust/daemon code, or any unlisted path. Do not authorize ticket completion.

On any hash/count mismatch, unexpected failure, leak, audit finding, syntax error, hang,
resource leak, network/provider access, or unlisted change: do not stage, commit, or push;
update the evidence stop reason if safe and return control to the reviewer.
