# BBD-WAL-007 Slice-4 Formatter Diff 01

Captured by: Jr Dev — Hermes

Normalized by: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `ffde7d03`

Result: **RETAINED RUST 1.98 FORMATTER OUTPUT — NO MUTATION**

Hermes retained the output of the once-only command authorized by the Slice-4 Green
01 handoff:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
```

It exited 1. The formatter identified exactly five paths and 52 mechanical layout
hunks. Rustfmt reported version `1.9.0-stable (88d9e12ae1 2026-08-18)` through the
Rust 1.98 toolchain, and the repository has no `rustfmt.toml`. The check did not
mutate any source or test file.

The following is the retained transformation inventory. Each item describes the
minus-to-plus layout rustfmt emitted; tokens and behavior do not change.

## `wallet-broker/src/xmr/account.rs` — 20 hunks

1. Collapse both `expose_wallet_password` and `expose_primary_address` parameter
   lists and chained bodies.
2. Wrap both `wipe_with` calls after their field accesses.
3. Wrap the crate-visible `derived_paths` parameters onto separate lines.
4. Collapse the first `restore_height` key/value tuple onto one line.
5. Collapse the second `restore_height` key/value tuple onto one line.
6. Put the `creation_secrets_wiped` `wipe_events().iter().any` receiver on the
   condition line and wrap the closure body.
7. Collapse the five arguments to `import_watch_only_inner` and put the opening
   match brace on the following line.
8. Collapse the software `self.success` call onto one line.
9. Collapse the watch-only `self.success` call onto one line.
10. Collapse the public `create_software` signature onto one line.
11. Bind both `primary_text` and `view_text` with the assignment on one line and
    their respective `expose` expressions on the next.
12. Collapse the `catch_unwind(...).is_err()` condition and opening brace.
13. Put `self.last_password =` on one line and the `Some(SecretBytes::new(...))`
    expression on the next.
14. Put `let surface =` on one line and collapse the `PathSqliteSurface::bind_created`
    call on the next.
15. Expand the four-entry vault/state/wallet/keys array in the `for` loop to one
    entry per line and attach the opening brace to the closing bracket.
16. Collapse the hostile-or-partial wallet-presence match arm to one line.
17. Put `let executable =` on one line and the chained installation verification on
    the next.
18. Collapse `exclusive_create_active_envelope` onto one line.
19. Wrap the final `directory.metadata()` chain.
20. Remove the two trailing blank lines.

## `wallet-broker/src/xmr/process.rs` — 11 hunks

1. Collapse the crate-visible `open_wallet` signature onto one line.
2. Put `owned_manager(account_id)?` on its own receiver line and collapse the
   `restore_deterministic_wallet` arguments.
3. Collapse the `get_primary_address` trait signature onto one line.
4. Wrap the `directory.metadata()` chain.
5. Expand the first crate-visible `create_wallet` signature to one parameter per line.
6. Expand the first crate-visible `query_mnemonic` signature.
7. Put `self.coordinator` on its own receiver line and collapse the
   `generate_from_keys` arguments.
8. Apply the same receiver/argument layout to `restore_deterministic_wallet`.
9. Collapse `self.coordinator.prove_owned_session(account_id, network)`.
10. Expand the second crate-visible `create_wallet` signature.
11. Expand the second crate-visible `query_mnemonic` signature.

## `wallet-broker/src/xmr/rpc.rs` — two hunks

1. Collapse the `QueryKeyMnemonic` match arm to the one-line
   `Zeroizing::new(...)` expression.
2. Collapse the `get_primary_address` implementation signature onto one line.

## `wallet-broker/src/xmr/store.rs` — 13 hunks

1. Place the target-gated `rusqlite::OpenFlags` import before the grouped
   `rusqlite::{Connection, OptionalExtension, params}` import.
2. Expand every field of all eight `IDENTITY_COLUMNS` and all five
   `RECEIVER_COLUMNS` `ColumnSpec` values to one field per line.
3. Put `self.primary_address` on a receiver chain and collapse the nested `expose`
   closures.
4. Collapse `exclusive_create_file`'s signature onto one line.
5. Expand the four entries in the unused-value tuple to one entry per line.
6. Put `let identity =` on one line and the `revalidate_opened_file` call on the next.
7. Wrap `self.state_handle.try_clone().map_err(...)` as a receiver chain.
8. Put `let listed =` on one line and collapse the `fs::symlink_metadata(...).map_err`
   expression on the next.
9. Collapse the two-value `Ok((row.get(...), row.get(...)))` tuple.
10. Collapse the receivers-autoindex guarded match-arm header and opening brace.
11. Collapse the three-part column mismatch `if` condition onto one line.
12. Collapse `exclusive_create_state_file`'s signature.
13. Expand `revalidate_opened_directory`'s signature and wrap its
    `directory.metadata()` chain.

## `wallet-broker/src/xmr/test_support.rs` — six hunks

1. Put `self.last_password =` on one line and the `Some(SecretBytes::new(...))`
   expression on the next.
2. Put `self.calls` on its own receiver line and indent the complete
   `AccountRpcCall::restore_deterministic_wallet` push.
3. Collapse the `SecretBytes::new(self.passphrase.expose(...))` construction and
   retain the wrapped `map_err` continuation.
4. Collapse `self.sealed_bytes.as_ref().ok_or_else(...)` and apply the same collapsed
   `SecretBytes::new(self.passphrase.expose(...))` layout to the following passphrase
   construction.
5. Wrap `events.len() == 4 && events.iter().all(...)` as the formatter emitted.
6. Put the final test-vault `SecretBytes::new(...).map_err(...)` expression on the
   line following `let mut passphrase =`.

All other paths and all semantic tokens are outside this formatter correction.
