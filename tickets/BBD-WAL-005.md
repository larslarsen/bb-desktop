# BBD-WAL-005 — Pay Snapshot Gating and Sanitized Preview

Status: ACTIVE — TEST-SOURCE SYNTAX CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Test and production source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Hermes

Source baseline: `14a6818740a1da0641f88a1f91dea3346554dddd`

Architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` §§2.2–2.3, 4.3,
5.3–5.4, 6.1–6.2, 11.4–11.6, and 13.2–13.5

Cross-repository baseline: `../bb-go` accepted BBGO-PAY-001 production at
`6bbb0629cb0dfaca9958f9cac7d57216760630ae` and final evidence at
`801f5d55d80fe02c6eb512ff35f8c09acfd679af`. This ticket does not edit
`../bb-go`. `../go-ipfs` remains deprecated.

## Objective

Add the bounded, headless Pay presentation model between the accepted wallet contract and
the existing Electron wallet boundary. It must:

1. derive payer and payee-entry eligibility from a sanitized broker snapshot without
   granting authority to Electron;
2. expose fixed, non-secret capability/blocker labels;
3. keep transparent-only Zcash hardware and restored Orchard funds out of private Pay;
4. build the exact `receiver.fresh` parameters for an eligible payee account; and
5. reduce an in-flight intent to a non-authoritative Electron preview with Cancel as its
   only action.

This is state/view-model wiring, not the visible Pay screen. WAL-010 owns DOM composition.
This ticket does not create, confirm, sign, extract, broadcast, fetch rates, or move funds.

## Closed module contract

Production may add one CommonJS module, `wallet-pay/model.js`, exporting exactly:

```text
sanitizeWalletSnapshot(value)
derivePayView(snapshot, context)
buildPayeeReceiverParams(snapshot, context)
```

The module uses only deterministic data and Node built-ins. It performs no I/O and imports
no Electron, child-process, filesystem, network, rate, coin, hardware, or native module.

`social-main.js` and `wallet-broker/supervisor.js` must use the same exported sanitizer;
they must not retain independent wallet snapshot allowlists.

### Sanitized snapshot

The sanitizer accepts an untrusted plain-data candidate and always returns a fresh plain
object with only this closed shape:

```text
{
  v: 1,
  broker: "down" | "locked" | "ready" | "syncing" | "degraded",
  accounts: [
    {
      account_id: 32 lowercase hex,
      label: bounded safe string,
      asset: "ZEC" | "XMR",
      network: matching Network,
      kind: "software" | "hardware_backed" | "watch_only",
      privacy: "private" | "transparent_not_private" | "unknown",
      balance_atomic: canonical nonnegative u64 decimal string,
      restored_pool: "orchard" | "ironwood" | null,
      probed_at: bounded safe string | null,
      probe_source: "static_fixture" | "library" | "device_app" | null,
      capabilities: CapabilitySet,
      sync: { state: bounded safe enum, progress: finite number from 0 through 1 },
      device: {
        present: boolean,
        label: bounded safe string,
        verified_fields: array drawn from "amount", "recipient", "network", "fee"
      }
    }
  ],
  intent_preview: null | {
    intent_id: 32 lowercase hex,
    state: fixed intent-state enum,
    asset: "ZEC" | "XMR",
    network: matching Network,
    amount_atomic: canonical positive u64 decimal string,
    status_label: fixed string,
    can_cancel: boolean,
    error_code: stable wallet error code | null
  }
}
```

`CapabilitySet` retains only the fixed capability names in architecture §6.2. Capability
flags must be booleans. `consensus_branch`, `pczt_version`, and `tx_version_max` retain
only bounded strings or null. Unknown capability keys are removed. Invalid accounts are
dropped, the account count is capped at 256, duplicate account IDs are dropped after the
first valid entry, and invalid nested fields become closed safe defaults rather than
passing through.

Labels are at most 128 UTF-8 bytes, contain no C0/C1, bidi/format controls, or line breaks,
and are copied only after descriptor-safe inspection. Accessors, exotic prototypes,
cycles, functions, symbols, non-finite numbers, oversized values, and all unknown fields
are rejected or removed without invoking user code.

The periodic snapshot and preview never expose receivers, peer IDs, request IDs, memo,
fee, seed/key/viewing material, backup bytes/paths, PCZT/transaction/proof bytes, RPC or
node endpoints, device fingerprints, passphrases/PINs, raw diagnostics, provider/rate/
quote fields, generic method names, or a confirmation action.

### Preview reduction

The source intent may contain additional broker-private fields, but the sanitizer copies
only the preview fields above. State labels are fixed by code, never accepted from input:

| State | `status_label` | `can_cancel` |
| --- | --- | --- |
| `preparing` | `Preparing in BitBook Wallet` | true |
| `prepared`, `awaiting_confirm` | `Confirm in BitBook Wallet` | true |
| `signing`, `signed_unverified`, `verified`, `broadcasting` | `Sending in BitBook Wallet` | true |
| `crash_recovery` | `Wallet restarted. Confirm again in BitBook Wallet to send, or cancel.` | true |
| `cancelled` | `Payment cancelled` | false |
| `expired` | `Payment request expired` | false |
| `failed`, `unknown_needs_scan` | `Payment status unavailable` | false |

Any malformed preview becomes `null`. Error input is reduced to a known stable code or
`INTERNAL`; its message, stack, nested data, and retry text are never copied. Electron has
no Confirm action; `can_cancel` only describes the already-allowlisted `intent.cancel`.

### Pay view and fixed precedence

`derivePayView(snapshot, context)` accepts only a sanitized snapshot and this exact
context:

```text
{
  role: "payer" | "payee",
  asset: "ZEC" | "XMR",
  network: matching Network,
  request_valid: boolean
}
```

It returns a new closed object containing `visible`, `can_begin`, `can_request`, and one
row per sanitized account with `account_id`, the three booleans, `reason_code`, and a fixed
`status_label`. A nonmatching asset/network row is inert with `WRONG_NETWORK`; it never
becomes a fallback selection. The result must not return or retain the context or snapshot.

The deterministic blocker precedence is:

1. malformed context or snapshot: `SCHEMA`;
2. broker `down`/`degraded`: `UNAVAILABLE`;
3. broker `locked`: `LOCKED`;
4. invalid payer request: `SCHEMA` (payee creation does not require an inbound request);
5. mismatched asset/network: `WRONG_NETWORK`;
6. `privacy != private`, including transparent-only Trezor: `CAPABILITY_MISSING` with
   `This account is not available for private BitBook payments`;
7. hardware absent: `DEVICE_DISCONNECTED`;
8. ZEC `restored_pool == orchard`: `MIGRATION_REQUIRED` with
   `Restored pre-Ironwood shielded funds cannot be sent until a later consented migration`;
9. `watch_only`: receive/request may remain available, spend is `WATCH_ONLY`;
10. missing current receive/prepare/sign/Ironwood/v6/PCZT capabilities:
    `CAPABILITY_MISSING` or `PROTOCOL_INCOMPATIBLE` according to the accepted WAL-002
    capability result; and
11. broker `syncing`: entry remains visible but payer `can_begin` is false with `SYNCING`.

For a payer, `visible` requires a valid inbound request and at least one matching private
account that is either ready or syncing. `can_begin` requires a ready spend-eligible
account. For a payee, `visible`/`can_request` requires a matching private account with
`can_receive_private` and `can_derive_fresh_receiver`; ZEC additionally requires
`can_receive_ironwood`. A watch-only account may create a request but cannot begin a
spend. Quote/fiat presence or absence never affects any result.

`buildPayeeReceiverParams(snapshot, context)` additionally requires exactly
`account_id` and `request_id` (32 lowercase hex) in the context. It returns exactly
`account_id`, `asset`, `network`, and `request_id` only when the selected account's
`can_request` is true. Otherwise it throws a stable non-secret error code matching the
view. It never returns a receiver from the periodic snapshot and never calls the broker.

## Required test groups

1. Closed sanitization: valid full snapshot, malformed roots, descriptors/accessors,
   prototypes, cycles, limits, duplicate IDs, fixed capability allowlist, safe labels,
   numeric bounds, cloning, and removal of every forbidden field/canary.
2. Payer matrix: software/hardware/watch-only across both assets; down, degraded, locked,
   syncing, disconnect, protocol mismatch, invalid request, wrong network, and ready.
3. Zcash privacy: transparent-only Trezor is never private/eligible; Orchard-restored
   funds are exactly `MIGRATION_REQUIRED`; Ironwood/v6/PCZT flags fail closed.
4. Payee matrix: fresh-receiver eligibility for software, hardware, and watch-only;
   exact returned parameters; request-ID/account selection boundaries; no periodic
   receiver; no fallback account on a requested-account failure.
5. Preview: every state, fixed label, Cancel-only action, malformed/unknown state, stable
   error normalization, cloning, and removal of receiver/fee/memo/peer/request/rate/raw
   transaction/PCZT/secret canaries.
6. Real boundary: supervisor publication and Electron subscription both traverse the
   shared sanitizer; hostile nested values never reach the renderer; the exact six-method
   preload and five-channel Electron allowlists remain unchanged.
7. Policy/CI: the new maintained module/test are syntax-checked and executed; imports are
   closed; renderer CSP gains no rate or wallet endpoint; no dependency or lockfile
   change occurs.

Tests use only Node built-ins, committed deterministic data, and the existing CommonJS
harness. They open no files beyond fixtures, spawn nothing, access no network or device,
and leave no timers/listeners/handles.

## Test-first phases and current authorization

### Phase A — test source only (accepted; expected red active)

Under `docs/handoff/CODEX_SOL_BBD_WAL_005_TESTS_01.md`, Codex Sol may create or edit
only:

- `test/walletPay.node.js`
- `test/walletSupervisor.node.js`
- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`
- `test/fixtures/wallet-pay/snapshots-v1.json`

