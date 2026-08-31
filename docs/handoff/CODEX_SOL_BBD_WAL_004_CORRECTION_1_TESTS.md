# Codex Sol Handoff — BBD-WAL-004 Correction 1 Test Source

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-01.md`,
`docs/testing/BBD-WAL-004-LOCK-GRAPH-REVIEW.md`,
`docs/handoff/CURRENT_TASK.md`, all five authorized test paths below, and the current
uncommitted production source/policy drop.

## Frozen pre-correction source

The 15-path production drop is intentionally uncommitted and must remain byte-identical
during this phase. Its exact line counts and SHA-256 values are the table in Sol's source
report and can be reverified by the reviewer; do not edit any production, policy,
manifest, workflow, validator, package, deny, lockfile, fixture, evidence, documentation,
ticket, or handoff path.

## Sole task and authorized paths

Author regression **test source only** for every blocking finding in production source
review 01. You may edit only:

- `wallet-broker/tests/vault_store.rs`
- `wallet-broker/tests/vault_session.rs`
- `wallet-broker/tests/native_surface.rs`
- `wallet-broker/tests/secret_hygiene.rs`
- `test/securityPolicy.node.js`

Preserve every accepted test and assertion. Add focused tests; do not rewrite existing
coverage or couple behavioral Rust tests to an alternate implementation.

## Required Rust regressions

1. Session: at exactly the existing authorization deadline, a late
   `NativeAuthorizationSucceeded` event must return stable `TIMEOUT`, lock the account,
   and report a real all-zero spend-material wipe. Add a separate invalid-account unlock
   case requiring stable `SCHEMA`, no session, and a wipe of the supplied material.
2. Native controller: invalid account identifiers for unlock/export must fail `SCHEMA`
   before prompt, dialog, or custody. Empty and 1,025-byte passphrases must fail `LOCKED`
   before custody for both unlock and restore, report a native-passphrase all-zero wipe,
   and never commit/partially act.
3. Diagnostics: retain the accepted `vault.open`/`LOCKED` case. Require exact accepted
   operation strings `vault.seal`, `vault.open`, `vault.store`, `vault.export`,
   `vault.restore`, `session.lock`, `native.unlock`, `native.export`, and
   `native.restore`. Require exact stable codes `ENTROPY`, `LOCKED`, `SCHEMA`, `LIMIT`,
   `WRONG_NETWORK`, `ACCOUNT_BUSY`, `NOT_FOUND`, `ALREADY_EXISTS`, `UNAVAILABLE`,
   `REPLAY`, `STATE_CORRUPT`, `TIMEOUT`, and `UNAUTH`. Reject an unknown operation/code,
   malformed account id, and a synthetic canary placed in each diagnostic field.
4. Linux port: extend the existing explicit `target/wal004-scratch/os-<pid>` test, with
   the same explicit nonrecursive cleanup, to call `LinuxStorePort`'s `read_bounded`,
   `write_all`, `set_permissions`, and `sync_file` directly on the known symlink. Every
   operation must return `UNAVAILABLE`; the regular target's bytes and `0600` mode must
   remain exact. This must fail the current source because at least the direct read
   follows the link. Do not add a race, sleep, thread, `/tmp`, unsafe, external process,
   or nondeterminism.

## Required policy regressions

Update the expected trigger inventories so `scripts/validate-rust-sbom.js` is included
in both routine social and security path filters and root `deny.toml` is included in the
security filter. Add mutations proving removal from every applicable trigger is rejected.
The SBOM workflow remains manual-only.

Reserve/export the exact seven-file Rust source inventory and require a pure policy
checker to reject a missing or extra `wallet-broker/src/*.rs` path. Require repository
policy to enumerate the actual directory and scan exactly that closed inventory.

Strengthen the Rust source policy test so the production vault requires the reviewed
`base64ct::Base64Unpadded`/`Encoding` and
`secrecy::SecretSlice`/`ExposeSecret`/`ExposeSecretMut` primitives and rejects handwritten
`encode_base64`/`decode_base64` helpers. Reject `to_string_lossy` in first-party native
path handling. Add a deterministic source mutation for each requirement.

Add a pure `checkWalletBrokerDenyPolicy` contract over root `deny.toml`. Require empty
advisory ignore, yanked deny, the exact reviewed permissive license set, no license
exceptions, wildcard deny, duplicate warning visibility with no skip/skip-tree, unknown
registry/git deny, only crates.io allowed, and no git allowlist. Mutations weakening each
class must fail. Do not introduce deprecated cargo-deny 0.20.2 advisory keys; all advisory
classes are denied by that version's default unless explicitly ignored.

Load and directly test `scripts/validate-rust-sbom.js`: accept one bounded synthetic
CycloneDX broker graph containing every exact direct component, and reject a desktop/npm
root, malformed/empty graph, and each omitted direct component. Fixtures remain inline
synthetic objects; do not create an SBOM file or new fixture.

## Restrictions and report

Use `apply_patch`. Do not run Rust, Cargo, Node, npm, tests, formatters, linters, builds,
scanners, policy, SBOM, Electron, native windows, wallets, nodes, devices, network, Git,
or GitHub. Do not install, stage, commit, push, delete, move, clean, use root/`sudo`, use
`/tmp`, or touch an unlisted path. Stop on a contradiction rather than changing source or
weakening a prior test.

After edits are complete, only read-only `wc -l` and `sha256sum` over the five authorized
test paths are allowed for the report. Report exact changed paths/counts/hashes and the
new test names. Luna will own expected-red execution and all Git operations after reviewer
acceptance.
