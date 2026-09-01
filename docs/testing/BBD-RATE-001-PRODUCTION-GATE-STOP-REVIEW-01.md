# BBD-RATE-001 Production-Gate Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance HEAD at review: `365e4a33`

Result: **VALID STOP — FIXTURE-ONLY CORRECTION REQUIRED**

Hermes completed all five authorized falsifications. Each focused test failed for the
intended reason, each mutation was reversed, and all nine accepted production identities
were restored. The full gate then stopped at the two rate suites before any production
commit or push.

The four reported failures reduce to two fixture defects:

1. `depth_eight_ok` places its final scalar at parser depth nine. The production parser
   begins at root depth one and correctly rejects `depth > 8`; the fixture contains root,
   `x`, `a`, `b`, `c`, `d`, `e`, `f`, then scalar `g`. Both depth fixtures are therefore
   one level deeper than their labels and ticket boundary.
2. `canaries.amount_atomic` is `123456789`, which occurs verbatim inside the accepted ZEC
   precision price `42.123456789012345678`. The tests intentionally scan serialized
   snapshots for every private-context canary, so this creates false leakage failures in
   three rate-worker observations and one rate-supervisor observation.

No production correction is indicated. The accepted production hashes in
`BBD-RATE-001-PRODUCTION-SOURCE-REVIEW-03.md` remain frozen. The rate test source and all
other fixtures remain frozen. Correction 01 may change only three JSON string values in
`test/fixtures/rates/provider-bodies-v1.json`: the two depth bodies and the colliding amount
canary. Hermes's uncommitted stop evidence must be preserved for the resumed gate to
replace with final complete evidence.

The reviewer ran no Node, npm, test, provider, package-manager, scanner, or network
command. Read-only inspection confirmed the fixture nesting and substring collision.
