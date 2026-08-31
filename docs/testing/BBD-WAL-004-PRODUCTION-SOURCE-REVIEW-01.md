# BBD-WAL-004 Production Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `624221fe228d56ea7a7db0400a67a6d1542b1678`

Result: **REJECTED PENDING TEST-FIRST CORRECTION 1**

No production command, Rust compile, test, formatter, linter, scanner, SBOM generation,
native window, wallet, node, device, Git staging, or production commit ran during this
source review.

## Drop integrity

Sol changed exactly the 15 production paths authorized by
`CODEX_SOL_BBD_WAL_004_PRODUCTION.md`; the report's 4,504 total lines and every reported
SHA-256 matched. `git diff --check` passed. All six accepted Rust tests, their fixture,
`test/securityPolicy.node.js`, `wallet-broker/Cargo.lock`, and every unlisted tracked path
remained byte-identical.

The source has no first-party `unsafe`, FFI, socket/network client, direct process spawn,
keyring/coin integration, `temp_dir`, secret logging, mutable dependency pin, or new
dependency. The fixed crypto composition, canonical envelope strategy, staged-store
ordering, explicit zeroization observations, session event split, native-origin
controller, pinned workflows, and bounded Rust SBOM validator are directionally correct.

## Blocking findings

1. `SessionManager::handle(NativeAuthorizationSucceeded)` computes a fresh deadline
   without first rejecting an existing session whose old deadline is already reached.
   If the timeout sweep is delayed, a success event at or after the boundary revives
   expired spend authority, contradicting the exact-boundary rule.
2. Session unlock accepts arbitrary account identifiers. Native unlock/export do not
   validate account identifiers before prompting or invoking custody. Native unlock
   rejects only oversized passphrases, not empty input, and restore performs no controller
   passphrase bound check. These trusted boundaries must reject before authority moves.
3. `DiagnosticEvent::new` accepts any nonempty operation and code and checks only account
   identifier length. A caller could place secret text in a diagnostic field. Operations
   and stable codes need closed allowlists, and the account identifier needs the exact
   lowercase-hex validator.
4. `LinuxStorePort::read_bounded`, `write_all`, `set_permissions`, and `sync_file` open or
   mutate a path without their own no-follow/regular-file validation. The earlier
   `VaultStore::inspect` is a separate operation and cannot enforce the invariant against
   replacement. Direct calls on a symlink currently follow the link; deterministic Linux
   regression coverage must prove each direct port operation rejects it and preserves
   target bytes/mode.
5. `RfdDialog` uses `to_string_lossy`, which can silently change the owner-selected path.
   A non-UTF-8 path must fail closed rather than target a different string path.
6. The reviewed direct dependencies `secrecy` and `base64ct` are unused. `SecretBytes`
   owns a raw `Vec<u8>` and vault Base64 is handwritten. Production must use
   `secrecy::SecretSlice` with explicit exposure/zeroization and
   `base64ct::Base64Unpadded`; the exact pins are not decorative.
7. `scripts/validate-rust-sbom.js` is absent from the social and security trigger
   inventories, and root `deny.toml` is absent from the security trigger. An isolated
   policy/validator change could therefore evade its relevant CI. The source policy also
   scans only a fixed expected list without proving no extra `wallet-broker/src/*.rs`
   exists, and it does not validate the deny policy itself.

## Required order

Correction 1 begins with regression test source only under
`CODEX_SOL_BBD_WAL_004_CORRECTION_1_TESTS.md`. The current flawed production drop remains
unchanged so Luna can later demonstrate those tests failing for the reviewed reasons.
Only after reviewer test-source acceptance and exact expected red will Sol receive a
separate bounded production correction. No existing assertion may be weakened and the
lockfile/dependency graph remains frozen.
