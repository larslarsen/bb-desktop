# WAL-009 decoded effects regression: source acceptance and expected red

Reviewer: Codex, XHigh. Actor: Jr Dev — Hermes, execution/evidence only.
Grok source authorization: 2a098115; launch checkpoint: 3ea994fb.
Grok session 7ddfcee6-c773-4d03-af90-51da48895a33, outer 32613, collected once
following owner done, exit 0. Runtime grok-4.6-build High.

## Source acceptance

ACCEPT the one-file test drop for the single expected-red execution below.
wallet-broker/src/zec/spend/verification_context_tests.rs: 757 lines, SHA-256
791e8cbb77d556ab05e35946e23cddbc3b5a27659c3ce7b5e2464a665210fda4.

All 12 saved search_replace operations affect that file only. Reversing them in
memory reproduces the exact 528-line starting SHA-256
45f3bdf2f1d08924d53c298517c631427ab052c2d4955b31e98aa555386d380a.
The net change is private helper context plus one appended 217-line test. Prior
test bodies and signing/proving/serialization semantics are unchanged. The actor
used read-only source/API inspection and three measurement/discovery terminal
commands; no test, compiler, formatter, Git, network, evidence or other write.

The regression constructs one real signed fixture, recovers outputs per action
with scoped outgoing keys, establishes internal change by incoming decryption,
derives fee from decoded value balance, and normalizes the recovered memo. It
checks a positive production-verifier result against that independent oracle.
All ten false metadata claims are then passed as agreeing actual/expected copies
over unchanged signed bytes. Every outcome is collected before the final
assertion at line 756, whose diagnostics contain only case labels and codes.
The replacement receiver is a valid different Orchard-only address. Counts and
pool cases supplement the four payment-field cases without new proof generation.

This is acceptance of test source, not a claim of execution, full recovered-spend
proof, change-attack coverage, or end-to-end publication/capability integration.
The architecture in the
[completed Grok handoff](GROK_BBD_WAL_009_DECODED_EFFECTS_TESTS_01.md)
continues to govern the later production repair. Do not reopen source authoring.

Reviewer also verified the other 25 frozen inventory rows and the new layout
evidence: all match. spend.rs remains 1046 lines, SHA-256
8b70ecf7dea541d951184d89b1ed2d917ce5c3c6dceabb73151840c79517328a.
Layout evidence remains 354 lines, SHA-256
ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43.

## Authorized paths and preflight

Hermes may create only:
docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md.
No source, test, dependency, existing evidence, governance or Git mutation.
Use the existing wallet-broker/target. Reviewer measured its filesystem as
ext2/ext3 (disk-backed) with 192 GiB free; local temporary storage is tmpfs.
Do not relocate caches or create large temporary artifacts.

Read AGENTS.md, TESTING.md, this handoff and the active CURRENT_TASK.md prefix.
Record hermes --version once and the actual provider/model/runtime session ID
from this run, using only relevant runtime metadata. Do not dump configuration
or credentials. No repeated version/Git/directory/transcript discovery.

Measure the frozen inventory once before the test and once afterward. Use the
26 rows in HERMES_BBD_WAL_009_NATIVE_LAYOUT_VALIDATION_01.md, replacing only the
verification_context_tests.rs row with the accepted 757-line identity above,
and add the layout evidence row above (27 existing paths total). A single
read-only script per measurement may parse that table and calculate line counts
and SHA-256 values. Report actual full measurements; stop before execution if
anything differs. The new expected-red record must not already exist.

## One authorized acceptance command

Execute this exact command ONCE in the repository root:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests::decoded_effects_reject_agreeing_metadata_that_disagrees_with_signed_bytes -- --exact
```

Expected result: exit 101, one executed test fails at its FINAL ten-case outcome
assertion after successful independent recovery and positive verification. The
current production source should wrongly accept all ten claims. Record the exact
actual diagnostic and test counts; do not invent a count or infer an unreached
assertion. A compile, fixture, recovery or signature error is an unexpected stop,
not accepted red. Unexpected green is also a stop. Do not fix or retry anything.

Launch through the terminal tool once with background execution so its complete
output is retained. Wait only on that process, at most 60 seconds per wait. After
completion use process.log to retrieve the complete saved output, paging if
needed and checking reported line counts. A wait-result tail is not a complete
log. Preserve the launch/command/process/completion identifiers and exit code.
No repeated command, formatter, check/no-run probe, broader tests, mutation,
falsification, dependency/network operation, Git, product, actor or subagent.

## Evidence and stop

After the final identity measurement, write the one named record with runtime,
authorization/launch lineage, the 27 measured identities, exact command and
process/result, and complete saved compiler/test output. Render the output block
directly from saved tool JSON; never reconstruct output or fill in a missing
prefix. If full output cannot be retrieved, say precisely what was retained and
stop; do not rerun to repair evidence. Normalize local absolute paths in evidence
and disclose that normalization. Do not report procedural compliance that the
actual transcript does not support.

Measure the new record's full SHA-256/line count once after writing; do not embed
its self-hash. Then stop with the outcome, failure assertion, record identity and
runtime session ID. No evidence-only correction or additional verification loop.
Hermes does not accept its own result. Reviewer will independently review it.

All implementation and ten prior evidence records remain uncommitted. No source
integration, publication of a wallet flow, broader WAL-009 acceptance, or network/
broadcast/mainnet/hardware/Monero work is authorized. Keep reviewer XHigh for the
next effects review. Reviewer publication scope: this handoff, CURRENT_TASK.md,
and tickets/BBD-WAL-009.md only. Launch once; collect after done; no actor polling.
