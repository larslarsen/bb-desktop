# Hermes Handoff — BBD-WAL-007 Phase-C Slice 4 Green Resume 04

You are **Jr Dev — Hermes**. Protected governance parent is the commit containing this
handoff.

Read `AGENTS.md`, `TESTING.md`, Green Resume 03 Stop Review 01, Format Correction 02
Source Review 01, the complete prior
`HERMES_BBD_WAL_007_PHASE_C_SLICE_04_GREEN_RESUME_03.md`, and `CURRENT_TASK.md`.

Resume 03 applies verbatim, including every identity, command, expected result, stop
condition, evidence requirement, and integration scope, with only these replacements:

- protected parent becomes the commit containing this handoff;
- `wallet-broker/src/xmr/test_support.rs` becomes 4,774 lines at
  `055af3ba8b55cb68bd87c56cb23d6050aca6b24dba47cff4372f37cd634de17b`.

Every terminal command must be a separate tool call that completes before the next is
launched. Do not batch independent commands, wrap, chain, redirect, pipe, alter, or
repeat them. On the first mismatch, restore any temporary falsification, run only
minimal read-only identity proof, and stop without evidence, staging, commit, or push.
Integrate only on exact success. No source/test authoring, repair, another actor, Slice
5, broader/final acceptance, or real local-Monero gate is authorized.
