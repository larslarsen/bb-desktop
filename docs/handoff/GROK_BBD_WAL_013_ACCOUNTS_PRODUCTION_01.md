# WAL-013 account service — production source only

Reviewer accepted Hermes expected red: HERMES_BBD_WAL_013_ACCOUNTS_RED_01.md closure;
Rust1.98 E0432 only missing accounts, fixed test ddb292489e73b4f25d72133d98fdf204163f13254f5fa44a3a297bffbc3b6fae (1508lines12tests).
Grok grok-4.6 High source-only. Resume6481c6a3 known API/test context. Read this and
original GROK_BBD_WAL_013_ACCOUNTS_01.md INCLUDING concurrency refinement. Use already
read vault/store/session APIs, avoid broad new reading/research. Complete promptly.
ONLY write NEW wallet-broker/src/accounts.rs and add pub mod accounts; to src/lib.rs
(baseline08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925).
All tests/Cargo/dependencies/existing primitives frozen. No production stub/test
shortcut, test execution/build/format/checks, Git, evidence or other actors/subagents.
Implement original fixed public API, native-origin checks, actual encrypted storage,
zeroizing secrets, authenticated captured restore, strict catalog bounds/modes and
per-root directory File::try_lock lifetime exclusion. Existing Linux constants match
store; no unsafe/dependencies. Session-erasure before releasing directoryguard onDrop.

Use entropy labels account-id,account-seed,vault-salt,vault-nonce,staging-name fixed by
reviewed test fixtures. SystemClock and SilentWipes public Rust port types can exist
for usable public LocalAccountManager alias. No secret getter/Debug/Serialize onmanager.
PreparedRestore only public summary, no Debug required. Return closed AccountError
codes; malformed prepare envelope SCHEMA; wrongnetworkUNAVAILABLE; wrongpass/non32LOCKED;
root/catalogboundsLIMIT; symlink/existing exporttargetALREADY_EXISTS; catalogmalformed
UNAVAILABLE. Failed origins UNAUTH beforeentropy/filesystem/KDF, consumedpassphrase
observed wiped on EVERY return. Use existing crypto wipe labels passphrase/plaintext.
Never unwrap lock/IO/entropy. Production manager validates current accountsdir before
reads so post-open hostile directory substitution isn't silently followed. All payments
remain unavailable; no signing/address/network/UI/runtime edits in this source drop.

Finish with exactchangedpaths/SHA256/lines and summary, no execution. Reviewer next
checks source; Hermes owns green/falsification/security/exactintegration later.
