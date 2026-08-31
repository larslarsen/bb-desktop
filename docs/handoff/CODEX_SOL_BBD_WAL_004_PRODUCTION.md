# Codex Sol Handoff — BBD-WAL-004 Production

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete source prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Implementation source baseline: `fe2fe7e78fab0012a5fa77f128716bb7262aba58`

Protected governance parent: the commit containing this handoff. Its only changes after
the implementation source baseline are the reviewer-authored lock review, authorization,
and routing records; it changes no test, fixture, lockfile, policy, or production byte.

Read completely before editing: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-TEST-SOURCE-REVIEW.md`,
`docs/testing/BBD-WAL-004-EXPECTED-RED.md`,
`docs/testing/BBD-WAL-004-LOCK-GRAPH-REVIEW.md`, and
`docs/handoff/CURRENT_TASK.md`. Read all six committed Rust test suites, their fixture,
the WAL-004 Node policy tests, the existing policy implementation, validators, and three
workflows completely before acting.

## Sole task

Author the smallest production implementation that satisfies the already-committed
BBD-WAL-004 tests and ticket without weakening an assertion. Implement the offline
encrypted v1 vault, Linux fail-safe store, lock sessions, secret hygiene, broker-owned
native controller and optional in-process eframe/rfd surface, plus the exact Rust policy,
routine CI, RustSec/cargo-deny, and manual Rust CycloneDX wiring.

This remains an opaque synthetic custody core. Do not add a seed, mnemonic, address,
coin adapter, wallet/node/device integration, transaction, signing, broadcast, mainnet,
rate provider, Electron authority, listening endpoint, or product wallet API.

## Authorized production paths

You may create or edit only:

- `wallet-broker/Cargo.toml`
- `wallet-broker/src/lib.rs`
- `wallet-broker/src/vault.rs`
- `wallet-broker/src/store.rs`
- `wallet-broker/src/session.rs`
- `wallet-broker/src/native.rs`
- `wallet-broker/src/native_ui.rs`
- `wallet-broker/src/hygiene.rs`
- `deny.toml`
- `scripts/security-policy.js`
- `scripts/validate-rust-sbom.js`
- `package.json`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`
- `.github/workflows/sbom.yml`

Do not edit tests, fixtures, `Cargo.lock`, `package-lock.json`, existing Electron/social
or JavaScript wallet-boundary source, Gitleaks files, documentation, tickets, handoffs,
evidence, or any unlisted path. Do not add or change a dependency, version, or feature.
The only manifest metadata correction is exact `license = "MIT"`.

## Vault and secret implementation

Implement the exact constants, field framing, canonical format, bounds, domain
separation, independent vector, failure normalization, and observer ports reserved by
the tests and ticket. Reject envelope size and every fixed-profile/header/canonicality
error before Argon2. Duplicate and unknown JSON fields must fail; serialization is the
frozen field order plus one LF and parsing round-trips to the exact input bytes.

Use Argon2id v19 at 65,536 KiB/3/1, HKDF-SHA-256 with the fixed length-delimited info,
and XChaCha20-Poly1305 with the fixed AAD. Use OS entropy only through the injected port.
Passphrases, plaintext, Argon output, and expanded keys must be secret-owned mutable
buffers with real zeroization on every success, error, cancellation, replacement,
unwind, and drop path before the test-only observer reports label/length/all-zero. Never
derive `Debug`, expose bytes to an observer, log a secret/ciphertext, or claim OS-wide
erasure.

First-party Rust source must remain entirely safe Rust and offline: no `unsafe`, FFI,
listener/socket/network client, keyring/coin import, process spawn, `temp_dir`, generic
method surface, or secret in an error. Transitive rfd OS-dialog internals are the sole
reviewed exception; do not call `zenity`, D-Bus, or another child/API directly.

## Store, restore, and session implementation

Match the `StorePort` test contract and provide the real Linux port with `0700` data
directories, `0600` files, `symlink_metadata`/regular-file checks, bounded reads,
exclusive randomized same-directory staging, full write, permissions, file sync,
atomic replacement, then parent-directory sync. Never recursively delete. Preserve the
old authenticated value before replacement and fail closed with a complete old/new or
recoverable staging value at compound failure points. Account filename validation,
per-account exclusion, export restrictions, epoch high-water behavior, restore
authentication-before-metadata, and explicit native confirmation must match every test.

