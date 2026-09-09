# WAL-009 cleanup lifecycle test-source acceptance and validation 01

Reviewer: Codex, XHigh. Actor: Jr Dev — Hermes, execution/evidence only.
Baseline governance: 4dd9abd7. Protected parent: the commit publishing this handoff.
No source integration or Git is authorized.

## Source accepted for bounded execution

Grok session c7b99290-f754-4e0e-838c-7950ecaf7840, outer 49083, was collected
once after owner done, exit 0. Saved runtime: grok-4.6-build High. That source
authorization is closed. The transcript contains two new test-file writes, six
successful bounded replacements, local reads and six read-only measurement
commands. No compiler, formatter, Cargo/test, Git or evidence execution occurred.

ACCEPT the five-path test source for this validation. Reviewer read all six new
library tests and the replacement integration test. Actual owner destructors supply
events, nonzero fixtures guard wiping, and marked/cross-checked panic payloads prevent
unrelated assertion failures from counting as intended unwind. Fixed expectation
tables cover all nine classes and fourteen exits; tests never synthesize input
events, call classify/record_event directly or treat stage counts as wipe evidence.

Reviewer independently verified both original production prefixes plus only the
cfg(test) declarations, all five resulting source identities, and the other 31
inventory rows unchanged. Exact reversal of the integration replacements reproduced
the accepted 1215-line starting hash. Only the prescribed helper/test regions
changed; all other 14 integration tests remain byte-identical. There are 13 library
tests (six new) and 15 integration tests (one replacement). Compile/runtime remain
unverified until the commands below run.

The replacement checks real wrong-seed rejection after raw-PCZT ownership but
before counted PCZT access. The new unit tests exercise deferred/default outcome,
marked unwind, real pre-consume cancellation/expiry/missing-handle/schema errors,
empty/unknown owners, and actual PCZT/serializer-buffer destruction. The retired
synthetic all-classes assertion is no longer a pending full-target failure; this
does not claim coverage of unobserved third-party key/prover memory.

## Exact writable scope

Create only docs/testing/BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md as evidence.
Stop if it exists. The only source writes are the prescribed temporary faults and
restoration in test_support.rs, spend.rs and vault.rs. All tests remain frozen.
No permanent source edits, source repair, formatter, dependencies, other evidence
or governance changes, Git, integration, network, actors or subagents.

The three new backups are exactly:
- wallet-broker/target/bbd-wal009-cleanup-lifecycle-validation-01-test-support-original.rs
- wallet-broker/target/bbd-wal009-cleanup-lifecycle-validation-01-spend-original.rs
- wallet-broker/target/bbd-wal009-cleanup-lifecycle-validation-01-vault-original.rs

The script creates each only on first use with exclusive creation, and retains it
after restoration. Stop if any exists at preflight. No deletion or other helper,
log, backup or cache path. Prior cleanup/decoded-effects backups remain untouched.
Use the existing disk-backed wallet-broker/target, already measured as ext4
(stat reports ext2/ext3). Do not rediscover filesystem layout or change target/cache.

Read AGENTS.md, TESTING.md, only active CURRENT_TASK.md/ticket prefixes, and this
handoff. Use explicit repository-root workdir for every terminal command; no cd,
Git ancestry/status/log checks, historical reload or configuration discovery.
Record hermes --version once and only HERMES_SESSION_ID for session identity.
Record actual provider/model if available; otherwise say unavailable for reviewer
verification against the completed-session database.

## Preflight and final inventory

Run this command once before testing, and once after final restoration changing
only pre to post. It parses all 36 frozen identities from this handoff. Do not
reconstruct a manual inventory. A preflight mismatch/error stops all tests.

