# Current Task

QUEUED OWNER REQUESTS (2026-09-17):
[rich media/IPFS attachments](../../../bb-go/tickets/BBGO-MEDIA-001.md) and
[libsignal messaging](../../../bb-go/tickets/BBGO-MSG-001.md).
The first covers posts and private messages: emoji/GIF pickers, inline images/video,
reactions, desktop paste/drop and Android native file selection. Shared React components
and the Android host are
new work, not an existing foundation. Private attachment keys travel inside libsignal
messages. Public posts use signed public content/IPFS and no libsignal. Both tickets
are queued; no desktop source assignment or actor launch.
The active daemon phase remains [NET-001](../../../bb-go/tickets/BBGO-NET-001.md).
Reviewer-only cross-repository documentation scope and baselines are in MEDIA-001.

ACTIVE REVIEWER PRIORITY: ring-of-trust architecture, including blacklists and whitelists.
Read [BB-TRUST-RESEARCH-SCOPE-01](../architecture/BB-TRUST-RESEARCH-SCOPE-01.md).
Owner directs resolving this before account/recipient architecture is frozen.
Initial research and the integrated proposal are recorded in
[BB-TRUST-ARCHITECTURE-01](../architecture/BB-TRUST-ARCHITECTURE-01.md).
T1 RESOLVED: apply community judgments automatically; retain filtered content in
Spam for inspection/restoration and persistent personal overrides. Zero-reputation
accounts have full ordinary feature access with a warning to others. Reputation alone
must not quarantine newcomers or disable messages, requests or calls. Limited
behavior-based spam controls are permitted. This supersedes the prior A recommendation.
Reviewer architecture selection is recorded in section 3A: replaceable default profile,
bounded assessor delegation, distinct-issuer matching and strict two-thirds quorum
(initially four of five source groups), explicit expiry and finite synchronization.
The actual source roster and workload calibration remain activation requirements.
Account authority is now selected: a backed-up controller with independently revocable
device keys. The first verifier contract is
[BBGO-ACC-001](../../../bb-go/tickets/BBGO-ACC-001.md), which is also the sole handoff.
The current daemon actor, phase and relay instructions are governed by that ticket
and [bb-go CURRENT_TASK](../../../bb-go/docs/handoff/CURRENT_TASK.md). Follow their
latest phase authorization. No desktop source change or actor launch is assigned here.
No further owner confirmation of T1 is needed.
The owner confirms switching reviewer effort to High. This supersedes the earlier
instruction to keep xhigh. Daemon evidence goes in the ticket's named repository report;
this desktop routing note does not independently activate a daemon phase.

SELECTED ACCOUNT ARCHITECTURE: read
[BB-ACCOUNT-RECIPIENT-PROPOSAL-01](../architecture/BB-ACCOUNT-RECIPIENT-PROPOSAL-01.md).
N1 PROVISIONAL / NOT A PROJECT GATE: the owner tentatively accepted the naming proposal
with reservations and now directs continuing. Earlier instructions to obtain another
naming decision before continuing are superseded. Do not report naming as solved or
reopen the discussion without a concrete feature dependency.
Completed source screen: grapeid/keri-go v0.1.5 at
`f06d1dac5a36dea4b619eb6934e43e6fdc68b835` is not accepted unchanged. The account proposal
records a rotation-threshold authorization gap, ignored threshold parse errors,
recovery-reconciliation gaps in the reviewed APIs, and limited mobile evidence.
These are source findings; no package execution or BitBook integration was performed.
The subsequent comparison selected the primary-key/device-key model using standard
Ed25519 and explicit grants/revocations. Backup restores the controller; losing every
backup or compromising that controller is not repaired by this profile. ACC-001 fixes
the first verifier's format and tests, with no new dependency or current integration.
Protected key custody, pairing, durable revocations/synchronization and admission
freshness remain follow-on contracts before portable accounts can be enabled.
This does not block unrelated UI or accepted v1 behavior. The exact cross-repository
reviewer governance paths are enumerated at the end of the trust scope.

COMPLETE: BBD-PAY-001 — received requests in automatically updated Messages.

Feature `f23950245c338d83072caa3d96503b69a21682e2` and closeout
`48c60b3f05591323b06f493df974819c50add76f` are published and reviewer verified.
[Final review](../testing/BBD-PAY-001-FINAL-REVIEW-01.md) accepts the delivered read-only
slice and records the exact remaining release blockers and evidence limitations.
All PAY execution handoffs are closed. No further testing, publication, source work,
restart or live node/wallet operation is assigned by that closeout. ACC-001 is the
separate active task above. Reviewer-only documentation correction is recorded in
the final review.

