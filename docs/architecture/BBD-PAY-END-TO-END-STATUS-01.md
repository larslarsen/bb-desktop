# Payment requests and payments — actual integration status

Reviewer: Codex. Owner-directed priority: requests and payments between BitBook users.
This supersedes the inventory-cleanup routing, not the recorded security findings.

Current reviewer status: WAL-019 COMPLETE; feature f2d9c9a and correction 7a31c41
are published, with final immutable-range scan accepted in
[WAL-019 final review](../testing/BBD-WAL-019-FINAL-REVIEW-01.md).
Daemon BBGO-PAY-003 is accepted and published: authenticated local read access to
stored signed records. See its
[final review](../../../bb-go/docs/testing/BBGO-PAY-003-FINAL-REVIEW-01.md).
Desktop [BBD-PAY-001](../../tickets/BBD-PAY-001.md) is COMPLETE and published.
Feature `f23950245c338d83072caa3d96503b69a21682e2`; actor closeout
`48c60b3f05591323b06f493df974819c50add76f`.
[Final reviewer acceptance](../testing/BBD-PAY-001-FINAL-REVIEW-01.md) verifies the
published content and records execution-evidence limitations. Received request cards
appear in peer Messages conversations, with automatic message/request updates. The
standalone Requests tab and payment connection/refresh controls were superseded.
Four inherited Rust inventory policy failures remain release blockers. No new source,
execution or live-wallet task is authorized. DEV-001 and older inventory-cleanup routing
remain inactive. Historical review requirements are resolved to the extent stated in
the final review; they are not current execution instructions.

The existing daemon binary was previously rebuilt and its identity retained in the
daemon final review. No application or daemon process restart is claimed here.

## Owner Monero availability update

Owner now reports monerod has synced. This supersedes the earlier planning assumption
that node synchronization is still in progress. It is an owner report, not a live RPC
verification of network/endpoint/height, wallet refresh or adapter readiness. Carry it
into the next WAL-007/XMR planning review. Existing XMR implementation defects and gates
remain; PAY-001 is complete and no live node/wallet operation is authorized.

## What works as components, and what is missing

| User-visible step | Current source evidence | Missing connection |
| --- | --- | --- |
| Request money from a peer | wallet-pay/model.js has payee gating; native account receiving exists | social/app.js has chat but no Pay/Request flow; runtime.rs:574 returns UNAVAILABLE for receiver.fresh |
| Deliver the signed request | BBGO-PAY-002 phase A now starts/closes payment.Service in the real daemon; request delivery, payer rejection and restart persistence are accepted | api/handler.go still has no payment routes; desktop cannot yet use this transport as a complete request flow |
| See received requests | BBGO-PAY-003 provides authenticated local GET /v1/payment/records; desktop authenticated bridge and request cards in Messages are accepted/published | Request creation and native approval are separate future steps |
| Open and approve the received request | preload/supervisor have begin/cancel boundaries; native signing components exist | runtime.rs:574 returns UNAVAILABLE for intent.begin and intent.cancel; no complete request-to-native-review path |
| Send funds | reviewed Zcash prepare/sign/verify and recovery components | chain submission and confirmation are later WAL-009 work, not implemented by the accepted non-broadcast slice |
| Show accurate status on both sides | signed status codec exists; transport permits cancellation | network paid receipts remain deliberately blocked; receipt/privacy semantics need a separate contract, and chain confirmation cannot be replaced by a peer claim |

BBGO-PAY-001 explicitly accepted a component without daemon startup, HTTP routes or
renderer integration. Therefore its acceptance did not establish a working product
payment-request flow. The prior conversation's description of transport being available
needed this qualification.

## Immediate outcome, not maintenance

Completed: BBGO-PAY-002 phase A proves that a running daemon accepts a valid signed,
payer-bound request over the actual libp2p stream, retains it across restart, and
rejects a request bound to another payer. Feature commit
`c00764d4a84eb0e149ec2779d1746248f86ba3a4` is published; CI run 34916979825 passed.
The daemon remains wallet-free: no keys, signing of coin
transactions, broker calls or chain submission move there.

Payment product sequence (later steps are planning only):

1. COMPLETE: BBD-PAY-001 integrates received requests into Messages through the
   authenticated read endpoint, with local identity binding, peer conversation cards,
   cancellation/expiry and automatic updates.
2. Separately freeze authenticated creation/cancellation access and connect native
   fresh receiving to signed request creation and a visible Request action.
   Preserve native account/network/privacy validation. Specialized
   nodes may be counterparties without requiring a human social profile.
3. Open received requests in native review with authenticated counterparty,
   immutable destination/amount/network and explicit approval. A renderer cannot
   supply replacement destination fields or authorize a spend.