```sh
python3 - pre <<'PY_INVENTORY'
from pathlib import Path
import hashlib
import json
import re
import sys
mode = sys.argv[1]
assert mode in ('pre', 'post')
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_CLEANUP_LIFECYCLE_VALIDATION_01.md').read_text()
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff, re.M)
assert len(rows) == 36, len(rows)
plan = json.loads(re.search(r'```json\n(.*?)\n```', handoff, re.S).group(1))
assert not Path('docs/testing/BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md').exists()
failed = False
for name, expected_lines, expected_hash in rows:
    data = Path(name).read_bytes()
    actual_lines = data.count(b'\n')
    actual_hash = hashlib.sha256(data).hexdigest()
    matches = actual_lines == int(expected_lines) and actual_hash == expected_hash
    print(name, actual_lines, actual_hash, 'MATCH' if matches else 'MISMATCH')
    failed = failed or not matches
for source_name, backup_name in plan['backups'].items():
    backup = Path(backup_name)
    if mode == 'pre':
        assert not backup.exists(), backup_name
    elif backup.exists():
        assert backup.read_bytes() == Path(source_name).read_bytes(), backup_name
        print(backup_name, 'EQUALS_RESTORED_SOURCE')
    else:
        print(backup_name, 'NOT_CREATED')
raise SystemExit(1 if failed else 0)
PY_INVENTORY
```

## Nine test invocations in this order

1. Run the initial six-test library green below.
2. Run faults A through F in order. For each: apply exactly one fault, run its
   one exact test once, retrieve output, restore that fault, then continue only
   if the expected assertion failure and exact restoration are confirmed.
3. Run the same six-test library command once on fully restored source.
4. Run the replacement integration command once.

Library green command (initial and final only):
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib cleanup_lifecycle_tests
```
Expected exit 0, six passed, seven filtered. Do not accidentally run the seven
older proof-heavy library tests.

Final replacement integration command:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify cleanup_lifecycle_wrong_seed_wipes_owned_buffers_before_pczt_access -- --exact
```
Expected exit 0, one passed, 14 filtered.

Run each test in the terminal background with retained output. Keep command strings
verbatim: no wrapper, redirection, pipeline, extra option or environment override.
Wait only on its process, with timeout at most 60 seconds per wait. Retrieve the
complete process.log upon completion, requesting all reported lines (page if needed).
Record command/launch/process/completion/log identifiers and exact exit/counts/timings.
Do not describe a retyped summary as a verbatim log. These selected tests do not
generate transaction proofs; no repeated expensive signing/library/full-target test
is authorized. An ordinary warning is not failure unless the command exits nonzero.

### Fault A

Run mutation mode apply-a, then this exact test command once:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop -- --exact
```
Expect exit 101, one failed, 12 filtered. Required diagnostic/invariant:
Seed/Success touch=1 expected=0 before owner Drop. It must be a test assertion failure, not a compile/setup error.
Retrieve the complete output, then run restore-a before any later stage.

### Fault B

Run mutation mode apply-b, then this exact test command once:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::test_support::cleanup_lifecycle_tests::panic_overrides_selected_outcome -- --exact
```
Expect exit 101, one failed, 12 filtered. Required diagnostic/invariant:
marked panic payload passes; Seed/Success touch=2 expected=0. It must be a test assertion failure, not a compile/setup error.
Retrieve the complete output, then run restore-b before any later stage.

### Fault C

Run mutation mode apply-c, then this exact test command once:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::test_support::cleanup_lifecycle_tests::preconsume_errors_use_actual_exit -- --exact
```
Expect exit 101, one failed, 12 filtered. Required diagnostic/invariant:
CANCELLED and stage guards pass; Seed/Error touch=1 expected=0. It must be a test assertion failure, not a compile/setup error.
Retrieve the complete output, then run restore-c before any later stage.

### Fault D

Run mutation mode apply-d, then this exact test command once:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop -- --exact
```
Expect exit 101, one failed, 12 filtered. Required diagnostic/invariant:
Seed/Success touch=2 passes; positive=0 expected=2 for nonzero owners. It must be a test assertion failure, not a compile/setup error.
Retrieve the complete output, then run restore-d before any later stage.

### Fault E