No production source, package/workflow/policy implementation, Rust, renderer DOM/CSS,
dependency, lockfile, evidence, documentation, Git, GitHub, command execution, network,
device, or unlisted path is authorized for the source actor.

The test drop is reviewer-accepted at the five exact hashes recorded in
`docs/testing/BBD-WAL-005-TEST-SOURCE-REVIEW-01.md`. Under
`docs/handoff/HERMES_BBD_WAL_005_EXPECTED_RED_01.md`, Hermes will run, in order:

```text
node test/walletPay.node.js
node test/walletSupervisor.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

Each command must exit nonzero only on the newly reserved WAL-005 behavior. Pre-existing
assertions must remain green. A later production handoff will authorize source paths and
targeted/broader green commands only after this expected-red evidence is reviewed.

Hermes stopped the first expected-red attempt before production evaluation because
`test/walletPay.node.js` did not parse. Only the exact one-line parenthesis correction in
`docs/handoff/CODEX_SOL_BBD_WAL_005_TESTS_CORRECTION_01.md` is now authorized.

## Falsification and acceptance

Final acceptance requires targeted green plus `npm test`, `npm run build`, and
`npm audit --audit-level=high`. Security-policy import/sink checks and the existing
Gitleaks workflow remain blocking. No package build or Rust gate is required because this
ticket changes neither packaging nor Rust.

Hermes must separately falsify and restore at least:

1. mark a transparent-only hardware account private/eligible;
2. allow an Orchard-restored ZEC account to begin Pay;
3. copy a forbidden receiver or fee into the Electron preview; and
4. add an Electron Confirm channel/action.

Each isolated mutation must make the focused test fail for the intended reason. The
restored tree must then pass the complete gate. No falsification mutation is committed.
