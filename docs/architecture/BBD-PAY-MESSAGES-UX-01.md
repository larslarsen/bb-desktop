# Payment requests belong in Messages

Decision recorded by Codex, 2026-09-15, following owner feedback.
This supersedes BBD-PAY-001's standalone Requests screen design.

## User experience

Alice requests money → Alice's conversation appears in Messages → opening it shows
the request alongside your messages. The card shows amount, currency, memo and status;
details expand in place. Cancellation or expiry updates the same card automatically.
A request must be discoverable even when Alice has never sent a text message.

Remove the standalone Requests tab. No Connect, Refresh, folder selection or daemon
instructions in this flow. Updates must arrive while the app is visible, including
when another conversation is open; do not depend on opening a request screen first.
Preserve the reader's scroll position and expanded details during background updates.

Owner additionally confirmed that text messages must update automatically too. Existing
live message events remain the immediate path; periodic conversation/active-history
reconciliation catches missed events and socket outages while HTTP remains available.
No reopening a conversation or manual refresh. Preserve drafts, reading position and
read-receipt semantics. The Messages handoff includes this in the same implementation.

The immediate slice displays received requests. Request creation, native approval and
sending remain unfinished; do not present working Pay/Send actions or a paid status.

## Engineering boundary and next work

Reuse the authenticated main-process read bridge and validated request data. Render
structured cards, not payment payloads disguised as chat text. Match counterparties
by verified peer ID, never display name. Before merging records into Messages, bind
the social session to the same local identity as the payment bridge; a renderer-selected
social server must not cause requests from another local identity to appear in its chat.
The [Messages implementation contract](../handoff/GROK_BUILD_BBD_PAY_001_MESSAGES_01.md)
fixes that binding: current config identity must exactly match the authenticated payment
DTO identity; session changes invalidate old work. This is a display consistency check,
not cryptographic authentication of the existing social HTTP surface or spend authority.

The current chat code groups conversations by peer ID in `social/app.js`, but renders
only text history. Integration must merge request-only conversations and request cards
without requiring a successful text-history fetch to make an existing request visible.
The accepted payment API supplies snapshots; automatic rereads remain necessary until
a separately authorized event interface exists. No daemon/protocol change is authorized.

The reviewer chose the standalone inbox and refined its controls before resolving
placement. That design is superseded. Existing transport work and evidence remain
available for reuse; no further standalone-screen polishing or capture is authorized.
Grok's [automatic inbox report](../testing/BBD-PAY-001-GROK-AUTOMATIC-INBOX-01.md)
is retained evidence, not acceptance of this Messages design. Messages integration is
not implemented or tested yet. No additional owner testing or actor relay is requested
for the obsolete screen.

Reviewer governance scope for this correction: this file, `docs/handoff/CURRENT_TASK.md`,
`tickets/BBD-PAY-001.md`, `docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md`,
`docs/handoff/GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md` and
`docs/handoff/HERMES_BBD_PAY_001_UI_RUNTIME_01.md`. Source and test files are unchanged.
