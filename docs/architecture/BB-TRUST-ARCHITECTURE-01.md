# Ring of trust — research and architecture proposal

Reviewer: Codex. Started 2026-09-16 against the
[owner's research scope](BB-TRUST-RESEARCH-SCOPE-01.md).
Status: initial research complete; integrated proposal for product review.
No trust implementation, protocol format or identity method is authorized.
Owner reports selecting xhigh; higher reasoning effort is not requested.

## Recommendation in plain language

Build a user-controlled trust system with named circles, personal blacklists and
whitelists, optional subscriptions to other people's lists, and scoped introductions.
Evaluate it locally against authenticated accounts. Show why each rule took effect.

Being in a circle answers a particular question: “May this person message me?”,
“Do I use this person's spam judgments?”, or “Whose introductions do I consider?”
Following someone, accepting a payment request, trusting a relay and authorizing a
recovery helper are different decisions. A single reputation score cannot express them.

The recommendation is proposal A below. It supports automatic application of a list
**after** an explicit subscription, without making every social connection a source
of inherited rules. This is a BitBook design recommendation inferred from the research,
not a claim that an existing protocol supplies the complete feature.

## T1: control over inherited rules

The owner requires extensive rings of trust, blacklists and whitelists. It is not yet
established whether other people's lists should act automatically. This product choice
is recorded before asking; the answer will be retained here.

| Choice | Behavior |
| --- | --- |
| A — recommended | Personal rules take priority. Other people's lists are recommendations until the user explicitly subscribes to specified effects. |
| B | Automatically inherit allow/block decisions within the user's selected trust rings, with explicit exceptions. |
| C | Personal circles and lists only; other people's rules never affect the user's interactions. |

Owner answer: pending. A is a reviewer recommendation, not owner approval. Research
on provenance, privacy, revocation, account continuity and threat models proceeds
independently of this answer. Naming choice N1 remains deferred.

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

The useful combination is explicit lists/circles plus authenticated, scoped assertions
and a local decision engine. No reviewed system eliminates policy choice, privacy
tradeoffs, compromised trusted users or the need for resource limits.

## 3. Model: circles, introductions and authority

Use “ring” in the product as an understandable view over these distinct concepts:

| Concept | Meaning | Does not imply |
| --- | --- | --- |
| Named circle | An owner-managed set such as Friends, Work or Local community; membership may overlap. | Members know each other, endorse each other, or receive every permission. |
| Personal blacklist/whitelist | The owner's deny/allow choice for a typed target and specified actions. | A public accusation or an identity/key-validity judgment. |
| Introducer | An account the owner chooses to recommend other accounts for a stated purpose. | General permission to select recovery keys, spend money or change local policy. |
| Introduction distance | A derived view of eligible directed endorsement paths from the owner. | Authority over the owner, or a count of independent humans. |
| Shared list/label | An issuer's signed claim, with subject, scope, version and optional reason/expiry. | Automatic adoption by recipients or a network-wide ban. |
| Subscription | The owner's choice of issuer/list, scope and maximum automatic effect. | Following that issuer's other lists or any list it subsequently names. |

Recommendation: named circles are the main editing UI; show introduction distance as
an optional discovery aid. There is no total ordering of people from “least” to
“most” trustworthy. A person can be trusted for technical introductions but denied calls.

Start with direct chosen introducers and at most one further endorsement edge for
discovery recommendations (two edges from the owner). Do not count cycles or repeated
paths as extra support. Do not let an endorsed stranger become an introducer by default.
Deeper paths require explicit delegable scopes/depth limits and their own evaluation
before enabling them. The first depth is a proposed product default, not a security
theorem or a selected on-wire constant.

Blacklisting a person does not automatically blacklist their friends. Endorsing a
person does not whitelist their friends. A subscriber can intentionally consume a
chosen issuer's deny list; that is direct adoption of a scoped list, not negative
trust spreading through an unlimited graph.

## 4. Actions, defaults and precedence

Proposed scopes are distinct. Exact names are illustrative until the protocol contract:
content visibility, discovery recommendations, incoming messages, attachments,
call invitations, payment requests, outgoing interactions and infrastructure use.
Account administration, policy editing, device enrollment, recovery and wallet spending
use explicit capability authorization outside this social-policy decision.

For each authenticated operation, compute both admission and presentation. Admission
is allow, hold for review or reject. Presentation can show normally, warn, mute or hide.
Content ranking is separate. “Hide” must not accidentally grant or revoke a capability.

Proposed default experience:

- People already explicitly permitted to message can use the normal Messages thread.
  Unknown valid senders go to a bounded message-request queue without typing/read
  disclosure, media fetching or active notifications. Existing follows are not silently
  migrated into message/call/payment permission. Keep review within Messages; this
  does not restore the removed standalone payment Requests tab or connection controls.
- Payment requests have their own permission and review queue; accepting chat alone
  does not grant it. Quarantined requests cannot begin native payment approval.
- Unknown callers do not ring or trigger camera/microphone permission. A permitted
  caller may ring; media still requires the user's explicit acceptance.
- Public content can be discovered under the owner's visibility preferences. No
  trust path means “not known through your chosen introducers,” not “malicious.”
- An allowlist-only mode rejects otherwise unapproved new interactions instead of
  queuing them. An open-inbox mode is possible but retains technical resource limits.
- Blocking preserves old history and payment records. The ordinary Block action stops
  new social interactions and outgoing queue retries for that account; further sends
  require explicit unblock. Mute only affects presentation/notifications.

### Proposed evaluation order for T1-A

| Order | Rule | Conflict behavior |
| --- | --- | --- |
| 0 | Cryptographic validity, recipient binding, schema, device authority and hard resource limits | Policy cannot permit an invalid object or a revoked/unauthorized device. |
| 1 | Owner's explicit all-social block on the target account | Wins over social allows. Unblocking must explicitly resolve this block. Narrow control-object handling is described below. |
| 2 | Owner's exact target/action decision | Overrides circle rules, subscriptions and defaults. Concurrent incompatible decisions are restricted pending resolution. |
| 3 | Owner's circle rules for that action | Among applicable rules, reject beats hold; hold beats allow. UI previews conflicting memberships. |
| 4 | Explicitly activated shared-list policies | Same restrictive ordering, within the subscribed scope/effect ceiling. No majority vote over unrelated issuers. |
| 5 | Action default | Introduction ranking alone never raises message, call or payment admission. |

An exact personal message allowance can override a subscribed spam-list restriction
for messages without overriding a full personal Block or allowing calls. An allowlist
entry does not bypass signatures, expiry, wallet confirmation, quotas or device authority.
Removing an inherited denial falls back to the remaining rules, not an implied whitelist.

Subscriptions select recommendation/warning, review-queue or deny behavior explicitly.
Positive shared lists can grant only the specific interaction permission the user chose
to import; the recommended subscription starts advisory. Issuer updates within that
approved scope apply automatically. New scopes or greater effects require a new local
choice. Unsubscribe removes derived effects and leaves personal rules intact. Imported
files preview changes before they become local rules; lists contain data, not executable
policy code, scripts, unrestricted patterns or remote commands.

## 5. Enforcement and protected control messages

```mermaid
flowchart TD
    Input[Incoming object or local action] --> Verify[Validate identity, signature and object]
    Verify --> Policy[Daemon evaluates local policy snapshot]
    Own[Personal rules and circles] --> Policy
    Shared[Verified claims from selected sources] --> Policy
    Policy --> Decision[Admission result and local explanation]
    Decision --> Storage[Admit or quarantine before side effects]
    Decision --> UI[Messages, discovery and call presentation]
    Decision --> Native[Native payment review rechecks exact binding]
```

Apply policy before normal persistence, notifications, receipts, attachment fetching
and call signaling side effects. Bounded quarantine is separate from the normal
conversation. Rate limits apply before expensive verification, and further limits
apply to authenticated accounts and accepted objects. Reject malformed data rather
than putting it in a review queue.

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
| Subscription | Exact issuer/list and allowed assertion types, local effect ceiling, freshness handling and optional delegation depth. |
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
  claims and apply local subscription policy to disagreement.
- Policy-writer permission is an explicit device capability. Ordinary message signing
  cannot modify lists. Check authority on importing operations; a claimed old timestamp
  from a revoked device does not prove authorization before revocation.
- Known publisher/key compromise invalidates dependent positive-decision caches and
  suspends new inherited allowances pending authorized refresh. Preserve historical
  provenance; do not silently turn uncertain old negative claims into permanent fact.
  Apply the subscription's declared stale-evidence behavior while resolving affected
  claims. A signed compromise/recovery update must not be blocked by ordinary social mute.
- Persist operations, causal state and retractions atomically. Restore cannot resurrect
  removed permissions or discard known blocks. Compact only with authenticated
  checkpoints and explicit device acknowledgement/retirement rules; an old device
  must resync before writing beyond a checkpoint it does not understand.
- A newly enrolled device receives a verified private policy snapshot before normal
  inbound notifications or privileged actions. Until then it uses the restricted
  onboarding state, not an empty “allow everyone” policy.
- Local blocks and explicit permissions continue offline against known valid state.
  Known authority conflicts prohibit new privileged operations. Missing graph edges
  provide no positive recommendation. Stale evidence cannot create a new inherited
  allowance. An expired claim stops asserting its label; a review hold may remain
  because evidence is unavailable, with that reason clearly distinguished.
- Subscription expiry/outage handling is declared up front. Recommended behavior for
  unresolved new interactions is a bounded hold for review, while existing explicit
  local permissions remain usable unless a higher-priority rule/authority check fails.
  Unsubscribe or an explicit personal choice can resolve social holds. Never describe
  unavailable evidence as a clean reputation.
- A device cannot enforce a block/revocation it has not received. Show synchronization
  status and limitations; no immediate all-device or network-wide guarantee. Trusted
  time/skew bounds, refresh budgets, compaction and maximum offline windows must be
  specified before the persistence/protocol implementation is accepted.

## 7. Threat model and bounded influence

| Failure/attack | Required response | Remaining limit |
| --- | --- | --- |
| Many fake accounts endorse each other | No global vote/count creates permission. Count distinct configured introducer roots, not every descendant/path; deduplicate identities and bound graph work. | A chosen introducer may itself be malicious or fooled. Multiple keys do not establish independence. |
| Compromised friend or list publisher | Limit adopted scope; revoke subscription/introducer rights, preserve provenance and re-evaluate cached decisions. Preview large list changes. | Even a scoped malicious list can hide legitimate content or recommend spam. |
| Negative-label brigading | Unsubscribed accusations are not automatically adopted; no negative trust propagation through friends. Explanations identify the actual adopted issuer. | A user may choose a poor source; content claims are opinions/evidence, not proof of wrongdoing. |
| Name changes, key/device changes and new accounts | Apply rules to stable accounts through verified continuity. New unlinked accounts inherit no whitelist/reputation. | A determined actor can make a fresh account; blocks are not proof of personhood. |
| Stale/forked updates or restored backups | Retain causal state/retractions, detect conflicts, restrict affected decisions and resync. | Network partitions hide unseen updates. |
| Huge graphs/lists or endless unknown messages | Bound edges, depth, bytes, verification work, issuer influence, queues and retention; add aggregate limits as well as per-account limits. | Sybils can bypass a purely per-account quota; numeric limits need measured budgets. |
| Provider/relay censorship or isolation | No mandatory list provider; keep local rules and multiple evidence paths. Keep transport diversity separate from social circles. | No decentralized design guarantees discovery/availability under total isolation. |
| Private graph leakage | Private policy storage, explicit sharing and local explanations; no raw private graph sent to a scorer. | Public endorsements, traffic patterns and shared screenshots can reveal relationships. |

Evaluate graph influence only over selected, authenticated data. Unknown or truncated
parts are marked incomplete; do not convert absence to condemnation. No automatically
recruited global trust anchors, proof-of-personhood requirement, mandatory domain
purchase, identity fee or blockchain registry is selected by this research.

## 8. Identity, payments and calls: changes to the earlier proposal

Trust and identity must be designed together at the boundary: trust records need a
stable account subject and authority verification, while identity recovery must not
depend implicitly on a changing reputation graph. This does not require finishing
every trust feature before building any identity code. Freeze their shared semantics
first, then implement bounded pieces.

- Account control must delegate policy editing and endorsement/list publishing
  separately from messaging. Normal device replacement preserves rules against the
  same verified account. Contested control/recovery suspends new inherited privileges;
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

Put everyday actions on the person/conversation: Mute, Block, Allow messages, Allow
payment requests, Allow calls, Add to circle, and Why is this restricted? Advanced
Trust settings manage introducers, shared-list subscriptions and private/public sharing.
Avoid an unexplained trust number or requiring users to edit graph algorithms.

Example explanations: “You blocked this account”; “Messages allowed by your Work
circle”; “Held by the spam list you subscribed to”; “Introduced by Alice for discovery”;
“Waiting for this device's trust settings to synchronize.” Show the action needed to
change the result locally without contacting the issuer. Do not imply an appeal can
force a third party to retract its claim.

| Scenario | Required result / future acceptance case |
| --- | --- |
| Bob is followed but has never been allowed to call | Following does not make his call ring. |
| Alice introduces Carol for discovery | Carol can be recommended; she gains no payment/call/recovery permission. |
| Personal message allowance conflicts with a subscribed spam denial | Exact allowance wins for messages; the UI explains the exception. A personal all-social Block still wins. |
| A shared whitelist includes someone personally blocked | The block remains; importing the list cannot undo it. |
| One person belongs to two circles with conflicting call rules | Restrictive circle rule wins, visibly; an explicit scoped personal decision can resolve it. |
| A malicious list adds a new action scope or another list subscription | No expansion beyond the user's stored subscription ceiling. |
| Phone blocks while an offline desktop unblocks an older block | Concurrent block survives synchronization; no timestamp override. |
| An old device restores a backup with a deleted whitelist | Verified checkpoint/retraction prevents reactivation; stale device resyncs. |
| A newly enrolled phone has not received private policy | No default-open inbox or call ringing while policy sync is incomplete. |
| A blocked payee cancels a request already on file | Valid exact cancellation disables payment, with no reopened social channel. |
| A blocked author sends through a different cache/route | Policy uses the verified author; changing route does not evade it. |
| A publisher retracts a label or a user unsubscribes | Recompute derived effects; personal decisions and original evidence remain distinct. |
| Thousands of sybil paths share one chosen introducer | No multiplied independent support or new admission authority; work stays bounded. |
| A different account acquires an old readable handle | No transfer of saved trust or an approved recipient binding. |
| User blocks during a queued send/native approval | Unsent social work stops; unsubmitted payment re-evaluates. Already submitted funds require reconciliation. |

These are behavior requirements for later meaningful tests, not authored test source
or claimed passing results.

## 10. Implementation sequence and review decision

1. **Product contract:** resolve T1 and review the proposed defaults/precedence above.
   Keep custom circles plus bounded introductions; no global reputation-to-permission
   algorithm. Select identity authority only after it can satisfy the required typed
   subjects, delegation, continuity and revocation behavior.
2. **Daemon foundation:** a deterministic policy evaluator with immutable input
   snapshots and explanations, then private durable operations, conflict/restart
   semantics and authenticated local management. Candidate boundary is a new
   `modern/trust` package; exact source paths/commands need a subsequent bounded ticket.
3. **First useful end-to-end slice:** personal allow/block/circle rules in Messages,
   admission/outgoing-queue enforcement, automatic UI changes and cancellation safety.
   This can start with explicitly typed legacy-peer subjects without pretending they
   are portable accounts; migration preserves their scope.
4. **Portable/shared layer:** selected account verifier, private device synchronization,
   signed shared lists/subscriptions and revocation. Freeze wire format, resource
   limits and local/native boundaries together before calling it cross-device trust.
5. **Introductions and later calls:** bounded discovery graph, explainable effects,
   private-sharing options and call enforcement. Graph ranking does not block useful
   personal protections; transport optimization is an independent later task.

One source phase and one execution/evidence phase per cohesive slice, subject to the
repository role rules; no handoff per design subsection. No implementation actor is
selected or launched by this proposal, and no work is assigned to Hermes now.

**Review outcome:** initial comparison and architecture proposal are complete. T1
remains an owner choice; recommendation A is used only to make this proposal concrete.
The full trust design is not accepted/frozen, and new account/payment contracts remain
gated on its shared semantics. Exact signing format, authority-method selection,
private-sync protocol, numeric resource/freshness bounds and authenticated policy
mutation API remain engineering work, not questions delegated to the owner.

Verification for this documentation work: read source/specifications and inspect the
exact scoped Git diff. Documentation reference audit checked 44 local links across
the six records with zero missing targets; scoped `git diff --check` exited 0.
No product/test code changes, tests, acceptance commands,
security scans, builds, live-node/wallet operations or private user-data reads.
Publication scope is the six reviewer-authored governance paths enumerated in the
[research scope](BB-TRUST-RESEARCH-SCOPE-01.md); unrelated work is preserved.
