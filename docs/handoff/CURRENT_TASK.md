# Current Task

ACTIVE: BBD-WAL-019 — runtime and local wallet refresh accepted; Hermes publication 01.

WALLET DESIGN DIRECTION: owner expects a possible mobile version and wants mobile
ZEC wallet references considered. See docs/architecture/BBD-WAL-MOBILE-DESIGN-DIRECTION-01.md.
Reviewer proposes Zodl (formerly Zashi) for simple home/action/status organization,
YWallet for compact/wide activity layouts, and MonteZecret as a secondary desktop
reference. Plan consistent flows across phone and desktop; no mobile framework or
shared-UI-code decision yet. Current Hermes publication scope stays active.

QUEUED WORKFLOW FOLLOW-UP: the owner expects the local daemon executable to stay
current through the development workflow. npm start currently launches Electron;
it does not build bb-go/modern/bitbookd. Its correct local executable path contains
an older build. bb-go CURRENT_TASK.md records the needed modern build/launch and
verified local-artifact refresh follow-up. The desktop wallet resource is now rebuilt
and verified; daemon automation remains queued. No process restart has occurred.

Owner priority is payments and usable UI; advertising remains deferred. The daemon
startup/request-delivery slice BBGO-PAY-002 phase A is accepted and published at
bb-go c00764d4a84eb0e149ec2779d1746248f86ba3a4 with successful CI. Do not rerun its
old handoff. End-to-end payments remain incomplete; see
docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md for the remaining sequence.

Execution review: docs/testing/BBD-WAL-019-EXPECTED-RED-REVIEW-01.md. Raw logs and
all seven before/after/current pins verified: formatter exit 0; six tests execute
and fail on the missing account-list Server control, exit 101; 16 filtered out.
The review corrects Hermes's inaccurate tool versions and running-view explanation.
Hermes corrected the main red-report errors; the remaining wording/link corrections
are included in publication 01, with no red rerun. Runtime red is accepted.

Production review: docs/testing/BBD-WAL-019-PRODUCTION-SOURCE-REVIEW-01.md.
Accepted wallet-broker/src/account_ui.rs: 1332 lines, SHA-256
09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a.
Test source remains 2654 lines, SHA-256
e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446.
Runtime/local-build review: docs/testing/BBD-WAL-019-GREEN-REVIEW-01.md.
Verified: focused 6 green; one dispatch mutation fails with an empty start list;
exact restoration gives 6 green; all 22 account UI tests pass. The debug and staged
wallet binaries match SHA-256
6f0b2ce78a7d871bde75d1fa8f330bd27dc7a00f598853dcc60944b92d15f985,
and the manifest matches. The next normal app restart loads this wallet resource.
The daemon executable is unchanged. The review corrects the green report's wrong
mutation hash and missing artifact details from raw evidence.

Owner may relay docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md to Hermes using
a free Nous Portal model. It authorizes report corrections, an exact 20-path staged
scan/commit/push and publication evidence. All source and artifacts stay frozen;
no rerun or rebuild. Keep all unrelated changes. Codex verifies publication before
final closeout. Social profile pictures, mobile-aware redesign and daemon development
build automation remain queued; no new implementation task is authorized here.

The WAL-009 inventory handoff remains cancelled. Preserve all unpublished WAL-009,
policy and other work. Recorded release blockers remain open; all earlier handoffs
below are historical and cannot authorize execution.

OWNER-DIRECTED WORKFLOW CORRECTION: Grok source handoffs may and should include
`cargo fmt` plus the focused offline compile/test command, with iteration on its own
compiler and focused-test failures. Do not route an unformatted or uncompilable Grok
drop to Hermes. Hermes owns final recorded acceptance and Git integration.

HISTORICAL DIAGNOSTIC: Hermes broader diagnostic completed with a truthful red.
`cargo fmt --check` exit 0; full Rust test suite exit 101 (17 passed, 1 failed —
`zec_hardware.rs` pre-existing overreaching transport-dependency assertion);
Clippy 35 lib + 40 test pre-existing warnings (not in WAL-009 paths);
native_surface check exit 0; security-policy.js exit 1; 6 policy test failures
(matching historical six-failure baseline, no new failures). Evidence:
`docs/testing/BBD-WAL-009-CRASH-RECOVERY-BROADER-DIAGNOSTIC-01.md`. No source edits
authorized by this diagnostic. Await Codex review.

