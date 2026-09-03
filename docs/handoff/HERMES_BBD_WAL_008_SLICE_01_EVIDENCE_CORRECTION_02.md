# Hermes Handoff — BBD-WAL-008 Slice-01 Evidence Correction 02

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Require clean `HEAD == origin/master` at the protected parent. Read Evidence Reviews
01 and 02 completely. Edit only the green evidence and current-task documents.

In the green evidence command table, set row 1's command cell to this complete literal
string:

```text
cd /home/lars/OpenBazaar/bb-desktop && /home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

Set row 2's command cell to this complete literal string:

```text
cd /home/lars/OpenBazaar/bb-desktop && /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hardware -- --skip persisted_narrowing_reopens_without_expansion_and_requires_a_fresh_exact_restoration --skip write_file_sync_directory_sync_and_commit_faults_publish_nothing_and_preserve_prior_bytes --skip invalid_records_and_reopen_drift_fail_closed_without_ready_publication --skip verified_fields_are_intersected_and_every_omission_is_host_trusting --skip success_error_debug_panic_and_persistence_representations_are_redacted
```

Do not abbreviate either string. Update current-task state and active prose to say
Correction 02 is complete and awaits reviewer acceptance; remove the claim that Hermes
is still authorized to perform Correction 01.

Run no formatter, Cargo, test, Node, npm, product, network, or other actor. Use only
read-only scope inspection and `git diff --check`. Stage exactly those two documents,
commit exactly `docs: complete WAL-008 slice one evidence`, push `master`, verify clean
`HEAD == origin/master`, and stop.
