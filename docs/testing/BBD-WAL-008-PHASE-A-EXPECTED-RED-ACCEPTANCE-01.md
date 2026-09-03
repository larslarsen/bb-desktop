# BBD-WAL-008 Phase-A Expected-Red Acceptance 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Integrated source/evidence commit: `eda78545`

Evidence-correction commit: `2eb897b0`

Result: **PHASE-A TEST CONTRACT AND EXPECTED RED ACCEPTED**

Hermes's fresh Rust 1.98 formatter check exited 0 without mutation. The exact focused
test then exited 101 before any test ran, solely because the new production hardware
module and 16 test-support items plus `HardwareDecision` do not yet exist. This is the
required absent-contract red, not a dependency, lockfile, syntax, existing-source, or
network failure.

The integrated scope is exactly the 117-line manifest at SHA-256
`7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530`, the
794-line test at `32959949c9da01834fe10ab1328777ab906fb9f8c7bc3e8ef66945f6961ad7a7`,
the evidence, and current-task record. The one mistyped evidence digit was corrected
without source or execution. `HEAD == origin/master == 2eb897b0` and the worktree is
clean. The reviewer ran no formatter, compiler, test, build, or product command.

This acceptance proves only a valid red test contract. It does not authorize or claim
production hardware support, persistence, transport, signing, PCZT exchange, or a real
device pin.
