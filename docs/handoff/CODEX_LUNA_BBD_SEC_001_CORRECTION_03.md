# Codex Luna Integration — BBD-SEC-001 Correction 03

You are **Jr Dev — Codex Luna** (`gpt-5.6-luna`). This file supplements the original
Luna handoff and supersedes the Electronegativity execution in Network Boundary 01.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance HEAD before integration: `990bf35a`

Read `AGENTS.md`, `TESTING.md`, the complete ticket, `CURRENT_TASK.md`, every prior
BBD-SEC-001 handoff, and especially the complete authorized/accepted Correction 03.

## Accepted correction hashes

```text
c794d5e063bb121f52ae09bd96bb3ced061a02bc38c2f866957fe629d6999089  test/electronSecurity.node.js
6dbd7d2bf534b587efd0445045a662911dfc83fd6498ae1b048e0439369d22dd  test/securityPolicy.node.js
473ce9702c8d8bd4518a6ef02b8bcb59b8e4cc8f62fdab9406c8990d544f93e3  scripts/security-policy.js
0e4fb13b7efb206137972eb70851703340d8f8f237ba2f64a380d5104365fd83  .github/workflows/security.yml
```

Verify these before any edit or execution. Stop on mismatch.

## Correction-specific test-first evidence

Do not repeat the earlier six falsifications or prior red reconstruction; they are
already complete/restored and must be summarized from the preserved record.

Produce one bounded red for the new maintained-source sink oracle by inserting a harmless
literal containing one forbidden sink into exactly one of the four scanned maintained
files with a minimal reversible patch. Run only `node test/electronSecurity.node.js` and
record that the new sink test fails for the intended file/token. Reverse exactly that
patch, verify the original delivered hash of the touched source from the Grok source
report, and then verify all four correction hashes above.

Run corrected green from the beginning:

```text
node test/electronSecurity.node.js       # must be 13/13
node test/securityPolicy.node.js         # must be 47/47
node scripts/security-policy.js
npm audit --audit-level=low
```

Then falsify obsolete-scanner rejection: insert exactly the former pinned
Electronegativity workflow step into `.github/workflows/security.yml` using a minimal
reversible patch. Both `node test/securityPolicy.node.js` and
`node scripts/security-policy.js` must fail specifically because the obsolete scanner is
forbidden. Reverse exactly that patch and restore/verify the accepted workflow hash.

## Remaining acceptance and SBOM

After restored full green, continue in the corrected ticket order:

```text
npm run build
npm run test:social
npm run test:security
/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks git --redact=100 --no-banner .
git diff --check
```

Stop on any failure or Gitleaks finding. Do not repair, suppress, baseline, or continue.
Never record a finding body or secret.

Then execute the unchanged manual SBOM workflow locally without any application/native
package build. Use only the explicit disk-backed cache/temp/artifact paths in Network
Boundary 01. Its Electronegativity command is superseded and must not run; its bounded
CycloneDX command remains authorized. Run blocking `npm ci` and `npm audit` as the manual
workflow specifies, generate only
`/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/bitbook-desktop.cdx.json`,
validate it with `scripts/validate-sbom.js`, and record format/spec, root component,
component/dependency counts, SHA-256, and byte size. Do not commit the SBOM.

No `/tmp`, `rm`, cleanup, deletion, root, global install, unresolved target, native
package, workflow dispatch, or source repair is allowed. Leave cache/artifact state for
owner inspection.

On complete green only, author the originally authorized
`docs/security/BBD-SEC-001-EVIDENCE.md`, update `docs/handoff/CURRENT_TASK.md`, verify the
exact authorized diff and unchanged `package-lock.json`, run final `git diff --check`,
commit the complete BBD-SEC-001 implementation/evidence, push `origin/master`, and stop
for reviewer acceptance. Record exact commands, versions, exit codes, counts, hashes,
safe result summaries, commit, and push result.
