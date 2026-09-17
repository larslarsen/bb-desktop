# Ring of trust — research priority

Owner requirement recorded by Codex, 2026-09-16.
Status: initial research and integrated proposal recorded in
[BB-TRUST-ARCHITECTURE-01](BB-TRUST-ARCHITECTURE-01.md). T1 is resolved: automatic
community filtering, reviewable Spam with personal overrides, full ordinary feature
access with warnings for zero-reputation accounts, and limited spam protection.
Section 3A now selects reviewer engineering semantics for source profiles, bounded
delegation, quorum evaluation and expiry. Wire/identity integration and calibration
remain open. The first account-verifier source task is bounded by
[BBGO-ACC-001](../../../bb-go/tickets/BBGO-ACC-001.md); its current phase controls authorization.

## Priority correction

The owner requires an extensive ring-of-trust system, including blacklists and
whitelists, and directs that this be researched and resolved first. This precedes
freezing portable account authority, naming/discovery, device enrollment and new
payment-recipient contracts. T1 and the shared trust semantics are now recorded;
the account proposal selects authority within those constraints.
Naming N1 is provisional and is not a project gate. The owner tentatively accepted
the proposal with reservations and directed continuing. Do not reopen a general naming
debate without a concrete feature dependency. Persistent identity and readable names
remain required; no final global namespace decision is claimed.

Owner refinement recorded 2026-09-16: expect broad community agreement and apply its
judgments automatically. Retain filtered content for inspection/override like a spam
folder. New/zero-reputation users can use all features; warn others until reputation
develops. Limited spam controls are acceptable if feasible. This supersedes the
reviewer's earlier explicit-subscription default and restrictions on unknown users.
Agreement is a product expectation, not a verified assumption for raw majority voting.

The owner previously selected xhigh, then raised the cost of continued discussion.
Codex subsequently recommended switching to High. This supersedes the earlier direction
to retain xhigh through authority architecture; do not request an increase. No setting
change by the owner is verified. The recommendation is task-specific judgment, not a
measured model-performance claim. Keep continuation bounded and avoid repeated menus
or handoffs. The [architecture record](BB-TRUST-ARCHITECTURE-01.md) retains the decisions.

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
The architecture record's section 3A now selects the source/bootstrap authority and
aggregation model: replaceable default profile, independent source groups, one level
of explicit assessor delegation, distinct-issuer matching, strict two-thirds quorum,
expiring evidence and finite synchronization budgets. Initial default: five groups,
four qualifying contributions. Source independence is an explicit reviewed assumption;
the actual launch roster and calibrated resource limits are not yet established.
The owner-selected behavior is settled. The account proposal now selects a backed-up
account controller with separately revocable device keys. BBGO-ACC-001 freezes its
first verifier contract. Private synchronization and enforcement integration remain
reviewer engineering work.

## Deliverable and continuation

Review existing workspace research and current daemon/client controls. Compare relevant
systems using primary documentation, record limitations, and develop concrete scenarios
before selecting data models or algorithms. Produce one integrated proposal with:

1. Owner-facing behavior and a small set of material product choices.
2. Threat model, scoped policy semantics and conflict/revocation rules.
3. Impacts on portable identity, discovery, messaging, payments and future calls.
4. Bounded implementation sequence and meaningful acceptance cases.

Record all findings and decisions in repository documents. Do not create a separate
actor handoff for each research question. Existing PAY-001 acceptance is unchanged.
Only BBGO-ACC-001's active phase authorizes source work; this scope does not authorize
live wallets, daemon operations or actor launches.

Reviewer governance scope: this scope document, `BB-TRUST-ARCHITECTURE-01.md`,
`BB-ACCOUNT-RECIPIENT-PROPOSAL-01.md`,
`BB-IDENTITY-TRANSPORT-DIRECTION-01.md`, `BBD-PAY-END-TO-END-STATUS-01.md` and
`../handoff/CURRENT_TASK.md`, all in bb-desktop. This continuation also authorizes
exactly `tickets/BBGO-ACC-001.md` and `docs/handoff/CURRENT_TASK.md` in bb-go for the
reviewer-authored contract and routing. No other daemon changes are included.

Cross-repository publication baselines: bb-desktop
`58c854dbeda3031a57ac18ee601f1a588496d28d`; bb-go
`cd497749a771b063300e2c3edf8748fe285b06c4`. Validate the scoped diffs, local links,
source baseline hashes and whitespace; commit and publish only these enumerated
governance paths in each repository. Preserve all unrelated dirty/untracked work.
This exception includes no source/test integration, test execution or implementation
evidence. Source authors and Hermes retain their existing roles.

Reviewer verification for this continuation: inspected the eight scoped document
changes and primary-source comparison; the local-link audit checked 89 links with
zero missing targets. The ticket's four frozen source SHA-256 values match, and all
six reserved source/test targets remain absent. Scoped tracked `git diff --check`
returned exit 0 in both repositories. No implementation or execution evidence is
claimed by these documentation checks.
