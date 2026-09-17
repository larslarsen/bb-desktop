# Portable accounts and recipient migration

Reviewer: Codex, 2026-09-16. Status: architecture proposal; no implementation
authorized. This develops the [identity/transport direction](BB-IDENTITY-TRANSPORT-DIRECTION-01.md).
PAY-001 remains complete. This proposal does not claim that account migration exists
or that its protocol gate has passed.

Owner priority update: [ring-of-trust research](BB-TRUST-RESEARCH-SCOPE-01.md),
including blacklists and whitelists, comes first. This proposal's choices remain
provisional and must be reassessed against that design. The owner directs continuing
the project; N1 is not a prerequisite for portable-account engineering. Naming remains
a provisional direction with reservations, not a solved problem or a global namespace decision.

The [integrated trust proposal](BB-TRUST-ARCHITECTURE-01.md) now records the first
research pass. Its sections 6 and 8 add account requirements: separate policy-writer
and assertion-publisher capabilities, private policy synchronization, conservative
revocation/conflict handling, and preservation of trust scope during legacy migration.
Social trust does not implicitly appoint account-recovery authorities. No identity
method is selected by that proposal. T1 is now resolved: automatic community filtering
with reviewable Spam/personal overrides; zero reputation means warnings and normal
feature access. These choices do not grant recovery or wallet-spending authority.
Section 3A now selects a replaceable community profile, bounded explicit assessor
delegation, distinct-issuer quorum evaluation and expiry. Account authority must verify
assessment and profile-update capabilities separately from message signing, support
causal revocation without resetting a source's identity, and never infer recovery
authority from participation standing. The real source roster is a later activation
requirement; it does not prevent assessing the account method now.

## Product result

Alice adds her phone to her existing BitBook account. Bob still sees one Alice in
Messages. Her messages and requests belong to that account even when a different
authorized device delivers them. Replacing a phone does not require Bob to replace
his contact. Names are visible; cryptographic identifiers belong in contact details,
verification and share links, not routine typing.

WebRTC can carry the same authenticated conversations. A future video call rings
the account's eligible devices, and accepting on one device ends the other rings.
Neither connecting a device nor accepting a call authorizes a wallet payment.

## Readable names — provisional direction, not a project gate (N1)

Latest owner direction, 2026-09-16: continue the project. The reviewer had not established
that choosing global name uniqueness was necessary to continue account portability.
Earlier instructions to obtain a naming decision before continuing are superseded.

The owner tentatively accepted the proposal with explicit reservations. Retain readable
names, the owner's ad-image/name-similarity proposal, optional verified domains and
reviewable trust filtering as provisional direction. Do not report a confident final
endorsement, guaranteed impersonation prevention, or a selected global namespace.
Do not reopen a general naming debate unless a specific feature requires a decision.

Account authority, device enrollment/revocation and signed recipient binding use stable
account references independently of presentation names. Their engineering review can
proceed. A future naming feature must explain its own dependency and behavior; it must
not become a prerequisite for unrelated project work. The next work below concerns the
actual desktop/phone requirement, not another naming choice for the owner.

## Reviewer account decisions

1. Contacts and new signed recipient contracts bind to an account, not a display name,
   daemon peer ID, IP address or transport session. Keep these as separate typed values.
2. Each device keeps its own keys. Account authority explicitly delegates allowed
   operations to devices; ordinary messaging permission does not grant enrollment,
   recovery, payment-request signing or wallet spending permission.
3. A saved contact pins the account identity. A name reassigned to another account
   never redirects that contact, a queued message or an approved request.
4. Recovery and device revocation must be designed before advertising portable
   accounts. Restoring account authority and restoring private message history are
   separate operations. Losing every authorized recovery factor can lose the account.
5. Keep all v1 signed bytes and their original peer-based interpretation. Introduce a
   separately versioned account protocol. A mapping can group old history, but cannot
   make a v1 request payable by a different device or silently upgrade its authority.
6. New native payment review consumes an immutable verified recipient/request
   binding. Renderer display text, repeated name lookup and changing network routes
   cannot replace its destination, amount, network or account identity.
