# Portable identity, readable names and WebRTC — design constraints

Owner requirements recorded by Codex, 2026-09-16. Status: architecture direction and
migration gate; no identity, transport or calling implementation authorized by this note.

## Required experience

A person should retain one identity and contacts across desktop and phone daemons, and
people should address them by a readable name. WebRTC is a planned connectivity option
for responsive communication and future video conferencing. Video calling is lower
priority and does not block the current received-request UI.

The owner had supplied identity/WebRTC research earlier; this is recovered architectural
context, not a newly invented product requirement. The reviewer should have translated
those research implications into explicit boundaries before treating daemon peer IDs as
the permanent product identity. Current read-only work can finish; future addressing
contracts must account for this requirement before they are frozen.

## Separate four jobs

| Concept | Job | Required behavior |
| --- | --- | --- |
| Account identity | Identifies the same person or specialized service over time | Survives device replacement; contacts and authorization bind here in the future model |
| Readable handle | Lets people find and recognize an identity | Verified mapping to identity; rename/reassignment cannot silently change an existing contact or signed recipient |
| Device identity | Identifies an authorized desktop/phone daemon | Separate device keys and explicit enrollment/revocation; transport peer ID remains useful here |
| Connection address/transport | Reaches a currently available device | May change between TCP, QUIC, WebRTC or relay without changing the intended account |

Example: Alice's readable handle resolves to her persistent identity; that identity
can authorize her laptop and phone. Replacing the phone changes a device authorization
and route, not the contact Bob has saved. A bare profile display name is not a verified
handle. Exact handle syntax, resolver and account-ID format are not selected here.

Proposed device model: separate device keys authorized by a recoverable account identity.
Do not use copying one daemon's private identity key onto every running device as the
multi-device architecture. Key export/import can preserve a key-derived ID for migration,
but does not by itself provide device revocation, state synchronization or safe concurrent
ownership. Lost-key recovery requires a deliberately designed recovery authority; readable
names alone cannot make an identity loss-proof.

## Actual implementation and migration impact

Reviewed baselines: bb-go `cd497749a771b063300e2c3edf8748fe285b06c4`; bb-desktop
`7a31c41cb29692a94acf1f24adb379f3a829d237` plus the pinned PAY-001 working inputs.
No source changed for this assessment.

- `bb-go/modern/network/identity.go` persists a local identity.key, and network/node.go
  configures libp2p with it. This supports stable identity across daemon restarts; it is
  not an implemented account/device enrollment or history-sync system.
- direct/types.go keys messages/conversations by peerId. payment/types.go's closed v1
  request signs payer_peer_id/payee_peer_id; payment/signature.go and service.go bind
  the signer and authenticated remote peer to those fields. That is a real protocol
  constraint, not just a long string used by the UI.
- Desktop payment-inbox.js binds the daemon payment identity to the social peer and
  groups requests by payee peer ID. app.js keys current sessions/conversations using
  these IDs. Those checks remain required for the current protocol.

Therefore, portable multi-device accounts will affect identity verification, signed
message/request envelopes, routing, conversation/contact storage and synchronization,
and desktop/native recipient presentation. This warrants a versioned migration, not
substituting a handle into a peer_id field or silently equating unrelated peer IDs.
Preserve historical signed bytes and original verified peer bindings. A future verified
mapping may associate old history with an account; it must not rewrite the old signatures.
Define compatibility/capability negotiation and downgrade handling before implementation.

Continue the bounded read-only PAY-001 UI against its existing v1 contract. Before the
next request-creation/native-approval contract or mobile account onboarding, write the
account/device/recipient migration contract. This prevents expanding the current peer-ID
assumption into more irreversible signed formats. The current UI cards are reusable,
but their future data grouping/addressing adapter will need verified account identities.

## Transport and latency

The daemon currently listens on TCP and QUIC by default (`modern/network/node.go:31`).
Plan WebRTC through the existing transport boundary where applicable; it must preserve
application authentication, request IDs, replay checks and durable delivery semantics.
Do not make a WebRTC session or signaling server the account identity authority.

WebRTC is supported by libp2p alongside other transports; browser connectivity may use
relay-assisted signaling. This supports a transport extension rather than replacing
recipient identity. It does not prove a speedup for this application's current native
QUIC path. Benchmark connection setup, first-message delay, ongoing delivery and relayed
paths on desktop/mobile networks before claiming latency improvement.
[libp2p WebRTC](https://libp2p.io/docs/webrtc/)

The present payment UI reads a local authenticated snapshot on a five-second cadence;
chat also has live events plus reconciliation. A faster remote transport will not remove
that local polling delay. A future authenticated daemon event notification can trigger
prompt reads while keeping reconciliation for missed events. That is a separate local
API/update contract; no manual connection/refresh control is reintroduced.

## Video conferencing: planned, lower priority

Reuse persistent contacts, verified device selection and authenticated call signaling.
Separate call/session IDs and media negotiation from durable messages/payment records.
libp2p's WebRTC transport uses data channels; enabling it is not itself an audio/video
calling implementation. Media capture/tracks and call lifecycle need their own feature.
[libp2p browser connectivity](https://libp2p.io/docs/browser-connectivity/)

Defer codecs, one-to-one/group topology, relay/media-server choice and UI until the calling
ticket. That ticket must define ring/accept/hang-up, device selection, microphone/camera
consent and mobile suspension/reconnect behavior. Do not relax the desktop's current
permission-denial defaults for a future feature, or treat approving a call as payment
approval. Identity and shared signaling boundaries are worth planning now; a call engine
is not part of PAY-001.

## Decisions still needed before migration implementation

- Account authority/key rotation and recovery; enrollment/revocation of a lost phone;
  stale devices and conflicting authority updates.
- Handle namespace/resolution/verification and continuity when a name changes owner;
  no mandatory centralized search service or domain purchase inferred from the examples.
- Private history/key synchronization, device fan-out, offline queues, deduplication and
  consistent read/cancellation status. A second device must not cause duplicate payments.
- Mapping v1 peer identities to accounts, authority for signing new payment requests,
  and native approval of the immutable resolved recipient and destination. Social
  device enrollment must not automatically distribute wallet keys or spending authority.
- WebRTC capability discovery, signaling, NAT/relay fallback and privacy, mobile power/
  background behavior, and later media-session requirements.

These are reviewer design work before bounded implementation; no owner answer is needed
now to finish the current read-only UI. Exact standards/services remain unselected.

## Research basis and qualification

The workspace's Distributed Social Media Research.md discusses DIDs/readable names at
lines 19–49, WebRTC/signaling at 202–206 and portable identity at 208–225. Its marketplace
examples are context; they do not restore marketplace scope. Its broad latency and
loss-proof portability claims are not accepted as implementation guarantees.

Primary-source cross-check: AT Protocol distinguishes stable DIDs from mutable handles
and verifies their relationship in both directions. This is a useful separation to
adopt in our design, not a decision to adopt its entire network or DID registry.
[AT Protocol handles](https://atproto.com/specs/handle)

Nostr NIP-05 maps a readable identifier to a public key; this naming specification alone
is not a multi-device recovery protocol.
[NIP-05](https://github.com/nostr-protocol/nips/blob/master/05.md)

The portable identity/transport note is public-scope architecture. The broader workspace
PRODUCT_WORKSTREAM_CONTEXT.md remains local planning context and is not added to a
publication manifest. Governance-only edits; daemon/product sources remain frozen.
