# WAL-019 production source review 01

Decision: source accepted for Hermes green/falsification and local development
wallet rebuild. Final runtime acceptance and publication remain pending.

Reviewer: Codex. HEAD: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`.
Accepted `wallet-broker/src/account_ui.rs`: 1332 lines, SHA-256
`09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a`.
Frozen test source remains 2654 lines, SHA-256
`e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446`.
All six other inputs from the production handoff still match. The change is
confined to the authorized source file: 87 added and 36 removed lines against HEAD.

## Source assessment

The account list now renders the existing endpoint and connection disclosure
before the enabled Sync action. The click calls the existing port once, records
the returned account/job and opens the existing progress view. Balance instead
opens that view without starting a job. A window-local endpoint survives navigation;
the running view disables edits and synchronizes idle edits back to that state.

Selection and lock gating use the refreshed account list. A list failure clears
selection through the existing path. The existing account/job result validation,
locked-balance masking, cancellation, inline unlock, port-error sanitization and
worker implementation are preserved. No new coin, network, privilege, dependency
or persistence behavior was added. The list reserves additional vertical space
and wraps the disclosure; the frozen pointer suite must establish actual usability
at both specified window sizes and protect existing account controls.

No blocking source finding remains. This is not a claim that Grok's focused tests
passed: no result transcript was supplied with this completion message. Codex did
not execute tests. Hermes must establish green on the frozen source and prove that
suppressing only the new list dispatch makes the one-click regression fail.

## Local runnable artifact

The owner expects accepted changes to reach the executable used locally.
`social-main.js` loads the development wallet from
`wallet-broker/target/app-resources`; source/test compilation alone does not refresh
that resource. The existing `scripts/build-wallet-broker.js` builds offline and
stages the binary with its matching SHA-256 manifest. After all validation gates,
Hermes is authorized to run that existing script and verify the debug binary,
staged binary and manifest agree. This is a local development refresh, not public
packaging/release or an app restart. A currently running process continues using
its loaded executable until restarted normally.

The separate stale bb-go daemon and build-before-launch automation remain queued;
this desktop task does not mutate or restart the daemon. Inherited WAL-009/security
release blockers are not cleared by this UI acceptance.

Next handoff: `docs/handoff/HERMES_BBD_WAL_019_SYNC_GREEN_01.md`. It also authorizes
the already-requested correction of the expected-red narrative from raw evidence.
Reviewer governance scope: this review, that handoff, WAL-019 ticket status,
desktop CURRENT_TASK.md, payment status-map routing and bb-go CURRENT_TASK.md.
