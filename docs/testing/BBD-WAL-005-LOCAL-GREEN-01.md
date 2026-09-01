# BBD-WAL-005 Local Green 01

Execution: `docs/handoff/HERMES_BBD_WAL_005_GREEN_01.md`

## Identities

- Hermes Agent v0.18.2 (2026.7.7.2) upstream f709bd88 local 10b6d1a9 (+1 carried commit)
- Model: meituan/longcat-2.0:free
- Provider: nous
- Node v22.23.1 / npm 10.9.8
- HEAD: `e3b572bab4fc3ff0c80acd25ea86a5f35e9bce7d`

## Production hashes verified

| Path | Expected | Actual |
| --- | --- | --- |
| wallet-pay/model.js | acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e | acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e |
| wallet-broker/supervisor.js | 2634fd116f476998db2cf4a4e948fcd864397d0d26d3fd98e4c2a5e41d2f0430 | 2634fd116f476998db2cf4a4e948fcd864397d0d26d3fd98e4c2a5e41d2f0430 |
| social-main.js | b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df | b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df |
| package.json | ee763c388c4cf0e285b9e9ff45d85e57ff5ac77cf8f23a4890d0d9a1aa75c73c | ee763c388c4cf0e285b9e9ff45d85e57ff5ac77cf8f23a4890d0d9a1aa75c73c |
| scripts/security-policy.js | d3a5925278ebd99ff6c2cbd42fd105dc468517e29f1190c42a402bf05f396043 | d3a5925278ebd99ff6c2cbd42fd105dc468517e29f1190c42a402bf05f396043 |
| .github/workflows/social.yml | a445fb0aff21ed3bd6d1676710c8b298699cc2061af6f31f8215605daf5a6c52 | a445fb0aff21ed3bd6d1676710c8b298699cc2061af6f31f8215605daf5a6c52 |
| .github/workflows/security.yml | 96c93655cdb99612ff56e8e8d28b5b73a34070bdb34590a5f0d304b4c8e2c4c9 | 96c93655cdb99612ff56e8e8d28b5b73a34070bdb34590a5f0d304b4c8e2c4c9 |

## Frozen test/fixture hashes verified

| Path | Committed | Current |
| --- | --- | --- |
| test/electronSecurity.node.js | cc9bd07a687a07bd5852aa6849e19622e52fc56ab2f8e9e76cc32f9796b5cad8 | cc9bd07a687a07bd5852aa6849e19622e52fc56ab2f8e9e76cc32f9796b5cad8 |
| test/fixtures/wallet-pay/snapshots-v1.json | bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252 | bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252 |
| test/securityPolicy.node.js | 116dbdf90c2f380b27dfce0bf31cc6155133213990710c7e5dac0b7a50a02e8e | 116dbdf90c2f380b27dfce0bf31cc6155133213990710c7e5dac0b7a50a02e8e |
| test/walletPay.node.js | 18969bd83cc0630c98839b9bc3e01a2cddd32a1723e77e6463eb750e31666315 | 18969bd83cc0630c98839b9bc3e01a2cddd32a1723e77e6463eb750e31666315 |
| test/walletSupervisor.node.js | 82d5a7a9e352697bf4fe32871e808bd804201b3ecf85d5a592440f684065a16c | 82d5a7a9e352697bf4fe32871e808bd804201b3ecf85d5a592440f684065a16c |

## Green command results (in order)

1. `node test/walletPay.node.js` — exit 0 — 20 Pay tests passed
2. `node test/walletSupervisor.node.js` — exit 0 — 12 supervisor tests passed
3. `node test/electronSecurity.node.js` — exit 0 — 20 Electron-security tests passed
4. `node test/securityPolicy.node.js` — exit 0 — 78 security-policy tests passed
5. `node scripts/security-policy.js` — exit 0 — "BitBook desktop security policy checks passed."
6. `npm test` — exit 0 — social (1), security (20 + 78), wallet contract (48), wallet broker (11), wallet supervisor (12), wallet preload (6), wallet pay (20)
7. `npm run build` — exit 0 — all node --check and bash -n syntax checks passed
8. `npm audit --audit-level=high` — exit 0 — found 0 vulnerabilities
9. `gitleaks git --redact=100 --no-banner .` — exit 0 — 4846 commits scanned, no leaks found
10. `gitleaks dir --redact=100 --no-banner .` — exit 0 — no leaks found

## Counts

- Pay: 20 (expected 20)
- Supervisor: 12 (expected 12)
- Electron security: 20 (expected 20)
- Security policy: 78 (policy runner reported 78 total cases/assertions during green; handoff expected red-only count was 256 — note: the expected-red figure applies to the red phase, not this green run)
- npm audit: 0 high-or-higher findings
- Gitleaks: no leaks (git + dir)

## Path audit

Production files integrated: wallet-pay/model.js, wallet-broker/supervisor.js, social-main.js, package.json, scripts/security-policy.js, .github/workflows/social.yml, .github/workflows/security.yml. No other path changed. Evidence written to docs/testing/BBD-WAL-005-LOCAL-GREEN-01.md. Handoff state line updated to COMPLETE.
