# BBD-WAL-002 — Offline Dual-Coin Wallet Reference Contract

Status: AUTHORIZED — TEST SOURCE ONLY

Reviewer: Lead Engineer/Reviewer — Codex

Test source actor: Temporary Sr Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Architecture baseline: `9dcade2cf4b0fc298733657148fdae002638ff48`

Owner-approved actor substitution: Grok Build returned a quota-exhausted 402 before
reading or editing any source. On 2026-08-30 the owner authorized Codex Sol at High to
author this bounded test-only drop, with Lead Engineer/Reviewer — Codex at XHigh for
independent review. This ticket-specific substitution does not change the standing roles
or give Sol execution, integration, evidence, Git, or production authority.

## Objective

Implement the first executable slice of BBD-WAL-001 as an offline,
dependency-free Node reference contract. Tests must lead. The slice defines canonical
payment objects, buffer framing, account capabilities, and the prepare-before-confirm
intent lifecycle for both ZEC and XMR without possessing a key, constructing a real
transaction, contacting a wallet or node, or moving funds.

This ticket is a safety boundary, not a usable wallet. It must remain impossible to
broadcast through every fake adapter.

## Fixed product and architecture invariants

- Built-in ZEC and optional local-node XMR shape the common contract from the first test.
- `software`, `hardware_backed`, and `watch_only` are first-class account kinds.
- ZEC native Pay means current NU6.3 transaction-v6/Ironwood spending. An
  Orchard-protocol receiver is not an invented Ironwood receiver type. Restored Orchard
  funds require an explicit later migration path.
- Transparent-only Trezor ZEC is not private Pay. Ledger shielded ZEC stays unavailable
  until its exact current capability is positively verified; a vendor name is never a
  capability. XMR hardware accounts are capability-driven and never fall back to a
  software signer.
- Exact atomic ZEC/XMR amounts are authoritative. Rates, fiat values, quote sources, and
  quote provenance are absent from signed request and review objects. Pay must work when
  every rate source is unavailable.
- Prepare produces an exact fee and `ReviewImageV1` before the broker-owned confirmation.
  Verification binds the signed result to the confirmed intent. Cancellation and expiry
  are rechecked after signing and immediately before broadcast.
- Electron, `bb-go`, network services, devices, real wallet processes, keys, and real
  transaction encodings are outside this slice.

## Current authorization: test source only

Codex Sol may create or edit only:

- `test/fixtures/wallet-contract/golden-v1.json`
- `test/walletContract.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

The first two paths define the reference contract and independent golden oracle. Changes
to the existing security tests must only establish fail-closed expectations for the
later maintained `wallet-contract/` source, package scripts, and workflow path filters.
New expectations must run after the pre-existing assertions so the red result does not
hide an inherited regression.

No production source, dependency, lockfile, package script, workflow, checker, evidence,
Git, GitHub, generated artifact, other repository, wallet, device, network, node, or
transaction change is authorized in the test-source phase. Sol must stop after writing
the four paths and must not execute anything.

## Reserved production paths — not yet authorized

After reviewer acceptance of the test source and Luna's expected-red evidence, a later
handoff may authorize only:

- `wallet-contract/canonical.js`
- `wallet-contract/framing.js`
- `wallet-contract/model.js`
- `wallet-contract/state-machine.js`
- `wallet-contract/fakes.js`
- `wallet-contract/index.js`
- `package.json`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`
- `scripts/security-policy.js`
- `docs/handoff/CURRENT_TASK.md`
- a ticket-named implementation evidence document

`package-lock.json` must remain byte-identical. No package may be added. Merely listing a
path here does not authorize production work.

## Required contract tests

### 1. Canonical signed objects and independent fixtures

Test closed-schema, strict-UTF-8 parsing and RFC 8785/JCS canonicalization for
`PaymentRequestV1`, `PaymentStatusEventV1`, and `ReviewImageV1`. Hash
`domain-separator || canonical-UTF8` with SHA-256. The fixture must store the input
object, exact canonical string, expected lowercase digest, and classification.

Required domain separators include their final newline:

```text
bitbook-payment-request-v1\n
bitbook-payment-status-v1\n
bitbook-intent-hash-v1\n
```

The positive golden request is:

