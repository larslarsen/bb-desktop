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

## Network boundary result

Reviewer executed the exact approved bounded commands outside the restricted network
sandbox, without sudo/root:

- `npm ci`: exit 0; 13 packages installed, 14 audited, zero vulnerabilities;
- pinned CycloneDX npm 6.0.1 JSON generation: exit 0; and
- output SHA-256:
  `9a628e9ff915b7fece623b8158cbd700c80351bdbff62ba5037ce74a6a95e17f`.

The generator emitted deprecation warnings for its transient `glob` and
`prebuild-install` packages; it did not change BitBook dependencies. Reviewer reverified
unchanged `package-lock.json` SHA-256
`7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`
and an empty lockfile diff.

Luna may resume at blocking `npm audit`, then validate the existing exact JSON and record
its safe format/spec/root/count/hash/size metadata. It must not regenerate the SBOM or
repeat completed security gates. Full green permits evidence/current, final diff/path
checks, exact commit/push, and reviewer return.
