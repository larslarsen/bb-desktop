# Grok Build Handoff — BBD-RATE-001 Production Correction 01

State: ACTIVE

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Governance baseline: `755c406c`

Read `AGENTS.md`, `tickets/BBD-RATE-001.md`, the committed RATE-001 tests, and
`docs/testing/BBD-RATE-001-PRODUCTION-SOURCE-REVIEW-01.md` completely before editing.

## Authorized paths

Edit only:

- `quote-worker/model.js`
- `quote-worker/framing.js`
- `quote-worker/worker.js`
- `quote-worker/supervisor.js`

The other five production paths are accepted and frozen at Review-01 hashes. All tests,
fixtures, package/lockfiles, policy, workflows, evidence, documentation, tickets, handoffs,
wallet, Pay, broker, preload, renderer, Rust, daemon, dependencies, Git, other repository,
and unlisted paths are frozen. Run no command, test, formatter, network/provider access,
package manager, or Git operation.

## Required correction

1. Make child framing incremental. Feed each stdin `data` chunk directly into the bounded
   decoder and immediately validate/respond to every complete request without waiting for
   EOF. On stdin EOF, exit 1 for an incomplete frame; otherwise shut down cleanly and exit
   0. A persistent supervisor pipe must receive a response while remaining open.
2. Preserve real asynchronous supervisor state. Do not clear a pending ID merely because
   `stdin.write` returned before stdout. Bind the pending query to its ID. On a later valid
   response, normalize/rebuild the snapshot from its quote rows and store it by query asset;
   clear pending state only after that transition. A later query may return that still-fresh
   rebuilt cache. Preserve the existing synchronous fake behavior and one-pending-request
   fail-closed rule. Wrong, duplicate, unsolicited, malformed, or stale responses still
   kill/quarantine the child and never populate cache.
3. Subscribe to child `error`, `exit`, and `close`; clear pending/cache/decoder state and
   fail closed without recursive or repeated kill behavior. Draining bounded diagnostic
   stderr must never forward its contents. Shutdown remains idempotent and clears all
   child/timer/listener state.
4. Make the untrusted response internally coherent before it can leave framing/supervision:
   unavailable display requires zero quotes and zero sources; a fresh display requires
   exactly one quote whose asset, currency, canonical price, and single source match the
   display. Require `fresh_until` and `expires_at` to be exactly five minutes after
   `fetched_at`; Kraken observation equals fetch time; Coinbase observation stays within
   its reviewed -10/+5-minute window. Reject noncanonical quote prices and impossible time
   relationships. The supervisor must rebuild from normalized quote rows rather than trust
   child display bytes.
5. Add one per-attempt settled transition in the worker. Exactly one success or failure may
   clear in-flight state, change cache/backoff, and schedule a timer. Late request/response
   errors after timeout/abort are ignored. Drain or destroy non-200, redirect, and invalid
   content-type responses so no response/socket handle is retained. Preserve the exact
   request pins, separate timeouts, size limits, one-in-flight rule, and causal backoff.
6. Construct parsed object properties without invoking the special `__proto__` setter (or
   reject that key consistently) in both strict parsers. Keep duplicate-key and depth
   behavior unchanged.

Do not change public exports, provider decisions, default-off behavior, tests, policy, or
package/workflow bytes. Do not add a dependency or UI/wallet integration.

## Stop and report

After the four-module correction, stop and report their SHA-256 values and line counts,
the exact incremental/async state transitions, confirmation that all five frozen production
hashes and four test hashes remain unchanged, and confirmation that no command, test,
network/provider access, Git operation, or unlisted edit occurred. Do not begin evidence,
integration, or another ticket.