7. Social device enrollment transfers no wallet keys or spending authority. Replayed
   delivery to two devices is one logical request, not two payment instructions.

## Account authority: requirements and method shortlist

Account identity must survive replacement of operational device keys. Its controller
state must support rotation, enrollment, revocation and a deliberately configured
recovery path. An authenticated history of those changes is needed; simply fetching
the highest sequence number advertised by an arbitrary peer is insufficient.

| Candidate | Primary-source finding | Reviewer disposition |
| --- | --- | --- |
| Raw permanent public key / `did:key` alone | The method cannot update or deactivate its document; changing its key changes the identifier. | Insufficient for the recoverable account authority. It can identify an individual key; adding a custom recovery log would be a new protocol, not a feature supplied by `did:key`. |
| `did:web` | AT Protocol documents its dependence on retaining the identifying domain. | Optional naming/discovery is plausible; do not make domain ownership the mandatory account identity. |
| `did:plc` | Supports rotation and recovery through signed history, with a central directory collecting and validating operations. | Useful reference. Do not silently adopt that directory as a required BitBook service. |
| KERI transferable identifiers | Specifies key-event history, rotation and mechanisms for detecting conflicting histories. | Candidate for a separate compatibility/implementation assessment; no library or deployment selected. A distributed trust model still needs a concrete policy for witnesses, freshness and unavailable services. |

