# BBD-WAL-001 — Dual-Coin Wallet Architecture Review

Status: CORRECTION 02 — SOURCE ONLY — awaiting Lead Engineer/Reviewer — Codex
acceptance

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Ticket: `tickets/BBD-WAL-001.md`

Governance baseline recorded by the ticket: `20c7f7e7e71a5d98c1e236fea9d7d3dc1eeffb8a`

Accepted security implementation (BBD-SEC-001): `47bf45884d737b4b89571f06d8ba3b4e20238bfb`

This document is the architecture gate. It does not implement a wallet, add a
dependency, change Electron runtime behavior, or authorize mainnet, device, or
node use. Implementation actors must not invent process topology, spend
authority, IPC methods, asset semantics, or hardware claims beyond this review
and later reviewer-bounded tickets.

This Correction 02 revises the Correction 01 source in place. Correction 01
resolved the seven recorded blockers and remains the dual-coin broker base.
This revision adds the omitted exchange-rate architecture and fixes two
canonical-validation imprecisions: bidi/format controls are rejected by
explicit codepoint, and `TimestampV1` requires strict calendar parsing with
round-trip equality. Rate fetching is not a `bb-go`, renderer, or broker
spend-core concern. Exact atomic ZEC/XMR amounts remain authoritative.

## 1. Context and fixed owner decisions

BitBook is a distributed social network. Wallet use is optional. Payments are
still a first-class native social feature: ordinary peer payments must not
require address copying or QR codes. Marketplace, listing, order, escrow,
dispute, exchange, custodial, and centralized payment-provider behavior remain
out of scope.

Fixed owner decisions that this review does not reopen:

- Both ZEC and XMR ship as product assets from the beginning. The common
  contract is dual-coin even if adapters reach usable depth sequentially.
- ZEC is the built-in basic wallet and must use the maintained shielded
  protocol. The inherited OpenBazaar transparent ZEC/BTC/BCH/ETH wallet is not
  an implementation base.
- XMR is optional and connects only to a user-controlled local `monerod`. The
  Monero node and the Monero wallet process are separate.
- Software, hardware-backed, and watch-only accounts are first-class. Ledger
  and Trezor support are requirements; they are expressed as capabilities, not
  vendor slogans. The owner prefers hardware signing.
- The maintained `../bb-go/modern` social daemon stays wallet-free. Spend
  authority, viewing keys, seeds, and coin clients do not move into `bb-go`.
- `../go-ipfs` is deprecated and out of scope.
- Chromium's sandbox stays enabled in packaged apps. Device access must not be
  used as a reason to disable it.
- The broker authorization surface is a minimal native window running inside
  the Rust wallet-broker process. It owns software-wallet passphrase entry,
  backup/restore, and the authoritative payment confirmation. Electron never
  owns or embeds that window. Hardware devices still perform their own
  independent confirmation when their capabilities allow it. A broker-started
  native OS file picker may select backup paths; a separate OS credential agent
  is not the v1 unlock design.

Current desktop facts this design extends rather than replaces:

- Product entry is `social-main.js` loading only `social/index.html`.
- Electron 44.0.0, `app.enableSandbox()`, `nodeIntegration: false`,
  `contextIsolation: true`, `sandbox: true`, denied navigation, denied new
  windows, denied permissions.
- BBD-SEC-001 currently proves there is **no preload and no IPC bridge**. That
  is a correct present-tense hardening of a wallet-free client. It is not a
  perpetual ban on a future allowlisted preload. Wallet work must replace
  "no IPC" with a fail-closed allowlist and must not weaken sandbox, context
  isolation, navigation, or permission denial.
- The renderer talks to the social daemon over user-selected HTTP/WebSocket
  (`default http://127.0.0.1:4002`). That channel is social only. It must never
  become a wallet API, a generic wallet HTTP API, or a relay to
  `monero-wallet-rpc`.
- README still describes the client as wallet-free. That remains true until a
  later accepted implementation ticket lands.

Inherited OpenBazaar exchange-rate path, **rejected as a production
foundation** (see §12 and §1.1):

- Desktop `js/utils/currency.js` polls the local daemon at
  `/ob/exchangerates/<coin>`. `js/utils/exchangeRateSyncer.js` repeats that
  poll every five minutes.
- Daemon `api/jsonapi.go` delegates that route to the legacy wallet
  `ExchangeRates()` implementation.
- Vendored ZEC code tries `https://ticker.openbazaar.org/api` first, then
  old Bittrex, Bitfinex, Poloniex, and Kraken endpoints. That entire chain
  is deprecated.
- OB1 published `OpenBazaar/tickerproxy` under MIT as a BitcoinAverage
  gatherer with local or S3 cache. Latest listed release is 2018. Use only
  as historical design evidence, not code or infrastructure.

Replacement rates are optional, untrusted presentation data. They must not
revive the daemon wallet endpoint, `ticker.openbazaar.org`, old exchange
fallbacks, a mandatory BitBook-operated proxy, or Pay that depends on fiat.

Compatibility risks treated as first-class, not footnotes:

- Zcash NU6.3 / Ironwood is **activated** on mainnet, not a future NU7-class
  guess. Light servers, PCZT versions, hardware apps, and viewing-key encodings
  can lag the chain. Generic "ZEC support" is not shielded signing. Ironwood is
  a distinct pool that reuses the Orchard protocol; it is not a new
  address/receiver type.
- Hardware firmware and vendor apps drift independently of librustzcash and
  `monero-wallet-rpc`. A device that signed Sapling in 2024 does not thereby
  sign Orchard-pool or Ironwood-pool spends, or v6 transactions, in 2026. A
  Trezor transparent ZEC account is not a private account.
- Monero node RPC, Monero wallet RPC, wallet refresh, and device presence are
  different liveness states. Treating `monerod` as "the wallet" is a design
  error. Treating `monero-wallet-rpc --restricted-rpc` as a spending API is a
  design error.
- Inherited daemon `/ob/exchangerates/<coin>` plus `ticker.openbazaar.org`
  and 2018-era tickerproxy are not a current rate path. Fiat quotes are
  optional presentation. Exact `amount_atomic` remains the payment
  authority. Unavailability of every quote source must not block wallet
  setup, balance, receive, Pay, signing, or broadcast.

### 1.1 Authoritative protocol snapshot (2026-08-30)

Reviewer-supplied facts. This review does not use network access. Exact crate
and binary pins remain later implementation decisions.

| Fact | Source |
| --- | --- |
| Zcash NU6.3 / Ironwood activated on **mainnet on 2026-07-28 at block 3,428,143**. The old Orchard pool is restricted. Funds exit through the turnstile into Ironwood. | <https://z.cash/upgrade/nu6-3/> |
| NU6.3 adds **transaction version 6** and a separate **Ironwood bundle/pool**. Ironwood reuses the Orchard protocol. It is **not** a new address or receiver type. | <https://zips.z.cash/zip-0229>, <https://zips.z.cash/zip-0258> |
| The Orchard-protocol receiver and incoming viewing key are scoped to the **protocol, not a pool**, and scan both Orchard and Ironwood under the NU6.3 wallet rules. | <https://zips.z.cash/zip-0326> |
| Orchard-to-Ironwood migration requires privacy-aware wallet behavior and **user consent** for residual cross-pool amount disclosure. | <https://zips.z.cash/zip-0318> |
| Current librustzcash wallet APIs construct **v6 PCZTs and Ironwood bundles** for post-NU6.3 payments. Exact release pins are not chosen here. | <https://github.com/zcash/librustzcash/blob/main/zcash_client_backend/CHANGELOG.md> |
| Official Monero documentation defines `monero-wallet-rpc --restricted-rpc` as restricted to **view-only** commands. A spending adapter therefore needs authenticated **full** wallet RPC on loopback, contained behind the broker. | <https://docs.getmonero.org/interacting/monero-wallet-rpc-reference/> |
| Inherited desktop `js/utils/currency.js` polls local daemon `/ob/exchangerates/<coin>`; `js/utils/exchangeRateSyncer.js` repeats every five minutes. | inherited `bb-desktop` source (historical; not a product API) |
| Inherited daemon `api/jsonapi.go` delegates that route to the legacy wallet `ExchangeRates()` implementation. | inherited `bb-go` source (historical; not a product API) |
| Vendored ZEC exchange-rate code tries `https://ticker.openbazaar.org/api` first, then old Bittrex, Bitfinex, Poloniex, and Kraken endpoints. **This entire chain is deprecated.** | inherited vendored ZEC wallet (rejected) |
| OB1 published <https://github.com/OpenBazaar/tickerproxy> under MIT. It gathered BitcoinAverage data and wrote cache output locally or to S3. Latest listed release is 2018. Historical design evidence only; not code or infrastructure. | <https://github.com/OpenBazaar/tickerproxy> |
| CoinPaprika documents a mostly keyless free API, current ticker data, and a 20,000 request/month free limit. A later provider ticket may evaluate it; this review does not pin or promise it. | <https://docs.coinpaprika.com/api-reference/rest-api/introduction> |
| CoinGecko documents `/simple/price`, stable asset IDs, timestamps, and a keyless public endpoint intended for low-volume/testing rather than production reliability. A later provider ticket may evaluate it; this review does not pin or promise it. | <https://docs.coingecko.com/reference/simple-price>, <https://docs.coingecko.com/docs/keyless-public-api> |
| Kraken documents public `Assets`, `AssetPairs`, and `Ticker` discovery; pair availability must be probed rather than assumed. Direct current Kraken market data is distinct from the inherited obsolete Kraken fallback URL. A later provider ticket may evaluate it; this review does not pin or promise a ZEC or XMR pair. | <https://support.kraken.com/articles/360000920306-api-symbols-and-tickers> |

Product consequences, normative in this review:

- New ZEC social Pay uses a Unified Address containing an **Orchard-protocol**
  shielded receiver (no invented "Ironwood receiver set"). The same receiver
  and incoming viewing key scan Orchard and Ironwood notes.
- New post-NU6.3 spends construct **v6** transactions with an **Ironwood**
  bundle when the account can spend Ironwood. Capabilities record `can_tx_v6`,
  `can_receive_orchard`, `can_receive_ironwood`, `can_sign_orchard`,
  `can_sign_ironwood`, `pczt_version`, and
  `can_migrate_orchard_to_ironwood` separately.
- Restored or scanned notes still in the restricted Orchard pool **fail
  closed** for spend and for native Pay. A later consented migration ticket
  may use ZIP-318. v1 must not auto-migrate and must not label unmigrated
  Orchard-pool value as spendable private Pay funds.
- XMR spending uses authenticated full `monero-wallet-rpc` (no
  `--restricted-rpc` on that process). Restricted **node** RPC on local
  `monerod` is a different interface and may be used where node data suffice.
- Exchange rates are optional untrusted presentation. `amount_atomic`,
  asset, network, receiver, and the prepared fee are authoritative. Do not
  fetch rates in `bb-go`, the social renderer, or the wallet broker's
  trusted spend core. A separate least-privileged desktop quote worker
  (§12) emits `RateSnapshotV1`. Wallet setup, balance, receive, Pay,
  signing, and broadcast continue if every rate source is missing, stale,
  disagreeing, rate-limited, malformed, or offline.
- v1 signed payment requests are **coin-denominated only**. Optional
  approximate local fiat display must never change a prepared atomic
  amount or dynamically reprice a signed request.

## 2. Trust boundaries and data flow

### 2.1 Process map

```text
                          untrusted social peers
                                   |
                                   v
                     +---------------------------+
                     | bb-go/modern (wallet-free)|
                     | identity, posts, chat,    |
                     | payment-request transport |
                     | NO rate fetch             |
                     +-------------+-------------+
                                   ^
                      HTTP/WS social API only
                                   |
+------------------------------------------------------------------------------+
| BitBook desktop (this repository)                                            |
|                                                                              |
|  +----------------------+     allowlisted IPC      +----------------------+  |
|  | sandboxed renderer   | <----------------------->| Electron main        |  |
|  | social UI, Pay CTA   |   frozen preload methods | supervisor           |  |
|  | sanitized preview    |   begin/cancel/snapshot  | spawn/kill broker    |  |
|  | optional fiat approx |   optional RateSnapshot  | spawn/kill quote     |  |
|  | no keys, no USB,     |   NO confirm/unlock      | worker (no wallet    |  |
|  | no generic wallet    |   NO backup/broadcast    |  handles or secrets) |  |
|  | NO rate HTTP fetch   |   NO spend/rate mix      | cancel, preview      |  |
|  +----------------------+                          | no tx construction   |  |
|                                                    | no secret entry      |  |
|                                                    +----+-----------+-----+  |
|                                                         |           |        |
|                         inherited anonymous broker pipes|           | inherited
|                         (no UDS/named-pipe/TCP/HTTP     |           | anonymous
|                          listener; no quote-worker      |           | quote pipes
|                          handle shared with broker)     v           v        |
|                                              +----------+---+ +----+-------+ |
|                                              | wallet broker| | quote      | |
|                                              | (Rust)       | | worker     | |
|                                              | accounts,    | | least      | |
|                                              | intents,     | | privilege  | |
|                                              | adapters,    | | no wallet  | |
|                                              | signers,     | | IPC/files/ | |
|                                              | native auth  | | devices/   | |
|                                              | NO rate fetch| | identity/  | |
|                                              | spend core   | | amounts/   | |
|                                              +--+----+---+--+ | request IDs| |
|                                                 |    |   |    | RateSnapV1 | |
|                                        ZEC lib  | XMR| HID    +-----+------+ |
|                                        embed    | ctl| mgmt         |        |
+-------------------------------------------------|----|------|--------|-------+
                                                  |    |      |        |
                                                  v    v      v        v
                                            compact-  loopback hardware pinned
                                            block     monerod  devices  quote
                                            light     node RPC (untrusted HTTPS
                                            service   + full    replies; providers
                                            (untrusted) wallet- confirm (untrusted
                                                        rpc     on device) unit
                                                        (local,            prices;
                                                        contained)         no amounts)
```

### 2.2 Trust zones

| Zone | Code | Trust | May hold | Must not |
| --- | --- | --- | --- | --- |
| Social renderer | `social/` | Untrusted (XSS, hostile post/chat/profile content, compromised peer JSON) | Display of non-secret wallet status, sanitized non-authoritative Pay previews, and optional approximate fiat already returned by allowlisted IPC | Seeds, spend keys, view keys, passphrases, backup bytes, PCZT with signing material, raw signer material, USB, generic RPC, `ipcRenderer`, confirm/unlock/broadcast authority, outbound rate HTTP, treating fiat as `amount_atomic` |
| Electron main supervisor | `social-main.js` plus small new supervisor modules | Privileged for windowing and child spawn; **not** a wallet; **not** an authorization surface; **not** a rate oracle | Child stdio handles (broker pipes and **separate** quote-worker pipes), packaged-binary pins, allowlist dispatch, cancel, forwarding of sanitized `RateSnapshotV1` as display-only | Confirm, unlock credentials, backup bytes, transaction construction, key decrypt, coin libraries, forwarding of unknown methods, broadcast, mixing broker and quote handles, sending wallet context to the quote worker |
| Broker native authorization surface | owner-selected minimal window in-process in the wallet broker; broker may invoke an OS file picker | Authoritative for software confirm, onboard, unlock, and backup; still below a hardware device that independently displays a field | User-visible prepared review image; passphrase entry into broker memory; optional **explicitly approximate** sanitized `RateSnapshotV1` overlay that cannot change `amount_atomic` or fee | Social HTML, renderer origin, daemon `connect-src`, remote content, Electron-delivered secrets, rate fetch, using quotes in prepare/sign/broadcast |
| Wallet broker | new native sidecar in this repo | Spend/view authority for software accounts; coordinator for hardware/watch-only; owner of authorization | Encrypted software secrets, viewing material, durable intents, adapter state, prepared artifacts | Listening TCP/UDS/named-pipe/HTTP wallet API; logs of secrets; auto-broadcast after crash; social identity keys; product/generic wallet HTTP; **rate fetch in the trusted spend core**; quote-worker handles; treating fiat as spend authority |
| Quote worker | new least-privileged desktop child in this repo | Untrusted for payment value; trusted only to not hold wallet or social secrets | Fixed asset-ID / quote-currency queries; bounded provider HTTP; normalized `RateSnapshotV1` | Wallet IPC handle, wallet files, device access, social identity, broker session, account IDs, addresses, balances, peer IDs, memos, payment amounts, request IDs, API keys, telemetry, OB1 endpoints |
| ZEC compact-block service | user-configured light server | Untrusted chain service | Compact blocks | Viewing keys, spend authority, payment-request contents |
| User `monerod` | user-controlled local node | User-trusted for chain data, not for keys | Chain and peer traffic; optional restricted **node** RPC | Wallet keys; must not be treated as signer or as wallet RPC |
| `monero-wallet-rpc` (full) | local child contained by the broker | Holds XMR keys; broker authenticates over loopback; **not** `--restricted-rpc` when spend is required | XMR wallet files | Public bind, unauthenticated RPC, renderer access, product HTTP, crossing the broker boundary |
| Hardware device | Ledger / Trezor / Keystone | Protects spend keys it holds; independently protects only fields it displays | Device-resident seed; on-device confirm of displayed fields | Host honesty about fields the device cannot show; host parsing, fees, broadcast |
| Social daemon | `../bb-go/modern` | Social identity and transport | libp2p identity, signed payment-request envelopes | Coin keys, adapters, broadcast, fresh-receiver derivation, exchange-rate fetch, `/ob/exchangerates` |
| Pinned quote providers | user-configured, source-pinned HTTPS endpoints | Untrusted presentation oracles | Unit prices for allowlisted asset IDs and quote currencies | Payment amounts, account context, authority to change fees or Pay eligibility |

