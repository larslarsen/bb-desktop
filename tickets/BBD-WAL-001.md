# BBD-WAL-001 — Dual-Coin Wallet Architecture Review

Status: SOURCE ACCEPTED — LUNA INTEGRATION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Governance baseline: `20c7f7e7e71a5d98c1e236fea9d7d3dc1eeffb8a`

## Owner decisions

- BitBook will offer both ZEC and XMR from the beginning rather than choosing one coin.
- ZEC is the built-in basic wallet and must use the maintained shielded protocol, not the
  inherited transparent-only OpenBazaar implementation.
- XMR is optional and connects to a user-controlled local `monerod`; the design must
  account for the separate Monero wallet process required to hold/view/sign.
- Wallet use remains optional, but payments are a first-class native social feature with
  no address copying or QR requirement for ordinary peer payments.
- Software, hardware-backed, and watch-only accounts are first-class. The owner has
  Ledger and Trezor devices and prefers hardware signing.
- BitBook remains a distributed social network. Marketplace, listing, order, escrow,
  dispute, exchange, custodial service, and centralized payment-provider behavior are
  out of scope.
- The legacy `go-ipfs` repository is deprecated and out of scope.

## Objective

Produce a threat-model-led architecture that can be converted into small, test-first
implementation tickets without putting spend authority in Electron or the social daemon.
The review must define the local wallet-broker boundary, coin adapters, signer contract,
payment-intent lifecycle, recovery model, hardware capability model, process isolation,
and staged verification plan.

This ticket does not implement a wallet or payment feature.

## Required architecture decisions

The review must make a concrete recommendation, identify rejected alternatives, and
state uncertainty for each of these topics:

1. **Repository and process boundary.** Decide whether a dedicated wallet broker belongs
   in this desktop repository, how it is launched, and why `../bb-go/modern` remains
   wallet-free. Do not propose reviving the legacy daemon or renderer.
2. **IPC boundary.** Prefer private child-process IPC without a listening wallet HTTP
   endpoint. Define message framing, request correlation, cancellation, timeouts, size
   limits, schema/version negotiation, error normalization, crash/restart behavior, and
   prevention of renderer access to generic wallet commands.
3. **Account and signer model.** Define software, hardware-backed, and watch-only
   accounts; device-neutral capabilities; prepare/review/sign/verify/broadcast phases;
   and the invariant that signed output is revalidated against user-confirmed intent.
4. **Key custody and recovery.** Keep social identity, ZEC, and XMR secrets separate.
   Define encrypted-at-rest software secrets, hardware-derived accounts, backup/restore,
   lock/unlock, zeroization limits, logs, diagnostics, and explicit prohibitions on seed
   or spend-key exposure to renderer, HTTP, crash reports, or ordinary evidence.
5. **ZEC adapter.** Use current maintained Zcash components, Ironwood-capable shielded
   operation, shielded-only receiving by default, PCZT/external signing, light-client
   synchronization, network privacy, and explicit capability handling for Keystone,
   Ledger, and Trezor. Trezor transparent-only ZEC support must not be represented as a
   private account.
6. **XMR adapter.** Use maintained Monero software, a user-controlled local `monerod`,
   authenticated loopback `monero-wallet-rpc`, subaddresses, Ledger/Trezor signing,
   synchronization state, device disconnects, and process lifecycle. No public/remote
   node fallback is assumed without a later owner decision.
7. **Payment protocol.** Define a coin-agnostic, signed, replay-resistant payment request
   that can later be implemented in `../bb-go/modern`: payer/payee peer IDs, asset,
   network, exact atomic amount, fresh receiver, memo or purpose, nonce, creation and
   expiry, request ID, status, and cancellation. Never bind a permanent public payment
   address directly to the social profile when a fresh receiver is available.
8. **User experience.** Define wallet onboarding, hot versus hardware accounts, explicit
   network/asset/recipient/amount/fee confirmation, unavailable/syncing/locked states,
   and the native Pay flow from a profile, post, or conversation.
