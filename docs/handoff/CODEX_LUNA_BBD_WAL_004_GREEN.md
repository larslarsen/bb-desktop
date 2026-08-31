# Codex Luna Handoff — BBD-WAL-004 Green and Integration

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, every
WAL-004 source/test/red/lock review, `CURRENT_TASK.md`, all tests, all 15 production
paths, and the workflows/policy/deny/SBOM validator.

## Preflight

Require `HEAD == origin/master` at the governance parent and clean index. The worktree
must contain exactly the accepted 15 production paths, no test modification, and no
extra path. Match all source-review-03 hashes plus these accepted unchanged hashes:

- vault `39c0aa7dac2930a2c276a11e65779788062337dfdc438bc2a47903c4b4cb9ce7`
- native UI `3a255c443eeabb0ea0e04e32815eba499ada940186769fbf1257bcc62579d9dc`
- hygiene `ea600c1a4d4f178570237c63892fd5de450ce6166a48117f5f86e3ce7da06dfe`
- lib `09e1ba98383fedda2b0db5e36ff716f0a8ce30ef37072901dc8b1e31be06dbdc`
- manifest `d1d338ff0cb63eb6c7f992b2573b6ebc4ee5d7d459301961eb0b4aaa8d2ebd7c`
- lock `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`
- deny `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8`
- Rust SBOM validator `6b90e1a5dcc423a6a891cbf3d5964536e00772fb6549e64d82bce3fcec84b4a0`
- package `9b8b03edc602554e98266d7a79168eacbffe4243a686b52d92bc1dd8a52e3893`
- social workflow `c2d7e2cca231d6b55b7403e756b39e2855421c5407d10fb2146d7493650f96a3`
- security workflow `64421f333299861103fdd8d3eee0df35414a40e45b5eef4f05d83cd1ebe3159a`
- manual SBOM workflow `8407f00fc0ed9ad7bd88c726d64e5cd02a61922653991f9cf4b7cf8bea528824`

Require Rust/Cargo 1.98.0 via `/home/lars/.cargo/bin/rustup run 1.98.0`. Use only the
repository's ignored `wallet-broker/target`, locked/offline Cargo where applicable, and
no `/tmp` cache/target. No native window, wallet, node, device, network client, package
binary, or real secret may run.

## Exact execution order

Run commands separately and preserve exit status/output:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 rustc --version
/home/lars/.cargo/bin/rustup run 1.98.0 cargo --version
/home/lars/.cargo/bin/rustup run 1.98.0 cargo fmt --manifest-path wallet-broker/Cargo.toml --check
npm run build
npm test
node scripts/security-policy.js
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features
/home/lars/.cargo/bin/rustup run 1.98.0 cargo clippy --manifest-path wallet-broker/Cargo.toml --locked --offline --all-targets --all-features -- -D warnings
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --locked --offline --features native-ui --test native_surface
/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock
```

If and only if the initial fmt check fails solely on formatting, run exact `cargo fmt`
for the manifest, inspect the entire diff, require only whitespace/layout changes in
Rust source/tests, rerun fmt check, and include every formatter-touched Rust path in the
integration manifest. Do not manually repair semantic code. Any compile, test, clippy,
native-feature, policy, audit, canary, unrelated, or path failure is a blocker: stop and
report without staging/commit. Do not install missing tools. `cargo-deny` and
`cargo-cyclonedx` are intentionally deferred to pinned GitHub workflows after commit.

Success requires all 65 Node policy cases green, all 78 Rust integration tests green,
the independent vector exact, every scratch cleanup complete, no canary in command
output, fmt/clippy/native compile green, and RustSec clean. Record actual counts rather
than assuming these projections.

## Evidence and integration

If exact green, create only `docs/testing/BBD-WAL-004-GREEN.md` with versions, every
command/status/count, formatting action/diff paths if any, no-canary/scratch result,
audit result, missing local deny/cyclonedx note, and final hashes. Update only
`docs/handoff/CURRENT_TASK.md` to `PRODUCTION GREEN INTEGRATED — CI SECURITY/SBOM GATES
PENDING` and link evidence.

Run `git diff --check`. Stage exactly all 15 production paths, any formatter-only
accepted Rust paths, green evidence, and `CURRENT_TASK.md`. Inspect staged names and full
diff; verify tests were not weakened. Commit once as `feat: add encrypted wallet custody
core` and push master. Require `HEAD == origin/master` and clean worktree except ignored
target/cache. Report commit, complete staged manifest, evidence count/hash, test/tool
results, final source hashes, and push status. Do not trigger GitHub workflows; reviewer
owns their post-push gates.
