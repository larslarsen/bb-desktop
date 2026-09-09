# WAL-009 real pipeline panic source acceptance and validation 01

Reviewer: Codex, High. Actor: Jr Dev — Hermes, execution/evidence only.
Baseline governance: 8aa93688. Protected parent: the commit publishing this handoff.
No source integration or Git is authorized.

## Source acceptance

Grok outer 43249 completed exit 0 after owner done; session
536b2be3-3b69-4f81-91f0-e657b766ba37, saved grok-4.6 High. Source actor is closed.
Transcript: 24 read_file, five grep, five successful search_replace, three read-only
Python measurements. No tests/compiler/formatter/Cargo/Git/evidence/actor execution.
The report omits the explicit session identifier; saved records establish it.

ACCEPT the five-region/two-path correction for bounded validation. Reviewer read
all changes and relevant owner scopes. Exact reversal reproduces both starting
hashes; a separate region reconstruction proves every byte outside the five allowed
regions unchanged. All 38 current source/evidence identities match the table below;
36 unchanged, two reviewed source updates. The integration target remains 1338
lines / 16 tests; the no-default library has 13 tests, six cleanup lifecycle tests.

PanicAfterVerification fires only after successful independent verification, before
serializer Drop, with the real raw-PCZT owner still live. The software call's
catch merges actual counters once, invalidates the original account with PanicUnwind,
and resumes the original payload. Normal Result success/error flow and all owner
Drops are unchanged. The queued PCZT/transaction events plus seed Drop are classified
by AttemptOwner during resumed unwind. No fabricated observations or stage counts.
The defensive fault arm falls back to the actual error code. The existing panic
helper routes through execute; any unexpected normal return uses a distinct payload
that cannot pass the frozen regression. No public operation/capability was added.

The accepted seed_accesses=0 regression red and all prior cleanup/signing/decoded-
effects results remain valid. Do not rerun them. Compile/runtime for this correction
remain unverified until the commands below run. Full typed-secret erasure, native OS/
owning-thread/capability integration and final security remain open.

## Exact writable scope and setup

Create only docs/testing/BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md as evidence.
Stop if it exists. The only source write is the exact temporary spend.rs fault and
restoration below. All tests, dependencies, other source, evidence and governance
are frozen. Create only the one new backup named in the JSON plan, exclusively on
apply; retain it after restoration. All older backups remain untouched.
No deletion, extra helper/log/backup files, formatter, repair, Git, integration,
network, product/native UI, hardware, configuration discovery or other actors.
Use the reviewed disk-backed wallet-broker/target; do not change target/cache or
rediscover filesystem layout. No large temporary artifacts.

Read exactly AGENTS.md, TESTING.md, this handoff, docs/handoff/CURRENT_TASK.md
lines 1-38 and tickets/BBD-WAL-009.md lines 1-18. The ticket is in root tickets/;
do not guess docs/tickets or search historical handoffs. Set explicit repository-
root workdir for every terminal command. Do not prepend cd or use an unavailable
shell tool. The terminal/process tools retain output; do not add 2>&1.
Record hermes --version once and only HERMES_SESSION_ID with read-only Python.
Record provider/model from actual runtime if available; otherwise say unavailable
for reviewer verification. No cargo --version or other discovery command.

## Exactly five invocations, in this order

Use each command string verbatim in terminal background mode. Wait only on its
process, timeout at most 60 seconds per wait. Fetch complete process.log on completion
(request all reported lines, page if necessary), retaining command/launch/process/
wait/completion/log identifiers and exact exit/counts/compile/test timing. Ordinary
warnings do not fail an otherwise exit-0 command. No wrappers/redirection/pipeline/
extra environment/options, selector changes, retries or full targets.

Before any test, run the inventory below in pre mode. Then:

1. Real pipeline panic green:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify real_pipeline_panic_releases_owners_and_never_publishes -- --exact
```
Expected exit 0, one passed / 15 filtered. This performs one actual proof and
verifies real stage/owner/invalidation assertions. A test may take several minutes.

2. Apply the exact lock-release fault below, then run that identical command once.
Expected exit 101, one failed / 15 filtered. It must reach the intended INTERNAL
panic and pass all stage/hardware/publication guards, then fail the first
account_lock_count(ACCOUNT)==0 assertion (currently tests/zec_sign_verify.rs:1291),
left 1 / right 0. Check actual source line before citing a line number; assertion
identity and order govern acceptance. A compile/setup failure or earlier wrong
panic/stage assertion is not the intended red. Fetch full output, then restore
before any later test, even if the result is unexpected. This is the second and
last proof-generating invocation.

3. After exact restoration, six cheap cleanup lifecycle unit tests:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib cleanup_lifecycle_tests
```
Expected exit 0, six passed / seven filtered.

