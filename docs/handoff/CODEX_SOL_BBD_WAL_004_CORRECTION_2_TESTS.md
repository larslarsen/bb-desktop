# Codex Sol Handoff — BBD-WAL-004 Correction 2 Test Source

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-PRODUCTION-SOURCE-REVIEW-02.md`, all earlier WAL-004 reviews
and red evidence, `docs/handoff/CURRENT_TASK.md`, the four authorized test paths, and
the complete current uncommitted production drop.

## Frozen production

All 15 production paths are intentionally uncommitted and must remain byte-identical.
The nine corrected-path hashes are:

| Path | SHA-256 |
| --- | --- |
| `wallet-broker/src/vault.rs` | `39c0aa7dac2930a2c276a11e65779788062337dfdc438bc2a47903c4b4cb9ce7` |
| `wallet-broker/src/store.rs` | `32a08c17f0cb139aa9b1905e8993b075d357d363cd8a4cf604fec6d2d9ba85aa` |
| `wallet-broker/src/session.rs` | `40a093c4327e23e41ed0dc7a07315375543d4c3672b4bb36c39ce2e909c1ee81` |
| `wallet-broker/src/native.rs` | `b3ce8a2dfa2b7a5646823b6d9f35de56e43a85dae4702b733376881fa40d6610` |
| `wallet-broker/src/native_ui.rs` | `3a255c443eeabb0ea0e04e32815eba499ada940186769fbf1257bcc62579d9dc` |
| `wallet-broker/src/hygiene.rs` | `ea600c1a4d4f178570237c63892fd5de450ce6166a48117f5f86e3ce7da06dfe` |
| `scripts/security-policy.js` | `486196e8b1791b522efee2ec36e1563e108ff83198b59a696273cbfbdffa0dda` |
| `.github/workflows/social.yml` | `c2d7e2cca231d6b55b7403e756b39e2855421c5407d10fb2146d7493650f96a3` |
| `.github/workflows/security.yml` | `64421f333299861103fdd8d3eee0df35414a40e45b5eef4f05d83cd1ebe3159a` |

The other six hashes remain those in Correction 1 source review. Do not edit production,
policy implementation, workflow, manifest, lockfile, dependency, deny, validator,
package, fixture, evidence, documentation, ticket, handoff, or any unlisted path.

## Sole task and authorized paths

Author focused regression **test source only** for every source review 02 finding. You
may edit only:

- `wallet-broker/tests/vault_session.rs`
- `wallet-broker/tests/native_surface.rs`
- `wallet-broker/tests/vault_store.rs`
- `test/securityPolicy.node.js`

Preserve every accepted test and assertion.

## Required regressions

1. Session global events: for each `AppBackgrounded`, `ScreenLocked`, `BrokerQuit`, and
   `BrokerRestarted`, unlock both accepted accounts, call `handle` with a malformed
   synthetic account value, require success, require both accounts locked, and require
   two real `session-spend-material` all-zero wipe observations. Account-scoped events
   continue to reject malformed account identifiers.
2. Session clock failure during unlock: configure the injected clock to fail before an
   unlock carrying bounded synthetic material. Require stable `TIMEOUT`, no created
   session, and an exact-length real all-zero `session-spend-material` observation for
   the supplied material. Do not infer wiping merely from `Drop`.
3. Native passphrase encoding: separately test a short invalid UTF-8 byte sequence for
   both unlock and restore. Require `LOCKED`, generic surface error, exact-length
   native-passphrase all-zero wipe, no custody call/material observation, no restore
   metadata/commit, and no partial success. Keep the existing length cases unchanged.
4. Linux descriptor mode: using a new explicit per-process path under the existing
   `target/wal004-scratch` convention, create one regular `0644` file with known bytes.
   Direct `LinuxStorePort` read, write, and sync must each return `UNAVAILABLE`; bytes and
   mode remain unchanged. Direct descriptor-based `set_permissions(path, 0600)` must
   then succeed so the operation can repair a regular file. Use explicit nonrecursive
   cleanup. No symlink race, sleep, thread, process, `/tmp`, unsafe, or nondeterminism.
5. Source inventory portability: extend the pure policy test to pass a deterministic
   fresh-checkout ordering (the exact seven paths lexicographically sorted) and require
   acceptance. Also require rejection of a duplicate and a non-string/malformed item in
   addition to existing missing/extra cases. The repository still enumerates actual
   regular `.rs` files and scans the exact closed set; order carries no authority.

Every new case must fail the frozen production for its reviewed reason and be green only
after a production correction. Do not add a broad end-to-end test, real secret, wallet,
coin, node, device, GUI, network, or timing dependency.

## Restrictions and report

Use `apply_patch`. Do not run Rust, Cargo, Node, npm, tests, formatters, linters, builds,
scanners, policy, SBOM, Electron, native windows, wallets, nodes, devices, network, Git,
or GitHub. Do not install, stage, commit, push, delete, move, clean, use root/`sudo`, use
`/tmp`, or touch an unlisted path.

After edits are complete, only read-only `wc -l` and `sha256sum` over the four authorized
test paths are allowed. Report exact paths/counts/hashes and new test names. Luna owns
expected-red execution and Git after reviewer source acceptance.
