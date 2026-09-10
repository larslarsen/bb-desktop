# BBD-WAL-014 — Native Zcash receive and honest balance status

Status: COMPLETE — accepted, rebuilt, committed and pushed.

Owner reconfirmed the original built-in Zcash wallet inside the broker architecture
and asked to continue after creating/unlocking a WAL-013 account. This slice connects
the existing offline address adapter to that account's native window. Live chain
transport is absent: display “Balance unavailable — not synced”, never a fabricated
zero or a fixture balance. Live synchronization remains separate work requiring its
endpoint/privacy contract. Testnet only; payments remain disabled.

Preserve existing encrypted accounts, vault format, idle locking, native-only secrets,
six-method preload, process/render settings and all unrelated pending npm/policy edits.
No dependencies, package/installer work, renderer, protocol, daemon, mainnet or network.

Source actor: Codex Sol gpt-5.6-sol High after Grok exited without a usable test drop;
escalation is recorded in SOL_BBD_WAL_014_RECEIVE_TESTS_01.md. Reviewer owns acceptance;
Hermes owned execution, evidence, source integration and pushes. See AGENTS.md and TESTING.md.

Exact test-first contract: docs/handoff/GROK_BBD_WAL_014_RECEIVE_TESTS_01.md.
Authorized tests: wallet-broker/tests/account_management.rs and
wallet-broker/tests/account_native_ui.rs. No Cargo registration is needed.
Accepted production paths: wallet-broker/src/accounts.rs,
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


## Final acceptance

Source aad0bdd003af823dd6cbfa55994e4692555b3d43 and integration evidence
648050628bf2916ca7f192e7585ce169e3a76833 are pushed to origin/master. Reviewer
independently verified all ten source/test/execution-record commit blobs and input
pins, current HEAD/origin agreement, actual actor metadata/tool calls, and actor exits.

Wallet → Manage accounts now offers Receive for a selected unlocked Zcash testnet
account. It derives an Orchard-only Unified Address from that account's authenticated
seed, binds persisted viewing state to that seed, and durably advances issuance across
restart. The native screen displays the full address and Copy address, with explicit
“Balance unavailable — not synced”. Polling/copy do not issue another address or extend
unlock time. Lock/expiry/removal/error/hide clear the receive screen. Existing encrypted
vault format and account data remain compatible; no user profile was accessed by QA.

Accepted evidence: BBD-WAL-014-RECEIVE-RED-01, GREEN-01/02/03 and INTEGRATION-01 in
docs/testing. 82 distinct Rust tests passed, including seven new tests and real-manager
native pointer/clipboard/database proof. 28 distinct JS groups passed; the native smoke
ran on both Xvfb and the actual desktop. Native-origin and foreign-UFVK falsifications
failed for the intended reason; exact restoration and all seven feature tests passed.
Production Clippy and pinned directory/committed-content Gitleaks passed (5346 commits,
no leaks). Reviewer inspected both host screenshots: Receive is actually rendered,
with shared SHA256 c05a2423a15eee8593a40a0e7d48fdaf18c6d6ec9c51c23b42a0da6c7ab014d0.

GREEN01's expanded test-target Clippy exposed seven byte-identical inherited helper
warnings; established production-only Clippy passed. GREEN02's build runner lacked
rustup on PATH; GREEN03 proved ENOENT and prepended the trusted cargo bin directory for
that single build command. Neither issue required production changes or weakened tests.
Those failed runs remain recorded and are not described as passing commands.

Final Linux x64 staged binary SHA256:
5ed35fe09b1172b2639faa6170621fca60fe99825636398b78fe1bd382509ae4 (496341880 bytes).
Quit and reopen BitBook to load it. Native rendering settings from WAL013 are preserved.

Six inherited security-policy failures and seven inherited test-helper lint warnings
remain. No release/installer/mainnet/live-sync/payment claim. Live balances require the
later endpoint/privacy/synchronization work; this slice never fabricates a zero balance.
All actors exited; no background work or further source authoring is authorized.
Four unrelated npm/policy edits and two historical evidence drafts are preserved.
