# BBD-WAL-014 — Native Zcash receive and honest balance status

Status: IN PROGRESS — test source authorized.

Owner reconfirmed the original built-in Zcash wallet inside the broker architecture
and asked to continue after creating/unlocking a WAL-013 account. This slice connects
the existing offline address adapter to that account's native window. Live chain
transport is absent: display “Balance unavailable — not synced”, never a fabricated
zero or a fixture balance. Live synchronization remains separate work requiring its
endpoint/privacy contract. Testnet only; payments remain disabled.

Preserve existing encrypted accounts, vault format, idle locking, native-only secrets,
six-method preload, process/render settings and all unrelated pending npm/policy edits.
No dependencies, package/installer work, renderer, protocol, daemon, mainnet or network.

Source actor: Grok Build grok-4.6 High. Reviewer owns contract/review; Hermes owns all
execution, evidence and source integration. See AGENTS.md and TESTING.md.

Exact test-first contract: docs/handoff/GROK_BBD_WAL_014_RECEIVE_TESTS_01.md.
Authorized tests: wallet-broker/tests/account_management.rs and
wallet-broker/tests/account_native_ui.rs. No Cargo registration is needed.
Later production paths (not yet authorized): wallet-broker/src/accounts.rs,
wallet-broker/src/session.rs, wallet-broker/src/zec.rs, wallet-broker/src/account_ui.rs.
No other source paths. Existing Zcash store/address cryptography is reused.

Required acceptance: focused test red, reviewed production, account/native UI/session
and existing Zcash address/store regressions, native Clippy, staged native rebuild and
real window render/lifecycle smoke, native widget real-manager receive/copy proof,
native-origin and wrong-viewing-key falsifications with exact restoration. Existing
policy six-failure baseline must not grow; pinned directory/committed Gitleaks scans
must find no leaks. No dependency change requires new dependency resolution.

Exact initial red commands (Hermes only, after test review):
`/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test account_management wal014_`
and same command with `--features native-ui --test account_native_ui wal014_`.
Expected red: missing AccountManager::fresh_receiver and AccountUiPort::receive API;
no unrelated compiler failures accepted. Green repeats these and the complete touched
targets. Execution handoffs will pin exact hashes and name the remaining commands.
