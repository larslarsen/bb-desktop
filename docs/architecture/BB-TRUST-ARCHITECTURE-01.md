# Ring of trust — research and architecture proposal

Reviewer: Codex. Started 2026-09-16 against the
[owner's research scope](BB-TRUST-RESEARCH-SCOPE-01.md).
Status: initial research complete; owner product decision T1 recorded below.
Automatic reputation/filtering mechanics remain an engineering design task.
No trust implementation, protocol format or identity method is authorized.
Owner reports selecting xhigh; higher reasoning effort is not requested.

## Owner-selected behavior in plain language

Apply community reputation and allow/block judgments automatically through the trust
system, with personal circles, blacklists and whitelists available as overrides.
Automatically filtered content remains inspectable in a Spam/Filtered view. The user
can reveal an item, restore it or override filtering for the person. Ordinary use
must not require manually subscribing to each issuer's list.

Zero reputation means unknown/new, not bad. New accounts can use every implemented
feature under the same normal consent, protocol validation and recipient choices as
other accounts. Show others a clear reputation warning until sufficient evidence
develops. Do not impose a reputation threshold for messaging, posting, calling,
payment requests or normal wallet use. Limited behavior-based spam protection is
compatible with this requirement; exclusion simply for being new is not.

Being in a circle answers a particular question: “May this person message me?”,
“Do I use this person's spam judgments?”, or “Whose introductions do I consider?”
Following someone, accepting a payment request, trusting a relay and authorizing a
recovery helper are different decisions. A single reputation score cannot express them.

The owner expects broad community agreement on good and bad actors. Treat that as a
product expectation to support, not proof that a raw vote count or every signed report
is reliable. Community-source selection, aggregation, abuse resistance and warning
thresholds still need a concrete engineering contract. No global registry, mandatory
operator or unchangeable community blacklist is selected here.

## T1: control over inherited rules

Owner decision recorded 2026-09-16: **B, with reversible filtering and full access
for zero-reputation accounts.** This supersedes the reviewer's earlier A recommendation
and the earlier proposed default quarantine/call restriction for unknown accounts.

| Choice | Behavior |
| --- | --- |
| A — earlier reviewer recommendation, not selected | Other people's lists act only after an explicit subscription to specified effects. |
| B — selected with the qualifications below | Community judgments act automatically; filtered content is reviewable and personal overrides remain available. |
| C | Personal circles and lists only; other people's rules never affect the user's interactions. |

Required qualifications:

1. Automatic adverse judgments normally move valid content to Spam/Filtered rather
   than make it irretrievable. Invalid signatures, malformed data and traffic exceeding
   resource limits may still be rejected; the folder cannot retain an unlimited flood.
2. Personal overrides can reveal/restore an item or allow the account for chosen actions.
3. Zero reputation permits normal use with an informational warning to others. It is
   not an adverse judgment, and accumulated positive reputation is not an entry fee.
4. Explore limited spam protection, without making ordinary newcomers earn feature access.

No further answer to T1 is needed. Naming choice N1 remains deferred. Source selection,
aggregation and limits are reviewer engineering work, not delegated to the owner.

## 1. What the existing research and code establish

The workspace's `Distributed Social Media Research.md` already discusses moderation
at lines 94–118 and shared labels/web-of-trust ranking at 214–226. It did not specify
BitBook's rules. Its suggestion that graph-based ranking renders fake reviews
ineffective is not an accepted guarantee. Marketplace examples do not restore
marketplace scope. The private workspace planning document remains unpublished.

Read-only source baselines: bb-go `cd497749a771b063300e2c3edf8748fe285b06c4`;
bb-desktop `0ecfbc45aa30dee94918c98cbf1e46aaf112d64f`.

| Current surface | Observed behavior | Architectural consequence |
| --- | --- | --- |
| Daemon `modern/direct/service.go:402` | Verifies a signed peer envelope, then processes follows, chat, typing and read events. | Authentication exists. There is no account trust-policy decision in this admission path. |
| `modern/direct/service.go:341` | Retries delivery using the stored peer recipient. | New policy must be checked again before queued delivery, not only when composing. |
| `modern/payment/service.go:287` | Verifies payer/payee/signature, linkage, replay and expiry before retaining records. | Add policy after identity/object validation and before admitting new requests; preserve those existing checks. |
| `modern/social/store.go:197,310,365` | Follows, profile fetches and signed-post authorship use peer identities. | Follow relationships cannot silently become trust endorsements. Public content retrieval and display need separate policy decisions. |
| `modern/api/handler.go:47` | Social routes include wildcard CORS; no trust management route is present. | Do not add sensitive policy mutation by copying this surface. |
| `modern/localclient/server.go:194` | Authenticated loopback read endpoint rejects browser Origin and accepts only GET records. | Useful boundary pattern; write authorization, anti-replay/concurrency and policy events need a new contract. |
| `modern/network/node.go:93` | Configures libp2p, DHT and Bitswap; includes DHT peer-diversity filtering. | Routing defenses are distinct from human/account trust and must remain independent. |
| Desktop `social/app.js`, `social/payment-inbox.js` and `wallet-pay/inbox-client.js` | Messages/request cards are grouped and validated by peer identity. | Display verified policy results; do not make the renderer the sole enforcement point. |
| Legacy `net/ban_manager.go`, `net/service/service.go:87`, `api/jsonapi.go:3301` | Has a peer-ID ban map, stream rejection and persisted blocked-node settings. | Historical behavior is a reference only; it is not an extensive trust system in the maintained modern daemon. |

Inspection and targeted searches found no current modern circles, shared policy-list
subscriptions or trust evaluator. This is source inspection, not runtime verification
of an installed daemon. Inspected product sources match the baselines; unrelated
existing edits, including modern README and wallet work, are preserved.

## 2. Research comparison

Primary sources checked 2026-09-16. These are behavior/design references, not approved
dependencies. Each takeaway below is the reviewer's inference for BitBook.

| Source | What it provides | Takeaway and limit |
| --- | --- | --- |
| [GnuPG trust models](https://www.gnupg.org/documentation/manuals/gnupg/GPG-Configuration-Options.html) | Separates key validity and trust in an introducer; supports several trust models and bounded certification depth. | Separate identity evidence from permission to introduce others. OpenPGP identity certification is not a complete social reputation or blocking policy. |
| [Scuttlebutt following and graph](https://ssbc.github.io/scuttlebutt-protocol-guide/#following) | Directed follow relationships; clients can choose different visibility and replication distances. | A graph can organize rings while display and network replication remain separate. Its public follow graph would reveal relationships that BitBook should allow to remain private. |
| [Nostr NIP-51](https://github.com/nostr-protocol/nips/blob/master/51.md) | Lists with public and encrypted private entries, including mute lists and named sets. Draft/optional. | Distinguish private personal lists from intentionally shared ones. Its encrypted-to-self format does not solve BitBook's independent-device key sharing. |
| [Nostr NIP-85](https://github.com/nostr-protocol/nips/blob/master/85.md) | Signed assertions from providers the user selects for particular result types. Draft/optional. | Keep issuer and purpose attached to an assertion. A signature authenticates the provider, not the truth of a score; no required scoring service. |
| [AT Protocol labels](https://atproto.com/specs/label) | Signed issuer/subject/value annotations with retraction and expiry; optional content-version binding. | Separate claims from the policy consuming them. Retraction is not an opposite endorsement. Do not copy timestamp ordering as our multi-device conflict solution. |
| [Matrix community moderation](https://matrix.org/docs/communities/moderation/) | Shareable lists, explicit subscriptions and different lists for different purposes. | Explicit subscription can automate useful action while preserving scope. A room/server moderator's authority is different from a BitBook user's personal authority. |
| [EigenTrust, original paper](https://nlp.stanford.edu/pubs/eigentrust.pdf) | Aggregates file-sharing experience into a global reputation measure; its algorithm uses pre-trusted peers to address malicious collectives. | A numerical graph metric brings assumptions about seeds and evidence. Do not import a global score or network-wide trusted operators as BitBook's permission model. |
| [Douceur, The Sybil Attack](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/) | Explains why multiple identities can undermine systems that assume independent participants. | Many keys, endorsements or paths need not mean many independent people. This motivates bounded influence; it is not a proof that our proposed controls defeat all fake-account attacks. |

The useful combination is personal lists/circles, authenticated community assertions
and a local decision engine. T1 selects automatic effects with reversible filtering.
No reviewed system eliminates policy choice, privacy
tradeoffs, compromised trusted users or the need for resource limits.

## 3. Model: circles, introductions and authority

Use “ring” in the product as an understandable view over these distinct concepts:

| Concept | Meaning | Does not imply |
| --- | --- | --- |
| Named circle | An owner-managed set such as Friends, Work or Local community; membership may overlap. | Members know each other, endorse each other, or receive every permission. |
| Personal blacklist/whitelist | The owner's filter/allow choice for a typed target and specified actions. | A public accusation or an identity/key-validity judgment. |
| Introducer | An account the owner chooses to recommend other accounts for a stated purpose. | General permission to select recovery keys, spend money or change local policy. |
| Introduction distance | A derived view of eligible directed endorsement paths from the owner. | Authority over the owner, or a count of independent humans. |
| Shared list/label | An issuer's signed claim, with subject, scope, version and optional reason/expiry. | Truth by signature alone, unlimited authority or a network-wide ban. |
| Community policy | The sources/rings and versioned aggregation rules used automatically for reputation and filtering. | Every network account getting one independent vote or power over recovery/spending. |
| Source subscription | The mechanism for retrieving community policy's sources, with optional personal additions/removals. | A required manual setup step for every ordinary user or unrestricted scope expansion. |

Recommendation: automatic reputation/filtering is the default experience. Named circles
and source settings offer personal control; introduction distance is an optional
discovery aid. A person can be trusted for technical introductions but filtered for calls.

Community reputation may use signed endorsements and feedback about identified
accounts/content, with reasons, issuer provenance and retractions. Account age, traffic
volume, number of keys and circular endorsements must not by themselves establish a
good reputation. Private messages and payment activity are not automatically published
as reputation evidence. Eligible evidence and decay rules remain to be specified.

Use bounded influence and traversal. The earlier two-edge introduction model is a
candidate for discovery only, not the selected community-reputation algorithm. Do not
count cycles, mirrored lists or repeated paths as independent agreement. Distinguish
unknown, established positive evidence, adverse evidence and disputed/stale evidence;
they must not collapse into one number where missing evidence becomes negative.

Automatically incorporating eligible judgments about Carol is different from filtering
Carol merely because she knows Bob. Do not spread guilt by association through every
follow edge. Lists and endorsements feed a bounded, explainable community evaluation;
one arbitrary accusation is not automatically community consensus.

## 4. Actions, defaults and precedence

Proposed scopes are distinct. Exact names are illustrative until the protocol contract:
content visibility, discovery recommendations, incoming messages, attachments,
call invitations, payment requests, outgoing interactions and infrastructure use.
Account administration, policy editing, device enrollment, recovery and wallet spending
use explicit capability authorization outside this social-policy decision.

For each authenticated operation, compute both admission and presentation. Admission
is allow, hold for review or reject. Presentation can show normally, warn, mute or hide.
Content ranking is separate. “Hide” must not accidentally grant or revoke a capability.

Defaults implementing the owner's product decision:

- An unknown account can post, appear in discovery and send messages/payment requests
  normally, with “No established reputation yet” shown to others. Zero reputation alone
  does not send these items to Spam, suppress ordinary notifications or disable actions.
- Calling remains available to newcomers when implemented. Calls can ring with the
  warning under normal recipient preferences; camera/microphone use always requires
  ordinary acceptance. Reputation is not a separate invitation requirement.
- Sufficient adverse evidence from the automatic community policy filters otherwise
  valid content. Show it in Spam/Filtered with the reason, sources and review controls.
  Filtered call invitations do not ring; retain a bounded entry, not recorded media.
- Reading a filtered payment card does not approve or send money. Restoring it makes
  the normal native-review action available, subject to current validity and expiry.
- Positive reputation can remove the newcomer warning when its evidence qualifies;
  it never unlocks special wallet, device or recovery authority. A lookup outage shows
  “Reputation unavailable,” not “bad actor” or fabricated positive standing.
- Personal Block/Allow/Mute and optional tighter recipient preferences remain. Default
  personal Block suppresses new interactions into the reviewable filtered view and
  stops outgoing retries. An explicit stronger reject-traffic control, if added, must
  explain that rejected bodies cannot appear there. Mute only changes presentation.
- Feature access does not mean unlimited message volume. Bounded, recoverable rate limits
  apply to abuse/overload without requiring endorsements to participate.

### Proposed precedence under owner-selected T1

| Order | Rule | Conflict behavior |
| --- | --- | --- |
| 0 | Cryptographic validity, recipient binding, schema, device authority and hard resource limits | Policy cannot permit an invalid object or a revoked/unauthorized device. |
| 1 | Owner's explicit all-social block on the target account | Wins over social allows; normally filters rather than discards valid content. Unblocking must explicitly resolve it. Control-object handling is described below. |
| 2 | Owner's exact target/action decision | Overrides circle rules, community policy and defaults. Concurrent incompatible decisions are restricted pending resolution. |
| 3 | Owner's circle rules for that action | Filtering beats allow in a conflict; explicitly chosen reject-traffic rules are stronger. UI previews conflicting memberships. |
| 4 | Automatically evaluated community policy and personal source settings | Qualifying adverse evidence filters to Spam; conflicting/insufficient evidence is explained. Raw report counts and arbitrary single reports do not establish consensus. |
| 5 | Normal action default | Unknown/zero reputation allows normal use with a warning. No positive trust path is required. |

An exact personal message allowance can override a community-derived spam restriction
for messages without overriding a full personal Block or allowing calls. An allowlist
entry does not bypass signatures, expiry, wallet confirmation, quotas or device authority.
Removing an inherited denial falls back to the remaining rules, not an implied whitelist.

Community policy is active by default. Its source discovery/bootstrap, aggregation,
freshness and maximum effects must be inspectable, versioned and user-changeable;
ordinary users should not configure each constituent list. Changing sources/retracting
claims automatically recomputes derived results without overwriting personal rules.
Community updates cannot expand from reversible social filtering to spending, account
recovery, private-data publication or destructive deletion. List data cannot execute
scripts or unrestricted matching rules. Personal imported rules still get a preview.

### Spam/Filtered review and override contract

Provide a clearly discoverable Spam/Filtered view for affected messages, payment cards
and encountered public content, with access from their normal surfaces. This is not
the removed standalone payment Requests tab. Public results can retain a verified
reference/preview and fetch on deliberate reveal; do not promise a complete archive of
the network. Store bounded message bodies where accepted; expose retention/overflow
and unavailable/expired content honestly.

Offer **View once**, **Restore this item**, **Always allow this account** (with scope),
and **Keep filtered**. Viewing once does not whitelist, publish a counter-rating, emit
a read receipt, load remote media or reopen calls. Restoring does not auto-pay, resume
an old outgoing queue or launch an expired call. A personal override persists across
community-list refreshes and, once synchronized, across authorized devices. New
community evidence may remain visible as context but cannot silently erase it.

Retractions and improved standing re-evaluate filtered items. Releasing them does not
replay old alerts, ring old calls or execute queued actions. A historical label can
remain in the audit explanation without remaining an active restriction.

## 5. Enforcement and protected control messages

```mermaid
flowchart TD
    Input[Incoming object or local action] --> Verify[Validate identity, signature and object]
    Verify --> Policy[Daemon evaluates local policy snapshot]
    Own[Personal rules and circles] --> Policy
    Shared[Verified community reputation evidence] --> Policy
    Policy --> Decision[Admission result and local explanation]
    Decision --> Storage[Admit or quarantine before side effects]
    Decision --> UI[Messages, discovery and call presentation]
    Decision --> Native[Native payment review rechecks exact binding]
```

Apply policy before normal persistence, notifications, receipts, attachment fetching
and call signaling side effects. Spam storage is separate from the normal conversation;
automatic reputation filtering retains reviewable valid content within its limits.
Unknown reputation alone takes the ordinary path with a warning. Rate limits apply
before expensive verification, and further limits apply to authenticated accounts and
accepted objects. Reject malformed data rather than putting it in a review queue.

Recheck outgoing queued messages on every attempt. Changes invalidate cached policy
results and trigger UI events; missed events use snapshot/cursor reconciliation. A
policy decision carries its policy revision, evidence references and freshness state
so the same snapshot can be explained and reproduced.

Locally serialize a policy update with admission/side-effect commitment, or check the
policy revision again under the same commit boundary and retry the decision. A block
must not return “applied” while an older queued decision can still start a new local
send, notification or call. Define the boundary for already-started network operations;
bytes already transmitted cannot be recalled. This local race guarantee does not imply
simultaneous enforcement on disconnected devices.

**Do not drop every object from a blocked account indiscriminately.** A valid
cancellation of an already-held request must still make that request unpayable.
Likewise, required authority revocations, list retractions and protocol teardown need
bounded handling even when ordinary social traffic is denied. These control objects
must be authenticated, restricted to their exact existing relationship/object and
unable to smuggle chat text, attachments, new requests or notifications. Reserve
separate quotas so an attacker cannot bypass blocking by claiming a control kind.
Fetching authority evidence from other peers does not itself authorize contact.

The peer-visible acknowledgement must not disclose private list names, introducers
or block reasons. Its new protocol semantics must not claim user-visible delivery
merely because a quarantined object was stored. A peer may still infer restrictions
from behavior; blocking does not promise invisibility.

Infrastructure connection gating remains separate. A social block governs the
authenticated author/account even when data comes through a relay or third-party
cache. Relaying public data is not endorsement of its author. An explicit connection
ban on a peer/service can reduce reachability and needs separate presentation; using
only social friends for DHT routing could make isolation attacks easier. Preserve
v1's existing live-peer checks; relay/account semantics require a new protocol.

## 6. Data, privacy and synchronization

### Semantic records (not wire schemas)

Separate owner policy, other people's assertions, and derived decisions. Minimum
bindings for portable records:

| Record | Required meaning |
| --- | --- |
| Personal rule/circle edit | Owner account, authorized policy-writer device, typed subject(s), action/effect, causal predecessors and stable operation ID; private by default. |
| Endorsement or label | Issuer account and delegated signing authority, typed target/account or content digest, purpose/value, audience, operation ID, causal version and retraction/expiry semantics. |
| Shared list | Stable list identity under issuer, declared purpose, versioned membership/claims and verified update history. |
| Community policy/source settings | Community-view identity/version, eligible sources/roots and aggregation rules, effect ceiling, freshness handling, optional personal subscriptions/overrides and bounded delegation. |
| Reputation result | Account, assessment scope, evidence coverage/confidence, supporting and adverse evidence, freshness and policy version; unknown must remain distinct from adverse. |
| Evaluated decision | Subject/action, outcome, effective policy revision, considered evidence and a local explanation; reproducible cache, not an authority certificate. |

Target types must distinguish account, legacy peer, content version and infrastructure
service. No matching by mutable display name or current IP address. The account can
represent a human or a specialized service without requiring a human profile.

Use a versioned, domain-separated authenticated record format, with strict field/size
bounds and explicit extension handling. Reuse the chosen account-authority verifier
and established signing/encryption implementations; this proposal invents no new
cryptographic primitive. Merely verifying an issuer's signature does not authorize
that issuer to change local rules. Public profile JSON must not be an alternate
policy-write path.

### Private by default

Keep personal blocks, circles, reasons and subscriptions private to authorized devices.
Sharing a list or endorsement is a separate action with an audience preview. Do not
publish private data in the profile root, public DHT or public Bitswap blocks. Private
replication needs authenticated encryption and distinct device access controls; it
cannot reuse NIP-51's shared-secret pattern by copying the account key onto every device.

A public accusation can persist after retraction; revocation cannot erase copies.
Explanations shown to the local user may name their private sources, but remote peers
receive no such details. Avoid querying arbitrary list providers with every contact
lookup: that can reveal the user's graph. Prefer bounded synchronization of chosen
lists; assess padding/query privacy if a later service needs targeted lookups.

Public posts remain public: a personal block cannot prevent an adversary from reading
public copies under another identity. Private audience sharing needs encryption and
membership-key management. It is not implemented by a block button.

### Concurrent devices and offline operation

Use authenticated operations with causal references, not wall-clock last-write-wins.
The signed format and storage algorithm are later engineering contracts; requirements:

- An unblock/removal supersedes the specific operations it observes. A simultaneous
  block on another device remains effective until explicitly resolved. Concurrent
  whitelist removal wins over re-addition until resolved; concurrent restrictive
  rules cannot be erased by a later wall-clock timestamp from an offline device.
- Independent issuers do not share a sequence counter. Conflicts within one issuer's
  history are different from two issuers disagreeing about an account. Retain both
  claims and apply the versioned community aggregation/personal policy to disagreement.
- Policy-writer permission is an explicit device capability. Ordinary message signing
  cannot modify lists. Check authority on importing operations; a claimed old timestamp
  from a revoked device does not prove authorization before revocation.
- Known publisher/key compromise invalidates dependent reputation caches and positive
  standing pending authorized refresh. Preserve historical provenance; do not silently
  turn uncertain old negative claims into permanent fact. Reverting to unknown standing
  does not remove baseline feature access. Apply the community policy's bounded
  stale-evidence behavior to affected claims. A signed compromise/recovery update must
  not be blocked by ordinary social mute.
- Persist operations, causal state and retractions atomically. Restore cannot resurrect
  removed permissions or discard known blocks. Compact only with authenticated
  checkpoints and explicit device acknowledgement/retirement rules; an old device
  must resync before writing beyond a checkpoint it does not understand.
- A newly enrolled device receives a verified private policy snapshot before normal
  inbound notifications or privileged actions. Until then it uses the restricted
  onboarding state, not an empty “allow everyone” policy. This protects an existing
  owner's rules; it is not a zero-reputation gate. A genuinely new account initializes
  the documented default policy and can participate immediately without endorsements.
- Local blocks and explicit choices continue offline against known valid state.
  Known identity-authority conflicts prohibit new privileged operations. Missing graph
  edges provide no positive standing and no adverse inference. Stale evidence cannot
  invent positive reputation. An expired claim stops asserting its label and the
  remaining evidence is re-evaluated; losing reputation information is not a new ban.
- Community-source expiry/outage handling is declared up front. If no still-valid
  adverse evidence or personal restriction applies, permit normal use with an
  unavailable/unknown warning. Previously valid adverse evidence may retain filtering
  only within the specified validity/staleness window, with age and reason visible.
  Do not hold all new users merely because a provider is offline. Source changes and
  personal overrides can resolve social filtering. Never invent a clean reputation.
- A device cannot enforce a block/revocation it has not received. Show synchronization
  status and limitations; no immediate all-device or network-wide guarantee. Trusted
  time/skew bounds, refresh budgets, compaction and maximum offline windows must be
  specified before the persistence/protocol implementation is accepted.

## 7. Threat model and bounded influence

| Failure/attack | Required response | Remaining limit |
| --- | --- | --- |
| Many fake accounts endorse each other | No raw global vote/count establishes good standing or adverse consensus. Bound influence per eligible source/root, collapse repeated/mirrored evidence and limit graph work. | An eligible source may itself be malicious or fooled. Multiple keys do not establish independence. |
| Compromised friend or list publisher | Limit adopted scope; revoke subscription/introducer rights, preserve provenance and re-evaluate cached decisions. Preview large list changes. | Even a scoped malicious list can hide legitimate content or recommend spam. |
| Negative-label brigading | Automatic aggregation accepts only eligible evidence with bounded influence; no guilt by association. Show provenance and preserve Spam review/personal overrides. | Community judgment can still be wrong; claims are opinions/evidence, not proof of wrongdoing. |
| Name changes, key/device changes and new accounts | Apply rules to stable accounts through verified continuity. New unlinked accounts inherit no whitelist/reputation. | A determined actor can make a fresh account; blocks are not proof of personhood. |
| Stale/forked updates or restored backups | Retain causal state/retractions, detect conflicts, restrict affected decisions and resync. | Network partitions hide unseen updates. |
| Huge graphs/lists or endless unknown messages | Bound edges, depth, bytes, verification work, issuer influence, queues and retention; add aggregate limits as well as per-account limits. | Sybils can bypass a purely per-account quota; numeric limits need measured budgets. |
| Provider/relay censorship or isolation | No mandatory list provider; keep local rules and multiple evidence paths. Keep transport diversity separate from social circles. | No decentralized design guarantees discovery/availability under total isolation. |
| Private graph leakage | Private policy storage, explicit sharing and local explanations; no raw private graph sent to a scorer. | Public endorsements, traffic patterns and shared screenshots can reveal relationships. |

Evaluate graph influence only over eligible, authenticated data. Unknown or truncated
parts are marked incomplete; do not convert absence to condemnation. Automatic source
bootstrap must work without per-list manual setup, but its roots and update authority
remain to be designed openly. No unchangeable global trust anchors, proof-of-personhood
requirement, mandatory domain purchase, identity fee or blockchain registry is selected.

### Limited spam protection: feasible direction, not a completed implementation

Existing libp2p supports bounding connections/streams and resource usage; those controls
are a foundation, not an application-level inbox filter. Source checked 2026-09-16:
[libp2p DoS mitigation](https://libp2p.io/docs/dos-mitigation/) and
[go-libp2p resource manager](https://github.com/libp2p/go-libp2p/blob/master/p2p/host/resource-manager/README.md).

Proposed application controls, to validate in the later implementation:

- Permit useful ordinary traffic from day one. Use short-window burst/byte budgets and
  recoverable cooldowns for excessive messages, requests or call attempts, not a
  permanent novice tier or a requirement to earn reputation to lift the restriction.
- Bound both per-account traffic and aggregate recipient/process load; many fresh
  identities defeat a per-account limit by itself. Do not infer a bad social reputation
  from a shared IP address, a retry, a provider outage or a single exhausted budget.
- Deduplicate authenticated object IDs and retries; coalesce repeated notifications
  and repeated calls. Do not double-count one signed item delivered by several devices.
- Bound Spam storage, attachment sizes/downloads, graph work and verification cost.
  Valid excess may be deferred or dropped at resource limits; make truncation/retention
  visible. Never promise an unlimited recoverable Spam archive during a flood.
- Treat throttling as a local delivery condition with a reason and retry behavior,
  not automatically as a published accusation. Personal allow/restore can override
  reputation filtering but cannot remove hard resource or cryptographic checks.

No mandatory paid access, mining challenge, phone verification, reputation prerequisite
or newcomer-only feature ban is proposed. Exact budgets require workload measurements
and tests; the feasibility finding is limited protection is practical to design, not
that spam or fabricated identities can be eliminated.

## 8. Identity, payments and calls: changes to the earlier proposal

Trust and identity must be designed together at the boundary: trust records need a
stable account subject and authority verification, while identity recovery must not
depend implicitly on a changing reputation graph. This does not require finishing
every trust feature before building any identity code. Freeze their shared semantics
first, then implement bounded pieces.

- Account control must delegate policy editing and endorsement/list publishing
  separately from messaging. Normal device replacement preserves rules and reputation
  against the same verified account. Contested control/recovery suspends use of disputed authority;
  a name claim cannot move either whitelist benefits or blacklist restrictions.
- Legacy peer migration keeps original signed messages/requests and rules. Carry
  negative restrictions conservatively through a verified account link. Do not turn
  a peer-only whitelist into permission for every device of a newly linked account
  without an explicit migration choice. Contradictory mappings remain unresolved.
- Contacts may use local trust context to help choose among same-name search results.
  This does not provide a global namespace or remove the later N1 product decision.
- Payment-request permission means permission to present a request. Payment approval
  still requires native review of the exact authenticated account, request digest,
  asset/network/amount/receiver and selected wallet. “Trusted” never becomes auto-pay.
  Zero reputation is a displayed warning, not a reason to disable request creation
  or payment; normal explicit recipient restrictions and validity checks still apply.
- A policy change before submission invalidates the relevant decision/approval binding
  and requires re-evaluation; a new allowance cannot reuse an old approval. Submission
  already attempted requires chain reconciliation; blocking cannot reverse it. Keep
  valid cancellation/expiry processing active for held requests without new notifications.
- The earlier single payment-execution-device proposal is still provisional. Trust
  lists do not solve cross-device duplicate spending or execution-authority transfer.
- WebRTC paths use the same caller/account rules. Gate invitations before ringing,
  signaling disclosures and media access. A later call contract must define which
  device wins acceptance and what blocking/revocation does to an active call. Transport
  connection and social permission remain distinct.

## 9. User-visible controls and scenarios

Normal use requires no initial reputation or list-configuration project. Show clear
unknown/adverse/disputed reputation context near a person's content and interactions.
Everyday controls include Spam/Filtered, View once, Restore, Always allow, Mute,
Block, Add to circle and Why was this filtered? Advanced settings manage community
sources, personal scopes, introducers and private/public sharing. Avoid an unexplained
trust number, repeated modal warnings or requiring users to edit graph algorithms.

Example explanations: “No established reputation yet”; “Filtered by community spam
evidence”; “You blocked this account”; “Allowed by your personal exception”;
“Introduced by Alice for discovery”;
“Waiting for this device's trust settings to synchronize.” Show the action needed to
change the result locally without contacting the issuer. Do not imply an appeal can
force a third party to retract its claim.

| Scenario | Required result / future acceptance case |
| --- | --- |
| A new account has no reputation, no adverse evidence and no personal restriction | Ordinary posts, discovery, messages, requests and calls work with a warning; no default quarantine, hidden actions or endorsement requirement. |
| Alice introduces Carol for discovery | Carol gains recommendation context; ordinary use never depended on that introduction, and recovery/spending authority is unchanged. |
| Community judgment qualifies content for filtering | Keep valid accepted content reviewable in Spam with a reason and override, subject to visible retention/resource limits. |
| User selects View once on a filtered message/payment card | Show it safely without a read receipt, remote-media fetch, whitelisting, call or payment side effect. |
| Personal message allowance conflicts with community filtering | Exact allowance wins for messages and survives community updates; a personal all-social Block still wins. |
| A shared whitelist includes someone personally blocked | The block remains; importing the list cannot undo it. |
| One person belongs to two circles with conflicting call rules | Restrictive circle rule wins, visibly; an explicit scoped personal decision can resolve it. |
| A malicious source adds account recovery or wallet authority to its list | No expansion beyond social reputation/filtering; no authority to grant either capability. |
| Phone blocks while an offline desktop unblocks an older block | Concurrent block survives synchronization; no timestamp override. |
| An old device restores a backup with a deleted whitelist | Verified checkpoint/retraction prevents reactivation; stale device resyncs. |
| A newly enrolled phone has not received private policy | No default-open inbox or call ringing while policy sync is incomplete. |
| A blocked payee cancels a request already on file | Valid exact cancellation disables payment, with no reopened social channel. |
| A blocked author sends through a different cache/route | Policy uses the verified author; changing route does not evade it. |
| A publisher retracts a label or a user unsubscribes | Recompute derived effects; personal decisions and original evidence remain distinct. |
| Thousands of sybil paths repeat one report/introducer | No multiplied independent support or automatically fabricated community consensus; work stays bounded. |
| Reputation sources are unavailable for a previously unknown account | Show reputation unavailable and preserve ordinary use; no outage-created blacklist. |
| A newcomer bursts past the traffic budget | Temporary local throttling with recovery, while ordinary rates and all feature types remain available; no reputation prerequisite. |
| Same signed message arrives through multiple devices | Deduplicate storage, budget accounting and alerts; do not fabricate multiple interactions or endorsements. |
| A different account acquires an old readable handle | No transfer of saved trust or an approved recipient binding. |
| User blocks during a queued send/native approval | Unsent social work stops; unsubmitted payment re-evaluates. Already submitted funds require reconciliation. |

These are behavior requirements for later meaningful tests, not authored test source
or claimed passing results.

## 10. Implementation sequence and review decision

1. **Product contract recorded:** T1 selects automatic community filtering, reviewable
   Spam and personal overrides, with full ordinary access plus warnings for unknown
   accounts. Next specify community source/bootstrap policy, bounded aggregation,
   evidence/decay thresholds and behavior-based spam budgets. Select identity authority
   only after it satisfies subjects, delegation, continuity and revocation requirements.
2. **Daemon foundation:** a deterministic policy evaluator with immutable input
   snapshots and explanations, then private durable operations, conflict/restart
   semantics and authenticated local management. Candidate boundary is a new
   `modern/trust` package; exact source paths/commands need a subsequent bounded ticket.
3. **First useful end-to-end slice:** personal allow/block/circle rules and the shared
   filtering/review evaluator in Messages, zero-reputation warnings, Spam/restore,
   outgoing-queue enforcement, automatic UI changes and cancellation safety.
   This can start with explicitly typed legacy-peer subjects without pretending they
   are portable accounts; migration preserves their scope.
4. **Portable/shared layer:** selected account verifier, private device synchronization,
   authenticated community-source synchronization, automatic reputation updates and
   revocation. A local-only foundation is not acceptance of the requested automatic
   community system. Freeze wire format, resource
   limits and local/native boundaries together before calling it cross-device trust.
5. **Introductions and later calls:** bounded discovery graph, explainable effects,
   private-sharing options and call enforcement. Graph ranking does not block useful
   personal protections; transport optimization is an independent later task.

One source phase and one execution/evidence phase per cohesive slice, subject to the
repository role rules; no handoff per design subsection. No implementation actor is
selected or launched by this proposal, and no work is assigned to Hermes now.

**Review outcome:** T1 is resolved by the owner's automatic-filtering/Spam/newcomer
requirements. The previous A recommendation and novice restrictions are superseded.
No further product confirmation is requested for those requirements. The full trust
protocol is not frozen; community-source/bootstrap and aggregation rules, exact signed
format, authority-method selection, private sync, resource/freshness bounds and the
authenticated policy API remain reviewer engineering work. New account/payment
contracts remain gated on the shared semantics; no implementation actor is authorized.

Initial research verification at commit `46c0f2367`: read source/specifications and
inspect the scoped diff; 44 local document links had zero missing targets and scoped
`git diff --check` exited 0. This owner-decision revision updates the behavior in place
and retains the source inspection baseline. Its repeated document reference audit
checked 44 local links with zero missing targets; scoped `git diff --check` exited 0.
No product/test code changes, tests, acceptance commands,
security scans, builds, live-node/wallet operations or private user-data reads.
Publication scope is the six reviewer-authored governance paths enumerated in the
[research scope](BB-TRUST-RESEARCH-SCOPE-01.md); unrelated work is preserved.
