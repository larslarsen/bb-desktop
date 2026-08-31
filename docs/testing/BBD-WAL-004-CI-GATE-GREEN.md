# BBD-WAL-004 CI Gate Local Green

Governance parent: `4a3a70b0f8c1c5abe7e9f576f41be04d43221b5d`

Pinned scanner: Gitleaks `8.30.1`; archive `8,230,402` bytes; SHA-256
`551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb`.

The baseline custom runner exited 0 with all 69 cases green. The corrected isolated
falsification exited 1 with 68 `ok`, one `not ok`, and final summary
`1 security policy test(s) failed`; the sole failure was the strict ratchet test because
the wrong-path candidate was accepted. The exact throw was restored via `apply_patch`,
the production policy hash returned to `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`,
and the post-restoration runner exited 0 with all 69 cases green.

Local gates, in order, all exited 0:

1. `node scripts/security-policy.js` — policy checks passed.
2. `npm run build` — syntax/build checks passed.
3. `npm test` — Electron security 19, security policy 69, wallet contract 48, broker protocol 11, supervisor 11, preload 6; no failures.
4. `npm audit --audit-level=low` — 0 vulnerabilities.
5. Pinned Gitleaks history scan — no leaks found; 4,656 commits scanned.
6. Pinned Gitleaks directory scan — no leaks found.
7. `git diff --check` — passed.

Accepted pre/post hashes: `.gitleaksignore` `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b`,
`scripts/security-policy.js` `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`,
`.github/workflows/sbom.yml` `dae5c48985ee9d70ccb06c33483fd13fa1f5351e431d251f6b878d31818a933e`,
and `test/securityPolicy.node.js` `99caa3f428f808cb53260cffadd4e5e8de7c556a9996075c26e77ab4a6adde47`.

No secret or canary was printed or recorded. No package build, Rust, Cargo, SBOM
generation, wallet, or wallet-broker runtime path was run.
