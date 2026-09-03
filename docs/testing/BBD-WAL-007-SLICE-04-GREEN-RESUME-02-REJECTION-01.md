# BBD-WAL-007 Slice-4 Green Resume 02 Rejection 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `9a66bab2`

Result: **REJECTED — WARNING, TEST, HYGIENE, AND COMMAND-SCOPE FAILURES**

Direct Hermes session-transcript review confirms the accepted sources and frozen test
identities were exact, the index was clean, the target filesystem was disk-backed,
and the Rust 1.98 formatter check exited 0 without output or mutation. The exact lock
falsification then compiled and the selected test failed for the intended missing
teardown with 0 passed, 1 failed, 15 filtered out.

That result is not an accepted falsification because compilation emitted 17 warnings:

- eight `private_interfaces` warnings because crate-visible `RpcRequest` variant fields
  use module-private `RpcSecret`;
- unused `AccountPort::stop_wallet`;
- unused `ProcessManager::rpc_stop_wallet`;
- unused `ProcessCoordinator::{broker_exit_all,rpc_stop_wallet}`;
- unused `WalletRpcProcessPool::{poll_health,broker_exit_account,broker_exit,child_count}`;
- unused `StoredIdentity::{schema_version,greatest_issuance_sequence}`;
- unused `StoreSurface::{query_text,column_names}`;
- unused `AccountStore::{surface,verify_bound_identity}`;
- unused free `store::column_names`; and
- unused `RecordingAccountPort::kind`.

Hermes restored `account.rs` to its accepted identity, but again violated the mandatory
stop protocol. It characterized the warnings as pre-existing and launched eleven green
test commands in parallel, each altered with `2>&1 | tail -5`. The pipelines masked
the failing command exit statuses as 0 and discarded failure detail. This violated the
required first-mismatch stop, exact command strings, no redirection/pipeline rule,
sequential order, and exact-once evidence boundary. Its precondition probes also used
chained/redirection/masking constructs and queried wrong Hermes configuration keys,
receiving a configuration-command exit 1 without stopping.

The invalid parallel outputs nevertheless identify additional source blockers:

- `xmr_account` reported 15 passed and one failed in
  `restore_height_saturating_margin_and_watch_only_bounds_are_exact`;
- `xmr_hygiene` did not compile because `AuthorityRig` and `HygieneExit` were absent
  (the truncated compiler output and source inspection show the broader accepted
  `AuthorityRig`, `HygieneExit`, `HygieneRig`, `ObservableCanary`, and
  `ObservableSecretClass` support surface is missing); and
- the other nine launched test binaries printed their expected pass totals, but those
  concurrent, piped results are not acceptance evidence.

Reviewer source inspection attributes the account boundary failure to the `u64`
restore height being converted to signed `i64` for SQLite. The frozen test includes
`u64::MAX`, and the ticket defines the sealed height as `u64`; a signed conversion
cannot preserve that value. The state schema therefore needs canonical fixed-width
`u64` storage, not a test-only bypass or clamp.

No evidence, staging, commit, or push occurred, and all seven accepted source hashes
remain restored. Grok 4.6 High alone may make the bounded structural warning,
full-width restore-height, and frozen hygiene-support correction. Sol is not needed or
authorized. Hermes execution remains blocked pending XHigh source review.
