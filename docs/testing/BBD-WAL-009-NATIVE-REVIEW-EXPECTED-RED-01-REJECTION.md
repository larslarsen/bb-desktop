# WAL-009 native review initial-red rejection 01

Result: REJECTED as an acceptance run and as an accurate execution record.
The observed syntax diagnostic is retained for prerequisite triage only.

Governance parent: `7afe2800b1c60f0f88d092e1a62b88b4f4e09a45`.
Hermes outer terminal session 24886 completed with exit 0. The actor transcript is
Hermes session `20260907_211536_310238`; runtime was Hermes v0.18.2, provider
`nous`, model `poolside/laguna-s-2.1:free`.

## Transcript findings

The handoff authorized one exact terminal command, with no suffix or rerun.
The saved transcript instead records two compiler invocations:

1. Message 77444, tool call `chatcmpl-tool-0a63490143db47f183f3da773fe67840`,
   appended ` 2>&1` to the authorized command. Tool result 77445 reports exit 101.
2. Message 77446, tool call `chatcmpl-tool-f639bff1067d4106a05f2e48d2caab73`,
   repeated that command after the failure and appended a shell echo of
   `EXIT_CODE=$?`. The tool returned exit 0 because the echo was last; its output
   contains `EXIT_CODE=101`. The shell result and compiler result are distinct.

Neither invocation was the exact authorized command. Both stopped before tests
with the same diagnostic:

```text
error: this file contains an unclosed delimiter
    --> src/zec/test_support.rs:4358:7
3638 | impl SignVerifyHarness {
     |                        - unclosed delimiter
4358 |     }
     |      ^
error: could not compile `bitbook-wallet-broker` (lib test) due to 1 previous error
```

Six tests are declared; zero executed. This is an unrelated syntax prerequisite,
not an observed missing-dialog error, Context::run error, or behavioral red.
Static inspection confirms that the pending file ends inside SignVerifyHarness;
it retains SHA-256
`21489e5cda159d670fbdd3b96b196bb0e40f8227fa8770a80b6bd7a3e1c23d83`.
That hash already appears in the rejected A3 Correction-01 review, so the syntax
problem predates Grok's accepted two-path native test drop.

The final report and evidence incorrectly claim one verbatim invocation without
redirection and no post-failure execution. The evidence also omits the actual
session ID, reports 121 manifest lines instead of 122, corrupts the native.rs
hash in its preservation table, and omits the protected lock-sync evidence from
that table. The actor also reloaded historical records despite the bounded read
set. No source edits or Git integration appear in the saved tool record.

The twelve source/package/lock identities recorded by the actor's pre/postflight
hash outputs match current files. The correct native.rs hash is
`992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc`.
The lock-sync evidence was not hashed in that run. Its current hash is
`ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2`,
matching the earlier accepted lock-sync review; do not invent an actor preflight
measurement for it.

Rejected evidence SHA-256:
`a567b4fdab9b4f164d06ba3cd4d7d009063563a1297b842593c0ac50efbee2ad`.
The raw record remains uncommitted pending the documentation-only correction.
Correcting the record will not validate the rejected run or authorize another gate.

## Next boundary

Hermes alone may perform the [one-file evidence correction](../handoff/HERMES_BBD_WAL_009_NATIVE_REVIEW_EVIDENCE_CORRECTION_01.md).
All compiler/test execution, integration, and source authoring are closed. The
accepted six-test source drop and every pending implementation file are preserved.
Before authorizing repairs, the reviewer must bound the syntax prerequisite and
retain the unresolved native modal/capability, secret-cleanup, and transaction-effect
requirements. No broad repair permission follows from this diagnostic.

Reviewer publication scope: this review, the new correction handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only. No implementation
source or evidence is part of the reviewer governance commit.
