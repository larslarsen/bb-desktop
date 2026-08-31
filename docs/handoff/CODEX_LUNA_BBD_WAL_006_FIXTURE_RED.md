# Codex Luna Handoff — BBD-WAL-006 Fixture and Expected Red

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is the complete
Phase-B integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `docs/handoff/CURRENT_TASK.md`,
`tickets/BBD-WAL-006.md`, `docs/architecture/BBD-WAL-006-UPSTREAM-REVIEW.md`,
`docs/testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md`, the original Sol handoff, and all
eight accepted uncommitted paths.

## Preflight and sole task

Require `HEAD == origin/master` at the protected governance parent, a clean index, and
exactly the eight unstaged/untracked paths, line counts, and SHA-256 values in the
test-source review. Require `wallet-broker/Cargo.lock` to be the unchanged committed
3,273-line file with pre-resolution SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Run `git diff --check` and stop before mutation or execution on any mismatch or extra
path.

Your sole task is to integrate the accepted test drop, resolve and inventory its exact
crate graph, prove the upstream fixture target formats/compiles/runs independently,
freeze its reviewed output, and record exact Node and Rust expected red. Do not repair,
format, or author test or production source. Production and production policy remain
unauthorized.

## Disk and process boundary

Before Cargo, inspect `wallet-broker/target` with `findmnt` and `du`. It was an ignored
directory on ext4 at review time. Stop if it is absent, symlink-derived, non-disk-backed,
not ignored, or lacks safe space. Create only these ignored directories if absent:

```text
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp
/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo
```

Every Cargo command must use the exact absolute `TMPDIR` and `CARGO_TARGET_DIR` values
above. Do not use `/tmp`, change Cargo/Rustup home, clean caches, delete output, follow a
symlink, or use root/`sudo`.

## Toolchain, formatting, and resolution

Run separately and record exact output/status:

```text
/home/lars/.cargo/bin/rustc +1.98.0 --version
/home/lars/.cargo/bin/cargo +1.98.0 --version
```

Both must report 1.98.0. Then run the formatter check with the exact environment values:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
```

It must exit 0 without changing a source byte. Any formatting diff is source rejection:
stop without formatting it.

Resolve the existing committed lockfile through crates.io only, then inventory both
feature modes:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 generate-lockfile --manifest-path wallet-broker/Cargo.toml
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --no-default-features -e features
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --all-features -e features
```

Stop if Cargo changes the manifest/test bytes, selects a different direct version,
introduces a git/path/patched source, reports incompatible MSRV/features, or uses a source
other than crates.io. Verify the six direct lock entries and checksums against the table
in the upstream review. Record new lockfile line count/hash, added/removed package counts,
duplicate crypto primitives, enabled feature union, licenses, and build-script/proc-macro
inventory for reviewer judgment; do not waive or make an acceptance decision.

## Upstream fixture gate

Run only the independent fixture-builder target:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --no-default-features --test zec_fixture_builder
```

It must compile and report exactly four passing tests. It may create only
`wallet-broker/target/wal006-fixture-build` plus normal ignored Cargo/temp state. Inspect
that output without executing it: require real private directories/files, exact manifest
parse, eight ordered canonical entries at 100–107, 15 unique compact files, all declared
lengths/hashes/links, and no extra entry. Run the same four-test command once more; it must
verify the existing output byte-for-byte and leave every output hash unchanged.

Freeze the exact generated bytes, without transformation, under these newly authorized
paths only:

- `wallet-broker/tests/fixtures/zec/manifest.json`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000100.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000101.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000102.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000103.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000104.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000105.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000106.compact`
- `wallet-broker/tests/fixtures/zec/blocks/canonical-000107.compact`
- `wallet-broker/tests/fixtures/zec/blocks/reorg-replacement-000107.compact`
- `wallet-broker/tests/fixtures/zec/blocks/discontinuity-wrong-prev-000107.compact`
- `wallet-broker/tests/fixtures/zec/blocks/discontinuity-height-gap-000109.compact`
- `wallet-broker/tests/fixtures/zec/blocks/impossible-tree-state-000107.compact`
- `wallet-broker/tests/fixtures/zec/blocks/truncated-000107.compact`
- `wallet-broker/tests/fixtures/zec/blocks/malformed.compact`
- `wallet-broker/tests/fixtures/zec/blocks/corrupt-wire-type-000107.compact`

Use explicit paths, not globs, and verify every frozen byte hash equals the generated
source. Do not commit ignored target output or edit a generated byte.

## Exact expected red

Run the complete custom Node policy runner once:

```text
node test/securityPolicy.node.js
```

Expected exit is 1 with exactly 66 `ok`, seven `not ok`, and final summary
`7 security policy test(s) failed`. The only failures must be:

1. `committed workflows satisfy the fail-closed checker`;
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`;
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`;
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`;
5. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`;
6. `WAL-006 Rust ZEC product source inventory remains empty during test-only Phase A`; and
7. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`.

The first three call the still-WAL-004-only production manifest/repository policy and
therefore reject the accepted new manifest. The last four require absent WAL-006 policy
exports. Any other prior failure, different count, exception before the named assertion,
or unexpected pass is unintended red.

Then run exactly one focused adapter test:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --no-default-features --test zec_address fresh_receiver_decodes_to_exactly_one_orchard_protocol_receiver
```

It must reach compilation after the fixture gate and fail only because the future
`bitbook_wallet_broker::zec` API/module is absent. A missing fixture, upstream API/type
failure, manifest/lock issue, unrelated library failure, linker failure, runtime failure,
or unexpected pass is rejection. Do not run another ZEC adapter target.

## Evidence and Git boundary

If and only if every result is exact, create only
`docs/testing/BBD-WAL-006-EXPECTED-RED-01.md` with timestamp/timezone, protected parent,
all accepted hashes, tool versions, disk paths, lock diff/hash and graph inventories,
fixture commands/results and every generated/frozen hash, Node totals/failure reasons,
the exact Rust compiler cause, and confirmation that no canary/secret/live endpoint,
production code, wallet/node/device, mainnet, signing, proving, extraction, broadcast, or
unlisted path/action appeared. Update only `docs/handoff/CURRENT_TASK.md` to `FIXTURE AND
EXPECTED RED RECORDED — REVIEW REQUIRED`, link the evidence, and leave Phase C
unauthorized.

Run `git diff --check`. Stage exactly the eight accepted Phase-A paths,
`wallet-broker/Cargo.lock`, the 16 frozen fixture paths, the evidence path, and
`docs/handoff/CURRENT_TASK.md` with an explicit `git add --` list. Inspect exact staged
names/diff; no ignored target path may be staged. Commit once as:

```text
test: record WAL-006 fixture and expected red
```

Push `master`, require `HEAD == origin/master`, and require a clean worktree except
ignored target/temp state. Do not amend, rewrite, force push, delete, clean, edit policy
or production, run broader Rust/Node/npm/security gates, install tools, build Electron,
package, create an SBOM, access a live Zcash service, or touch `../bb-go`/`../go-ipfs`.
Stop and report exact results; XHigh owns graph/fixture/red acceptance and the separate
Phase-C source authorization.
