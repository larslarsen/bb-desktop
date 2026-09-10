# WAL-011 awaitable supervisor shutdown production 01

Actor: Grok Build, grok-4.6 High; production source only, no subagents.
Protected parent: reviewer publication following 82598823; one CURRENT-only launch
commit may follow. Read CURRENT lines 1–40 only, this handoff, ticket, AGENTS.md
and TESTING.md. Expected red is accepted: six absent-API failures, no cleanup
failure, Node exit 1. Do not repeat the red or execute tests yourself.

Only writable path: wallet-broker/supervisor.js. Verify these identities before
editing using separate read-only hash commands:

| Path | SHA-256 |
| --- | --- |
| wallet-broker/supervisor.js | 1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8 |
| wallet-broker/protocol.js | 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4 |
| test/walletSupervisorShutdown.node.js | 69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7 |
| test/fixtures/wallet-broker/shutdown-child.js | 0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb |
| test/fixtures/wallet-broker/transport-child.js | ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91 |
| test/walletSupervisorTransport.node.js | e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700 |

Read only the named source/tests, plus test/walletSupervisor.node.js for existing
quit compatibility. No history, broader source searches, runtime/config/home or
credential discovery. Separate read-only HEAD/status, named hashes/counts and
bounded source searches are permitted. No shell command chains, Node/syntax/test
execution, builds, dependencies, network, actor calls, docs/evidence edits or Git
mutations. Preserve every unrelated pending file and both frozen new test paths.

## Fixed implementation contract

Add shutdown() to the existing supervisor object. Keep quit() synchronous with
its existing undefined return and existing cancellation/close semantics. No main
wiring, binary pin/config changes, Rust changes, new methods on the wire, schema
changes, automatic restart or package expansion.

1. The first shutdown call creates and stores one Promise. Every later call returns
   that exact object, whether pending, resolved or rejected. Do not implement the
   public method as async (which wraps the returned Promise). Establish completion
   state before initiating quit so synchronous child close and reentrant calls do
   not lose the completion signal or create another Promise.
2. Initiate existing quit semantics once: attempt tracked intent cancellation before
   SIGTERM, clear intents, settle pending public requests with existing UNAVAILABLE,
   publish down once and leave the supervisor unbound/unstartable. Repeated shutdown
   and quit calls must not repeat cancellation or termination signals.
3. Track observed child close separately from child exit. Resolve with undefined
   only when no child object exists or close was observed, including before the
   first shutdown. killed, signalCode, exitCode, exit/error/end events and kill()
   return values do not establish close. Normal shutdown is completion even if the
   child needed SIGKILL or had already failed before closing; no native descendant
   or forced-OS-exit guarantee is added.
4. Use the existing system timer seam for a fixed 1500 ms deadline from the first
   shutdown call when close is still needed. Reject with makeError('TIMEOUT') on
   expiry. Do not invent error messages, attach diagnostics, retry, process.exit,
   or resolve on timeout. Preserve the settled rejected outcome after late close.
5. Preserve existing SIGTERM and 250 ms SIGKILL escalation. Handle a child that
   closes synchronously inside signalChild: do not install a leftover kill timer
   after that close. A narrow post-signal child-state check in terminate is
   authorized for this existing race. Do not change escalation timing or other
   failure paths to satisfy a test.
6. Clear the shutdown timer on resolution/rejection and retain one terminal outcome.
   No timer is needed when no child or already closed. Use existing close handling
   to notify completion rather than accumulating per-call listeners. Keep safety
   handlers needed until late close after timeout, and preserve caller observers.
   Once closed, no newly introduced shutdown resource may remain pending.
7. Before-start, failed-start, prior quit, protocol failure and close-before-call
   cases use these same rules. Do not reset failure state or start another child.
   Both cancellation and termination ordering must remain valid under reentrancy.

Keep the implementation local and small. Internal helpers/state are allowed for
shared Promise completion, child-close observation and timer cleanup. Do not expose
new test-only production hooks or refactor unrelated transport/validation code.
If the frozen tests reveal a contract conflict, report it without editing tests.

Report the changed path, SHA-256, line count and semantic changes; stop without
execution. After source review Hermes will run:

- node test/walletSupervisorShutdown.node.js
- node test/walletSupervisorTransport.node.js
- node test/walletSupervisor.node.js

These affected lifecycle regressions are required; unrelated accepted Rust and
Electron suites are not replayed. Falsification will resolve shutdown at initiation
before child close; the held-close case must fail, then exact source restoration
and focused green must pass. Reviewer will specify the exact mutation and driver
after inspecting the drop. No execution or integration is authorized for Grok.