Owner reports monerod has synced. Availability is recorded in WAL-007/WAL-009 and the
status map; node/network/height and wallet readiness are not independently verified.
PAY is complete; no live RPC or XMR implementation is authorized by this availability update.

Owner's portable identity/readable-name, WebRTC and future video-call requirements are
recorded in [identity/transport direction](../architecture/BB-IDENTITY-TRANSPORT-DIRECTION-01.md).
Read-only v1 UI is complete; settle account/device migration before the next signed-recipient
creation/approval contract or mobile enrollment. ACC-001 authorizes only the isolated
verifier test source; it does not enable identity/transport integration.

All authorizations below are historical and do not grant current execution authority.

## Hermes finish red (closed; report now present and reviewed)

Routing clarification: the owner relayed a sixteen-file BBGO-PAY-003 publication list.
That is the completed daemon task, confirmed by bb-go's CURRENT_TASK and final review.
It provides no evidence for this desktop run. Hermes must work in **bb-desktop** using
the linked finish-red handoff; no daemon republication or Git action is authorized.

Completion check: after the owner's latest done notice, the reviewer found neither
`docs/testing/BBD-PAY-001-HERMES-FINISH-RED-01.md` nor `dist/pay001-finish-red01/`
in this checkout. Workspace filename/reference searches found no alternate report.
The four test/production hashes still match the handoff. Execution is unverified;
no production repair is authorized. Hermes must finish the existing handoff and save
its required record. If execution occurred elsewhere, recover its actual captures and
identify that cwd before considering another run. Do not reconstruct missing results.

Sol's two test files are reviewed; production hashes are unchanged. Source has 70
registered cases, not yet run. Review: [test-source review 03](../testing/BBD-PAY-001-MESSAGES-REVIEW-03.md).
Sole execution authority: [Hermes finish red 01](HERMES_BBD_PAY_001_MESSAGES_FINISH_RED_01.md).
Verify frozen identities, run the two syntax checks and bounded Node suite, save the
named report/raw captures. No source changes, Electron, broader acceptance or Git.
Next source authorization follows reviewer inspection of actual red results.
No actor launched and no owner UI test requested.

## Sol finish test-source phase (closed)

Recovery is closed. [Review 02](../testing/BBD-PAY-001-MESSAGES-REVIEW-02.md) verifies
the eleven saved captures, corrects recovery-report counts and records remaining
read-status, receipt ownership, runner/viewport and Electron driver defects.
Former source authorization: [Sol finish tests 01](SOL_BBD_PAY_001_MESSAGES_FINISH_TESTS_01.md).
Use gpt-5.6-sol High under the documented Grok-usage-exhaustion fallback. Only its two
test files may change; no execution or production edits. Hermes records red after
reviewer test-source inspection, before production repair. No actor launched.

## Record recovery (closed)

Owner reports Grok exhausted weekly usage. Correction source and eleven captures are
present, including a 63-test restored green, but Grok's completion report is absent.
Former handoff: [Hermes record recovery](HERMES_BBD_PAY_001_CORRECTION_RECORD_RECOVERY_01.md).
Read and document existing evidence only; no reruns, source changes or Git mutation.
Grok is paused. Codex reviews the recovered drop before any further source assignment;
Sol High is the documented fallback if source work remains. No actor launched.
The correction routing below is historical and does not grant execution authority.

## Messages correction authorization (paused; usage exhausted)

Governing decision: [requests in Messages](../architecture/BBD-PAY-MESSAGES-UX-01.md).
Requests appear as cards in the requesting peer's conversation, including peers with
no text history, and update automatically. The standalone Requests tab is superseded.
Grok's automatic-inbox drop is delivered; its handoff is closed. Hermes standalone UI
capture is superseded and must not run. Paused source authorization:
[Grok Messages correction 01](GROK_BUILD_BBD_PAY_001_MESSAGES_CORRECTION_01.md), four
source/test paths, focused commands and its named report. Messages-01 hashes/logs are
verified (54 inbox tests, 32 security), but review found stale session/chat mutations,
polling ownership/cleanup defects, unstable rendering and Electron fixture failures.
Review: [Messages review 01](../testing/BBD-PAY-001-MESSAGES-REVIEW-01.md).
The correction preserves automatic updates for text and requests. Owner relays the handoff;
no actor has been launched. Hermes acceptance and Git remain unauthorized pending source
review. No additional owner test is requested.
The earlier authorizations below are historical and do not grant execution authority.

## Automatic inbox authorization (historical; closed)

