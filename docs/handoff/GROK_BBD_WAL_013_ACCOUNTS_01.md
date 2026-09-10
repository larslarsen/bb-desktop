# WAL-013 account service — tests first

Actor Grok Build grok-4.6 High; no subagents. Owner requested native account management.
Read AGENTS.md, TESTING.md, this contract, CURRENT first 22 lines only. Source reads:
wallet-broker/{Cargo.toml,src/lib.rs,src/vault.rs,src/store.rs,src/session.rs,src/native.rs},
wallet-broker/tests/{vault_crypto,vault_store,vault_session,native_surface}.rs as needed
for APIs/style. No history, unrelated tests, home/config discovery or web.
Only write NEW wallet-broker/tests/account_management.rs and append one [[test]]
name="account_management", path="tests/account_management.rs" to Cargo.toml.
No dependency/feature changes or reformatting existing manifest. No production stub.
No execution, syntax/build/test checks, formatting, evidence, Git or extra actors.
Keep tests focused; stop with file SHA256/line/test counts for review.

## Fixed service contract

Later production ONLY new wallet-broker/src/accounts.rs plus pub mod accounts in lib.rs.
AccountManager<C:MonotonicClock,E:EntropyPort,W:WipeObserver> is a Rust-only service.
with_ports(broker_root:&Path, clock:C, entropy:E, wipes:W) -> Result<Self,AccountError>.
LocalAccountManager alias uses SystemClock (std::time::Instant), OsEntropy, SilentWipes;
LocalAccountManager::open(&Path) creates these production ports. No test hook on wire.
Reuse VaultStore<LinuxStorePort>, seal_vault/open_vault_bytes/parse_vault and
SessionManager for timed secret custody. AccountManager MUST NOT implement Debug or
Serialize; it has no seed getter. Its sessions may use SilentWipes: existing session
wipe proof remains tested by vault_session; supplied W observes new vault operations.

Broker root must be nonempty absolute, existing real non-symlink directory mode0700.
Account directory is fixed broker_root/accounts. Absent directory created0700 through
existing store; existing must already be real non-symlink0700, never chmod hostile
existing paths. Active files use accepted {32lowerhex}.vault names, mode0600 and
canonical bounded v1 encrypted envelope. No other store layout or dependency change.
Read through existing safe bounded Linux store methods; no symlink/FIFO following.
Enumerate at most1024 directory entries and at most256 active accounts. Known
.{32lowerhex}.{16lowerhex}.stage entries may remain from failed encrypted writes;
ignore those only, never follow them. Other unexpected entries, invalid active
names, mode/type/size/canonical/header/filename/asset/network mismatch fail list closed
with UNAVAILABLE. Do not silently omit a corrupt account. Return sorted account IDs.
Validate current catalog before create/restore and enforce256 cap. All listed accounts
are ZEC/zec-testnet/software; existing mainnet/XMR/other payloads are unavailable.
Metadata of locked files is untrusted display data, NEVER spending authority; all
unlock/restore checks authenticate before material enters a session.

Public AccountSummary derives Clone/Debug/Eq/Serialize, exact fields:
account_id:String, asset:&'static str="ZEC", network:&'static str="zec-testnet",
kind:&'static str="software", locked:bool. No balance, address, seed, passphrase,
backup path/bytes, viewing key or invented ready/payment capability.
AccountError has code()->&'static str and Display/Debug fixed closed values; codes
UNAUTH, SCHEMA, LOCKED, UNAVAILABLE, LIMIT, ALREADY_EXISTS. No raw I/O diagnostics.

Public APIs (names/signatures fixed):
- list(&mut self)->Result<Vec<AccountSummary>,AccountError>: check session deadlines
  first; no extension on list/status activity. Restart always returns locked accounts.
- create_software(&mut self, origin:ActionOrigin, passphrase:SecretBytes)
  ->Result<AccountSummary,AccountError>: NativeSurface ONLY, refuse others before
  filesystem/entropy/KDF. Generate fresh 16-byte account ID and 32-byte seed with E;
  epoch1 ZEC testnet VaultMetadata; seal RAW32 seed bytes using accepted vault
  encryption and fresh entropy, persist encrypted bytes via store. Return LOCKED.
  No auto-unlock and no plaintext file. Check ID collision before write; refuse
  ALREADY_EXISTS, never overwrite. Seed scratch is Zeroizing, all error exits wipe.
- unlock(&mut self, origin:ActionOrigin, account_id:&str, passphrase:SecretBytes)
  ->Result<(),AccountError>: NativeSurface ONLY. Validate ID, safely read/parse and
  bind filename/header/network, authenticate/decrypt and require plaintext length32
  BEFORE SessionManager.unlock. Wrong passphrase/corrupt envelope/body length return
  LOCKED, never unlock. No failed action extends another active deadline.
- lock(&mut self, account_id:&str)->Result<(),AccountError>: validates existing
  account and invokes existing manual lock. Locking is safe reduction of authority.
- lock_all(&mut self): invokes existing broker-quit/all-session lock event.
- tick(&mut self)->Result<(),AccountError>: check existing session deadlines; clock
  failures/backwards time fail closed and lock all using existing session behavior.