Run mutation mode apply-e, then this exact test command once:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::cleanup_lifecycle_tests::pczt_owner_wipes_on_error_and_unwind -- --exact
```
Expect exit 101, one failed, 12 filtered. Required diagnostic/invariant:
normal error guard passes; zec-authoritative-pczt event count 0 expected 1. It must be a test assertion failure, not a compile/setup error.
Retrieve the complete output, then run restore-e before any later stage.

### Fault F

Run mutation mode apply-f, then this exact test command once:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::cleanup_lifecycle_tests::extracted_owner_wipes_on_error_and_unwind -- --exact
```
Expect exit 101, one failed, 12 filtered. Required diagnostic/invariant:
normal error, label and length=64 pass; actual all_zero=false. It must be a test assertion failure, not a compile/setup error.
Retrieve the complete output, then run restore-f before any later stage.

## Mutation and restoration command

Run this exact inline script with the specified mode argument: apply-a/restore-a
through apply-f/restore-f. Change only that first argument. It reads the reviewed
machine-readable plan below; do not create a helper script or manually retype
replacement source. Reviewer computed the fault hashes in memory, without editing
source or executing any test. No current fault is installed.

The guard requires all unrelated mutable source paths at baseline before writing,
and checks the active fault/base hash and newline count. A restore never overwrites
unknown bytes. Do not change tests to obtain an expected failure.

```sh
python3 - apply-a <<'PY_MUTATE'
from pathlib import Path
import hashlib
import json
import re
import sys
mode, key = sys.argv[1].split('-', 1)
assert mode in ('apply', 'restore')
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_CLEANUP_LIFECYCLE_VALIDATION_01.md').read_text()
plan = json.loads(re.search(r'```json\n(.*?)\n```', handoff, re.S).group(1))
faults = {entry['key']: entry for entry in plan['plans']}
assert len(faults) == 6 and key in faults
fault = faults[key]
source = Path(fault['path'])
backup = Path(plan['backups'][fault['path']])
digest = lambda data: hashlib.sha256(data).hexdigest()
current = source.read_bytes()
baselines = {entry['path']: entry['base_hash'] for entry in plan['plans']}
for name, expected in baselines.items():
    if mode == 'restore' and name == fault['path']:
        expected = fault['fault_hash']
    assert digest(Path(name).read_bytes()) == expected, name
if mode == 'apply':
    if backup.exists():
        assert backup.read_bytes() == current, backup.as_posix()
    else:
        with backup.open('xb') as output:
            output.write(current)
        assert backup.read_bytes() == current
    old, new = fault['old'].encode(), fault['new'].encode()
    assert current.count(old) == 1
    changed = current.replace(old, new, 1)
    expected = fault['fault_hash']
    expected_lines = fault['fault_lines']
else:
    changed = backup.read_bytes()
    expected = fault['base_hash']
    expected_lines = fault['base_lines']
assert digest(changed) == expected
assert changed.count(b'\n') == expected_lines
source.write_bytes(changed)
actual = source.read_bytes()
assert actual == changed
print(sys.argv[1], source.as_posix(), actual.count(b'\n'), digest(actual))
PY_MUTATE
```

## Reviewed mutation plan

D and F omit wiping before observation: actual known nonzero contents must produce
all_zero=false. Ordinary later destruction still occurs. E suppresses the explicit
PCZT owner notification, while its contained SecretBytes still receives ordinary
Drop. These distinctions prevent claiming notification loss is proof of erased or
unerased freed memory. A/B/C change only the named outcome bookkeeping.

