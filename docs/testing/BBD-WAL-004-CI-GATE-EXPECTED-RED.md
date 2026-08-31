# BBD-WAL-004 CI Gate Expected Red

Governance parent: `9bcce182657ec8ea27cdd209526de2b8e3355e28`

Command: `node test/securityPolicy.node.js`

Exit: `1`

Custom-runner result: `66 ok`, `3 not ok`; final summary: `3 security policy test(s) failed`.

The intended failures were:

1. `checker constants match the ticketed Action and tool pins` — production Gitleaks constants still describe the old eight-entry ratchet.
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced` — production ratchet still has the old eight-entry rationale/content.
3. `WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts` — production command omits `--all-features`.

Accepted test integrity: 2,063 lines; SHA-256 `6b48023598984d91499466869533cf5c4b2d3b6a697cac567753f225dc044493`.

No other path changed, and no secret or canary appeared in the command result.
