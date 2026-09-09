# WAL-009 actual cleanup source acceptance and validation 01

Reviewer: Codex, XHigh for source acceptance; High suffices for bounded result collection.
Actor: Jr Dev — Hermes, execution/evidence only. No source integration or Git.
Baseline governance: d9897ffa. Protected parent: the commit publishing this handoff.

## Source review accepted for execution

Grok correction session f2231b84-4294-48e8-95cb-87f495940173, outer 7343, was
collected once following owner done, exit 0. Saved runtime: grok-4.6-build High.
That source authorization is closed. The transcript contains five successful
test_support.rs replacements, two read-only inventories and local source reads.
No test, compiler, formatter, Cargo, Git or evidence work occurred. The initial
200-line CURRENT/ticket reads extended beyond the requested active prefixes;
record this limited read-scope deviation without another correction task.

Reviewer reconstructed the starting test_support.rs hash by reversing all five
replacements and independently verified the final 4463-line identity below.
The other 32 inventory rows are unchanged. The correction fixes both mutable
bindings, preserves actual early error labels, makes finish select an outcome
without draining, and leaves finalization exclusively in AttemptOwner Drop.
Unwind overrides any selected outcome. Success is selected after publication work
and result construction. The owner survives failure cleanup and partial moves.
Unexpected labels increment unclassified; canaries remain outside the collector.

The three-path production drop's actual seed/PCZT/transaction owners, removal of
fabricated producers and actual triggered-fault tracking are retained. No additional
source correction is required before this bounded execution. This is source
acceptance for testing, not full memory-erasure or lifecycle acceptance.

## Exact scope and runtime

Create only docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md as evidence.
Stop if that path or the new backup below already exists. The only source writes
authorized are the exact temporary falsifications and restoration of
wallet-broker/src/zec/test_support.rs prescribed here. No permanent source/test
change, other evidence edit, formatter, source repair, dependency or Git operation.

Use the existing disk-backed wallet-broker/target. Its filesystem was already
measured as ext4 (stat reports ext2/ext3); do not rediscover or change the target,
cache or temporary filesystem. The sole new backup is
wallet-broker/target/bbd-wal009-cleanup-observations-validation-01-test-support-original.rs.
Leave it in place after restoration; no deletion and no other helper/log/backup file.
The old decoded-effects backup is unrelated and must not be reused or changed.

Read AGENTS.md, TESTING.md, active CURRENT_TASK.md and ticket prefixes, and this
handoff. Work from the repository root with explicit terminal workdir. No cd or
persistent shell-state changes. Record hermes --version once and only
HERMES_SESSION_ID for session identity. Record actual provider/model if available;
otherwise state unavailable for reviewer database verification. No config or broad
environment discovery. Use no network, actor/subagent or generic completion checks.

## Preflight and final inventory

Run this inline inventory once before any test. On the final restored state run it
once more, changing only argument pre to post. No manually reconstructed path list,
helper file or repeated inventory. A preflight mismatch/error stops execution.

```sh
python3 - pre <<'PY_INVENTORY'
from pathlib import Path
import hashlib
import re
import sys
mode = sys.argv[1]
assert mode in ('pre', 'post')
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_CLEANUP_OBSERVATIONS_VALIDATION_01.md')
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff.read_text(), re.M)
assert len(rows) == 33, len(rows)
evidence = Path('docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md')
backup = Path('wallet-broker/target/bbd-wal009-cleanup-observations-validation-01-test-support-original.rs')
assert not evidence.exists(), 'Evidence already exists'
if mode == 'pre':
    assert not backup.exists(), 'Backup already exists'
failed = False
for name, expected_lines, expected_hash in rows:
    data = Path(name).read_bytes()
    actual_lines = data.count(b'\n')
    actual_hash = hashlib.sha256(data).hexdigest()
    matches = actual_lines == int(expected_lines) and actual_hash == expected_hash
    print(name, actual_lines, actual_hash, 'MATCH' if matches else 'MISMATCH')
    failed = failed or not matches
raise SystemExit(1 if failed else 0)
PY_INVENTORY
```

## Six test invocations, sequentially

Run each command below once at its specified stage, in the terminal background with
retained output. Wait only on its process, at most 60 seconds per wait. Retrieve
the complete process.log on completion, including every reported line. Preserve
command, launch, process, completion and log identifiers, exit and counts/timing.
Do not add wrappers, redirection or pipelines to the command strings. No retries.

