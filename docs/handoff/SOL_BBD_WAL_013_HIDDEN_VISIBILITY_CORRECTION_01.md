# WAL-013 native hidden visibility correction — authorized

Hermes53848/session20260910_112856_a138ab collectedexit0,driverexit1. Mutantbuild
succeeded, actualwindow test failed BEFORE account.manage atinitialhiddenassertion:
windowwasinitiallyvisible. This is not acceptablefalsification: openingmechanismnotyet
exercised. Runtime source andstagedproduction rebuilt/restored in finally; allpinsmatch.
No securitycommands ran. Ignoreactor's erroneousverbalfixture-message-drift/evidence
claims; rawshows realstartupvisibility failure and evidenceexists. Underlying test
already authored andred for exactbug; no newtestrequired beforeproductionfix.

Reviewer inspected pinned eframe0.36.1 epi_integration.rs post_rendering: firstpaint
calls window.set_visible(true); glow appliesviewportoutput afterpostrendering. Existing
AccountWindow hiddenbranch onlyscrubs input, never reasserts desiredhiddenstate.
Sol gpt-5.6-sol High source-only (recordedUI escalation), ONLY account_ui.rs hash
5f0baa710045735f3c12bb62eebc4ba5100950ef339e14ebe0137f18f4890058.
Make ONE bounded correction in service(): in `if !self.control.visible()` branch,
after scrubbing, send egui::ViewportCommand::Visible(false). Preserve deadline/list
refresh/repaint fallthrough; no earlyreturn, no Focus, no changes toquit/nativeclose,
request_open orcustody. Thus everyhiddenlogic/UIpass keeps OSwindowhidden evenframework
startupshowing. No tests/otherpaths/dependencies/formatters/execution/Git/evidence/
delegation. Read only thishandoff and branchspan; hashes/countandstop.
Actualwindow startup assertion andfalsification will be rerun against actualbuiltbytes.