4. Finish verified testnet submission/recovery and independently checked chain
   confirmation. Distinguish delivery, approval, submission and confirmation in
   the UI. Peer status alone never proves payment; paid receipts need their own
   privacy and verification contract.

Step 1 is accepted and published; its execution handoffs are closed. No later step is
authorized for implementation by this planning sequence.
Before steps 2–3 freeze new recipient/creation/approval contracts, settle the account/device
migration in [identity/transport direction](BB-IDENTITY-TRANSPORT-DIRECTION-01.md).
The owner requires persistent identity across desktop/phone, readable names and planned
WebRTC connectivity, with video calls later. The read-only v1 UI is complete; its current
peer-ID binding must not silently acquire account/handle semantics. Later steps are planning
dependencies. Do not introduce
an unauthenticated payment
API by inheriting the current wildcard-CORS social surface without a specific trust
contract. Do not enable paid receipts or expose on-chain links just because requests
now travel. No real funds, mainnet or public peer run is authorized by this status map.

The completed UI slice is BBD-WAL-019: show server/disclosure before a
single Sync action, keep separate Balance navigation, preserve cancellation and
locked-balance protections. Source inspection confirms the two actions are currently
navigation then start, not evidence that the sync engine needs two start calls.
Its real-pointer regressions, runtime behavior and publication are accepted.

Social UI recovery is separate: profile pictures first, followed by a comparison
of old frontend capabilities with current equivalents before more replacement work.
The larger native wallet redesign should organize balance, receive, send, activity
and sync around user tasks, building on existing components. Existing-wallet visual
reference research belongs to that design ticket; no external design comparison is
claimed for the original plan. Owner has now named MonteZecret as a UI reference;
see the reference note below. Advertising/auction/search design remains deferred. Preserve the
specialized-peer payment use case without building an advertising subsystem now.

## Owner-selected UI reference: MonteZecret

Subsequent owner steering broadens the reference set to mobile ZEC wallets for a
future mobile version. [Mobile/desktop design direction](BBD-WAL-MOBILE-DESIGN-DIRECTION-01.md)
now proposes Zodl as the primary simplicity reference, YWallet for activity layout
adaptation and MonteZecret as a secondary desktop reference. The observations below
remain useful; MonteZecret is no longer the sole design starting point.

Reviewed the developer's [project thread and June 24 Receive screenshot](https://forum.zcashcommunity.com/t/montezecret-a-desktop-wallet-for-zcash-in-rust-instead-of-tweets/56164/3).
It shows persistent left navigation (Home, Send, Receive, History, Accounts,
Contacts, Settings), visible sync status, and a Receive panel with QR/address/Copy.
The author describes a custom Rust/SDL2 UI. These are useful layout and navigation
references for a BitBook implementation in the existing egui native wallet.

Design direction: persistent task navigation, visible account/network/sync context,
and a focused Receive screen. Retain BitBook's explicit approval and privacy gates;
do not expose unsupported address modes merely because the reference shows them.
Actual Send/history states must reflect implemented capabilities and confirmed data.

Search and the author's public GitHub repository listing did not locate the wallet
source or its license. No source/assets were copied, no code-reuse permission is
claimed, and no GUI-framework migration is proposed. Resolve source/license before
any direct code or asset reuse. The next wallet design ticket should translate the
observed structure into reviewable BitBook screens. Current WAL-019 Hermes acceptance
and the separate profile-picture recovery work retain their scope.

The desktop inventory task is cancelled as the active task. Four remaining policy failures and
global lint debt remain recorded release blockers, but are not the next implementation
priority. No more Rust proving-suite reruns for this network integration slice.

## Roles and authorized documents

Codex remains reviewer. Owner manually relays actors; none was launched. WAL-019
runtime, falsification, 22-test account UI target and publication are accepted; its
local wallet resource and manifest are verified. No WAL-019 handoff remains active.
The daemon binary has been refreshed directly; neither app was restarted.
BBGO-DEV-001 is cancelled. BBGO-PAY-003's read endpoint and lifecycle integration
are accepted. BBD-PAY-001 is the active desktop source task. Routine affected local
artifact refresh is included in completion.
Existing payment integration gaps and release blockers remain open.

Reviewer governance paths for this authorization: this status map, desktop AGENTS.md,
docs/engineering/DEVELOPMENT_ROLES.md, docs/handoff/CURRENT_TASK.md,
tickets/BBD-PAY-001.md and docs/handoff/GROK_BUILD_BBD_PAY_001_INBOX_01.md.
Repository documents are the owner's existing control plane for decisions,
authorizations and evidence; chat only relays pointers.
No product source edit, publication or acceptance execution is claimed.