4. Focused signer-error regression:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```
Expected exit 0, one passed / 15 filtered; no proof generation.

5. Account-lock regression with the old standalone panic subsection removed:
```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify account_authorization_lock_is_scoped_and_released_on_every_exit -- --exact
```
Expected exit 0, one passed / 15 filtered; no proof generation. Its remaining helper
cases are account-gate checks, not additional real pipeline panic evidence.

Keep invocation 1's green valid on restored byte-identical source. Do not run a
third proof or repeat accepted happy paths. Finally run post inventory, write the
one evidence record, measure its SHA-256/newlines/bytes as the last tool operation,
then report and stop (no subsequent todo/status/read). Copy full test outputs;
normalize only local repository/home path prefixes to $REPO/$HOME and label that
normalization. Do not retype diagnostics or label abbreviated scripts/logs verbatim.
Include both inventories, all message/process IDs and actual deviations.

Stop on any unexpected result: restore a known active fault with the guarded script,
run post inventory, write actual stop evidence, measure and report. No repair or retry.
If a source hash is unknown or a mutation guard fails, stop source writes; never
force restoration over unknown bytes. If preflight fails, no tests may run.

## Preflight and final inventory

Run once as pre and once as post, changing only that mode argument. Evidence must
still be absent; the backup must be absent at preflight. Post verifies any created
backup equals restored source.

```sh
python3 - pre <<'PY_INVENTORY'
from pathlib import Path
import hashlib
import json
import re
import sys
mode = sys.argv[1]
assert mode in ('pre', 'post')
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_PIPELINE_PANIC_VALIDATION_01.md').read_text()
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff, re.M)
assert len(rows) == 38, len(rows)
plan = json.loads(re.search(r'```json\n(.*?)\n```', handoff, re.S).group(1))
assert not Path('docs/testing/BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md').exists()
failed = False
for name, expected_lines, expected_hash in rows:
    data = Path(name).read_bytes()
    actual_lines = data.count(b'\n')
    actual_hash = hashlib.sha256(data).hexdigest()
    matches = actual_lines == int(expected_lines) and actual_hash == expected_hash
    print(name, actual_lines, actual_hash, 'MATCH' if matches else 'MISMATCH')
    failed = failed or not matches
backup = Path(plan['backup'])
if mode == 'pre':
    assert not backup.exists(), plan['backup']
elif backup.exists():
    assert backup.read_bytes() == Path(plan['source']).read_bytes(), plan['backup']
    print(plan['backup'], 'EQUALS_RESTORED_SOURCE')
else:
    print(plan['backup'], 'NOT_CREATED')
raise SystemExit(1 if failed else 0)
PY_INVENTORY
```

## Guarded fault and restoration

The sole mutation removes AccountAuthorizationLease Drop's account removal. It
leaves the actual panic, ownership, wipe notifications and every test assertion
intact. Compute/apply exactly from the plan, create the backup with exclusive xb,
then later run the same command with restore instead of apply. Do not reconstruct
scripts or manually edit source.

```sh
python3 - apply <<'PY_MUTATE'
from pathlib import Path
import hashlib
import json
import re
import sys
mode = sys.argv[1]
assert mode in ('apply', 'restore')
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_PIPELINE_PANIC_VALIDATION_01.md').read_text()
plan = json.loads(re.search(r'```json\n(.*?)\n```', handoff, re.S).group(1))
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff, re.M)
assert len(rows) == 38, len(rows)
source = Path(plan['source'])
backup = Path(plan['backup'])
digest = lambda data: hashlib.sha256(data).hexdigest()
for name, expected_lines, expected_hash in rows:
    if mode == 'restore' and name == plan['source']:
        expected_lines, expected_hash = plan['fault_lines'], plan['fault_hash']
    data = Path(name).read_bytes()
    assert data.count(b'\n') == int(expected_lines) and digest(data) == expected_hash, name
current = source.read_bytes()
if mode == 'apply':
    assert not backup.exists(), plan['backup']
    with backup.open('xb') as output:
        output.write(current)
    assert backup.read_bytes() == current
    old, new = plan['old'].encode(), plan['new'].encode()
    assert current.count(old) == 1
    changed = current.replace(old, new, 1)
    expected_hash, expected_lines = plan['fault_hash'], plan['fault_lines']