Residual trust, stated honestly:

- A **compromised OS or compromised broker** is outside this product's
  protection. Inherited anonymous pipes, loopback RPC, and file modes do not
  stop same-user malware, ptrace, or a hostile kernel.
- A **hardware device that cannot display** amount, destination, network, or
  fee leaves those fields **inside** the host/broker trust boundary. The
  account is still hardware-backed for key custody and is host-trusting for
  undisplayed fields. UI must say so.
- Authenticated loopback XMR wallet RPC stops other users and remote callers.
  It does not stop same-user local malware that can read broker memory,
  stolen loopback credentials, or the wallet process.
- Quote providers are **untrusted**. A lying, stale, or missing price cannot
  change `amount_atomic`, fee, receiver, network, or Pay eligibility. A
  quote worker compromise must not yield wallet secrets because the worker
  is specified to hold none.

### 2.3 Data flow for a native Pay

1. Payee desktop asks the broker for a **fresh receiver** bound to
   `(account_id, asset, network, request_id)`. For ZEC this is a UA with an
   Orchard-protocol shielded receiver; for XMR a fresh subaddress. Not a
   profile field.
2. Desktop asks the wallet-free daemon to assemble and **social-sign** a
   `PaymentRequestV1` using the canonical encoding in §11.2. The daemon never
   talks to the broker.
3. Daemon transports the signed request to the payer peer over the social
   channel (later `bb-go` ticket).
4. Payer renderer shows a Pay affordance. It may only call
   `wallet.beginIntent` (submit the signed social request for review) or
   `wallet.cancelIntent`. It cannot confirm, unlock, export, sign, or
   broadcast. Any on-screen preview is sanitized and non-authoritative.
5. Supervisor forwards `intent.begin` over inherited child IPC. The broker
   validates the request, **prepares** the spend, and produces an exact
   `ReviewImageV1` that includes `fee_atomic` and `fee_bound_atomic` **before**
   any user confirmation.
6. The **broker-owned** native surface shows that prepared image. Hardware
   confirmation, if any, occurs on the device. Electron does not receive
   `intent.confirm` and does not collect a passphrase.
7. **One** explicit user confirmation on the broker surface authorizes the
   normal `sign → verify → broadcast` sequence. It does not authorize a later
   mutated image.
8. After signing, the broker **re-checks cancellation and expiry**, then
   verifies the signed artifact against the confirmed `intent_hash`. Failure
   never broadcasts.
9. Immediately before broadcast, the broker **again** checks cancellation and
   expiry, then broadcasts only if still `verified`.
10. Status events return to the renderer as non-secret snapshots. Optional
    social receipts are daemon objects, not wallet proofs of spend-key
    possession.
11. Independently, the quote worker may supply a bounded `RateSnapshotV1`
    to desktop presentation. The renderer and the broker confirm surface
    may show it as **explicitly approximate** fiat. Prepare, fee, confirm
    eligibility, `intent_hash`, sign, and broadcast **ignore** it. Missing,
    stale, disagreeing, rate-limited, malformed, or offline quotes never
    block this flow.

Crash: a signed artifact recovered after broker or app crash enters
`crash_recovery`. It requires a **fresh** broker-owned confirmation and
revalidation. It must never auto-broadcast.

### 2.4 What a hardware wallet protects, and what it does not

A hardware signer protects the seed and the signature operation for keys it
holds. It independently protects a field only when `verified_fields` includes
that field **and** the device actually displayed it.

It does not protect:

- payment-request parsing and peer-identity binding
- choice of asset/network
- fresh-receiver substitution after display
- fee calculation and change destination, unless the device shows them
- compact-block or `monerod` honesty (withholding, stale tips, eclipse)
- broadcast, replace-by-fee, or submission to the wrong network
- malicious dependency updates of the host
- renderer phishing that tricks the user before the broker or device screen
- Orchard-to-Ironwood migration disclosure, unless a later ticket puts that
  consent on a device-verified field

If the selected device cannot independently display amount, destination, and
network, the account remains hardware-backed for key custody but **host-trusting
for those fields**. The UI must say so. It must not claim "the device verified
this payment."

## 3. Threat table

Each row names the attacker, the invariant, and the control that later tickets
must test. Spend authority never enters Electron, the renderer, `bb-go`,
product/generic HTTP, logs, or evidence. Authenticated loopback XMR wallet RPC
is an internal adapter exception and must not cross the wallet-broker boundary.

| ID | Threat | Invariant | Control | Test layer |
| --- | --- | --- | --- | --- |
| T01 | Compromised renderer | Renderer cannot spend, export seeds, confirm, unlock, or send generic broker commands | Frozen preload allowlist without confirm/unlock/backup/broadcast; broker ignores confirm from supervisor/Electron; only broker native surface confirms | Electron boundary + broker method allowlist |
| T02 | Malicious social peer | Hostile profile/post/chat cannot inject HTML or wallet commands | Existing DOM text-node rendering and CSP; payment fields parsed as data, never as script; no `innerHTML` of memos | Renderer fixtures + memo oracle |
| T03 | Malicious payment request | Payer cannot be bound to wrong asset, network, amount, receiver, or peer | Social signature over JCS canonical request (§11.2); broker binds intent to `request_id` + peer IDs + atomic amount + network; reject unsigned, non-canonical, expired, cancelled, replayed, unknown-field, or mutated requests | Canonical golden vectors + property tests |
| T04 | Hostile or stale chain service | Light server / `monerod` cannot silently change destination or create spendable funds from withheld data | Local trial decryption; re-verify consensus branch; fee/expiry quotes die; never treat "server said confirmed" as device-verified | Fake compact-block / fake RPC failure injection |
| T05 | Compromised remote site/content | No remote UI, no WebUSB, no remote script | BBD-SEC-001 navigation/permission denial retained; broker native surface has no remote content; no `shell.openExternal` from renderer URLs | Existing Electron security tests extended |
| T06 | Forged device response | Host must not accept a signature over a different PCZT/intent than shown | Revalidate signed artifact against confirmed `intent_hash`; device fingerprint binding; reject empty/truncated/wrong-branch signatures | Fake signer mutation |
| T07 | Device disconnect mid-sign | Disconnect aborts the intent; it does not retry with software keys and does not broadcast a partial | State `failed` with `DEVICE_DISCONNECTED`; no signer fallback; durable intent never `broadcasting` | Failure injection |
| T08 | Wallet-broker crash | Crash does not auto-broadcast, does not leak keys into crash dumps, and does not unlock on restart | Unsigned → abort; signed-unverified/verified → `crash_recovery` requiring fresh broker confirm + revalidate; unlock required; protocol pipe ≠ diagnostics | Crash/restart tests |
| T09 | Rollback or corrupt state | Restoring older broker state cannot resurrect spent notes/key images without warning and lock | Monotonic epoch; refuse stale epoch; restore is a user-confirmed import on the broker surface; ZEC nullifier / XMR key-image conflict fails closed | Corrupt-fixture + rollback tests |
| T10 | Concurrent spends | Two prepares cannot double-spend the same notes/key images | Per-account exclusive prepare lock; adapter-level reservation; second intent gets `ACCOUNT_BUSY` | Concurrency tests |
| T11 | Replay | Same payment request cannot be paid twice; captured broker frames cannot be replayed across sessions | `request_id` + `nonce` uniqueness; paid/cancelled terminal states; session transcript bound to parent/child identities and nonces; IPC replay window rejected | Property tests |
| T12 | Address substitution | Receiver shown to the user is the receiver in the signed artifact | `intent_hash` covers receiver bytes; post-sign byte-equality check; hardware destination mismatch fails closed | Independent oracle |
| T13 | Decimal / atomic-unit confusion | Protocol amounts are integer atomic strings; UI conversion cannot change value | `amount_atomic` regex `^[0-9]+$`; per-asset exponent table (ZEC 8, XMR 12); reject floats, scientific notation, and locale commas in protocol; rate display uses `DecimalString` and never IEEE-754 | Table-driven unit tests |
| T14 | Wrong network | Testnet/stagenet funds and mainnet funds cannot mix; social request network must match account network | `network` in every account, request, intent, and sign payload; branch-id / genesis binding; native confirm labels network | Boundary tests |
| T15 | Malicious dependency or update | New coin/hardware crates cannot silently enter the packaged graph | Pin versions; SBOM on packaged broker + Electron app; npm audit ratchet; no wallet HTTP client libraries in renderer | SBOM + lockfile + policy tests |
| T16 | Local unprivileged other-user process | Another user must not speak the broker protocol | No listening endpoint; inherited anonymous pipes/handles only; packaged-binary verify before spawn; session transcript binding | Spawn/handle tests (no real FS listener) |
| T17 | Logs and crash dumps | Seeds, spend keys, view keys, PINs, passphrases, backup bytes, and raw signing blobs never appear | Structured log field allowlist; secret canaries; redact device serial in ordinary logs; diagnostics pipe is not the protocol pipe | Canary tests |
| T18 | Clipboard | Ordinary Pay never copies or trusts clipboard addresses | No clipboard write in Pay success; no clipboard read as payment authority | UX/unit prohibition tests |
| T19 | Accidental transparent ZEC downgrade | A shielded/private account cannot receive or send transparent-only outputs as its social Pay path | Default receive is a UA with Orchard-protocol shielded receiver, no transparent receiver; Pay requires `can_receive_private`; transparent-only hardware is not a private account; `TRANSPARENT_DOWNGRADE` is a hard error | Capability + adapter tests |
| T20 | Electron-confirmed or Electron-unlocked software spend | Compromised social page or main-owned HTML cannot click-confirm a hot spend or collect a passphrase | `intent.confirm`, `account.unlock`, backup export, and broadcast are absent from every Electron preload and `ipcMain` allowlist | Preload surface tests + broker allowlist |
| T21 | Social-identity key reuse as wallet seed | libp2p/social keys never derive ZEC/XMR | Separate KDFs and storage roots; tests reject identical seed bytes across domains | Unit prohibition |
| T22 | XMR remote-node coercion | Optional XMR path cannot "helpfully" fall back to a public node | Only loopback `monerod`; configuration rejecting non-loopback; no remote-node UI | Adapter config tests |
| T23 | NU6.3 / firmware skew | Pre-v6 or Ironwood-incapable libraries or devices must not appear ready for current mainnet shielded spend | Separate v6 / Orchard / Ironwood / PCZT / migration caps; `PROTOCOL_INCOMPATIBLE` blocks prepare/receive-as-spendable | Capability probe tests |
| T24 | Watch-only presented as hot | Watch-only cannot sign | `can_sign_spend=false` is enforced in state machine, not just UI hiding | Signer matrix |
| T25 | Payment memo leakage | Memos do not go to logs, crash reports, light-server extra APIs, or evidence files | Log `request_id` only; evidence fixtures use synthetic memos and assert absence in logs | Canary tests |
| T26 | Same-user local malware | Architecture does not claim OS compromise is solved | No false "0700 socket protects you" claim; residual documented; still no listening API that widens the surface | Documentation + no-listener tests |
| T27 | Post-sign cancel or expiry | A signed artifact must not broadcast if cancelled or expired after sign | Recheck cancel/expiry after sign and immediately before broadcast | State-machine tests |
| T28 | Restricted wallet RPC used to spend | XMR spend cannot be configured with `monero-wallet-rpc --restricted-rpc` | Adapter spawn args omit `--restricted-rpc`; spend methods fail closed if the process is view-only; node `--restricted-rpc` is a separate bind | Adapter config tests |
| T29 | Product/generic wallet HTTP | Renderer, main, and daemon never speak wallet HTTP | Forbidden product HTTP; contained authenticated loopback XMR wallet RPC stays behind the broker and is not a public or Electron API | Policy + connect-src tests |
| T30 | Invented Ironwood receiver or silent Orchard migration | No new address kind; restored Orchard-pool funds do not auto-spend or auto-migrate | Receiver kind remains Orchard-protocol UA / XMR subaddress; `can_migrate_orchard_to_ironwood` false in v1; unmigrated notes fail closed | Capability + restore fixtures |
| T31 | Confirm without prepared fee | User cannot confirm a spend whose fee is unknown | `awaiting_confirm` requires a `prepared` `ReviewImageV1` with `fee_atomic` and `fee_bound_atomic` | State-machine tests |
| T32 | Secret in argv or environment | Launch token, RPC password, or seed never appears in process argv/env | Handshake nonces only on the inherited protocol pipe; RPC credentials only on broker-internal loopback config not exported to Electron | Spawn-env tests |
| T33 | Hostile, stale, or missing rate provider | Quotes cannot change `amount_atomic`, fee, receiver, network, or Pay eligibility; missing quotes cannot block coin payment | Quote worker isolated from spend core; stale/invalid quotes absent not zero; confirm/sign/broadcast ignore `RateSnapshotV1`; Pay remains enabled when every source fails | Rate fixtures + state-machine tests (RATE-001 / WAL-002 negative) |
| T34 | Quote worker learns private context | Provider query, quote-worker logs, and quote IPC carry no wallet or social secrets | Worker receives only fixed asset-ID / quote-currency queries; no wallet IPC handle, files, devices, identity, broker session, account IDs, addresses, balances, peer IDs, memos, payment amounts, or request IDs; query/log canary | Canary tests |
| T35 | Renderer or `bb-go` fetches rates | Social page and daemon never become HTTP rate clients | No renderer rate HTTP; no `/ob/exchangerates`; daemon stays wallet-free and rate-free; only the quote worker performs provider I/O | Policy + connect-src + daemon API tests |
| T36 | Broker spend core fetches or applies rates | Trusted prepare/sign/broadcast path is rate-free | No provider HTTP in the broker; `ReviewImageV1` / `intent_hash` omit quotes; fee and Pay eligibility ignore snapshots | Broker allowlist + intent-hash tests |
| T37 | IEEE-754 or amount-to-provider conversion | Display conversion cannot silently change value; providers never see a payment amount | Fixed-decimal strings; integer/decimal arithmetic only; fetch a unit price and convert locally; reject NaN/Inf/scientific/negative/zero prices | Decimal + parser tests |
| T38 | Unauthenticated P2P peers as a price oracle | Social graph cannot set displayed or signed value | No peer-supplied quotes in aggregation; providers are source-pinned configuration, not gossip | Aggregation tests |
| T39 | Inherited OB1 ticker path revived | No `ticker.openbazaar.org`, old Bittrex/Bitfinex/Poloniex/Kraken fallbacks, tickerproxy, or daemon `/ob/exchangerates` in product | Allowlist excludes those endpoints; package-content checks; no embedded API key or BitBook-operated mandatory proxy | Policy + SBOM + package-content |
| T40 | Dynamic repricing of a signed request | A signed `PaymentRequestV1` never changes `amount_atomic` because a quote moved | v1 requests are coin-denominated; later fiat-origin requests convert locally **once** before sign and record non-authoritative provenance; never reprice | Golden + state-machine tests |
| T41 | Zero-priced or disagreeing quotes look like free or exact fiat | Invalid quotes are omitted; disagreement is visible; UI says approximate or unavailable | Status `unavailable` / `disagreement`; never coerce to `"0"`; median only inside a bounded spread of fresh independent sources | Aggregation fixtures |
| T42 | Quote-worker handle mixed with broker IPC | A compromised rate path cannot speak spend methods | Distinct inherited pipes; supervisor never passes broker handles to the quote worker or quote handles to the broker spend core | Spawn/handle tests |

