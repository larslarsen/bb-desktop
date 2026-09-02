# Hermes Handoff — BBD-WAL-007 Expected Red 01

State: AUTHORIZED — NOT STARTED

You are Jr Dev — Hermes. Own only this dependency-resolution, expected-red, evidence,
and integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-01.md`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`, the accepted
source review, this handoff, and `docs/handoff/CURRENT_TASK.md`.

## Authorized paths

Do not edit the ten accepted source paths. You may integrate their exact accepted bytes.

You may create or edit only:

- `wallet-broker/Cargo.lock` — Cargo resolution only;
- `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md` — evidence;
- `docs/handoff/HERMES_BBD_WAL_007_EXPECTED_RED_01.md` — state line only; and
- `docs/handoff/CURRENT_TASK.md` — leading task-state/actor/handoff block only.

Every production path and every other test, dependency, manifest, fixture, policy
implementation, package, workflow, ticket, architecture record, evidence record, and
repository is read-only.

## Preconditions

Record the Hermes Agent version and actual provider/model. Record protected `HEAD`,
`origin/master`, index/worktree inventory, accepted hashes/line counts, the original
`Cargo.lock` SHA-256, and `git diff --check`. Stop on any source hash/path mismatch,
staged entry, unexpected lock delta, or `HEAD != origin/master`.

Do not repair source. Do not run the feature-gated `xmr_local_gate`, `monerod`,
`monero-wallet-rpc`, a wallet, Electron, npm, a browser, a scanner, a full test suite, or
any network other than Cargo's one dependency resolution.

## Lock resolution and dependency inspection

From the repository root, run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --no-default-features --lib
```

This command must succeed and may change only `wallet-broker/Cargo.lock`. It must add the
direct registry crate `md-5 0.11.0` and only its unavoidable registry dependency closure;
it may add `md-5` to the existing `bitbook-wallet-broker` dependency list. No existing
package version, checksum, source, or dependency list may change except for that one
root-package dependency addition; no package may be removed. Stop on a git dependency,
wildcard, patch, vendored source, downloader, native code, network/runtime/TLS/URL
dependency, or any build script in the new closure.

Then run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features -e features -p md-5@0.11.0
```

Inspect the resolved local registry source for every newly locked package in the direct
crate's closure: normalized/original manifests, complete file inventory, licenses, and
every build-script declaration/file. Record their registry checksums and exact closure.
The direct package must be RustCrypto `md-5 0.11.0`, Rust 1.85-compatible,
`MIT OR Apache-2.0`, `build = false` with no `build.rs`, defaults off, and `zeroize`
enabled through `digest/zeroize`. Its library name is `md5`; its use remains limited to
Digest interoperability. Any discrepancy is a stop.

After resolution, every remaining Cargo build/test command is `--locked --offline`;
`cargo fmt --check` performs no resolution.

## Formatting and focused execution

Run exactly in this order:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test native_surface
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_distribution
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_process
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_rpc
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_account
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_receiver
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test xmr_hygiene
node test/securityPolicy.node.js
```

Formatting must pass without modifying files. Continue through all seven expected-red
Rust commands. `native_surface` must exit nonzero only because the frozen XMR native
selection port/controller is absent. Each `xmr_*` command must exit nonzero only because
`bitbook_wallet_broker::xmr` production is absent. A syntax/format error, dependency or
lock error, unrelated regression, linker/runtime failure, hang, unexpected pass, or any
different primary failure is a stop.

The Node policy command must exit zero with exactly 86 `ok` cases and final line:

```text
BitBook security policy tests passed (86).
```

Do not run `xmr_local_gate` under any feature or environment.

## Evidence and integration

On the exact accepted results, create
`docs/testing/BBD-WAL-007-EXPECTED-RED-01.md`. Record the environment, identities,
accepted hashes, original/final lock hashes, exact lock additions, crate provenance,
licenses/build-script result, every command and exit code, concise primary compiler
failure for each red target, Node count, path audit, and confirmation that no real binary,
wallet, node, network gate, or production source ran.

Change this handoff's state line to `State: COMPLETE`. Update only the leading block of
`docs/handoff/CURRENT_TASK.md` to `PHASE B EXPECTED RED COMPLETE — REVIEW REQUIRED`, with
no authorized source or integration actor and this handoff as the completed handoff.

Run `git diff --check`. Stage exactly the ten accepted source paths plus:

- `wallet-broker/Cargo.lock`;
- `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md`;
- `docs/handoff/HERMES_BBD_WAL_007_EXPECTED_RED_01.md`; and
- `docs/handoff/CURRENT_TASK.md`.

Commit exactly `test: reserve BBD-WAL-007 Monero adapter`, push `master` to `origin`, and
prove `HEAD == origin/master` with a clean worktree/index. Do not amend, rebase, merge, or
force-push. Report the commit, push result, dependency delta, exact command outcomes, and
final status. Do not authorize production.

On any stop condition, do not commit or push. Record the stop only if it can be done
within the authorized evidence/state paths, preserve all source bytes, and return control
to the reviewer.