1. Baseline focused green, restored source: exit 0, one passed, 14 filtered.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

2. Apply fault-a using the mutation command below, then run this same focused
   command once. Expect exit 101, one failed, 14 filtered at the final forbidden
   observation assertion, with exactly these two entries:
   ProofWorkspace/SignerError/touch_count/1 and
   ProofWorkspace/SignerError/positive_wipe_count/1.
   All earlier real-stage and positive-Seed guards must pass.
3. Run restore-a using the same mutation script. Then apply fault-b, and run the
   focused command once. Expect exit 101, one failed, 14 filtered at the first
   positive-Seed guard: "Seed was not touched for SignerError". The unchanged
   public-error, stage-count, no-publication and released-lock guards must pass.
   This suppresses only delivery of the actual seed owner's wipe notification;
   the owner and its actual zeroization remain. It falsifies non-vacuous observation,
   not physical erasure of the fixture's already-zero seed.
4. Run restore-b, then the focused command once: exit 0, one passed, 14 filtered.
5. Run the affected real software pipeline once: exit 0, one passed, 14 filtered.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify software_signs_proves_finalizes_extracts_and_independently_decodes_exact_v6_effects -- --exact
```

6. Run the affected synthetic external pipeline once: exit 0, one passed, 14 filtered.

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify synthetic_keystone_v2_returns_tagged_ironwood_contributions_for_retained_pczt -- --exact
```

The last two tests previously took about 155s each; they are rerun because the
actual artifact owners and observation plumbing changed in both routes. Do not run
the full integration target, expensive decoded-effects library matrix, native check,
formatter, no-run/compile probe or any other test. Compilation happens through the
authorized test commands only. Retain the previously accepted 1.87s expected red;
do not recreate it.

## Exact temporary mutations and restoration

Use this command at the stated steps, changing only its first argument fault-a to
restore-a, fault-b or restore-b as directed. These are the only authorized source
mutations. The code is a command embedded in this handoff, not a new helper file.
Reviewer computed both fault identities in memory without editing source or running
tests. On any unexpected test outcome, retrieve output, restore the active known
fault with its matching restore mode, perform the final inventory, write evidence
and stop. Do not continue to the next test. If a mutation/restore guard fails,
stop source writes and record the actual state; do not force a restore over unknown
bytes. An ordinary warning is not a failed test unless the command exits nonzero.

```sh
python3 - fault-a <<'PY_MUTATE'
from pathlib import Path
import hashlib
import sys
source = Path('wallet-broker/src/zec/test_support.rs')
backup = Path('wallet-broker/target/bbd-wal009-cleanup-observations-validation-01-test-support-original.rs')
mode = sys.argv[1]
base_hash = '11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb'
a_hash = '923c5fcbfc43a82e44222661b56d7b2b0eed0cdb54bbceaf5ae0d589410087a2'
b_hash = 'bc088bdccf63a90342e2bf701aee904a45ad787912460ebbfde4a4e9256554c4'
digest = lambda data: hashlib.sha256(data).hexdigest()
current = source.read_bytes()
if mode == 'fault-a':
    assert digest(current) == base_hash
    assert not backup.exists()
    with backup.open('xb') as output:
        output.write(current)
    assert backup.read_bytes() == current
    old = b'                _ => self.observations.record_unclassified(),\n            }\n        }\n    }\n}\n\nimpl Drop for AttemptOwner {'
    new = b'                _ => self.observations.record_unclassified(),\n            }\n        }\n        self.observations.record_event(TouchedSecretClass::ProofWorkspace, exit, 1, true);\n    }\n}\n\nimpl Drop for AttemptOwner {'
    assert current.count(old) == 1
    changed = current.replace(old, new, 1)
    expected = a_hash
elif mode == 'fault-b':
    assert digest(current) == base_hash
    assert backup.read_bytes() == current
    old = b'        let seed_owner = seed.into_observed("zec-operation-seed", Box::new(attempt.collector()));'
    new = b'        let seed_owner = seed.into_observed("zec-operation-seed", Box::new(IgnoreWipes));'
    assert current.count(old) == 1
    changed = current.replace(old, new, 1)
    expected = b_hash
elif mode in ('restore-a', 'restore-b'):
    assert digest(current) == {'restore-a': a_hash, 'restore-b': b_hash}[mode]
    changed = backup.read_bytes()
    assert digest(changed) == base_hash
    expected = base_hash
else:
    raise AssertionError(mode)
assert digest(changed) == expected
source.write_bytes(changed)
actual = source.read_bytes()
assert actual == changed
print(mode, source.as_posix(), actual.count(b'\n'), digest(actual))
PY_MUTATE
```

