# WAL-019 publication record

Date: 2026-09-15. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and session

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 110baa09 · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Session identifier: not recoverable from Hermes version output.

## Baseline

HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` (verified before/after).
Branch: master.
Origin: `larslarsen/bb-desktop` on GitHub.
Remote master equals baseline (verified via `git ls-remote`).

## Pin verification

All ten pins from the green handoff match preflight, restoration, postflight and
current bytes.

| Path | Lines | SHA-256 | Match |
|---|---|---|---|
| wallet-broker/src/account_ui.rs | 1332 | 09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a | yes |
| wallet-broker/tests/account_native_ui.rs | 2654 | e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446 | yes |
| wallet-broker/Cargo.toml | 126 | 435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b | yes |
| wallet-broker/Cargo.lock | 5771 | a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a | yes |
| wallet-broker/src/accounts.rs | 1006 | 8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d | yes |
| wallet-broker/src/zec/live.rs | 1238 | 17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f | yes |
| wallet-broker/src/zec/live_transport.rs | 214 | a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e | yes |
| scripts/build-wallet-broker.js | 152 | 4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445 | yes |
| wallet-broker/launch-config.js | 140 | 68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8 | yes |
| social-main.js | 299 | d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a | yes |

## Staged paths (exact 20)

```
wallet-broker/src/account_ui.rs
wallet-broker/tests/account_native_ui.rs
tickets/BBD-WAL-019.md
docs/handoff/CURRENT_TASK.md
docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md
docs/architecture/BBD-WAL-MOBILE-DESIGN-DIRECTION-01.md
docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_TESTS_01.md
docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_TESTS_CORRECTION_01.md
docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_PRODUCTION_01.md
docs/handoff/HERMES_BBD_WAL_019_SYNC_EXPECTED_RED_01.md
docs/handoff/HERMES_BBD_WAL_019_SYNC_GREEN_01.md
docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md
docs/testing/BBD-WAL-019-TEST-SOURCE-REVIEW-01.md
docs/testing/BBD-WAL-019-TEST-SOURCE-REVIEW-02.md
docs/testing/BBD-WAL-019-EXPECTED-RED-REVIEW-01.md
docs/testing/BBD-WAL-019-PRODUCTION-SOURCE-REVIEW-01.md
docs/testing/BBD-WAL-019-GREEN-REVIEW-01.md
docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md
docs/testing/BBD-WAL-019-SYNC-GREEN-01.md
docs/testing/BBD-WAL-019-PUBLICATION-01.md
```

## Test acceptance

WAL-019 runtime behavior and local development wallet refresh accepted by Codex in
[BBD-WAL-019-GREEN-REVIEW-01.md](BBD-WAL-019-GREEN-REVIEW-01.md). This is a record-only
publication; no new tests were executed.

## Scanner identity

`target/security-tools/gitleaks-v8.30.1/gitleaks`: size 21,958,840 bytes.
SHA-256: `88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`
Version: gitleaks v8.30.1 (from embedded metadata).

## Staged-content secret scan

Command:
```
target/security-tools/gitleaks-v8.30.1/gitleaks git --pre-commit --staged --redact=100 --ignore-gitleaks-allow --no-banner .
```

First scan: exit 0, no leaks found.
Second scan (after adding publication report and restaging): exit 0, no leaks found.

Staged bytes verified unchanged between scans; accepted source hashes confirmed.

## Commit and push

Commit message: "Make native wallet sync start with one click"
Push destination: `origin HEAD:master`
Force-push: none.
Amend: none.

Report returned to owner for Codex review.
