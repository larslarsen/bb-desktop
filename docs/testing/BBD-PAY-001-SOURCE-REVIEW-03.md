# BBD-PAY-001 source review 03

Reviewer: Codex. Date: 2026-09-15.
Decision: correction-02 source fixes accepted; UI execution and final acceptance pending.
Input: [Grok correction 02](BBD-PAY-001-GROK-CORRECTION-02.md).

## Verified

All nineteen current source/input identities in the report match (five changed,
eight unchanged drop paths, six original frozen inputs). All six correction-02 command
metadata hashes and their twelve stdout/stderr hashes match. Timestamp red records
exit 1 on `.1Z` rejected as invalid; restored green records exit 0 for 38 inbox and
32 Electron security tests. Combined wallet/inbox records 58 passing tests, twice.
The second identical combined run was unnecessary; do not repeat it at the UI gate.
HEAD is `7a31c41cb29692a94acf1f24adb379f3a829d237`; index is empty.

Source now accepts the daemon's fractional UTC receipt timestamps while retaining
calendar validation and the frozen signed canonical timestamp policy. The additional
header/body limit, coalescing, observed descriptor I/O, root symlink and both IPC-channel
checks are present. Memo/peer text wraps; clipping was removed. These fixes are accepted
for the next UI execution gate, not a claim that the whole product or release is accepted.
Reviewer inspected source and existing evidence; no implementation tests were run.

Grok's baseline paragraph says "before this pass except the five writable paths".
That wording does not independently establish their exact edit chronology. The current
hashes and retained red/green results are verified; no missing historical evidence is
invented from that sentence.

## UI still unexecuted

The screenshot command again exited 1/SIGTRAP before the harness started. No PNG,
sandbox-status or live renderer assertion exists. Grok reports no escalation tool and
an ordinary host process with Seccomp 0/NoNewPrivs 0; those host-context statements are
actor-reported. An escalation capability alone is not proven sufficient to solve it.
Do not rerun the same `--disable-setuid-sandbox` command in the same context.

One harness issue remains visible by inspection: `layoutOk()` iterates children of
every details element, including paragraphs intentionally hidden by closed details.
Those paragraphs have no rendered box, yet their zero geometry is compared with the
visible parent's position. Only the first details was opened. This can fail the smoke
for correctly collapsed rows. Preserve non-vacuous checks for visible content, test
each details open with actual keyboard input, and exclude only intentionally collapsed
descendants from closed-state geometry. Keep missing/hidden required controls failing.

## Existing-helper execution route

Read-only inspection found `/opt/google/chrome/chrome-sandbox`, mode 4755, size 15232,
SHA-256 `c100b678a8c171ad0733e51b6f18d98d936d38ab945681c41da00f2ee22e7571`.
The review tool's user namespace maps only host uid 1000, so it displays the helper
owner as nobody; host uid 0 ownership must be verified by the executor before use.
No ownership or permission change has been made or authorized.

The installed Electron executable contains `CHROME_DEVEL_SANDBOX`. Chromium documents
that variable for choosing a developer SUID helper and reports a version error if the
helper API is incompatible. The documentation warns it is largely historical, so actual
runtime success and the existing OS sandbox assertions remain mandatory:
[Chromium SUID development documentation](https://chromium.googlesource.com/chromium/src/+/main/docs/linux/suid_sandbox_development.md).

Authorize one materially different isolated run using this pre-existing helper,
without `--disable-setuid-sandbox`, after its host ownership/mode/hash checks.
This invokes the installed helper normally; it neither installs privileged code nor
changes host security policy. Do not use it from a confined agent context to bypass
tool permissions. Use the normal approved host execution mechanism when required.
If unavailable/incompatible, retain the exact blocker instead of another identical run.

## Active task

[Grok UI finish 01](../handoff/GROK_BUILD_BBD_PAY_001_UI_FINISH_01.md) is the only active
source/execution handoff. Only the UI smoke harness and demonstrated inbox CSS fixes
may change. Payment transport, IPC and other source are frozen. Hermes final acceptance
and Git publication remain unauthorized until actual UI evidence is reviewed.

Reviewer governance changes are exactly this review, UI-finish handoff, CURRENT_TASK
active prefix, PAY-001 ticket status and integration status map. This records the next
decision in the repository control plane before execution.
