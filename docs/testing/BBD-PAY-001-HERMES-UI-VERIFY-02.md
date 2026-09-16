# BBD-PAY-001 — Hermes Messages UI verification 02

Date: 2026-09-16. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and versions

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 15650b76 · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Node: v22.23.1

## Baseline

HEAD: `7a31c41cb29692a94acf1f24adb379f3a829d237` (verified before/after).
Empty index. Unrelated dirty work preserved.

## Source manifest (21 pins verified before/after)

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 1110 | `ead9f500b8eaafbc19d30961e8327b992437838b09d033e3edacf0d4f79192e1` |
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
| scripts/security-policy.js | 2806 | `0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d` |
| test/securityPolicy.node.js | 3816 | `ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c` |

## Preflight

- Filesystem: ext2/ext3, disk-backed, 160G free
- Host uid: 1000 (lars)
- chrome-sandbox: regular file, root:root, 4755, 15232 bytes, SHA-256 `c100b678...`
- Parents: /opt/google root-owned, /opt/google/chrome root-owned, no symlinks
- Original Electron: SHA-256 `180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`
- Copied Electron: SHA-256 `180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8`
- Runtime copy: 73 entries, all match original (modes/sizes/hashes), exactly one omission (chrome-sandbox), no extra entries, no symlinks

## Commands executed

### run-01-syntax

Command: `timeout --signal=TERM --kill-after=5s 30s node --check test/paymentInbox.electron.js`
Exit: 0. Syntax valid.

### run-02-ui

Command: `CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-verify02/ui-01 timeout --signal=TERM --kill-after=5s 300s xvfb-run -a dist/pay001-finish-verify01/runtime/electron test/paymentInbox.electron.js`
Exit: 1. Failed.

## Failure: configured ready empty payment state missing

The driver completed bootstrap successfully:
- `bootstrap-diagnostics.json`: outcome=success, phase=complete, elapsed 475ms, DOM identity verified, fixture storage ready, load-file resolved

But the test then waited 8 seconds for:
1. `document.getElementById("conversationList").innerText.indexOf("No conversations") !== -1` (line 741)
2. `document.getElementById("statusDot").classList.contains("online") && document.getElementById("paymentNotice").classList.contains("hidden") && document.querySelectorAll(".paymentRequest").length === 0` (line 747-749)

The wait for "No conversations" text timed out, suggesting the production app.js renders something different for an empty conversation list.

## Sandbox status (verified working)

```json
{
  "browser": { "pid": 715767, "Seccomp": "0", "NoNewPrivs": "0", "NSpid": "715767" },
  "renderer": { "pid": 715813, "Seccomp": "2", "NoNewPrivs": "1", "NSpid": "715813\\t4" }
}
```

Sandbox active: renderer Seccomp 2, NoNewPrivs 1, additional NSpid level.

## Evidence paths

- `dist/pay001-ui-verify02/actor.txt`
- `dist/pay001-ui-verify02/preflight.json`
- `dist/pay001-ui-verify02/syntax.stdout.log`
- `dist/pay001-ui-verify02/ui.stdout.log`
- `dist/pay001-ui-verify02/ui.stderr.log`
- `dist/pay001-ui-verify02/ui-01/bootstrap-diagnostics.json`
- `dist/pay001-ui-verify02/ui-01/sandbox-status.json`

## Verdict

UI RUN FAILED — empty payment state assertion timed out. Bootstrap setup succeeded
(DOM correct, sandbox active, load resolved), but the test's expected "No conversations"
text was not found in the conversation list within 8 seconds.

No source edits. 21/21 pins verified unchanged. Stop for Codex review.
