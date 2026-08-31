# Codex Sol Handoff — BBD-WAL-006 Support-Dependency Production Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Accepted unchanged manifest: `wallet-broker/Cargo.toml`, 81 lines, SHA-256
`6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632`.

Policy before correction: `scripts/security-policy.js`, 2,285 lines, SHA-256
`fe2b46c80ff20f741eed37938ac059db9c522ed3626b598a5594b419191888a5`.

Read completely: `AGENTS.md`, `TESTING.md`,
`docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-PRODUCTION-SOURCE-REVIEW-01.md`, the
original production handoff, the full manifest checker, and the complete WAL-004/WAL-006
manifest tests.

## Sole correction

Edit only `scripts/security-policy.js`. Immediately after the exact ordered
`actualDependencies == expectedDependencies` check, add a whole-manifest guard that:

1. derives the exact reviewed dependency names from `expectedDependencies`;
2. scans every manifest line, not only the `[dependencies]` slice;
3. retains every assignment line whose left-hand name is a reviewed dependency; and
4. requires that complete sequence to equal `expectedDependencies` exactly.

On mismatch, throw a stable wallet Rust manifest dependency/duplicate/displaced error
matching the committed policy tests. This must reject the existing mutation that appends
`zcash_client_backend = "=0.20.0"` after the test tables, while accepting each exact
reviewed assignment once in the real dependency block.

Do not restore a blanket `zcash` rejection. Do not change the expected dependencies,
constants, exports, test targets, regex, manifest, tests, lockfile, Rust source, or any
other path/behavior. Use `apply_patch`; read-only inspection is permitted. Do not run
Node, npm, Cargo, Rust, tests, formatters, linters, builds, policy tools, scanners, Git,
network, install, cleanup, commit, or push.

Report the corrected policy line count/SHA-256 and exact semantic change. Luna owns later
execution, evidence, lock resolution, integration, and Git. ZEC Rust source is frozen.
