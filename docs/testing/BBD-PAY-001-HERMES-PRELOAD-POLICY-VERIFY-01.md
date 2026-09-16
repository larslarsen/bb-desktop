# BBD-PAY-001 — Hermes preload policy verification 01

Date: 2026-09-16. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and versions

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 3c3ab69a · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Node: v22.23.1

## Baseline

HEAD: `7a31c41cb29692a94acf1f24adb379f3a829d237` (verified before/after).
Empty index. Unrelated dirty work preserved.

## Source manifest (21 pins verified)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 1599 | `f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54` |
| test/paymentInbox.node.js | 2748 | `638b1f4aacf7d75458d9b5592d9e9c8d58a471018a802591b319376baffb8f1a` |
| social/app.js | 1264 | `539def8f2240023c004b788736888fd021ecb5c12c05997d06b96928011bbdda` |
| social/payment-inbox.js | 340 | `437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64` |
| social/index.html | 189 | `6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24` |
| social/styles.css | 988 | `f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898` |
| wallet-pay/inbox-client.js | 609 | `a41463f491fdf1aa317d972f69438b62c273ccb59ad8e2a99444fabe563de8e5` |
| social-main.js | 322 | `298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6` |
| wallet-preload.js | 72 | `6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025` |
| social/core.js | 64 | `4d27e4701ff299ef9753c02174bd8868aa01c94509b84d5a8187978fa239fb8f` |
| wallet-contract/canonical.js | 373 | `32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761` |
| test/socialCore.node.js | 21 | `ee54f31e2845c70f34cb2d6969f8955bd0d309f5fbfd0d03aa33e6d201663521` |
| test/fixtures/payment-inbox/records-v1.json | 81 | `76f5a86a7337c3733b36696ad212c976ff8a7c2c848b7b1cdda0e1b968ef0650` |
| test/walletPay.node.js | 790 | `ba18dd463e552cff33f104fdd80c403e9dac7f370af84f8db8906d5114fc1c12` |
| test/walletPreload.node.js | 140 | `373801529b001ea700872d4330d09b1c31c4bc98cc6e1ae0824bcbabf1cffa6e` |
| test/electronSecurity.node.js | 2195 | `22962ae4a3259290d9d64250f3cbac5bcf56ce8cf0ccb0764842ec90664c094d` |
| wallet-pay/model.js | 590 | `acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e` |
| package.json | 41 | `76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5` |
| package-lock.json | 389 | `0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8` |
| scripts/security-policy.js | 2808 | `c5431ee340564af26f0d3d2f5f19a3aa72869a45d9b3b0d64353a7da266732cb` |
| test/securityPolicy.node.js | 3922 | `29e4e213c803ca7792fae51cff99aaa61f96abdf43830157fda50c89292799fd` |

## Executor results

All 10 stages completed successfully via the bounded executor.

### Stage 00: hermes-version
Exit: 0. Hermes Agent v0.18.2 (2026.7.7.2)

### Stage 01: node-version
Exit: 0. Node v22.23.1

### Stage 02: policy-syntax
Exit: 0. `node --check scripts/security-policy.js` passed.

### Stage 03: test-syntax
Exit: 0. `node --check test/securityPolicy.node.js` passed.

### Stage 04: focused-green
Exit: 0. **3/3 passed.**
- wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC
- PAY-001 preload policy accepts the exact seven channels and real bridge
- PAY-001 preload policy rejects missing duplicate computed and unlisted channels

### Stage 05: falsified-positive
Exit: 1. **Effective falsification.** Removing the two payment channel literals
caused the positive group to fail at its independent list comparison first:
```
Expected 7 channels, got 5 (missing payment:inbox:get, payment:inbox:connect)
```

### Stage 06: falsified-real-preload
Exit: 1. **Effective.** Invoking the copied checker directly on the unchanged
real preload copy produced the specific unlisted-IPC failure:
```
PolicyError: wallet-preload.js contains dynamic or unlisted IPC invoke
```

### Stage 07: restored-focused
Exit: 0. **3/3 passed.** After restoring the copy byte-for-byte, all three
focused groups passed again.

### Stage 08: repository-policy
Exit: 1. Inherited inventory failure (Grok's dirty WAL-009 spend files).

### Stage 09: policy-suite
Exit: 1. **90 pass / 4 fail.** Same 4 inherited failures:
1. committed workflows satisfy the fail-closed checker
2. strict nine-line reviewed Gitleaks ratchet bytes and content are enforced
3. WAL-004 Rust source inventory is exported closed and enumerated by repository policy
4. BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory

**No new failures from the PAY preload policy changes.**

## Verdict

PAY PRELOAD POLICY VERIFIED. The bounded correction is accepted:
- Focused green: 3/3 passed
- Falsification: effective (positive group fails without the two literals)
- Real preload: correctly rejects unlisted IPC
- Restored focused: 3/3 passed
- Full suite: 90/94 (4 inherited inventory failures, no new PAY failures)

21/21 source pins verified. Index empty. Stop for Codex review.
