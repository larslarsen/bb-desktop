# Hermes Handoff — BBD-WAL-005 Production Green 01

State: ACTIVE

Accepted production identity:
`docs/testing/BBD-WAL-005-PRODUCTION-SOURCE-REVIEW-01.md`

## Authorized paths

Integrate the accepted production bytes at exactly:

- `wallet-pay/model.js`
- `wallet-broker/supervisor.js`
- `social-main.js`
- `package.json`
- `scripts/security-policy.js`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`

Create or edit only:

- `docs/testing/BBD-WAL-005-LOCAL-GREEN-01.md`
- `docs/handoff/HERMES_BBD_WAL_005_GREEN_01.md` (state line only)

The accepted test/fixture paths are frozen. No preload, renderer DOM/CSS, wallet contract,
Rust, dependency, lockfile, ticket, roadmap, current-task, other documentation, or
unlisted path may change.

## Required execution

1. Record Hermes version, actual provider/model, Node/npm versions, and actual HEAD.
2. Verify the seven production hashes and five frozen test/fixture hashes.
3. Run in order and stop on the first failure:

```text
node test/walletPay.node.js
node test/walletSupervisor.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
node scripts/security-policy.js
npm test
npm run build
npm audit --audit-level=high
gitleaks git --redact=100 --no-banner .
gitleaks dir --redact=100 --no-banner .
```

Expected focused counts are 20 Pay, 12 supervisor, and 20 Electron-security tests; the
policy runner reported 256 total cases/assertions during expected red and must report its
exact passing total. All commands must exit zero; npm audit must report zero
high-or-higher findings, and both Gitleaks scans must report no leaks.

4. Write `docs/testing/BBD-WAL-005-LOCAL-GREEN-01.md` with identities, exact results,
   counts, and path audit.
5. Change only this handoff's state line to `COMPLETE`.
6. Run `git diff --check`, stage only the nine authorized paths, commit exactly
   `feat: add BBD-WAL-005 pay state`, and push `master` to `origin`.
7. Report commit, push, final status, and exact command results. Do not run falsification
   and do not authorize completion.

On any changed hash, unexpected count, failure, leak, audit finding, syntax error, hang,
resource leak, or unlisted path, do not commit or push; record the stop if possible and
return control to the reviewer.