```json
{
  "backups": {
    "wallet-broker/src/zec/test_support.rs": "wallet-broker/target/bbd-wal009-cleanup-lifecycle-validation-01-test-support-original.rs",
    "wallet-broker/src/zec/spend.rs": "wallet-broker/target/bbd-wal009-cleanup-lifecycle-validation-01-spend-original.rs",
    "wallet-broker/src/vault.rs": "wallet-broker/target/bbd-wal009-cleanup-lifecycle-validation-01-vault-original.rs"
  },
  "plans": [
    {
      "key": "a",
      "path": "wallet-broker/src/zec/test_support.rs",
      "old": "    fn finish(&mut self, exit: WipeExit) {\n        self.selected_outcome = Some(exit);\n    }\n",
      "new": "    fn finish(&mut self, exit: WipeExit) {\n        self.selected_outcome = Some(exit);\n        self.classify(exit);\n    }\n",
      "base_hash": "d795d7fc1ee8c91b25412de52cee1fbd99d220c54515a3d6d0dd61e6efffe22a",
      "base_lines": 4466,
      "fault_hash": "7c737ab9ed84f217d59b61b296c75a7275088aebac4e99309abf340ef318c3cd",
      "fault_lines": 4467,
      "test": "zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop",
      "reason": "Seed/Success touch=1 expected=0 before owner Drop"
    },
    {
      "key": "b",
      "path": "wallet-broker/src/zec/test_support.rs",
      "old": "impl Drop for AttemptOwner {\n    fn drop(&mut self) {\n        let exit = if std::thread::panicking() {\n            WipeExit::PanicUnwind\n        } else {\n            self.selected_outcome.unwrap_or(WipeExit::Error)\n        };\n        self.classify(exit);\n    }\n}",
      "new": "impl Drop for AttemptOwner {\n    fn drop(&mut self) {\n        let exit = self.selected_outcome.unwrap_or(WipeExit::Error);\n        self.classify(exit);\n    }\n}",
      "base_hash": "d795d7fc1ee8c91b25412de52cee1fbd99d220c54515a3d6d0dd61e6efffe22a",
      "base_lines": 4466,
      "fault_hash": "3ca92085ccd108818a6526e79c26d5e30bb7956a501b6e5058fd24ae14d03aa7",
      "fault_lines": 4462,
      "test": "zec::test_support::cleanup_lifecycle_tests::panic_overrides_selected_outcome",
      "reason": "marked panic payload passes; Seed/Success touch=2 expected=0"
    },
    {
      "key": "c",
      "path": "wallet-broker/src/zec/test_support.rs",
      "old": "        let artifact = match account.prepare.consume(&review, &clock.value) {\n            Ok(artifact) => artifact,\n            Err(error) => {\n                attempt.finish(wipe_exit_for_error(&error, None));\n                return Err(error);\n            }\n        };",
      "new": "        let artifact = match account.prepare.consume(&review, &clock.value) {\n            Ok(artifact) => artifact,\n            Err(error) => {\n                return Err(error);\n            }\n        };",
      "base_hash": "d795d7fc1ee8c91b25412de52cee1fbd99d220c54515a3d6d0dd61e6efffe22a",
      "base_lines": 4466,
      "fault_hash": "7449e62e73f774b296b5a661e2d61ba7ffd377aeff728e3da23cd3c8857463e4",
      "fault_lines": 4465,
      "test": "zec::test_support::cleanup_lifecycle_tests::preconsume_errors_use_actual_exit",
      "reason": "CANCELLED and stage guards pass; Seed/Error touch=1 expected=0"
    },
    {
      "key": "d",
      "path": "wallet-broker/src/vault.rs",
      "old": "        let bytes = secret.expose_secret_mut();\n        let length = bytes.len();\n        bytes.zeroize();\n        observer.observe(WipeEvent {",
      "new": "        let bytes = secret.expose_secret_mut();\n        let length = bytes.len();\n        observer.observe(WipeEvent {",
      "base_hash": "f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49",
      "base_lines": 794,
      "fault_hash": "4bbeb892e3e359c5db512e938374d009fd13af109b423a5e7fac6c2fb291810c",
      "fault_lines": 793,
      "test": "zec::test_support::cleanup_lifecycle_tests::attempt_defers_classification_until_owner_drop",
      "reason": "Seed/Success touch=2 passes; positive=0 expected=2 for nonzero owners"
    },
    {
      "key": "e",
      "path": "wallet-broker/src/zec/spend.rs",
      "old": "impl Drop for ObservedSigningArtifact<'_> {\n    fn drop(&mut self) {\n        self.raw.wipe_with(\"zec-authoritative-pczt\", self.observer);\n    }\n}",
      "new": "impl Drop for ObservedSigningArtifact<'_> {\n    fn drop(&mut self) {\n    }\n}",
      "base_hash": "cb1f0889749b5f084f2421f41e9b3c77b9fe50a2241c0f39d29bf77ef793f81a",
      "base_lines": 1156,
      "fault_hash": "a6c68d59461f2078aec575bdccf289daa381db3c0a51dae3a7ca22eb50d8c2dd",
      "fault_lines": 1155,
      "test": "zec::spend::cleanup_lifecycle_tests::pczt_owner_wipes_on_error_and_unwind",
      "reason": "normal error guard passes; zec-authoritative-pczt event count 0 expected 1"
    },
    {
      "key": "f",
      "path": "wallet-broker/src/zec/spend.rs",
      "old": "impl Drop for ExtractedTransaction<'_> {\n    fn drop(&mut self) {\n        let length = self.bytes.len();\n        self.bytes.as_mut_slice().zeroize();\n        let all_zero = self.bytes.iter().all(|byte| *byte == 0);",
      "new": "impl Drop for ExtractedTransaction<'_> {\n    fn drop(&mut self) {\n        let length = self.bytes.len();\n        let all_zero = self.bytes.iter().all(|byte| *byte == 0);",
      "base_hash": "cb1f0889749b5f084f2421f41e9b3c77b9fe50a2241c0f39d29bf77ef793f81a",
      "base_lines": 1156,
      "fault_hash": "ff4b81bd32d56964d3bf580810b098442cc5d64a356647b2c65ff3bc0f6e73b6",
      "fault_lines": 1155,
      "test": "zec::spend::cleanup_lifecycle_tests::extracted_owner_wipes_on_error_and_unwind",
      "reason": "normal error, label and length=64 pass; actual all_zero=false"
    }
  ]
}
```

