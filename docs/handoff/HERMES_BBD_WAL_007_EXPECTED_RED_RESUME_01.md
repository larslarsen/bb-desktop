# Hermes Handoff — BBD-WAL-007 Expected Red Resume 01

State: AUTHORIZED — NOT STARTED

You are Jr Dev — Hermes. Resume only the corrected dependency-resolution,
expected-red, evidence, and integration gate.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-02.md`

Preserved first-stop evidence:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md`
- 78 lines
- SHA-256 `d321f924f3ed817eb8112b0e503319e949cf01dae2bbb9009b75f6c230159899`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`, both test-source
reviews, the stopped handoff, the first-stop evidence, this resume handoff, and
`docs/handoff/CURRENT_TASK.md`.

## Authorized paths

Do not edit the ten accepted source paths or the preserved first-stop evidence. You may
integrate their exact hashes.

You may create or edit only:

- `wallet-broker/Cargo.lock` — Cargo resolution only;
- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — resumed evidence;
- this handoff — state line only; and
- `docs/handoff/CURRENT_TASK.md` — leading task-state/actor/handoff block only.

Every other path and repository is read-only.

## Preconditions

Record Hermes version/provider/model, `HEAD`, `origin/master`, status/index inventory,
all accepted hashes/line counts, both the first-stop evidence identity and original
`Cargo.lock` SHA-256 `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01`,
and `git diff --check`. Stop on any mismatch, staged entry, or divergent HEAD.

No source repair is authorized. Do not run `xmr_local_gate`, Monero binaries, a wallet,
node, Electron, npm, browser, scanner, full suite, or network other than the one Cargo
registry resolution.

## Corrected resolution and inspection

Run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --no-default-features --lib
```

It must succeed. The only tracked mutation is `wallet-broker/Cargo.lock`. Its complete
semantic delta must be exactly:

1. add `md-5` to the existing `bitbook-wallet-broker` dependency list; and
2. add one registry package, `md-5 0.11.0-pre.4`, depending only on existing `cfg-if`
   and exact existing `digest 0.11.0-pre.9`.

No existing package record/version/checksum/source/dependency may change; no other
package may be added or removed.

Then run exactly:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features -e features -p md-5@0.11.0-pre.4
```

Inspect the complete resolved local registry package: normalized/original manifests,
file inventory, licenses, checksum, dependency list, and build declaration/file. It must
be RustCrypto `md-5 0.11.0-pre.4`, Rust 1.72-compatible, `MIT OR Apache-2.0`, library
`md5`, `build = false` with no `build.rs`, defaults off, exact Digest pre.9 reuse, and
`zeroize` forwarding to `digest/zeroize`. Stop on any git/patch/vendor/downloader,
native/FFI/assembly activation, build script, network/runtime/TLS/URL dependency, or
different closure.

## Formatting and focused execution

Run exactly in order:

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

Formatting must pass without mutation. Continue through all seven Rust commands.
`native_surface` must fail only on the absent XMR native selection port/controller; each
`xmr_*` target must fail only because `bitbook_wallet_broker::xmr` is absent. Syntax,
format, dependency, lock, linker, runtime, unrelated, timeout, or unexpected-pass results
are stops. Node policy must exit zero with exactly 86 `ok` cases and final line:

```text
BitBook security policy tests passed (86).
```

Never run the feature-gated real test.

## Evidence and integration

On exact acceptance, create `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` recording all
identities, the first stop, original/final lock hashes and exact delta, registry package
inspection, every command/exit/result, seven primary expected-red failures, Node count,
path audit, and proof that no production or real Monero boundary ran.

Set this handoff state to `State: COMPLETE`. Update only the leading current-task block
to `PHASE B EXPECTED RED COMPLETE — REVIEW REQUIRED`, with no authorized actor and this
completed handoff.

Run `git diff --check`. Stage exactly the ten accepted source paths, resolved lockfile,
both expected-red evidence records, this resume handoff, and `CURRENT_TASK.md`. Commit
exactly `test: reserve BBD-WAL-007 Monero adapter`, push `master`, and prove
`HEAD == origin/master` with a clean worktree/index. Do not amend/rebase/merge/force-push
or authorize production.

On any stop, do not commit or push. Preserve accepted bytes, record only within the
authorized resumed evidence/state paths, and return control to the reviewer.