else:
    changed = backup.read_bytes()
    expected_hash, expected_lines = plan['base_hash'], plan['base_lines']
assert digest(changed) == expected_hash
assert changed.count(b'\n') == expected_lines
source.write_bytes(changed)
actual = source.read_bytes()
assert actual == changed
print(mode, plan['source'], actual.count(b'\n'), digest(actual))
PY_MUTATE
```

```json
{
  "source": "wallet-broker/src/zec/spend.rs",
  "backup": "wallet-broker/target/bbd-wal009-pipeline-panic-validation-01-spend-original.rs",
  "base_hash": "9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9",
  "base_lines": 1160,
  "fault_hash": "b9937a7e5674f36d1b877fe0011055abff7090d826a0c5bf13eff42c0aca4ccc",
  "fault_lines": 1159,
  "old": "        mutex_lock(&self.held).remove(&self.account_id);\n",
  "new": ""
}
```

Reviewer publication scope: this handoff, CURRENT_TASK.md and tickets/BBD-WAL-009.md.
No source/evidence integration. Launch once, collect on owner done, never poll.

## Frozen inventory (38 paths)

| Path | Lines | SHA-256 |
| --- | --- | --- |
| wallet-broker/src/zec/spend.rs | 1160 | 9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9 |
| wallet-broker/src/zec/test_support.rs | 4479 | 255600063e348a54654cd5923496a9bba5e2dfa447222997c6bfc79b2d592c27 |
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
| wallet-broker/tests/zec_sign_verify.rs | 1338 | 99c0d053c247b21af1ebc090e6b284bfdc0c0e4f2b559e3720939a591411196b |
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
| docs/testing/BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md | 400 | 46415af46bd4313598c9c30caea3c07b06cb2fdb5d0d6c2ce479919a7c58a869 |
| docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md | 100 | acda820c0aa1b6be7cb8a8eea7a1310dc6b588502c1067d381ea4691795958c3 |

## Collected validation acceptance — 2026-09-09

ACCEPT the real pipeline panic correction and all five bounded validation outcomes.
Hermes outer 5671 completed exit 0 after one owner-triggered collection; no actor
polling or relaunch. Saved session 20260909_000439_f17d68, provider nous, model
poolside/laguna-s-2.1:free. Version output 79565 records Hermes Agent v0.18.2.
The actor queried HERMES_SESSION_ID through execute_code rather than its terminal
environment; unavailable there does not establish that the terminal variable was
unset. Saved session identity is authoritative for this acceptance.

Reviewer verified all five command strings verbatim, both inventory scripts and
both mutation modes against this handoff. Current bytes match all 38 reviewed
identities, and the retained backup equals restored spend.rs. The only runtime
source mutation was the prescribed one-line lock-release omission and restoration.
All tests remain frozen. No third proof, full target, formatter, Git or integration.

| Run | Command / launch or result / completion | Exit | Passed / failed / filtered | Compile / test seconds |
| --- | --- | --- | --- | --- |
| Real pipeline panic green | 79574 / 79575 / 79581 | 0 | 1 / 0 / 15 | 4.94 / 138.88 |
| Lock-release fault red | 79588 / 79589 / 79595 | 101 | 0 / 1 / 15 | 3.30 / 144.91 |
| Restored cleanup library | 79602 / 79603 / same foreground result | 0 | 6 / 0 / 7 | 3.04 / 6.03 |
| Restored signer-error regression | 79606 / 79607 / same foreground result | 0 | 1 / 0 / 15 | 3.37 / 1.70 |
| Restored account-lock regression | 79610 / 79611 / same foreground result | 0 | 1 / 0 / 15 | 0.16 / 8.89 |

Green process proc_271bc7aa975f used waits 79576/79578/79580 and red process
proc_456e4daed36a used 79590/79592/79594, all timeout=60. The two real proof
runs consumed 283.79 seconds of test runtime. The remaining three tests consumed
16.62 seconds. No accepted proof-heavy happy path was repeated.

The positive regression reaches actual signing/proving/finalization/extraction/
independent decoding/verification before the fixed INTERNAL panic. It proves no
verified/broadcast publication, the exact three observed buffer classes at
PanicUnwind, lock release/reacquisition, invalidated old handle/session and no
belated observations through harness Drop. The deliberately broken Drop reaches
INTERNAL at spend.rs:540, passes every preceding stage/publication/hardware guard,
then fails account_lock_count at zec_sign_verify.rs:1291, left 1 / right 0.
It is the intended mechanism failure, not a setup, compile or unrelated panic.

Pre-inventory 79570/79571 and post 79614/79615 each match all 38 rows. Apply
79584/79585 produces the frozen 1159-line b9937a7e... fault. Restore 79598/79599
returns the exact 1160-line 9c41f984... base. Post verifies the retained backup
EQUALS_RESTORED_SOURCE. Keep the initial green valid on those restored identical
bytes; no additional proof run is warranted.

Evidence write 79616/79617 and final measurement 79618/79619 produce
343 lines / 13637 bytes / SHA-256
 a25a5868749a0ac9458712ff91c849f02bd29d483cc3c6ea49a1d1fee531efb4.
The measurement was the final tool operation. The execution authorization is closed.

Record execution/reporting limitations once here, without a correction actor or rerun:

- No process.log call occurred. Reviewer inspected saved completions/foreground
  results directly. Green and the three short outputs retain their outcome evidence;
  the red completion is a 55-line tail beginning partway through a compiler warning.
  It retains all test-stage/failure/result diagnostics, but is not a complete compiler
  log. Do not claim that the missing warning prefix was retrieved.
- Invocations 3-5 ran in foreground rather than authorized background/process-log
  mode. Their exact commands, exit codes and short outputs are present in terminal
  results. This affects capture procedure, not selected tests or outcomes.
- execute_code read PROVIDER and MODEL in addition to HERMES_SESSION_ID, outside
  the requested identity procedure. All returned UNAVAILABLE; no secret/config dump.
- The evidence omits message IDs and actual inventory rows, abbreviates mutation
  scripts as ellipses despite calling them verbatim, and retypes alleged full logs.
  Warning indentation/carets differ; the library report inserts a space inside
  ZecNativeReview; the red report omits the backtrace note and repeated failure list;
  the two restored compile outputs omit their Compiling line. Counts/timings and
  intended assertion are independently verified from saved results above.
- Local absolute repository paths remain in reported logs, contrary to the required
  portability normalization. The report must not be integrated unchanged under
  AGENTS.md. Correct these facts/portability during future bounded integration;
  no report-only actor or test repetition is needed.

This closes the bounded real software-pipeline panic/observed-owner/account-lock
coverage gap. It does not prove all possible panic sites, unobserved third-party
key/prover memory erasure, native OS/owning-thread/capability integration or final
security. Those remaining gaps must not be mislabeled accepted. Prior lifecycle,
signing and decoded-effects results remain valid. Implementation and eighteen
actor evidence records remain uncommitted. No actor is active or authorized;
next reviewer work is to bound native confirmation integration and the remaining
security requirements. High is sufficient for this completed fixed review.
Reviewer publication scope remains this handoff, CURRENT_TASK.md and BBD-WAL-009.md.

## Next-step native integration findings — 2026-09-09

Read-only tracing after acceptance found a runtime integration gap, not another
cleanup regression. wallet-broker/Cargo.toml and src/lib.rs currently define a
library, with no src/main.rs/src/bin executable or explicit binary target.
Native confirmation is a crate-private library adapter; EframeSurface invokes
run_synchronous_zec_review, which creates its own eframe::run_native event loop.
Current native tests exercise egui/dialog/App frames but do not establish a real
broker process or OS event-loop thread as the production authority owner.

social-main.js constructs createWalletSupervisor() with no broker options and has
no walletSupervisor.start call. supervisor.start requires a binary path, hash pin
and data directory before it will spawn. The supervisor's write/receive seam is
exercised through injected objects in tests; production stream framing and ownership
must be reviewed before treating that seam as a working native process connection.
The accepted architecture reserves sidecar packaging/binary pins for WAL-011;
there is no tickets/BBD-WAL-011.md yet. Do not silently call a test-only executable
or another egui simulation the completed product integration.

Next reviewer decision must explicitly separate the remaining WAL-009 native
confirmation evidence from executable/runtime/packaging scope, fix the UI event-loop
owner and request/cancel/close lifetime, and preserve native-only capability minting
and the existing no-Electron-confirm/no-broadcast boundary. Read pinned local framework
code before fixing event-loop/thread behavior. No implementation contract is yet
accepted for this change; no source/tests, runtime launch or actor is authorized.

The reviewer recommends XHigh for this architecture decision and is pausing per
the owner's explicit request to flag reasoning-level changes. The reviewer was on
High for the completed read-only trace; no setting change is claimed. This is an
owner-requested workflow pause, not an AGENTS.md approval requirement. All accepted
pipeline/cleanup results remain valid and must not be rerun.