HISTORICAL EXECUTION: Hermes applied `cargo fmt` (resolving 5 layout hunks in `store.rs`), then
`cargo fmt --check` passed (exit 0, no mutation). Three crash-recovery tests
were verified passing individually (each takes ~150-300s):

- `crash_recovery_requires_fresh_native_confirmation_with_exact_bindings` — PASS
- `crash_recovery_revalidates_recovered_artifact_before_verified_publication` — PASS
- `verified_restart_reopens_locked_in_crash_recovery_without_broadcast` — PASS

A pre-existing test/manifest contradiction was identified:
`operation_and_source_inventories_exclude_broadcast_network_mainnet_xmr_and_real_hardware`
at `zec_sign_verify.rs:1260` fails because the frozen `Cargo.toml` contains `tonic`
and `tokio` deps (required by `src/zec/live_transport.rs`) but the test asserts
`!manifest.contains("tonic")` and `!manifest.contains("tokio")`. This was failing
before any WAL-009 or Hermes changes.

Evidence: `docs/testing/BBD-WAL-009-CRASH-RECOVERY-GREEN-RESUME-02.md`

Reviewer decision needed on:
1. Whether the inventory test is correct (deps should be removed) or the test is
   wrong (deps are legitimate).
2. Whether the crash-recovery `PreparedCleanup` fix to `test_support.rs` and
   `prepare.rs` is acceptable, or whether a different approach is preferred.

HISTORICAL: Grok's corrected WAL-009 verified crash-recovery production drop resolved
the substantive production defects, but its recovery replay helper still forced
`UNAUTH` after the actual second consumption call and therefore could not falsify a
broken one-shot capability. The owner may manually relay
`docs/handoff/GROK_BUILD_BBD_WAL_009_CRASH_RECOVERY_PRODUCTION_CORRECTION_02.md`
to Grok Build 4.6 High. Only `wallet-broker/src/zec/test_support.rs` may change; the
exact correction removes that forced error. No tests, execution or Git. Codex reviews
the returned one-path source before any Hermes green authorization.

HISTORICAL: Grok's first WAL-009 verified crash-recovery production drop is reviewer
rejected. The owner may manually relay
`docs/handoff/GROK_BUILD_BBD_WAL_009_CRASH_RECOVERY_PRODUCTION_CORRECTION_01.md`
to Grok Build 4.6 High. Only `wallet-broker/src/zec/store.rs`, `zec/spend.rs`, and
`zec/test_support.rs` may change; no tests, execution or Git. Codex reviews the
corrected source before any Hermes green authorization.

HISTORICAL: Hermes's WAL-009 verified crash-recovery expected red is reviewer accepted.
The owner may manually relay
`docs/handoff/GROK_BUILD_BBD_WAL_009_CRASH_RECOVERY_PRODUCTION_01.md` to Grok Build
4.6 High. Grok may edit only `wallet-broker/src/zec/store.rs`, `zec/spend.rs`, and
`zec/test_support.rs`, without execution or Git. Tests and all other production paths
are frozen. Codex reviews the returned source before any Hermes green authorization.

HISTORICAL COMPLETE: Hermes executed both gates per `HERMES_BBD_WAL_009_CRASH_RECOVERY_EXPECTED_RED_RESUME_01.md`.
`cargo fmt --check` exited 0 (no mutation). `cargo test zec_sign_verify --locked --offline --no-default-features`
exited 101 with 40 missing-contract diagnostics (absent `DurableIntentStatus`,
`SessionBindingForTest`, reopen/recovery/revalidation methods, `reopen` associated
function, `ConfirmationMutation::ActiveRecoverySession` variant,
`mutate_recovered_receiver_for_test`, and `revalidate_recovery_with_receipt`). Zero
tests executed. Evidence: docs/testing/BBD-WAL-009-CRASH-RECOVERY-EXPECTED-RED-RESUME-01.md.
All eight frozen identities verified before and after — all MATCH. Cargo.lock unchanged.
Stopped for Codex review. No source edit, Git, network, or actor action authorized.

HISTORICAL: Hermes correctly stopped on a two-hunk formatter failure before the WAL-009
crash-recovery expected red. The owner may manually relay
`docs/handoff/GROK_BUILD_BBD_WAL_009_CRASH_RECOVERY_FORMAT_CORRECTION_01.md` to
Grok Build 4.6 High. Grok may apply only the two exact rustfmt layouts in
`wallet-broker/tests/zec_sign_verify.rs`; no semantic edit or execution is authorized.
Codex reviews the correction before Hermes may rerun the gates.

