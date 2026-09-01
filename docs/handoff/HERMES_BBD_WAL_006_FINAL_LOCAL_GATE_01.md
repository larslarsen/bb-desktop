# Hermes Handoff — BBD-WAL-006 Final Local Gate 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Own the complete final local
acceptance gate, evidence, and documentation integration. Do not alter application, policy, test,
manifest, lock, workflow, package, or scanner configuration source.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, this handoff, the prepare,
policy, and falsification gate evidence, all maintained workflow/policy files, `deny.toml`,
`.gitleaksignore`, both lockfiles, and `CURRENT_TASK.md`.

## Preconditions

Record Hermes version/provider/model. Require `HEAD == origin/master`, clean worktree/index,
clean `git diff --check`, ext4 Cargo directories, Rust/Cargo 1.98.0, and these identities:

| Path | SHA-256 |
| --- | --- |
| `package-lock.json` | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` |
| `wallet-broker/Cargo.lock` | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |
| `deny.toml` | `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8` |
| `.gitleaksignore` | `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b` |
| `scripts/security-policy.js` | `d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5` |
| `wallet-broker/src/zec/prepare.rs` | `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e` |
| `wallet-broker/src/zec/scan.rs` | `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad` |

Require `/home/lars/.cargo/bin/cargo-audit` and
`target/security-tools/gitleaks-v8.30.1/gitleaks` executable. The reviewer confirmed
`/home/lars/.cargo/bin/cargo-deny` is absent. Install only exact cargo-deny 0.20.2 using command 0;
leave the installed tool in place and do not clean its build directory.

## Exact commands

Run once each in this exact order from the repository root. Stop on first mismatch.

0. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-tools-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo install cargo-deny --version 0.20.2 --locked`
   — exit 0; `/home/lars/.cargo/bin/cargo-deny --version` reports exactly 0.20.2.
1. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check`
   — exit 0, no mutation.
2. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features`
   — exit 0, exactly 127 integration tests passed and zero failed; no skipped/ignored test.
3. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --all-targets --all-features -- -D warnings`
   — exit 0, zero warning/diagnostic.
4. `TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface`
   — exit 0.
5. `npm run build`
   — exit 0 with every syntax/shell check green.
6. `npm test`
   — exit 0: social green; Electron security 19, repository policy 75, wallet contract 48,
   broker protocol 11, supervisor 11, preload 6; zero failures.
7. `node scripts/security-policy.js`
   — exit 0, exact line `BitBook desktop security policy checks passed.`
8. `npm audit --audit-level=low`
   — exit 0, zero vulnerabilities, no lock mutation.
9. `/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock`
   — exit 0, zero vulnerabilities/advisory denial.
10. `/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources`
    — exit 0, no advisory/license/source/ban denial.
11. `target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .`
    — exit 0, no unsuppressed finding.
12. `target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .`
    — exit 0, no unsuppressed finding.

After command 12 require both lockfile hashes unchanged, exact source hashes unchanged, clean
`git diff --check`, and a clean worktree/index.

## Evidence and integration

Only on complete exact success, use `apply_patch` to:

1. create `docs/testing/BBD-WAL-006-FINAL-LOCAL-GATE-01.md` recording every command/result,
   counts, tool versions, audits/scans, dependency/security conclusions, immutable identities,
   the accepted prepare/policy/falsification evidence, and final Git state;
2. correct only the stale first-command final line in
   `docs/testing/BBD-WAL-006-FINAL-POLICY-GATE-01.md` from
   `BitBook desktop security policy tests passed (75).` to the actual exact
   `BitBook security policy tests passed (75).`;
3. update only the leading current-task block to `FINAL LOCAL GATE COMPLETE — REVIEWER ACCEPTANCE
   PENDING`, with no authorized actor and this handoff/evidence completed.

Stage exactly those three documentation paths. Commit exactly
`test: record WAL-006 final local gate`, push `master`, and prove clean worktree/index with
`HEAD == origin/master`.

Do not run another command after final proof; do not modify accepted source/test/policy/config;
do not run a wallet/node/device/mainnet/sign/prove/extract/broadcast action; do not amend/rebase/
merge/force-push, clean, or delete. On any mismatch stop without evidence/integration and report it.
