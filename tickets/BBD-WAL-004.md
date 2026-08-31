# BBD-WAL-004 — Encrypted Software Custody and Broker-Native Authorization Surface

Status: AUTHORIZED — TEST SOURCE ONLY

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Test source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `abdd2b1980cbe8c5483a26b08b7ae43c82ae420b`

Architecture: `docs/architecture/BBD-WAL-001-REVIEW.md` §§2.2, 3 T01/T08/T09/T17/T20/T21,
5.3–5.4, 6.1–6.2, 10, 13.1, 14, and resolved Q10

## Objective and owner decision

Establish the first Rust wallet-broker crate as an offline, deterministic,
Linux-first encrypted vault core with lock/unlock sessions, fail-safe persistence,
broker-owned backup/restore, explicit zeroization hooks, and the controller contract
for a minimal broker-native authorization window.

The owner selected a native window **inside the Rust wallet-broker process**. It is
not an Electron `BrowserWindow`, renderer page, daemon page, second HTTP service, or
OS credential helper. A native OS file picker started by that broker window may
select backup/import paths. Hardware devices remain separately authoritative for
the fields they can display and confirm. This ticket proves unlock, lock, export,
and restore. Later payment tickets reuse the same surface for prepared-payment
confirmation; this ticket does not construct, sign, or broadcast a transaction.

This is not a usable wallet yet. The encrypted payload is bounded synthetic/opaque
future adapter material. No seed generation, real recovery phrase, ZEC/Monero
library, node, device, chain state, or mainnet behavior is added.

## Fixed v1 vault format

The v1 envelope is closed-schema UTF-8 JSON, at most 128 KiB, with exactly these
logical fields and no aliases:

```text
format              = "bitbook-wallet-vault"
version             = 1
account_id          = 32 lowercase hex
asset               = "ZEC" | "XMR"
network             = the matching WAL-002 Network value
epoch               = canonical positive u64 decimal string
kdf.algorithm       = "argon2id"
kdf.version         = 19
kdf.m_cost_kib      = 65536
kdf.t_cost          = 3
kdf.p_cost          = 1
kdf.salt_b64        = canonical unpadded Base64 for exactly 16 bytes
aead.algorithm      = "xchacha20poly1305"
aead.nonce_b64      = canonical unpadded Base64 for exactly 24 bytes
aead.ciphertext_b64 = canonical unpadded Base64 for ciphertext + 16-byte tag
```

Passphrases are 1–1,024 UTF-8 bytes. Plaintext is an opaque secret byte string of
1–65,536 bytes. Neither is ever JSON,
logged, formatted with `Debug`, rendered, returned to Electron, or placed in a test
fixture. JSON serialization is deterministic in the field order above and ends in
one LF. A parsed document must round-trip to the exact input bytes, so noncanonical
JSON whitespace or field ordering is rejected. Parsing also rejects BOM, CRLF,
whitespace variants in Base64, padding, duplicate or unknown fields, extra bytes,
wrong field types, invalid UTF-8, asset/network mismatch, zero/overflow epoch, and
every non-exact v1 KDF/AEAD parameter **before** performing an attacker-selected
expensive KDF.

Encryption uses the following exact composition:

1. Argon2id v19 with the recorded fixed v1 costs, 16-byte random salt, passphrase
   bytes exactly as entered (UTF-8, no Unicode normalization), and 32-byte output.
2. HKDF-SHA-256 expands that output with no second salt and an unambiguous,
   length-delimited info value containing `BitBook wallet vault key v1`, asset,
   network, and raw 16-byte account id.
3. XChaCha20-Poly1305 encrypts in place with a fresh 24-byte OS-random nonce.
   Additional authenticated data is an unambiguous, length-delimited encoding of
   format/version/account/asset/network/epoch and every KDF/AEAD algorithm and cost
   field except salt, nonce, and ciphertext. Salt and nonce are bound by including
   them in the AAD immediately after that fixed header.
4. Passphrase, Argon2 output, HKDF output, and plaintext buffers use secret wrappers
   and are explicitly zeroized on success, error, cancel, lock, replacement, and
   drop. Ciphertext, salt, nonce, account id, network, and epoch are not secret.

