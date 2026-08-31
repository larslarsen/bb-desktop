# BBD-WAL-004 CI Gate Acceptance

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Integrated correction: `482650d5fe1e2a0d311040a7a811143afcbdd934`

Result: **ACCEPTED — BBD-WAL-004 COMPLETE**

## Local evidence

`docs/testing/BBD-WAL-004-CI-GATE-GREEN.md` is 31 lines with SHA-256
`a842f0a6523c8924328711e60fcb01d1ce0179113d6a6113cc4acaf8207699c4`.
The policy runner passed all 69 cases before and after the isolated falsification. The
temporary `throw`-to-`return` fail-open mutation made the wrong-path ratchet candidate
actually pass and produced exactly one intended test failure; inverse `apply_patch`
restored the accepted production hash before all remaining commands.

Repository policy, syntax/build checks, and the complete npm suite passed: Electron
security 19, security policy 69, wallet contract 48, broker protocol 11, supervisor 11,
and preload 6. Npm audit reported zero vulnerabilities. Pinned Gitleaks 8.30.1 scanned
4,656 commits and the complete current directory with no unsuppressed finding. The only
new historical suppression is the exact reviewer-published synthetic-vector
commit/path/rule/line fingerprint; the live ticket is relabeled and has no ignore.

## GitHub evidence

- Social client run `33362294160` succeeded at the integrated correction commit. No
  package job was required.
- Maintained client security run `33362304024` succeeded at the same commit. Npm audit,
  both Node security layers, repository policy, RustSec, all-features cargo-deny, and
  both full-history/current-directory Gitleaks scans all passed.
- Manual CycloneDX SBOM run `33362309557` succeeded at the same commit. The pinned npm
  and Rust generators ran; both documents passed their repository validators; and both
  artifacts uploaded. The Rust all-features graph includes the optional direct native
  components that the previous default-feature document omitted.

Final accepted hashes are:

- `.gitleaksignore`
  `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b`
- `scripts/security-policy.js`
  `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`
- `.github/workflows/sbom.yml`
  `dae5c48985ee9d70ccb06c33483fd13fa1f5351e431d251f6b878d31818a933e`
- `test/securityPolicy.node.js`
  `99caa3f428f808cb53260cffadd4e5e8de7c556a9996075c26e77ab4a6adde47`

The encrypted custody core production commit remains
`0e42fb4b477cfe76757ed207d3a561270b9e9efe`; its seven-case falsification evidence remains
at `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`. No platform package build, native window,
live wallet, node, device, signing, broadcast, or mainnet action was used for acceptance.
