# Codex Luna Handoff — BBD-WAL-006 Fixture and Expected Red Resume 04

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, roles, ticket, architecture review, test-source review,
dependency gate evidence/review 02, original fixture/red handoff, and all six tests. This
resume supersedes the original handoff's baseline, formatter, resolution/tree/metadata,
staging set, and network-capable Cargo commands. Do not rerun them. Its fixture semantics,
inspection rules, expected-red semantics, prohibitions, and reviewer boundary remain.

## Preflight and frozen inputs

Require protected `HEAD == origin/master`, clean index, `git diff --check`, and exactly
these six untracked paths:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_fixture_builder.rs` | 890 | `efb104bedeaf48f5e3a0850f84a6b504651bad2267eb3fc4a443864ae2fd3c81` |
| `wallet-broker/tests/zec_address.rs` | 277 | `2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3` |
| `wallet-broker/tests/zec_store.rs` | 334 | `492e4e6934f8cd9589de22cc338fd5e93131f3f3d3fcca5f79b44455b297e1ca` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `084aa1758cc10bdd1f9fc63f935e70328aca1f2f668ff8f67234cef7f5656434` |
| `wallet-broker/tests/zec_prepare.rs` | 412 | `a616245fdcf8d2226277e34d785bb1792d3b8b54ebf268c3d1dc5f15566da942` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

Require committed inputs:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/src/vault.rs` | 759 | `89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b` |

Revalidate that `wallet-broker/target` is real, ignored, disk-backed ext4 with safe
space; that `wal006-tmp` and `wal006-cargo` are safe existing directories; and that
`wallet-broker/target/wal006-fixture-build` is absent. Stop rather than delete or reuse
an unexpected fixture output. Use the exact absolute TMPDIR/CARGO_TARGET_DIR below for
every Cargo command. All Cargo execution is locked and offline.

## Independent fixture generation

Run exactly:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_fixture_builder
```

It must exit 0 with exactly four passed, zero failed/ignored/measured/filtered, and may
create only normal ignored Cargo/temp state plus
`wallet-broker/target/wal006-fixture-build`. Inspect without mutation. Require real
private directories/files, a closed exact manifest, eight ordered canonical entries at
heights 100–107, 15 unique compact files, correct generator versions/local activation
heights/birthday/checkpoint/scenario labels, exact declared byte lengths/SHA-256 values,
and valid height/hash/previous-hash links. Reject extra files, symlinks, paths, fields,
secrets, mnemonics, mainnet material, or undeclared values.

Record every generated path, length, mode, and SHA-256. Run the same command exactly once
more. It must again report four passed and verify the existing output byte-for-byte;
every path/length/mode/hash must remain unchanged.

Freeze bytes without transformation under exactly these new paths:

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

Use explicit source/destination paths, not a glob or recursive copy. Verify every frozen
byte hash equals its generated source and that no extra committed fixture path exists.

## Exact expected red

Run the complete custom policy runner once:

```text
node test/securityPolicy.node.js
```

Expected exit is 1 with exactly 66 `ok`, seven `not ok`, and final summary
`7 security policy test(s) failed`. Only these tests may fail:

1. `committed workflows satisfy the fail-closed checker`;
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`;
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`;
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`;
5. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`;
6. `WAL-006 Rust ZEC product source inventory remains empty during test-only Phase A`; and
7. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`.

Then run exactly one adapter test:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_address fresh_receiver_decodes_to_exactly_one_orchard_protocol_receiver
```

It must exit 101 during test-target compilation with zero executed tests and only
`E0433` diagnostics at the two `bitbook_wallet_broker::zec` imports in
`wallet-broker/tests/zec_address.rs`. Missing fixture/upstream type/API, manifest/lock,
library, link, runtime, other source path, warning-as-error, unexpected pass, canary, or
secret is unintended red; stop.

## Evidence and integration

If and only if all results are exact, create
`docs/testing/BBD-WAL-006-EXPECTED-RED-01.md` with timestamp/timezone, protected parent,
disk paths, the two four-test results, complete generated/frozen manifest and file
inventory, repeat determinism, Node totals/named failures, Rust error codes/sites/zero
tests, pre/post hashes, and confirmation that no secret/canary/live endpoint/production/
wallet/node/device/mainnet/signing/proving/extraction/broadcast/unlisted action appeared.
Update only `docs/handoff/CURRENT_TASK.md` to
`FIXTURE AND EXPECTED RED RECORDED — REVIEW REQUIRED` and link the evidence.

Run `git diff --check`. Stage with an explicit list only the six accepted ZEC tests, the
16 exact frozen fixture paths, evidence, and `CURRENT_TASK.md`. Inspect exact staged names
and content; no ignored target path may be staged. Commit
`test: record WAL-006 fixture and expected red` and push. Require final
`HEAD == origin/master`, clean index/worktree except ignored target state. Do not amend,
delete/clean, edit source/test/manifest/policy/lock/generated bytes, run another test or
suite, resolve/fetch/network, build/package Electron, create an SBOM, or touch
`../bb-go`/`../go-ipfs`. Stop; XHigh owns graph/fixture/red acceptance and Phase-C source
authorization.
