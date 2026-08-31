# BBD-WAL-004 CI Gate Green Run 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `9195ea85563ff1cf36a6032e68d58f691e88b8f2`

Result: **FALSIFICATION METHOD CORRECTION REQUIRED — SOURCE RESTORED**

Luna's four-path preflight, line counts, hashes, scanner/version archive, ticket label,
and `git diff --check` all matched. The baseline
`node test/securityPolicy.node.js` command exited 0 with all 69 cases green.

The handoff then temporarily removed the wrong-path-specific rejection block. The suite
exited 1 with exactly 68 `ok` and the strict ratchet test as the sole `not ok`, but the
mutated wrong path was still rejected by the generic terminal exact-nine check. The test
failed because the generic diagnostic did not match `/wrong path|path/i`, not because the
bad fingerprint was accepted. This proves defense in depth, but it does not match the
handoff's intended acceptance-bypass falsification and is not recorded as final
falsification evidence.

Luna restored the exact block with inverse `apply_patch`; `scripts/security-policy.js`
returned to SHA-256
`affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`,
and `git diff --check` passed. No remaining gate, evidence, staging, commit, push, or
GitHub workflow ran.

The corrected falsification changes only the path-mismatch branch's `throw` statement to
`return;`. That simulates a real fail-open acceptance of the wrong path before the
generic terminal rejection can run. The strict test must then be the sole failure because
`assertRejects` observes no rejection for its wrong-path candidate. The exact source hash
must be restored before any remaining gate.
