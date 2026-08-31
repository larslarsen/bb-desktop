# BBD-WAL-004 Green Integration Evidence

Governance baseline: `HEAD == origin/master == be7e25c9898432325334bc9d62c301cc6113d7be`
Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`; `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

Preflight verified the exact 15 production paths and six accepted Rust test paths,
including the Correction 6 compatibility test and five rustfmt-only tests. The final
formatted `wallet-broker/src/vault.rs` hash is
`519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41`.

## Green commands

All commands exited 0. `cargo fmt --check` passed. `npm run build` passed. `npm test`
passed: Electron security 19, security policy 69, wallet contract 48, broker protocol
11, supervisor 11, and preload 6. `node scripts/security-policy.js` passed.
`npm audit --audit-level=low` found 0 vulnerabilities. Locked/offline full Rust tests
passed 78/78, including the independent crypto vector. All-targets/all-features
Clippy passed, and native-ui feature compilation passed.

`/home/lars/.cargo/bin/cargo-audit audit --file wallet-broker/Cargo.lock` exited 0,
loaded 1,226 advisories, and scanned 327 dependencies with no vulnerabilities. It
reported only that the local crates.io index was unavailable. No canary, scratch
residue, unrelated change, native window, package binary, wallet, node, device,
network client, or real secret was used. Local cargo-deny/cargo-cyclonedx remain
deferred to pinned GitHub workflows.

## Final hashes

`wallet-broker/Cargo.toml` `d1d338ff0cb63eb6c7f992b2573b6ebc4ee5d7d459301961eb0b4aaa8d2ebd7c`
`wallet-broker/src/lib.rs` `09e1ba98383fedda2b0db5e36ff716f0a8ce30ef37072901dc8b1e31be06dbdc`
`wallet-broker/src/vault.rs` `519b544e34cba1dcba240e93f03b19d05fb5137477bfb2441dd3a57c91c79a41`
`wallet-broker/src/store.rs` `611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236`
`wallet-broker/src/session.rs` `42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227`
`wallet-broker/src/native.rs` `50a078f05d8d66127fac0aae99343070758b0da549d5468ed2e0bd71ba0483e9`
`wallet-broker/src/native_ui.rs` `1c1da0425460154ea84a0751bfb9f40d0cea58b15efbf13aaebd0c463bde60b8`
`wallet-broker/src/hygiene.rs` `7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf`
`wallet-broker/tests/vault_crypto.rs` `26475b2ccddd692b036e5440fdfde66d105f943f6bde912d81391efe7984b76e`
`wallet-broker/tests/vault_format.rs` `5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193`
`wallet-broker/tests/vault_store.rs` `582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5`
`wallet-broker/tests/vault_session.rs` `67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b`
`wallet-broker/tests/native_surface.rs` `0308eaf8db147789287474a69b51de2ef50a6e93f286882cb1aa62d7de0f2586`
`wallet-broker/tests/secret_hygiene.rs` `3f809e06e96add88a91c232b7824531ddaaf320182e79d9e51cf3c6b61e42323`
`wallet-broker/Cargo.lock` `1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`
`deny.toml` `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8`
`scripts/security-policy.js` `02df7a6f656826a580d972a722e79038ad6eee70bce7e55c27a4c0245db8a853`
`scripts/validate-rust-sbom.js` `6b90e1a5dcc423a6a891cbf3d5964536e00772fb6549e64d82bce3fcec84b4a0`
`package.json` `9b8b03edc602554e98266d7a79168eacbffe4243a686b52d92bc1dd8a52e3893`
`.github/workflows/social.yml` `c2d7e2cca231d6b55b7403e756b39e2855421c5407d10fb2146d7493650f96a3`
`.github/workflows/security.yml` `64421f333299861103fdd8d3eee0df35414a40e45b5eef4f05d83cd1ebe3159a`
`.github/workflows/sbom.yml` `8407f00fc0ed9ad7bd88c726d64e5cd02a61922653991f9cf4b7cf8bea528824`
