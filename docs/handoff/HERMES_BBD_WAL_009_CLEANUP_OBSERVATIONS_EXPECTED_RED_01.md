# WAL-009 cleanup observations test-source acceptance and expected red 01

Reviewer: Codex, High; the test contract was fixed at XHigh.
Actor: Jr Dev — Hermes, one test execution and one evidence record only.
Baseline governance: c9d701ec; Grok source authorization: 8ca77e71.
Protected parent: the reviewer commit publishing this handoff.

## Source accepted for expected-red execution

Grok session 9942d648-4b13-43f6-b241-eab7e337a0c8, outer 49550, collected once
following owner done, exit 0; saved runtime grok-4.6-build High. Actor is closed.
Exactly one test appended to wallet-broker/tests/zec_sign_verify.rs: 1215 lines,
SHA-256 c95fa9ae836ca7151f35ef92e664dbc569f24b54b1ff4fb8ad78c1fce997a787.
The original 1118-line prefix exactly reproduces SHA-256
7a481d3a954a92e04d324be047863a824ecf303bfd6232be70e9d13d3a295987.
All other 30 frozen source/evidence identities match. Existing assertions and
imports are unchanged. The new suffix is exactly 97 lines and one test.

Source review confirms one real software signer-fault invocation, exact reached/
unreached call guards, no verified output, released lock and positive Seed cleanup
guards before the final assertion. The four forbidden classes are checked across
all fourteen WipeExit variants and all three counters before harness teardown.
Diagnostics contain only class/exit/counter/count. No fake pipeline or canary setup.

Saved source calls contain one search_replace edit and no compiler/test/formatter/
Git/evidence work. The first post-edit measurement script failed with an undefined
bchr after measuring the final file and original prefix; a corrected read-only
script completed suffix/other-file checks. Record this measurement-script error
without a source/report correction task; reviewer independently verified identities.

## Bounded execution

Create only docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md.
Stop if it exists. All source, tests, prior evidence, dependencies and governance
are frozen. No backup, temporary source mutation, helper file, Git command, source
repair, formatter, no-run/compiler probe, other test, network or actor launch.

Read AGENTS.md, TESTING.md, active CURRENT_TASK.md, ticket active prefix and this
handoff. Work in the repository root for every terminal command. Set the terminal
workdir explicitly; do not issue cd commands that alter persistent shell state.
Use existing wallet-broker/target on the already measured disk-backed ext4
filesystem; do not create another target/cache or rediscover filesystem layout.
Record hermes --version once and only HERMES_SESSION_ID for runtime identity.
Record runtime provider/model if available; otherwise say unavailable so reviewer
can verify the completed-session database. No configuration/environment discovery.

Before execution and after completion, measure the 31 frozen identities once each
using this inline read-only command. No helper file, manually reconstructed path
list, silent retry or repeated measurement. Stop and record any mismatch/error.

