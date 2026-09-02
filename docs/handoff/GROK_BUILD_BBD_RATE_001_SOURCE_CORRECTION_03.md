# Grok Build Handoff — BBD-RATE-001 Source Correction 03

State: COMPLETE — ACCEPTED IN `BBD-RATE-001-SOURCE-CORRECTION-03-REVIEW-01.md`

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Governance baseline: the commit containing this handoff

Read `AGENTS.md`, `tickets/BBD-RATE-001.md`, and
`docs/testing/BBD-RATE-001-PRODUCTION-GATE-STOP-REVIEW-02.md` completely before editing.

## Authorized paths

Edit only:

- `quote-worker/model.js`
- `test/securityPolicy.node.js`

Every other production path, test, fixture, evidence, handoff, ticket, current-task,
package/lockfile, workflow, policy implementation, dependency, wallet, renderer, Rust,
daemon, Git, and unlisted path is frozen. Preserve the uncommitted Hermes stop evidence
exactly. Run no command, test, formatter, parser, script, network/provider access, package
manager, or Git.

## Exact production correction

In `buildRateSnapshot`, reject any input quote array whose length is not exactly one.
Change only:

```diff
-  if (!inspectArray(quotes)) {
+  if (!inspectArray(quotes) || quotes.length !== 1) {
```

Preserve all subsequent closed-copy, asset/provider/currency/freshness checks and all
public exports.

## Exact test-source correction

In `test/securityPolicy.node.js`, make exactly these four semantic corrections:

1. Add `'quote-worker/**'` to `SOCIAL_PATHS` immediately after `'wallet-pay/**'`.
2. Add `'quote-worker/**'` to `SECURITY_PATHS` immediately after `'wallet-pay/**'`.
3. Set `TOP_LEVEL_TEST_CMD` to exactly:

```text
npm run test:social && npm run test:security && npm run test:wallet && npm run test:wallet-broker && npm run test:wallet-pay && npm run test:rate
```

4. In `RATE-001 quote-worker package, syntax, top-level, and routine CI commands are exact`,
   change the exact package-script expectation from
   `` `${TOP_LEVEL_TEST_CMD} && ${RATE_CI_COMMAND}` `` to `TOP_LEVEL_TEST_CMD`.

Do not change any assertion count, policy behavior, RATE mutation, wallet assertion, test
runner, or other byte.

Stop and report both final SHA-256 values/line counts, the exact five changed source lines,
all frozen-path confirmations, and that no command/test/network/Git or other edit occurred.
Do not begin integration, edit Hermes evidence, or authorize another gate.
