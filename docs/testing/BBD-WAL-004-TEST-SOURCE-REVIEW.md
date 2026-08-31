# BBD-WAL-004 Test-Source Review

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance HEAD at review: `679b15ef`

Result: **CORRECTION 3 ACCEPTED FOR EXPECTED RED; PRODUCTION NOT AUTHORIZED**

## Accepted uncommitted paths

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 48 | `278c26e3354fca0af5c3b456f8ecdf07defac032b8eb648d8992d3b369feedd7` |
| `wallet-broker/tests/vault_crypto.rs` | 325 | `98306e4fe254ce07f78a75f832701c64a9333111b593fb30a4ba95bf01b1bac1` |
| `wallet-broker/tests/vault_format.rs` | 320 | `7bd7754ea8c17d0d6f0981e82e4627c020839c29fcddbe7026be65ecf8d34877` |
| `wallet-broker/tests/vault_store.rs` | 464 | `5e002b440e220ed5dae0170e2e8077c4d8027fb90ff51b9dd2fb18640c167454` |
| `wallet-broker/tests/vault_session.rs` | 179 | `9a24bee6a7f2e761fcdfa32f461fccecd6810272f66a5fa5deff4b3eb660c55b` |
| `wallet-broker/tests/native_surface.rs` | 293 | `a2fe135d054256ec4eae2350e2f72c8672e533aa2afaae5056c4efa6d52773c9` |
| `wallet-broker/tests/secret_hygiene.rs` | 181 | `c3f73e3e087dab13f7300483859b8671b748dbf750c6672157977e84aad8d590` |
| `wallet-broker/tests/fixtures/vault-v1.json` | 1 | `022f7dc640ef36071c7b4de6347fec4b0b84560a8cbab781403a9eaceaea37e4` |
| `test/securityPolicy.node.js` | 1,778 | `5f8d79fef4176dac6f8f4e912c4056b70c65062e9d14cf29091d413091ca5c12` |

The Rust inventory is 67 tests: crypto 11, format 11, store 19, session 9, native
surface 9, and secret hygiene 8. The Node policy inventory is 64 tests: 58 accepted
pre-WAL-004 tests plus six WAL-004 tests.

## Review findings and corrections

- The initial deterministic crypto check compared production to itself. Correction 1
  freezes u32-big-endian field framing and compares every byte to a complete envelope
  independently computed offline with cached Go `x/crypto` v0.54.0 primitives. The
  Argon2 result, HKDF key, AAD, ciphertext, Base64, canonical JSON, and final LF are fixed
  in the ticket.
- Parameter downgrade/DoS cases now traverse the normal future `open_vault_bytes` path:
  malformed exact-profile fields observe zero KDF calls; the valid fixed envelope
  observes one.
- Store failure tests now distinguish pre-replacement preservation from post-replacement
  directory-sync ambiguity and preserve complete active/staging bytes through recovery
  failure.
- A real Linux boundary test uses `LinuxStorePort`, real `symlink_metadata`, actual Unix
  `0700`/`0600` modes, symlink rejection, and only explicit nonrecursive cleanup beneath
  `target/wal004-scratch/os-<pid>`.
- Correction 2 removes the proposed public mutable port accessor. Fault switching stays
  entirely in the fake through test-owned `Rc<Cell<...>>`; no production invariant
  escape hatch is reserved.

The exact direct pins and feature allowlist were checked against the current official
crate documentation. The optional native UI remains off by default; eframe defaults are
disabled and only `default_fonts`, `glow`, `wayland`, and `x11` are selected. rfd defaults
are disabled and only `xdg-portal` and `wayland` are selected. Actual compatibility,
lock-graph, license, build-script, duplicate-primitive, and advisory review waits for
Luna to resolve the lockfile after the owner installs Rust.

## Reviewer execution

Rust/Cargo were unavailable and were not installed. No Rust test or dependency resolution
ran. `git diff --check` passed. `node test/securityPolicy.node.js` exited 1 with 57 `ok`
and seven `not ok` in the expected source-first state. One of the 58 pre-WAL-004 tests is
deliberately strengthened by the new `wallet-broker/**` path expectation and is red until
the future policy constant changes; the other 57 pre-existing checks pass. The six new
WAL-004 checks are red because future policy/workflow production is absent. No prior
assertion failed for a different reason.

The seven expected-red names are:

- `checker constants match the ticketed Action and tool pins`
- `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`
- `WAL-004 Rust first-party source policy forbids unsafe and unreviewed authority`
- `WAL-004 exact Rust test build lint and native compile commands are reserved`
- `WAL-004 routine Linux CI is single-platform, locked, package-free, and path-filtered`
- `WAL-004 RustSec and cargo-deny gates use exact tool versions and locked inputs`
- `WAL-004 manual SBOM contains separately validated npm and Rust CycloneDX JSON artifacts`

No secret, valid mnemonic, real address, user wallet path, mainnet material, production
source, lockfile, build output, root action, `/tmp` build tree, wallet, node, device, or
network service was introduced. The test source remains uncommitted so Codex Luna can
own expected-red evidence and Git integration after the owner toolchain gate.

## Post-review resolution finding

After the owner installed Rust/Cargo 1.98.0, the first approved crates.io resolution
proved that the accepted manifest's `secrecy = 0.10.3` `alloc` feature does not exist.
Cargo exited 101 before creating `Cargo.lock`; no red test ran. The path hashes above are
the replacement accepted set after Correction 3 removed only the nonexistent feature,
changed the policy expectation to an empty feature list, and added a mutation proving
that optional `serde` is rejected. Defaults remain disabled and the exact version is
unchanged. The reviewer reran `node test/securityPolicy.node.js`: exit 1, 57 `ok`, and the
same seven expected `not ok` names. Luna resume 01 is authorized to retry resolution.
