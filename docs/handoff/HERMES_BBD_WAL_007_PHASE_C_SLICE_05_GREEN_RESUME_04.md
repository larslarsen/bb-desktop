# Hermes Handoff — BBD-WAL-007 Phase-C Slice 5 Green Resume 04

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role policies, `CURRENT_TASK.md`, Green
Resume 03 and its rejection, Clippy-Correction-01 Source Review 01, and every complete
record/source/test path required by Green Resume 03.

Run the complete Green Resume 03 contract from its beginning. Every exact command,
expected result/count, falsification, restoration, frozen identity, evidence rule,
eleven-path staging scope, commit message, push rule, prohibition, and sensitive-output
restriction is incorporated unchanged except the two identities below:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/receiver.rs` | 868 | `daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9` |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc` |

All other Resume-03 source/test/draft and committed/frozen identities remain exact and
mandatory. The temporary falsification must restore `receiver.rs` to the new 868-line
identity above.

Do not reuse any prior result. Submit every fenced execution command as the terminal
command string byte-for-byte, alone, once, and sequentially. Use the tool-returned exit
code; append no `echo`, redirection, pipeline, wrapper, shell operator, or other text.

Stop immediately on the first mismatch. After a mismatch, the only permitted terminal
command is one single receiver line-count/SHA-256 proof after restoring the temporary
falsification, if restoration was necessary. Do not rerun the failed command and do not
run Git/status/diff, another inspection, evidence, staging, commit, push, or later gate.

Only after every exact success may you replace the frozen stop draft, update
`CURRENT_TASK.md`, stage the exact eleven Resume-03 paths, commit exactly
`feat: add BBD-WAL-007 Monero viewing and receivers`, and push `master`. Then use only
the authorized read-only post-push proof and stop for reviewer acceptance.