9. **Testing and security evidence.** Apply `TESTING.md`: independent canonical fixtures,
   parser fuzzing, property tests, boundary tests, failure injection, compound failures,
   fake signer/device/node implementations, testnet/stagenet only, mutation/falsification,
   secret-log canaries, crash recovery, dependency scanning, SBOMs, and package-content
   checks. Define hard mainnet and real-device gates.
10. **Ticket decomposition.** Produce ordered, independently reviewable tickets. Both
    coins and all three account types must shape the common contract from the first
    implementation ticket, even if the adapters reach usable depth sequentially.

## Required threat model

At minimum cover a compromised renderer, malicious social peer, malicious payment
request, hostile or stale chain service, compromised remote site/content, forged device
response, device disconnect, wallet-broker crash, rollback/corrupt state, concurrent
spends, replay, address substitution, decimal/atomic-unit confusion, wrong network,
malicious dependency/update, local unprivileged process, logs/crash dumps, clipboard,
and accidental transparent ZEC downgrade.

Clearly separate what a hardware wallet protects from what it does not protect. The host
still owns chain scanning, payment-request interpretation, fee calculation, transaction
construction, display, and broadcast unless the selected device independently verifies a
field.

## Authorized path

Grok Build may author only:

- `docs/architecture/BBD-WAL-001-REVIEW.md`

The reviewer may author only the ticket and handoff governance paths named in this
baseline. Codex Luna may later update `docs/handoff/CURRENT_TASK.md` and commit the
reviewer-accepted architecture document.

No production, test, dependency, lockfile, workflow, package, generated artifact, other
repository, GitHub setting, hardware, wallet, node, network, or secret change is
authorized.

## Execution and safety rules

- Grok runs in one foreground session and writes its complete reasoning to the authorized
  document; ephemeral chat is not authoritative.
- Grok runs no tests, builds, installs, formatters, scanners, Git, GitHub, wallet, node,
  device, USB, HID, PC/SC, network, or package commands.
- Do not use `/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable
  targets, or unresolved paths.
- Do not inspect, request, display, or invent a seed, spend key, PIN, device recovery
  phrase, wallet file, or real address.
- Mainnet transaction construction, signing, and broadcast are forbidden.

## Acceptance criteria

- The review is specific enough to turn into bounded test-first tickets without an
  implementation actor making architecture decisions.
- Both ZEC and XMR shape the first common contract.
- Ledger and Trezor are represented through capabilities, not vendor assumptions.
- The document does not claim current shielded-ZEC hardware support without a verified
  device/protocol capability.
- No spend authority enters Electron, the renderer, `bb-go`, product/generic HTTP, logs,
  or evidence. The authenticated loopback `monero-wallet-rpc` transport is an internal
  XMR-adapter exception and must not cross the wallet-broker boundary.
- The social daemon stays wallet-free and the old OpenBazaar wallet is rejected as an
  implementation base.
- The first implementation slice is offline, deterministic, credential-free, and cannot
  create or move funds.

## Reviewer source decision — Correction 01

Grok's initial source at 1,245 lines and SHA-256
`d9c0107ee4b7381cf5c70240eeae1d041791d0eaa2f926f9c2abf79e63c3a816` is not accepted.
It has a sound dual-coin direction, but the following blockers must be corrected before
Codex Luna may integrate it:

1. Ironwood is incorrectly described as NU7-class and future/approximate. It activated
   as Zcash NU6.3 on 2026-07-28 at mainnet height 3,428,143. The old Orchard pool is now
   restricted.
2. The document invents an “Ironwood receiver set.” Ironwood is a distinct pool using
   the Orchard protocol receiver and incoming viewing key. Pool, transaction-v6,
   migration, and signer capabilities must be modeled explicitly without inventing a
   new address kind.
3. The XMR spending design ambiguously enables `monero-wallet-rpc --restricted-rpc`.
   That mode is view-only. Spending requires full authenticated wallet RPC bound only to
   loopback and contained behind the broker; a restricted local `monerod` RPC is a
   separate matter.
4. The payment request and `intent_hash` have no single normative canonical encoding,
   cite the wrong section, and leave timestamps/status mutation underspecified.
5. The state diagram prepares after confirmation even though the confirmation UI must
   show the prepared fee, and it conflicts with the separate broadcast method and the
   one-button UX. Cancelled, signed-but-unverified, crash, and post-sign cancellation
   behavior are not unambiguous.
6. The proposed Electron confirm/unlock window can transmit spend authorization and a
   software-wallet passphrase through Electron, contradicting this ticket's trust
   boundary. A broker-owned native authorization/secret-entry surface (or equivalently
   isolated broker-controlled OS credential agent) must be the authority. Electron may
   request review, show a non-authoritative preview, receive sanitized state, and cancel;
   it cannot confirm, unlock, export, sign, or broadcast.
7. A listening UDS/named-pipe endpoint is unnecessary for a single supervised child and
   weakens the local-process boundary. V1 must use inherited anonymous bidirectional
   child pipes/handles, exact packaged-binary verification, and transcript/session
   binding with no secret in argv or environment. Any later listening endpoint requires
   a separate threat review.

The exact correction contract is
[`GROK_BUILD_BBD_WAL_001_CORRECTION_01.md`](../docs/handoff/GROK_BUILD_BBD_WAL_001_CORRECTION_01.md).
Grok may edit only the same architecture document and must run nothing. Luna remains
stopped until the reviewer accepts the corrected source and records its exact hash.

## Reviewer source decision — Correction 02

Correction 01 source at 1,776 lines and SHA-256
`6389f4b920c594f97e3d2eb8048d308ab563c2cce23f345f1279982d57283aa4` resolves the seven
original blockers. Before integration, the owner identified one omitted inherited
dependency: exchange-rate display. Reviewer trace established:

- the inherited desktop polls local daemon endpoint `/ob/exchangerates/<coin>` every five
  minutes;
- the daemon delegates to the inherited multiwallet exchange-rate providers;
- the ZEC provider tries `https://ticker.openbazaar.org/api` first, then historical
  Bittrex, Bitfinex, Poloniex, and Kraken endpoints;
