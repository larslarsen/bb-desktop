# BBD-SEC-001 Security Evidence

Date: 2026-08-30
Reviewer: Lead Engineer/Reviewer — Codex
Integration actor: Jr Dev — Codex Luna
Governance head before final integration: `1ee8709e`

## Scope and safe-handling summary

This evidence covers the maintained BitBook Electron client and its security/SBOM
controls. The inherited marketplace tree and deprecated `go-ipfs` remain out of scope.
No native package was built, no GitHub workflow was dispatched, and no secret, finding
body, or sensitive source diff was recorded.

The generated SBOM remains only at:

`/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/bitbook-desktop.cdx.json`

The scanner/tool cache and temporary state remain under the explicit disk-backed paths
specified by the ticket. No local `/tmp` state, root access, global install, deletion,
cleanup, or `rm` was used.

## Correction 04 integration evidence

Accepted source hashes were verified before execution and after every bounded restore.
The Correction 04 red checks were non-vacuous and used only synthetic values:

- Empty-but-present `.gitleaksignore`: policy suite and checker rejected the missing
  ratchet; the exact eight-line file was restored and hash-checked.
- Removed current-tree Gitleaks step: policy suite and checker rejected the missing
  immediately-following directory scan; the workflow was restored and hash-checked.
- Synthetic Countly loader body in `addMetrics`: policy suite and checker rejected the
  retained remote loader; the no-op was restored and hash-checked.

The eight remaining falsifications also failed closed and were restored/hash-checked:
ratchet under-count, synthetic global/current-tree fingerprint, lexical reorder,
command-line ignore, baseline flag, Git/dir scan reorder, synthetic Doorbell loader/key,
and non-blocking directory scan.

## Final acceptance commands

All commands were run from the repository root in this order after final restoration:

| Command | Result |
| --- | --- |
| `node test/electronSecurity.node.js` | exit 0; 13/13 passed |
| `node test/securityPolicy.node.js` | exit 0; 50/50 passed |
| `node scripts/security-policy.js` | exit 0; policy checks passed |
| `npm audit --audit-level=low` | exit 0; 0 vulnerabilities |
| `npm run build` | exit 0; JavaScript and shell syntax passed |
| `npm run test:social` | exit 0; social core tests passed |
| `npm run test:security` | exit 0; Electron 13/13 and policy 50/50 |
| `/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks git --redact=100 --no-banner .` | exit 0; v8.30.1, 4,558 commits scanned, no leaks |
| `/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks dir --redact=100 --no-banner .` | exit 0; no leaks |
| `git diff --check` | exit 0 |

Electronegativity was not rerun. Correction 03 permanently removed that obsolete,
unmaintained, non-blocking scanner concept; the executable Electron boundary/sink tests,
blocking npm audit, and both blocking Gitleaks scans are the accepted controls.

## Manual SBOM evidence

The reviewer-authorized bounded external-network setup completed before this resume:

- `npm ci`: exit 0; 13 packages installed, 14 audited, 0 vulnerabilities.
- Pinned `@cyclonedx/cyclonedx-npm@6.0.1` JSON generation: exit 0.
- `node scripts/validate-sbom.js /home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/bitbook-desktop.cdx.json`: exit 0.

Safe document metadata:

- Format: CycloneDX
- Spec: 1.6
- Root component: `bitbook-desktop`
- Components: 13
- Dependency entries: 14
- Bytes: 30,515
- SHA-256: `9a628e9ff915b7fece623b8158cbd700c80351bdbff62ba5037ce74a6a95e17f`

The SBOM is not committed or uploaded from this local integration.

## Final accepted source hashes

```text
c794d5e063bb121f52ae09bd96bb3ced061a02bc38c2f866957fe629d6999089  test/electronSecurity.node.js
3aea90640053e02dc8f1e4cbd1f9be257cc50010d132dd4f1c934236125c2d01  test/securityPolicy.node.js
306def13efe09ee6d435dc5ad0b206e65731c3e536aa6e708a67c340dbb81fce  .gitleaksignore
c81417697b3a01dfbbd7c4ef71b7dd21665e999bc49dfdfeced02e276d7bc0c6  js/utils/metrics.js
376b0ff528931d4cb9db3c427600e5e4d89aa0218f4cff0e469692bb594e244d  js/utils/feedback.js
c693e4055c0da3d244603857b44a6ea849d1ebb521624cce698d3bc87894f72a  scripts/security-policy.js
38836e829d2edcb9941fb40139d5e32146e781b5447b542791b8cbd64f5a3ca7  .github/workflows/security.yml
5d8ffabbffb03a58d159c1d86136b8924de3e8e4c4f1bbb3cbc12fede58720b8  social-main.js
0c30c232b06ae92019b441fa8a51b9817b7ade1346a7ddae974dde3be36ac931  social/index.html
833e497427cec330e764c539569c72553d10aa5927420530f43add6df5c6e136  package.json
24bfa9eed1fd86f811dc20b7b82323a61cb73f0052d928a6e5dab8305ca8f1ec  .github/workflows/social.yml
4b00884d9a022ca60a71fe621065b3bef4f19356695ed1962e4232f8ffb795f6  .github/workflows/sbom.yml
418f33dca4d85e93a85a584305aec41e15bc97236e2ca1d218fddfac8f637657  scripts/validate-sbom.js
7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413  package-lock.json
```

All source paths above were authorized by BBD-SEC-001. The package lock remained
unchanged.
