# Grok Build Handoff — BBD-RATE-001 Tests Correction 02

State: COMPLETE — SOURCE ACCEPTED FOR HERMES EXPECTED RED

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Governance baseline: `dd4ac204`

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-RATE-001.md`, and
`docs/testing/BBD-RATE-001-TEST-SOURCE-REVIEW-02.md` completely before editing.

## Authorized paths

Edit only:

- `test/rateWorker.node.js`
- `test/rateSupervisor.node.js`

The accepted fixture and policy test are frozen at their Review-02 hashes. Every other
test, production, package, workflow, policy implementation, wallet, documentation,
dependency, lockfile, evidence, Git, and unlisted path is frozen. Run no command, test,
network access, formatter, or Git operation.

## Exact correction

1. In supervisor failure tests, keep hostile canaries in the fake raw stderr/stdout capture
   but assert they are absent only from the supervisor's logs, returned snapshot, and any
   forwarded/public result. Do not require the deliberately hostile raw capture to be clean.
   Continue proving protocol failure kills the child before cleanup and clears pending work.
2. Model the timeouts separately. Assert the HTTPS request/socket uses the 5-second connect
   timeout. Assert a distinct injected 10-second overall timer exists, fire it independently,
   and prove it aborts an in-flight request, schedules the 30-second backoff, and prevents a
   second fetch before that backoff expires. Prove both controls and all timers are cleared
   after completion/shutdown.
3. Require abort/destroy only where the request or response is still active or over a limit.
   For an already-emitted TLS error, non-200, redirect, or wrong content type, require only
   fail-closed unavailable output, no cache, no retry/redirect, and cleanup; do not require
   a redundant request abort.
4. Restore an explicit one-in-flight test: hold the first provider response, issue a second
   query for that provider, prove the HTTPS request count stays one, then release/fail the
   first request and prove cleanup/backoff.
5. Replace the claimed child-entry test with a real offline subprocess boundary using
   `process.execPath` and the actual `quote-worker/worker.js`, zero provider arguments, one
   exact framed request on stdin, stdin EOF, one exact unavailable response on stdout with
   the same ID, empty diagnostics, bounded subprocess timeout, status 0, and no leftover
   process. Do not require an undocumented factory `exitCode`. The existing injected
   default-off HTTPS test remains the proof that this path cannot contact a provider.
6. Keep all other accepted assertions and the simplified Coinbase-ZEC/Kraken-XMR decision
   unchanged. Rename any test whose body still does not exercise its title.

Stop after these two paths. Report their SHA-256 values, line counts, test counts, the
specific corrected test names, and confirmation that no command/test/network/Git or other
path ran. Do not begin production.
