# WAL-013 final widget test navigation and lint corrections — authorized

Hermes54202/session20260910_112229_eea8f8 collectedexit0,driverexit1. Diagnosticproved
exactcallsite1182 RESTORE_BACKUP missing after oldconfirm-coordinate replay;10existing
nativewidgets green. Clippyonly2redundantguard warnings lines384/391. Allpinsmatch.
Sol gpt-5.6-sol High source-only, nootheractors/execution/checks/formatter/Git/evidence.
Exactly two paths/baselines:
wallet-broker/src/account_ui.rs 03044acaea68d6ddbc2dbe91039e7ffd425c110c29aaa5f88011578a653cd92f
wallet-broker/tests/account_native_ui.rs 6c59850abcc28727af72afd89a7cc583ac93d656f23c9423210957a4d954906b

1 tests/account_native_ui.rs ONLY after firstcancel/replay assertions around1173-1176:
Staleconfirmcoordinate correctly cannotcommit, but now overlaps list Createaccount
button and opens freshCreateform. CREATE_ACCOUNT text is also heading, so original
assertdidnotestablishlistscene. Preserveallreplay confirm_calls==0/notCONFIRMassertions.
Use actualwidgetnavigation: if replay.paints(CONFIRM_PASSPHRASE) identifies Createform,
click actualCancel and assert resulting list has RESTORE_BACKUP. Otherwise require
RESTORE_BACKUP inreplay observation directly. Never change productionlayout to fitold
coordinate, never setformstate/directport, never delete approval/replayassertions.
Beforefreshrestorequeue ensurelistvia the observed uniquecontrol. No othertestchanges.
2 src/account_ui.rs ONLY two Key eventarms: replace `key` binding plus
`if key == egui::Key::Backspace`/Delete guards with direct key:egui::Key::Backspace /
Delete patterns; preserve pressed:true and actions. No lintallow or semanticchange.
Read only relevanttargetspans/AGENTS/TESTING andthishandoff; no broadresearch/full dumps.
Stop withhash/count.