## Unexpected outcome, evidence and final stop

If any test produces an unexpected result, retrieve its complete output, restore
the active known fault with its matching mode if one is installed, run final
inventory, write evidence and stop. Do not run remaining tests or repair/retry.
If a mutation/restore guard fails, stop source writes and report the exact state;
do not force a restore. A baseline compilation failure also stops this run.
Never leave a recognized temporary fault installed intentionally.

After final inventory, write the one evidence record with actual runtime/source
lineage, both inventories, every executed mutation/restore result, exact test
commands, tool/process IDs, counts/timings and complete saved process output.
Preserve indentation and diagnostic bytes. Normalize only local absolute paths
to portable repo-relative placeholders and disclose that normalization. Do not
invent hashes, warning lines, timings, missing output or no-deviations claims.
Missing logs are an evidence gap, not permission to rerun a test.

Disclose deviations and unrun stages. Finish todo bookkeeping before measuring
the completed evidence SHA-256/newline count once; then stop with no further tools.
No evidence self-hash, Git, generic Node/build completion work, report-only correction
or source integration. This will add a sixteenth uncommitted actor evidence record;
implementation and the fifteen prior records remain uncommitted.

Successful outcomes accept the named owner/lifecycle tests and falsification only.
They do not establish full proof-to-publication panic integration, third-party
typed-secret erasure, native OS/owning-thread/capability behavior or final security.
Those remain open. Retain prior truthful-observation, signing-pipeline and decoded-
effects acceptance; no repeated proof-heavy checks. Network/broadcast/mainnet/
hardware/Monero/Electron send remain parked.

Reviewer publication scope: this handoff, docs/handoff/CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. High is sufficient for collecting these fixed outcomes;
use XHigh again before new lifecycle/security design or broader acceptance.
Launch Hermes once, record its outer/session identity, collect after owner done,
never poll the actor.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1156 | cb1f0889749b5f084f2421f41e9b3c77b9fe50a2241c0f39d29bf77ef793f81a |
| wallet-broker/src/zec/test_support.rs | 4466 | d795d7fc1ee8c91b25412de52cee1fbd99d220c54515a3d6d0dd61e6efffe22a |
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
| wallet-broker/tests/zec_sign_verify.rs | 1258 | 46a0cc520421e43e4ca3215240be9a7f92f7ef5581525727964d2247e7f660a7 |
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
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md | 564 | 5bfed1076b4668f2df03cc2766983989323a8c0332ada7b4f43db34f2c8868a1 |
| wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs | 452 | b280e7dfa0eb3f65360b33d6f2f5e7debb3fe53c1d4ea4ea8ba1fcfd54500c06 |
| wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs | 174 | ca7a373f8d009e9fc66e65ecfb9d63b4b1483fb924ab9f66dcba511a3f904ccc |
