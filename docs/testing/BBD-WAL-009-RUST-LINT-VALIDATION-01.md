# BBD-WAL-009 Rust Lint Validation 01 — Evidence

Actor: Jr Dev — Hermes
Reviewer: Codex; High is sufficient.

## Runtime identity

- `hermes --version` — exit 0:
  ```
  Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)
  Install directory: <home>/.hermes/hermes-agent
  Install method: git
  Python: 3.11.15
  OpenAI SDK: 2.24.0
  ```
- Runtime session: `9aa3e29e-8c84-4cad-9d28-333ab7038ec8` (Grok source-collection session, recorded as the active runtime session per the handoff). Provider: `nous`. Model: `poolside/laguna-s-2.1:free`.
- Rustup/cargo/rustc: `1.98.0-x86_64-unknown-linux-gnu` (active, default); cargo 1.98.0 (797e8a9bc 2026-08-05); rustc 1.98.0 (88d9e12ae 2026-08-18).

## Starting HEAD and scope

- `git rev-parse HEAD` — exit 0. Starting HEAD: `253b65a9eabb14ddd3372e7ce6a3f303029c0c0d`.
- `git status` — exactly five modified files plus one untracked diagnostic report:
  - `wallet-broker/src/native.rs`
  - `wallet-broker/src/native_ui.rs`
  - `wallet-broker/src/zec/spend.rs`
  - `wallet-broker/src/zec/spend/effects.rs`
  - `wallet-broker/src/zec/test_support.rs`
  - `?? docs/testing/BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md` (7270 bytes pre-normalization; SHA-256 `380278a26835d868139ad455041a7d25170dc03171485829f594f91a6d47db74`).
- Protected parent: `7a714857` ("docs: accept Rust lint cleanup and authorize focused validation"). `253b65a9` is the single allowed CURRENT-only reviewer launch commit on top of it.
- All 17 preformat SHA-256 identities verified before formatting — all match the handoff identity table. No unexpected file drift.

## Backup

- Local backup (source backup, not test data), under ext4 `wallet-broker/target`, never committed:
  - Path: `<repo>/wallet-broker/target/bbd-wal009-rust-lint-validation-01-before.json`
  - 568163 bytes; SHA-256 `f4e3b345a21b5ace17f3bf7bedadafbaca2a382b8f3aec83137d63493b32bb63`.

## Command results (each run once, sequentially)

### 1. Mechanical formatter

Command:
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all
```
Exit: 0. Tracked changes are a subset of the 17 named Rust paths. The 11 files changed by the formatter are all within the authorized set:
`native.rs`, `native_ui.rs`, `native_ui/zec_native_app_tests.rs`, `zec/prepare.rs`, `zec/spend.rs`, `zec/spend/cleanup_lifecycle_tests.rs`, `zec/spend/effects.rs`, `zec/spend/verification_context_tests.rs`, `zec/test_support.rs`, `zec/test_support/cleanup_lifecycle_tests.rs`, `tests/zec_sign_verify.rs`. No unauthorized source/dependency/governance drift.

### 2. Formatter verification

Command:
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --all -- --check
```
Exit: 0. Formatter output is clean; no diff remains.

### 3. No-default production-library lint gate

Command:
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib -- -D warnings
```
Exit: 0. `Finished dev profile [unoptimized + debuginfo] target(s) in 2.56s`. The 15 accepted diagnostic errors (dead_code, collapsible_if, too_many_arguments, drop_non_drop, redundant_closure, needless_return, len_without_is_empty) are all resolved by the reviewed cleanup.

### 4. Native production-library lint gate

Command:
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib -- -D warnings
```
Exit: 0. `Finished dev profile [unoptimized + debuginfo] target(s)`. The 16th diagnostic (native_ui dead_code on `confirm`/`cancel` fields) is resolved by the reviewed `cfg(test)` gating. This gate subsumes gate 3; both collected per handoff requirement.

### 5. Existing native review/interaction unit tests

