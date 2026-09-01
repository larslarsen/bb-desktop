# Hermes Handoff — BBD-WAL-006 Prepare Serde Lock Capture 01

You are **Jr Dev — Hermes** using the free configured Nous route. Own only this offline lock
resolution capture and its evidence commit.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Serde Feature Production Source Review 01,
the accepted expected-red evidence/review, and `CURRENT_TASK.md`.

## Preconditions

Record Hermes version/provider/model, `HEAD`, `origin/master`, exact six-path uncommitted inventory,
all accepted source/manifest/policy identities, old lock identity, and `git diff --check`. Verify
the existing WAL-006 Cargo temp/target directories are ext4. Stop on a mismatch or extra path.

## Sole command

Run exactly from repository root:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" rustup run 1.98.0 cargo metadata --manifest-path wallet-broker/Cargo.toml --offline --format-version 1
```

Expected: exit 0, no network, and only `wallet-broker/Cargo.lock` newly changes. The lock must retain
every package version/source/checksum and add exactly two dependency-edge lines:

- `"serde"` in the `zcash_client_sqlite 0.22.0` dependency array;
- `"serde_core"` in the `uuid 1.26.0` dependency array.

Expected new lock length is 5,381 lines. Record the new SHA-256. Any other lock/path/content change,
warning/error, package/version/source/checksum change, network attempt, or different line count is a
stop; do not run a formatter, Clippy, test, Node, or second Cargo command.

## Evidence integration

On the exact capture, use `apply_patch` to create only
`docs/testing/BBD-WAL-006-PREPARE-SERDE-LOCK-CAPTURE-01.md` and update only the leading
state/actor/active-handoff block of `docs/handoff/CURRENT_TASK.md`. Record command/result, old/new
lock identities, exact diff, source identities, ext4 paths, and negative capability.

Stage exactly the evidence file and `CURRENT_TASK.md`; leave the lock and all six production/policy
paths unstaged. Commit exactly `docs: capture WAL-006 prepare serde lock delta`, push `master`, and
prove `HEAD == origin/master` plus exactly seven unstaged/untracked paths.

Do not modify source/test/manifest/policy/lock after the sole command, or edit another path. Do not
amend, rebase, merge, force-push, fetch, run network, Electron, wallet/node/device, fixture,
cleanup, or deletion. Stop and report any mismatch.
