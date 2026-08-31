# BBD-WAL-004 Expected-Red Evidence

Timestamp: 2026-08-30T18:44:16-0700 (PDT)
Governance `HEAD == origin/master`: `035b8dea8d2c749b1615b5e9d35f11fd1f9a37cf`

Accepted source paths were verified at their exact counts and hashes:

- `wallet-broker/Cargo.toml` — 48 lines — `278c26e3354fca0af5c3b456f8ecdf07defac032b8eb648d8992d3b369feedd7`
- `wallet-broker/tests/vault_crypto.rs` — 325 lines — `98306e4fe254ce07f78a75f832701c64a9333111b593fb30a4ba95bf01b1bac1`
- `wallet-broker/tests/vault_format.rs` — 320 lines — `7bd7754ea8c17d0d6f0981e82e4627c020839c29fcddbe7026be65ecf8d34877`
- `wallet-broker/tests/vault_store.rs` — 464 lines — `5e002b440e220ed5dae0170e2e8077c4d8027fb90ff51b9dd2fb18640c167454`
- `wallet-broker/tests/vault_session.rs` — 179 lines — `9a24bee6a7f2e761fcdfa32f461fccecd6810272f66a5fa5deff4b3eb660c55b`
- `wallet-broker/tests/native_surface.rs` — 293 lines — `a2fe135d054256ec4eae2350e2f72c8672e533aa2afaae5056c4efa6d52773c9`
- `wallet-broker/tests/secret_hygiene.rs` — 181 lines — `c3f73e3e087dab13f7300483859b8671b748dbf750c6672157977e84aad8d590`
- `wallet-broker/tests/fixtures/vault-v1.json` — 1 line — `022f7dc640ef36071c7b4de6347fec4b0b84560a8cbab781403a9eaceaea37e4`
- `test/securityPolicy.node.js` — 1,778 lines — `5f8d79fef4176dac6f8f4e912c4056b70c65062e9d14cf29091d413091ca5c12`

Rust tool versions: `rustc 1.98.0 (88d9e12ae 2026-08-18)` and `cargo 1.98.0
(797e8a9bc 2026-08-05)`. Root `.gitignore` is 12 lines, SHA-256
`9e528c7294e2b5d37b9016991a20fcb111afd77dd318ba9073500e84d83e8ec5`, with only the
authorized appended `target/` line. Temp: `/home/lars/.cache/bb-desktop-rust-tmp`;
target: `/home/lars/OpenBazaar/bb-desktop/target/wal004-cargo`.

## Lock and tree resolution

`cargo +1.98.0 generate-lockfile --manifest-path wallet-broker/Cargo.toml` exited 0,
locking 326 packages. `wallet-broker/Cargo.lock`: 3,273 lines, SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`.
Both locked tree commands exited 0. Direct graph includes argon2 0.6.0,
chacha20poly1305 0.11.0, getrandom 0.4.3, hkdf 0.13.0, secrecy 0.10.3,
serde 1.0.229, serde_json 1.0.151, sha2 0.11.0, and zeroize 1.9.0. All-features
adds the optional eframe/rfd native UI graph with the reviewed feature set.

## Expected red

`node test/securityPolicy.node.js` exited 1 after all tests: 57 `ok`, exactly seven
`not ok` (the strengthened constants test plus the six WAL-004 policy tests), with no
canary output. The failures were absent future Rust toolchain/manifest policy, source
policy, reserved commands, CI workflow, RustSec/cargo-deny, and SBOM wiring.

The locked command
`cargo +1.98.0 test --manifest-path wallet-broker/Cargo.toml --locked --no-default-features --test vault_crypto deterministic_entropy_produces_one_stable_envelope_and_is_fully_openable`
reached compilation and exited 101 only because `bitbook_wallet_broker` is absent
(`E0433` at `tests/vault_crypto.rs:5`). No production/native window/wallet/node/device
action ran, and no canary appeared.
