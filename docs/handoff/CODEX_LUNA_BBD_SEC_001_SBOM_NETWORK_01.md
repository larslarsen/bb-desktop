# BBD-SEC-001 Manual SBOM Network Boundary 01

All corrected application/security acceptance gates are green: Electron 13/13, policy
50/50, checker, zero-vulnerability npm audit, build/social/security tests, Gitleaks full
history with zero findings across 4,558 commits, Gitleaks current tree with zero findings,
and diff check.

The unchanged manual SBOM sequence stopped at sandboxed `npm ci`, exit 1, because npm DNS
returned `EAI_AGAIN` fetching `undici-types`. No SBOM, evidence, commit, push, or source
change followed.

This authorizes one exact bounded `npm ci` retry with external registry network access.
It does not use sudo/root and writes dependency/cache/temp state only under the repository
or explicit disk-backed paths:

```sh
timeout --signal=TERM 180s env \
  TMPDIR=/home/lars/OpenBazaar/.security-tmp/bbd-sec-001-20260829 \
  npm_config_cache=/home/lars/OpenBazaar/.security-cache/bbd-sec-001-20260829/npm \
  npm_config_fetch_retries=1 \
  npm_config_fetch_retry_mintimeout=1000 \
  npm_config_fetch_retry_maxtimeout=5000 \
  npm_config_fetch_timeout=30000 \
  npm ci
```

Timeout, network/tool error, lockfile/source diff, or npm failure stops. On exit 0, Luna
may resume only the still-unrun manual SBOM steps: blocking audit, the already-authorized
bounded CycloneDX 6.0.1 command with the same `TMPDIR`/npm settings, validation, safe SBOM
metadata, evidence/current documents, exact commit/push, and reviewer return.

Do not repeat completed tests/falsifications/scanners. Do not use Electronegativity,
`/tmp`, deletion/cleanup, native packaging, workflow dispatch, suppression, or finding
bodies. Leave dependency/cache/artifact state for owner inspection.
