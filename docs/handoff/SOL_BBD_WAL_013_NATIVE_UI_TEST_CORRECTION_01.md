# WAL-013 native window test correction — Sol escalation

Reviewer escalation under AGENTS: Grok2195bb7e outer47873 stopped exit1 "Max turns
reached" without a usable completed drop. Draft has compile blockers and contradictory
assertions. Sol gpt-5.6-sol High authorized as fill-in, no subagents. Only write
wallet-broker/tests/account_native_ui.rs. Baseline SHA256 326e3d83e6fa0a9813ee94c9101e01925659e357b497f57509d28f4dc350e069.
Read AGENTS,TESTING, original NATIVE_UI contract and following exact review. Do not
repeat Grok broad research; pinned APIs paths below if necessary. No production,
Cargo/feature/dependency edits, test/syntax/build/format execution, Git/evidence or
other actors. Keep nine focused tests and real event-driven proof. Finish promptly.

Required fixes:
1 Import AccountSummary,LocalAccountManager from accounts module, not account_ui:
no uncontracted production reexports. first_list currently accepts only FakePort but
real-composition test passes SharedAccountPort: make it generic over P/D like other
helpers. Specify Vec<AccountSummary> for small-layout collected accounts.
2 Pinned egui0.36.1 PlatformOutput.accesskit_update is cfg(accesskit); our existing
native-ui features exclude it. Remove direct uncompiled field access and qualify
claims: test WidgetInfo/platform event output; AccessKit isn't in the approved build.
Don't change Cargo to enable it. Inspect installed output.rs/context.rs APIs if needed.
3 Populated-list test locks UNLOCKED_ID then expects Unlock to staydisabled. Wrong:
after Lock, reselect that nowlocked account and assert Unlock opens form; cancel.
4 Restore test performs FOUR preparations (cancel,Escape,Xclose,success), expects3;
fix4. After successfulconfirm it calls click_label on a label requiredabsent: capture
confirmbutton rect BEFORE confirmation, assert label gone then rawclick oldposition,
assert exactlyone commit/no replay. Preserve distinct actualwidget confirmation proof.
5 Remove tautology !control.request_open() || control.request_open(). Use oneassert
and actual reopenobservations. Don't introduce vacuous assertion alternatives.
6 field_near must require actual painted field geometry; remove guessed-coordinate
fallback. Assert field/labels fully within screen AND clip, not justpositive1pixel
intersection (smalllayout usability proof). Buttonclick derives real painted labels;
no production flag mutation or test-only authority. Realfunctioncallcounts remain.
7 Cleanup currently swallows all errors and readdir failures. As corrected account
service test style (read its cleanuphelpers if needed), assert owned root/accounts
absent on normalcompletion; reportcleanupfailure without secondpanic during unwind.
Never follow links/recursive delete. Regularfile scanning must notsilentlyskipIOerrors.
8 Fix other obvious compile/API/fixture issues by source review, including unused
imports/fields (no checkexecution). Ensure direct run_logic API uses actual pinned
signature and that all exact output text required is consistent with fixedcontract.
9 Strengthen over1024 test: provide matching confirmation too, so length refusal is
proved independently of missing confirmation. Include UTF8 byte-boundary pair; exact
1024bytes accepted, >1024 rejected before appending any fragment. Use fakeport, no
extra KDF. Preserveunicodebackspace,copy/cut/undo/cancel stale-secret tests.

No other scope expansion. Production password buffer must preallocate1024 and never
reallocate containingsecret (latest clarification in originalcontract). These tests
remain red due missing modules; Hermes handles checks after review.
