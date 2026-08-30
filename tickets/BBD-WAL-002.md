# BBD-WAL-002 — Offline Dual-Coin Wallet Reference Contract

Status: CORRECTION TEST SOURCE ACCEPTED — EXPECTED RED AUTHORIZED

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
transaction change is authorized in the test-source phase. Sol may use read-only shell
commands solely to read the exact required repository documents/current tests and to
report final line counts and SHA-256 hashes for the four authorized paths. It must not
run Node, tests, builds, installs, formatters, scanners, Git, network, wallet, node,
subprocess, hardware, or device commands, and must stop after writing the four paths.

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

## Reviewer source decision — Correction 01

The initial Sol test drop is not accepted. It contains the four authorized paths and no
execution or scope violation, but reviewer XHigh inspection found blockers before the
expected-red phase:

1. The malformed-UTF-8 signed-object case is structurally incomplete. A decoder that
   replaces invalid bytes can still return `SCHEMA` for missing fields, so the assertion
   does not prove strict UTF-8 rejection.
2. `golden-v1.json` contains only three positive vectors. The architecture requires
   durable, fixture-driven adversarial vectors so later Go and Rust implementations
   consume the same invalid inputs and classifications rather than copying Node test
   construction logic.
3. Status/review schema coverage does not systematically exercise every missing field,
   declared field type, or timestamp validation, and it never directly proves a valid
   XMR request plus `ReviewImageV1` pair.
4. Explicit `machine.cancel()` state transitions do not prove that durable cancellation
   is re-read after signing and immediately before broadcast. A mutable injected request
   status must win at both boundaries without a broadcast call.
5. Crash recovery re-confirms an unchanged artifact only. It must also revalidate a
   recovered mutated artifact to `INTENT_MISMATCH`, and cancellation/expiry must still
   win during recovery.
6. The broadcasting-crash test uses `deferBroadcast` to make fake `broadcast()` return
   `ok: true`. That contradicts the absolute ticket invariant that every fake adapter
   broadcast returns `UNAVAILABLE` or `CAPABILITY_MISSING`. The state may be restored or
   injected as already-broadcasting for crash handling, but no fake broadcast success is
   permitted and its call count must remain zero in that setup.
7. Package/security tests require `test:wallet` but do not require top-level `npm test`
   and the build syntax contract to cover the wallet reference modules. They must also
   reject `node:` and dynamic-import forms of forbidden process/network/device modules,
   not just a few literal `require()` spellings.

The exact bounded correction contract is
[`CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_01.md`](../docs/handoff/CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_01.md).
Sol may edit only the same four test paths, may use only the previously authorized
read-only inspection/measurement commands, and must run nothing. Luna remains stopped.

## Reviewer source decision — Correction 02

Correction 01 resolves its seven named blockers, but the corrected source is not yet
accepted. Second XHigh inspection found these focused defects:

1. `test/securityPolicy.node.js` adds the wallet command assertion inside the pre-existing
   `routine social check keeps offline syntax and Node tests only` test. This violates the
   explicit red contract that all 50 inherited policy assertions remain unchanged and
   execute before appended wallet expectations. The assertion is already covered by an
   appended wallet test and must be removed from the inherited test.
2. The field-type table does not reject JSON integers `0`, `2`, or `-1` for `v`, even
   though every signed schema requires integer `1` only. Review pool tests also omit an
   empty ZEC pool list and unknown/mixed pool entries.
3. The common schema is not proven across every enumerated network. Offline decoding
   must accept ZEC mainnet/testnet/regtest and XMR mainnet/stagenet/testnet when each is
   paired with the correct asset/receiver kind, while rejecting cross-asset pairs.
   Decoding is not transaction construction or mainnet use.
4. Status and review tests cover types but not enough lexical identifiers. They must
   reject malformed event/request/nonces, payment/memo hashes, blank intent/prepared/
   account/peer/receiver values, and invalid review asset/network/receiver/change/tx/pool
   combinations. Include positive paid-with-nonempty-`tx_ref` and expired-with-empty-
   `tx_ref` status events so the rule cannot be implemented as “only cancelled works.”
5. The forbidden-module tests enumerate literal imports but permit computed imports such
   as `require(name)` or `import(name)`. The reference modules need a fail-closed module
   allowlist: only literal `crypto`, `node:crypto`, `buffer`, or `node:buffer` imports may
   appear; computed/nonliteral imports and other static/dynamic imports are rejected.

The exact bounded correction contract is
[`CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_02.md`](../docs/handoff/CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_02.md).
Sol may edit only the same four test paths, may use only the previously authorized
read-only inspection/measurement commands, and must run nothing. Luna remains stopped.

## Reviewer source decision — Correction 03

Correction 02 resolves all five of its source defects, but its reviewer-specified import
allowlist is over-constrained: it rejects all relative imports, including the imports
`wallet-contract/index.js` necessarily uses to export its five sibling modules. Forcing
duplication or an empty façade would make the green contract structurally unsound.