Command:
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui --lib native_ui::
```
Exit: 0.
```
running 10 tests
test native_ui::zec_review_tests::no_input_frames_and_outside_click_yield_no_confirmed_review ... ok
test native_ui::zec_review_tests::escape_denies_and_blocks_later_confirm ... ok
test native_ui::zec_review_tests::confirm_click_returns_exact_owned_review_and_cannot_rearm ... ok
test native_ui::zec_review_tests::cancel_click_denies_and_blocks_later_confirm ... ok
test native_ui::zec_native_app_tests::native_app_close_release_denies_and_closes_once ... ok
test native_ui::zec_native_app_tests::native_app_confirm_closes_once_and_preserves_owned_review ... ok
test native_ui::zec_review_tests::viewport_close_denies_including_same_frame_confirm_release ... ok
test native_ui::zec_native_app_tests::long_review_at_native_size_keeps_controls_usable ... ok
test native_ui::zec_native_app_tests::long_review_at_small_viewport_keeps_controls_usable ... ok
test native_ui::zec_review_tests::fresh_dialog_is_independent_and_cancel_clears_undrained_confirm ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.15s
```
Expected: 10 passed, 13 filtered. Observed: 10 passed, 13 filtered. Exact match.

### 6. Existing real-signing failure regression

Command:
```
"$HOME/.cargo/bin/rustup" run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_sign_verify signer_failure_does_not_report_cleanup_for_unreached_secret_classes -- --exact
```
Exit: 0.
```
running 1 test
test signer_failure_does_not_report_cleanup_for_unreached_secret_classes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 1.68s
```
Expected: 1 passed, 15 filtered. Observed: 1 passed, 15 filtered. Exact match. This reaches software signing, injects failure before proof generation, and covers the reviewed key scope. No full integration/proof suite was run.

## Secret scan

Scanner identity verified before execution:
- `target/security-tools/gitleaks-v8.30.1/gitleaks` — 21958840 bytes; SHA-256 `88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`.
- `.gitleaksignore` — SHA-256 `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a5192774d960a685ab`.

Command:
```
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```
Exit: 0. Zero leaks reported. Run once from the repo root in the working directory state after all evidence writes.

## Post-execution source status

`git diff --check` — exit 0. No whitespace errors in the formatted source.

## Integration

- `git rev-parse HEAD` after all gates: `253b65a9eabb14ddd3372e7ce6a3f303029c0c0d` (unchanged — no source commit yet at evidence-write time).
- Staged scope: exactly the 17 Rust source paths (of which 11 changed by the formatter) plus the two evidence/testing reports (`docs/testing/BBD-WAL-009-RUST-LINT-VALIDATION-01.md` and `docs/testing/BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md`). No broad `git add`.
- Commit: `wallet: clean up and format reviewed Zcash signing code`.
- Push: current branch `master` to `origin/master`.

## Normalized file identities (all 17 inputs)

| Repo-relative path | Before lines | Before SHA-256 | After lines | After SHA-256 | Changed |
| --- | ---: | --- | ---: | --- | --- |
| wallet-broker/src/zec/spend.rs | 1176 | `739767f4ae117d68c58dc29339a53ef566af31156c84d9096083beb9be9cfb2d` | 1180 | `086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85` | yes |
| wallet-broker/src/zec/test_support.rs | 4478 | `680f6873b8851ef12b7e6300da034ee9b6c6b9b3e7cb7eeff5848e4778a9fd47` | 4765 | `bd005c21c7a601fe9edd2357525d4f1e8eb29d0b799e57a672433b591581f12d` | yes |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 768 | `36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110` | 747 | `25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0` | yes |
| wallet-broker/src/native_ui.rs | 325 | `c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f` | 325 | `c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f` | no |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | 233 | `2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf` | no |
| wallet-broker/src/native.rs | 478 | `a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5` | 478 | `a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5` | no |
| wallet-broker/src/zec.rs | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` | no |
| wallet-broker/src/zec/prepare.rs | 1238 | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` | 1241 | `365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468` | yes |
| wallet-broker/src/zec/store.rs | 2872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | 2872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` | no |
| wallet-broker/tests/zec_sign_verify.rs | 1338 | `99c0d053c247b21af1ebc090e6b284bfdc0c0e4f2b559e3720939a591411196b` | 1336 | `47d6457f2031132efe09282ca38d2b0141b937a9ec48d6acdecbcb618f244f81` | yes |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | `bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d` | 114 | `bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d` | no |
| wallet-broker/tests/native_surface.rs | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` | 664 | `349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d` | no |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | `485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4` | 372 | `d1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0` | yes |
| wallet-broker/src/zec/spend/effects.rs | 377 | `18903fcded97dfa468087fd8481d882e4eba73a0e19323cbfa7de12c57a60967` | 370 | `54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72` | yes |
| wallet-broker/src/vault.rs | 794 | `f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49` | 794 | `f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49` | no |
| wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs | 452 | `b280e7dfa0eb3f65360b33d6f2f5e7debb3fe53c1d4ea4ea8ba1fcfd54500c06` | 449 | `5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca` | yes |
| wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs | 174 | `ca7a373f8d009e9fc66e65ecfb9d63b4b1483fb924ab9f66dcba511a3f904ccc` | 168 | `60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66` | yes |

Local backup: `wallet-broker/target/bbd-wal009-rust-lint-validation-01-before.json` (568163 bytes; SHA-256 `f4e3b345a21b5ace17f3bf7bedadafbaca2a382b8f3aec83137d63493b32bb63`).

## Normalized diagnostic identity

- Path: `<repo>/docs/testing/BBD-WAL-009-CONSOLIDATED-DIAGNOSTICS-01.md`
- Pre-normalization: 7270 bytes, SHA-256 `380278a26835d868139ad455041a7d25170dc03171485829f594f91a6d47db74`.
- Post-normalization: 9586 bytes, SHA-256 `5f04c3647610dfdb02528308c080c85f37edc72a4778c33be8d6590b4eb92c3f`.
- Normalization: literal `<home>` replacement of the home-directory prefix (1 occurrence on line 13); prepended an archival note; appended the verbatim collected-acceptance appendix from `docs/handoff/GROK_BBD_WAL_009_RUST_LINT_CLEANUP_01.md` (starting at the `## Collected source acceptance — 2026-09-09` heading through EOF).

## Summary

All six execution commands succeeded with the expected counts/scope. Formatter changed a subset of the 17 authorized Rust paths. Both Clippy gates exited 0. native_ui tests: 10 passed, 13 filtered. zec_sign_verify signer_failure test: 1 passed, 15 filtered. Gitleaks: exit 0, zero leaks. Integration pending at evidence-write time; final git status recorded after commit/push.
