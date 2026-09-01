# Grok Build Handoff — BBD-RATE-001 Production Correction 02

State: COMPLETE — ACCEPTED IN `BBD-RATE-001-PRODUCTION-SOURCE-REVIEW-03.md`

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Governance baseline: `6fcb399c`

Read `AGENTS.md` and
`docs/testing/BBD-RATE-001-PRODUCTION-SOURCE-REVIEW-02.md` completely before editing.

## Authorized paths

Edit only:

- `quote-worker/worker.js`
- `quote-worker/supervisor.js`

Every other production path, test, fixture, package/lockfile, workflow, policy, evidence,
documentation, ticket, handoff, dependency, wallet, renderer, Git, and unlisted path is
frozen. Run no command, test, formatter, network/provider access, package manager, or Git.

## Exact correction

1. Attach a fail-closed handler to the 5-second request/socket timeout. On that event,
   abort/destroy the still-active request and enter the existing single `settle(provider,
   null)` transition, producing exactly one 30-second backoff. Late timeout/error/overall
   callbacks must remain no-ops through the existing settled guard. Keep the separate
   10-second overall timer unchanged.
2. In every supervisor child-failure path, capture the current child, detach its listeners,
   clear the authoritative `child` reference, then best-effort signal the captured process
   only when appropriate. Catch kill errors. A natural `error`/`exit`/`close` must therefore
   leave no reusable dead handle, pending response, decoder, or cache. A later query may
   spawn a fresh child. Preserve protocol-failure tests that observe `kill()` on a live fake.
3. Keep all other Correction-01 bytes and every public export/behavior unchanged.

Stop and report the two final hashes/line counts, both lifecycle transitions, all frozen
hash confirmations, and that no command/test/network/Git or other edit occurred. Do not
begin integration or evidence.
