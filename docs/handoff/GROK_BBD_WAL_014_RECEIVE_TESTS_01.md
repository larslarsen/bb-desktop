# WAL-014 receive — tests first

You ARE appointed Sr Dev Grok Build, grok-4.6 High. Parent Codex is reviewer.
Read AGENTS.md, TESTING.md, tickets/BBD-WAL-014.md and this contract. No subagents.
Only WRITE wallet-broker/tests/account_management.rs and account_native_ui.rs in the
same tests directory. Reuse their existing helpers; preserve all existing assertions.
Read relevant existing accounts/session/vault/zec/address/store/account_ui APIs and
existing zec_address.rs oracle as needed. No broad history/docs/config discovery.
Do not execute tests/builds/formatters/checks, Git, network, evidence, production edits
or Cargo changes. Finish a usable bounded source drop, with hashes/counts if available.

## Fixed semantics for later production

AccountManager::fresh_receiver(&mut self, origin:ActionOrigin, account_id:&str)
 -> Result<crate::zec::FreshReceiverV1,AccountError>.
NativeSurface only; other origins UNAUTH before filesystem/session/derivation work.
Validate ID (SCHEMA), current vault identity/catalog, and session deadline. Locked or
expired account -> LOCKED; no address/state creation. Polling/receive does not extend
the original 15-minute deadline. Clock failure/backwards locks all as already defined.

Reuse current encrypted RAW32 seed and existing Zcash AddressAccount SQLite storage.
AccountManager retains broker root PathBuf; session gets only a crate-private scoped
callback (no public seed getter) that checks deadlines and borrows SecretBytes. Copy
only into zeroizing SecretBytes needed by derive_ufvk/bootstrap, wipe temporary copies
on all exits. No extra retained spend key, no secret in result/log/UI/clipboard.

New crate-private zec helper receives root/id/temporary seed/wipe observer. Network is
fixed Testnet. On absent broker_root/zec-testnet/account_id directory, bootstrap the
existing AddressAccount. On ANY present entry, reopen strictly; never rebuild/reset
corrupt or partially initialized state. Compare stored UFVK with UFVK derived from
authenticated account seed before issuing (wrong binding -> UNAVAILABLE). Existing
SQLite account/network binding checks still apply. Reuse existing fresh_receiver(0)
atomic issuance; no alternative address derivation/database schema. Reject symlinks,
unsafe modes, broken state with fixed UNAVAILABLE, LIMIT for exhaustion. New operation
does not modify encrypted vault bytes. Root manager's lifetime lock serializes calls.
First receiver index0/sequence1, next1/2; restart/unlock continues2/3. All receivers
decode as TESTNET UA with Orchard only, independently derived from scripted seed.

AccountUiPort adds receive(&mut self,id:&str)->Result<FreshReceiverV1,&'static str>.
Provide a default Err("UNAVAILABLE") for other Rust implementors, overridden by real
SharedAccountPort to call manager NativeSurface. FakePort records real receive calls.
Native list gets “Receive” button enabled for selected unlocked account only. Clicking
once issues once and opens dedicated receive scene, displaying “Receive Zcash”,
“Zcash testnet”, account ID, full wrapping receiver text, and exact label
“Balance unavailable — not synced”. No numeric balance, sync-ready assertion, QR or
network work. “Copy address” writes ONLY displayed public receiver through egui
copy_text, and “Back” returns list. No address issued/copied automatically by repaint,
poll, selection, unlock, scene open/close. Native Back and next Receive issue a new
address; repaint does not. Address scene is cleared on account lock/expiry/removal,
catalog/port error, selection change, native hide/quit. Copy impossible after clearing.
Existing masked-secret copy/cut/undo protections and hidden startup remain intact.
Keep all controls reachable at minimum360x480; use scrolling if necessary.

## Focused tests

Add names prefixed wal014_, about 5-7 service tests and 3 UI tests, not huge matrices.
1 Real encrypted account/scripted seed -> unlock -> independently derived and decoded
   UA matches. Two issues and persisted restart third; exact index/sequence, vault
   bytes unchanged, SQLite files0600/directories0700, no seed/passphrase in files.
2 Native-only/locked/invalidID/unknown account and exact idle boundary refuse before
   creating viewing state. Receive does not extend idle deadline. No raw errors.
3 Corrupt existing viewing DB and symlink account directory refuse without replacing
   bytes/target. Use exact owned fixtures; cleanup explicit files/dirs, no recursive
   delete. Update existing Scratch cleanup narrowly to support nested owned ZEC dirs.
4 Valid SQLite state with substituted UFVK from a different scripted account seed
   must not issue; assert durable issuance state unchanged. Use upstream oracle or
   existing test_support to create authentic foreign viewing DB, no production hook.
5 Native UI pointer events: selected unlocked Receive calls port once, full receiver
   and unavailable balance label actually painted; repaint issues zero additional;
   Copy output exact public address; Back/newReceive once more. Locked disabled.
6 Expiry/lock/list failure/hide removes receive address and copy action. Existing
   secret canary/platform-output tests remain valid. Check360x480 control geometry.
7 Real SharedAccountPort with real fresh owned manager: create/unlock then actual
   pointer Receive/Copy; independently decode actual public address and inspect
   persisted issuance. No direct fake-port-only substitute for native boundary proof.

Production remains frozen. Reviewer inspects your test source then Hermes runs red.