Identities: baseline 4463 lines, 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb;
fault-a 4464 lines, 923c5fcbfc43a82e44222661b56d7b2b0eed0cdb54bbceaf5ae0d589410087a2;
fault-b 4463 lines, bc088bdccf63a90342e2bf701aee904a45ad787912460ebbfde4a4e9256554c4.
Every successful restoration must reproduce the baseline bytes/hash. No fault may
remain intentionally installed at completion.

## Evidence and stop

After the final inventory, write the one evidence record with actual runtime,
source lineage, both complete inventory measurements, mutation/restoration output,
all executed command/tool identifiers, exit/counts/timings and complete saved process
output. Preserve output indentation. Normalize only local absolute paths to portable
repo-relative placeholders and disclose that normalization. Do not reconstruct a
test result from memory. Missing output is an evidence gap, not permission to rerun.

Disclose deviations and unrun stages accurately. Measure the finished evidence
SHA-256/newline count once, then stop without a self-hash in that record, further
source/backup checks, Git, Node work or report-only correction. No integration is
authorized. Source and fourteen prior actor evidence records remain uncommitted;
this run will add a fifteenth evidence record.

Success accepts only the focused truthful-observation regression and preservation
of the two working pipelines. The historical synthetic all-classes wipe test stays
byte-identical and unaccepted as cleanup evidence. Real lifecycle coverage, including
early error and late unwind classification, and upstream typed-secret erasure remain
open. Full native OS/owning-thread/capability and final security acceptance remain
open. Network, broadcast, mainnet, real hardware, Monero and Electron send stay parked.
Decoded-effects acceptance and earlier valid checks remain in force.

Reviewer publication scope: this handoff, docs/handoff/CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. High is sufficient for collecting these fixed outcomes;
return to XHigh for the next cleanup lifecycle design/acceptance. Launch Hermes once,
record outer/session identity, collect after owner done, never poll the actor.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1153 | b4e9b87c743394e638011076cab673945328b0fbd23b36de53a42db3c93028fe |
| wallet-broker/src/zec/test_support.rs | 4463 | 11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 768 | 36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110 |
| wallet-broker/src/native_ui.rs | 321 | d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf |
| wallet-broker/src/native.rs | 489 | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |
| wallet-broker/Cargo.toml | 122 | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | 5395 | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |
| package.json | 42 | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |
| package-lock.json | 396 | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| wallet-broker/src/zec.rs | 274 | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 2872 | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 120 | ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 199 | 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 181 | 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2 |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md | 216 | 79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2 |
| wallet-broker/tests/zec_sign_verify.rs | 1215 | c95fa9ae836ca7151f35ef92e664dbc569f24b54b1ff4fb8ad78c1fce997a787 |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 |
| docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 184 | e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b |
| wallet-broker/src/zec/spend/effects.rs | 379 | cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035 |
| docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md | 354 | ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md | 196 | a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md | 165 | fe82cce82ebba2595747a411a5ffd2258d86abed36dc9b3de60517193d4babad |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md | 345 | 02d078de8a0269ba2dc41664450965a3eec1036ccb5343db6bb8f90518a64f5a |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md | 177 | 996f26aeea6b1cb27799200b24258e0e41defaa05505a0ee2b704c60471c2e9e |
| wallet-broker/src/vault.rs | 794 | f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49 |

## Collected validation — accepted

Reviewer: Codex, applying the fixed validation contract. Hermes outer 38061 was
collected once after owner done, exit 0. The completed-session database confirms
session 20260908_213504_01112b, provider nous, model poolside/laguna-s-2.1:free.
Version result 79222 reports Hermes v0.18.2 (2026.7.7.2), upstream b1f003e1,
local 10b6d1a9. The actor and this execution authorization are closed.

ACCEPT the focused truthful-observation regression, its two falsifications and
preservation of the affected software and synthetic external pipelines. All six
test commands and the six inventory/mutation/restoration scripts match the
authorized command text exactly. Every invocation selected one test and filtered
14. No acceptance command is to be rerun.

