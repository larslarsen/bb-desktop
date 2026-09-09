# WAL-009 real pipeline panic source acceptance and expected red 01

Reviewer: Codex, High. Actor: Jr Dev — Hermes, execution/evidence only.
Baseline governance: 9e527129. Protected parent: the commit publishing this handoff.
No source/test mutation or Git is authorized.

## Collected source acceptance

Grok outer 3531 completed exit 0 after one owner-triggered collection. Session
c839c8c4-45a3-4db9-aed2-1d8a97a6f723; saved session model grok-4.6, reasoning
high. No actor polling/relaunch occurred. Its transcript has 18 local read_file,
five grep, two successful search_replace and three read-only Python measurement
commands, with no tests/compiler/formatter/Cargo/Git/evidence/actor execution.
The actor report omitted the explicit session identifier; saved records supply it.

ACCEPT the regression source for one expected-red run. Reviewer read the complete
new test, checked the existing fixed observation/pipeline APIs, and verified all
37 baseline identities: 36 unchanged, plus the reviewed integration target at
1338 lines / 16 tests / SHA-256
99c0d053c247b21af1ebc090e6b284bfdc0c0e4f2b559e3720939a591411196b.
Exact reversal of both replacements reproduces the original 1258-line hash.
The actor deleted 818 bytes (the named 817-byte subsection plus its trailing
newline), not the exact 817 reported. The closing brace and other content are
preserved. Accept that harmless formatting deviation; no correction actor needed.

The new test matches only INTERNAL (&str/String), checks real stage counts before
accepting unwind cleanup, requires no publication/hardware activity, proves lock
release/reacquisition, checks all class/exit cells against fixed expectations,
and requires old-handle/fresh-prepare rejection plus unchanged cleanup through Drop.
Its three positive owner classes are Seed, AuthoritativePczt and ExtractedTransaction.
It never installs canaries, sets counters or synthesizes input wipe events. The
initial helper-only panic must fail seed_accesses 0 versus 1; no compile/runtime
outcome is claimed before this execution.

Owner correction: the reviewer was already on High when this panic task was designed.
Earlier XHigh labels were inaccurate. This collection requires no setting change.
Preserve all accepted cleanup/signing/decoded-effects evidence. No repeated proof-
heavy baseline or report-only correction is authorized.

## Exact scope and sequence

Hermes may create only docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md.
Stop if it exists. All source/tests, dependencies, prior evidence, governance,
backups and Git are frozen. No network, product/UI, hardware, other actor, test
repair, formatter, audit or broader target. Existing wallet-broker/target is the
reviewed disk-backed target; do not rediscover filesystem/configuration or change
cache/target. No extra helper/log/backup files.

Use explicit repository-root workdir on every terminal command, without cd or
shell wrappers. Read AGENTS.md, TESTING.md and only active CURRENT/ticket prefixes
plus this handoff. Do not reload historical material or query Git/status/ancestry.

1. Record hermes --version once. Record only HERMES_SESSION_ID using
   python3 -c 'import os; print(os.environ.get("HERMES_SESSION_ID", "unavailable"))'.
   Report provider/model only if known from actual runtime; otherwise write
   unavailable for reviewer verification. Do not guess from prior reports.
2. Run the inventory command below once before testing.
3. Run the one exact Cargo command below in the terminal background with retained
   output. No wrapper, redirection, pipeline, added option or environment override.
4. Wait only on that process, timeout at most 60 seconds per call. On completion
   retrieve process.log with enough lines for the entire log (page if needed).
   Preserve exact command/launch/process/wait/completion/log identifiers, exit,
   counts and compile/test timings. Do not describe a summary as a verbatim log.
5. Run the same inventory command once after completion, before writing evidence.
6. Write the one evidence record with actual identity, both inventories, complete
   test output, identifiers and actual outcome. Normalize any local absolute
   repository/tool paths to $REPO/$HOME in the evidence and clearly label that
   sole normalization. Do not silently edit diagnostics, counts, timing or hashes.
7. Last tool operation: measure this evidence's SHA-256/newlines/bytes once with
   read-only Python, then final report and stop. No further status/todo/read tool.

If any preflight identity differs, stop before testing. If execution fails to
compile/setup or reaches a different assertion, stop without retry or repair:
perform final inventory, record the actual stop, then final measurement/report.
An unexpected result never authorizes a second test. There is no source mutation
to restore in this phase.

## Inventory command (exactly twice, before evidence creation)

```sh
python3 - <<'PY_INVENTORY'
from pathlib import Path
import hashlib
import re
handoff = Path('docs/handoff/HERMES_BBD_WAL_009_PIPELINE_PANIC_EXPECTED_RED_01.md').read_text()
rows = re.findall(r'^\| ([^|]+) \| (\d+) \| ([0-9a-f]{64}) \|$', handoff, re.M)
assert len(rows) == 37, len(rows)
assert not Path('docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md').exists()
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

## One expected-red invocation

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify real_pipeline_panic_releases_owners_and_never_publishes -- --exact
```

Expected exit 101; one failed, 15 filtered. The caught panic text is INTERNAL.
The following assertion then fails with message
"real pipeline panic must access the operation seed", left 0, right 1.
This absence of real stage activity is the regression, not a compilation failure,
setup error, unrelated panic or failure after a manually fabricated stage count.
No production implementation or green/falsification run is authorized yet.

Reviewer publication scope is this handoff, CURRENT_TASK.md and BBD-WAL-009.md.
Launch once, collect after owner done, never poll. No source/evidence integration.

## Frozen inventory (37 paths)

| Path | Lines | SHA-256 |
| --- | --- | --- |
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
