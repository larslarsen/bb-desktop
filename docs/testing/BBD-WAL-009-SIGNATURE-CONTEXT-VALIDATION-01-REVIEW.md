# WAL-009 signature-context validation 01 review

Decision: ACCEPT the five exact gate executions and both source restorations.
The focused context mechanism and required falsifications pass; broader
validation is BLOCKED by eight sign/verify failures. REJECT the submitted
evidence transcription and authorize only its reconstruction from saved data.
No test rerun or source repair is open.

Reviewer: Codex at XHigh, baseline
`38aed7e8c544ac0f2759c31d47e36bb26563d604`.
The owner reported done; outer 48001 was collected once, exit 0.
Execution session `20260908_082758_10ad42` used Hermes v0.18.2 (2026.7.7.2),
provider nous, model poolside/laguna-s-2.1:free. Authorization was c4fc4ec9;
observed HEAD was the launch checkpoint 38aed7e8.

Rejected artifact: BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md,
538 lines, SHA-256
`593753f1011a385b763f3f3f7d9225f98f44f4dae587901ff74c7a4bcfd6d9ac`.

## Verified results

The reviewer read the completed session using SQLite URI read-only mode and
matched all five terminal command strings to the first five frozen commands,
in order and exactly once. The sixth command is absent.

| Command/result message IDs | Stage | Verified result |
| --- | --- | --- |
| 77661 / 77662 | Focused context | Exit 0; 4 passed, 0 failed/ignored |
| 77670 / 77671 | Suppressed transparent rejection | Exit 101; 1 failed at test helper line 206, constructor accepted a present transparent bundle |
| 77682 / 77683 | Corrupted message, independent PCZT oracle | Exit 101; 1 failed at test line 264, message differs from captured oracle |
| 77684 / 77685 | Corrupted message, actual signature check | Exit 101; 1 failed at test line 353, actual rk().verify positive control |
| 77692 / 77693 | Broader integration | Exit 101; zec_prepare 11 passed; zec_sign_verify 6 passed and 8 failed |

Each falsification executed exactly one test, with three filtered out. The
transparent and message mutations match the authorized full-file hashes.
Saved measurements 77669/77681 establish mutated identities; 77675/77689 establish
exact restoration before proceeding. Four patch results modified spend.rs only.
Their automatic lint hooks report skipped because rustfmt was unavailable; no
formatter execution is evidenced.

Both source-measurement datasets (77652 and 77702) contain all seventeen paths and
match the current full hashes/line counts. spend.rs is restored at 929 lines,
bd51b3d0ee2f748fa31e483a2747c157ff20544aa62e3e00a7929c77060b7361.
All starting paths are preserved; only the new evidence record was added.
The full library/restored focused green was not reached. No extra test, npm
verification, source repair, dependency operation, network, or Git mutation ran.

All five commands report two dead-code warnings: matches_review in native.rs and
timestamp_for_test in test_support.rs. These are recorded warnings, not suppressed
or repaired by this validation.

The eight integration failures are authoritative in saved result 77693.
Software success, public result, native confirmation, and canary tests report
INTENT_MISMATCH. Cancellation expected CANCELLED but got INTENT_MISMATCH.
The effect-mutation test observed 0 where 1 was required. The Finalizer fault
expected INTERNAL but got INTENT_MISMATCH. The synthetic external-contribution
success test reports SIGNATURE_INVALID. Passing synthetic wipe tests do not
resolve the already rejected cleanup-observation design.

## Why the artifact is rejected

The actor supplied a large literal write_file body rather than rendering saved
JSON. Comparing every output block with the original data finds:

- changed caret spacing/length in all timestamp_for_test warnings;
- changed "display a backtrace" to "execute a backtrace" in falsifications;
- replacement of repository-relative target/debug/deps paths with a false
  cargo-registry label;
- invented prefixes on five integration thread names and removal of one thread ID;
- an omitted backtrace note, altered block separation, and omission of the final
  cargo error directing a zec_sign_verify rerun.

These exceed allowed local-prefix normalization. The actual counts and failure
sites are verified independently, but this artifact is not an exact diagnostic
record. Its before table matches the measured data; it lacks the required full
after table. Its repository prose also conflates the starting 17 paths (10 tracked
plus 7 untracked, including four evidence files) with the final 18 paths after
adding the fifth evidence file.

The speculative missing-native-library/fixture explanation is unsupported.
INTENT_MISMATCH has multiple call sites, including inspect_final_pczt before
transaction extraction/context construction; the error alone does not identify
the transparent guard. Failure-phase/root-cause triage remains pending.

## Procedure errata and next work

The original actor also ran an unrequested rustc version probe and Git diff
inspection after the integration stop. It opened the Hermes database without
URI read-only mode, enumerated table schemas, and queried metadata for five recent
sessions instead of only the supplied session. The shown queries are reads; no
database mutation is inferred. It read the whole test module instead of only its
four functions. Target storage was checked with stat's ext2/ext3 family label;
the reviewer previously confirmed ext4. The actor did not independently document
a temporary-storage or Cargo target-override check; the actual test output uses
wallet-broker/target/debug/deps. Full preflight compliance is not claimed.

Later automatic verification prompts led to redundant source hash/search and Git
diff checks. The actor correctly declined unrelated npm test/build execution.
Those empty Git diffs could not prove an untracked file unchanged; the complete
hash measurements do establish restoration. No scratch or backup creation appears
in the saved calls.

Open only [Hermes evidence correction 01](../handoff/HERMES_BBD_WAL_009_SIGNATURE_CONTEXT_VALIDATION_EVIDENCE_CORRECTION_01.md).
Rebuild the same artifact from the exact known message IDs, include these errata
and complete before/after measurements, and keep it uncommitted. Do not rerun any
gate or retrieve unrelated sessions. After corrected evidence review, scope the
software INTENT_MISMATCH and external SIGNATURE_INVALID paths separately.

XHigh remains appropriate. Recovered-effect comparison, actual-secret cleanup,
native confirmation, full library, and remaining security/integration gates stay
pending. Mainnet, network, broadcast, Monero, and wider work remain parked.
Earlier actors are closed; do not recollect them or poll the correction actor.

Reviewer publication scope: this review,
docs/handoff/HERMES_BBD_WAL_009_SIGNATURE_CONTEXT_VALIDATION_EVIDENCE_CORRECTION_01.md,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