## 4. Process, repository, and IPC decision

### 4.1 Repository boundary

**Decision:** the local wallet broker belongs in `bb-desktop` as a dedicated
**Rust** native sidecar. `../bb-go/modern` remains wallet-free. Coin adapters,
signer backends, encrypted wallet state, device I/O, and the native
authorization surface live on the desktop side of the product boundary.

The broker source should later sit under a new tree such as
`wallet-broker/` (Rust crate) plus a small Electron supervisor module. Those
paths are not created by this ticket. The first implementation slice must not
install Rust or add coin crates.

**Why `bb-go` stays wallet-free:** the daemon is a long-running social identity
and transport process. Putting spend authority there would couple every
headless/social user to coin libraries, USB, and chain clients; it would also
turn the existing HTTP API into a wallet API, which this ticket forbids. The
daemon may later **transport and social-sign payment requests**. It may not
derive receivers, construct transactions, or broadcast.

**Why Rust despite adding a toolchain:** librustzcash is the maintained Zcash
wallet stack that already constructs v6 PCZTs and Ironwood bundles. A broker
that embeds that stack keeps NU6.3 semantics, PCZT external signing, and
note/nullifier state in one memory domain with explicit zeroization hooks.
Node/`utilityProcess` would put spend material in a V8 heap and the npm
supply chain. The Rust toolchain is an accepted packaging cost, not a reason
to put keys in Electron.

**Rejected alternatives:**

| Alternative | Reason rejected |
| --- | --- |
| Revive inherited `js/models/wallet`, `js/data/walletCurrencies.js`, marketplace spend modals, or `main.js` | Transparent multi-coin OpenBazaar wallet; BTC/BCH/ETH/escrow assumptions; old Electron stack; not NU6.3 Ironwood-capable shielded ZEC |
| Put the broker in `../bb-go/modern` | Violates wallet-free daemon; expands daemon HTTP into spend authority |
| Go sidecar in `bb-desktop` | Does not provide librustzcash; would need cgo/FFI or a Zcash rewrite; creates a second broker language for one contract; increases the chance of merging spend authority back into `bb-go`. Bound: v1 does not add a Go wallet process. Go remains appropriate for later `PaymentRequestV1` social-sign in `bb-go`, using the same golden vectors. |
| Spawn zallet as a process | Recreates a listening wallet API / process RPC. If zallet later exposes a stable **in-process** library that is the same librustzcash client, the ZEC adapter may use it **inside** the broker without a zallet listener. v1 does not spawn zallet. |
| Renderer-hosted wallet (JS/WASM keys in the social page) | Spend authority in the least trusted process; XSS = theft |
| Electron `utilityProcess` Node wallet | V8 heap for secrets, npm supply chain inside the signer, weak isolation from Electron updates |
| Electron-owned HTML confirm/unlock window | Spend authorization and passphrases would transit Electron, contradicting the trust boundary |
| zcashd / old multiwallet spawned like OpenBazaar | zcashd is not the maintained Zcash wallet path; transparent-era integration |
| Generic localhost wallet HTTP (`http://127.0.0.1:<port>/`) | Any local process and a future XSS `connect-src` can call it; **forbidden product/generic wallet HTTP**. This is distinct from contained authenticated loopback XMR **wallet RPC** used only by the XMR adapter behind the broker. |
| Listening UDS or named-pipe server | Unnecessary for one supervised child; any same-user process that can open the path can speak the protocol; v1 uses inherited anonymous pipes/handles only |
| WebUSB / renderer HID | Permission handlers deny all permissions; sandbox must remain; devices belong to the broker |
| Remote/custodial/light-wallet-with-viewkey-on-server | Centralized payment provider; out of scope |
| `../go-ipfs` anything | Deprecated |
| Inherited `/ob/exchangerates/<coin>`, `ticker.openbazaar.org`, old Bittrex/Bitfinex/Poloniex/Kraken fallbacks, or OB1 tickerproxy as production | Deprecated centralized/legacy rate chain; 2018-era BitcoinAverage/S3 service; not a current foundation |
| Fetch rates in `bb-go`, the social renderer, or the broker trusted spend core | Spend/social identity coupled to untrusted HTTP; quotes could influence prepare/fee; XSS could become a rate+wallet client |
| Unauthenticated P2P peers as a price oracle | Hostile peers set displayed value; not independent, not fresh, not pinned |
| Mandatory BitBook-operated rate proxy or embedded API key | Centralized payment-adjacent service; silent telemetry risk; key in the package |
| IEEE-754 doubles for unit price or atomic conversion | Silent rounding; non-canonical display; tests cannot oracle equality |

**Uncertainty:** exact crate names and versions for librustzcash/PCZT and the
packaged `monero-wallet-rpc` pin are implementation-ticket pins, not this
review. The adapter contracts below are stable even if the crate graph moves.

### 4.2 Process launch

Electron main is a **supervisor**, not a signer and not a confirm UI:

1. On app ready, after `app.enableSandbox()`, create a per-user broker
   directory under the Electron user-data path with mode `0700`. File modes
   protect other users on a shared host. They do **not** solve same-user OS
   compromise.
2. Verify the packaged `bitbook-wallet-broker` bytes against the packaged
   pin **before** spawn. Missing binary or hash mismatch fails closed. The
   pin is created by a later packaging ticket; until then tests use fixtures
   and do not spawn a real coin binary.
3. Create inherited **anonymous** bidirectional pipes/handles (POSIX
   `pipe`/`socketpair` with **no filesystem path**; Windows anonymous pipes).
   Do not `bind` a UDS, named-pipe listener, TCP port, or HTTP listener.
4. Spawn `bitbook-wallet-broker` with a cleaned environment, no renderer
   handles, working directory set to the broker data dir, **no secret in
   argv or environment** (no launch token, no RPC password, no passphrase).
5. Protocol frames use the inherited protocol pipe (child stdin/stdout or a
   dedicated inherited pair). **Redacted diagnostics** use a separate pipe
   (child stderr or a second inherited handle). Mixing protocol and
   diagnostics is a defect.
6. Fail closed if the child exits before handshake or if the hello transcript
   does not bind.
7. On quit, cancel in-flight intents, then SIGTERM the broker and its XMR
   children; do not leave `monero-wallet-rpc` bound.

The social daemon is not spawned as a wallet dependency. Existing user-run
`bitbookd` remains independent. XMR remains optional: if `monerod` is absent,
ZEC software accounts still function; XMR accounts show `NODE_UNAVAILABLE`.

The supervisor also spawns a **separate** quote worker (or equivalent
least-privileged child) with its own inherited anonymous pipes, its own
packaged-binary pin, and **no** broker pipe, wallet file descriptor, or
device handle. Quote-worker absence, crash, or provider failure degrades
only fiat presentation. It must not take the broker down or disable Pay.
BBD-RATE-001 specifies that child; WAL-002 and WAL-003 must not require
it.

### 4.3 IPC boundary

**Decision:** v1 broker IPC is **only** the inherited anonymous bidirectional
child channel. No listening wallet HTTP endpoint. No UDS path. No named-pipe
server. The social renderer never receives `ipcRenderer`, never sees child
handles, and never sees credentials.

Product/generic wallet HTTP is forbidden. The XMR adapter's authenticated
loopback wallet RPC is not IPC, is not exposed to Electron, and is not a
public API. See §8.2.

#### Framing

- Byte protocol: 4-byte big-endian length prefix + UTF-8 JSON object on the
  **protocol** pipe.
- Max frame 1 MiB. Methods that do not carry encoded transactions have a
  tighter 64 KiB limit. Oversize frames close the connection.
- One JSON object per frame. No concatenated JSON, no NDJSON.
- IPC framing JSON is **not** the signed canonical form. Signed objects use
  RFC 8785 JCS as specified in §11.2. v1 IPC does not use CBOR.
- Text encoding UTF-8. Unknown fields **fail closed** on all v1 broker
  control messages (handshake, begin, cancel, status, account, intent).
  Renderer snapshots may be reduced by the supervisor; the broker still
  fail-closes on unknown fields it receives.

#### Envelope (v1)

```text
{
  "v": 1,
  "id": "<32 lowercase hex>",
  "seq": 1,
  "kind": "hello" | "hello_ack" | "req" | "res" | "evt" | "cancel" | "error",
  "method": "<dotted.name>",
  "params": {},
  "result": {},
  "error": { "code": "LOCKED", "message": "<non-secret>", "retryable": false },
  "cancel_of": "<id>",
  "expires_ms": 30000
}
```

`seq` is monotonic per connection starting at 1 after hello. Reuse of `id` on
the same connection is `SCHEMA`. `expires_ms` is the broker's deadline for
that request.

#### Version negotiation and session binding

Child speaks first after spawn. **No shared secret is passed in argv or
env.**

1. Broker → supervisor `hello`:
   `{ "protocol": "bitbook-wallet-broker", "min": 1, "max": 1,
      "child_nonce": "<32 lowercase hex>", "child_pid": "<decimal string>" }`
2. Supervisor → broker `hello_ack`:
   `{ "protocol": "bitbook-wallet-broker", "version": 1,
      "parent_nonce": "<32 lowercase hex>", "parent_pid": "<decimal string>" }`
3. Both compute
   `session_id = SHA-256( "bitbook-wallet-session-v1\n" ||
     parent_pid || "\n" || child_pid || "\n" || parent_nonce || "\n" ||
     child_nonce )` as lowercase hex.
4. The next frame from each side is `req`/`res` carrying `"session":
   "<session_id>"` on the first post-hello message; mismatch exits.
5. Protocol name mismatch, no overlapping version, or pid/nonce format
   error: exit.

The transcript is bound to **both** process identities and both nonces.
Anonymous pipes already exclude unrelated processes that do not hold the
handles. Transcript binding detects confused splices and restart races.
Same-user malware that can steal handles or ptrace is residual (T26).

No application methods are accepted before `session_id` match.

#### Request correlation, cancellation, timeouts

- Every `req` gets a `res` or `error` with the same `id`, or the connection
  dies.
- `cancel` references `cancel_of`. Cancellation is best-effort against a
  racing terminal **non-broadcast** result. After `signed_unverified` or
  `verified`, a cancel **must** be applied before broadcast (T27).
- Default timeouts: handshake 2s, status 2s, prepare 30s, hardware sign 180s,
  broadcast 30s, sync subscribe idle 60s heartbeat.
- Supervisor may cancel on app quit and on user cancel from the social UI.
  Supervisor **cannot** confirm.

#### Error normalization

Stable codes only (string enum). `message` is safe for UI. No nested original
RPC bodies from `monero-wallet-rpc` or light servers. Codes:

`SCHEMA`, `UNAUTH`, `UNAVAILABLE`, `LOCKED`, `SYNCING`, `NODE_UNAVAILABLE`,
`DEVICE_DISCONNECTED`, `CAPABILITY_MISSING`, `PROTOCOL_INCOMPATIBLE`,
`INTENT_MISMATCH`, `EXPIRED`, `CANCELLED`, `REPLAY`, `WRONG_NETWORK`,
`AMOUNT_INVALID`, `TRANSPARENT_DOWNGRADE`, `ACCOUNT_BUSY`, `WATCH_ONLY`,
`MIGRATION_REQUIRED`, `LIMIT`, `STATE_CORRUPT`, `TIMEOUT`, `INTERNAL`.

Rate presentation uses **non-spend** snapshot status only
(`RATE_UNAVAILABLE`, `RATE_STALE`, `RATE_DISAGREEMENT` as display fields).
Those strings are never broker spend-error codes and never disable
prepare, confirm, sign, or broadcast.

`INTERNAL` never includes rust backtraces with paths to wallet files.

#### Crash/restart

- Supervisor emits `evt wallet.broker.down` as a non-secret `unavailable`
  snapshot to the renderer. It does not show a confirm UI.
- Restart does not preserve in-memory unlock. Software accounts return
  `LOCKED`.
- Durable intents: see §5.2 crash-recovery. Never auto-broadcast.
- Exponential backoff on spawn, capped, with no request buffering that could
  replay a spend.

#### Renderer prevention of generic wallet commands

BBD-SEC-001 must be **amended by a later ticket**, not weakened in place:

- Social preload exports a frozen `window.bitbookWallet` with only:
  `getSnapshot`, `subscribeSnapshot`, `beginIntent`, `cancelIntent`,
  `listAccounts`, `getPayeeRequest`.
- No `invoke(channel, ...)`, no `sendSync`, no method-name strings from the
  page.
- **Absent from every Electron preload and from `ipcMain`:**
  `intent.confirm`, `account.unlock`, `account.exportBackup`,
  `account.createSoftware`, `signer.sign`, `tx.broadcast`, `intent.broadcast`.
- There is **no** Electron confirm window and **no** confirm preload.
- Main `ipcMain` handler: exact channel allowlist, sender-frame checks,
  structured clone of known shapes, size limits matching the broker.
- Electron may render a sanitized preview of broker-supplied snapshot
  fields. That preview is not authoritative and has no Confirm-send control
  that reaches the broker as confirmation.
- The renderer does not `fetch` quote providers. Optional fiat comes only
  from the supervisor snapshot. `connect-src` must not gain CoinGecko,
  CoinPaprika, Kraken, `ticker.openbazaar.org`, or other rate hosts as a
  renderer permission.

**Rejected IPC alternatives:** Electron `ipcRenderer.invoke` with a generic
channel; HTTP-on-loopback with a bearer token in the renderer; Chromium
`window.postMessage` to an unsandboxed guest; sharing the social WebSocket
with wallet methods; UDS/named-pipe listeners; launch tokens in argv/env.

## 5. Versioned message and state-machine contracts

### 5.1 Shared primitives

```text
Asset       = "ZEC" | "XMR"
Network     = "zec-mainnet" | "zec-testnet" | "zec-regtest"
            | "xmr-mainnet" | "xmr-stagenet" | "xmr-testnet"
AccountId   = 32 lowercase hex issued by broker (not a receiver)
IntentId    = 32 lowercase hex issued by broker
RequestId   = 32 lowercase hex issued by payee desktop/daemon
PreparedId  = 32 lowercase hex issued by broker per successful prepare
AmountAtomic= decimal string "^[0-9]+$"  (no sign, no fraction, no leading zeros
              except the value "0")
PeerId      = BitBook peer ID string
```

Amount exponents (display only; never in protocol arithmetic as floats):

- ZEC: 8 (1 ZEC = 100000000 zatoshis)
- XMR: 12 (1 XMR = 1000000000000 piconero)

Maximum `AmountAtomic` length is 20 decimal digits. Zero is allowed only for
non-spend methods (e.g. receiver derivation tests). Spend amounts must be
`>= dust` per adapter.

Signed objects use the single canonical encoding in §11.2. IPC envelopes are
ordinary JSON as in §4.3. There is no "CBOR/JSON" choice.

### 5.2 Payment-intent state machine

Prepare happens **before** confirmation. Confirmation sees an exact fee.

