# BBD-WAL-007 Slice-2 Owned-Child Test Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `38c4fac2`

Result: **TEST CORRECTION ACCEPTED — HERMES EXPECTED RED AUTHORIZED**

Sol edited only `wallet-broker/tests/xmr_process.rs`. `HEAD == origin/master ==
38c4fac2`, the index is clean, the test is 374 lines with SHA-256
`12cb52a5efca6a5ebfa53b1e856fc816c5ae7e8e01849b9034bd11d5a74d6f06`, the count
remains exactly 12, and `git diff --check` is clean.

The diff contains exactly the three accepted substitutions:

- the test names exact owned-child kill and reap rather than process-group kill;
- the expected teardown operation is `kill-exact-owned-child`; and
- the ownership assertion is `killed_only_owned_child()`.

The three hostile child outcomes, complete teardown order, zero child/handle results,
and every other test assertion remain byte-identical. The corrected contract matches the
ticket's permitted exact-child branch and stable Rust 1.98 `Child::kill`; it adds no PID,
group-signal, helper-process, dependency, FFI, unsafe, nightly, or fake-success authority.

The reviewer ran no formatter, compiler, test, Cargo, Node, build, binary, network, or
acceptance command. Only the focused Hermes expected-red handoff is authorized.
Production source and every later gate remain closed.
