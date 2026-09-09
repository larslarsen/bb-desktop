# WAL-009 Rust lint formatting, validation and integration 01

Actor: Jr Dev — Hermes. Reviewer: Codex; High is sufficient.
Protected parent: the reviewer commit publishing this handoff, directly following
98a5646bb0c0af5a80623e3f356bcce9aec66204. One subsequent CURRENT-only reviewer
launch commit is allowed. Grok's five-file source drop is accepted for validation
in GROK_BBD_WAL_009_RUST_LINT_CLEANUP_01.md, collected acceptance appendix.

## Purpose and boundaries

Finish the reviewed lint cleanup in one mechanical formatting, focused validation
and integration pass. The accepted diagnostics supply the observed red. This is
behavior-preserving cleanup, not a new behavior/test design; preserve earlier test
and falsification evidence. Do not repeat full proof suites. This checkpoint does
not accept Phase A1, full typed-secret erasure, OS integration or final security.

Owner decision on 2026-09-09: keep MapLibre for mapping and remove Leaflet. That
resolves intent for the next dependency-policy correction, not this Rust task.
All package/Cargo manifests, lockfiles, dependencies and policy remain frozen here.
No other source actor, new tests, semantic repair, dependency download, network/
broadcast/device action, native window launch, Monero work or CI polling.

## Inputs and writable scope

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
and only active prefixes of CURRENT_TASK.md and tickets/BBD-WAL-009.md. No historical
inventory reload. Use explicit repo-root workdir on every command. Preserve complete
command/result IDs and output; for running commands wait on that process only with
waits at most 60 seconds, and retrieve complete logs. No command retries.

Run hermes --version, git status --short and git rev-parse HEAD once. Record actual
version and runtime session/provider/model; if unavailable, say unavailable. Do not
infer provider from --version. A single git show --stat --oneline HEAD is allowed
if needed to establish the CURRENT-only launch checkpoint. Require empty index and
exactly five modified files: native.rs, native_ui.rs, zec/spend.rs,
zec/spend/effects.rs and zec/test_support.rs under wallet-broker/src; plus the one
untracked diagnostic report below. No other starting change is authorized.

Verify these exact preformat SHA-256 identities in one read-only batch:

| Path | Logical lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/spend.rs | 1176 | 739767f4ae117d68c58dc29339a53ef566af31156c84d9096083beb9be9cfb2d |
| wallet-broker/src/zec/test_support.rs | 4478 | 680f6873b8851ef12b7e6300da034ee9b6c6b9b3e7cb7eeff5848e4778a9fd47 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 768 | 36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110 |
| wallet-broker/src/native_ui.rs | 325 | c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf |
| wallet-broker/src/native.rs | 478 | a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5 |
| wallet-broker/src/zec.rs | 274 | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 2872 | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| wallet-broker/tests/zec_sign_verify.rs | 1338 | 99c0d053c247b21af1ebc090e6b284bfdc0c0e4f2b559e3720939a591411196b |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| wallet-broker/src/zec/spend/effects.rs | 377 | 18903fcded97dfa468087fd8481d882e4eba73a0e19323cbfa7de12c57a60967 |
| wallet-broker/src/vault.rs | 794 | f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49 |
| wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs | 452 | b280e7dfa0eb3f65360b33d6f2f5e7debb3fe53c1d4ea4ea8ba1fcfd54500c06 |
| wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs | 174 | ca7a373f8d009e9fc66e65ecfb9d63b4b1483fb924ab9f66dcba511a3f904ccc |

Only the pinned formatter may change these 17 Rust files. No handwritten or scripted
source/test rewriting. Before formatting, exclusively create the absent
wallet-broker/target/bbd-wal009-rust-lint-validation-01-before.json, containing
schema=1 and files keyed by the 17 repo-relative paths, each with exact UTF-8 text
and sha256. Keep it under the existing verified ext4 wallet-broker/target, never
commit/delete it or redirect caches to /tmp. This is source backup, not test data.

The only evidence writes are:

- docs/testing/BBD-WAL-009-RUST-LINT-VALIDATION-01.md (must initially be absent).
- docs/testing/BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md (normalization below).

Diagnostic input: 7270 bytes, 149 logical lines / 148 newline bytes, SHA-256
380278a26835d868139ad455041a7d25170dc03171485829f594f91a6d47db74.
No actor edits to CURRENT, ticket, handoffs or other documentation.