```text
                beginIntent
                    |
                    v
                preparing
                    |
     prepare_ok / prepare_fail / expire / cancel / capability
                    |
                    v
                 prepared          (ReviewImageV1 exists:
                    |               amount, fee_atomic, fee_bound_atomic)
                    v
             awaiting_confirm
                    |
     user_confirm (broker native only) / cancel / expire / broker_down
                    |
                    v
                 signing
                    |
     sign_ok / sign_fail / disconnect / cancel / expire
                    |
                    v
            signed_unverified
                    |
            verify_ok / verify_fail
                    |
                    v
                 verified
                    |
     cancel+expiry recheck / crash / broadcast
                    |
                    v
              broadcasting
                    |
     accepted / rejected_by_network / unknown_needs_scan
                    |
                    v
          accepted_unconfirmed --> confirmed
```

Named states that later tickets must implement as distinct, testable values:

| State | Meaning |
| --- | --- |
| `preparing` | Bound request; adapter is building a prepared artifact |
| `prepared` | Exact `ReviewImageV1` exists, including fee and fee bound; not yet shown as awaiting |
| `awaiting_confirm` | Broker native surface is showing the prepared image; Electron preview is non-authoritative |
| `signing` | User confirmed; signer in progress; cancel/expiry still live |
| `signed_unverified` | Artifact present; verify has not succeeded |
| `verified` | Artifact matches confirmed `intent_hash`; not yet broadcast |
| `broadcasting` | Submit in flight |
| `cancelled` | Terminal; user or payee cancel won |
| `expired` | Terminal; `expires_at` reached or skew rule failed |
| `failed` | Terminal; prepare/sign/verify/capability/device/network rejection |
| `crash_recovery` | Durable signed artifact after crash; **not** terminal; cannot broadcast until fresh broker confirm + revalidate |
| `accepted_unconfirmed` | Broadcast accepted by adapter; not chain-confirmed |
| `confirmed` | Terminal success |

Invariants:

- No `awaiting_confirm` without `prepared`. Confirm UI/native surface must
  display `fee_atomic` and `fee_bound_atomic`. "Fee unavailable" is not a
  confirmable Pay.
- One broker-native `user_confirm` may authorize the happy-path
  `signing → signed_unverified → verified → broadcasting` sequence.
- After `sign_ok`, the broker **must** re-read cancel and expiry before
  verify completes into `verified`.
- Immediately before entering `broadcasting`, the broker **must** re-read
  cancel and expiry. If cancelled or expired, go to that terminal state and
  **do not broadcast**.
- No transition to `broadcasting` unless current state is `verified` **and**
  `verify(signed_artifact, intent_hash) == ok` **and** cancel/expiry clear.
- Watch-only accounts cannot leave `signing` via `sign_ok`.
- `intent_hash` is defined in §11.4. It is SHA-256 over domain-separated
  JCS of `ReviewImageV1`. It is **not** "CBOR/JSON" and is **not** specified
  in this section's historical §7 pointer.
- Terminal: `cancelled`, `expired`, `failed`, `confirmed`. Terminal states
  are durable.
- `rejected_by_network` is `failed` with that code. `DEVICE_DISCONNECTED`
  during sign is `failed` with that code.

Crash-recovery behavior:

| Durable state at crash | On restart (still locked) |
| --- | --- |
| `preparing`, `prepared`, `awaiting_confirm`, `signing` without a complete signed artifact | `failed` or `cancelled` (unsigned abort). Do not resume prepare silently. |
| `signed_unverified` or `verified` | `crash_recovery`. Re-show the recovered ReviewImage on the **broker** surface after unlock. Fresh user confirm. Revalidate artifact against `intent_hash`. Never auto-broadcast. If revalidation fails, `failed` / `INTENT_MISMATCH`. |
| `crash_recovery` | Stay until confirm, cancel, expire, or fail. |
| `broadcasting` | `unknown_needs_scan` then adapter status refresh. Do not blindly resubmit unless the adapter proves idempotent submit of the **same** verified artifact; still no new signature. |
| Terminal | Unchanged. |

### 5.3 Broker methods (v1)

Supervisor → broker, after handshake. Electron is never the confirm caller.

| Method | Caller | Effect |
| --- | --- | --- |
| `status.get` | supervisor | Non-secret broker liveness |
| `account.list` | supervisor (renderer may see sanitized) | Accounts + capabilities |
| `account.createSoftware` | broker native surface after onboarding confirm | Creates locked software account |
| `account.importWatchOnly` | broker native surface | Watch-only; never spend |
| `account.attachHardware` | broker native surface | Probe device; persist fingerprint + caps |
| `account.unlock` | broker native surface | Software unlock; timeout |
| `account.lock` | supervisor idle / native surface | Zeroize software spend material in process |
| `account.exportBackup` | broker native surface | Encrypted backup bytes; never to Electron |
| `receiver.fresh` | supervisor for payee request | Fresh private receiver bound to `request_id` |
| `intent.begin` | social preload path via supervisor | Bind request, start `preparing` |
| `intent.confirm` | **broker native surface only** | User-confirmed sign→verify→broadcast sequence |
| `intent.cancel` | supervisor or native surface | Abort if not yet successfully broadcast |
| `sync.subscribe` | supervisor | Events: sync, lock, device, intent status |

`intent.broadcast` is **not** an Electron-callable or supervisor-callable v1
method. Broadcast is an internal broker step after `verified` plus cancel and
expiry recheck. Crash recovery must not invoke that internal step without a
new `intent.confirm` on the broker surface.

Methods that must not exist in v1: `seed.export` to renderer, `rpc.raw`,
`http.proxy`, `monero.raw`, `zcash.raw`, `device.raw`, generic wallet HTTP
proxy, Electron-callable `intent.broadcast`, broker `rate.fetch`,
broker `rate.apply`, any method that sends an amount or account id to a
quote provider.

Quote-worker methods (separate pipe, RATE-001): `rate.query` with only
allowlisted asset IDs and quote currencies; `rate.snapshot` returning
`RateSnapshotV1`. No other methods.

### 5.4 Snapshot shown to the social renderer

```text
{
  "v": 1,
  "broker": "down" | "locked" | "ready" | "syncing" | "degraded",
  "accounts": [
    {
      "account_id": "...",
      "label": "Shielded ZEC",
      "asset": "ZEC",
      "network": "zec-testnet",
      "kind": "software" | "hardware_backed" | "watch_only",
      "privacy": "private" | "transparent_not_private" | "unknown",
      "capabilities": { "...": true },
      "balance_atomic": "0",
      "sync": { "state": "idle", "progress": 1 },
      "device": { "present": false, "label": "Ledger", "verified_fields": [] }
    }
  ]
}
```

Absence of keys, seeds, full receivers list, backup material, and raw RPC is
mandatory. A single current unused receiver may be included only when
generating a payee request, and only through `receiver.fresh`, not in the
periodic snapshot.

Sanitized preview of an in-flight intent may include asset, network, amount,
and "awaiting confirmation in BitBook Wallet". It must not include a working
Confirm control, passphrase field, or raw receiver-export button.

Optional presentation-only fiat estimate, **absent when no fresh usable
quote exists** (never `"0"` as a stand-in):

```text
"fiat_estimate": {
  "quote_currency": "USD",
  "price": "<DecimalString>",
  "approx_display": "<decimal string or omitted>",
  "method": "median" | "single_labeled_source" | "unavailable",
  "source_ids": ["<provider_id>", "..."],
  "label": "approximate" | "fiat estimate unavailable" | "quotes disagree",
  "status": "fresh" | "stale" | "disagreement" | "unavailable"
}
```

The field is optional on the snapshot. WAL-002 fixtures without it are
valid. Renderer and broker confirm may display it as approximate. They
must not use it as `amount_atomic`, fee, or Pay eligibility.

## 6. Account and signer capability model

### 6.1 Account kinds

All three kinds exist in the first common contract.

| Kind | Custody | Can view | Can derive fresh private receiver | Can sign spend |
| --- | --- | --- | --- | --- |
| `software` | Encrypted seed/spend keys in broker store | Yes after unlock | Yes after unlock | Yes after unlock, plus broker-native confirm |
| `hardware_backed` | Seed on device; host may hold viewing key if the device exported one | If viewing material present | If viewing material can diversify/subaddress | Only on device; no software fallback |
| `watch_only` | Viewing material only | Yes | If the viewing material supports it | Never |

An account is **eligible for native social Pay** only if
`can_receive_private && (can_sign_spend || kind==watch_only for request-only)`
and `privacy == "private"` and `network` matches the payment request.

For current ZEC mainnet (post-NU6.3), `can_receive_private` requires
`can_receive_ironwood` and an Orchard-protocol receiver in the default UA.
`can_sign_spend` for private Pay requires `can_tx_v6` and
`can_sign_ironwood`. `can_sign_orchard` without `can_sign_ironwood` is not
sufficient for current private Pay. Restored Orchard-pool notes do not make
the account spendable on Pay (`MIGRATION_REQUIRED`).

Transparent-only ZEC hardware is representable as
`kind=hardware_backed`, `privacy=transparent_not_private`,
`can_receive_private=false`. It is **not** a private account and **must not**
appear as the built-in ZEC wallet or as a Pay destination/source.

### 6.2 Device-neutral capabilities

Capabilities are booleans probed and persisted with a `probed_at` and
`probe_source` (`static_fixture` | `library` | `device_app`). UI copy may only
claim a behavior when the boolean is true **and** `consensus_branch` matches
the connected chain.

```text
CapabilitySet {
  can_view
  can_derive_fresh_receiver
  can_receive_private             // derived: current private receive path
  can_receive_orchard             // ZEC: scan/store Orchard-pool notes
  can_receive_ironwood            // ZEC: scan/store Ironwood-pool notes
  can_prepare_tx
  can_sign_spend                  // derived: current private spend path
  can_sign_orchard                // ZEC Orchard-pool spend
  can_sign_ironwood               // ZEC Ironwood-pool spend
  can_tx_v6                       // ZEC v6 transactions
  can_migrate_orchard_to_ironwood // ZIP-318 consented migration; false in v1 Pay
  can_sign_transparent            // never sufficient for privacy=private
  can_display_amount_on_device
  can_display_recipient_on_device
  can_display_network_on_device
  can_verify_pczt_on_device       // ZEC external signing
  can_export_viewing_material
  can_broadcast
  consensus_branch                // opaque string, e.g. ZEC NU6.3 branch id or XMR HF version
  pczt_version                    // ZEC only, or null; v6-capable PCZT for current mainnet
  tx_version_max                  // ZEC only, integer string "5" | "6"
}
```

XMR accounts set ZEC-only flags false/null. ZEC accounts do not invent an
Ironwood receiver capability; receive uses the Orchard-protocol receiver.

Signer backends implement `Signer`:

```text
prepare(intent) -> PreparedTx + ReviewImageV1
review_fields(prepared) -> ReviewImageV1
sign(prepared, auth) -> SignedTx
verify(signed, intent_hash) -> ok | INTENT_MISMATCH
```

`auth` is `software_unlock` collected by the broker native surface or
`device_session`. It is never a seed, passphrase, or PIN from Electron.

### 6.3 Prepare / review / sign / verify / broadcast

1. **Prepare** in the broker using the coin adapter **before** confirm.
   Outputs a coin-specific prepared artifact (ZEC v6 PCZT with Ironwood
   bundle when spending Ironwood; XMR unsigned tx set) plus coin-agnostic
   `ReviewImageV1`: asset, network, `amount_atomic`, `fee_atomic`,
   `fee_bound_atomic`, receiver bytes, change policy, memo hash,
   `request_id`, expiry, tx version, ZEC pool list.
2. **Review** on the broker native surface. If `can_display_*_on_device`,
   the surface states which fields the device will show and which it will
   not. Electron may show a sanitized preview only.
3. **Sign** with the selected signer only, after one user confirm.
4. **Verify** is mandatory and independent of the signer: decode signed
   artifact; compare network, destination, amount, fee (`fee_atomic` exact
   and `<= fee_bound_atomic`), memo hash, and `request_id` binding to the
   confirmed `intent_hash`. Failure is `INTENT_MISMATCH` and never
   broadcasts. Then recheck cancel/expiry.
5. **Broadcast** only after `verified` and a second cancel/expiry check.
   Broadcast is adapter-defined (ZEC via light submit or later local zebra;
   XMR via full wallet-rpc submit to local `monerod`). Not an Electron
   method.

**Rejected:** treating vendor SDK `sign()` success as intent success; skipping
verify for hardware; auto-broadcast on prepare or crash; fee-bump without a
new prepare+confirm; confirm before fee is known; Electron confirm.

## 7. ZEC adapter design

### 7.1 Maintained stack

**Decision:** implement ZEC inside the wallet broker with the maintained
Rust Zcash stack (librustzcash family: protocol/primitives, client backend,
SQLite wallet store, PCZT). Receive with Unified Addresses that include an
**Orchard-protocol** shielded receiver. Default product path is **shielded
receive and shielded spend into the current Ironwood pool** using v6
transactions. Use compact-block light sync so the built-in wallet does not
require the user to run a Zcash full node.

Do not shell out to `zcashd`. Do not embed the OpenBazaar wallet. Do not run
a generic zallet HTTP API or zallet process. If zallet later exposes a stable
in-process library API that is the same librustzcash client, the adapter may
switch behind this contract without changing IPC.

Every ZEC account stores `consensus_branch`, `tx_version_max`,
`pczt_version`, pool capabilities, and a wallet birthday. If the connected
chain, PCZT crate, light server, or signer probe cannot construct or sign v6
Ironwood spends while the network is NU6.3, the account is
`PROTOCOL_INCOMPATIBLE`. It may scan for awareness but must not create
receivers that the signer cannot later spend, and must not prepare spends.

librustzcash currently constructs v6 PCZTs and Ironwood bundles for
post-NU6.3 payments. The implementation ticket pins the crate versions. This
review does not pin them.

### 7.2 Light-client synchronization and network privacy

- Compact blocks are downloaded from a **user-configured** light endpoint.
  This review does not appoint a default public lightwalletd. Shipping a
  hardcoded operator endpoint would create a centralized chain service.
- Trial decryption happens locally. Viewing keys never go to the light
  server. Under ZIP-326, the Orchard-protocol incoming viewing key scans
  both Orchard and Ironwood pools.
- The light server is T04: it can stall, eclipse, or serve stale/compact-invalid
  data. The adapter must verify header/commitment consistency to the extent
  the chosen library supports and must fail closed on branch mismatch.
- IP-level privacy of compact-block download is not equivalent to shielded
  addressing. Tor/proxy is an owner choice (open question Q3), not a silent
  default in v1.
- No "restore from light server account". Restoration uses user backup or
  hardware + birthday. Restored Orchard-pool notes are visible if
  `can_receive_orchard` but are **not spendable** in v1 Pay
  (`MIGRATION_REQUIRED`).

**Rejected:** hosting viewing keys on a remote wallet server; using
transparent t-addr scanning as the default; assuming lightwalletd is a
trusted oracle of balances; inventing an Ironwood-only address type.

### 7.3 PCZT / external signing

Hardware and airgapped signers use PCZT:

- Host builds a **v6** PCZT from the prepared intent, including an Ironwood
  bundle when the spend consumes/creates Ironwood value.
- Device or external signer returns an updated PCZT.
- Host extracts the signed transaction, then runs §6.3 verify.
- `pczt_version` is part of `CapabilitySet`. A device or crate that cannot
  produce/consume the required PCZT version is `PROTOCOL_INCOMPATIBLE`, not
  a silent rewrite.

Software shielded spend may use in-process librustzcash signing of the same
prepared description so that hardware and software share prepare/verify.

### 7.4 Shielded-only receiving by default

- New ZEC accounts advertise a UA whose receivers are shielded
  **Orchard-protocol** receivers (ZIP-326). No transparent receiver is
  included in the default UA. There is no separate Ironwood receiver type.
- Native Pay refuses accounts with `can_receive_private=false`.
- Incoming transparent funds to a non-default address, if they occur, are
  displayed as `transparent_not_private` and cannot be spent by the social
  Pay button until a later owner-approved shielding ticket. That ticket is
  not implied here.
- Incoming/restored **Orchard-pool** notes after NU6.3 activation are not
  native Pay spendable. v1 fails closed. A later ticket may implement
  ZIP-318 migration with explicit consent for residual cross-pool amount
  disclosure. `can_migrate_orchard_to_ironwood` stays false for Pay until
  that ticket.
