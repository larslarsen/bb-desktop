# BBD-WAL-013 — Native account management

Status: ACTIVE — account service tests authorized
Owner explicitly requested account management after completed WAL-011 startup.
Reviewer Codex; source Grok grok-4.6 High; execution/integration Hermes only.

Deliver a usable opt-in native Zcash TESTNET software-account management flow:
create, list, unlock, manual/idle lock, encrypted export and confirmed restore.
Reuse reviewed vault encryption/store/session primitives. No address/seed export,
network, transaction/signing/broadcast, mainnet, hardware or XMR onboarding. Those
are separate existing tickets; keep all payment capabilities false until supported.
This is functional native management, not another installer or policy detour.

Implementation sequence, carried through without owner done messages:
1. Account service composes actual encrypted store and timed sessions (current).
2. Broker-native management window and fixed opt-in application entry.
3. Running broker list/status/lock integration and native-capable development build.
4. Real persisted-account/native UI proof, focused regressions, falsification,
   source review and exact integration. Do not declare whole task done at step 1.

Initial contract: docs/handoff/GROK_BBD_WAL_013_ACCOUNTS_01.md.
Only its test paths are authorized now. Reviewer will fix UI/runtime semantics
before authorizing those paths. Existing unrelated pending package/policy changes
and two old evidence drafts stay preserved; no policy repair is authorized.
Existing inherited release failures stay visible; new boundary failures block.