| Stage | Command / launch | Saved completion | Exit | Outcome | Test duration |
| --- | --- | --- | ---: | --- | ---: |
| Focused green | 79233 / 79234 | 79237 | 0 | 1 passed | 1.75s |
| Invented ProofWorkspace event | 79248 / 79249 | 79251 | 101 | 1 failed as required | 2.26s |
| Suppressed seed notification | 79273 / 79274 | 79276 | 101 | 1 failed as required | 1.99s |
| Restored focused green | 79281 / 79282 | 79284 | 0 | 1 passed | 1.69s |
| Real software pipeline | 79287 / 79288 | 79294 | 0 | 1 passed | 143.73s |
| Synthetic external pipeline | 79297 / 79298 | 79310 | 0 | 1 passed | 140.59s |

The corresponding process IDs, in order, are proc_0f63d9d74c0e,
proc_0590a3e1d8f9, proc_13b21aaa302b, proc_c73077458baf, proc_16f962490a4b
and proc_04a1da0537c2. Saved completion outputs contain 33, 46, 46, 33, 33
and 33 lines respectively, including the complete test results. Reviewer read
those outputs directly. The three ordinary dead-code warnings did not fail tests.

Fault-a failed at tests/zec_sign_verify.rs:1211 with exactly
ProofWorkspace/SignerError/touch_count/1 and
ProofWorkspace/SignerError/positive_wipe_count/1, after the real-stage and
positive-Seed guards passed. Fault-b failed at line 1156 with
"Seed was not touched for SignerError", after the public-error, stage,
no-publication and released-lock guards. The latter proves the test requires the
real owner's notification; it does not prove physical erasure of an already-zero seed.

Mutation outputs 79245 and 79272 match the two prescribed fault hashes.
Restoration outputs 79267 and 79278 reproduce the accepted 4463-line adapter hash
11ef7579a342ef3feee455418fa2522a98ee5616ea719aea91e6396927ee5cdb.
Saved preflight 79230 and postflight 79314 each exactly match all 33 table rows.
Reviewer independently confirmed all 33 current identities and that the retained
backup equals restored test_support.rs byte-for-byte. No fault remains installed.

New actor evidence:
docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md, 564 lines,
SHA-256 5bfed1076b4668f2df03cc2766983989323a8c0332ada7b4f43db34f2c8868a1.
It remains uncommitted. Its no-deviations and verbatim-output claims are rejected;
the saved command/completion records above govern acceptance.

The following deviations do not change the measured source or test outcomes:

- Calls 79221 and 79225 performed prohibited read-only Git log/status and ancestry
  checks, with results 79224/79226. No Git mutation occurred. Runtime commands
  in 79221 omitted explicit workdir. The report's no-Git claim is inaccurate.
- Call 79216 explicitly read CURRENT_TASK.md from the historical section at line
  43; the ticket read also extended beyond its active prefix.
- Five waits requested 600 seconds, although the tool clamped them to 60 seconds.
  Later waits used 60. The actor never issued the prescribed process.log calls;
  saved process.wait completions provide the output used for this review.
- The report alters warning indentation, drops closing diagnostic backticks,
  transcribes fault-b's warning line as 1603 instead of 1602, and changes the
  software test compilation time from 0.15s to 0.19s. Its repeated fault/vault
  hash strings contain transcription errors despite correct saved measurements.
  Local absolute paths remain despite the normalization claim, and the exact
  command/completion identifiers above were omitted from its identifier section.
- After final evidence measurement 79359, call 79362 updated the actor todo list,
  contrary to the final no-further-tools instruction. No source/test/evidence
  mutation or execution followed the measurement.

Do not launch a report-only correction or rerun valid checks for these deviations.
This acceptance does not adopt the altered report as a verbatim log or authorize
its integration. There are now fifteen uncommitted actor evidence records alongside
the uncommitted implementation.

No source, execution or integration actor remains authorized. The next reviewer
task is the real cleanup lifecycle test contract: replace the unsupported synthetic
all-classes claim with coverage of actual owned buffers, early failures and unwind
classification. Preserve its current source until that bounded test authorization.
No broader cleanup acceptance or new implementation semantics are implied here.
Full typed-secret lifetime/erasure, native OS/owning-thread/capability integration
and final security remain open. Retain decoded-effects acceptance and all earlier
valid results; network/broadcast/mainnet/hardware/Monero/Electron send remain parked.

The reviewer stops at the reasoning-change boundary requested by the owner.
Use XHigh for the next lifecycle design/acceptance step; no actor is running and
there is nothing to poll.
