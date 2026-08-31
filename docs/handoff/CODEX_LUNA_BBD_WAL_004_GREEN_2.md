# Codex Luna Handoff — BBD-WAL-004 Green Rerun and Integration

You are **Jr Dev — Codex Luna**. This durable file is the complete prompt; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`, all WAL-004 reviews and
red/green evidence, prior green handoff, `CURRENT_TASK.md`, all source, and all tests.

## Preflight and frozen formatted state

Require `HEAD == origin/master` at the governance parent and clean index. The worktree
must contain exactly the accepted 15 production paths plus formatter-only modifications
to the six Rust test paths, with no extra path. Match these current hashes:

```text
d1d338ff0cb63eb6c7f992b2573b6ebc4ee5d7d459301961eb0b4aaa8d2ebd7c  wallet-broker/Cargo.toml
09e1ba98383fedda2b0db5e36ff716f0a8ce30ef37072901dc8b1e31be06dbdc  wallet-broker/src/lib.rs
03ebaf98327094842c60a40de6cdb16670e559a4e00f599d13cad97db8097525  wallet-broker/src/vault.rs
59948a11da60c398035e88ef1b17530d241911e982397223a549d00fc3d82499  wallet-broker/src/store.rs
42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227  wallet-broker/src/session.rs
50a078f05d8d66127fac0aae99343070758b0da549d5468ed2e0bd71ba0483e9  wallet-broker/src/native.rs
6887b3468a10a946c9f8b8f05aa260538065e883c59fc3b39900ce860d75fad0  wallet-broker/src/native_ui.rs
7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf  wallet-broker/src/hygiene.rs
57b4f432bf4a96d9023f74e1f25c43ebd091737c341ebf837fd0ea3994077655  wallet-broker/tests/vault_crypto.rs
5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193  wallet-broker/tests/vault_format.rs
582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5  wallet-broker/tests/vault_store.rs
67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b  wallet-broker/tests/vault_session.rs
0308eaf8db147789287474a69b51de2ef50a6e93f286882cb1aa62d7de0f2586  wallet-broker/tests/native_surface.rs
3f809e06e96add88a91c232b7824531ddaaf320182e79d9e51cf3c6b61e42323  wallet-broker/tests/secret_hygiene.rs
3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8  deny.toml
02df7a6f656826a580d972a722e79038ad6eee70bce7e55c27a4c0245db8a853  scripts/security-policy.js
6b90e1a5dcc423a6a891cbf3d5964536e00772fb6549e64d82bce3fcec84b4a0  scripts/validate-rust-sbom.js
9b8b03edc602554e98266d7a79168eacbffe4243a686b52d92bc1dd8a52e3893  package.json
c2d7e2cca231d6b55b7403e756b39e2855421c5407d10fb2146d7493650f96a3  .github/workflows/social.yml
64421f333299861103fdd8d3eee0df35414a40e45b5eef4f05d83cd1ebe3159a  .github/workflows/security.yml
8407f00fc0ed9ad7bd88c726d64e5cd02a61922653991f9cf4b7cf8bea528824  .github/workflows/sbom.yml
```

Lockfile remains SHA `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Inspect the complete six-test diff and require every change to be canonical rustfmt
layout (including rustfmt's optional trailing commas and block braces), with no changed
identifier, literal, operator, call, assertion, branch, test name, or case membership.
Green Run 01 already executed these formatted tests with 77/78 passing and the sole
failure independently attributed to the now-corrected production wire error. Stop on
any non-formatter test change.

## Exact full rerun

Use `/home/lars/.cargo/bin/rustup run 1.98.0`, locked/offline Cargo where shown, the
ignored repository target, and no `/tmp`. Run separately in order:

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

Do not run formatter mutation again: current state must already pass check. Success
requires all npm suites, all 65 Node policy cases, all 78 Rust integration tests and
independent vector, clippy/all-features, native feature compile, and RustSec audit green;
no canary, scratch residue, unrelated change, native window, wallet/node/device, network
client, or package binary. Stop without staging on any failure. Do not install missing
cargo-deny/cyclonedx or trigger GitHub.

## Evidence and integration

On exact green create only `docs/testing/BBD-WAL-004-GREEN.md` with every command/status/
count, no-canary/scratch result, formatter-only test manifest, RustSec result, deferred
local-tool note, and final hashes. Update only `CURRENT_TASK.md` to `PRODUCTION GREEN
INTEGRATED — CI SECURITY/SBOM GATES PENDING` and link evidence.

Run `git diff --check`. Stage exactly the 15 production paths, six formatter-only Rust
tests, evidence, and `CURRENT_TASK.md`. Inspect the full staged diff and names; tests must
be semantic-equivalent. Commit once `feat: add encrypted wallet custody core` and push
master. Require final `HEAD == origin/master`, no non-ignored changes, and report commit,
manifest, evidence line/hash, all counts/tools, final hashes, and push. Reviewer owns
post-push GitHub gates.
