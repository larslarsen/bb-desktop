# WAL-014 receive production — accepted focused red

Principal Dev Codex Sol gpt-5.6-sol High, same session01a08cff-bb38-7ce1-846e-2d081897b596.
You are the source actor; parent is reviewer. Grok escalation remains documented in
SOL_BBD_WAL_014_RECEIVE_TESTS_01.md. Hermes actor59242 exited0, actual session
20260910_133510_857582 / nous / meituan/longcat-2.0:free. Reviewer independently verified
raw wal014-receive-red-01.json, unchanged input hashes, exact launcher and process exit:
account_management failed solely15 E0599 missing fresh_receiver; account_native_ui
failed solely E0407 missing receive trait member. No unrelated compiler error.
Seven new tests are accepted for the fixed behavior, with cleanup refinement below.

Implement the already-read GROK_BBD_WAL_014_RECEIVE_TESTS_01.md semantics NOW. Read this
handoff and focused existing source spans only as needed; APIs/tests are already in
your session. Avoid broad rereads, new research, or stylistic patch churn. Hermes will
format the six touched Rust files. Make a complete concise source drop and stop.

ONLY production paths:
- wallet-broker/src/accounts.rs (baseline f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1)
- wallet-broker/src/session.rs (42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227)
- wallet-broker/src/zec.rs (045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b)
- wallet-broker/src/account_ui.rs (f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7)

Two narrow TEST-helper exceptions, no assertions/behavior weakened: at the beginning
of each new unlink_zec_account(network,account_id), use symlink_metadata on network
itself; absent => return Ok, non-directory/symlink => return Ok without constructing
descendant paths (outer cleanup unlinks network itself). Other errors return a fixed
cleanup error. This prevents following an unexpected network-parent symlink. Existing
account-entry check and exact eight-file cleanup list stay intact.
Test baselines: account_management.rs 84ac590cb389aafb3e21b381c890aca9b0f434811c0398e35d515bc8413f8718;
account_native_ui.rs 783fe38024cc59c83304396eb9764f57540b5661b7622ece7712e61346e63ab5.

Key implementation points:
- Keep native origin guard first. Validate ID/catalog/current vault and session deadline.
  Unknown account UNAVAILABLE; known locked/expired LOCKED. No deadline extension.
- Add only a crate-private scoped session callback borrowing SecretBytes after checking
  deadlines; no public seed getter, secret serialization or extra retained spend key.
- Reuse AddressAccount/StateRoot through a crate-private zec.rs function. Hardcode
  Network::Testnet. Copy seed only into SecretBytes with Drop/wipe handling. Derive
  expected UFVK from the authenticated seed; on absent account directory bootstrap;
  on any present entry strictly reopen, never reset state. Always compare existing
  viewing_key_binding to expected UFVK before fresh_receiver(0). No test_support calls.
  Existing atomic issuance and store checks stay intact. Errors fixed UNAVAILABLE,
  exhaustion LIMIT. Encrypted vault unchanged. Store root is original broker_root.
- New native Receive scene owns the returned public FreshReceiverV1. One explicit
  Receive click => one service call, never on paint/poll. Selected unlocked account only.
  Full wrapped address, exact testnet/title/balance labels from tests, Copy address via
  egui copy_text only on click, Back returns list. Clear scene on stale selection,
  lock/expiry/removal/list failure/hide/quit. Reserve room for new list button at360x480.
- AccountUiPort receive default returns UNAVAILABLE; real SharedAccountPort overrides
  and calls manager with NativeSurface. Existing secret entry/copy protections stay.

No test/build/check/formatter execution, Cargo/dependency changes, Git, evidence,
network, other files or actors. Payments/wire/renderer remain unchanged. Finish hashes
and brief change summary. Reviewer reviews; Hermes executes formatting/green/mutations.
