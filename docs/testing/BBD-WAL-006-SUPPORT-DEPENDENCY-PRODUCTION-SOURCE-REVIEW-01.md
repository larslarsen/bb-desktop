# BBD-WAL-006 Support-Dependency Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `9e5ed88a`

Reviewed drop:

- `wallet-broker/Cargo.toml`: 81 lines, SHA-256
  `6643435bdf59608b09e906c8b7010baf1c17bbaa785d8eb70e37039d4bb37632`
- `scripts/security-policy.js`: 2,285 lines, SHA-256
  `fe2b46c80ff20f741eed37938ac059db9c522ed3626b598a5594b419191888a5`

Result: **POLICY CORRECTION REQUIRED — NO EXECUTION OR INTEGRATION AUTHORIZED**

The manifest change is accepted unchanged: it adds exactly the two tested support pins
after `zcash_keys`, changes no existing dependency/test, and leaves the lockfile frozen.
The policy correctly adds/exports the six Zcash dependencies, two support dependencies,
and six ZEC test targets; extends exact dependency/test inventories; removes only the
obsolete blanket `zcash` manifest rejection; and leaves broader feature/source policy
absent.

One fail-closed regression blocks the policy source. The existing WAL-004 test appends a
loose second `zcash_client_backend` assignment after the manifest's `[[test]]` tables.
The checker compares only the `[dependencies]` block and previously rejected that
mutation via the blanket `zcash` regex. After correctly removing the blanket regex, it
no longer inspects the appended assignment and would accept the mutation.

Sol must add a whole-manifest exact-assignment guard for every reviewed dependency name.
After the existing ordered `[dependencies]` equality, gather every line anywhere in the
manifest that assigns one of the expected dependency names and require that sequence to
equal the exact expected dependency lines once each. This preserves the valid accepted
Zcash declarations while rejecting a duplicate, loose, or displaced reviewed dependency
outside the dependency block.

Only `scripts/security-policy.js` may change. The accepted manifest, tests, lockfile,
Rust source, and every other policy behavior remain frozen. Luna does not execute until
the corrected policy hash is reviewer-accepted.