- export_encrypted(&mut self, origin:ActionOrigin, account_id:&str, destination:&Path)
  ->Result<(),AccountError>: NativeSurface ONLY, absolute chosen path, no overwrites;
  existing safe store export writes exact encrypted active bytes0600, never decrypts.
- prepare_restore(&mut self, origin:ActionOrigin, source:&Path, passphrase:SecretBytes)
  ->Result<PreparedRestore,AccountError>: NativeSurface ONLY. Absolute native-chosen
  regular nonlink0600 bounded file; parse canonical v1, require ZEC testnet, decrypt
  and require raw32 seed. Drop/wipe seed immediately. Return opaque NON-Clone,
  NON-Serialize PreparedRestore owning authenticated ENCRYPTED bytes + metadata.
  Only summary(&self)->&AccountSummary is public; no bytes/path/secret getter.
- confirm_restore(&mut self, origin:ActionOrigin, prepared:PreparedRestore,
  confirmed:bool)->Result<Option<AccountSummary>,AccountError>: NativeSurface ONLY;
  false cancels without writes; true persists CAPTURED authenticated encrypted bytes
  as new locked account, never re-read file path. Refuse ANY already existing ID,
  even newer epoch (replacement not supported in this first management flow).
  Recheck catalog/cap at commit, no overwritten active account or reusable approval.

All consumed passphrases must be wiped even on invalid origin/ID/path/entropy/storage
failure. Existing crypto wipes validated call paths; use crate-visible wipe_with for
outer failure paths and zeroizing seed scratch. Rust Drop covers panic cleanup, do
not claim OS SIGKILL overwrites. No callback/log includes secret bytes. New-account
IDs/seeds generated ONLY inside Rust from OsEntropy in production. No test fixture
seed, environment-secret input or configurable seed generation in running broker.

## Focused independent tests

Use real owned temp directory under wallet-broker/target (disk-backed), unique name,
root mode0700; unlink individually owned files/links, rmdir in reverse, never recursive
delete or follow symlinks. No real user data. Fake clock via shared test handle and
scripted entropy via trait; wipe recorder via shared handle. Production crypto/store
must run. Explicit simple fixtures, no duplicated giant matrix from old vault tests.
Cover about10-12 tests:
1 create real encrypted account; independent open_vault_bytes with known passphrase
  proves exact scripted32 seed; ciphertext differs; no plaintext anywhere in tree;
  exact public summary, mode0600, root/accounts0700 and no auto-unlock.
2 restart lists persisted same locked identity; deterministic ordering and no secret
  canaries in public JSON/error/debug summaries.
3 unlock wrong/corrupt/incorrect-length never unlocks; valid unlock/lock work.
4 fake clock15-minute exact deadline, list doesn't extend; backwards clock locks all.
5 nonnative origin for EVERY privileged API yields UNAUTH before side effects;
  passphrases wiped on refused calls, prepared restore not committed.
6 entropy failure and duplicate generated ID never overwrite an existing vault.
7 encrypted export works while locked, exact byte copy0600; existing/link target refused.
8 authenticated restore then cancel leaves tree unchanged; confirm captured bytes
  after source changed imports captured identity locked, not replacement bytes.
9 wrong password/wrong network/non32 seed/malformed restore rejected; existing ID
  restore never overwrites even if larger epoch; public errors no raw path/canary.
10 hostile root/account directory, active symlink/FIFO/permissions/filename mismatch
  and oversized file/catalog fail closed. Use only minimal cases needed to prove
  new enumeration/composition; preexisting primitive matrices retained.
11 cap boundary256 and257 if inexpensive: synthesize canonical encrypted fixed
  fixtures with bound headers/IDs; do not execute256 KDFs. No spoofed unlocked state.
No test-only exported production authority or bypass. Count real #[test] functions.

Hermes expected red AFTER source review: cargo test --manifest-path wallet-broker/Cargo.toml
--locked --offline --no-default-features --test account_management (Rust1.98).
Expected E0432 unresolved accounts import; no missing dependency, syntax or fixture error.
Green same target; affected regression vault_crypto,vault_format,vault_store,vault_session,
native_surface. Falsify bypassing NativeSurface authorization and prove forbidden-origin
case fails, restore; falsify auto-unlock? use expiration suppression in manager tick/list
and prove deadline test fails, restore. No unrelated signing/proving suite replay.
Exact execution follows review. Security for final combined management: warning-denied
Clippy of changed library/binary/native feature, unchanged npm audit, pinned gitleaks
Git+dir zero leaks, existing security-policy commands with no new boundary failures;
new native dependency graph is unchanged (existing optional eframe/rfd pins only).
Full runtime/UI execution proof is a following contract, still within owner's task.

## Baseline

- wallet-broker/Cargo.toml: 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503
- wallet-broker/Cargo.lock: b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71
- wallet-broker/src/lib.rs: 08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925
- wallet-broker/src/vault.rs: f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49
- wallet-broker/src/store.rs: 611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236
- wallet-broker/src/session.rs: 42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227
- wallet-broker/src/native.rs: a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5