The owner rejected manual connection management and needing Refresh for incoming P2P
requests. Remove both controls and daemon/folder instructions; no replacement setting.
The accepted local endpoint is snapshot-only: reread on activation and five seconds
after each completed read while Requests is visible, with bounded scheduling, cleanup
and stable background rendering. This is automatic local synchronization, not push.
Recorded source authorization:
[Grok automatic inbox 01](GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md).
Only its four UI/test paths and named completion report are writable. Preserve all
transport/IPC/daemon source and prior evidence. No actor launched by Codex.

Hermes UI runtime 01 is paused until the changed UI/harness hashes are reviewed and
its baseline updated. Do not run the older frozen capture during these edits. Broader
acceptance and Git publication remain pending. Owner's prior empty-inbox observation
stands; no further manual testing is requested before this removal is implemented.

## Hermes UI runtime 01 authorization (paused pending revised UI baseline)

UI-finish harness source and retained evidence are verified; actual Electron still
aborted on its adjacent unconfigured helper before UI startup. Review:
[UI finish review 04](../testing/BBD-PAY-001-SOURCE-REVIEW-04.md).
The former [Hermes UI runtime 01](HERMES_BBD_PAY_001_UI_RUNTIME_01.md) authorization
was to stage an identical isolated Electron copy omitting its unconfigured helper,
use the verified pre-existing installed helper, run the frozen smoke once and record
artifacts. No source/test edits, host permission changes, broader gates or Git mutation.
New report: `docs/testing/BBD-PAY-001-HERMES-UI-RUNTIME-01.md`.
Owner reports the actual inbox shows no requests and "everything works". The label
"Connect local daemon" is confusing. Observation, interpretation and the queued
plain-language connection-setting follow-up are recorded in
[owner UI check](../testing/BBD-PAY-001-OWNER-UI-CHECK-01.md). No source changes are
authorized by this feedback. Populated fixture UI, sandbox proof, screenshots and
final acceptance remain pending. No actor launched by Codex.

## Grok UI finish 01 authorization (historical; closed)

Codex verified correction 02: all nineteen source/input identities, six command
metadata/log pairs, timestamp red, 38 inbox / 32 security / 58 combined green.
Review: [source review 03](../testing/BBD-PAY-001-SOURCE-REVIEW-03.md).
Actual Electron screenshots and final acceptance remain pending. Do not repeat the
old failed sandbox command or rerun unchanged Node suites.
Owner may relay only [Grok UI finish 01](GROK_BUILD_BBD_PAY_001_UI_FINISH_01.md).
It authorizes a conditional run using the already-installed Chrome sandbox helper,
the collapsed-details geometry correction and demonstrated inbox CSS fixes only.
Transport/IPC and all other source stay frozen. No helper/host policy changes.
New report: `docs/testing/BBD-PAY-001-GROK-UI-FINISH-01.md`.
No Hermes acceptance, Git mutation or real user-process restart. No actor launched.

## Correction 02 authorization (historical; closed)

Correction 01 source/capture hashes are verified: 33 inbox, 6 preload, 32 security,
53 combined tests green; most earlier source defects are addressed. Review found a
new daemon compatibility defect: fractional-second received_at values are rejected.
UI screenshot execution still fails before startup; layout assertions also need repair.
Review: [source review 02](../testing/BBD-PAY-001-SOURCE-REVIEW-02.md).
Owner may relay only [Grok correction 02](GROK_BUILD_BBD_PAY_001_CORRECTION_02.md).
It authorizes the narrow fixes, missing focused checks and a tool-escalated isolated
UI run where available, retaining Chromium sandboxing and any tool approval requirement.
New report: `docs/testing/BBD-PAY-001-GROK-CORRECTION-02.md`.
No Hermes acceptance, Git mutation or user process restart. No actor launched by Codex.

## Correction 01 authorization (historical; closed)

Codex reviewed Grok's saved drop. Source and raw-log hashes match, but request cleanup,
reselection/disposal, descriptor type checks, record validation and IPC need correction.
The Requests UI exists in source; Electron aborted before screenshots or UI validation.
Review: [source review 01](../testing/BBD-PAY-001-SOURCE-REVIEW-01.md).
Owner may relay only [Grok correction 01](GROK_BUILD_BBD_PAY_001_CORRECTION_01.md).
It authorizes bounded test-first fixes and isolated sandboxed UI evidence, with a new
report at `docs/testing/BBD-PAY-001-GROK-CORRECTION-01.md`. Preserve original evidence.
No Hermes acceptance, Git mutation or real process restart. No actor launched by Codex.
The original implementation handoff below is historical and superseded for execution.

## Original PAY-001 authorization (historical)

Owner-directed next step: make received requests visible in the desktop UI.
Ticket: [BBD-PAY-001](../../tickets/BBD-PAY-001.md).
Selected source actor: Grok Build 4.6 High, manually relayed by the owner.
Original handoff: [Grok inbox handoff](GROK_BUILD_BBD_PAY_001_INBOX_01.md).
It authorizes bounded tests, UI/main-process implementation, focused offline checks,
and Grok's own repository completion report. No actor has been launched by Codex.
No Hermes acceptance or Git mutation is authorized at this stage.

