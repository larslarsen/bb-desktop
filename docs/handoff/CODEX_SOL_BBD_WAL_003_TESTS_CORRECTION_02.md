# Codex Sol Handoff — BBD-WAL-003 Test Source Correction 02

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `e0fa8898ca20e7552f41b0bccc536d69354fa270`

Read the ticket, architecture §§4.2–4.3 and 5.3–5.4, current task, both prior Sol
handoffs, and all six current test paths. Correction 01 is reviewer-rejected before
execution. Preserve its fixes and change only the same six test/fixture paths.

## Exact remaining corrections

1. **Never send application frames before binding.** The quit-order test currently calls
   `start()` and then requires two `intent.cancel` writes before the handshake. That
   contradicts the rule that no application method is sent or accepted before the
   session transcript binds. Complete the real fixture handshake in the positive quit
   case, then assert session-bound cancel envelopes precede termination. Add an unbound
   quit case that terminates without sending any application/cancel frame. Use only
   transport-observed writes/calls, not an implementation-owned log.
2. **Close dispatcher non-vacuity and inventory.** After every forbidden/invalid call,
   assert the send count is still exactly the seven positive sends. Add missing, extra,
   wrong-type, invalid-ID, and asset/network cases for `receiver.fresh`; add extra-field
   and non-plain/accessor cases for `intent.begin`; and prove nested accessors are not
   invoked. The sanitized positive snapshot must use the architecture-valid 32-lowercase-
   hex `account_id`, not `account-test-1`. A JSON-looking hello passed as diagnostic text
   or bytes—not as an already-parsed object—must remain diagnostics and never bind.
3. **Prove main-process size validation and cloning.** The Electron oversize case must use
   the otherwise valid top-level `{payment_request:{...}}` shape so it cannot pass merely
   as an unknown-field rejection, and it must prove zero supervisor calls. Change the
   supervisor mock so it does not itself hide a raw-reference bug by cloning before
   observation: record/attack the exact received params and prove main passes a distinct
   deep clone, preserves the renderer input, returns a distinct sanitized clone, and does
   not invoke getters. Keep one positive call per fixed channel and the subscription
   canary/clone proof.
4. **Make literal preload IPC usable and fail closed.** The source-policy positive preload
   example currently contains no `ipcRenderer.invoke`, so a checker that rejects every
   invocation could pass the tests while making the required preload impossible. Export
   and assert the exact five invoke-channel literals plus the one snapshot subscription
   literal. Positively check reviewed literal `invoke`, `on`, and `removeListener` use;
   reject a dynamic channel/method, every unlisted/authority-bearing channel, `send`,
   `sendSync`, and listener removal on a different channel. Add `spawnSync`, `execSync`,
   `execFileSync`, inherited/mixed environment, nonempty argv, and non-pipe/inherited
   stdio negatives so the source-policy proof cannot be satisfied by a narrow substring
   checker.

Keep every inherited assertion, the independent fixture hash, and all other Correction
01 coverage. Do not add production, package/workflow/policy implementation, evidence, or
new paths.

Use `apply_patch`. Only read-only inspection and final `wc -l`/`sha256sum` over the six
paths are allowed. Do not execute Node, npm, tests, builds, formatters, scanners, Git,
GitHub, network, Electron, child processes, wallets, nodes, hardware, or devices. Do not
install anything or use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or unresolved
destructive targets.

Stop after authoring and report all changed tests, totals, six hashes/line counts,
non-vacuity, expected red, preserved counts, and confirmation that nothing ran. Reviewer
XHigh must accept the corrected source before Codex Luna executes it.
