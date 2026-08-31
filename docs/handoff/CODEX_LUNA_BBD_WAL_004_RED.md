# Codex Luna Handoff — BBD-WAL-004 Lock Resolution and Expected Red

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable
integration prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

**Do not act until the owner explicitly reports that official rustup and the minimal
Rust 1.98.0 toolchain are installed user-level under `/home/lars`.** No install authority
is delegated to Luna.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-TEST-SOURCE-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, the
original Sol handoff, Corrections 1 and 2, and all nine accepted paths.

## Accepted source boundary

Verify `HEAD == origin/master`, the protected governance parent is an ancestor, and the
only uncommitted paths are the nine exact paths/hashes/line counts in the test-source
review. Use `git status --short --untracked-files=all`, `wc -l`, `sha256sum`, and
`git diff --check`. Stop without editing, resolution, execution, staging, or Git if any
path or byte differs.

## Disk and ignore boundary

Before Cargo, add exactly one line `target/` to the root `.gitignore`; do not remove or
reorder existing lines. Create exactly `/home/lars/.cache/bb-desktop-rust-tmp` with
`mkdir -p`. Every Rust/Cargo command below must set:

```text
TMPDIR=/home/lars/.cache/bb-desktop-rust-tmp
CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/target/wal004-cargo
```

Do not set Cargo/Rustup home to `/tmp`, do not clean caches, and do not use recursive
deletion. Cargo home and rustup home remain their user-level defaults under `/home/lars`.

## Toolchain and lock resolution

Run separately and record exact output/status:

```text
/home/lars/.cargo/bin/rustc +1.98.0 --version
/home/lars/.cargo/bin/cargo +1.98.0 --version
```

Both must report 1.98.0. Then, with the two environment values above, run:

```text
/home/lars/.cargo/bin/cargo +1.98.0 generate-lockfile --manifest-path wallet-broker/Cargo.toml
/home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --no-default-features
/home/lars/.cargo/bin/cargo +1.98.0 tree --manifest-path wallet-broker/Cargo.toml --locked --all-features
```

Only crates.io registry dependencies are permitted. Stop if resolution changes the
manifest, selects a non-exact direct version, introduces a git/path dependency outside
the crate, reports an incompatible feature/MSRV, or creates any path other than
`wallet-broker/Cargo.lock`, ignored target/cache state, and normal user Cargo/Rustup
state. Save concise tree inventories in evidence; do not invent an acceptance judgment.
The reviewer performs the complete graph/license/build-script/duplicate/advisory review.

## Expected red

Run the exact Node command:

```text
node test/securityPolicy.node.js
```

It must exit 1 with exactly 57 `ok` and seven `not ok`: one pre-existing constants test
is deliberately strengthened by the new `wallet-broker/**` path expectation, and the six
new WAL-004 tests require future policy/workflow exports and steps. The seven exact names
must match the test-source review output; any other prior failure is rejection.

Then run this exact Cargo command with the two environment values above:

```text
/home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --no-default-features --test vault_crypto deterministic_entropy_produces_one_stable_envelope_and_is_fully_openable
```

It must reach Rust compilation and fail only because the future
`bitbook_wallet_broker` library/API is absent. Missing toolchain, manifest parse, direct
primitive API/type error, dependency/network failure, linker/host GUI failure, test
runtime, or an unexpected pass is rejection. Native UI must remain uncompiled and no
window may launch.

## Evidence and Git boundary

If and only if every result matches expected red, create
`docs/testing/BBD-WAL-004-EXPECTED-RED.md` recording timestamp/timezone, governance HEAD,
all nine accepted hashes/line counts, tool versions, lockfile hash/line count, direct and
all-feature tree summaries, exact commands/exits, Node counts, exact Rust compiler cause,
disk-backed temp/target values, and confirmation that no canary appeared in output and no
production/native window/wallet/node/device action ran. Update only
`docs/handoff/CURRENT_TASK.md` to `EXPECTED RED RECORDED — LOCK GRAPH REVIEW REQUIRED` and
link the evidence.

Stage exactly the nine accepted test paths, root `.gitignore`,
`wallet-broker/Cargo.lock`, the evidence path, and `docs/handoff/CURRENT_TASK.md` with an
explicit `git add --` path list. Commit `test: define encrypted wallet custody`, push
`master`, then prove `HEAD == origin/master`. The final worktree must be clean except for
ignored target/cache state.

Do not edit test bytes, manifest, fixture, production, package files, policy/workflow
implementation, SBOM validators, any other documentation, or another repository. Do not
run broader tests, fmt, clippy, audit, deny, SBOM, npm, build, Electron, native UI,
packaging, production, wallet, node, network service, hardware, or device actions. The
crates.io resolution and Git push above are the only authorized network operations. Do
not use root, `sudo`, `/tmp`, cleanup, `rm`, recursive deletion, globs, or unresolved
destructive targets.

Report tool versions, resolution/tree results, exact red results/counts, changed
paths/hashes, evidence hash/line count, commit/push/final state, and every stop condition.
Stop; the reviewer must inspect the resolved lock graph and red evidence before any
production source is authorized.
