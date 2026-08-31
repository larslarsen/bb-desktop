# Codex Luna Handoff — BBD-WAL-004 Isolated Falsification

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, all WAL-004
tests/source/reviews/evidence, `BBD-WAL-004-GREEN-INTEGRATION-REVIEW.md`, `CURRENT_TASK.md`,
and the complete production/test paths named below.

## Preflight and universal rules

Require `HEAD == origin/master` at the governance parent, clean index/worktree, and all
final hashes from `BBD-WAL-004-GREEN.md`. Use Rust 1.98.0, locked/offline Cargo, the
existing ignored disk-backed target, and no `/tmp`. Do not install, network, stage, commit,
push, delete, invoke a formatter, launch a window, or use Git to restore source during a
mutation. Use `apply_patch` for each exact temporary mutation and its inverse.

For each case: record the baseline hash, apply only the named mutation, run only its
target command, require nonzero exit for the intended assertion (not compile/tool/path
failure), immediately restore the exact bytes with `apply_patch`, verify the baseline
hash and `git diff --check`, then proceed. A test that passes, fails for another reason,
or cannot restore exactly is a blocker. Never leave two mutations active together.

## Seven required falsifications

1. **Real zeroization, not an observer claim.** In `SecretBytes::wipe_with` in
   `wallet-broker/src/vault.rs`, temporarily remove only `bytes.zeroize();`, leaving the
   observer and its post-wipe scan intact. Require intended failure:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene observed_secret_drop_reports_post_wipe_state_not_predeclared_success -- --exact
   ```

2. **Epoch is authenticated.** In `vault_aad` in `wallet-broker/src/vault.rs`,
   temporarily remove only the `metadata.epoch` frame. Require intended failure:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto authenticated_domain_mutations_all_fail_locked -- --exact
   ```

3. **Polling cannot extend authorization.** In `SessionManager::handle` in
   `wallet-broker/src/session.rs`, temporarily route `StatusPolled` through the same
   deadline-reset arm as `NativeAuthorizationSucceeded` and remove it only from the
   no-op arm. Require intended failure:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_session polling_sync_backup_browsing_and_failed_or_cancelled_prompts_never_extend -- --exact
   ```

4. **File sync precedes atomic replacement.** In `VaultStore::write_active_locked` in
   `wallet-broker/src/store.rs`, temporarily swap only `sync_file(&staging)` and
   `replace_atomic(&staging, &active)`. Require intended failure:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_store write_order_is_exclusive_complete_synced_atomic_and_directory_synced -- --exact
   ```

5. **Equal epochs are stale.** In `evaluate_restore` in
   `wallet-broker/src/store.rs`, temporarily change only `candidate.epoch <= current` to
   `< current`. Require intended failure:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_store stale_and_equal_restore_epochs_are_refused_even_when_confirmed -- --exact
   ```

6. **No generic method bridge gains native authority.** In
   `NativeAction::from_method` in `wallet-broker/src/native.rs`, temporarily accept only
   `"account.unlock"` as `Ok(NativeAction::Unlock { account_id:
   "00112233445566778899aabbccddeeff".to_owned() })`, retaining `SCHEMA` for other
   strings. Require intended failure:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface generic_unlock_backup_and_future_payment_confirmation_methods_are_absent -- --exact
   ```

7. **Manual SBOM output cannot omit Rust.** In `.github/workflows/sbom.yml`, temporarily
   remove only the complete `Upload Rust CycloneDX document` step, leaving npm upload and
   Rust generation/validation intact. Require intended failure:

   ```text
   node --test --test-name-pattern="WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts" test/securityPolicy.node.js
   ```

## Evidence and Git

After all seven intended failures and exact restorations, require `HEAD == origin/master`,
every final production/test/workflow hash from green evidence, clean source/test/workflow
diff, and no scratch/canary residue. Create only
`docs/testing/BBD-WAL-004-FALSIFICATION.md` with mutation, exact command, nonzero status,
intended failing assertion, and restored hash for every case. Update only
`docs/handoff/CURRENT_TASK.md` to `FALSIFICATION GREEN — CI SECURITY/SBOM GATES PENDING`,
link the evidence, and set active handoff to `NONE — REVIEWER CI GATE REVIEW`.

Run `git diff --check`. Stage only the evidence and `CURRENT_TASK.md`, inspect the full
staged diff/names, commit once as `test: falsify wallet custody invariants`, and push
master. Require final `HEAD == origin/master`, clean worktree, and report commit,
evidence line/hash, seven exact failure summaries/restored hashes, and push. Do not
manually dispatch workflows; reviewer owns that external action.