```json
{"v":1,"request_id":"00112233445566778899aabbccddeeff","payer_peer_id":"12D3KooWPayer","payee_peer_id":"12D3KooWPayee","asset":"ZEC","network":"zec-testnet","amount_atomic":"100000000","receiver":"u1testreceiver","receiver_kind":"zec-ua-orchard-protocol","memo":"coffee","nonce":"ffeeddccbbaa99887766554433221100","created_at":"2026-08-30T12:00:00Z","expires_at":"2026-08-30T12:15:00Z"}
```

Its exact canonical string is:

```text
{"amount_atomic":"100000000","asset":"ZEC","created_at":"2026-08-30T12:00:00Z","expires_at":"2026-08-30T12:15:00Z","memo":"coffee","network":"zec-testnet","nonce":"ffeeddccbbaa99887766554433221100","payee_peer_id":"12D3KooWPayee","payer_peer_id":"12D3KooWPayer","receiver":"u1testreceiver","receiver_kind":"zec-ua-orchard-protocol","request_id":"00112233445566778899aabbccddeeff","v":1}
```

Its digest is
`c21d03fcacab9128ce5d058b6b3b9b95adbf22de222df6d70d92390361ca60dc`.

The positive cancellation event canonical string is:

```text
{"at":"2026-08-30T12:05:00Z","event_id":"11112222333344445555666677778888","nonce":"9999aaaabbbbccccddddeeeeffff0000","request_id":"00112233445566778899aabbccddeeff","status":"cancelled","tx_ref":"","v":1}
```

Its digest is
`9e4b7b6ef01506b93aa76ef1e609a90a70d6fb491dd996e0660881de5c38c3aa`.

The positive review image canonical string is:

```text
{"account_id":"account-test-1","amount_atomic":"100000000","asset":"ZEC","change_policy":"shielded_internal","expires_at":"2026-08-30T12:15:00Z","fee_atomic":"10000","fee_bound_atomic":"12000","intent_id":"intent-test-1","memo_hash":"37290d74ac4d186e3a8e5785d259d2ec04fac91ae28092e7620ec8bc99e830aa","network":"zec-testnet","payee_peer_id":"12D3KooWPayee","payer_peer_id":"12D3KooWPayer","payment_request_hash":"c21d03fcacab9128ce5d058b6b3b9b95adbf22de222df6d70d92390361ca60dc","prepared_at":"2026-08-30T12:00:10Z","prepared_id":"prepared-test-1","receiver":"u1testreceiver","receiver_kind":"zec-ua-orchard-protocol","request_id":"00112233445566778899aabbccddeeff","tx_version":"6","v":1,"zec_pools":["ironwood"]}
```

Its intent digest is
`ad55816f327c002be813a29d41f9a7ae429782b6856a4d3bb2e6c498c6f9e3c0`.
The memo hash is SHA-256 of the UTF-8 bytes for `coffee`.

Adversarial coverage must include duplicate keys before ordinary JSON parsing; unknown
and missing keys; trailing bytes; BOM; malformed UTF-8; non-object roots; strings where
`v: 1` is required; floats, nulls, and booleans; invalid enum and asset/network/receiver
combinations; amount zero, leading zero, excessive digits, and wrong types; fee above
bound; invalid tx version or pool; a rate/fiat/quote field; and a status field added to a
request. Whitespace and key-order permutations of valid JSON must converge to the same
canonical bytes and digest.

Timestamp tests must enforce the exact UTC-second form, strict Gregorian dates, years
2020–2100 inclusive, and round-trip equality. Include valid `2024-02-29T12:00:00Z` and
reject at least February 30, 2026-02-29, April 31, hour 24, leap second, timezone offset,
fractional seconds, years 0000/2019/2101, and expiry not strictly after creation.

All signed strings reject malformed UTF-8, C0/C1, noncharacters, unpaired surrogates,
the bidi controls U+202A–U+202E and U+2066–U+2069, and the additional format controls
enumerated by BBD-WAL-001 §11.2. Memo must already be NFC and no more than 512 UTF-8
bytes. Identifiers, enums, peers, and receivers use their architecture-defined ASCII
subsets.

### 2. Incremental framing

Use a four-byte unsigned big-endian length prefix. Tests must cover a frame arriving one
byte at a time, multiple frames in one chunk, a split prefix, an empty control object,
and preservation of unread bytes. Control messages are limited to 64 KiB and the
absolute frame ceiling is 1 MiB. Test exactly-at-limit and limit-plus-one behavior.
Zero/invalid length, malformed UTF-8, invalid JSON, trailing JSON, and oversize input
must fail closed and permanently close that decoder; later bytes cannot revive it.

### 3. Capability matrix