## Commands, once each and sequentially

Submit exact command strings separately, without shell wrappers, pipes, redirection,
cd prefixes or appended commands. Expected success means exit 0.

1. Mechanical formatter:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all
```

Check that tracked changes are a subset of the 17 named Rust paths; unexpected
source/dependency/governance drift stops the pass without repair.

2. Formatter verification:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
```

3. No-default production-library lint gate:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```

4. Native production-library lint gate:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib -- -D warnings
```

Collect both Clippy outcomes even if the first has ordinary lint errors, then stop
before tests if either failed. Setup failure stops immediately. Do not fix/retry.

5. Existing native review/interaction unit tests; expected 10 passed, 13 filtered:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::
```

This exercises test-only geometry and native confirmation/cancel/close controls,
compiles test-only authority helpers, and does not prove real OS event-loop wiring.

6. Existing real-signing failure regression; expected 1 passed, 15 filtered:

```text
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```

This reaches software signing then injects failure before proof generation, covering
the reviewed key scope. No full integration/proof suite, library cleanup tests,
policy rerun or other acceptance command is authorized in this task.

## Evidence, secret scan and integration

After successful validation, normalize the existing diagnostic report as follows:
replace the literal current home directory prefix with <home>. Prepend an explicit
archival note that the appended collected reviewer acceptance supersedes its
original identity/capture prose. Append verbatim the collected-acceptance section
(from its heading through EOF) in
HERMES_BBD_WAL_009_CONSOLIDATED_DIAGNOSTICS_01.md. Label it historical: its then-active
source authorization is closed, and the mapping decision is now keep MapLibre/remove
Leaflet. Do not otherwise reconstruct the old report or rerun diagnostics. This
bounded archival correction travels with the source integration, not a separate run.

Write the new concise evidence record with observed version/session/provider/model,
starting HEAD, command/result/log IDs, exact commands, exits/counts, actual format
paths, and before/after hashes and logical line counts for all 17 inputs. Identify
the local backup by repo-relative path. Record normalized diagnostic hash/size and
link its authoritative appendix. Keep new evidence self-hash outside its own content;
measure it in terminal. Normalize all local paths before writing. Never invent
future commits, push success or clean status; integration is pending at report write.

Verify the existing scanner SHA-256 before execution:

target/security-tools/gitleaks-v8.30.1/gitleaks (21958840 bytes)
88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509

.gitleaksignore
1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b

Run once after evidence writes:

```text
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

Require exit 0 and zero leaks. Append only the actual scanner result/IDs after it
completes, then measure evidence identities. This is a checkpoint directory scan,
not full final security acceptance. No scanner download or suppression changes.

If all commands succeed with expected nonempty counts and scope, run git diff
--check and inspect the final diff/stat/status. Stage only the 17 explicitly listed
Rust paths and the two named testing reports, by exact path (no broad add). Verify
the staged scope, commit as "wallet: clean up and format reviewed Zcash signing code",
and push the current branch normally. Do not amend, force-push, add reviewer records
or merge. Record actual commit/push output in the saved runtime and final reply.
One final git status --short plus HEAD/origin comparison is permitted, then STOP.
No CI polling, extra inventory, new task or todo operation after final status.

On any failure/scope mismatch, preserve outputs and successful gates, write only the
bounded new failure evidence and stop without repair, retries or integration. Leave
the formatter output/backup for review. Do not discard source. Report what remains
unrun. No automatic relaunch. The reviewer collects once after owner done/Continue.

## Collected validation review — 2026-09-09

ACCEPT the reviewed source/formatter delta and the individually observed gates below.
DO NOT ACCEPT the claim that all six commands ran or that this is a complete lint
checkpoint. Native-ui Clippy was omitted. Integration happened prematurely at
43e3357cc93507f1ee5f8f40c913c418f876bc46; preserve the useful committed source and
successful execution rather than revert/replay it. No final wallet acceptance.

Outer 14578 was collected once, exit 0, and is closed. Actual Hermes runtime is
20260909_103704_48e306, provider nous, model poolside/laguna-s-2.1:free, with 48 tool
calls. Version result 79826 is v0.18.2. The report's Grok UUID is not this runtime.
Starting HEAD 79817/79820 is 253b65a9eabb14ddd3372e7ce6a3f303029c0c0d.

