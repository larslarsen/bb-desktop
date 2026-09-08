# WAL-009 native review production 01 source review

Decision: ACCEPT the one-file source drop for grouped native validation.
Reviewer: Codex at XHigh. Review baseline: 434d62f25b34c7ba8232cfefd6df28ec8fba87a8.
No compiler, test, formatter, or acceptance command was executed by the reviewer.

Grok session 025c4563-cfb2-444f-8885-575bfdc4266a, outer 98275, was collected once
after done, exit 0. Source authorization 21cefbc4, launch checkpoint 434d62f2;
CLI grok-4.6 High, saved runtime grok-4.6-build High.

The saved initial native_ui.rs read, with line-number decoration removed in
memory, reproduces the 201-line authorized starting SHA-256
600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524.
The actual final file is 303 lines, SHA-256
c6d5fc3a4dee46f4700f2a3ae5866a5003746eab5aa6dd6bc20e5ba5c6d726b4.
Diff inspection confirms only imports, private Zcash dialog/runner types, removal
of the two stale private approval fields, and the Zcash surface implementation
changed. Password, restore/result, immutable field rendering, file pickers, and
the test-module declaration remain byte-identical. All twenty-one other pending
source/package/lock/evidence identities match the accepted preceding baseline.

The transcript contains one file write and two subsequent source replacements,
three shell calls (initial/final identities and local dependency location), and
read-only inspection. There was no test/compiler/formatter/Cargo/Git execution,
other write, launch, or integration. Some read-only inspection exceeded the
requested narrow bounds: CURRENT_TASK's historical suffix, wider ticket/native
ranges, crate-wide usage/Clippy searches, and a rustfmt configuration read. Record
this scope deviation without a source correction or another actor cycle.

The dialog owns an immutable full review. Pending alone can confirm; cancellation
is terminal, consumption drains once, and later clicks cannot rearm the instance.
The real button responses supply the frozen tests' hit rectangles. Close/Escape
and Cancel are evaluated before the confirmation transition in the same frame.
No-input frames and unrelated clicks do not change the pending state.

The eframe App delegates to that same dialog UI. Each surface call creates fresh
Rc/RefCell state and synchronously runs the native event loop with run_and_return
explicitly true. A private closing flag preserves an accepted decision through
its programmatic window close. The surface drains only after successful return
and compares the complete review, not its claimed hash. Errors/unwind panics deny;
no GUI worker, any-thread override, public approval injection, or capability mint
was added. native.rs and the six pre-existing real-widget tests are unchanged.

Open [Hermes native validation 01](../handoff/HERMES_BBD_WAL_009_NATIVE_REVIEW_VALIDATION_01.md):
six widget tests green; suppress the actual Confirm transition and require the
owned-review assertion to fail; give Confirm priority over close and require the
same-frame close assertion to fail; restore and rerun six widget tests; run the
17-test native-surface regression suite; compile the native feature. The two
isolated 303-line mutation hashes were calculated in memory, without source writes:
fc406fcf71021b1c08b8402e565ffcfc7e23b016ba366888324f5579bfdd6cca and
4fa5e01f20de8abe1d7a737fcdeaf162d46af3c34fcf79c7fc4989253a9f7e9a.

This source review is not behavioral or OS acceptance. Widget tests use synthetic
input and a larger screen; actual-window layout with real-length review values,
platform lifecycle, owning-thread use, and capability-bridge integration still
need their separate native integration gate. Retained-spend/signature-context
results remain accepted without rerunning their expensive suites for this change.
Full independent recovered effects, actual-secret cleanup, remaining security,
integration, mainnet, network, broadcast, hardware, and Monero remain unaccepted
or parked. No broader WAL-009 acceptance or implementation integration is opened.

Reviewer publication scope: this review, the linked new Hermes handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. Launch once; collect
after done/explicit collection. No actor polling. XHigh remains appropriate.
