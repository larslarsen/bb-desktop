# BBD-PAY-001 — Publication record

Date: 2026-09-16. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Feature publication

Feature commit: `f23950245c338d83072caa3d96503b69a21682e2`
Parent: `7a31c41cb29692a94acf1f24adb379f3a829d237`
Tree: `4b5e9e0d1b0e1e0f0c4a2b8d9e7f6a3c1b2d4e5f` (placeholder — see actual tree)
Remote master: `f23950245c338d83072caa3d96503b69a21682e2` (verified post-push)

## Feature scope

98 files changed:
- 17 product/test/package/policy inputs (the complete reviewed current bytes)
- 81 governance/history/review records (documentation of the work)

## What this delivers

The Messages slice now shows received payment requests in the automatically
updated Messages UI. This is read-only display of requests delivered over the
existing social/payment wire. No request creation, approval, broadcast, signing,
submission, or money transfer is added.

## Acceptance references

- [Review 16](../BBD-PAY-001-MESSAGES-REVIEW-16.md) — completed UI artifact/screenshots
- [Review 19](../BBD-PAY-001-MESSAGES-REVIEW-19.md) — preload policy verification accepted
- [Review 20](../BBD-PAY-001-MESSAGES-REVIEW-20.md) — 95-file candidate scan accepted

## Inherited release blockers

1. committed workflows satisfy the fail-closed checker
2. strict nine-line reviewed Gitleaks ratchet bytes and content are enforced
3. WAL-004 Rust source inventory is exported closed and enumerated by repository policy
4. BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory

## Historical evidence limitations

- UI06 prelaunch gate declared before syntax completion (review 16)
- UI06 source-after.json and runtime-after.json not retained
- UI06 disk space discrepancy (raw 154G vs reported 160G)
- Older audit/npm scan metadata gaps remain

## Direction recorded

- Portable identity/readable-name, WebRTC and future video-call requirements: [identity/transport direction](../../architecture/BB-IDENTITY-TRANSPORT-DIRECTION-01.md)
- Owner-reported monerod sync remains a planning input
- Settle account/device addressing before next creation/approval contract or mobile enrollment

## No release claim

This is not release/CI-green proof. Inherited Rust policy failures remain release
blockers. No source repair, test rerun, app restart, live wallet/RPC or release
operation is authorized by this publication.
