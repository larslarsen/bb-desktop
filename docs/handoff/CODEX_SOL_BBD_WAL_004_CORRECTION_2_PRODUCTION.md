# Codex Sol Handoff — BBD-WAL-004 Correction 2 Production

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, source review 02,
Correction 2 test/red/integration reviews, `CURRENT_TASK.md`, every accepted test, and
the four current production paths below.

## Sole task and paths

Correct all accepted Correction 2 red without changing tests. Edit only:

- `wallet-broker/src/session.rs`
- `wallet-broker/src/native.rs`
- `wallet-broker/src/store.rs`
- `scripts/security-policy.js`

Do not edit any other production, workflow, manifest, lockfile, dependency, deny,
validator, package, test, fixture, evidence, documentation, ticket, or handoff path.

## Exact corrections

1. In session dispatch, process `AppBackgrounded`, `ScreenLocked`, `BrokerQuit`, and
   `BrokerRestarted` as unconditional global lock-all events before account validation;
   they return success and ignore the irrelevant account argument. All account-scoped
   events retain exact account validation. On `unlock`, explicitly wipe supplied spend
   material through the session observer when `read_clock` fails, after its existing
   lock-all behavior, and return the same `TIMEOUT` without creating a session.
2. In native unlock and restore, accept only 1–1,024 bytes that are valid UTF-8. Invalid
   encoding follows the exact existing invalid-length path: explicit native-passphrase
   wipe, generic locked surface result, stable `LOCKED`, and no custody/confirmation.
   Keep pre-prompt account validation and every prior cancel/error behavior.
3. The safe no-follow regular-file opener must optionally enforce descriptor mode
   `0600`. Direct read, write, and sync require private mode before any read/truncate/
   write/sync. Descriptor-based `set_permissions` opens a no-follow regular file without
   requiring its old mode so it can repair it, then sets the requested mode on that same
   descriptor. Preserve `O_NONBLOCK`, `O_NOFOLLOW`, bounded reads, and all prior errors.
4. `checkRustWalletSourceInventory` must treat order as non-authoritative. Retain array,
   nonempty string, duplicate, exact-length, missing, and extra rejection, but accept any
   permutation of the exact seven reviewed paths. Use exact set membership; do not sort
   caller input in place or weaken repository regular-file enumeration/source scanning.
   The three repository-level Node failures must become green for this one reason.

Use `apply_patch`. Do not run Rust, Cargo, Node, tests, formatters, linters, builds,
scanners, network, Git, or any project command. Do not install, stage, commit, push,
delete, use `/tmp`, or use root. Stop on contradiction.

After editing, only `wc -l` and `sha256sum` over the four authorized paths are allowed.
Report exact counts/hashes and corrections. Luna owns all execution, formatting,
evidence, Git, and push after reviewer source acceptance.
