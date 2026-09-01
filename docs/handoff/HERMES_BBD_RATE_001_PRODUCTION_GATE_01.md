# Hermes Handoff — BBD-RATE-001 Production Gate 01

State: ACTIVE

You are **Jr Dev — Hermes** using only the free configured Nous route. Integrate the
reviewer-accepted production bytes, execute exactly five temporary falsifications, restore
the accepted bytes after each, and run the complete local green/security gate. No
falsification byte may be committed.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-RATE-001.md`,
`docs/testing/BBD-RATE-001-PRODUCTION-SOURCE-REVIEW-03.md`, this handoff, the five
`quote-worker` modules, the four frozen test/fixture paths below, and `CURRENT_TASK.md`.

## Preconditions and accepted identity

Record Hermes version, actual provider/model, Node/npm versions, branch, HEAD, and
`origin/master`. Require `master`, `HEAD == origin/master`, a clean index, and exactly the
nine accepted production paths below as worktree changes. No other path may be dirty.
Require clean `git diff --check` and these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `quote-worker/providers.js` | 50 | `e473fb6d32f6dcaa19f8f5825ef47c0a63ca068767f0b75960a2c65d9102470e` |
| `quote-worker/model.js` | 490 | `5e00387eb93d2c2a8e7407e262400175c2aaa37006c35ad7a63daca1fb5969fa` |
| `quote-worker/framing.js` | 460 | `abb27a761e7ba42157ced917ee0da4409c9cd97e5681c83bcb5058ebcf80404e` |
| `quote-worker/worker.js` | 333 | `cbde0dd4242aaf85b3310b149b803e217a34799389edf7b924d0bcb1f7e19674` |
| `quote-worker/supervisor.js` | 286 | `b465c4c5bf3c5226f4e2acf7b555e2b96c90ef043e43ec601423edc831bba825` |
| `package.json` | 38 | `f8b13d53e80c8f91c87a473e3c873999a337078f8eae90779814ac368a10197a` |
| `scripts/security-policy.js` | 2,667 | `f66f6df408d434082b14b8e8a5e1bb61722a7f5bc09c97c7a5e224793b301e7e` |
| `.github/workflows/social.yml` | 153 | `5968dc31bbc72bfc010417381a3b6f83df1f1fa6abf9f71275b007b8254dc9b2` |
| `.github/workflows/security.yml` | 61 | `9b890179bcb5b8ade9503a43ec97c18ed3bca0ab4e2d7e1f0ebcec495225be4e` |
| `test/rateWorker.node.js` | — | `242de6e756e5d4db27d21fda16aedd2d7b7b9183d39a218ffe22c55a949736f8` |
| `test/rateSupervisor.node.js` | — | `8f3938cc46a86a893760d92a07da9a30b8d11f713514b89d2f8d77d42c866d80` |
| `test/securityPolicy.node.js` | — | `1bc23cc41fd5c50b855637d80c342e0de3cc9ed8cd48219163f626b06e4391f6` |
| `test/fixtures/rates/provider-bodies-v1.json` | — | `7179969a17b92c02115ab6224266609c5a29af51c486734c6e321393da52a15b` |

For every falsification below: use `apply_patch` for the exact forward patch; run only its
one focused command; require nonzero exit for the named assertion and intended reason;
then apply the exact reverse patch even if the observation differs. Verify the mutated
source hash is restored, all thirteen identities above are exact, the index remains clean,
only the nine accepted production paths are dirty, and `git diff --check` is clean before
continuing. If a patch or restoration does not apply exactly, stop after exhausting safe
exact restoration. Do not use checkout, reset, clean, or file replacement.

## Falsification 1 — coerce precision price through IEEE-754

In `quote-worker/model.js`, change only:

```diff
-    const price = canonicalDecimal(priceRaw);
+    const price = canonicalDecimal(String(Number(priceRaw)));
```

Run this named test directly:

```text
node -e "const n='parser: recorded Coinbase ZEC and Kraken XMR bodies become exact closed quotes'; const {tests}=require('./test/rateWorker.node'); const t=tests.find((x)=>x.name===n); if(!t) throw new Error('missing focused test'); t.fn();"
```

Require exit 1 because the accepted fixture decimal no longer equals the returned price.
Reverse the one-line patch and require the accepted model hash.

## Falsification 2 — leak private context into an HTTPS request

In `quote-worker/worker.js`, add exactly one header after `Accept-Encoding`:

```diff
         Accept: provider.accept,
         'Accept-Encoding': provider.accept_encoding,
