# Sol — Messages finish tests and driver

CLOSED — two-file drop reviewed for bounded red execution. See
[review 03](../testing/BBD-PAY-001-MESSAGES-REVIEW-03.md) and
[Hermes finish red 01](HERMES_BBD_PAY_001_MESSAGES_FINISH_RED_01.md).
No further source edits authorized. Historical source scope follows.
Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed.
Reviewer: Codex. Grok exhausted weekly usage and stopped without an accepted correction
drop. This records the fallback exception allowed by AGENTS.md and DEVELOPMENT_ROLES.
Read those role rules, TESTING.md, CURRENT_TASK and
[review 02](../testing/BBD-PAY-001-MESSAGES-REVIEW-02.md).

## Scope and baseline

HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index. Preserve dirty work.
Exactly two writable source paths:

| Path | Starting SHA-256 |
| --- | --- |
| test/paymentInbox.node.js | 1fee6ac398dd35de7ebfaa185e85b8aba608dd73ff850381846ecfdd7c84750c |
| test/paymentInbox.electron.js | 932257736999d2c5d04deb55e7bba26806ce190d31503b5c21a362d41970d492 |

Production stays frozen: app.js
`daa737ff2be1b89b6bbf6c74c51c2ffaaeaed2339e28796302b6d787ace08e5e`,
payment-inbox.js `437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64`.
All other source/fixture/dependency/security files remain frozen at recovery identities.
No test execution, syntax checks, formatter, runtime, installs, Git or repository-record
edits. Do not inherit Grok's focused-execution authority. No actor launches.

## Required test source

Keep the original 38 transport cases and useful Messages regressions. Address R1–R3
with focused tests against actual app.js, deferred HTTP fixtures and fake clock:

1. An outgoing text changes unread→read via a live event and via fallback poll; assert
   the existing bubble displays the read label automatically. Distinct identical text
   entries at the same timestamp survive unrelated request/status updates. Use genuine
   social message IDs where the response provides them; do not invent a production
   schema or silently deduplicate distinct messages.
2. Hold a read-receipt POST, deliver repeated events/polls and assert one receipt in
   flight for that session/peer. Release it while history still repeats the same unread
   set; assert no immediate unbounded acknowledgement loop. A later newly displayed
   unread message is acknowledged. Failure retries follow normal cadence; hidden/closed
   chat is never acknowledged. Peer/session change/disposal aborts owned receipt work
   and late completion cannot schedule updates. Exercise typing timer/disposal too.
3. Remove unconditional successful process exit from the exported runner. Ensure every
   created fixture/app is disposed and pending deferred work is settled/aborted in
   teardown. Do not delete lifecycle assertions or disable event dispatch. Keep runner
   failures nonzero and exported completion awaitable. Add a bounded subprocess check
   if needed to prove natural exit and that code after awaited run() is reachable;
   avoid recursive suite invocation. No success exit/timeouts masking leaked handles.

## Electron driver repair

Fix R4 within the existing harness; retain real main/preload/renderer and sandbox proof.
Allow only exact owned social HTTP origin and required method/path pairs. Explicitly
deny and classify the exact owned social `/ws` connection as expected (HTTP fallback
scenario), plus exact default-origin bootstrap attempts. Any other request fails.
Keep payment endpoint main-only. Restrict fixture CORS to its actual file-origin use.

Bound reload waiting, assert successful config plus ready empty payment state before
empty screenshot, and recover from identity mismatch by opening the returned peer row
before looking for cards. Assert exact request IDs for cancellation; advance time across
expiry on an existing card. Stop/restart the owned payment fixture at the same descriptor
endpoint to prove unavailable→recovered without manual refresh. Never touch real data.

Populate enough text to force scrolling at both required window sizes. Focus an expanded
request summary, scroll elsewhere, capture visible entry/offset, then inject status and
text changes; assert focus and reading anchor remain stable without a jump. Also prove
bottom-follow when initially at bottom, composer draft preservation and unchanged node
stability. Use the real DOM geometry after resizing, positive layout checks and actual
keyboard/pointer interactions. Preserve exact amount/network/memo/status assertions.
No production renderer bypass flags or replacement rendering functions.

## Execution sequence after source review (not authorized for Sol)

Sol stops after test/driver authoring. Completion chat only points to these two files;
the reviewer reads them directly. No owner log transcription and no Sol evidence report.
Hermes will own the named next record:
`docs/testing/BBD-PAY-001-HERMES-FINISH-RED-01.md`.

After reviewer checks final test identities, the Hermes handoff will pin them and run
`node test/paymentInbox.node.js` against the frozen production above. Expected red is
the new read-state/receipt regressions, not a harness syntax error. Only after that red
does a bounded Sol production correction become authorized. Green verification will
use that same command plus `node test/socialCore.node.js`,
`node test/electronSecurity.node.js` and the existing three syntax commands. Hermes
will capture exact source manifests, commands, results and raw logs; do not reconstruct
the missing historical manifests. A temporary read-status-update suppression with
restoration hashes will falsify the new high-value regression in the later execution
contract. Broader Messages acceptance/scans and isolated sandboxed Electron execution
remain pending; no old runtime handoff is revived.

Reviewer governance scope for this routing: this handoff, Messages review 02,
CURRENT_TASK, the recovery handoff, tickets/BBD-PAY-001.md and the end-to-end status map.
