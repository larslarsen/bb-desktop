# PAY preload policy verification review 19

Reviewer: Codex. Decision: ACCEPT the bounded preload policy correction. PAY functional
and boundary verification is complete for the reviewed working-tree inputs; prepare the
final publication snapshot/secret scan. This is not release or Git publication acceptance.
No tests, builds, syntax, scans or live RPC executed by reviewer.
HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; master; empty index.

## Captured evidence verified

[Hermes verification report](BBD-PAY-001-HERMES-PRELOAD-POLICY-VERIFY-01.md) has the
complete automatic capture under dist/pay001-policy-verify01/. executor.py matches the
handoff block byte-for-byte. All ten command metadata records have the authorized timeout
argv, ordered real start/end times, expected observed exits and matching raw-log hashes.
The run spans 2026-09-16 07:32:04–07:32:06 UTC, without timeout/forced termination.
All 21 before/after source hashes and line counts match each other and the current tree.
Git before/after facts agree. Copied test/preload files were unchanged; only the two policy
literals were removed for falsification, yielding the exact old policy hash. Restored/
final copy hashes match the real source. Original sources were never mutated.

| Check | Observed outcome |
| --- | --- |
| Two syntax checks | exit 0 |
| Three focused boundary/PAY groups | 3 pass, exit 0 |
| Isolated positive group with old five-channel list | 1 expected failure, exit 1 |
| Isolated real preload against old policy | unlisted IPC PolicyError, exit 1 |
| Restored three focused groups | 3 pass, exit 0 |
| Repository checker | exit 1, Rust source inventory missing/extra |
| Full policy suite | 90 pass / 4 fail, exit 1 |

The first three full-suite failures now have the inherited root Rust inventory cause;
the fourth retains the eight-versus-fourteen ZEC inventory mismatch. No PAY IPC failure
remains. All four stay release blockers. The actor's paraphrase of the positive failure
is not literal stderr (actual output is a deep-equality diff). Hermes reports execution;
this reviewer record supplies acceptance, not the actor report's self-acceptance sentence.

## Coverage and historical evidence disposition

The original ticket's maintained-source requirement is satisfied by
`test/electronSecurity.node.js`'s exact six-path injection/eval/javascript-sink scan,
including social/payment-inbox.js and wallet-pay/inbox-client.js. The retained earlier
32-case security run explicitly passes that group. This resolves the new-module scan-
coverage question recorded in reviews 17/18; no extra scanner/source change is required.
It is a bounded dangerous-sink check, not a claim of exhaustive static analysis.

Accept prior Node/boundary proof and review-16's completed UI artifact/screenshots for
this read-only Messages slice. Earlier missing metadata, rejected UI prelaunch claims
and actor-reported audit exits remain historical limitations; do not reconstruct them.
Their absence does not erase the independently inspected raw assertion/output artifacts.
No unchanged UI/application-suite/audit rerun is required. Final secret coverage must use
the actual candidate bytes because policy/tests and governance changed after scans01.

## Publication scope decision

The candidate contains the 13 PAY source/test/fixture paths, both complete current policy
files and the two tested package inputs. The package delta removes Leaflet and pins the
already-resolved MapLibre 6.8.0; it changes no retained MapLibre/Electron version. This
follows the recorded keep-MapLibre/remove-Leaflet decision and is needed to publish the
exact package/policy/test inputs already reviewed and exercised, rather than a different
combination. Older policy extractor/manifest/package checks are preserved as accepted
work, supported by the linked WAL-009 reviews. No new policy capability is introduced
by selecting these complete tested files.

Exact prospective paths are enumerated in
[publication paths](../handoff/BBD_PAY_001_PUBLICATION_PATHS_01.txt).
PAY governance/history plus the three supporting WAL-009 records are included. All
unrelated Rust edits, wallet tickets, role-policy changes and private/generated artifacts
remain outside this set. Only a read-only snapshot and scan are authorized now. The
snapshot is not a clean full-repository checkout or proof of a green release candidate;
inherited Rust failures and the historical evidence limits remain explicit.

## Key artifact identities

Relative to dist/pay001-policy-verify01/:

| Artifact | SHA-256 |
| --- | --- |
| executor.py | f9e37c4839fff4a92cd2b0f664a77fb793fc47cb4153ce1ac602401e1b048c4e |
| source-before.json and source-after.json | d7a2dc9eeed21dfbbeef5bc5d93e993c48f6ab89da033557dc459785ed59114a |
| 04-focused-green.stdout.log and 07-restored-focused.stdout.log | ddc6220dcf704b26b38bebcf2935e14e6789c5dd3334f80cc248d9db4a58acf2 |
| 05-falsified-positive.stderr.log | 764f7945ba6033e807e2140b619aa7628d39a69cf138495e8ac333b6ff328a42 |
| 06-falsified-real-preload.stderr.log | 71b1b68020076fabe5a00c91e6da914785c4f1bb3e8c270e8532a782dae7e3e9 |
| 08-repository-policy.stderr.log | e9da1bab82a534bff2c127778580a7a8b00c5090d4211c781500ec3d8d56ae49 |
| 09-policy-suite.stdout.log | 1c1ad4434b8381ac28a62bef34a15a1e12434a554fd3c4a2eec69c17bd00c6ff |
| 09-policy-suite.stderr.log | 855a09bbfe113b771afc59f610a78de909b038239e783ce879539cd7784544ae |

Next authority: [Hermes publication preparation 01](../handoff/HERMES_BBD_PAY_001_PUBLICATION_PREP_01.md).
Only new final-byte snapshot/secret capture; no staging/commit/push, source repair,
UI rerun, owner test, restart or live monerod operation. The synced-node report and
identity/WebRTC/video direction stay recorded. Reviewer will inspect immutable candidate
identities before giving an exact publication instruction.

Governance: this review, preparation handoff/path list, closed verification handoff,
CURRENT_TASK, PAY ticket and end-to-end status map.