Required drop: working Requests tab, authenticated local-daemon read bridge, clear
empty/disconnected states, focused test evidence and actual fixture UI screenshots in
`docs/testing/BBD-PAY-001-GROK-01.md`. The owner relays pointers, not evidence in chat.
No desktop inbox implementation or screenshot is claimed by this task authorization.

Daemon BBGO-PAY-003 is accepted and published; its final review is
`../../../bb-go/docs/testing/BBGO-PAY-003-FINAL-REVIEW-01.md`.
Daemon sources/binary remain frozen; no real app or daemon restart is authorized.
Preserve all unrelated dirty WAL-009, dependency, policy and historical record work.
This active prefix supersedes the stale routing below; earlier handoffs stay closed.

## Historical WAL-019 closeout and earlier routing

COMPLETE: BBD-WAL-019 — runtime, local wallet refresh and publication accepted.

NO ACTIVE DEV-001 TASK. Owner cancelled the helper project; the request was simply
to rebuild the existing daemon. Codex successfully rebuilt ../bb-go/modern/bitbookd
directly with Go 1.27.0. No process restart. Do not relay or execute DEV-001 handoffs.
At this historical closeout, daemon PAY-003 was next. It is now accepted; the
active prefix above authorizes desktop PAY-001. Desktop WAL-019 remains complete.
Routine binary builds follow AGENTS.md and are included in task completion.

Codex verified immutable capture 02: all 17 command metadata/log pairs, actual
Gitleaks 8.30.1 output, exit 0 and no leaks across the two published commits.
HEAD/tree/remote stay fixed at correction 7a31c41cb29692a94acf1f24adb379f3a829d237;
all ten source pins and wallet artifacts match. Final decision:
docs/testing/BBD-WAL-019-FINAL-REVIEW-01.md. Historical pre-commit evidence gaps
remain documented; the new exact published-range scan resolves final-byte coverage.
All WAL-019 handoffs are closed. No further implementation or actor execution is
authorized under WAL-019. No app restart occurred; the next full restart loads the refreshed wallet.
Mobile-aware UI, social profile pictures and remaining payment work stay queued.
Daemon build automation is cancelled. Inherited release blockers remain open. Closeout records
are local; the feature and correction commits are already published.

HISTORICAL CAPTURE 02:

Correction commit 7a31c41cb29692a94acf1f24adb379f3a829d237 is pushed and its exact
eight paths, accepted source/artifacts and 32 raw-file links/digests are verified.
Closeout still waits: the reported final scanned tree differs from the committed
tree, and exit/version metadata was not captured. See
docs/testing/BBD-WAL-019-PUBLICATION-REVIEW-02.md. Owner may relay
docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_CAPTURE_02.md to Hermes. Its sole
authorization is the supplied read-only scan/capture driver with an ignored output
directory. No report edits, staging, commits, pushes, tests, builds or restart.
Runtime acceptance stands. Earlier publication/correction instructions below are
historical and closed. Codex has launched no actor; no new implementation is active.

HISTORICAL CORRECTION 01:

Codex verified feature commit f2d9c9a513b3285531352df84ada818fdac0faf6 on local
and origin master, exactly 20 authorized paths, all ten source pins and wallet
artifacts. Runtime acceptance stands. Publication evidence requires correction:
only one scan log is retained, final staged-byte coverage is unsubstantiated, and
report corrections remain incomplete. See
docs/testing/BBD-WAL-019-PUBLICATION-REVIEW-01.md.
Owner may relay docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_CORRECTION_01.md to
Hermes on a free Nous Portal model. It authorizes exact record corrections, new
captured publication scans and an eight-path corrective commit. No tests, builds,
source changes or restart. The older publication 01 instructions below are
historical and must not be rerun. Codex has launched no actor. Final closeout waits
for correction verification; no new implementation task is authorized.

WALLET DESIGN DIRECTION: owner expects a possible mobile version and wants mobile
ZEC wallet references considered. See docs/architecture/BBD-WAL-MOBILE-DESIGN-DIRECTION-01.md.
Reviewer proposes Zodl (formerly Zashi) for simple home/action/status organization,
YWallet for compact/wide activity layouts, and MonteZecret as a secondary desktop
reference. Plan consistent flows across phone and desktop; no mobile framework or
shared-UI-code decision yet. WAL-019 publication is complete; redesign remains queued.

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

HISTORICAL PUBLICATION 01: Owner could relay docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md to Hermes using
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
