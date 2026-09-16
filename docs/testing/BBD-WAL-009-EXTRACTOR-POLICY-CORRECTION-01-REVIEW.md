# WAL-009 exact extractor policy correction 01 — source review

Reviewer: Codex. Date: 2026-09-12. Source accepted; formal execution pending.
No findings in this bounded policy change.

The new constants bind each exact path to its own literal three-line statement.
Escaped multiline anchors preserve indentation and count adjacent duplicated statements
without consuming a shared separator. The own-statement count must be one and the
foreign-statement count zero. Only that match is removed from authorityScreened, used
solely for the original generic sign/prove/extract/finalize expressions. Other paths
retain their original screened content. Generic unsafe, FFI, process/listener/network,
WAL-006 service/endpoint/broadcast/connect and mainnet checks retain their original input.
There is no early return, directory-wide allowance, exported helper or inventory change.

Removing the new constants/helpers/allowance block and reversing the two screened-input
substitutions in memory reconstructs the exact prior 2746-line policy SHA-256
`414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532`.
This isolates the current change from older dirty package/manifest policy changes.

Accepted `scripts/security-policy.js`: **2806 lines**, SHA-256
`0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d`.
The other 22 frozen inputs match independently; HEAD remains
`2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`.

No Grok execution transcript was supplied with the completion message. Source review
does not establish syntax/test exits. Authorize Hermes's retained focused acceptance
and broader policy diagnostic through
`docs/handoff/HERMES_BBD_WAL_009_EXTRACTOR_POLICY_ACCEPTANCE_01.md`.
The two existing mutation-test groups must pass; a complete policy-suite run is expected
to retain the four exact inventory-related failures, not become globally green.
The earlier 86/6 retained run is the observed red for this correction. Existing mutation
tests independently exercise forbidden authority, changed mappings and duplicate calls.

Inventory reconciliation, global all-feature lint, parked Monero defects, typed-secret
erasure and release acceptance remain open. No Rust execution, source integration,
commit or push is authorized. Reviewer performed read-only inspection/hash/Git checks,
not acceptance commands. Governance edits: this review, the named Hermes handoff,
CURRENT_TASK.md's active prefix and tickets/BBD-WAL-009.md's status prefix.
