# Ring of trust — research priority

Owner requirement recorded by Codex, 2026-09-16.
Status: research scope only; no trust model selected or implementation authorized.

## Priority correction

The owner requires an extensive ring-of-trust system, including blacklists and
whitelists, and directs that this be researched and resolved first. This precedes
freezing portable account authority, naming/discovery, device enrollment and new
payment-recipient contracts. The account proposal remains provisional input.
Its naming question N1 is deferred; the owner need not answer it before this work.

The owner also requested higher reasoning effort for architecture work. Codex must
ask for the session setting to be raised before the substantive architecture pass;
it must not claim to have changed the setting itself. This turn records scope and
priority only. No trust research completion, design acceptance or implementation
result is claimed.

## Questions the research must resolve

These are research questions, not assumed meanings of the owner's term “ring.”

- What a ring represents: explicit circles, degrees of connection, endorsements,
  delegated trust, or combinations; whether relationships are directional and scoped.
- Which actions trust governs: discovery/ranking, follows, messages, attachments,
  calls, payment requests, relay use, device enrollment and account recovery.
  Distinguish proof of identity from permission, reputation and willingness to interact.
- Blacklist/whitelist semantics: individual choices, list subscriptions, shared lists,
  scope, expiry, revocation, conflicting entries, overrides and explainable outcomes.
- Whether trust propagates through other people, how far, with what attenuation,
  and how cycles, collusion, fabricated accounts and compromised endorsers are handled.
- Continuity across account/device/key changes; block evasion; contested recovery;
  whether a trust relationship follows a verified account migration.
- Private versus published relationships; selective disclosure, correlation risks,
  list poisoning, harassment and the consequences of distributing negative assertions.
- Multi-device synchronization, offline decisions, stale/revoked endorsements and
  deterministic policy evaluation when devices have different information.
- User control: understandable defaults, reasons for a decision, manual exceptions,
  import/export and recovery from mistaken blocks or malicious subscribed lists.

The research must explicitly examine whether any proposed trust/reputation mechanism
would grant sensitive capabilities. Do not infer wallet spending authority or account
recovery authority from social proximity, list membership or a generic reputation score.
No particular scoring algorithm, trust depth or precedence rule is selected here.

## Deliverable and continuation

Review existing workspace research and current daemon/client controls. Compare relevant
systems using primary documentation, record limitations, and develop concrete scenarios
before selecting data models or algorithms. Produce one integrated proposal with:

1. Owner-facing behavior and a small set of material product choices.
2. Threat model, scoped policy semantics and conflict/revocation rules.
3. Impacts on portable identity, discovery, messaging, payments and future calls.
4. Bounded implementation sequence and meaningful acceptance cases.

Record all findings and decisions in repository documents. Do not create a separate
actor handoff for each research question. Existing PAY-001 acceptance is unchanged;
no source, tests, live wallets, daemon operations or actors are authorized here.

Reviewer governance scope: this scope document, `BB-ACCOUNT-RECIPIENT-PROPOSAL-01.md`,
`BB-IDENTITY-TRANSPORT-DIRECTION-01.md`, `BBD-PAY-END-TO-END-STATUS-01.md` and
`../handoff/CURRENT_TASK.md`, all in bb-desktop. The existing daemon direction link
leads to this priority; no daemon edit or publication is included.
