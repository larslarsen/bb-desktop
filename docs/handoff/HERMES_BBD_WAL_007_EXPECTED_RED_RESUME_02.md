# Hermes Handoff — BBD-WAL-007 Expected Red Resume 02

State: AUTHORIZED — NOT STARTED

You are Jr Dev — Hermes. Resume only the corrected formatting and focused expected-red
gate, evidence completion, and integration.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-03.md`

Preserved stop evidence:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md` — 78 lines — SHA-256
  `d321f924f3ed817eb8112b0e503319e949cf01dae2bbb9009b75f6c230159899`;
- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — current stop record — SHA-256
  `521d7087b7d632f7ff6771afc1852aa100c90584acd0ea9731c395443489375f`.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`, all three
test-source reviews, both prior Hermes handoffs, both stop records, this handoff, and
`docs/handoff/CURRENT_TASK.md`.

## Authorized paths

Do not edit the ten accepted source paths, resolved lockfile, or first-stop evidence.
You may integrate their exact hashes.

You may create or edit only:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — preserve both stops and complete the
  resumed execution evidence;
- this handoff — state line only; and
- `docs/handoff/CURRENT_TASK.md` — leading task-state/actor/handoff block only.

Every other path and repository is read-only.

## Preconditions and resolved dependency

Record Hermes version/provider/model, `HEAD`, `origin/master`, full status/index
inventory, all Review 03 hashes/line counts, both stop-evidence identities, resolved
`Cargo.lock` SHA-256
`29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`,
and `git diff --check`. Stop on any mismatch, staged entry, or divergent HEAD.

The prior successful resolution and package inspection are accepted. The complete
lockfile delta relative to protected HEAD is exactly:

1. add `md-5` to the existing `bitbook-wallet-broker` dependency list;
2. add `zeroize` to the existing `block-buffer 0.11.0-rc.3` dependency list;
3. add `zeroize` to the existing `digest 0.11.0-pre.9` dependency list; and
4. add one registry package, `md-5 0.11.0-pre.4`, checksum
   `117b97b6b9ae1ec9a396b357698efa3ecff4fc1f40e0ec59ae7c1270b460ac1d`,
   depending only on `cfg-if` and `digest 0.11.0-pre.9`.

No existing version/checksum/source changes and no other package/edge changes are
allowed. Correct the current evidence's narrower lock-delta sentence when completing
it. Do not resolve or modify the lockfile again.

Confirm the accepted offline feature tree without mutation:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features -e features -p md-5@0.11.0-pre.4
```

Stop on any lock mutation or disagreement with the already recorded RustCrypto package,
license, no-build-script, and dependency inspection.

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
format, dependency, lock, linker, runtime, unrelated, timeout, or unexpected-pass
results are stops. Node policy must exit zero with exactly 86 `ok` cases and final line:

```text
BitBook security policy tests passed (86).
```

Never run `xmr_local_gate`, Monero binaries, a wallet, node, Electron, npm, browser,
scanner, full suite, or any network operation.

## Evidence and integration

On exact acceptance, update `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` without
erasing either prior stop. Record all identities, the corrected exact lock delta and
offline tree confirmation, every command/exit/result, seven primary expected-red
failures, Node count, path audit, and proof that no production or real Monero boundary
ran.

Set this handoff state to `State: COMPLETE`. Update only the leading current-task block
to `PHASE B EXPECTED RED COMPLETE — REVIEW REQUIRED`, with no authorized actor and this
completed handoff.

Run `git diff --check`. Stage exactly the ten Review 03 source paths, resolved lockfile,
both expected-red evidence records, this resume handoff, and `CURRENT_TASK.md`. Commit
exactly `test: reserve BBD-WAL-007 Monero adapter`, push `master`, and prove
`HEAD == origin/master` with a clean worktree/index. Do not amend/rebase/merge/force-push
or authorize production.

On any stop, do not commit or push. Preserve accepted bytes, record only within the
authorized evidence/state paths, and return control to the reviewer.
