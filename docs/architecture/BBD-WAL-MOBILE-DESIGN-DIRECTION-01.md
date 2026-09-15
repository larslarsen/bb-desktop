# Wallet design direction — mobile and desktop

Owner steering: consider mobile ZEC wallet designs because a mobile BitBook version
is likely later. MonteZecret remains one reference; code reuse is not the selection
criterion. This is reviewer design direction, not a mobile implementation or
framework migration ticket. WAL-019 acceptance remains active.

## Reference assessment

| Reference | Observed pattern | Proposed use in BitBook |
| --- | --- | --- |
| [Zodl, formerly Zashi](https://zodl.com/download/) | The official [onboarding screenshot](https://support.zodl.com/article/23-creating-your-zodl-wallet) shows a prominent balance, labeled action buttons, an actionable status banner and activity underneath. | Primary reference for a simple wallet home and clear next action. |
| [YWallet history](https://ywallet.app/history) | Documentation and screenshots show compact transaction rows and a wider sortable table. | Reference for adapting the same activity information to available space. |
| [MonteZecret](https://forum.zcashcommunity.com/t/montezecret-a-desktop-wallet-for-zcash-in-rust-instead-of-tweets/56164/3) | Persistent desktop task navigation and a QR/Copy Receive panel. | Secondary reference for larger-screen navigation and receive layout. |

These observations come from public product documentation/screenshots, not running
the wallets or validating their transaction behavior. Selection is a UX judgment.
No upstream code, artwork, swap features or branding is imported by this direction.

## Proposed common experience

- **Wallet:** account selector/name, asset/network context, known balance with
  honest freshness/availability, prominent Send, Receive and Request actions, a
  concise status row and recent activity. Missing balance data is not zero.
- **Receive:** supported private address, QR, Copy/Share and optional amount/request
  flow. A BitBook peer request retains its authenticated peer and expiry semantics;
  an address QR alone does not establish the sender's social identity.
- **Send:** choose recipient or open a verified request, enter amount, review full
  destination/network/amount/fee, then explicit trusted approval. Scanning is an
  input option when the platform supports it. Failed or unavailable capabilities
  remain visibly distinct from a completed payment.
- **Activity:** readable incoming/outgoing entries with detail on selection. Keep
  request delivery, user approval, transaction submission and chain confirmation
  distinct; a pending peer request is not a confirmed transaction.
- **Settings:** accounts, backup, locking, server and advanced details. Keep the
  selected connection and required disclosure visible before explicit sync starts.

Proposed primary destinations: Wallet, Activity, Settings. On a phone use bottom
navigation and one main column; on desktop use a side rail and an optional activity
detail pane. Keep action names and task sequence consistent. Desktop should use
its space without stretching a phone screenshot, and mobile actions must not depend
on hovering, right-clicking or keyboard shortcuts. Account/network identity remains
visible during Receive and Send review. Social profile/chat Pay or Request actions
should enter these same wallet flows with authenticated context.

## Implementation boundaries and next design deliverable

Share the behavior contract and state vocabulary first. Keep the Rust wallet core
and trust checks separate from presentation; reuse on mobile requires a dedicated
review of lifecycle, storage, locking and approval boundaries. Electron IPC is a
desktop transport, not the proposed mobile architecture. No choice of mobile GUI
framework or claim of shared UI code is made here.

The next wallet design ticket should produce paired phone/desktop layouts for
Wallet, Receive, Send review and Activity, with examples of locked, syncing, ready,
failed and pending states. Use the same representative data in both sizes so
differences are reviewable. Preserve existing account/backup/restore/receive work.
Build out implemented capabilities in bounded tickets; this design does not turn
unfinished payment or balance capabilities on. Profile pictures remain separate.