The same passphrase/salt/nonce/plaintext under a different account, asset, network,
or epoch must not decrypt. Social identity is not a valid vault domain or payload
source. There is no compatibility downgrade, parameter negotiation, weak-profile
import, or fallback cipher in v1. Future tuning requires a new reviewed format
version and migration ticket.

## Persistence, backup, and rollback contract

- The broker data directory is private. On Unix it is mode `0700`; each active
  account vault and write staging file is mode `0600`. WAL-004 is accepted on Linux
  only and makes no Windows/macOS custody claim.
- Active filenames are derived only from a validated account id. Every path operation
  rejects symlinks and non-regular files. The reader is bounded before allocation.
- Updates use a same-directory, exclusive, randomized staging file, complete write,
  file sync, atomic replacement, and parent-directory sync. Failure at create,
  permission, write, sync, replace, or directory sync never reports success and
  preserves either the prior authenticated vault or a fail-closed recoverable staging
  state. It must never silently install a partial/new plaintext state.
- Each successful state replacement increments a checked u64 epoch. Equal or lower
  imported epochs are stale and refused when an active account/high-water record
  exists. A higher authenticated epoch may replace only after explicit broker-native
  confirmation. A first restore with no local high-water record also requires explicit
  confirmation. Full rollback of the entire data directory is an acknowledged OS-level
  residual risk; this ticket must not claim tamper-proof monotonic storage.
- Export copies the already encrypted, authenticated vault envelope to a new path
  chosen by the broker-owned native file dialog. It never decrypts for export and
  refuses symlink, directory, device, FIFO, existing target, and self-overwrite paths.
- Restore reads through the broker, authenticates and validates the full envelope
  before showing account/asset/network/epoch metadata, then requires an explicit
  broker-native confirmation. Cancel and failed confirmation leave active state
  byte-for-byte unchanged. Backup bytes and selected paths never traverse Electron.
- Wrong passphrase, corrupted ciphertext/tag, and authenticated-header mismatch expose
  the same public `LOCKED` result and generic native message. Diagnostics may identify
  only operation, account id, and stable error code—not which authentication check
  failed and never secret/ciphertext/backup bytes.

## Lock and secret-lifetime contract

- A software account unlock session is broker-local and defaults to exactly 15 minutes
  of broker authorization idle time, measured by an injected monotonic clock.
- Only successful broker-native authorization activity resets the deadline. Status,
  account-list, snapshot, renderer polling, sync events, failed/cancelled prompts, and
  backup browsing do not extend it.
- Manual lock, timeout boundary, app background, screen-lock notification, broker quit,
  broker restart, panic/error unwinding, account replacement, and restore success wipe
  the in-memory spend material. Restart always begins locked.
- At the exact deadline the session is locked. Backward or overflowing clock input
  fails locked; it never extends a session.
- Viewing capability decisions remain future adapter work. WAL-004 stores no live chain
  state and must not use a spend-secret session merely to service `status.get`.
- Test-visible wipe observers receive only region labels, lengths, and an `all_zero`
  result computed after the wipe. They never receive bytes. The production observer is
  inert, and no test hook is reachable through broker IPC or the native UI.
- Claims remain honest: tests prove explicit wipe operations and drop paths, not that
  allocator slack, registers, swap, core dumps, or the operating system are clean.

## Broker-native surface contract

- The core exposes a controller/port boundary so deterministic tests use a fake surface,
  fake file chooser, fake monotonic clock, fake entropy, and fault-injected store. None
  is reachable through Electron or the broker wire protocol.
- The production surface is an optional `native-ui` crate feature using exact reviewed
  pins. `eframe`/`egui` renders the minimal broker-owned window; password edits are
  masked and non-copyable. No webview, remote content, links feature, inspection port,
  persistence feature, accessibility-tree exposure of a passphrase, telemetry, network
  client, JavaScript, or Electron dependency is permitted. `rfd` supplies a
  broker-invoked native save/open dialog.
- Unlock accepts a bounded passphrase only in the native process. Cancel/window close
  wipes it and returns no partial action. Native error text is closed and secret-free.
- Export and restore paths are returned to the broker core, not to Electron. The surface
  never receives backup bytes or plaintext; it only receives non-secret prompt metadata
  and a success/failure result.