+        Cookie: options.env && options.env.COOKIE,
```

Run:

```text
node -e "const n='worker: complete private-context canaries never appear in query, logs, or HTTPS'; const {tests}=require('./test/rateWorker.node'); const t=tests.find((x)=>x.name===n); if(!t) throw new Error('missing focused test'); t.fn();"
```

Require exit 1 because the cookie/private-context canary is observed in the HTTPS request.
Remove exactly the added header and require the accepted worker hash.

## Falsification 3 — permit redirect handling

In `quote-worker/worker.js`, change only:

```diff
-      maxRedirects: 0,
-      followRedirect: false,
+      maxRedirects: 1,
+      followRedirect: true,
```

Run:

```text
node -e "const n='worker: enabled Coinbase and Kraken fetches use exact pinned requests and closed snapshots'; const {tests}=require('./test/rateWorker.node'); const t=tests.find((x)=>x.name===n); if(!t) throw new Error('missing focused test'); t.fn();"
```

Require exit 1 at the redirect-pin assertion. Restore both exact lines and require the
accepted worker hash.

## Falsification 4 — accept an extra Kraken result pair

In `quote-worker/model.js`, change only:

```diff
-      if (!resultKeys || resultKeys.length !== 1 || resultKeys[0] !== provider.result_pair) return null;
+      if (!resultKeys || !resultKeys.includes(provider.result_pair)) return null;
```

Run:

```text
node -e "const n='parser: decimal digit bounds and strict Coinbase timestamps fail closed'; const {tests}=require('./test/rateWorker.node'); const t=tests.find((x)=>x.name===n); if(!t) throw new Error('missing focused test'); t.fn();"
```

Require exit 1 identifying `kraken_extra_pair` as producing a quote. Restore the exact
closed-result condition and require the accepted model hash.

## Falsification 5 — enable providers and spawn by default

In `quote-worker/supervisor.js`, change only the undefined-option branch:

```diff
   if (options.enabledProviders === undefined) {
-    enabled = [];
+    enabled = Object.keys(PROVIDERS);
```

Run:

```text
node -e "const n='default-off supervisor never spawns, never contacts a provider, and returns a closed unavailable snapshot'; const {tests}=require('./test/rateSupervisor.node'); const t=tests.find((x)=>x.name===n); if(!t) throw new Error('missing focused test'); t.fn();"
```

Require exit 1 because a child was spawned without opt-in. Restore `enabled = [];` and
require the accepted supervisor hash.

## Full restored green/security gate

After all five restorations, run in order and stop on the first failure:

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
82 repository-policy, 20 wallet-Pay, and 48 wallet-contract cases. Record the exact
`npm test` and maintained policy totals. Every command must exit zero; npm audit must
report zero findings, and both Gitleaks scans must report no leaks. Run no live provider,
package-build/publish, Rust, device, daemon, or cross-repository command.

## Evidence and integration

Create only `docs/testing/BBD-RATE-001-PRODUCTION-GATE-01.md` with identities, all five
mutation observations/restorations, exact command results/counts, scanner/audit results,
and final path audit. Change only this handoff's state line to `COMPLETE`.

Stage exactly the nine accepted production paths, that evidence file, and this handoff.
Commit exactly `feat: add BBD-RATE-001 quote worker` and push `master` to `origin`. Report
the commit, push, final status, and exact evidence. Do not edit the ticket, current task,
tests, fixture, lockfiles, wallet/Pay/broker/preload/renderer/Rust/daemon code, or any
unlisted path. Do not authorize ticket completion.

On any hash/count mismatch, unexpected failure, leak, audit finding, syntax error, hang,
resource leak, network/provider access, or unlisted change: restore any active mutation,
do not stage, commit, or push production, record the stop if possible, and return control
to the reviewer.