HISTORICAL EXECUTION: Hermes executed the formatter gate for WAL-009 crash-recovery
expected-red. The formatter (`cargo fmt --check`) exited 1 with a two-hunk
diff in `zec_sign_verify.rs` (lines 131 and 237). Per handoff section 1, a
formatter failure is NOT the intended red — the gate must pass (exit 0)
before the expected-red test command is authorized. Hermes stopped after
recording the formatter failure; `cargo test zec_sign_verify` was not run.
Evidence: docs/testing/BBD-WAL-009-CRASH-RECOVERY-EXPECTED-RED-01.md.

HISTORICAL: Grok's first WAL-009 verified crash-recovery test drop was reviewer rejected.
The owner may manually relay
`docs/handoff/GROK_BUILD_BBD_WAL_009_CRASH_RECOVERY_TESTS_CORRECTION_01.md` to
Grok Build 4.6 High. Only `wallet-broker/tests/zec_sign_verify.rs` may change. Grok
does not execute tests or touch production/Git. Codex reviews the corrected source
before any Hermes red authorization. Codex must not launch or substitute another actor.

HISTORICAL: manually relayed Grok test-source task for WAL-009 verified crash recovery.
Read `docs/handoff/GROK_BUILD_BBD_WAL_009_CRASH_RECOVERY_TESTS_01.md`. Grok Build
4.6 High may edit only `wallet-broker/tests/zec_sign_verify.rs`, does not execute
tests, and stops with a source drop. The owner launches Grok manually; Codex must
not launch or substitute another actor. Codex reviews the returned test source before
Hermes receives any expected-red authorization. No production source, execution, Git,
wallet-data access or other actor is authorized by this handoff.

HISTORICAL REVIEW: manually relayed Grok review to resume the testnet send work in WAL-009.
Read docs/handoff/GROK_BBD_WAL_009_RESUME_REVIEW_01.md. Only bounded read-only
inspection and a reply to the owner are authorized; no source edits or execution.
The owner launches Grok manually. Codex must not launch or substitute Sol; earlier
escalation permissions are historical. Codex reviews the returned work and freezes
the next test-first source contract. Hermes remains the later authorized executor.
No background actors have been launched for this task.

Owner confirmed application Sync complete at4339332/4339332 with zero confirmed
and pending received shielded funds. WAL-018 is complete. Its records below are
historical and do not supersede this active routing instruction.

WAL-018 default-server correction is reviewer accepted and rebuilt. Sync now opens
with https://zaino.testnet.unsafe.zec.rocks:443, avoiding the original provider's
inconsistent checkpoint. The user's selected endpoint in the reported retry was not
confirmed. A private viewing-cache clone reached actual engine Current at4339325;
original profile was opened readonly only. Temporary clone/helper/example are removed.

All16native UI tests passed, restoring the old default caused the intended runtime
regression failure, restored source passed, and production Clippy/build passed.
Staged broker SHA25665972fcab1661872232bb435c1ffe0e479f93b4f74df62c3eaafadb7c6b082a1.
Fully quit and restart BitBook, unlock the account and press Sync; saved progress resumes.

The six inherited policy failures remain release blockers; directory secrets scan exit1
contains352 classified public-checksum matches and zero credentials, unchanged baseline.
See docs/architecture/BBD-WAL-018-REVIEW.md for the diagnostic executor restart caveat,
docs/testing/BBD-WAL-018-GREEN-01.md for verification, and WAL018_INTEGRATE.json plus
BBD-WAL-018-INTEGRATION-01.md for exact publication evidence. No further source work or
user-profile mutation is authorized. Earlier entries below are historical.

WAL-017 diagnosis is COMPLETE: the default testnet server returns inconsistent block
and checkpoint hashes at4308319, reproducing both with separate and persistent HTTP2
connections. The broker correctly rejects that batch before commit. User's checkpoint
4308219/4338623 remains saved; no wallet data was modified or copied. No product code,
dependencies, tests or binary changed. The unused temporary diagnostic example was
removed without execution.

Operator-advertised alternative https://zaino.testnet.unsafe.zec.rocks:443 returned the
same prior checkpoint and block batch with a matching end checkpoint. User can set the
native Sync screen Server field to this endpoint and Sync to resume. This establishes
consistency for the failing100block batch, not full-wallet sync completion. Other
probed alternatives: ZEC.PRO HTTP521; Nighthawk connection failed.

