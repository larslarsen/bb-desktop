# Payment requests and payments — actual integration status

Reviewer: Codex. Owner-directed priority: requests and payments between BitBook users.
This supersedes the inventory-cleanup routing, not the recorded security findings.

Current planning baseline: bb-desktop HEAD 2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e
plus preserved unpublished work; bb-go HEAD
2b695a8719a7381f3919bb83c63e54251f43536d. The daemon has an existing
untracked modern/bitbookd binary; it was not run, replaced or used as evidence.

## What works as components, and what is missing

| User-visible step | Current source evidence | Missing connection |
| --- | --- | --- |
| Request money from a peer | wallet-pay/model.js has payee gating; native account receiving exists | social/app.js has chat but no Pay/Request flow; runtime.rs:574 returns UNAVAILABLE for receiver.fresh |
| Deliver the signed request | BBGO-PAY-002 phase A now starts/closes payment.Service in the real daemon; request delivery, payer rejection and restart persistence are accepted | api/handler.go still has no payment routes; desktop cannot yet use this transport as a complete request flow |
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

Next payment product sequence:

1. Freeze authenticated local-client access for request creation, listing and
   cancellation before implementing payment HTTP routes. Bind the desktop main
   process to the intended daemon instance; never put authority in renderer input.
2. Connect native fresh receiving to signed request creation and a visible Request
   action/inbox. Preserve native account/network/privacy validation. Specialized
   nodes may be counterparties without requiring a human social profile.
3. Open received requests in native review with authenticated counterparty,
   immutable destination/amount/network and explicit approval. A renderer cannot
   supply replacement destination fields or authorize a spend.
4. Finish verified testnet submission/recovery and independently checked chain
   confirmation. Distinguish delivery, approval, submission and confirmation in
   the UI. Peer status alone never proves payment; paid receipts need their own
   privacy and verification contract.

These are planning dependencies, not implementation authorizations. Do not introduce
an unauthenticated payment
API by inheriting the current wildcard-CORS social surface without a specific trust
contract. Do not enable paid receipts or expose on-chain links just because requests
now travel. No real funds, mainnet or public peer run is authorized by this status map.

The immediate authorized UI slice is BBD-WAL-019: show server/disclosure before a
single Sync action, keep separate Balance navigation, preserve cancellation and
locked-balance protections. Source inspection confirms the two actions are currently
navigation then start, not evidence that the sync engine needs two start calls.
Grok first authors real-pointer regression tests; production follows reviewed red.

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

The desktop inventory task is cancelled as the active task. Six policy failures and
global lint debt remain recorded release blockers, but are not the next implementation
priority. No more Rust proving-suite reruns for this network integration slice.

## Roles and authorized documents

Codex remains reviewer. Owner manually relays actors; none was launched. Active
publication handoff: docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md.
WAL-019 runtime, falsification and the 22-test account UI target are accepted; the
local wallet resource and manifest have been rebuilt and verified. Hermes may
correct the specified records and publish the exact scoped path set after scanning.
All source/artifacts stay frozen. Publication verification remains pending. The
daemon has not been refreshed and neither app restarted; daemon build automation
is queued. Existing payment integration gaps and release blockers remain open.

Reviewer governance paths for this planning turn: this status map, desktop
CURRENT_TASK.md, tickets/BBD-WAL-019.md, its Grok test handoff and bb-go
CURRENT_TASK.md. No publication or acceptance execution is claimed.
