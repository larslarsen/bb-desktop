# WAL-009 native widget validation acceptance

Decision: ACCEPT the six required validation outcomes and restored source.
Reject the actor's procedural-compliance and exact-transcript claims. Record
errata here and proceed with native layout/lifecycle test coverage; no passing
test is repeated solely to repair its report.

Reviewer: Codex at XHigh, baseline 2c8ea8f3187681f550299cc2a0c1cf242c619a2a.
No acceptance/compiler/test/formatter command was run by the reviewer.
Hermes outer 29799 was collected once after done, exit 0. Exact completed session
20260908_132224_391188, provider nous, actual model poolside/laguna-s-2.1:free.
Saved version 78446: Hermes 0.18.2 (2026.7.7.2), upstream fef0e16f, local 10b6d1a9.
Authorization aada9adb; launch/execution checkpoint 2c8ea8f3. The earlier
434d62f2 is the prior Grok launch checkpoint, not this Hermes launch.

The reviewer inspected all six saved command strings and completion outputs:

| Stage | Command / completed result message | Actual result |
| --- | --- | --- |
| Original six widgets | 78490 / 78497 | exit 0; 6 passed, 6 filtered; 0.14 s |
| Suppressed Confirm | 78506 / 78507 | exit 101; 1 failed at zec_review_tests.rs:153; 0.04 s |
| Confirm before close | 78520 / 78521 | exit 101; 1 failed at zec_review_tests.rs:190; 0.08 s |
| Restored six widgets | 78528 / 78529 | exit 0; 6 passed, 6 filtered; 0.12 s |
| Native surface | 78532 / 78533 | exit 0; 17 passed, none filtered; 0.00 s |
| Native compilation | 78536 / 78537 | exit 0; three recorded dead-code warnings; 6.28 s |

Both fault tests failed the exact intended assertions, not compilation or setup.
They are expected failures, not two passing tests as the actor's final reply
claims. The saved outputs contain all test names, diagnostics, and summaries;
their lengths are 1604, 1931, 1697, 1598, 2579, and 1074 characters. These small
captured completions suffice to establish results despite protocol deviations.

Mutation measurements 78505 and 78519 match the authorized 303-line hashes
fc406fcf71021b1c08b8402e565ffcfc7e23b016ba366888324f5579bfdd6cca and
4fa5e01f20de8abe1d7a737fcdeaf162d46af3c34fcf79c7fc4989253a9f7e9a.
Restoration measurements 78513 and 78525 return native_ui.rs to 303 lines,
c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4.
Both actor inventory passes and reviewer byte measurement confirm all 23 frozen
identities, including unchanged tracked native_surface.rs. Only the new evidence
record was left as an additional pending file: 503 lines, SHA-256
3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1.

## Authoritative errata and deviations

- The record reports the adoption model meituan/longcat instead of this session's
  actual poolside model, uses outer 29799 as its session ID, and confuses prior
  Grok and current Hermes launch lineage. Correct values are above.
- Only stage 1 set background=true. Its real handle was proc_875858cfe67f (OS PID
  3379109); saved 78491 was its launch, not final completion. The actor tried an
  unsupported get_log action, read a running partial log, then accepted wait
  completion 78497 without retrieving a final process.log. Stages 2-6 ran in the
  foreground, with exact command strings and actual returned exit statuses.
  The blanket background/full-capture/no-deviations claim is false.
- Evidence was written as a literal reconstruction, not rendered from saved JSON.
  All six blocks alter warning spacing/carets or blank lines. Stage 5 invents
  invalid_passphrase_lengths_wipe_before_unlock_or_restore_custody_or_commit;
  the actual passing name ends at _custody. Git status also misspells RETAINED-SPEND
  as RETAINED-Spend. Counts and the two intended failure diagnostics remain valid.
- The inventory has 23 paths: 22 pending plus tracked unchanged native_surface.rs.
  The report's final inventory is abbreviated; the actual saved final measurement
  78541 contains all 23. The reviewer independently compared all bytes.
- The record retains local absolute paths contrary to the publication rule.
  It must be normalized, with these errata incorporated, before eventual evidence
  integration. Group that work with a later authorized execution/integration;
  do not create another correction-only turn. This reviewer does not edit or
  integrate actor evidence. An extra target directory listing and broad CURRENT
  read are also recorded. No extra Cargo/npm command or Git mutation occurred.

## Remaining native coverage

The accepted six tests call the dialog directly. They do not exercise the actual
eframe App callback's programmatic-close handling, and use a large 1280x800
viewport with short synthetic strings. Native production uses a fixed 520x720
window, and platform constraints can provide less space. Controls must remain
visible and usable with longer review fields; this remains unproved.

Open [native layout and App lifecycle tests](../handoff/GROK_BUILD_BBD_WAL_009_NATIVE_APP_TESTS_01.md)
for Grok Build 4.6 High, test source only. This groups real App callback coverage
and constrained layout regressions before any production correction. Use the
locally pinned eframe Frame::_new_kittest and actual egui events/output. No OS
window launch is claimed; owning-thread/window/capability integration remains a
separate gate. Full recovered effects, actual-secret cleanup, remaining security,
integration, network/broadcast/mainnet/hardware, and Monero stay unaccepted or parked.

Reviewer publication scope: this review, linked new Grok handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. All pending developer
source/package/locks and eight evidence records remain uncommitted. No actor
polling; launch once and collect after done. XHigh remains appropriate.
