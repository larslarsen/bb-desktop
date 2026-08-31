# Codex Sol Handoff — BBD-WAL-004 Correction 1 Production

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-CORRECTION-1-TEST-SOURCE-REVIEW.md`,
`docs/testing/BBD-WAL-004-CORRECTION-1-EXPECTED-RED.md`,
`docs/testing/BBD-WAL-004-CORRECTION-1-RED-REVIEW.md`,
`docs/handoff/CURRENT_TASK.md`, all accepted tests, and all current production source.

## Sole task and authorized paths

Correct every accepted expected-red failure without changing or weakening tests. You may
edit only these nine existing production paths:

- `wallet-broker/src/vault.rs`
- `wallet-broker/src/store.rs`
- `wallet-broker/src/session.rs`
- `wallet-broker/src/native.rs`
- `wallet-broker/src/native_ui.rs`
- `wallet-broker/src/hygiene.rs`
- `scripts/security-policy.js`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`

Do not edit `wallet-broker/src/lib.rs`, `wallet-broker/Cargo.toml`, `Cargo.lock`,
`deny.toml`, `scripts/validate-rust-sbom.js`, `package.json`, the manual SBOM workflow,
any test/fixture/evidence/document/ticket/handoff, or any unlisted path. Do not add a
dependency, file, feature, compatibility path, platform claim, process, socket, network,
device, wallet/coin behavior, or IPC authority.

## Required Rust correction

1. Session validation and expiry:
   - Validate every authority-moving account identifier with the shared exact 32-byte
     lowercase-hex rule. Invalid unlock returns stable `SCHEMA`, creates no session,
     reads no clock unnecessarily, and explicitly wipes the supplied spend material.
   - A successful native-authorization event at or after the session's existing deadline
     must lock/wipe and return `TIMEOUT` before computing any replacement deadline. Keep
     backward/overflow clock fail-locked behavior and all prior event semantics.
2. Native controller boundary:
   - Reject invalid unlock/export account identifiers with `SCHEMA` before password
     prompt, file dialog, or custody call.
   - For both unlock and restore, reject passphrases outside 1–1,024 bytes as `LOCKED`,
     explicitly wipe them, show only the closed generic error, and invoke no custody or
     restore confirmation. Preserve cancel/window-close wipe behavior.
3. Diagnostics:
   - Accept only the nine exact operations and 13 exact codes enumerated in the accepted
     tests. Validate account identifiers with the shared lowercase-hex rule. Reject every
     other value with `SCHEMA` before allocating diagnostic strings; never include the
     rejected value in an error, format, debug, display, log, or observer.
4. Linux direct file operations:
   - `read_bounded`, `write_all`, `set_permissions`, and `sync_file` must each open the
     selected existing object with Linux no-follow semantics and validate the opened file
     descriptor is a regular file. Use only safe standard-library Rust: `OpenOptionsExt`
     with a local Linux `O_NOFOLLOW` flag and descriptor metadata/operations; no `unsafe`,
     FFI, libc/nix dependency, pre-open-only check, canonicalization, race, subprocess,
     or lossy path conversion.
   - A write must not truncate until after the no-follow descriptor is open and validated.
     Permission and sync operations must act on the validated descriptor, not reopen the
     pathname. Map all symlink/special/open validation failures to stable `UNAVAILABLE`.
     Preserve bounded reads, exact modes, staged ordering, atomic replace, and prior
     fault behavior.
5. Reviewed primitives:
   - Replace `SecretBytes`' raw `Vec<u8>` ownership with `secrecy::SecretSlice<u8>` and
     use `ExposeSecret`/`ExposeSecretMut` only inside the existing bounded closure/wipe
     boundary. Preserve redacted `Debug`/`Display`, explicit observer-after-zeroization
     semantics, replacement/drop wiping, and all public behavior.
   - Replace handwritten vault Base64 with `base64ct::Base64Unpadded` and `Encoding`.
     Remove the handwritten `encode_base64`/`decode_base64` helpers. Preserve exact
     unpadded canonical encoding and rejection of padding, whitespace, and aliases.
6. Native file dialog paths:
   - Replace `to_string_lossy` with exact UTF-8 `Path::to_str`; non-UTF-8 selection fails
     closed with the existing closed native error and never returns a changed path.

## Required policy/workflow correction

- Add `scripts/validate-rust-sbom.js` to both push and pull-request path inventories in
  the social workflow and to every path-filtered trigger in the security workflow. Add
  root `deny.toml` to every security path filter. Update the exported policy constants
  to the exact same order expected by the accepted tests. Do not make the manual SBOM
  workflow routine or change any job/permission/action/command.
- Export the exact seven-path `WAL004_RUST_SOURCE_PATHS` and a pure
  `checkRustWalletSourceInventory(actual)` that rejects missing, extra, duplicate,
  malformed, reordered, or non-string input. `checkRepository` must enumerate actual
  regular `wallet-broker/src/*.rs` directory entries and check the closed inventory,
  then scan exactly each reviewed path.
- Strengthen path-specific `checkRustWalletSource`: `vault.rs` must contain the reviewed
  `Base64Unpadded`, `Encoding`, `SecretSlice`, `ExposeSecret`, and `ExposeSecretMut`
  primitives and must not contain handwritten Base64 helper names; `native_ui.rs` must
  reject `to_string_lossy`. Retain every existing source-authority prohibition.
- Export `WAL004_ALLOWED_LICENSES` in the exact reviewed order and a pure
  `checkWalletBrokerDenyPolicy(text)`. It must enforce `[advisories]` version 2, yanked
  deny, empty ignore and no deprecated advisory-class override; `[licenses]` version 2,
  confidence 0.93, exact allowlist, and empty exceptions; `[bans]` duplicate warn,
  wildcard deny, highlight all, empty allow/deny/skip/skip-tree; and `[sources]` unknown
  registry/git deny, crates.io index only, empty git allowlist. Reject duplicate keys,
  duplicate sections, unknown bypass keys, malformed/non-string/empty input, or any
  weakening without introducing a TOML dependency.
- Add `deny.toml` and the Rust SBOM validator to repository required-file checks, invoke
  the deny policy checker from `checkRepository`, and export all new constants/checkers.
  Preserve the already-passing Rust SBOM validator byte-for-byte.

## Restrictions and report

Use `apply_patch`. Do not run Rust, Cargo, Node, npm, tests, formatters, linters, builds,
scanners, policy, SBOM, Electron, native windows, wallets, nodes, devices, network, Git,
or GitHub. Do not install, stage, commit, push, delete, move, clean, use root/`sudo`, use
`/tmp`, or touch an unlisted path. Stop on a contradiction rather than changing a test,
dependency, lock, validator, or contract.

After edits are complete, only read-only `wc -l` and `sha256sum` over the nine authorized
paths are allowed. Report their exact line counts/hashes and the production corrections.
Luna owns formatting, execution, evidence, integration, Git, and push after reviewer
source acceptance.