```sh
python3 - <<'PY_INVENTORY'
from pathlib import Path
import hashlib
import re
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_CLEANUP_OBSERVATIONS_EXPECTED_RED_01.md')
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff.read_text(), re.M)
assert len(rows) == 31, len(rows)
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

Run this exact command ONCE, in the terminal background with retained output:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

Wait only on that process, at most 60 seconds per wait. Retrieve the complete
process.log on completion; ensure reported total lines are all returned. Preserve
command/launch/process/completion/log identifiers and exact exit/counts/timings.
Do not create an extra log file; saved tool output and the one evidence record
are sufficient. Do not repeat the command on any outcome.

Expected result: exit 101, one failed, 14 filtered. The final assertion must report
forbidden cleanup observations, after the real-signer and positive-Seed guards
passed. Expected forbidden entries are exactly:

```text
ProofWorkspace/SignerError/touch_count/1
ProofWorkspace/SignerError/positive_wipe_count/1
ExtractedTransaction/SignerError/touch_count/1
ExtractedTransaction/SignerError/positive_wipe_count/1
```

Those observations are spurious because prover/extractor never ran. A compile,
setup, earlier guard or crypto failure is not the intended red. Any unexpected
outcome means record the actual result and stop, not repair or retry. This path
stops before proof generation; no old proof-heavy validation is authorized.

## Evidence and final stop

After the post-execution inventory, write the one named record with actual runtime,
source lineage, both complete inventory measurements, exact command/tool identifiers,
exit, test counts/timing, assertion and full saved process output. Preserve output
bytes and indentation; normalize only local absolute paths to portable repo-relative
placeholders and disclose that normalization. Do not invent/retype a test result or
claim normalization while leaving local absolute paths. If any full output is
missing, report the gap and stop without rerunning anything.

Disclose deviations accurately. Measure the completed evidence SHA-256/newline count
once, then stop. No self-hash, subsequent source/backup verification, Git, Node
build/test in response to generic completion prompts, or report-only correction work.
No source/evidence integration or broader acceptance. Reviewer will collect once
after owner done. This expected red cannot establish full cleanup acceptance.

The decoded-effects acceptance and earlier valid results remain in force.
Implementation and thirteen prior actor evidence records remain uncommitted.
Full actual-secret cleanup, native OS/capability integration and remaining security
stay open; no usable send flow is delivered. High is sufficient for this collection;
return to XHigh before cleanup production architecture/acceptance.
Reviewer publication scope: this handoff, docs/handoff/CURRENT_TASK.md and
tickets/BBD-WAL-009.md only. Launch once; record outer identity; never poll actor.

## Frozen inventory

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1125 | 0f47d4f8a7e611997d3276aa090a8585b962b27e820408feea6afd47f14a0b9a |
| wallet-broker/src/zec/test_support.rs | 4373 | f310bdd3ebc95baa5e4f1cf3fd710fb23aaf5fdcfad869eb9bd5105e721be145 |
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

## Collected expected red — accepted

Reviewer: Codex, High, applying the previously fixed XHigh regression contract.
Hermes outer 29552 collected once following owner done, exit 0. Actual runtime
session 20260908_203457_d46097; completed-session database confirms nous,
poolside/laguna-s-2.1:free. Version result 79193: v0.18.2 (2026.7.7.2), upstream
b1f003e1, local 10b6d1a9. Actor is completed and closed.

ACCEPT the understood regression red. Command 79197, launch 79198,
proc_b7d883627e69 (PID 3500044), completion 79200 exit 101. Full saved log 79202
contains all 46 reported lines. Test result: 0 passed, 1 failed, 14 filtered,
1.87s; compilation 2.63s with three recorded dead-code warnings. Final assertion
at tests/zec_sign_verify.rs:1211:5 reports exactly the four entries specified above.
The earlier real-signer, zero-prover/extractor, no-publication, released-lock and
positive-Seed guards all passed. This demonstrates the invented cleanup accounting;
it does not establish actual key erasure or accept production cleanup.

Preflight result 79196 and postflight 79204 each match all 31 frozen identities.
Reviewer independently verified those identities unchanged. New evidence record:
docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md, 177 lines,
SHA-256 996f26aeea6b1cb27799200b24258e0e41defaa05505a0ee2b704c60471c2e9e.
No source/test mutation, retry, additional acceptance command or Git work appears
in the saved calls. The final evidence measurement 79208 was followed by no tool.

The actor's no-deviations claim is inaccurate. It prefixed both Python inventories
with rustup run 1.98.0 and appended 2>&1 to the test command. These are recorded
command deviations; they did not change test arguments, selected test, source or
observed outcome. Its report omits the test redirection, command/completion/log
identifiers, changes warning indentation, and retains local absolute paths contrary
to the normalization instruction. The raw saved log governs. Do not create a
report-only correction task or rerun the valid red for these reporting differences.

No actor, execution or integration remains authorized. Next: XHigh reviewer fixes
the production ownership/observer contract before a source actor is launched.
Remove invented accounting and observe actual owners; do not merely suppress
unreached counters, relabel exits, or weaken the regression. Distinguish actual
owned-byte wiping from third-party object drop; do not claim ordinary drop erases
key memory. Full cleanup/native/security work remains open. The decoded-effects
acceptance and earlier valid outcomes remain in force. Implementation and fourteen
actor evidence records remain uncommitted; no usable send flow is delivered.
