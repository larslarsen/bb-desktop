# BBD-PAY-001 — owner usability check

Status: OWNER-REPORTED SUCCESS for the empty inbox; connection-label usability issue.
Prepared by Codex after the owner volunteered to test on 2026-09-15.
This is a supplementary usability check. Formal fixture/sandbox acceptance is pending.

## Steps

1. Close BitBook normally and reopen it using your usual launcher for this bb-desktop
   checkout. A full restart loads the new main/preload code. Do not change launch flags,
   sandbox permissions or daemon configuration for this check. If startup fails, stop
   and record that instead of changing settings.
2. Look for **Requests** beside Feed, My posts and Network. Open it. Expected heading:
   **Payment requests**. If the tab is missing, record that and the launcher you used;
   an installed copy or an old process may not be using this checkout.
3. Check the displayed state. **No received requests** is valid for a connected daemon
   with no incoming requests. **Local daemon unavailable** is valid if no compatible
   daemon is running or the selected data folder is unavailable. The page should not
   stay blank or loading indefinitely. No sample requests are inserted into your data.
4. Click **Refresh**. The control should become usable again after completion; a
   disconnected daemon should produce a clear message within roughly five seconds.
5. Click **Connect local daemon**, then cancel the folder picker. The screen should
   recover and keep its existing selection. You do not need to locate a data folder
   or change your actual daemon for this check.
6. Resize the window smaller. Requests navigation, Refresh, Connect and status text
   should stay readable without overlap or text cut off. If real received requests
   already exist, check amount/asset/network/memo/status and expand Request details.
   This slice has no request creation, payment approval or send action.

## Recorded owner observations

Recorded by Codex from the owner's 2026-09-15 feedback:
"It says no requests. everything works, although I have no idea what connect local daemon is."
This is the owner's usability observation, not an independently executed acceptance test.

- App/inbox opens: owner reached the screen; exact launcher not specified.
- Initial message: owner reports "no requests"; no incoming rows available.
- General usability: owner reports "everything works".
- Refresh, picker cancellation and resize: no problems reported; individual steps
  were not separately described, so no separate measured results are claimed.
- Populated request details: not exercised; no requests available.
- Confusing control: "Connect local daemon" does not explain its purpose to the owner.

## Reviewer interpretation and follow-up

The empty state is rendered only after a ready inbox reply with zero inbound rows.
The control selects a BitBook data folder through a native picker; it does not start a
daemon, create a request or connect to a remote payment service. Normal startup already
uses the default local folder automatically. The owner need not use the picker for
normal default-folder operation.

Owner's subsequent decision: remove the control entirely. Other distributed app
features work without it, and the technical explanation did not establish a user need.
The previous reviewer suggestion to move it into advanced settings is superseded.
The owner then asked why Refresh was necessary when requests arrive through the P2P
loop. Recorded decision: remove both Connect and Refresh and make Requests update
automatically. Existing P2P receipt/storage works; automatic UI snapshot rereads were
missing. The bounded implementation uses the existing authenticated local interface,
reads on activation and repeats five seconds after settlement while the view is visible.
No new public API or payment event subscription is claimed.
Bounded source authorization: [automatic inbox handoff](../handoff/GROK_BUILD_BBD_PAY_001_AUTOMATIC_INBOX_01.md).
Hermes capture is paused for the revised UI baseline. The owner is not asked to repeat
this check before the visible change is implemented.

Owner observations are not engineering acceptance; populated fixture execution,
sandbox proof and final review remain pending. No additional owner check is requested
for the already reported empty state.