Exact public comparison and publication are recorded by WAL017_INTEGRATE.json in
docs/testing/BBD-WAL-017-INTEGRATION-01.md. No additional source work is authorized.
Prior completed records below are historical; native binary remains the WAL016 build.

WAL-016 sync interruption correction is COMPLETE and reviewer accepted.
Ordinary 15-minute auto-lock still wipes spending keys; explicitly started viewing-only
sync continues on the same screen through completion. Balances stay hidden until masked
inline unlock reveals the same result without another sync. Explicit cancellation and
manual lock/hide/drop still stop work and clear results. Session timeout is unchanged.

Hermes verified 48 account/native-UI/session tests and production Clippy; a real-engine
Testnet fixture finished after forced expiry. Removing returned-copy redaction caused
the intended runtime failure; exact restoration passed. Native broker rebuilt and both
Xvfb and actual desktop open/hide/reopen/close checks passed using empty test profiles.
Staged broker SHA256 e805ff8b9ecb88f6bab4c0360572b5e1d1261187cdbb0ebd3a154c42f09ff9aa.
No user profile or dependencies changed. The six inherited policy failures remain;
pinned directory scan exit1 contains352 previously classified public-checksum matches,
zero credential findings. See docs/testing/BBD-WAL-016-GREEN-01.md and FINAL-01.md.

Source d9cd7ac4253f5956ce58509cadbe56ad1a6c835a and integration evidence
593ed713f04a39f9e29cef3a9fbfcc8271feca8c are pushed to origin/master. Reviewer
independently verified every integrated blob and unchanged tested working bytes.
Committed scanner exit1 has82 previously classified public-checksum matches and zero
credential findings. All source work and execution are complete; no background work
remains authorized. See docs/testing/BBD-WAL-016-INTEGRATION-01.md.
Fully quit and reopen BitBook to load the rebuilt broker.
The completed WAL015 entry below is historical and its binary hash is superseded.

WAL-015 native live Zcash TESTNET balance synchronization is COMPLETE.
Source edda35308913d59683a08be58052dc7c2d95dbaf and integration evidence
105a2e36af03d26dbc15a3d052901d836b2a63b5 are pushed to origin/master.
No implementation or background actor work remains authorized by this ticket.

Restart BitBook, then Wallet -> Manage accounts -> select/unlock -> Sync balance ->
Sync. The server is editable; default https://testnet.zec.rocks:443 uses direct TLS.
The native screen shows connection disclosure, progress, cancellation and confirmed/
pending received shielded funds. First sync covers history from testnet NU5; restart
resumes committed progress. Receive/Copy and existing account custody remain intact.
No user profile was read or modified. Mainnet and payments remain disabled.

Accepted111distinctRusttests,28appJSgroups,33manifestpolicygroups; productionClippy;
real crash/journal recovery falsification and exactrestore; native UI pointer proof;
Xvfb and actualdesktopwindowproof; dependency audit with no newvulnerabilities.
Final staged Linux x64 broker SHA256:
6c2880cef4eafc5b36e0f47428d4f9ace98deed0f62703c7bab6fd3a7c909f7
552181016bytes. All37integratedblobs independentlyverified. All executionCLIactors exited.

Six inheritedsecurity-policy failures remain releaseblockers;7oldtest-helperlint
warnings remain recorded. Gitleaks exits1 for verified publicfilechecksums in WAL015generatedrecords:
184directory/32committedfindings in integration, zero actualsecretfindings after exact
captured-value comparison to4recomputedpublicfilechecksums at integration. Rules/config/ignore unchanged;
never describe that scan as exit0. See ticket exception and integration evidence.
No installer or release-ready claim. Original npm/policyworkingedits and2historical
untracked evidence drafts preserved; only5WAL015policychanges staged on originalHEAD.

See tickets/BBD-WAL-015.md and docs/testing/BBD-WAL-015-LIVE-INTEGRATION-01.md.
All earlier WAL015handoff authorizations and failures are historical execution records.

Finalgovernance scanner:82public-checksum findings,0actualcredentials after reviewer
verified the fifth referenced publicfile. See
docs/architecture/BBD-WAL-015-FINAL-SCAN-REVIEW.md for exactclassification, final
review and runnertermination. No active/background implementation remains.