Sources checked 2026-09-16: [did:key method](https://w3c-ccg.github.io/did-key-spec/),
[AT Protocol DIDs](https://atproto.com/specs/did),
[DID PLC](https://github.com/did-method-plc/did-method-plc),
[KERI specification](https://trustoverip.github.io/kswg-keri-specification/).
These findings explain the shortlist; none establishes library suitability or a
production-ready BitBook recovery system.

**Recommendation:** evaluate an existing transferable-identity method before inventing
a BitBook key-rotation protocol. Keep the account identifier opaque outside the
identity verifier. The choice of method is an engineering gate, not a request for the
owner to choose cryptography. No `did:bb` syntax, new algorithm or dependency is
approved by this proposal.

### Initial implementation-fit assessment, 2026-09-16

Read-only daemon baseline remains `cd497749a771b063300e2c3edf8748fe285b06c4`.
Inspected `modern/network/identity.go`, `modern/direct/types.go` and `modern/go.mod`;
these files have no working-tree diff. The daemon persists one libp2p key and its
message/conversation records use peer IDs. This gives restart continuity for that
daemon; it supplies no separate desktop/phone account authority. A new phone's own
key must be authorized under the same account, and removing that phone must leave
the other device authorized. This is the concrete portability requirement being assessed.

| Evidence checked | Consequence for selection |
| --- | --- |
| [DIF's Go implementation, kerigo](https://github.com/decentralized-identity/kerigo) is archived; its README describes runnable witness/validator services as future work. | Do not select that repository as a maintained, complete drop-in authority implementation. This does not establish that no suitable Go implementation exists. |
| [KERIpy's documented dependencies](https://github.com/WebOfTrust/keripy) include Python and libsodium. | It is an implementation reference; adopting it would require an explicit desktop/mobile packaging and process-boundary assessment, not simply adding a Go module. |
| [KERI's rotation and recovery rules](https://trustoverip.github.io/kswg-keri-specification/) depend on previously committed next keys and specify which competing events can supersede others. | Preserving an identifier through rotation is supported in the design. A backup/recovery promise still needs a concrete key-custody profile; witness receipts alone do not repair theft of the next authority keys. |

**Reviewer disposition:** KERI remains a candidate, not a selected dependency or a
completed recovery design. The next comparison must establish a supportable verifier
for the actual desktop/phone deployment and map the existing device capabilities to
it. Do not require the owner to choose cryptography or introduce a mandatory hosted
identity service to bypass this assessment. No implementation, build or runtime
verification was performed. Naming remains outside this selection.

### Pinned Go implementation review, 2026-09-16

**Decision: do not adopt `github.com/grapeid/keri-go` v0.1.5 unchanged.**
Source reviewed at commit `f06d1dac5a36dea4b619eb6934e43e6fdc68b835`, which the upstream
GitHub tag API identifies as v0.1.5. This is a separate implementation from the archived
DIF `kerigo`; the earlier archived-library finding did not exhaust Go candidates.

Reviewed the pinned README, module manifest, validation, signed-log, delegation,
duplicity and keychain sources plus mobile documentation. The manifest declares Go
1.24 and BLAKE3 dependencies. This is compatible in declared language level with the
current Go 1.27 daemon, but no compilation or integration was performed.

| Finding | Evidence and consequence |
| --- | --- |
| Rotation does not enforce the prior next-key threshold against actual signers in the reviewed path | `checkPrecommitment` checks the old threshold against public keys *listed* in the rotation. `ValidateSignedKEL` then switches to the rotation's new threshold before checking signatures. The verified signer set is not checked against the old threshold. A rotation listing two previously committed keys, setting its new threshold to one and carrying one valid signature follows those checks despite a prior threshold of two. This conflicts with KERI's dual-threshold rule and prevents accepting the package unchanged for account authority. This is source reasoning, not an executed reproduction. |
| Malformed threshold parsing can leave validation without a threshold | `ParseEvent` discards errors from `parseThreshold`; the signature path enforces the threshold only when it is non-nil. Presence/order checks do not validate the threshold value's type. A strict caller would need to reject such inputs before trusting the result. |
| The reviewed APIs do not provide reconciliation of an already observed history with a valid superseding recovery | `ValidateKEL` checks a contiguous single chain; `Watcher.Observe` reports differing events at the same sequence as duplicity. Neither reviewed path applies KERI's superseding-recovery rules. The application cannot equate every such result with an unrecoverable account conflict. |
| Mobile readiness remains limited evidence | The upstream mobile note reports cross-compilation/binding but explicitly says nothing has run on a device. Those are upstream claims, not BitBook acceptance results. |

Pinned primary evidence:
[precommitment and parsing](https://github.com/grapeid/keri-go/blob/f06d1dac5a36dea4b619eb6934e43e6fdc68b835/validation.go),
[signature validation](https://github.com/grapeid/keri-go/blob/f06d1dac5a36dea4b619eb6934e43e6fdc68b835/signed_kel.go),
[observed-history handling](https://github.com/grapeid/keri-go/blob/f06d1dac5a36dea4b619eb6934e43e6fdc68b835/duplicity.go),
[mobile limitations](https://github.com/grapeid/keri-go/blob/f06d1dac5a36dea4b619eb6934e43e6fdc68b835/MOBILE.md).
The [KERI specification](https://trustoverip.github.io/kswg-keri-specification/) sections
“Rotation using pre-rotation” and “Superseding Recovery” supply the comparison rules.
The README's conformance/interoperability pass counts were not independently reproduced.
No dependency was installed, no upstream code was executed or modified, and no upstream
report was sent. A reproduction/correction would require a bounded source/test task.

This completes the initial source screen of this candidate, not the entire account
method selection. It rules out a particular adoption path rather than blocking the
whole project. Next assess an established primary-key/device-subkey model against the
narrow portability requirement, including its root-key backup/compromise limitation.
The existing shortlist's rejection of `did:key` *alone* must not be misread as proof
that a backed-up stable controller with separately revocable device keys cannot work.
No identity-method decision is delegated to the owner.

Review checks: inspected the two-file diff, checked 28 local document links with no
missing targets, and ran scoped `git diff HEAD --check` (exit 0). These are document
checks, not execution evidence for the candidate or the daemon.

### Required behavior of the selected method/profile

- New-device pairing authenticates both endpoints and displays a confirmation tied
  to the new device's keys. Discovery proximity or a matching name is insufficient.
- Enrollment grants specific capabilities and binds the device's application signing
  key and transport identity to the account. Route updates cannot expand capabilities.
- Keep recovery authority separate from routine online device keys. Define where it
  is held, how it is backed up and which factor can replace a lost device. Do not
  promise recovery after loss of all factors or theft of every recovery factor.
- Account-control changes follow the selected method's ordering/conflict rules.
  Persist verified state and evidence atomically; restarting must not discard a known
  revocation or allow an older state to restore a removed device.
- Distinguish verified historical membership from current authority. A device's
  claimed timestamp cannot prove it signed before revocation. Retain old accepted
  history with its provenance; newly arriving objects must satisfy the new protocol's
  current-admission rule rather than relying on a backdated timestamp.
- On conflicting authority histories, do not choose by wall-clock time or silently
  overwrite a contact. Block new privileged operations pending a verified resolution.
- A partition can hide a revocation. Pinning prior state only prevents rollback below
  what this verifier already saw. The selected profile must specify refresh/freshness
  rules and the availability tradeoff for new enrollment and payment approval.

## Recipient contract and payment safety

The following are semantic requirements, not a JSON schema or canonical signed format.
Wire fields, domain separators, bounds and test vectors must be frozen together after
the authority method is selected.

| Binding | Meaning |
| --- | --- |
| Account identity + account-state reference | Which enduring counterparty and which verified authority state |
| Signing device + delegation reference | Which key acted, and why it had permission for this operation |
| Intended recipient account | Which account may receive the message/request; routing to one of its devices does not change it |
| Protocol version + operation + object ID/digest | Prevent cross-protocol substitution and recognize the same signed object across routes/devices |
| Payment asset, network, atomic amount, receiver and expiry | Immutable economic terms already required by v1, preserved in the new contract |
| Local approval binding | Exact request digest, selected native wallet/account and intended counterparty reviewed by the user |

The daemon verifies account authority and the signed social/payment request. The
native wallet independently validates its trusted input, receiver/asset/network and
approval binding; it never trusts renderer names as recipient evidence. A specialized
service can be an account without a human social profile. Non-peer service endpoints
need their own explicit counterparty type rather than a fabricated peer ID.

First multi-device payment scope should use **one explicitly designated payment
execution device**, with other devices able to view a request. A request ID stored in
two independent wallets does not provide global exactly-once payment. Transfer of
execution authority needs a durable reconciliation protocol before enabling a second
execution device; an unreachable old device is not proof it never submitted. This is
a proposed limitation for the payment design, not an implementation delivered here.
Normal manual transfers outside the request workflow cannot be deduplicated by BitBook.

At native approval, refresh the required authority evidence and bind the reviewed
account state. Define how revocation/conflict arriving before submission cancels the
unsubmitted intent. Never repeat a name lookup to change the recipient of an old approval.
Unknown submission outcome requires reconciliation, not blind retry. A peer's claim
of payment still does not establish a confirmed chain transaction.

## Message history and synchronization

- Route to verified eligible devices of the account; keep transport delivery attempts
  distinct from the logical message. Deduplicate by account, protocol and object ID,
  and reject conflicting content for an existing ID. Deduplication survives restart.
- Acknowledgement by one device means that device accepted delivery. It does not mean
  every device has synchronized, that the user read the message, or that payment occurred.
- Sync original authenticated records plus provenance. Read markers can advance
  monotonically per conversation/device; an old snapshot cannot undo a known read.
  Cancellation must refer to the exact request and an authorized account operation.
- A new device sees old private history only through an explicitly authorized transfer
  or encrypted backup. Account recovery does not magically recover missing ciphertext
  or its decryption keys. Encryption/key-sharing protocol remains a separate gate.
- Keep private bodies, contact lists and wallet information out of public profile/DHT
  records. Optional offline relays may carry only appropriately encrypted private data;
  no relay storage feature is implied by current signed direct transport.
- Local daemon events trigger automatic UI updates with cursor/snapshot reconciliation
  after missed events or restart. No user-facing Connect or Refresh requirement.

## Versioned migration

1. Add account verification alongside the current peer verifier. Preserve existing
   local data and v1 protocols. New account/device state uses a separate storage schema.
2. Link an existing peer to an account only with proofs from both the old peer key and
   current account authority, bound to the exact migration operation. Account claims,
   display names and photos alone cannot merge old histories. Loss of the old key
   leaves that history unlinked unless separately verified by an explicit future process.
3. Persist migration proofs and prevent one old peer from silently moving between
   conflicting account mappings. Show unresolved conflicts rather than merging them.
4. Negotiate authenticated account-protocol capabilities. Legacy peers stay in a
   clearly identified peer conversation. No automatic account-to-peer downgrade for a
   newly signed payment request. A legacy request retains its original payer/payee.
5. Expose account-aware contacts/conversations and a versioned authenticated local API.
   Desktop social and payment views must agree on the same verified account/device
   binding before they can share a conversation.
6. Migrate public profile/post authorship and follow relationships as well. The current
   signed posts, profile-root publication and following/follower sets are peer-based.
   Multiple devices cannot independently overwrite one account's public root without
   an explicit authenticated update/merge contract. Retain original post signatures.
7. Add account-aware request creation and native approval only after the new verifier,
   native trust input and single-executor rules have retained acceptance evidence.

Migration is additive and reversible at the presentation layer. It does not rewrite
old signatures or promise that an old binary can interpret new account records.

## WebRTC and later calls

Keep TCP/QUIC as available transports and add WebRTC under the same account/device
authentication contract. libp2p distinguishes relay-signaled WebRTC from WebRTC Direct;
select paths for actual desktop/mobile reachability and measure latency rather than
assuming WebRTC is faster. [libp2p transport documentation](https://libp2p.io/docs/webrtc/)

Authenticated call signaling binds caller/callee accounts, participating devices,
call ID and negotiated session. Media permissions, ringing, device selection,
hang-up, reconnect and replay protection belong to a separate calling feature.
Video does not require a different contact model or payment recipient format.
Select media protection, relay policy and mobile background behavior in that feature;
data-channel transport support alone does not deliver video conferencing.

## Source evidence and next gate

Read-only baselines: bb-go `cd497749a771b063300e2c3edf8748fe285b06c4` and
bb-desktop `f778a287864dd2569d201b3e71c686f6ff35d5fd`. Existing unrelated dirty
work is preserved. Inspected implementation:

- Daemon `modern/network/identity.go` persists the device key; `network/node.go`
  publishes/resolves profile roots under that peer identity.
- `modern/direct/types.go` and `direct/service.go` sign sender/recipient peer IDs,
  verify the authenticated remote and group conversations by peer.
- `modern/social/store.go` uses peer identities for follows, profile retrieval and
  signed-post verification; `modern/api/handler.go` exposes peer-addressed social APIs.
- `modern/payment/types.go`, `signature.go` and `service.go` bind v1 requests to
  payer/payee peers, including live remote checks and persisted-record validation.
- `modern/localclient/server.go` reports peer identity. Desktop
  `wallet-pay/inbox-client.js`, `social/payment-inbox.js` and `social/app.js` preserve
  that binding and group the current cards by payee peer.

No product/test source changed and no tests, scans, builds, live nodes or wallet
operations ran for this architecture review. No implementation actor was launched.

Next reviewer work is to assess the shortlisted authority method's implementation and
deployment fit against trust section 3A, then freeze one bounded account-verifier
contract with failure cases for forged enrollment, rollback, revocation, conflicts
and restart.
N1 is provisional and not a gate for this work; no global naming mechanism is assumed.
That contract must name exact source paths and retained evidence once ready. Do not
send an implementer this proposal as an implicit source authorization. Consolidate
source work and execution into their normal role-bounded phases; do not create a
handoff for each individual design subsection.

Reviewer-authored governance scope for this review: this proposal,
`BB-IDENTITY-TRANSPORT-DIRECTION-01.md`, `BBD-PAY-END-TO-END-STATUS-01.md` and
`../handoff/CURRENT_TASK.md`, all in bb-desktop. No daemon publication is part of it;
the daemon's existing direction link reaches this proposal through the direction note.
