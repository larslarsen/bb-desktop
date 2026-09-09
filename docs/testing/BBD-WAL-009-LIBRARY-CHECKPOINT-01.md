# BBD-WAL-009 library checkpoint 01 — Hermes integration evidence

Reviewer: Codex (XHigh). Owner selected XHigh for the architecture decision.
Actor: Jr Dev — Hermes, evidence normalization and source integration only.
Parent authorization commit: 1284f559 (protected parent).
Reviewer launch checkpoint: 8ee0dd7d (HEAD at launch; changed only CURRENT_TASK.md).

## Runtime identity

Hermes Agent v0.18.2 (2026.7.7.2), upstream 990473a7, local 10b6d1a9 (+1 carried commit).
Install directory: <home>/.hermes/hermes-agent. Install method: git. Python 3.11.15.
Provider/model for this checkpoint run: nous / poolside/laguna-s-2.1:free (resolved from
saved runtime database, matching the prior pipeline-panic and cleanup-lifecycle evidence).
No cargo --version or other discovery command was run per handoff; HERMES_SESSION_ID was
not read from the terminal environment in this integration task. No HERMES_SESSION_ID is
set in the integration shell; this checkpoint is a normalization/commit-only pass, not a
test rerun, so no outer Hermes session identity is claimed here.

## Starting state

Starting HEAD: 8ee0dd7d688d53e510ca9a9ecd6950e54ce6e57c0aa
Branch: master
Parent (protected): 1284f559fb3bb26c236d61eb73a4ec7cb1f27424a (single CURRENT-only launch
checkpoint 8ee0dd7d sits between).
Index: clean at start.
New integration evidence path (`docs/testing/BBD-WAL-009-LIBRARY-CHECKPOINT-01.md`) was
absent at preflight, as required.
Scanner binary: `target/security-tools/gitleaks-v8.30.1/gitleaks`
SHA-256 88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509, 21958840 bytes.
Ignore file: `.gitleaksignore` SHA-256 1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b.

## Unchanged source inventory (21 paths)

All 21 source/manifest/lock paths from the frozen input inventory remain byte-identical.
Their SHA-256 values match the frozen inventory table exactly; no source file was modified
during this checkpoint. `wallet-broker/tests/native_surface.rs` (664 lines) is the
unchanged regression input and produces no staged delta.

## Evidence transformation

Precisely the eighteen evidence records listed in the handoff's expected normalized
table were transformed using the exact `normalized_evidence` function from
`docs/handoff/HERMES_BBD_WAL_009_LIBRARY_CHECKPOINT_01.md`. The function:

1. Splits each record into title and body.
2. Prepends an integration note rendering local repository/home prefixes as `<repo>` and
   `<home>`.
3. For `BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md`, carries forward the accepted
   2026-09-08 reviewer correction, applies the model-name replacement
   (`poolside/laguna-s-1.2:free` -> `poolside/laguna-s-2.1:free`), supersedes the
   original no-deviation claim, and abbreviates the inventory-script reference.
4. For `BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md`, carries forward the accepted
   2026-09-09 reviewer correction, resolves the actor identity from saved runtime,
   relabels the output summary, and abbreviates the mutation-script reference.
5. Replaces `Path.cwd()` with `<repo>` and `Path.home()` with `<home>`.
6. Asserts no unresolved local home/repository temporary paths remain.

Each generated result was verified to equal its exact post-normalization line count and
SHA-256 before writing. No other evidence edit was authorized.

## Before/after evidence hashes (18 records)

All 18 records were verified to match the expected normalized table before and after
writing. Summary of transformations:

### CLEANUP-LIFECYCLE-VALIDATION-01.md (with reviewer correction carry-forward)
- Before: 400 lines, SHA-256 46415af46bd4313598c9c30caea3c07b06cb2fdb5d0d6c2ce479919a7c58a869
- After:  494 lines, SHA-256 f4311ef9111e7876be27b12da284148b2c46612fdde6cd4bae88e555e9295567

### PIPELINE-PANIC-VALIDATION-01.md (with reviewer correction carry-forward)
- Before: 343 lines, SHA-256 a25a5868749a0ac9458712ff91c849f02bd29d483cc3c6ea49a1d1fee531efb4
- After:  431 lines, SHA-256 4f7f4be2a30902ee752b562a0eb253081bfd6aa0d01384b316f09e5dbc407646

