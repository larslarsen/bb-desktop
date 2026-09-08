# WAL-009 native layout repair source review

Decision: ACCEPT for grouped validation. High remains sufficient.
Grok session 1e93d0db-3e54-4d88-a7e3-ce1c35a7bd9a, outer 38040, collected once
after done, exit 0; runtime grok-4.6-build High. Authorization f2c942b8,
launch checkpoint b9c1ce8c. No reviewer test/compiler/formatter was run.

Two saved source replacements account for the entire change. Reversing them in
memory reproduces the exact 306-line starting hash b0d4da8770a400315889e5d69ad830e3e08d707c5bc62ca1f6e633798703349c.
Final native_ui.rs: 321 lines,
d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960.
All 25 other source/evidence identities match. The transcript shows source edits,
read-only inspection and measurements, without tests, compiler, formatter, Git,
evidence, or other source writes.

The repair reserves two button heights from font/padding/minimum interaction
metrics plus spacing. Remaining height is clamped nonnegative and bounds the
vertical review ScrollArea. Every immutable field remains in the renderer; both
buttons remain outside the scroller. The stable scroll ID, existing labels and
actual response rectangles are retained. Confirmation/cancellation transitions,
App/runner, native authority, and all ten tests remain byte-identical.

Open [grouped Hermes validation](../handoff/HERMES_BBD_WAL_009_NATIVE_LAYOUT_VALIDATION_01.md):
ten UI tests green, isolated unbounded-body falsification, isolated programmatic-
close re-entry falsification, restored ten-test green, and native compilation.
Exact mutation hashes were calculated in memory without changing source:
cf18eb0a012fb485ec091f6a142a34be479b6368516e51e7c73de59e1c63a366 and
3788efe82a143601e7197f2769180c2972002ca4ea090b11056c7ef01db1a9bc.
No additional speculative tests or old expensive suite repeats are authorized.

Implementation remains uncommitted. This does not close OS/capability integration,
independently recovered effects, actual-secret cleanup, or remaining security.
The owner requested continued wallet work with less churn, not a new demo scope.
Reviewer publication scope: this review, linked handoff, CURRENT_TASK.md, ticket.
Launch once; collect after done, with no actor polling.

## Capacity checkpoint

Hermes outer 10169 collected once after done: upstream HTTP 429 after stage 1.
Completed session 20260908_145855_f82684, nous/poolside/laguna-s-2.1:free. Saved
78608/78610 launched the exact first command once; 78612 exited 0 with all ten
tests passed, 6 filtered, 0.15 seconds. Complete saved output is accepted. No
mutation or later command was reached; all 26 identities match and evidence is
absent. Resume the same task once for stages 2-5/evidence, without stage-1 repeat.
The existing handoff records the preflight path typo and clamped wait. High remains
sufficient; provider capacity does not call for a reasoning change.