The broker-local session uses only the injected monotonic clock and locks at exactly
900,000 ms. Only successful native authorization extends it. Backward time, overflow,
clock error, manual/background/screen/quit/restart/error/restore events wipe and fail
locked. Status/list/snapshot/sync/polling and failed/cancelled operations never request a
spend secret or extend authority.

## Native surface

Keep the controller generic over fake surface/dialog/custody ports. It accepts unlock,
export, and restore only from the native origin. The surface gets non-secret metadata and
results; the dialog exchanges paths only; custody retains backup/plaintext bytes. Generic
unlock/backup calls and payment confirmation remain absent.

Behind only `native-ui`, provide the real minimal in-process eframe surface and rfd
open/save dialog adapter. Do not launch a window in a test or add a binary. The password
editor must be masked, non-copyable, bounded to 1,024 UTF-8 bytes, locally wiped on
close/cancel/error, and unavailable to accessibility exposure. Do not enable or implement
links, inspection, persistence, web, wgpu, telemetry, remote content, JavaScript, or
Electron integration. Eframe's transitive clipboard infrastructure is not authority;
do not copy a passphrase to it. Rfd receives/returns path selection only.

## Policy, deny, and SBOM

Extend `scripts/security-policy.js` fail closed without weakening any accepted check:

- exact Rust/tool pins, manifest dependency/feature allowlist, required library/lockfile,
  and first-party Rust source checks;
- exact WAL-004 routine test/fmt/clippy/native-check, audit, deny, and CycloneDX commands;
- `wallet-broker/**` on every relevant routine trigger and the exact routine Linux Rust
  test with no routine packaging, artifact, install, or native window run;
- pinned cargo-audit 0.22.2 and cargo-deny 0.20.2 installs plus locked audit and
  all-features advisories/bans/licenses/sources checks in the manual/PR security job; and
- pinned cargo-cyclonedx 0.5.9 in the manual-only SBOM job, producing separate validated
  npm and Rust CycloneDX JSON artifacts at the exact tested upload paths.

Use shell `rustup toolchain install 1.98.0 --profile minimal` (adding rustfmt/clippy only
where the job needs them); do not introduce an unreviewed GitHub Action. Keep all existing
Action SHAs, permissions, Gitleaks behavior, audits, path filters, package guards, and npm
SBOM behavior exact.

Add root `deny.toml` with no ignores/skips: deny vulnerabilities, unsound advisories,
yanked crates, wildcard dependencies, unknown registries, and unknown git sources;
retain duplicate-version visibility as warnings because the reviewed optional UI graph
has ten unavoidable platform duplicates. Allow only crates.io and the reviewed
permissive SPDX set: MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause,
BSD-3-Clause, BSL-1.0, ISC, Zlib, 0BSD, Unlicense, Unicode-3.0, OFL-1.1, and
Ubuntu-font-1.0. Do not add a dependency exception or license clarification override.

The Rust SBOM validator must parse a bounded JSON file, require CycloneDX, a valid spec
version, root identity `bitbook-wallet-broker`, nonempty components and dependencies,
and at least the exact direct Rust dependencies. It must reject an npm/desktop root,
empty graph, missing broker/direct components, malformed JSON, and unbounded/non-file
input. Add its syntax check to `npm run build` without changing dependencies.

## Source-actor restrictions and report

Use `apply_patch`. Do not run Cargo, Rust, Node, npm, tests, formatters, linters, builds,
scanners, policy checkers, SBOM generation, Electron, native windows, wallets, nodes,
devices, child processes, network, Git, or GitHub. Do not install, delete, clean, move,
stage, commit, or push anything. Do not use root, `sudo`, `/tmp`, `rm`, globs, or an
unresolved destructive target.

Stop on an API/test contradiction, required dependency/feature/lockfile change, need for
first-party unsafe/network/process authority, or an unlisted path. Otherwise report the
exact changed paths with line counts and SHA-256 hashes, design notes, and any concern.
Codex Luna—not Sol—will inspect the drop, execute targeted green and broader acceptance,
perform every required falsification, write evidence, and own all Git operations.
