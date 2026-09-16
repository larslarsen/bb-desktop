# BBD-PAY-001 — Hermes publication preparation 01

Date: 2026-09-16. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and versions

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 3c3ab69a · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Node: v22.23.1

## Baseline

HEAD: `7a31c41cb29692a94acf1f24adb379f3a829d237` (verified before/after).
Empty index. Unrelated dirty work preserved.

## Source manifest (21 pins verified before/after)

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

All 3 stages completed successfully.

### Stage 00: hermes-version
Exit: 0. Hermes Agent v0.18.2 (2026.7.7.2)

### Stage 01: gitleaks-version
Exit: 0. Version: 8.30.1

### Stage 02: candidate-scan
Exit: 0. Gitleaks: **0 findings** (empty array)

## Candidate snapshot

- **Path count**: 95
- **Path list**: `docs/handoff/BBD_PAY_001_PUBLICATION_PATHS_01.txt`
- **Path list SHA-256**: verified
- **Snapshot location**: `dist/pay001-publication-prep01/snapshot/`
- **Candidate source manifest**: `dist/pay001-publication-prep01/candidate-source.json`
- **Candidate copy manifest**: `dist/pay001-publication-prep01/candidate-copy.json`
- **Manifest SHA-256**: `dae69a08f036bfd3e7ef244658248a8109da426606c09b6e3696be0be7980ee3`
- **Source/copy match**: exact byte-for-byte equality verified
- **Post-scan verification**: source and copy manifests unchanged

## Gitleaks verification

- **Scanner**: `target/security-tools/gitleaks-v8.30.1/gitleaks`
- **Scanner size**: 21958840 bytes
- **Scanner SHA-256**: `88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`
- **Command**: `gitleaks dir --redact=100 --no-banner --report-format json --report-path dist/pay001-publication-prep01/gitleaks.json dist/pay001-publication-prep01/snapshot`
- **Exit**: 0
- **Findings**: 0 (empty array `[]`)

## UI06 evidence addendum (review 16 corrections)

Per review 16, the following corrections to the UI06 report are acknowledged:

1. **Enter toggles details**: The journey does not prove sending a chat message with Enter.
   The `/ob/chat` POST is the typing path, not proof of successful message sending.

2. **electron-smoke.json samples truncated**: Retains only the first 12 social/payment hits
   and first 8 expected blocks (12 social samples: 11 GET + 1 POST, 12 payment, 8 expected).
   Samples do not establish complete traffic counts. No response status retained.

3. **Missing source-after.json and runtime-after.json**: UI06 report did not include these files.

4. **Prelaunch gate contradiction**: prelaunch-gate.json claimed all checks passed at 01:45:00Z
   and syntax exit 0, while syntax metadata says the command ended at 01:45:01Z.

5. **Disk space discrepancy**: Raw preflight shows 154G disk space while JSON/report say 160G.

## Verdict

PUBLICATION PREP COMPLETE. Gitleaks scan of 95 candidate paths: **0 findings**.
Source/copy manifests match exactly. Post-scan verification confirms no drift.

21/21 source pins verified. Index empty. Stop for Codex review.