- `TRANSPARENT_DOWNGRADE` is raised if prepare would produce a transparent
  output other than an explicit, separately confirmed exception. v1 has **no**
  such exception for social Pay.

### 7.5 Hardware capability handling (ZEC)

This review **does not claim** that Ledger, Trezor, or Keystone currently
sign shielded Orchard-pool or Ironwood-pool v6 transactions. NU6.3 activated
on 2026-07-28; firmware and vendor apps may lag.

| Device | How it attaches | Private social Pay | Required representation |
| --- | --- | --- | --- |
| Keystone | PCZT airgap/QR or vendor transport via broker, never renderer | Only if probe sets `can_sign_ironwood`, `can_tx_v6`, matching `pczt_version` and `consensus_branch` | Otherwise attach as incompatible or watch-only if viewing material exists |
| Ledger | Vendor app over HID from broker | Only after a **verified** probe against a named app version and branch; unverified = `can_sign_ironwood=false` | May exist as `transparent_not_private` if only t-addr signing is proven; that account is not private |
| Trezor | Vendor app over HID/webusb-equivalent in broker | Transparent-only ZEC **must not** be represented as a private account, even if the device is honest | `privacy=transparent_not_private`; excluded from native Pay |

Probe algorithm (all vendors):

1. Identify device fingerprint (model, app name, app version).
2. Look up a **shipped capability table** pinned in the broker (reviewable
   fixtures). Unknown combo → all spend/Ironwood/v6 caps false.
3. Optionally run a live `get_features`-class call. Live results cannot
   *expand* capabilities beyond the pinned table; they may only *narrow*
   them (device disconnected, old app).
4. Persist the narrowed set. UI uses the persisted set, not vendor
   marketing strings.

A later real-device ticket may update the pinned table with evidence. Until
then, fake devices in tests implement the matrix, including a Trezor-like
transparent ZEC signer that fails if marked private, and an Ironwood-lag
device that is `PROTOCOL_INCOMPATIBLE` for current Pay.

### 7.6 Rejected ZEC alternatives

- Inherited OpenBazaar ZEC (transparent, old stack)
- zcashd RPC wallet
- zallet as a spawned process or HTTP API
- Advertising Sapling-only, Orchard-pool-only, or pre-v6 stacks as "current
  shielded" on post-NU6.3 mainnet
- Inventing an Ironwood receiver/address kind
- Auto-migrating restored Orchard-pool funds without ZIP-318 consent
- Equating "device shows ZEC app" with Ironwood/v6 signing
- Bundling a default remote lightwalletd as a mandatory service
- Putting ZEC keys in `bb-go`

## 8. XMR adapter design

### 8.1 Maintained stack and split processes

**Decision:** use maintained Monero software. The user runs a local `monerod`.
The broker manages a separate `monero-wallet-rpc` that holds/view/signs.
The renderer never talks to either RPC port. Product/generic wallet HTTP is
forbidden; this loopback wallet RPC is a contained adapter exception.

Two RPC classes, never collapsed:

| Interface | Process | Mode | Used for | Not used for |
| --- | --- | --- | --- | --- |
| Node RPC | user `monerod` | May use **restricted node RPC** where sufficient (height, info, submit if that is the chosen submit path) | Chain data, sync liveness | Keys, signing, subaddress creation |
| Wallet RPC | `monero-wallet-rpc` | Authenticated **full** wallet RPC on loopback. **Must not** pass `--restricted-rpc` (that flag is view-only) | View, subaddresses, spend, hardware wallet-rpc flags | Renderer, Electron, daemon, public bind |

Lifecycle:

```text
user monerod (loopback)
        ^
        | node RPC (restricted node RPC allowed)
        |
monero-wallet-rpc (loopback, random port, broker-generated auth,
        ^          full wallet RPC, spawned or detected, contained)
        |
        | authenticated full JSON-RPC, broker only
wallet broker
        ^
        | inherited anonymous pipes (not HTTP)
Electron supervisor
```

States are independent:

| Process | Syncing | Locked | Down |
| --- | --- | --- | --- |
| `monerod` | `NODE_SYNCING` | n/a | `NODE_UNAVAILABLE` |
| wallet-rpc | `WALLET_REFRESHING` | `LOCKED` | `UNAVAILABLE` |
| device | n/a | n/a | `DEVICE_DISCONNECTED` |

XMR account snapshot must not collapse these into a single "syncing" bit
without the substate.

Watch-only XMR is a wallet-file and capability property
(`can_sign_spend=false`), not `--restricted-rpc` on wallet-rpc. Using
restricted wallet RPC would make a later spend attach a mode footgun. v1
always uses full wallet RPC and enforces watch-only in the broker.

### 8.2 Authentication and bind

- Bind `127.0.0.1` only. IPv6 loopback only if also limited to the local user.
- Non-loopback host is a hard configuration error (T22).
- RPC login: broker-generated random username/password, held only in broker
  memory and/or the `0700` directory. Never argv, never environment, never
  renderer, never snapshots, never Electron IPC.
- `--confirm-external-bind` stays off.
- Do **not** pass `--restricted-rpc` to `monero-wallet-rpc`.
- Restricted RPC on **`monerod`** is allowed when the adapter only needs
  restricted node commands.
- Do not publish the port, login, or cookie to the renderer snapshot.
- No public or remote wallet-rpc fallback.

Residual limit: loopback plus random credentials do not stop same-user
malware. They do stop other users and remote processes. That is the intended
bar, stated honestly.

### 8.3 Subaddresses and watch-only

- Every payment request uses a fresh subaddress
  (`account_index`, `subaddress_index`) bound to `request_id`.
- Never bind the primary address to the social profile.
- Watch-only: import view-only wallet material into wallet-rpc; broker sets
  `can_sign_spend=false`.
- Hardware: wallet-rpc hardware-wallet flags; keys stay on device; wallet
  files on host are not a second hot seed.

### 8.4 Device disconnects and signing

Monero hardware signing typically needs the device present for spend, and
sometimes for initial view-key export. Rules:

- Disconnect during sign → `failed` / `DEVICE_DISCONNECTED`; no software-key
  fallback. Unsigned intents do not broadcast.
- If viewing can continue without the device, snapshot says `view_online,
  spend_requires_device`.
- Forged/truncated signed tx: same verify step against `ReviewImageV1`.

### 8.5 No public/remote node fallback

v1 configuration accepts only local `monerod`. A later owner decision would
be required for any remote node, including "trusted friend" nodes. This
review does not sneak that in as a hidden setting.

**Rejected:** embedding a public Monero node list; talking to node and wallet
as one process; exposing wallet-rpc to the social `connect-src`; using
payment IDs instead of subaddresses; treating stagenet and mainnet wallets
as interchangeable files; `--restricted-rpc` on wallet-rpc for a spending
account; presenting wallet-rpc as product HTTP.

**Uncertainty:** whether BitBook packages a pinned `monero-wallet-rpc` binary
or detects a user-installed official binary. That is owner question Q1. The
adapter API is the same.

## 9. Hardware support policy

### 9.1 First-class, capability-shaped

Ledger and Trezor are first-class **signer backends**. Keystone is a first-class
ZEC PCZT backend. None of them is "the ZEC wallet" or "the XMR wallet." The
account contract is the product; devices plug into it.

Policy:

1. Never display a vendor name as proof of shielded, Ironwood, v6, or private
   capability.
2. Never silently substitute software signing when a hardware account is
   selected.
