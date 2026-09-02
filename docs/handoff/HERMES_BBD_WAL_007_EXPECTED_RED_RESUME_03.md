# Hermes Handoff — BBD-WAL-007 Expected Red Resume 03

State: AUTHORIZED — NOT STARTED

You are Jr Dev — Hermes. Run the final formatting and focused expected-red gate,
complete the preserved evidence, and integrate the accepted Phase-A tests.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Accepted source identity:
`../testing/BBD-WAL-007-TEST-SOURCE-REVIEW-04.md`

Preserved evidence identities:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-01.md` — 78 lines — SHA-256
  `d321f924f3ed817eb8112b0e503319e949cf01dae2bbb9009b75f6c230159899`;
- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — 68 lines — SHA-256
  `d38dd8478d88efb2b080bb8930b451fef6d017ac7e4dd15c487266c556f5ba05`.

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`, Review 04, the
two evidence records, the three prior Hermes expected-red handoffs, this handoff, and
`docs/handoff/CURRENT_TASK.md`.

## Authorized paths

Do not edit the ten Review 04 source paths, resolved lockfile, or first evidence record.
You may integrate their exact hashes.

You may edit only:

- `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` — preserve both formatting stops and
  append the completed execution evidence;
- this handoff — state line only; and
- `docs/handoff/CURRENT_TASK.md` — leading task-state/actor/handoff block only.

Every other path and repository is read-only.

## Preconditions and dependency identity

Record Hermes version/provider/model, `HEAD`, `origin/master`, complete status/index,
all Review 04 hashes/line counts, both evidence identities, resolved `Cargo.lock`
SHA-256 `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420`,
and `git diff --check`. Stop on any mismatch, staged entry, or divergent HEAD.

The accepted lock delta is exactly the root `md-5` edge, the activated `zeroize` edge
in each existing `block-buffer 0.11.0-rc.3` and `digest 0.11.0-pre.9` record, and one
`md-5 0.11.0-pre.4` registry record with checksum
`117b97b6b9ae1ec9a396b357698efa3ecff4fc1f40e0ec59ae7c1270b460ac1d`
depending only on `cfg-if` and `digest 0.11.0-pre.9`. No existing
version/checksum/source or other package/edge changes are allowed. Do not resolve or
modify the lockfile.

Confirm the tree without mutation:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features -e features -p md-5@0.11.0-pre.4
```

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
`native_surface` must fail only on the absent XMR native-selection port/controller; each
`xmr_*` target must fail only because `bitbook_wallet_broker::xmr` is absent. Any other
failure or unexpected pass is a stop. Node must exit zero with exactly 86 `ok` cases and
final line `BitBook security policy tests passed (86).`

Never run `xmr_local_gate`, Monero binaries, a wallet/node, Electron, npm, browser,
scanner, a full suite, or any network operation.

## Evidence and integration

On exact acceptance, update `docs/testing/BBD-WAL-007-EXPECTED-RED-02.md` without
erasing either stop. Record identities, exact lock delta/tree, every command/exit/result,
the seven primary expected-red failures, Node count, path audit, and proof no production
or real Monero boundary ran.

Set this handoff state to `State: COMPLETE`. Update only the leading current-task block
to `PHASE B EXPECTED RED COMPLETE — REVIEW REQUIRED`, with no authorized actor and this
completed handoff. Run `git diff --check`.

Stage exactly the ten Review 04 source paths, resolved lockfile, both evidence records,
this handoff, and `CURRENT_TASK.md`. Commit exactly
`test: reserve BBD-WAL-007 Monero adapter`, push `master`, and prove
`HEAD == origin/master` with a clean worktree/index. Do not amend/rebase/merge/force-push
or authorize production.

On any stop, do not commit or push. Preserve accepted bytes, record only within the
authorized evidence/state paths, and return control to the reviewer.
