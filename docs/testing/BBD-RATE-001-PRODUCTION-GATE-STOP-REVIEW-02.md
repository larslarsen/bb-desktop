# BBD-RATE-001 Production-Gate Stop Review 02

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance HEAD at review: `7d4cd72f`

Result: **VALID STOP — ONE PRODUCTION AND ONE TEST-SOURCE CORRECTION REQUIRED**

Hermes verified the corrected fixture, all accepted production identities, and all frozen
test identities before running the resumed gate. The supervisor, Electron-security,
wallet-Pay, wallet-contract, build, maintained policy, and audit commands reported green.
No production commit or push occurred.

The rate-worker failure exposes a production mismatch, not a ticket ambiguity. The ticket
states that zero, duplicate, mismatched, **or multiple quotes** produce no price. Therefore
`buildRateSnapshot` must reject a quote array whose input length is not exactly one before
selection. Filtering `[ZEC, XMR]` down to one ZEC quote and returning it violates the
fail-closed multiple-input rule. Only `quote-worker/model.js` requires correction.

The three policy-test failures expose contradictory frozen test expectations:

- the test's `SOCIAL_PATHS` and `SECURITY_PATHS` constants omit the ticketed
  `quote-worker/**` filter even though the RATE-specific tests require it;
- the shared `TOP_LEVEL_TEST_CMD` remains the pre-rate command, causing two inherited
  exact-command assertions to reject the ticketed package script; and
- the RATE-specific assertion appends `npm run test:rate` to that old command rather than
  treating the completed command as the shared exact value.

The accepted production policy and package bytes match the ticket and RATE-specific test
intent. Only `test/securityPolicy.node.js` requires correction. No workflow, package,
policy implementation, other production module, fixture, other test, or dependency change
is indicated.

Hermes's uncommitted stop evidence must be preserved and replaced only by the next resumed
gate. `git diff --check` is clean. The reviewer ran no Node, npm, test, provider,
package-manager, scanner, or network command.
