# Codex Luna Integration — BBD-SEC-001 Correction 04

You are **Jr Dev — Codex Luna** (`gpt-5.6-luna`). This file supplements all prior Luna
handoffs and supersedes their Gitleaks stop only after Correction 04 is independently
verified.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read `AGENTS.md`, `TESTING.md`, the complete ticket/current task, Gitleaks Triage 01, and
every prior handoff. The accepted Correction 04 source report is authoritative.

Verify all six Correction 04 hashes before any edit/execution and verify these preserved
critical hashes as well:

```text
c794d5e063bb121f52ae09bd96bb3ced061a02bc38c2f866957fe629d6999089  test/electronSecurity.node.js
7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413  package-lock.json
```

Stop on mismatch.

Guard correction: the first version of this handoff transcribed the lockfile hash
incorrectly. Luna stopped before all work. Reviewer independently recomputed the hash
above and confirmed it matches the original Grok source report; `git diff` for
`package-lock.json` is empty. This corrected value is authoritative.

## Correction-specific red and falsification

Do not repeat earlier Electron/CSP/obsolete-scanner red or falsifications. Use only
minimal reversible `apply_patch` edits, one at a time, never `rm`, deletion, cleanup,
unresolved targets, or original finding bodies.

Bounded red before green:

1. Make `.gitleaksignore` empty while leaving the file present; the policy suite/checker
   must fail for missing ratchet content. Restore exact bytes/hash.
2. Remove only the exact dir-scan workflow step; the policy suite/checker must fail for
   missing current-tree scan. Restore the workflow hash.
3. Replace only the empty `addMetrics` body with a synthetic loader/key-shaped fixture
   from the test (never an inherited value); the structural oracle/checker must fail for
   the intended loader reason. Restore the metrics hash.

Run restored green:

```text
node test/electronSecurity.node.js       # 13/13
node test/securityPolicy.node.js         # 50/50
node scripts/security-policy.js
npm audit --audit-level=low
```

Then falsify and restore each authorized mechanism from Correction 04: under-count,
global/current-tree fingerprint, lexical reorder, command-line ignore, baseline flag,
git/dir scan reorder, synthetic feedback loader/key, and non-blocking dir scan. Record
the specific expected rejection and verify accepted hashes after every restore. Do not
print sensitive source diffs or use a real finding value.

## Corrected acceptance

After final restored green, run in exact order and stop on any failure/finding:

```text
node test/electronSecurity.node.js
node test/securityPolicy.node.js
node scripts/security-policy.js
npm audit --audit-level=low
npm run build
npm run test:social
npm run test:security
/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks git --redact=100 --no-banner .
/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks dir --redact=100 --no-banner .
git diff --check
```

Both Gitleaks commands must exit zero with no findings. Do not write a scan report. The
default root `.gitleaksignore` is the only historical ratchet; no command-line ignore,
baseline, config, range, report, altered exit, or suppression is allowed.

Only after both scans and diff check pass, run the unchanged manual SBOM workflow locally
under the explicit disk-backed paths and bounded npm settings in the prior handoffs. Do
not run Electronegativity or any native application package. Generate only
`/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/bitbook-desktop.cdx.json`,
validate it, and record required safe metadata. Do not commit it.

No `/tmp`, root, global install, cleanup, deletion, native package, workflow dispatch,
source repair, suppression, finding body, or secret value. Leave tools/caches/artifacts
for owner inspection.

On complete green only, author the already authorized evidence/current-task documents,
verify exact paths and unchanged package lock, final `git diff --check`, commit the full
BBD-SEC-001 source/evidence, push `origin/master`, and stop for reviewer inspection.
Record exact commands, versions, counts, exit codes, hashes, safe summaries, commit, and
push result.