3. Never expand capabilities from the network (no "download firmware support
   matrix from vendor"). Expansion requires a reviewed pin in source.
4. Real-device tests are a hard gate, not part of ordinary CI.
5. A device that can sign XMR but only transparent ZEC may be used for XMR
   private Pay and must still be `transparent_not_private` on ZEC.
6. Hardware confirmation of displayed fields happens **on the device**. The
   broker native surface still shows the full ReviewImage, including fields
   the device cannot show, labeled as host-trusting.

### 9.2 Verified-fields disclosure

Broker confirm surface lists `verified_fields`: a subset of
`{amount, recipient, network, fee, memo}`. Missing fields are labeled
"verified only on this computer." Electron preview must not claim device
verification.

### 9.3 NU6.3 and Monero hard-fork drift

After a consensus change, the pinned capability table may go stale. Broker
behavior: if adapter reports a newer `consensus_branch` than the table,
**narrow** to non-spend until a ticket updates the pin. Do not guess. Do not
treat Orchard-pool spend capability as Ironwood spend capability.

## 10. Key and recovery policy

### 10.1 Separation

Three secret domains, never one mnemonic for all:

| Domain | Owner process | Purpose |
| --- | --- | --- |
| Social identity | `bb-go` | Peer ID, profile/post/chat/payment-request signatures |
| ZEC | wallet broker | Shielded spend/view material |
| XMR | `monero-wallet-rpc` under broker control | Spend/view keys / hardware session |

Deriving any domain from any other is forbidden (T21).

### 10.2 Encrypted-at-rest software secrets

- Store under the broker data dir, mode `0600`.
- AEAD (e.g. XChaCha20-Poly1305) with a key from a KDF (Argon2id) of the user
  passphrase plus a random salt. If an OS keychain is used later, it wraps
  the data key; it does not replace account isolation. An OS keychain used
  for unlock must be started by the **broker**, not by Electron.
- Distinct files per account. No global "wallet.dat" mixing ZEC and XMR.
- Passphrase entry only on the owner-selected broker native surface. Never in
  the social page, never in Electron HTML, never in HTTP. A later OS keychain
  may wrap a data key but does not become the v1 passphrase-entry surface.

### 10.3 Hardware-derived accounts

Persist: device fingerprint, account index, viewing material if exported,
birthday/restore height, capability snapshot. Do not persist a seed. Do not
invent a software seed "backup" of a hardware account.

### 10.4 Backup and restore

- Software: user-initiated backup is an encrypted export opened via the
  **broker native surface** and a privileged save dialog owned by the broker
  (or an equivalently broker-controlled OS file dialog). Electron never
  receives the file bytes and must not proxy them.
- Restore is a broker-surface import. Rollback rules in T09 apply.
- Hardware: recovery is device seed recovery **on the device**, then
  re-attach. BitBook must not show or store device recovery phrases.
- Watch-only backup is viewing material, labeled "cannot spend."

### 10.5 Lock / unlock / idle

- Unlock lasts a bounded session (default 15 minutes of broker idle, reset
  on broker-confirm activity, not on renderer snapshot polls).
- App background / screen lock / broker restart → lock.
- `account.lock` zeroizes in-process spend keys. Viewing material may remain
  to allow sync, or drop too; v1 recommendation: drop spend keys, keep
  encrypted view material in memory only while unlocked if needed for
  scan, otherwise scan while locked using stored encrypted view keys
  decrypted to a view-only context. Implementation must not leave spend
  keys in Rust `String`s after lock.

### 10.6 Zeroization limits (honest)

Rust zeroization is best-effort: copies in allocator slack, core dumps, and
swap can remain. Controls: disable core dumps on the broker in packaging
tickets if the OS allows; mlock where practical; never log secrets; do not
claim "RAM is clean." Tests assert explicit buffer wipe hooks are called,
not that the OS cooperated.

### 10.7 Logs, diagnostics, evidence

Prohibited in logs, crash reports, ordinary test evidence, and reviewer
reports: seed words, spend keys, view keys, PINs, passphrases, session
nonces used as if they were long-term secrets in evidence, wallet-rpc
passwords, full PCZT/unsigned blobs, payment memos, mainnet addresses from
real users, backup bytes.

Allowed: `account_id`, `intent_id`, `request_id`, error codes, capability
booleans, synthetic fixture receivers in tests.

Protocol pipe and diagnostic pipe stay separate. Diagnostics are redacted.

### 10.8 Explicit exposure prohibitions

Renderer, social HTTP, product/generic wallet HTTP, crash reports, clipboard
(ordinary path), SBOM narratives, and CI logs must never contain seed or
spend-key material. Implementation tickets that need backup UX must use the
broker native surface and OS file dialogs owned by that surface.

**Rejected:** BIP39 displayed in the social webview; QR of seed; emailing
logs; reusing OpenBazaar server `walletd` files; Electron passphrase fields.

## 11. Payment-request design

### 11.1 Coin-agnostic signed request

Later implemented in `../bb-go/modern` as a social object; specified here so
both coins and the desktop broker share one shape. The daemon signs with the
**payee social identity**, not with coin keys.

Canonical fields (v1). `status` is **not** a field of this object (immutable
initial request; see §11.3).

```text
PaymentRequestV1 {
  v: 1                          // JSON integer 1 only
  request_id: string            // 32 lowercase hex
  payer_peer_id: string
  payee_peer_id: string
  asset: Asset                  // "ZEC" | "XMR"
  network: Network
  amount_atomic: AmountAtomic
  receiver: string              // fresh UA or XMR subaddress; not a profile field
  receiver_kind: "zec-ua-orchard-protocol" | "xmr-subaddress"
  memo: string                  // purpose; max 512 bytes UTF-8 NFC
  nonce: string                 // 32 lowercase hex
  created_at: TimestampV1
  expires_at: TimestampV1
}
```

`receiver_kind` `zec-ua-orchard-protocol` is the Unified Address with an
Orchard-protocol shielded receiver. It is valid for scanning Orchard and
Ironwood pools. It is not an Ironwood-specific address kind.

v1 requests are **coin-denominated**. There is no fiat field, no rate
field, and no quote provenance on `PaymentRequestV1`. A later
owner-approved ticket would be required before a "request $10" flow
(§12.6). Absence of any rate object is valid and does not affect
canonicalization.

### 11.2 Single normative canonical encoding

**Decision:** one cross-language canonical representation for
`PaymentRequestV1`, `PaymentStatusEventV1`, and `ReviewImageV1` /
`intent_hash` input: **RFC 8785 JSON Canonicalization Scheme (JCS)** over a
closed schema, then a domain-separated SHA-256.

This replaces any "CBOR/JSON" ambiguity. IPC framing JSON (§4.3) is not JCS
and is not signed. Implementors must not mix the two.

Domain separators (ASCII, including the final newline, no NUL):

```text
DS_PAYMENT_REQUEST_V1 = "bitbook-payment-request-v1\n"
DS_PAYMENT_STATUS_V1  = "bitbook-payment-status-v1\n"
DS_INTENT_HASH_V1     = "bitbook-intent-hash-v1\n"
```

Signed or hashed bytes:

```text
to_sign_or_hash = DS_* || JCS(object)
digest          = SHA-256(to_sign_or_hash)   // 32 bytes; hex lowercase in fixtures
```

The payee social signature is over `to_sign_or_hash` for
`PaymentRequestV1` or `PaymentStatusEventV1` as specified by the later
`bb-go` ticket. Desktop and broker consume the digest as
`payment_request_hash` / `status_event_hash`.

#### Closed-schema production rules

Signed objects contain only:

- JSON objects with UTF-8 string keys
- nested objects
- JSON arrays of the above values where specified (`zec_pools`)
- JSON integers **only** for `v` (the integer `1`)
- UTF-8 strings for every other field

Forbidden in signed objects: JSON floats, JSON null, JSON booleans, empty
keys, duplicate keys, comments, `undefined`, CBOR.

#### Timestamp normalization (`TimestampV1`)

Producers emit exactly this **shape**:

```text
^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$
```

The regex is a shape check only. It **accepts impossible calendar dates**
(for example `2026-02-30T00:00:00Z`, `2026-01-01T24:00:00Z`,
`0000-01-01T00:00:00Z`) and is therefore not sufficient.

UTC, second precision, `Z` suffix, no fractional seconds, no `+00:00`
offset, no leap-second `60`. Consumers **reject** any other form. They do
not normalize a different encoding into this form.

After the shape matches, consumers MUST:

1. Parse the digits as a **strict proleptic Gregorian UTC** date and
   time of day. The month, day, hour, minute, and second must exist on
   that calendar. Hour `24`, minute `60`, second `60`, month `00` or
   `13`, day `00`, and February 30 are `SCHEMA`.
2. Leap-day `YYYY-02-29` is valid only when `YYYY` is a Gregorian leap
   year (`Y` divisible by 4 and not by 100, unless also divisible by
   400).
3. Year must be in **2020–2100 inclusive**. Year `0000` and every year
   outside that closed range are `SCHEMA` even if the rest of the civil
   date exists.
4. Re-format the parsed instant to canonical `YYYY-MM-DDTHH:MM:SSZ`
   (zero-padded, `Z` suffix, no offset, no fraction). The re-encoded
   string **must equal** the input bytes (round-trip equality). Any
   parse that would fold hour-24, rewrite a timezone, or otherwise
   change digits is `SCHEMA`.

`expires_at` must be strictly greater than `created_at` as UTC instants
**after** this calendar validation.

Required golden failures: `2026-02-30T00:00:00Z`,
`2026-01-01T24:00:00Z`, `0000-01-01T00:00:00Z`. Additional required
failures: `2026-02-29T00:00:00Z` (2026 is not a leap year),
`2026-04-31T00:00:00Z`, `2026-13-01T00:00:00Z`,
`2101-01-01T00:00:00Z`, `2019-12-31T23:59:59Z`. A valid in-range
leap-day (e.g. `2024-02-29T12:00:00Z`) must pass shape, calendar,
year-range, and round-trip.

#### Unicode policy

- All strings are well-formed UTF-8, no BOM.
- `memo` must already be NFC (`UAX #15`). Non-NFC is `SCHEMA` (reject; do
  not silently compose).
- `memo` max 512 **bytes** of UTF-8.
- Reject C0 and C1 controls: U+0000–U+001F, U+007F, U+0080–U+009F.
- Reject noncharacters (U+FDD0–U+FDEF and U+nFFFE / U+nFFFF for planes
  0–16).
- Unpaired surrogates are impossible in well-formed UTF-8 and are
  `SCHEMA` if a decoder would produce them.
- Bidi formatting and isolate controls are **not** C0/C1 controls. Reject
  them explicitly in every signed-object string:
  U+202A LEFT-TO-RIGHT EMBEDDING,
  U+202B RIGHT-TO-LEFT EMBEDDING,
  U+202C POP DIRECTIONAL FORMATTING,
  U+202D LEFT-TO-RIGHT OVERRIDE,
  U+202E RIGHT-TO-LEFT OVERRIDE,
  U+2066 LEFT-TO-RIGHT ISOLATE,
  U+2067 RIGHT-TO-LEFT ISOLATE,
  U+2068 FIRST STRONG ISOLATE,
  U+2069 POP DIRECTIONAL ISOLATE.
- Additional deliberately prohibited format controls in signed-object
  strings:
  U+200B ZERO WIDTH SPACE,
  U+200C ZERO WIDTH NON-JOINER,
  U+200D ZERO WIDTH JOINER,
  U+200E LEFT-TO-RIGHT MARK,
  U+200F RIGHT-TO-LEFT MARK,
  U+061C ARABIC LETTER MARK,
  U+2060 WORD JOINER,
  U+206A–U+206F (deprecated ISO 6429 format controls),
  U+FEFF BYTE ORDER MARK / ZERO WIDTH NO-BREAK SPACE,
  U+FFF9–U+FFFB INTERLINEAR ANNOTATION CHARACTERS,
  U+E0001–U+E007F TAG characters (including language tags).
- `peer_id`, `receiver`, enums, and hex IDs: ASCII subset as specified;
  reject other Unicode there.

#### Unknown fields and exact validation

Validation is fail-closed. Order of checks for `PaymentRequestV1`:

1. Input is a UTF-8 JSON object with no BOM.
2. Parse. Reject trailing data.
3. Reject any key not in the closed field list in §11.1.
4. Reject missing keys.
5. `v` is JSON integer `1` (not the string `"1"`).
6. String fields have the types and regexes in this section.
7. `asset`/`network`/`receiver_kind` consistency:
   ZEC ↔ `zec-*` ↔ `zec-ua-orchard-protocol`;
   XMR ↔ `xmr-*` ↔ `xmr-subaddress`.
8. `amount_atomic` matches `^[1-9][0-9]{0,19}$` for spend requests (no
   leading zeros, not zero).
9. `request_id` and `nonce` match `^[0-9a-f]{32}$`.
10. Timestamps match `TimestampV1` **shape, strict calendar, year range,
    and round-trip equality**, and `expires_at > created_at`. Regex shape
    alone is not acceptance.
11. Unicode policy on `memo`, including explicit bidi formatting/isolate
    and listed format-control rejection.
12. Re-encode with RFC 8785 JCS. If a transport copy of canonical bytes is
    supplied, it **must equal** this JCS output (`SCHEMA` otherwise).
13. Compute `payment_request_hash = SHA-256(DS_PAYMENT_REQUEST_V1 || JCS)`.
14. Social-signature verify is the daemon's job when present. Broker still
    binds intents to `payment_request_hash` and rejects non-canonical
    objects.

The same unknown-field fail-closed rule applies to `PaymentStatusEventV1`
and `ReviewImageV1`.

#### Independent golden vectors

BBD-WAL-002 commits golden files containing, for each vector: input object,
JCS bytes, SHA-256 hex of `DS || JCS`, and pass/fail classification.
Adversarial vectors cover unknown keys, non-NFC memo, `+00:00` timestamps,
`"1"` vs `1` for `v`, CBOR-looking bytes, extra whitespace, key-order
permutations (must still JCS to the same bytes if valid), Ironwood receiver
kind (must fail), status-on-request (must fail), impossible
`TimestampV1` calendar values (February 30, hour 24, year 0000, and the
additional failures in the TimestampV1 rules), and memos containing
explicit bidi formatting/isolate controls (U+202A–U+202E, U+2066–U+2069)
or the other prohibited format controls listed above.

A Node oracle in WAL-002, using the existing desktop toolchain, must match
the committed hashes. Later **Go** (`bb-go`) and **Rust** (broker)
implementations must match the **same** hashes. Neither Go nor Rust is
installed in WAL-002.

### 11.3 Status events (immutable initial request)

The initial signed request has no `status` field and is never mutated.
Implied initial status is `open`.

Cancellation and paid/expired notifications are separate signed objects:

```text
PaymentStatusEventV1 {
  v: 1
  request_id: string            // 32 lowercase hex, same as the request
  event_id: string              // 32 lowercase hex, unique per event
  nonce: string                 // 32 lowercase hex, distinct from request nonce
  status: "cancelled" | "paid" | "expired"
  at: TimestampV1
  tx_ref: string                // empty string unless status=paid; never a key
}
```

Payee social-signs `DS_PAYMENT_STATUS_V1 || JCS(event)`. Payer broker
refuses to prepare, confirm, sign, or broadcast a request whose durable
status is `cancelled`, `paid`, or `expired`. A `cancelled` event received
after local `verified` still blocks broadcast (T27).

v1 may stop at local status plus payee-signed `cancelled`. A social `paid`
receipt is owner question Q8 and must not include viewing keys.

### 11.4 ReviewImageV1 and intent_hash

```text
ReviewImageV1 {
  v: 1
  intent_id: string
  prepared_id: string
  account_id: string
  request_id: string
  payment_request_hash: string  // 64 lowercase hex
  payer_peer_id: string
  payee_peer_id: string
  asset: Asset
  network: Network
  amount_atomic: AmountAtomic
  fee_atomic: AmountAtomic      // exact prepared fee; required before confirm
  fee_bound_atomic: AmountAtomic
  receiver: string
  receiver_kind: "zec-ua-orchard-protocol" | "xmr-subaddress"
  change_policy: "shielded_internal" | "xmr_change" | "none"
  memo_hash: string             // SHA-256 hex of UTF-8 memo bytes; 64 hex
  tx_version: string            // "6" for current ZEC Pay; "0" for XMR n/a
  zec_pools: array of "orchard" | "ironwood"   // XMR: empty array
  expires_at: TimestampV1
  prepared_at: TimestampV1
}
```

Rules:

- `fee_atomic` must be `<= fee_bound_atomic`.
- v1 ZEC native Pay: `tx_version` is `"6"` and `zec_pools` is
  `["ironwood"]` for new spends. `"orchard"` in `zec_pools` is not a v1 Pay
  path (`MIGRATION_REQUIRED`).
- `intent_hash = SHA-256(DS_INTENT_HASH_V1 || JCS(ReviewImageV1))`
  encoded as 64 lowercase hex.
- User confirm binds this `intent_hash`. Verify and broadcast check it
  byte-for-byte.
- `ReviewImageV1` contains **no** rate, fiat, or quote-provenance field.
  An approximate snapshot may be shown beside it; it is not hashed.

### 11.5 Protocol rules

- Fresh receiver required. Never bind a permanent public payment address to
  the social profile when a fresh receiver is available (it is available for
  private ZEC and XMR accounts by construction).
- Replay resistance: unique `request_id` + `nonce`; payer broker stores paid
  IDs durably; duplicates are `REPLAY`.
- Expiry: broker refuses prepare if `now >= expires_at` or clock skew exceeds
  a bounded skew (5 minutes) without a **new** prepared image and confirm of
  a refreshed request. Expiry is rechecked after sign and before broadcast.
- Cancellation: payee social-signed cancel; payer must not pay cancelled
  requests even if the receiver still "works" on chain. Recheck after sign
  and before broadcast.
- Payer binding: if `payer_peer_id` is set, only that local social identity
  may begin an intent. Open requests with empty payer are an owner choice
  (Q4); v1 recommendation: **always bind payer** for native Pay to reduce
  replay across the social graph. Empty `payer_peer_id` is `SCHEMA` in v1
  unless Q4 changes this.
- Amount is exact `amount_atomic`. No "pay any amount to this receiver"
  in v1 social Pay. No fiat-denominated v1 request. Optional local fiat
  display is not a request field and cannot reprice a signed object.
- Memo is user purpose, not a chain-consensus requirement. For ZEC it may be
  copied into the shielded memo if it fits; for XMR it stays off-chain in
  the request object unless the adapter has a tx description field. The
  broker confirm surface shows it either way.

### 11.6 Desktop ↔ daemon split

```text
payee broker --receiver.fresh--> desktop supervisor --HTTP social--> bb-go signs/sends
payer bb-go --stores request--> desktop --intent.begin--> broker
  --prepare ReviewImage--> broker native confirm --sign/verify/broadcast--> chain
```

`bb-go` never calls the broker. If the desktop is offline, the daemon may
still store an inbound request; Pay remains unavailable until the broker is
up. Renderer-supplied request bytes are a hint; schema and JCS checks still
run; when the daemon is present, the daemon's stored signed object wins.

**Rejected:** ZIP-321 or `monero:` QR as the ordinary path; putting a UA on
the profile; daemon-side coin clients; floating-point amounts; using chat
plaintext unsigned JSON as the only authenticator (the object may travel in
chat, but the social signature over JCS is the authenticator); CBOR as a
second canonical form; mutating `status` on the signed request;
fiat-denominated v1 requests; hashing quotes into `intent_hash`.

## 12. Exchange-rate architecture

Rates are **optional, untrusted presentation data**. Authoritative payment
fields remain `amount_atomic`, asset, network, receiver, and the prepared
fee. Wallet setup, balance, receive, Pay, signing, and broadcast **must
continue** if every rate source is missing, stale, disagreeing,
rate-limited, malformed, or offline.

The common contract (BBD-WAL-002) must not make a rate mandatory. Rate
implementation is **not** added to WAL-002. User-visible fiat UX waits
for a separately reviewable `BBD-RATE-001`.

### 12.1 What is rejected

- Daemon route `/ob/exchangerates/<coin>` and desktop
  `js/utils/currency.js` / `js/utils/exchangeRateSyncer.js` as product
  code.
- `https://ticker.openbazaar.org/api` and the inherited Bittrex,
  Bitfinex, Poloniex, and old Kraken fallbacks.
- OB1 `tickerproxy` as running infrastructure or vendored code (2018
  BitcoinAverage/S3 cache; historical evidence only).
- Fetching rates in `bb-go`, the social renderer, or the wallet broker's
  trusted spend core.
- Unauthenticated P2P peers as a price oracle.
- Embedded API keys, silent telemetry, or a mandatory BitBook-operated
  proxy.
- IEEE-754 floating point for unit prices or atomic conversion.
- Sending a payment amount, account id, address, peer id, memo, or
  request id to a provider.
- Treating a missing quote as atomic `"0"` or as a free payment.
- Dynamically repricing a signed `PaymentRequestV1`.

### 12.2 Quote worker process

**Decision:** a separate least-privileged desktop quote worker (or
equivalent child), spawned by the Electron supervisor, with inherited
anonymous bidirectional pipes **distinct** from the broker session.

The worker:

- has **no** wallet IPC handle, wallet files, device access, social
  identity, broker session, account IDs, addresses, balances, peer IDs,
  memos, payment amounts, or request IDs;
- receives only a fixed **asset-ID / quote-currency** query from the
  allowlist;
- emits a bounded normalized `RateSnapshotV1` to desktop presentation;
- is the only process that performs provider HTTPS;
- cannot construct, sign, or broadcast a transaction.

The supervisor may copy a sanitized snapshot into:

- the renderer wallet snapshot as optional `fiat_estimate`;
- the broker native confirm **display overlay** labeled approximate.

The broker spend core, prepare, fee calculator, state machine,
`intent_hash`, sign, verify, and broadcast **must ignore** that overlay.
A broker confirm still works when the overlay is absent.

**Rejected placements:** in-broker HTTP client beside librustzcash;
renderer `fetch` to CoinGecko/CoinPaprika/Kraken; `bb-go` cron; sharing
the broker protocol pipe.

### 12.3 Provider interface and `RateSnapshotV1`

Provider choice is **replaceable and source-pinned in configuration**.
This review does **not** promise that CoinPaprika, CoinGecko public,
direct Kraken market data, or any ZEC/XMR pair will remain available.
A later provider ticket may evaluate those three against **current**
terms and availability.

Pinned configuration names a provider id, base URL, TLS requirements,
stable asset-id mapping (not an ambiguous ticker), quote-currency
allowlist, and parser. No API key is embedded in v1.

Normalized schema (presentation; **not** a signed JCS object in v1):

```text
DecimalString = unit prices are strictly positive.
              = "^[1-9][0-9]{0,11}(\.[0-9]{1,18})?$"
                or "^0\.[0-9]{1,18}$" containing at least one non-zero digit
              no sign, no exponent, no NaN/Inf, no grouping, not "0"

RateQueryV1 {
  v: 1                              // JSON integer 1
  asset_ids: [stable ids]           // from the pinned allowlist only
  quote_currencies: ["USD", ...]    // ISO 4217 allowlist
}

RateQuoteV1 {
  v: 1
  asset: "ZEC" | "XMR"              // after pinned map; fail closed on collision
  asset_id: string                  // stable id from the pin, not a raw ticker
  quote_currency: string
  price: DecimalString              // quote currency per 1 whole coin
  provider_id: string
  provider_observed_at: TimestampV1
  fetched_at: TimestampV1           // local fetch time
  fresh_until: TimestampV1
  expires_at: TimestampV1
  status: "fresh"
}

RateSnapshotV1 {
  v: 1
  queried_at: TimestampV1
  quotes: [RateQuoteV1]             // well-formed only; invalid omitted
  display: {
    asset: "ZEC" | "XMR" | omitted
    quote_currency: string | omitted
    price: DecimalString | omitted
    method: "median" | "single_labeled_source" | "unavailable"
    source_ids: [provider_id]
    spread: DecimalString | omitted
    label: "approximate" | "fiat estimate unavailable" | "quotes disagree"
  }
}
```

Unknown fields on quote-worker control messages fail closed. Only
`status=fresh` quotes appear in `quotes`. Stale, invalid, unavailable,
or unparsable provider results are **omitted**, not stored as `"0"` or
as a kept stale row. Aggregate outcome lives in `display.method` /
`display.label`.

Local display conversion (never IEEE-754):

```text
approx = amount_atomic * price / 10^exponent
```

using integer or decimal arithmetic with a documented rounding rule
(v1 recommendation: round half-even to the quote currency's minor
units, max 2 decimal places for USD). The result is a display string.
It does not enter `ReviewImageV1`.

Do **not** send `amount_atomic` to a provider. Fetch a unit price;
convert locally.

### 12.4 Bounds

| Bound | v1 rule |
| --- | --- |
| Response body | Max 64 KiB; oversize is that provider invalid |
| Decimal digits | Shape in `DecimalString`; more digits → invalid quote |
| Time skew | `provider_observed_at` more than 5 minutes in the future → invalid |
| Freshness | Quote older than `fresh_until` is stale → omit, do not zero |
| Expiry | After `expires_at` the quote must not be displayed |
| Redirects | **None.** Any 3xx is that attempt failed |
| TLS | HTTPS only, TLS 1.2+, no renderer-supplied CAs, no HTTP downgrade |
| Parse depth | JSON nesting max 8; reject the body beyond that |
| Asset allowlist | Only pinned stable ids mapped to `ZEC` / `XMR`. Duplicate ids or ticker collision → that provider invalid |
| Quote allowlist | Only configured ISO 4217 codes (v1 recommendation: `USD` until Q11) |
| Timeouts | Connect 5s, overall 10s per provider |
| Cache lifetime | Max 5 minutes for a `fresh` quote; do not serve past `expires_at` |
| Backoff | Exponential on failure, cap 1 hour, no request amplification, no retry storms |
| Query | Asset ids + quote currencies only |

### 12.5 Aggregation and failure

- Stale or invalid quotes are **absent, not zero**.
- Provider disagreement is **visible** and **cannot block** coin
  payment.
- Provider data **cannot** change fees or Pay eligibility.
- Prefer a **median** only when enough **fresh independent** providers
  agree within a bounded spread (exact N and spread are RATE-001 / Q12;
  this review requires the rule to exist and to fail closed to a weaker
  display rather than invent a blended number).
- Otherwise show **one labeled source** or **"fiat estimate
  unavailable"**, according to the later provider ticket.
- Do not use unauthenticated P2P peers as a price oracle.
- Rate-limited, timed-out, redirected, or malformed bodies drop that
  provider for the backoff window.

### 12.6 v1 product behavior and later fiat-origin requests

**v1 recommendation:** coin-denominated signed payment requests only,
with optional approximate local fiat display.

If a later **owner-approved** ticket allows “request $10”:

1. Conversion occurs **locally once** before signing, using a then-fresh
   snapshot and the documented rounding rule.
2. The immutable signed request still contains exact `amount_atomic`.
3. A signed **non-authoritative** quote-provenance object may record
   quote currency, decimal price, source ids, observation time, expiry,
   rounding rule, and the resulting `amount_atomic`.
4. The signed request is **never** dynamically repriced when later
   quotes move.
5. That provenance is a new schema and a new domain separator. It is
   not added by WAL-002 or RATE-001 without that owner ticket.

### 12.7 Proposed ticket `BBD-RATE-001`

Separately reviewable, **before** user-visible wallet UX (before
BBD-WAL-010). Test-first with **recorded fixtures only**. Ordinary tests
use **no network** and **no API key**.

In scope:

- Quote-worker framing (inherited pipes, no broker handle).
- `RateQueryV1` / `RateQuoteV1` / `RateSnapshotV1` codecs.
- Fake providers over recorded bodies.
- Malformed and oversized bodies, duplicate assets, symbol collision,
  negative / zero / extreme / non-finite / scientific values, stale and
  future timestamps, provider disagreement, timeouts, redirects,
  cache/backoff, deterministic decimal rounding.
- Query/log canary proving no private context is sent.
- Proof that Pay / prepare / confirm eligibility fixtures still pass
  when every quote is absent.

Must not: live provider calls in ordinary CI, API keys, wallet IPC in
the worker, fee changes from quotes, adding rate fields to
`PaymentRequestV1` or `ReviewImageV1`, implementation inside WAL-002.

## 13. UX states

Wallet onboarding is skippable. Social features work with broker down.

### 13.1 Onboarding

Onboarding happens on the **broker native surface**, not in Electron HTML.
Electron may offer "Set up payments" which **requests** onboarding. The
broker owns the flow:

1. Skip (default until user opts in).
2. Create software ZEC account (built-in basic wallet) on testnet until the
   mainnet gate. Disclose NU6.3 / Ironwood receive via Orchard-protocol UA.
3. Attach hardware (probe → show capabilities in plain language, including
   v6/Ironwood/Orchard flags and host-trusting fields).
4. Import watch-only.
5. Optional: enable XMR (requires local `monerod`; otherwise explain and
   stop).

Hot versus hardware: labels are "On this computer" versus "On your device".
Watch-only is "Cannot send."

Passphrase, backup, and seed-adjacent material never appear in Electron.

Onboarding and account creation **do not wait** on quote sources. A missing
rate worker is not an onboarding error.

### 13.2 Native Pay entry

Pay appears on profile, post, and conversation when:

- local broker `ready` or `syncing` with at least one private account that
  can sign the request's asset, **or** the user is the payee creating a
  request
- the counterparty request is valid

No address copy, no QR on this path. Advanced "show receiver" is behind an
explicit disclosure and is not required. Electron Pay control calls
`beginIntent` only.

Fiat estimate availability is **not** a Pay-entry condition. If
`fiat_estimate` is absent, stale, disagreeing, or unavailable, Pay still
appears. The UI may show "fiat estimate unavailable" next to the coin
amount. It must not hide Pay or treat the coin amount as zero.

### 13.3 Confirmation (broker native surface)

Authoritative confirm is **not** an Electron window. The broker shows:

- asset and **network name in full** (e.g. `zec-testnet`, not a green icon)
- payee or payer display name **and** full peer ID
- amount in display units **and** `amount_atomic`
- **exact** `fee_atomic` in display units **and** atomic, plus
  `fee_bound_atomic`
- receiver truncated with expand-to-full; full bytes available without copy
- memo/purpose
- account kind and device `verified_fields`
- ZEC tx v6 / Ironwood pool (or XMR subaddress)
- expiry
- for hardware: which fields the device will show
- optional **explicitly approximate** fiat overlay from `RateSnapshotV1`,
  or the string "fiat estimate unavailable" / "quotes disagree". This
  overlay must not change `amount_atomic` or `fee_atomic`. Confirm stays
  enabled when the overlay is missing.

Buttons: Cancel, Confirm send. Confirm disabled while `SYNCING`, `LOCKED`,
`DEVICE_DISCONNECTED`, `PROTOCOL_INCOMPATIBLE`, `WATCH_ONLY`,
`MIGRATION_REQUIRED`, or while fee is missing. Confirm is disabled until
state is `awaiting_confirm`. Confirm is **not** disabled for
`RATE_UNAVAILABLE`, `RATE_STALE`, or `RATE_DISAGREEMENT`.

One Confirm send authorizes sign→verify→broadcast with the rechecks in §5.2.
There is no second Electron "broadcast" click in the happy path. After crash
recovery, Confirm send is required again on this surface.

Electron may show a sanitized, non-authoritative preview and a Cancel
control. It must not show an authoritative Confirm send, unlock field, or
backup export.

### 13.4 Non-spend states

| State | User-visible | Spend |
| --- | --- | --- |
| Broker unavailable | "Wallet is not running" | No |
| Locked | "Unlock in BitBook Wallet" (broker surface) | No |
| Syncing | progress, still no Pay confirm | No |
| Node unavailable (XMR) | "Local Monero node is not running" | No XMR |
| Device disconnected | "Connect the device labeled …" | No for hardware account |
| Protocol incompatible | "This account cannot send on the current Zcash/Monero network" | No |
| Migration required | "Restored pre-Ironwood shielded funds cannot be sent until a later consented migration" | No native Pay |
| Transparent not private | "This device account is transparent and is not available for private BitBook payments" | No native Pay |
| Ready | Pay enabled (`beginIntent`) | Confirm on broker surface |
| Awaiting confirm | "Confirm in BitBook Wallet" | Broker confirm only |
| Broadcasting / unconfirmed / confirmed / failed | status on the thread | No double pay |
| Crash recovery | "Wallet restarted. Confirm again in BitBook Wallet to send, or cancel." | No auto-broadcast |
| Fiat estimate unavailable / stale | "Fiat estimate unavailable" beside the coin amount | Yes — coin Pay unchanged |
| Quotes disagree | "Quotes disagree" / one labeled source; never a silent blend | Yes — coin Pay unchanged |

### 13.5 Failed Pay

Errors use the normalized codes. `INTENT_MISMATCH`,
`TRANSPARENT_DOWNGRADE`, and `MIGRATION_REQUIRED` are explained as blocked
for safety, not retried automatically. Quote failures are presentation,
not Pay failures.

## 14. Offline test and falsification plan

`TESTING.md` applies. Wallet tests are offline, deterministic, and
credential-free. No live public daemon, no mainnet, no real device in
ordinary commands. WAL-002 adds canonical golden vectors and the corrected
state/capability model without Electron UI, Rust installation, coin
dependencies, network, devices, wallets, nodes, or real keys. WAL-002 does
**not** implement the quote worker. It must prove the common contract does
not require a rate. `BBD-RATE-001` owns recorded rate fixtures, still
offline and credential-free.

### 14.1 Independent oracles and fixtures

- Canonical `PaymentRequestV1`, `PaymentStatusEventV1`, `ReviewImageV1`
  encode/decode fixtures: JCS bytes, domain-separated SHA-256, adversarial
  extras, unicode memos, max/min amounts, network mix-ups, unknown fields,
  non-canonical timestamps, invented Ironwood receiver kind (fail), status
  field on the request (fail), impossible calendar timestamps (February
  30, hour 24, year 0000, and the additional TimestampV1 failures), memos
  with U+202A–U+202E, U+2066–U+2069, and the other prohibited format
  controls.
- `intent_hash` oracle: committed hashes. WAL-002 Node hasher must match.
  Later Go and Rust hashers must match the same vectors.
- Amount tables: `"1"` ZEC display ↔ `"100000000"` atomic; `"0.00000001"`;
  XMR 12-exp; reject `"1e8"`, `"1.0"`, `"1,000"`.
- Capability matrix fixtures: software ZEC private with v6+Ironwood,
  software XMR private, watch-only both, Ledger-like XMR with device fields,
  Trezor-like transparent ZEC **not private**, Keystone-like PCZT Ironwood
  when the fixture says so, NU6.3 mismatch, restored Orchard-pool
  `MIGRATION_REQUIRED`, `can_sign_orchard` without `can_sign_ironwood`
  ineligible for current Pay.
- Intent machine fixtures: prepare-before-confirm; confirm without fee
  rejected; post-sign cancel; pre-broadcast expiry; crash_recovery never
  auto-broadcasts; prepare/confirm/sign eligibility with **no**
  `RateSnapshotV1` present.
- RATE-001 recorded provider bodies (no network, no API key): malformed
  and oversized JSON, duplicate assets, symbol collision, negative / zero
  / extreme / non-finite / scientific prices, stale and future
  `provider_observed_at`, disagreement, timeouts, redirects, cache and
  backoff, deterministic decimal rounding, query/log canary with no
  private context. Invalid quotes absent not zero. Median only inside the
  bounded spread; otherwise labeled source or unavailable.

### 14.2 Parser fuzzing and properties

- Length-prefixed frame parser: truncated length, oversize, invalid UTF-8,
  extra bytes, `v=0`, missing `id`, mixed diagnostics on the protocol pipe.
- Payment-request parser: field omission, wrong types, unknown asset,
  receiver_kind vs asset mismatch, unknown keys, CBOR input.
- Properties: confirm∘mutate(receiver) never verifies; paid request never
  prepares; cancel after sign never broadcasts; watch-only never signs;
  transparent ZEC never `privacy=private`; Orchard-pool-only restored
  balance never Pay-spendable; restricted-rpc spawn args never used for
  wallet-rpc; Pay eligibility is independent of quote presence; applying
  a quote never changes `fee_atomic` or `amount_atomic`; IEEE-754 parse
  of a rate body is rejected.

### 14.3 Boundary, failure, compound failure

- Timeouts and cancellation racing sign.
- Broker kill during `verified` (must become `crash_recovery`, not broadcast).
- Corrupt SQLite/wallet file → `STATE_CORRUPT`.
- `monero-wallet-rpc` auth fail; `monerod` down while wallet-rpc up (later
  adapter tickets; WAL-002 uses fakes).
- Device disconnect between prepare and sign, then reconnect with a
  different fingerprint.
- Compound: crash during lock + corrupt intent log.

### 14.4 Fake signer / device / node

First implementation uses only fakes, in-process, no coin libraries:

- `FakeZecAdapter` that understands v6/Ironwood/Orchard flags and **cannot**
  serialize a broadcastable mainnet transaction
- `FakeXmrAdapter` with stub node RPC vs stub full wallet RPC distinction
- `FakeSoftwareSigner`, `FakeHardwareSigner`, `FakeWatchOnly`
- Hardware fake can be scripted to sign the **wrong** destination for T12

Real `monerod`, real lightwalletd, and real USB are out of ordinary tests.

### 14.5 Secret canaries

Tests inject seed-like strings and spend-key-like hex into adapter internals
and assert they do not appear in log buffers, IPC snapshots, error messages,
argv, or env. A canary appearing is a failed test, not a redaction task.

RATE-001 injects account IDs, addresses, balances, peer IDs, memos,
payment amounts, and request IDs into the quote-worker environment and
asserts the outbound query and logs contain none of them.

### 14.6 Falsification (required per ticket)

High-value tests to break without committing the break:

- Allow a social preload method `invokeRaw` and prove the security test
  fails.
- Put `intent.confirm` on an Electron preload and prove allowlist tests fail.
- Skip post-sign verify and prove an `INTENT_MISMATCH` fixture still
  broadcasts (must fail the test).
- Skip post-sign cancel recheck and prove a cancelled verified intent
  broadcasts (must fail the test).
- Auto-broadcast from `crash_recovery` and prove the crash test fails.
- Mark Trezor-transparent fixture `privacy=private` and prove Pay
  eligibility tests fail.
- Bind a non-loopback XMR node and prove adapter tests fail.
- Spawn wallet-rpc with `--restricted-rpc` and prove spend tests fail.
- Drop session transcript binding or put a secret in argv and prove
  handshake tests fail.
- Advertise an Ironwood receiver kind or auto-migrate Orchard-pool notes
  and prove capability tests fail.
- Accept February 30, hour 24, or year 0000 as `TimestampV1` and prove
  calendar tests fail.
- Accept a memo containing U+202E or U+2066 and prove Unicode tests fail.
- Require `RateSnapshotV1` for `intent.begin` and prove WAL-002 still
  fails that test (rates must remain optional).
- Apply a quote to mutate `fee_atomic` or `amount_atomic` and prove
  RATE-001 / intent tests fail.
- Treat a missing quote as `"0"` and prove aggregation tests fail.
- Put an account id or amount on `RateQueryV1` and prove the canary
  fails.
- Follow a redirect or use `ticker.openbazaar.org` in the provider pin
  and prove allowlist tests fail.

### 14.7 Hard gates (not ordinary CI)

| Gate | When |
| --- | --- |
| Testnet/stagenet with local fake or owner-local nodes | After adapters exist; still no mainnet |
| Real Ledger/Trezor/Keystone | Dedicated ticket; owner-held devices; evidence without secrets |
| Mainnet construction/sign/broadcast | Forbidden until an explicit mainnet-gate ticket accepted after capability evidence |

Mutation of production verify/allowlist is the falsification method; real
funds are never the oracle.

## 15. Security, SBOM, and release gates

Wallet work inherits BBD-SEC-001: sandbox on, context isolation on, no
renderer Node, denied navigation/permissions, blocking `npm audit`,
complete-history and current-tree Gitleaks, CycloneDX SBOM on manual
dispatch.

Additions when broker binaries and coin libraries exist:

- Packaged application **retains** `chrome-sandbox` mode `4755` on Linux.
  Hardware access is not a sandbox exception.
- SBOM must include the wallet-broker crate graph, the quote-worker graph
  if shipped, and any bundled `monero-wallet-rpc` / Zcash library
  artifacts, not just npm.
- Pin crate versions and binary hashes in a later packaging ticket. Spawn
  verifies that hash (T32). Quote-worker pin is separate from the broker
  pin.
- New findings fail the ratchet. Coin libraries will introduce a reviewed
  inherited baseline **only** by a named ticket; this review does not
  pre-authorize suppressions.
- Package-content checks: no seed fixtures, no mainnet wallet files, no
  default remote-node lists, no renderer `connect-src` to wallet ports, no
  listening UDS path in shipped config, **no embedded API keys**, no
  `ticker.openbazaar.org`, no OB1 tickerproxy, no inherited
  `/ob/exchangerates` product route, no mandatory BitBook rate proxy.
- Security scans of maintained source must include supervisor + preload +
  broker glue + quote-worker glue. Inherited OpenBazaar `js/models/wallet`
  and `js/utils/currency.js` / `exchangeRateSyncer.js` stay unmaintained
  reference and are not revived or scanned as product wallet or rate
  code.
- Quote-worker network in ordinary tests is forbidden. Live provider
  evaluation, if ever needed, is a named RATE ticket with recorded
  fixtures as the acceptance oracle.

Release: no mainnet spend in a published build until the mainnet gate. A
build that includes the broker may still ship with testnet defaults and
mainnet **account creation disabled**.

## 16. Ordered ticket decomposition

Both coins and all three account kinds shape ticket 1's fixtures even when
real adapters are stubs. WAL-002 is offline, deterministic, credential-free,
and cannot construct or move funds.

| Order | Ticket (proposed id) | Repo | Goal | Must not |
| --- | --- | --- | --- | --- |
| 1 | BBD-WAL-002 | bb-desktop | Common contract: primitives, NU6.3 capability matrix (v6, Orchard, Ironwood receive/spend, PCZT version, migration flag), prepare-before-confirm intent machine including crash_recovery, JCS golden vectors for PaymentRequestV1/status/intent_hash including strict TimestampV1 calendar and explicit bidi/format-control failures, Node oracle matching committed hashes, fake ZEC+XMR adapters, fake software/hardware/watch-only signers, secret canaries. Existing Node toolchain only. **Rates are not a required field**; Pay fixtures pass with no `RateSnapshotV1` | Electron UI; Rust install; coin crates; network; devices; wallets; nodes; real keys; constructing or moving funds; **quote worker / provider HTTP / making a rate mandatory** |
| 2 | BBD-WAL-003 | bb-desktop | Supervisor spawn, packaged-binary verify, inherited anonymous pipes, session transcript binding, **allowlisted social preload without confirm/unlock/backup/broadcast**, amend BBD-SEC-001 tests from "no IPC" to "fail-closed allowlist", broker method allowlist, native-confirm stub owned by broker | Electron confirm window; keys in main; generic invoke; UDS/named-pipe listeners; secrets in argv/env; transaction construction; mixing quote-worker handles into the broker pipe |
| 3 | BBD-WAL-004 | bb-desktop | Encrypted software custody, lock/unlock on the owner-selected in-broker native surface, broker-invoked backup/restore file dialogs, zeroize hooks, domain separation tests | Renderer or Electron passphrase fields; seed display in social page; backup bytes in Electron |
| 4 | BBD-WAL-005 | bb-desktop | Pay snapshot gating, sanitized Electron preview, Trezor-transparent ineligibility, Orchard-pool `MIGRATION_REQUIRED` gating, receiver.fresh for payee path. Optional `fiat_estimate` may be absent | Electron confirm; bb-go protocol implementation (spec/fixtures only); requiring quotes for Pay |
| 5 | BBD-RATE-001 | bb-desktop | Quote worker + `RateQueryV1`/`RateSnapshotV1`, pinned replaceable providers, recorded-fixture parsers, aggregation (absent-not-zero, disagreement visible, median only inside bound), decimal conversion, query/log canary, Pay still works with every source down. Before user-visible wallet UX | Network in ordinary tests; API keys; wallet IPC/files/devices/identity/amounts in the worker; fee or `amount_atomic` mutation from quotes; OB1 ticker path; P2P price oracle; implementation inside WAL-002 |
| 6 | BBGO-PAY-001 | bb-go | Transport + social-sign of JCS PaymentRequestV1 and status events; must match WAL-002 golden hashes; wallet-free **and rate-free** daemon remains; no coin libraries | Broker IPC; spend; product wallet HTTP; `/ob/exchangerates`; rate fetch |
| 7 | BBD-WAL-006 | bb-desktop | ZEC librustzcash adapter on recorded compact-block fixtures / regtest; Orchard-protocol UA receive; v6 PCZT prepare with Ironwood bundle; restored Orchard-pool fail-closed; no broadcast | Mainnet; claiming hardware shielded support; auto ZIP-318 migration; zallet process |
| 8 | BBD-WAL-007 | bb-desktop | XMR adapter against fake then local loopback `monerod` (restricted **node** RPC ok) + **full** wallet-rpc; subaddresses; process split; no `--restricted-rpc` on wallet-rpc; no remote node | Public nodes; product HTTP; bundling policy until Q1 answered |
| 9 | BBD-WAL-008 | bb-desktop | Hardware probe table, fake device mismatch tests, Keystone PCZT path behind caps; real-device ticket gated | Unverified `can_sign_ironwood=true` or `can_tx_v6=true` |
| 10 | BBD-WAL-009 | bb-desktop | Internal broadcast + confirmation + concurrent locks on testnet/stagenet; post-sign cancel/expiry; crash_recovery reconfirm | Mainnet; auto-broadcast on crash; Electron broadcast method |
| 11 | BBD-WAL-010 | bb-desktop | UX: skippable onboard on broker surface, Pay on profile/post/conversation, states in §13, sanitized Electron preview, optional approximate fiat from RATE-001 | QR/address ordinary path; clipboard authority; Electron confirm/unlock; blocking Pay on missing quotes |
| 12 | BBD-WAL-011 | bb-desktop | Package broker sidecar and quote worker, SBOM, sandbox retained, binary pins, spawn-time hash verify | `--no-sandbox` as product instruction; listening wallet endpoints; embedded rate API keys |
| 13 | BBD-WAL-012 | bb-desktop | Mainnet gate after capability evidence | Skipping verify, remote XMR node, transparent private label, silent Orchard migration |

Routing: WAL-002–011 and RATE-001 are architecture-sensitive (Grok Build
or as reviewer assigns). Mechanical schema codecs after 002 may be Spark
only if this review is accepted and the ticket forbids semantic invention.
Codex Luna integrates and runs commands; it does not author tests. Luna
stays stopped until this corrected source is accepted.

## 17. Open questions that require owner choice

Engineering must not fill these silently.

**Q1. XMR wallet-rpc distribution.** Bundle a pinned `monero-wallet-rpc`
binary in the desktop package, or detect a user-installed official binary by
hash, or require the user to start wallet-rpc themselves? Node (`monerod`)
stays user-controlled in all options.

**Q2. Built-in ZEC light endpoint.** Leave lightwalletd URL empty until the
user sets one; ship a configurable default list; or later bundle/run a local
compact-block source. This review rejects a mandatory centralized default.

**Q3. IP privacy for ZEC compact-block download.** Require Tor/proxy, offer
it, or document the leak and defer.

**Q4. Unbound payment requests.** v1 recommendation is payer-bound requests
only (empty `payer_peer_id` is `SCHEMA`). Owner may allow "anyone may pay
this" requests; that widens replay and substitution surface.

**Q5. Transparent ZEC shielding.** v1 social Pay never spends or receives
transparent as private. Owner may later want an explicit shield flow; it
needs its own ticket and must not be called "private Pay."

**Q6. Software hot wallet in the first user-visible release.** Owner prefers
hardware. This review still specifies software accounts as first-class
because that was an owner decision. Owner may still choose to disable
software spend in packaged builds until hardware probes exist.

**Q7. Local full node for ZEC.** Built-in path is light client. Owner may
later want optional `zebrad` local submit; not required to start.

**Q8. Social paid-receipt.** Optional on-chain-linked social receipt after
confirm; privacy implications (linking peer ID to tx) need an owner yes/no.

**Q9. Default networks in development vs packaged unsigned builds.** This
review requires non-mainnet until BBD-WAL-012. Owner should confirm packaged
unsigned test artifacts stay testnet/stagenet.

**Q10. Broker native toolkit — RESOLVED 2026-08-30.** The owner selected a
minimal in-process native window in the Rust sidecar for unlock, backup/restore,
and payment confirmation. The broker may invoke a native OS file picker for a
backup path, but Electron does not mediate it and a separate OS credential agent
is not the v1 unlock design. Hardware confirmation remains on the device when
supported. WAL-004 implements only the custody/lock/backup portion; later
payment tickets reuse the same broker-owned surface and may not move Confirm
into Electron.

**Q11. Quote providers and quote currency.** A later provider ticket may
evaluate CoinPaprika, CoinGecko public, and direct Kraken market data
against current terms and availability. This review does **not** promise
that any of those, or any ZEC/XMR pair, will remain available. Owner
chooses which sources may be pinned and whether v1 display is USD-only
or a short ISO 4217 allowlist.

**Q12. Aggregation spread and median N.** This review requires absent-not-
zero, visible disagreement, and median only when enough fresh independent
providers agree within a bound. Exact N and spread are RATE-001 / owner
choice. Until then, show one labeled source or "fiat estimate
unavailable."

**Q13. Quote worker in packaged builds.** Default-on with fail-open to
"fiat estimate unavailable," default-off until RATE-001 is accepted, or
user opt-in. None of these may make Pay depend on quotes.

**Q14. Fiat-origin payment requests (“request $10”).** v1 recommendation
is coin-denominated requests only. Owner may later approve a ticket that
converts locally once before signing and records non-authoritative quote
provenance. Engineering must not add that schema in WAL-002 or RATE-001.

Questions **not** reopened: dual-coin from the start; shielded ZEC as the
built-in wallet; local-only XMR node; wallet-free `bb-go`; no marketplace;
Ledger/Trezor first-class as capabilities; no QR for ordinary Pay; NU6.3
activation facts; full vs restricted XMR wallet RPC; inherited anonymous
pipes; broker-owned confirm; rates as optional untrusted presentation;
rejection of the inherited OB1 ticker path.

## 18. First implementation slice (cannot create or move funds)

**Proposed ticket BBD-WAL-002** is the only slice that should follow
acceptance of this review.

In scope:

- Versioned JSON envelope and length-prefixed framer as pure tests over
  buffers (no spawned broker required).
- `Asset`, `Network`, `AmountAtomic`, `CapabilitySet` including v6, Orchard,
  Ironwood receive/spend, `pczt_version`, and
  `can_migrate_orchard_to_ironwood`.
- `PaymentRequestV1`, `PaymentStatusEventV1`, `ReviewImageV1` JCS codecs,
  domain separators, `intent_hash`, committed golden vectors.
- Intent state machine with `prepared`, `awaiting_confirm`, `signing`,
  `signed_unverified`, `verified`, `broadcasting`, `cancelled`, `expired`,
  `failed`, and `crash_recovery`.
- Fake ZEC and fake XMR adapters that **cannot** serialize a broadcastable
  mainnet transaction and that refuse `broadcast` in all cases
  (`CAPABILITY_MISSING` / `UNAVAILABLE`).
- Fake software, hardware, and watch-only signers exercising the matrix,
  including Trezor-transparent-not-private, NU6.3 mismatch, and restored
  Orchard-pool fail-closed.
- Secret canaries and log-field allowlist tests.
- Independent golden fixtures for amounts, replay, expiry, cancellation,
  unknown fields, post-sign destination mutation (`INTENT_MISMATCH`),
  crash_recovery non-broadcast, impossible `TimestampV1` dates, and
  prohibited bidi/format-control memos.
- Negative rate contract: `PaymentRequestV1`, `ReviewImageV1`, and Pay
  eligibility fixtures succeed with **no** rate object. Absence is valid.

Out of scope for the slice:

- Electron UI or preload (BBD-WAL-003)
- Rust toolchain installation or `wallet-broker/` crate
- Coin dependencies (librustzcash, Monero binaries)
- Real keys, USB, HID, PC/SC
- `monerod`, `monero-wallet-rpc`, lightwalletd, zebrad, zcashd, zallet
- Network sockets other than in-process pipes/fakes
- Git, packaging, SBOM regeneration, mainnet
- Quote worker, provider HTTP, `RateSnapshotV1` implementation
  (BBD-RATE-001)

Acceptance idea (commands to be named by the implementation ticket, not run
here): Node tests over fixtures with the existing desktop toolchain, red then
green, with falsification of verify-on-sign, of private-labeling transparent
ZEC, of confirm-before-fee, and of auto-broadcast after crash.

This slice still **shapes** both coins and all three account types: any
implementation that only models "a ZEC hot wallet" fails the contract tests.

## 19. Decision register (principal)

| Topic | Decision |
| --- | --- |
| Where the broker lives | `bb-desktop` Rust native sidecar; not `bb-go`, not renderer, not inherited OB wallet, not a Go sidecar, not a zallet process |
| Why Rust | Maintained librustzcash/PCZT/v6/Ironwood stack in-process; secrets out of V8/npm; toolchain cost accepted |
| Spend authority | Wallet broker + hardware devices; never Electron (including main-owned HTML), never `bb-go`, never product/generic HTTP |
| Authorization surface | Owner-selected minimal native window runs in the Rust broker for onboard/unlock/backup and authoritative payment confirmation; a broker-invoked OS file picker may select backup paths; no v1 OS credential agent; hardware confirms independently on device; Electron may preview and cancel only |
| IPC | Inherited anonymous bidirectional pipes/handles; packaged-binary verify; transcript bound to both pids+nonces; no secret in argv/env; protocol pipe ≠ diagnostics; no UDS/named-pipe/TCP/HTTP listener |
| Confirm / broadcast | Prepare exact ReviewImage (fee + bound) before confirm; one broker confirm authorizes sign→verify→broadcast; cancel/expiry rechecked after sign and before broadcast; crash recovery reconfirms; no Electron `intent.broadcast` |
| Canonical encoding | RFC 8785 JCS + domain-separated SHA-256 for PaymentRequestV1, status events, and intent_hash; closed schema; fail-closed unknown fields; TimestampV1 **shape plus strict Gregorian calendar, year 2020–2100, round-trip equality**; NFC memo; **explicit** bidi formatting/isolate and listed format-control rejection (not implied by C0/C1); independent Go/Rust/Node golden hashes |
| ZEC | librustzcash/PCZT, Orchard-protocol UA (not an Ironwood receiver type), v6 + Ironwood for new spends, light client, restored Orchard-pool fail-closed pending consented ZIP-318 ticket |
| XMR | User `monerod` node RPC (restricted node RPC allowed) + separate **full** wallet-rpc (never `--restricted-rpc`), loopback random auth, subaddresses, no remote node; residual same-user malware limit honest |
| HTTP | Product/generic wallet HTTP forbidden; contained authenticated loopback XMR wallet RPC does not cross the broker boundary |
| Hardware | Capability table with live narrowing; separate v6/Orchard/Ironwood/PCZT flags; no unverified shielded-ZEC claims; Trezor transparent ≠ private; undisplayed fields stay host-trusting |
| Keys | Social / ZEC / XMR separated; encrypted at rest; no seed/passphrase/backup to renderer/logs/Electron |
| Pay protocol | Signed JCS, payer-bound, exact atomic amount, fresh receiver, nonce+expiry, immutable request, separate signed status events; v1 coin-denominated only |
| Exchange rates | Optional untrusted presentation; separate least-privileged quote worker; no fetch in `bb-go`, renderer, or broker spend core; unit price then local decimal conversion; stale/invalid absent not zero; disagreement cannot block Pay; no OB1 ticker path, no P2P oracle, no embedded API key, no mandatory BitBook proxy; no provider promised |
| First code | Offline fake dual-coin contract, golden vectors, corrected state/capability model; cannot construct or move funds; no Electron/Rust/coins/devices; **rates not mandatory**; quote worker is BBD-RATE-001, not WAL-002 |

End of BBD-WAL-001 architecture review (Correction 02).
