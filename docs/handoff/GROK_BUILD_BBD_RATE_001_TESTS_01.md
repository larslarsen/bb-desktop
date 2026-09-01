# Grok Build Handoff — BBD-RATE-001 Tests 01

State: ACTIVE

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Baseline: `54cc0ccc17bc55eec5b23a1d414f3250f4c728cc`

Read completely before editing:

- `AGENTS.md`
- `TESTING.md`
- `tickets/BBD-RATE-001.md`
- `docs/architecture/BBD-RATE-001-PROVIDER-REVIEW.md`
- the rate sections of `docs/architecture/BBD-WAL-001-REVIEW.md`

## Authorized paths

Create or edit only:

- `test/rateWorker.node.js`
- `test/rateSupervisor.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/rates/provider-bodies-v1.json`

All production, package, workflow, policy implementation, wallet, Pay, broker, preload,
renderer, Rust, dependency, lockfile, documentation, evidence, Git, and unlisted paths are
frozen.

## Required drop

Author complete test source for every Phase-A contract in the ticket. The fixture must
contain a recorded-shape Coinbase ZEC body and Kraken XMR body plus bounded malformed,
precision, wrong/extra-product-or-pair, duplicate-key, depth, and oversize cases. Preserve
both prices as JSON strings so tests can detect IEEE-754 loss.

The tests must be offline and deterministic. Use injected clocks, HTTPS, spawn, child,
and timers. Assert non-vacuous request bytes, exact frame bytes, exact closed outputs,
exact single-source selection, exact decimal/rounding boundaries, cleanup, and the complete
private-context canary set. Require the future package/build/CI/source-policy contract to
fail closed without changing production policy now.

Do not create production stubs or mocks, skip/ignore tests, weaken existing assertions,
execute commands, access a provider/network, author evidence, or perform Git operations.

## Stop and report

After the four-path drop, stop and report:

- changed paths;
- SHA-256 and line count for each path;
- test-group inventory and expected production absence for each new suite; and
- confirmation that no command, network access, or Git operation ran.

Do not begin production source.