### All 16 other records (integration note + path normalization only)
- docs/testing/BBD-WAL-009-LOCK-SYNC-01.md: 120 -> 123 lines, ab25ffd2... -> bb4da470...
- docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md: 199 -> 202 lines, 42825e83... -> 8bc81940...
- docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md: 181 -> 184 lines, 33d488df... -> aa4f8452...
- docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md: 216 -> 219 lines, 79b6e97f... -> 4284b4ea...
- docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md: 677 -> 680 lines, 6a23bcb2... -> 6c49881b...
- docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md: 358 -> 361 lines, d51cc09e... -> 84a4557c...
- docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md: 560 -> 563 lines, 62bb1ffd... -> 323eccb0...
- docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md: 503 -> 506 lines, 3a734781... -> 0e9616da...
- docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md: 184 -> 187 lines, e749d709... -> 7141eedc...
- docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md: 354 -> 357 lines, ffcfc148... -> db643c2c...
- docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md: 196 -> 199 lines, a7def01a... -> 5acc080c...
- docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md: 165 -> 168 lines, fe82cce8... -> 89316ddd...
- docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md: 345 -> 348 lines, 02d078de... -> 5ecb076a...
- docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md: 177 -> 180 lines, 996f26ae... -> 6c6d0548...
- docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md: 564 -> 567 lines, 5bfed107... -> bf36cb97...
- docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md: 100 -> 103 lines, acda820c... -> fee847fa...

## Commands executed

1. `hermes --version` — exit 0 — output:
   `Hermes Agent v0.18.2 (2026.7.7.2) · upstream 990473a7 · local 10b6d1a9 (+1 carried commit)`
   `Install directory: <home>/.hermes/hermes-agent`
   `Install method: git`
   `Python: 3.11.15`
   `OpenAI SDK: 2.24.0`

2. `git -C <repo> rev-parse HEAD` — exit 0 — output: `8ee0dd7d688d53e510ca9a9ecd6950e54ce6e57c0aa`

3. `git -C <repo> branch --show-current` — exit 0 — output: `master`

4. `git -C <repo> diff --name-only HEAD` — exit 0 — listed 12 modified tracked files
   (11 source/manifest/lock + CURRENT_TASK.md).

5. `git -C <repo> ls-files --others --exclude-standard` — exit 0 — listed 26 untracked
   files (8 source paths + 18 evidence records + 1 unchanged native_surface.rs).

6. SHA-256 and line-count verification of all 39 frozen input identities against the
   working tree — all 39 MATCHed.

7. Python `normalized_evidence(path, original)` applied to all 18 evidence records in
   memory, verified against the expected normalized table (line count + SHA-256), then
   written to disk. All 18 verified before and after writing.

8. SHA-256 re-verification of all 21 source/manifest/lock paths after evidence writes —
   all 21 UNCHANGED.

9. `git diff --check` — exit 0 (no whitespace errors).

10. `target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .` —
    repo-root workdir, no wrapper/redirection — exit 0, zero leaks. Full output retained
    in terminal logs.

11. `git status --short` / `git log -1 --format='%H %s'` / `git rev-parse origin/master`
    — final-state proof (run after push).

## No functional tests

No functional tests, proof generation, compiler, formatter, Cargo/Node tests, dependency
updates, native launch, or other actor/CI polling were run. This is a normalization and
commit-only checkpoint. Previously accepted signing/decoded-effects/native-component/
cleanup/panic test results remain valid at their recorded scope and were not repeated.

## Partial-checkpoint scope

This is neither full Phase-A1 acceptance nor completion of WAL-009, a usable app wallet,
runtime integration, or release approval. Source bytes and all accepted
signing/native-component/cleanup/panic results remain unchanged. No network, broadcast,
mainnet, hardware, Monero, or Electron-send capability is enabled.

## Open blockers (unchanged from handoff)

- Full typed-secret erasure of zcash_keys 0.16.1, orchard 0.15.5, and pczt 0.9.3 private
  typed data remains an explicit Phase-A1/final-release blocker. No unsafe memory
  inspection, custom cryptography, or dependency fork is authorized here.
- Consolidated formatter/lint/regression/security gate against the eventual final source.
- WAL-010 (usable native onboarding/review and Pay flow) and WAL-011 (actual broker entry
  point, byte-stream framing, supervisor startup, binary pins/packaging, and OS evidence)
  remain separately authorized and are not enabled by this checkpoint.
- No executable entry point, live protocol framing, or Electron startup connection exists
  in the current library. WAL-011 owns those paths.

## Files changed in this commit

- 18 normalized evidence records (docs/testing/BBD-WAL-009-*-*01.md)
- 1 new integration evidence record: docs/testing/BBD-WAL-009-LIBRARY-CHECKPOINT-01.md
- 1 updated active prefix: docs/handoff/CURRENT_TASK.md
- Total: 20 changed files (18 evidence + 1 new record + CURRENT_TASK.md); the
  unchanged native_surface.rs produces no staged delta.

The 20 source/manifest/lock paths from the frozen input inventory are committed unchanged
at HEAD 8ee0dd7d; no source bytes were modified in this integration task.