- OB1 released MIT-licensed `OpenBazaar/tickerproxy`, but it is a 2018-era
  BitcoinAverage-backed caching/S3 service and is not a current production foundation.

The architecture must include a replacement before acceptance. It must not revive the
daemon wallet endpoint, put rate fetching in the wallet broker's trusted core, trust P2P
peers as price oracles, or make wallet availability depend on fiat data. It must also fix
the remaining Unicode/timestamp validation imprecision found during reviewer inspection.

The exact bounded contract is
[`GROK_BUILD_BBD_WAL_001_CORRECTION_02.md`](../docs/handoff/GROK_BUILD_BBD_WAL_001_CORRECTION_02.md).
Grok may edit only the architecture document and must run nothing. Luna remains stopped.

## Reviewer acceptance — architecture source

Correction 02 source is accepted at exactly 2,271 lines and SHA-256
`aae487b169689f310b222640427c1cdae62850d39ebb0243e29f10568d6fcb3f`.

Reviewer verified that the final architecture:

- keeps the social daemon and Electron outside spend authorization;
- uses a Rust broker sidecar with broker-owned native authorization and inherited
  anonymous child pipes;
- models NU6.3/Ironwood, ZEC hardware capability drift, full authenticated XMR wallet
  RPC, and Ledger/Trezor without unverified privacy claims;
- defines one canonical signed request, a prepare-before-confirm state machine, and
  crash/cancel/expiry behavior that cannot auto-broadcast;
- makes exchange rates optional, untrusted presentation from a separate quote worker,
  rejects the inherited OB1 chain, and keeps exact atomic amounts authoritative;
- preserves an offline, credential-free BBD-WAL-002 first slice that cannot construct or
  move funds.

Open questions Q1–Q14 are intentionally deferred to their named bounded tickets; none is
required to integrate this design document. Codex Luna is authorized only by
[`CODEX_LUNA_BBD_WAL_001.md`](../docs/handoff/CODEX_LUNA_BBD_WAL_001.md) to verify and
commit the exact accepted document. No implementation, dependency, test, wallet, node,
device, provider call, or other-repository change is authorized.
