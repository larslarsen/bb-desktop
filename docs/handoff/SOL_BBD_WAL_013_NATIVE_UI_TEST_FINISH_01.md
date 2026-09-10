# WAL-013 native UI test finish — two bounded corrections

Sol gpt-5.6-sol High source-only, no subagents. Previous actor71163/session
01a08c59-3d1a-7c33-8a78-2135f79e0874 collected exit0. Its nine-test correction is
accepted except reviewer-caused output-access mistake and paint-timing fixture.
Only wallet-broker/tests/account_native_ui.rs writable, SHA256
1baf72d66ea8ff810efd529957f79894eaf9cabb910c9583f1647f1dbda861aa,1477lines.
No production, execution/checks/formatters, Cargo/Git/evidence/otheractors. Read this,
AGENTS/TESTING if not retained, and ONLY relevant helper/struct spans in target.
No whole-file reread or other repository discovery needed. Two changes, stop.

1 Reviewer corrects own prior wrong instruction: installed egui0.36.1 output.rs:176
has UNCONDITIONAL pub accesskit_update:Option<accesskit::TreeUpdate>. Retain coverage,
no featurechanges: restore FrameObservation accesskit:String; from_output formats
actual optional update; assert_no_substr checks it as before. Rename platform-only
canary test back to includeaccessibility if desired. The correction contract item2
already corrected on disk; initial cfg claim is withdrawn. Read installed source
lines160-180 if desired. No sourceactor fault in following earlier directive.
2 click helper's visual observation currently comes from pointer release frame, which
can paint old scene before click handler updates state. After release run ONE empty
input UI frame and return that settled visual observation; still issue real hover,
press,release events and assert actual portcalls in tests. Direct ui.run Escape/close/
quit observations remain immediate and unchanged so lifecyclecommands aren't lost.
This permits ordinary event-driven repaint without making production artificially
paint a replacementscene in the click-processing pass. Do not mutate productionflags,
add sleeps/retries or weaken existing actualcallcount/absence assertions.

All other source accepted, including cleanupproof, generichelper, exactbyteboundary,
actualgeometry, fourpreparations and no-replay assertions. Finish hash/lines/tests only.
