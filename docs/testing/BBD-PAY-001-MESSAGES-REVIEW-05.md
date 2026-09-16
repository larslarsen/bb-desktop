# Messages finish source review 05

Reviewer: Codex. Decision: Sol's bounded production drop is accepted for verification.
This is source review, not final acceptance. No tests, syntax checks or Electron runs
were executed by the reviewer. HEAD remains
`7a31c41cb29692a94acf1f24adb379f3a829d237`; index is empty.

## Source findings

Read the two authorized files and verified the frozen inputs. The five behavioral
failures accepted in [review 04](BBD-PAY-001-MESSAGES-REVIEW-04.md) have corresponding
repairs:

- Text keys use messageId scoped to session/local identity/conversation, with occurrence
  fallback for older messages. Reused bubbles update their read label in place.
- Read receipts have one AbortController owner, captured context and unread IDs.
  Success remembers only captured IDs; later unread arrivals remain eligible. Failed
  receipts wait for the scheduled reconciliation cadence.
- Peer/session changes and disposal invalidate and abort receipt ownership; stale
  completion cannot clear a newer owner. Typing timers are cancelled and context-guarded.
- Focus restoration uses preventScroll and restores the saved anchor afterward.
  The existing browser assertions still need actual Electron execution.
- Exported registered test functions now await fixture teardown themselves, so direct
  invocation by walletPay also cleans up. Both test and cleanup errors remain visible;
  all fixtures are attempted even if one cleanup fails. Standalone cleanup is idempotent.

There are still 70 registered inbox cases, including the five known failing assertions.
The source inspection found no remaining blocker to executing them. It does not prove
runtime success. The frozen 903-line Electron driver is unchanged.

## Reviewed identities

All paths are relative to bb-desktop. The first two rows are the delivered source drop;
the remaining identities are preserved inputs. Hermes must retain machine-readable
before/after manifests rather than relying only on this review's table.

| Path | SHA-256 | Lines |
| --- | --- | --- |
| social/app.js | 539def8f2240023c004b788736888fd021ecb5c12c05997d06b96928011bbdda | 1264 |
| test/paymentInbox.node.js | 638b1f4aacf7d75458d9b5592d9e9c8d58a471018a802591b319376baffb8f1a | 2748 |
| test/paymentInbox.electron.js | d884456fcef8955ca5835da2ae13e6864e334bf4ef6305742385386e478e58bf | 903 |
| social/payment-inbox.js | 437541ac7d161ad78380fded0adac1c2486f64116f166e908b7cacdb57aa6e64 | 340 |
| social/index.html | 6fe484ea8692fe661f13b100496fd9472aab7f18bfded0f7735f2b52b0162f24 | 189 |
| social/styles.css | f2572a739fa836cef9b1a2b7f32fd5aa3aced8e8d41bc72b107e22409c1c3898 | 988 |
| wallet-pay/inbox-client.js | a41463f491fdf1aa317d972f69438b62c273ccb59ad8e2a99444fabe563de8e5 | 609 |
| social-main.js | 298362de5c8c6f846ac63beb582574cea6194fa819efa9e4e10f99508431e9f6 | 322 |
| wallet-preload.js | 6c22b08bed294c5eac750f96d12f7227548574eee178765954195585cae6f025 | 72 |
| social/core.js | 4d27e4701ff299ef9753c02174bd8868aa01c94509b84d5a8187978fa239fb8f | 64 |
| test/socialCore.node.js | ee54f31e2845c70f34cb2d6969f8955bd0d309f5fbfd0d03aa33e6d201663521 | 21 |
| test/fixtures/payment-inbox/records-v1.json | 76f5a86a7337c3733b36696ad212c976ff8a7c2c848b7b1cdda0e1b968ef0650 | 81 |
| test/walletPay.node.js | ba18dd463e552cff33f104fdd80c403e9dac7f370af84f8db8906d5114fc1c12 | 790 |
| test/walletPreload.node.js | 373801529b001ea700872d4330d09b1c31c4bc98cc6e1ae0824bcbabf1cffa6e | 140 |
| test/electronSecurity.node.js | 22962ae4a3259290d9d64250f3cbac5bcf56ce8cf0ccb0764842ec90664c094d | 2195 |
| wallet-contract/canonical.js | 32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761 | 373 |
| wallet-pay/model.js | acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e | 590 |
| package.json | 76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5 | 41 |
| package-lock.json | 0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8 | 389 |
| scripts/security-policy.js | 0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d | 2806 |
| test/securityPolicy.node.js | ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c | 3816 |

## Next authorized work

[Hermes finish verification 01](../handoff/HERMES_BBD_PAY_001_MESSAGES_FINISH_VERIFY_01.md)
combines focused green, isolated read-label falsification, restored green and the
sandboxed populated Messages journey in one assignment. No further owner UI test is
needed before those results. Sol's source handoff is closed; Grok remains unavailable.

The prior red's metadata gaps remain recorded, not reconstructed. Final security scans,
publication and reviewer acceptance remain pending. Inherited policy failures are not
waived. JS/HTML/CSS load directly; no unchanged native or daemon rebuild is needed.

Reviewer governance paths: this review, the new Hermes handoff, closed Sol production
handoff, CURRENT_TASK, tickets/BBD-PAY-001.md and the end-to-end status map.
