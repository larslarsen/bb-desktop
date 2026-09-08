# WAL-009 signature-context validation evidence correction 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
EVIDENCE ONLY. All test execution, falsification, source, and integration
authorizations are closed. No gate or compiler/version probe may be rerun.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
the first 60 lines only of docs/handoff/CURRENT_TASK.md, and
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01-REVIEW.md.
The original validation handoff's seventeen-row baseline table may be parsed for
identity checks only; its commands are not authorized. Do not read source bodies,
historical task logs, dependency files, or other evidence.

## One writable path and frozen inputs

Only writable path: docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md.
Starting identity: 538 lines, SHA-256
`593753f1011a385b763f3f3f7d9225f98f44f4dae587901ff74c7a4bcfd6d9ac`.

Measure that file and all seventeen paths from
HERMES_BBD_WAL_009_SIGNATURE_CONTEXT_VALIDATION_01.md before editing. Those
seventeen paths must retain every full starting identity; spend.rs is 929 lines,
bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361, and the context
test module is 528 lines,
45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a.
Stop on mismatch, a staged index, or any unexpected file; do not repair anything.

Allowed read-only Git preflight/final checks: git rev-parse HEAD,
git status --short --untracked-files=all, and git diff --cached --name-only.
There are eighteen pending paths for this correction: the original seventeen
plus the evidence file. Record one hermes --version result for this correction's
runtime; this is the only new version command authorized. Use the supplied current
session ID for correction metadata, keeping it distinct from the original run.

## Retrieve exact saved data

Original execution session: `20260908_082758_10ad42`.
Open ONLY the known Path.home()/".hermes"/"state.db" with SQLite URI read-only
mode: use its as_uri() plus "?mode=ro" and sqlite3.connect(..., uri=True).
No plain SQLite connection, schema enumeration, directory discovery, recent-session
query, configuration, credentials, prompts, or unrelated session retrieval.

The known sessions columns are id, model, billing_provider. The needed messages
columns are id, session_id, role, tool_calls, content. Query only the original
session and these exact message IDs; query only the correction's own supplied ID
for its runtime/version/final measurement records.

| Input | Message IDs |
| --- | --- |
| Original Hermes version | 77645 |
| Original observed HEAD, starting status, empty index | 77646, 77647, 77648 |
| Original target stat and rustup-path check | 77649, 77650 |
| Full starting measurement JSON | 77652 |
| Focused command/result | 77661, 77662 |
| Transparent patch, measured mutation, command/result, restoration measurement | 77666, 77669, 77670, 77671, 77675 |
| Message patch, measured mutation, oracle command/result | 77678, 77681, 77682, 77683 |
| Signature command/result, restoration measurement | 77684, 77685, 77689 |
| Broader integration command/result | 77692, 77693 |
| Full ending measurement JSON | 77702 |
| Final status after evidence creation | 77713 |

Assistant tool_calls are JSON arrays containing function.name and
function.arguments (the arguments are JSON strings); parse the command field.
Tool content is JSON with output and exit_code. Measurement 77652.output is a JSON
array; 77702.output is a JSON array followed by ALL_MATCH: True. Parse the array
with JSONDecoder.raw_decode and preserve the trailing assertion separately.
Use measured keys path, sha256, lines_newline/newlines. Do not substitute the
expected values embedded in the ending measurement's comparison fields.

## Rebuild directly, never transcribe output

Rebuild the same evidence file using a Python renderer that reads and parses
those database rows and writes the resulting text directly to the authorized
path. Do NOT use write_file with a manually composed diagnostic body. Do NOT paste
saved-output text or expected hash tables into renderer literals. Static headings
and short classification/errata prose are allowed; commands, version output,
diagnostics, exit codes, counts, and measured tables must come from parsed data.

Include five complete command/result blocks in original order. Keep the combined
integration output as ONE complete block, including both targets, their separator,
every thread name/ID, caret, note, and final cargo error. Counts are 4/0 focused,
three required 0/1 falsification results, 11/0 prepare, and 6/8 sign/verify; verify
these against the parsed summaries rather than constructing summaries by hand.

Only replace literal local prefixes with <repo>/, <cargo-registry>/, or <home>/.
Do not change already relative wallet-broker/target/debug/deps paths or label them
as registry paths. Preserve every other output byte. Add only the newline needed
before a closing code fence when the saved output lacks one. Render submitted
command strings verbatim from their saved arguments, including quoted HOME syntax.
Use normalized tokens in prose; no local absolute installation path.

Render a full seventeen-row before/after table with both actual SHA-256 strings
and both newline counts. Include the two actual mutated identities and the two
actual restoration measurements. Record authorization c4fc4ec9 separately from
observed HEAD 38aed7e8. Original runtime is taken from the original session and
77645; add a clearly separate correction-runtime/parent section.

State that five commands executed, with the full library command unrun after the
8-failure integration stop. This is partial validation: focused context and
falsifications verified, prepare passed, sign/verify blocked, restored full-library
green outstanding. Do not mark an entire broader stage complete because it stopped.

Apply the review's authoritative errata: correct the starting/final path counts;
disclose the altered prior transcription, out-of-scope database/read/after-stop
inspection, missing independently recorded preflight checks, and redundant final
checks. Remove speculative native-library/fixture/root-cause claims. The actual
failure phase remains unresolved; no source investigation or repair is authorized
to this correction actor. Do not claim full original procedural compliance.

Before reporting, re-read the generated file and programmatically compare EACH
of its five output blocks with its corresponding normalized saved output, allowing
only the closing-fence newline. Also verify all five command strings, exit codes,
and every before/after table row against parsed data. Print concise comparison
booleans and the evidence's final hash/line count. A mismatch or missing saved
record is a stop; never regenerate execution to repair evidence.

No scratch, backup, renderer script file, other artifact, or edits to older
records/governance. Keep the renderer in the terminal invocation and data in memory.
All eighteen existing pending paths except this evidence file must remain
byte-identical. Do not embed the evidence's own changing hash inside itself.

No tests, source patches, compiler, formatter, npm/Cargo, lint, scanner, network,
native launch, dependency operation, other actor, or Git mutation. Leave the
corrected record uncommitted. Report original stage results/unrun library gate,
correction runtime/session ID, exact-data comparison results, preserved source
identities, final evidence identity, and stop. The reviewer collects once only
after the owner reports done; no polling or duplicate actor.
