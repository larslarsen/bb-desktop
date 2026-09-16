# Grok — BBD-PAY-001 Payment requests inbox

Actor: Grok Build 4.6 High. Reviewer: Codex. Owner manually relays this pointer.
Status: authorized source/test work and focused checks; no acceptance or Git authority.

Read `AGENTS.md`, `TESTING.md`, the active prefix of `docs/handoff/CURRENT_TASK.md`
and `tickets/BBD-PAY-001.md`. The ticket is the complete architecture, trust, UI,
writable-path, test, falsification and completion contract. Follow repository documents;
do not treat a chat completion claim as evidence or infer a wider task from old handoffs.

## Starting identity

Require desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237` and verify these
current working-file SHA-256 values. Preserve all unrelated tracked/untracked work.
Record baseline mismatch in your named report and stop; do not reset or repair it.

| Path | SHA-256 |
| --- | --- |
| social-main.js | d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a |
| wallet-preload.js | 3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df |
| social/app.js | 3eff1370db10731a93b618190b82852a894e781bea5b28a47bddb7854285da10 |
| social/index.html | 0c30c232b06ae92019b441fa8a51b9817b7ade1346a7ddae974dde3be36ac931 |
| social/styles.css | f0c3d15349846b7849dc12c430440520d99c091630ffea71567f9bcb5a5d1c7d |
| test/walletPreload.node.js | 60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e |
| test/electronSecurity.node.js | d70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482 |
| test/walletPay.node.js | 18969bd83cc0630c98839b9bc3e01a2cddd32a1723e77e6463eb750e31666315 |

Verify and preserve these frozen inputs (including existing unpublished edits):

| Path | SHA-256 |
| --- | --- |
| wallet-contract/canonical.js | 32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761 |
| wallet-pay/model.js | acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e |
| package.json | 76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5 |
| package-lock.json | 0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8 |
| scripts/security-policy.js | 0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d |
| test/securityPolicy.node.js | ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c |

New paths named in the ticket must be absent initially. Do not overwrite another drop.
Frozen daemon reference: `../bb-go/modern/localclient/server.go`, SHA-256
`5b00cc6e10694a5fbaccda27637751b866cbc6bb1f0bfd98f05dc94479cc7a93`.

## Execute and return to review

1. Author the ticket's tests first; run its targeted red commands and retain output.
2. Implement the visible Requests tab and main-only authenticated read bridge within
   the thirteen source/test paths. Do not return only transport infrastructure.
3. Iterate the ticket's focused offline commands on your own changes, perform the
   specified instance-binding falsification and prove exact restoration.
4. Run the actual sandboxed Electron fixture smoke and retain screenshots at both
   supported sizes. Keep the test transport/supervisor isolation in test source only.
5. Save `docs/testing/BBD-PAY-001-GROK-01.md` with complete results and relative links
   to immutable raw captures/screenshots under `dist/pay001-grok/`. Include actual
   model/tool identity and all required hashes/counts. Do not write local absolute
   paths or credentials into repository records; use repo-relative paths for evidence.
6. Stop for Codex source review. Return only the report pointer and a brief status in
   chat. A failure or unavailable screenshot must be documented honestly in the report.

No Git mutation, broader acceptance/security commands, dependency installation, actor
launches, real daemon/wallet activity, user-data access or process restart. No edits to
CURRENT_TASK, the ticket or other governance by Grok. Hermes execution is not active.

## Reviewer authorization record

Codex fixed this task's scope in `tickets/BBD-PAY-001.md`, selected Grok here, and
activated it in CURRENT_TASK. This implements the owner's existing repository control
plane: decisions and authorizations are durable before execution; chat is only a relay.
No developer implementation, test result, screenshot or acceptance is claimed yet.
Reviewer-authored paths for this authorization are exactly this handoff, the ticket,
AGENTS.md, docs/engineering/DEVELOPMENT_ROLES.md, docs/handoff/CURRENT_TASK.md and
docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md. Preserve previous edits in those files.