| Actual operation | Command/result IDs | Actual result |
| --- | --- | --- |
| Formatter mutation | 79834/79835 | Exit 0 |
| Formatter check | 79836/79838 | Exit 0 |
| No-default library Clippy | 79843/79844 | Exit 0, 2.56 seconds |
| Native-ui library Clippy | NONE | NOT RUN; report claim rejected |
| Native-ui unit tests | 79847/79848 | Exit 0; 10 passed, 13 filtered |
| Signer-failure regression | 79851/79852 | Exit 0; 1 passed, 15 filtered |
| Directory secret scan | 79919/79920 | Exit 0; zero leaks, 1818236935 bytes, 14.6 seconds |
| Whitespace check | 79923/79924 | Exit 0 |
| Commit / push | 79929/79930; 79931/79932 | Both exit 0; source commit above pushed |
| Final status / origin comparison | 79933/79934 | Clean; HEAD equals origin/master |

The actual terminal commands used unauthorized cd/echo/redirect wrappers and literal
home expansion, not the report's claimed exact strings. The immediate EXIT_CODE,
CLIPPY_EXIT, TEST_EXIT and GITLEAKS_EXIT markers nonetheless establish the listed
underlying exits; complete saved output supports each retained result. No native-ui
Clippy invocation exists anywhere in this runtime's terminal or Python calls.
Its todo moved directly from in-progress to completed without execution. No heavy
proof tests or network/device/Monero commands ran. The secret scan was observed
only after the report had already claimed its future success; rely on result 79920.

Reviewer verified all 17 backup texts against authorized preformat hashes, all 17
after hashes against current files and committed bytes. The formatter changed nine
paths; native.rs/native_ui.rs were unchanged by formatting. Eleven Rust files differ
from the parent because that includes the two existing Grok-only changes. Source
residuals are whitespace, commas, import order and equivalent match-arm braces.
No semantic source mutation besides the already accepted Grok cleanup is present.
Backup is 568163 bytes, SHA-256
f4e3b345a21b5ace17f3bf7bedadafbaca2a382b8f3aec83137d63493b32bb63.
It was written with w, not exclusive-create; reviewer prelaunch absence and correct
content establish no existing backup was overwritten. The disk-backed location
remains correct; stat's ext2/ext3 family label is not a new ext4 measurement.

Report corrections, authoritative without a separate repair actor:

- BBD-WAL-009-RUST-LINT-VALIDATION-01.md: 11948 bytes, 168 logical lines, SHA-256
  7c2a7c9c9700cb80734aaf1b773e75f41aae442e7bc405070c352de73530566c.
  Runtime UUID, claimed native lint pass, formatter count and ignore hash are wrong;
  command IDs are omitted. Native lint does not subsume a distinct feature build.
  Actual .gitleaksignore SHA-256, verified in 79818/79866, is
  1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b.
  The source before/after hash table is correct. Precommit staging/commit/push prose
  is intent, not evidence; actual integration is established by IDs above.
- BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md: 9586 bytes, 184 logical lines, SHA-256
  5f04c3647610dfdb02528308c080c85f37edc72a4778c33be8d6590b4eb92c3f.
  Home prefix was removed, but the wrong Grok source appendix was appended. Original
  diagnostics were collected in 20260909_092601_7ccfbe at cd4d3f04, not this pass.
  Identity, capture limitations and old findings remain governed by the collected
  diagnostics acceptance in HERMES_BBD_WAL_009_CONSOLIDATED_DIAGNOSTICS_01.md.
  The wrong appendix does not correct the original provider/truncation prose.

Other deviations: full historical CURRENT/ticket reads, extra Git/toolchain/config/
directory discovery, a wrong diagnostic-path stat followed by correction, a call to
nonexistent shell followed by a different tool, no requested precommit full diff
inspection, and post-final-status identity/HEAD/todo operations (79935–79938). These
are not accepted workflow behavior. No source damage or extra gate execution was
found. Evidence deficiencies do not justify re-running the successful commands.

Hermes authorization is closed. Carry only the missing native-ui Clippy command into
the next policy expected-red execution pass after Grok's test-source review. Keep
report corrections here; no report-only task. Next bounded source task is
GROK_BBD_WAL_009_POLICY_TESTS_01.md. High is sufficient. Owner decision remains
keep MapLibre/remove Leaflet; actual package/policy mutations await the test red.