- The future payment-confirm controller is broker-owned by this same toolkit, but no
  Confirm action, `ReviewImageV1`, signing, or broadcast is implemented by WAL-004.
- The first production feature set must explicitly disable eframe browser links,
  inspection, state persistence, and web builds. Native UI dependencies are not default
  authority for headless vault tests.

## Dependency and toolchain contract

The test manifest reserves Rust 1.98.0, edition 2024, and exact direct pins for the
smallest reviewed set: RustCrypto `argon2` 0.6.0, `chacha20poly1305` 0.11.0, `hkdf`
0.13.0, `sha2` 0.11.0, `base64ct` 1.8.3, `zeroize` 1.9.0, `secrecy` 0.10.3,
`getrandom` 0.4.3, `serde` 1.0.229, and `serde_json` 1.0.151. Optional native UI uses
`eframe` 0.36.1 with defaults disabled and only `default_fonts`, `glow`, `wayland`, and
`x11`, plus `rfd` 0.17.2 with defaults disabled and only `xdg-portal` and `wayland`.
Accessibility, links, persistence, inspection, web, and wgpu features are excluded.
Sol must verify actual compatible feature names in the manifest source and report any
contradiction instead of substituting a version.

No git dependency, wildcard, loose major-only requirement, build-time downloader,
vendored binary, OpenSSL, keyring, wallet/coin crate, direct HTTP client, direct general
async network runtime, or unsafe first-party source is allowed. The transitive D-Bus/
portal support selected solely by `rfd` remains subject to lock-graph review and gains
no wallet/network authority. `Cargo.lock` is committed after Luna resolves the accepted
manifest. The reviewer must inspect the complete direct/transitive graph, features,
licenses, build scripts, duplicate cryptographic primitives, and advisory results before
production source is authorized.

Rust and Cargo are not currently installed. The source actor does not install or run
them. When the test drop is accepted, the owner will be asked to install the official
user-level rustup toolchain; no root is required. Cargo home, rustup home, build target,
and temporary build paths remain disk-backed under `/home/lars`, never a substantial
tree in `/tmp`.

## Current authorization — test source only

Codex Sol may create or edit only:

- `wallet-broker/Cargo.toml`
- `wallet-broker/tests/vault_crypto.rs`
- `wallet-broker/tests/vault_format.rs`
- `wallet-broker/tests/vault_store.rs`
- `wallet-broker/tests/vault_session.rs`
- `wallet-broker/tests/native_surface.rs`
- `wallet-broker/tests/secret_hygiene.rs`
- `wallet-broker/tests/fixtures/vault-v1.json`
- `test/securityPolicy.node.js`

`Cargo.toml` is a test-harness/dependency contract in this phase. It must define the
explicit integration-test targets so Cargo parses the tests even though `src/lib.rs`
does not exist. No production source, `Cargo.lock`, toolchain file, `deny.toml`, package
script, policy implementation, workflow, SBOM validator, JavaScript broker boundary,
Electron, renderer, inherited source, evidence, documentation, Git, GitHub, install,
network, command execution, cleanup, or unlisted path is authorized for Sol.

## Required test groups

1. **Crypto and domain separation:** exact algorithm/cost constants; independent
   Argon2id, HKDF-SHA-256, and XChaCha20-Poly1305 published vectors; deterministic
   envelope vector; unique salt/nonce; same-input randomized ciphertext; every
   account/asset/network/epoch/header/salt/nonce/ciphertext mutation fails; ZEC/XMR and
   social-domain substitution fail; wrong passphrase and corrupt tag normalize equally.
2. **Closed format and bounds:** exact golden bytes; round trip; every unknown,
   duplicate, missing, reordered-policy, wrong-type, noncanonical Base64/number/JSON,
   invalid UTF-8, BOM/CRLF/trailing byte, network mismatch, parameter downgrade/DoS,
   overflow, empty/oversize plaintext, 128-KiB envelope boundary, and allocation-before-
   bound rejection.
3. **Store and compound failure:** `0700`/`0600`, symlink/FIFO/directory/device rejection,
   regular-file and size checks, validated filename, create/write/file-sync/replace/
   directory-sync order, each injected failure, failure-during-recovery, prior-byte
   preservation, staging collision, concurrent account update, and no plaintext staging.
