# BBD-WAL-004 Isolated Falsification Evidence

Governance baseline: `HEAD == origin/master == a6acb1393fccce25137ca3822b2bdd1e716a880d`.
All seven mutations were isolated, used the existing locked/offline disk-backed target,
and were restored immediately with inverse `apply_patch`. No canary or scratch residue
was disclosed or left behind.

1. Removed only `bytes.zeroize();` from `SecretBytes::wipe_with`. The exact
   `secret_hygiene panic_unwind_zeroizes_secret_before_control_returns -- --exact`
   command exited 101 because the test reported `missing post-zeroize observation for
   decrypt-plaintext`. Restored vault hash:
   `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41`.
2. Removed only the `metadata.epoch` AAD frame. The exact
   `vault_crypto authenticated_domain_mutations_all_fail_locked -- --exact` command
   exited 101 on `called Result::unwrap_err() on an Ok value: SecretBytes([REDACTED])`.
   Restored vault hash: `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41`.
3. Routed only `StatusPolled` through the successful-authorization deadline-reset arm.
   The exact `vault_session polling_sync_backup_browsing_and_failed_or_cancelled_prompts_never_extend -- --exact`
   command exited 101 on `StatusPolled extended authorization`. Restored session hash:
   `42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227`.
4. Swapped only `sync_file` and `replace_atomic`. The exact
   `vault_store write_order_is_exclusive_complete_synced_atomic_and_directory_synced -- --exact`
   command exited 101 on the order assertion. Restored store hash:
   `611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236`.
5. Changed only `candidate.epoch <= current` to `< current`. The exact
   `vault_store stale_and_equal_restore_epochs_are_refused_even_when_confirmed -- --exact`
   command exited 101 on `called Result::unwrap_err() on an Ok value: Replace`.
   Restored store hash: `611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236`.
6. Accepted only `account.unlock` in `NativeAction::from_method`. The exact
   `native_surface generic_unlock_backup_and_future_payment_confirmation_methods_are_absent -- --exact`
   command exited 101 on `called Result::unwrap_err() on an Ok value: Unlock`.
   Restored native hash: `50a078f05d8d66127fac0aae99343070758b0da549d5468ed2e0bd71ba0483e9`.
7. Removed only the complete Rust CycloneDX upload step from the SBOM workflow. The
   exact Node test command exited 1 with TAP `tests 1`, `pass 0`, `fail 1` and
   `ERR_TEST_FAILURE`. Restored workflow hash:
   `8407f00fc0ed9ad7bd88c726d64e5cd02a61922653991f9cf4b7cf8bea528824`.

Final verification returned all mutated paths to baseline and `git diff --check` passed.