Table-drive both assets across software, hardware-backed, and watch-only accounts.
Private ZEC Pay requires matching network/consensus, private receive, transaction v6,
Ironwood signing, and a supported PCZT. Orchard-only restored funds return
`MIGRATION_REQUIRED`. Transparent-only Trezor returns `CAPABILITY_MISSING` and is never
labelled private. An unverified Ledger/ZEC capability remains unavailable. XMR hardware
may be eligible only from explicit probed capabilities; disconnect never selects a
software signer. Watch-only can receive when capable but cannot spend. Absence of a rate
object must never affect eligibility.

### 4. Intent lifecycle and inert fakes

Tests must prove the named prepare-before-confirm states and effects in BBD-WAL-001 §5.2.
There is no confirm before a prepared review with exact fee and bound. Confirm binds the
intent hash. A post-sign destination, amount, network, fee, request, memo, or change
mutation returns `INTENT_MISMATCH` and records zero broadcast calls.

Cancellation and expiry must win in every allowed pre-broadcast state, including after
sign and from `verified`. Watch-only signing, device disconnect, signer failure,
capability failure, and concurrent prepare (`ACCOUNT_BUSY`) fail closed. Crashes from an
unsigned state abort. A complete signed artifact enters `crash_recovery`, requires fresh
broker confirmation, revalidates, and never automatically broadcasts. A crash while
broadcasting enters `unknown_needs_scan` and never blindly resubmits.

Fake ZEC and XMR adapters and fake software, hardware, and watch-only signers contain no
real transaction bytes, key material, address parser, RPC, socket, subprocess, USB/HID,
or device library. Fake adapter `broadcast` always returns `UNAVAILABLE` or
`CAPABILITY_MISSING`; no success path may claim funds moved.

### 5. Secret and rate negative contracts

Inject canaries shaped like seeds, spend keys, PINs, passphrases, raw transactions, and
receivers. Assert normalized logs contain only allowlisted identifiers/state/error codes
and none of the canaries. The test output itself must use synthetic labels, not realistic
secret material. Assert request/review schemas reject fiat, rate, provider, and quote
fields and that all ordinary operations work with no quote component.

## Required test structure

Use only Node built-ins and the repository's plain CommonJS assertion harness style.
Tests are offline, deterministic, credential-free, clock-injected, and leave no timer,
listener, child-process, file, or network resource behind. Do not weaken an assertion
based on platform, environment, or missing implementation.

The test may define the smallest stable CommonJS API it needs through
`require('../wallet-contract')`; that interface becomes the production contract for the
green phase. It must test observable values and adapter call counts rather than source
text or self-reported booleans.

## Red, green, falsification, and acceptance

Codex Luna alone runs commands.

Expected-red commands, in this order:

```text
node test/walletContract.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

The wallet test must fail because the required `wallet-contract` module does not exist,
not because of test syntax or a malformed fixture. The existing security suites must
first preserve all old passing assertions, then fail because the future maintained
wallet paths/scripts/workflow filters are absent.

Targeted green commands:

```text
node test/walletContract.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

Broader acceptance commands, only after production authorization:

```text
npm run build
npm test
node scripts/security-policy.js
npm audit --audit-level=low
```

Security acceptance blocks on any new dangerous maintained sink, workflow fail-open,
unpinned dependency change, lockfile change, audit regression, secret exposure, or
network/process/device capability. Routine WAL-002 does not build packages or regenerate
an SBOM; release SBOM/scanning remains enforced by the existing manual release workflow.

Required falsifications, applied temporarily and never committed:

1. bypass post-sign verification and prove an `INTENT_MISMATCH` mutation test fails;
2. mark transparent-only or unverified ZEC hardware private and prove the capability
   table fails; and
3. make a fake broadcast report success and prove the inert-adapter assertion fails.

## Acceptance criteria

- Test source was reviewer-inspected before any red execution or production source.
- Luna records the intended red causes without integrating a failing commit.
- The eventual green implementation uses no new dependency and cannot construct, sign,
  or broadcast a real transaction.
- Golden hashes match independent committed bytes, schemas are closed, and parsers fail
  closed at malicious input and framing boundaries.
- Both coins and all account kinds are materially exercised; rate absence is valid.
- Security policy treats the new reference modules as maintained source and CI runs the
  wallet contract on relevant changes.
- Falsifications fail for the claimed reason, are restored, and final acceptance is
  green with a clean worktree and unchanged lockfile.
