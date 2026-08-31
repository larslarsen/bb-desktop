# BBD-WAL-004 Production Source Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `cde2bc91`

Result: **REJECTED PENDING TEST-FIRST CORRECTION 2**

No Rust, Cargo, Node, npm, formatter, build, scanner, SBOM generation, native window,
wallet, node, device, staging, or production commit ran during this source review.

## Drop integrity and accepted direction

Sol edited exactly the nine production paths authorized by
`CODEX_SOL_BBD_WAL_004_CORRECTION_1_PRODUCTION.md`. The reported 4,433 total lines and
every SHA-256 independently matched. All tests, manifest, lockfile, `deny.toml`, Rust
SBOM validator, package file, manual SBOM workflow, documentation, and unlisted paths
remained byte-identical. `git diff --check` passed.

The correction now uses `SecretSlice`, `Base64Unpadded`, closed diagnostic fields,
pre-prompt account validation, bounded controller passphrases, exact UTF-8 dialog paths,
descriptor-level no-follow operations, closed deny parsing, a seven-file source
inventory, and complete workflow triggers. These changes address the first expected-red
set directionally, but source acceptance is blocked by the findings below.

## Blocking findings

1. `SessionManager::handle` validates `account_id` before dispatching every event.
   Global `AppBackgrounded`, `ScreenLocked`, `BrokerQuit`, and `BrokerRestarted` events
   must lock all sessions regardless of an irrelevant account field; a malformed value
   currently returns `SCHEMA` and leaves every spend session unlocked.
2. `SessionManager::unlock` propagates a monotonic-clock error with `?`. Existing
   sessions are wiped by `read_clock`, but the newly supplied spend material is merely
   dropped and never explicitly wiped through the session observer. The fail-locked
   contract requires the same observable post-zeroization event as overflow/invalid
   unlock paths.
3. The native controller checks only passphrase byte length. `SecretBytes` is a byte
   boundary and a fake/future surface can provide invalid UTF-8; such material currently
   reaches custody even though the fixed contract requires 1–1,024 UTF-8 bytes. Unlock
   and restore must reject and wipe it before custody.
4. `open_existing_regular` checks no-follow plus regular-file type but not the required
   `0600` descriptor mode. `VaultStore::inspect` checks mode on a separate pathname
   lookup, so replacement with a wrong-mode regular file between inspect and open still
   bypasses the private-file invariant. Direct read, write, and sync must reject a
   wrong-mode opened descriptor before reading, truncating, writing, or syncing;
   descriptor-based `set_permissions` remains the operation allowed to repair a regular
   file's mode.
5. Repository source inventory feeds raw `readdirSync` order into an order-sensitive
   pure checker. Directory enumeration order is not stable across filesystems or fresh
   Git checkouts. Security depends on exact membership, not inode insertion order; the
   checker must accept any ordering of the exact seven unique paths while still
   rejecting missing, extra, duplicate, malformed, or non-file entries.

## Required order

Correction 2 begins with regression test source only under
`CODEX_SOL_BBD_WAL_004_CORRECTION_2_TESTS.md`. All 15 production paths remain unchanged
so Luna can demonstrate exact red. Only after reviewer acceptance and expected-red
evidence may Sol receive a three-module/one-policy production correction. No prior test,
dependency, lock, deny file, validator, workflow, or package assertion may weaken.
