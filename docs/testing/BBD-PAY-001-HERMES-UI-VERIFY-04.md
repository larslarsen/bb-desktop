# BBD-PAY-001 — Hermes UI verification 04

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
| test/paymentInbox.electron.js | 1373 | `1e47205c5961a3d5f24ebadd42052e5bb8c7d0afcaff4ee2812b56ecb94a3927` |
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
- Runtime copy: 72 original entries, 71 copied entries, exactly one omission (chrome-sandbox), no extra entries, no mismatches

## Commands executed

### run-01-syntax

Command: `timeout --signal=TERM --kill-after=5s 30s node --check test/paymentInbox.electron.js`
Exit: 0. Syntax valid.

### run-02-ui

Command: `CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-verify04/ui-01 timeout --signal=TERM --kill-after=5s 300s xvfb-run -a dist/pay001-finish-verify01/runtime/electron test/paymentInbox.electron.js`
Exit: 1. Failed.

## Failure: keyboard did not close details

The driver completed bootstrap and the full UI journey reached keyboard interaction
testing. The Origin fix from review 11 worked — social fixture now returns 200 for
all GET routes including `/ob/config`, `/ob/chatconversations`, `/ob/profile`, etc.

The test failed on `keyboard did not close details` — an assertion that pressing
Escape should close expanded payment request details. This is a different, later
assertion than the prior `configured ready empty payment state missing`.

## Sandbox status (verified working)

```json
{
  "browser": { "pid": 737376, "Seccomp": "0", "NoNewPrivs": "0", "NSpid": "737376" },
  "renderer": { "pid": 737422, "Seccomp": "2", "NoNewPrivs": "1", "NSpid": "737422\\t4" }
}
```

## UI state at failure

From `ui-failure-diagnostics.json`:
- `online_status: true` — daemon connection established
- `payment_notice_hidden: true` — no payment notice blocking
- `request_card_count: 1` — one payment request card rendered
- `conversation_count: 1` — one conversation present
- `connection: "HTTP only"` — connected via HTTP (not WebSocket)
- `fixture_failures: 0` — no fixture errors
- `blocker.unexpected: 0` — no unexpected blocked requests
- Social fixture: 9 hits, all 200 (config, chatconversations, profile, following, posts, peers, chatmessages)
- Payment fixture: 4 hits, all 200

## Screenshots

| File | Dimensions | SHA-256 |
| --- | --- | --- |
| fixture-empty-messages-1440x1000.png | 1440x1000 | `823a825e90b8f6553857416fac68ae7c1394db3f125768229d44daecbf1c25ee` |
| fixture-failure.png | 1440x934 | `aec564221de3f28fce72677991ea87c6b5657ac3deb65c0c0f0c3b9a3d77674c` |

## Evidence paths

- `dist/pay001-ui-verify04/actor.txt`
- `dist/pay001-ui-verify04/source-before.json`
- `dist/pay001-ui-verify04/driver-source.js`
- `dist/pay001-ui-verify04/preflight.json`
- `dist/pay001-ui-verify04/runtime-before.json`
- `dist/pay001-ui-verify04/syntax.stdout.log`
- `dist/pay001-ui-verify04/syntax.stderr.log`
- `dist/pay001-ui-verify04/ui.stdout.log`
- `dist/pay001-ui-verify04/ui.stderr.log`
- `dist/pay001-ui-verify04/ui-01/bootstrap-diagnostics.json`
- `dist/pay001-ui-verify04/ui-01/sandbox-status.json`
- `dist/pay001-ui-verify04/ui-01/ui-failure-diagnostics.json`
- `dist/pay001-ui-verify04/ui-01/fixture-empty-messages-1440x1000.png`
- `dist/pay001-ui-verify04/ui-01/fixture-failure.png`

## Verdict

UI RUN FAILED — `keyboard did not close details`. The Origin fix worked: social
fixture returns 200, payment module ready, UI journey reached keyboard interaction
testing. The failure is on Escape key not closing expanded payment details.

No source edits. 21/21 pins verified unchanged. Stop for Codex review.