4. **Backup/restore/rollback:** ciphertext-only export, exclusive new destination,
   self/symlink/existing-target rejection, full authentication before metadata/confirm,
   cancel preservation, first-import confirmation, higher-epoch confirmation, stale and
   equal refusal, asset/network/account mismatch, corrupt current state, and rollback
   residual stated without pretending to solve full-directory rollback.
5. **Session and clock:** 15-minute exact boundary, only successful native authorization
   resets, polling and failed/cancelled operations do not, manual/background/screen/quit/
   restart/error/restore lock, multiple accounts isolated, backward/overflow clock fail
   locked, and no status path needs a spend secret.
6. **Native surface controller:** fake surface proves only native-origin unlock/export/
   restore actions, masked/non-copyable password configuration, close/cancel wipe,
   generic error text, path-only file-dialog exchange, no backup/plaintext in surface,
   no Electron/HTTP/generic method, and future payment confirmation remains absent.
7. **Secret hygiene:** `Debug`/display/error/log/snapshot/evidence canaries, successful and
   every error/drop/cancel/replace path reports post-wipe all-zero, ciphertext files omit
   the canary, and removing the actual zeroize operation—not merely the hook—fails.
8. **Policy/supply chain:** tests reserve the exact Rust test/build/audit commands,
   toolchain pin, lockfile, first-party `unsafe` prohibition, direct dependency/features
   allowlist, no git/network/coin/keyring dependency, Linux-only claim, routine single-
   platform tests without packaging, pinned RustSec/cargo-deny tools, and manual npm plus
   Rust CycloneDX JSON documents with exact uploads and validation.

Tests use synthetic canaries only. They never contain seed words, a valid mnemonic,
real address, real wallet file, secret key, user path, or mainnet material. Disk boundary
tests use a ticket-defined validated scratch root under the repository's ignored,
disk-backed build tree; they do not use `std::env::temp_dir()`, `/tmp`, recursive removal,
or an unresolved cleanup target.

## Expected red, green, security, and falsification gates

After reviewer acceptance and owner installation of Rust, Luna will receive a separate
handoff. It will resolve and record the accepted lockfile first, then run one named
no-default-features test target. Expected red must be compilation of the authored test
through the missing `bitbook_wallet_broker` library/API—not missing Cargo, missing host
GUI libraries, network failure, or a test that never parsed. Exact commands are frozen
in that handoff after test-source review.

Later targeted green must include all six Rust suites, `cargo fmt --check`, `cargo clippy
--all-targets --all-features -- -D warnings`, native UI feature compilation without
launching a window, and the amended Node policy suite. Broader acceptance includes
`npm test`, `npm run build`, `npm audit --audit-level=high`, repository policy, Gitleaks,
RustSec audit, cargo-deny advisories/bans/licenses/sources, and the manual dual npm/Rust
CycloneDX workflow. Tool pins currently reserved for policy review are cargo-audit
0.22.2, cargo-deny 0.20.2, and cargo-cyclonedx 0.5.9; release asset hashes/action commits
must be fixed by production policy before execution.

At minimum Luna must perform and exactly restore these isolated falsifications after
green:

- remove the post-decrypt zeroize operation while leaving the observer call;
- omit `epoch` or `asset` from authenticated domain data;
- reset the idle deadline on status/snapshot polling;
- reorder atomic replacement before file sync;
- allow stale backup epoch equality;
- add one Electron/native generic unlock or backup bridge; and
- omit Rust components from the manual SBOM output.

No cross-platform package build, native window launch, live wallet, network provider,
hardware, node, coin adapter, real child process, signing, broadcast, or mainnet action
is an acceptance gate.

## Stop conditions

Stop and report rather than improvise if exact dependency versions are incompatible,
the test manifest cannot parse the tests without production source, a native UI feature
requires a network/browser/inspection capability, a test needs real secret material or
`/tmp`, Linux file semantics cannot be injected deterministically, an existing accepted
WAL-002/WAL-003 assertion would be weakened, or any required behavior would place a
passphrase, plaintext, backup bytes, file path, confirm authority, or generic method in
Electron.