Permit literal relative imports only for the exact six sibling basenames
`./canonical[.js]`, `./framing[.js]`, `./model[.js]`, `./state-machine[.js]`,
`./fakes[.js]`, and `./index[.js]`, in addition to crypto/buffer. Continue rejecting
parent traversal, absolute paths, other relative files, packages, built-ins, computed
specifiers, filesystem/network/device capabilities, fetch, and WebSocket.

The exact bounded correction contract is
[`CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_03.md`](../docs/handoff/CODEX_SOL_BBD_WAL_002_TESTS_CORRECTION_03.md).
Sol may edit only `test/electronSecurity.node.js` and `test/securityPolicy.node.js`, may
use only the previously authorized read-only inspection/measurement commands, and must
run nothing. The fixture and wallet test must remain byte-identical. Luna remains
stopped.

## Reviewer acceptance — test source

Correction 03 test source is accepted for expected-red execution at these exact values:

- `test/fixtures/wallet-contract/golden-v1.json`: 231 lines,
  SHA-256 `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- `test/walletContract.node.js`: 1,344 lines,
  SHA-256 `a814bf327345dbdde276343fc40ff6fd8ca770569b12afc0860c664a8c99b7d9`
- `test/electronSecurity.node.js`: 639 lines,
  SHA-256 `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- `test/securityPolicy.node.js`: 1,396 lines,
  SHA-256 `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`

The accepted inventory is 3 positive and 19 invalid fixture vectors, 38 wallet tests,
14 Electron security tests (13 inherited plus one appended), and 54 security-policy
tests (50 inherited plus four appended). The corrected source:

- preflights and independently hashes fixtures before importing the absent production
  module;
- closes all three signed schemas across types, lexical fields, Unicode, Gregorian time,
  asset/network/receiver relations, fee/pool rules, and all six network enums;
- proves incremental frame boundaries, both coins, every account kind, post-sign
  cancellation/expiry, intent mismatch, crash recovery, account locking, inert
  broadcast, secret canaries, and rate absence;
- preserves the inherited security tests and appends fail-closed package/CI/source
  expectations; and
- permits only crypto/buffer and exact pure sibling imports while rejecting external,
  computed, parent, filesystem, network, Electron, device, and worker capabilities.

Jr Dev — Codex Luna is authorized only by
[`CODEX_LUNA_BBD_WAL_002_RED.md`](../docs/handoff/CODEX_LUNA_BBD_WAL_002_RED.md) to verify
the hashes, run the three expected-red Node commands, write the named red-evidence file,
and commit/push that evidence file alone if every failure matches. Luna must not modify,
stage, commit, or push the failing test source. Production remains unauthorized.

## Reviewer acceptance — expected red

Jr Dev — Codex Luna produced accepted red evidence in commit
`3a60b33c2b4c2f8355007fbd3535066cd0a0d1c6`. Reviewer independently verified that the
commit contains only `docs/testing/BBD-WAL-002-RED-EVIDENCE.md`, at 31 lines and
SHA-256 `f8b33e5f5c510188676deee11a073d6c053ae3d9b4de9279a3cfca25a68e2819`, and that
`HEAD` equals `origin/master`.

The three commands failed for the intended reasons only: late `MODULE_NOT_FOUND` after
fixture preflight; 13 inherited Electron assertions green before one missing-source red;
and 50 inherited policy assertions green before four absent-wiring reds. This is accepted
evidence that production—not the tests, fixtures, dependencies, or inherited behavior—is
missing.

Temporary Sr Dev — Codex Sol is now authorized only by
[`CODEX_SOL_BBD_WAL_002_PRODUCTION.md`](../docs/handoff/CODEX_SOL_BBD_WAL_002_PRODUCTION.md)
to author the bounded green source. Sol must not run anything or modify the accepted
tests. Codex Luna remains stopped until reviewer source acceptance.

## Reviewer source decision — production drop and Corrections 04–05

The first production drop is bounded correctly but rejected before execution. XHigh
inspection found untested fail-open behavior: prepared reviews were not bound back to the
selected request/account; account/adapter/signer substitution and loose synthetic
protocol pins could pass; crash recovery released or failed to reacquire account locks
and lacked required terminal/restart behavior; injected dependency exceptions could
escape with locks held; and the structured-log sanitizer validated field names but not
their values.

Corrections 04 and 05 add seven behavioral tests without changing the fixture or either
security suite. Correction 05 closes the final two non-vacuity gaps by requiring a
repeated recovery crash to be a successful snapshot-preserving no-op and proving lock
release after an injected broadcast exception.

The corrected source is accepted for expected-red execution at:

- `test/walletContract.node.js`: 1,697 lines, SHA-256
  `3e51281d16da7eec4a178eeb799ec23e2854206a096ed741cba920fc35825ee9`

The accepted inventory is now 45 wallet tests: the prior 38 unchanged tests plus seven
correction tests. Fixture, Electron-security test, security-policy test, and lockfile
hashes remain the previously accepted values. Jr Dev — Codex Luna is authorized only by
[`CODEX_LUNA_BBD_WAL_002_CORRECTION_RED.md`](../docs/handoff/CODEX_LUNA_BBD_WAL_002_CORRECTION_RED.md)
to run the single wallet expected-red command and commit/push the named evidence file
alone. Production correction remains unauthorized.
