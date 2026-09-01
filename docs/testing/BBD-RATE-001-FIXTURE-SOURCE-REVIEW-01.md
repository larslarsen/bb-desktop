# BBD-RATE-001 Fixture-Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance HEAD at review: `1799d291`

Result: **ACCEPTED FOR HERMES RESUMED GREEN GATE**

Accepted corrected fixture identity:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/fixtures/rates/provider-bodies-v1.json` | 263 | `28598e483853b853b08b666f1107772cf8cdb28a6d3bf7a6962cce508c738922` |

The diff contains exactly the three authorized JSON string-value changes:

- `depth_eight_ok` now places scalar `f` at depth eight;
- `depth_nine_fail` now places scalar `g` at depth nine;
- `canaries.amount_atomic` is now the unique valid u64 decimal
  `18446744073709551614` and no longer collides with either accepted precision price.

Formatting, ordering, fixture name, all other fixture values, all test source, and all nine
accepted production identities remain exact. Hermes's prior five falsifications completed
and restored successfully; they need not be repeated. The resumed gate must rerun both rate
suites and then the complete green/security sequence from the beginning.

`git diff --check` is clean. The reviewer ran no Node, npm, test, provider, package-manager,
scanner, or network command.
