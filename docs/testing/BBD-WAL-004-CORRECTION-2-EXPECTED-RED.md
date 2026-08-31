# BBD-WAL-004 Correction 2 Expected-Red Evidence

Timestamp: 2026-08-30 (PDT)
Governance baseline: `HEAD == origin/master == 6e5a427c68f66981f0d4bd127ff1496d958a23a6`

Accepted test paths were verified unchanged before execution:

- `wallet-broker/tests/vault_session.rs` — 305 lines — `6857c2a91fa70f13cdcd0767b1b5207243aab505a87d6edef9b9b9581e1a08c9`
- `wallet-broker/tests/native_surface.rs` — 427 lines — `302b97649681673034e5979a3691531860107c64047380a427ec28466f5f7be9`
- `wallet-broker/tests/vault_store.rs` — 524 lines — `94c94f37df25c5123c4caddffc948370a43d4dabafdfd790c0b7f0f44b6c2175`
- `test/securityPolicy.node.js` — 2,053 lines — `2fe970d8dbea296714a483c02d288612350c6b8e5f4bd9f6650c8cb5d2c19ca3`

Rust/Cargo reported 1.98.0. All locked offline commands used the repository
`wallet-broker/target`; no `/tmp` target or cache was used.

## Rust commands

- `/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_session` — exit 101; 13 tests, 11 passed and 2 expected failed:
  `clock_failure_during_unlock_explicitly_wipes_supplied_material` and
  `global_lock_events_ignore_malformed_account_but_scoped_events_reject_it`.
- Same command with `--test native_surface` — exit 101; 13 tests, 11 passed and 2 expected failed:
  `invalid_utf8_restore_passphrase_wipes_before_custody_or_commit` and
  `invalid_utf8_unlock_passphrase_wipes_before_custody`.
- Same command with `--test vault_store` — exit 101; 20 tests, 19 passed and 1 expected failed:
  `linux_direct_operations_reject_wrong_mode_until_descriptor_repair`.

All three binaries compiled and reached execution. No prior accepted test failed.

## Node policy command

`node test/securityPolicy.node.js` — exit 1 after all 65 tests ran: 61 `ok`, 4
`not ok`. Failures were:

1. `committed workflows satisfy the fail-closed checker` — source inventory is
   rejected as missing, extra, or reordered.
2. `strict eight-line inherited Gitleaks ratchet bytes and content are enforced` —
   repository checking reaches the same order-sensitive source inventory defect.
3. `WAL-004 Rust first-party source policy forbids unsafe and unreviewed authority` —
   frozen vault source omits the reviewed `Base64Unpadded` primitive.
4. `WAL-004 Rust source inventory is exported closed and enumerated by repository policy` —
   filesystem enumeration order is rejected despite exact membership.

The five new Rust regressions and the extended inventory regression failed only for
the reviewed Correction 2 production gaps. No secret canary appeared. No compile,
setup, offline, unrelated, abort, signal, or unlisted-path failure occurred.

All 15 frozen production paths remained byte-identical before and after execution:

`wallet-broker/Cargo.toml` `d1d338ff0cb63eb6c7f992b2573b6ebc4ee5d7d459301961eb0b4aaa8d2ebd7c`;
`wallet-broker/src/lib.rs` `09e1ba98383fedda2b0db5e36ff716f0a8ce30ef37072901dc8b1e31be06dbdc`;
`wallet-broker/src/vault.rs` `39c0aa7dac2930a2c276a11e65779788062337dfdc438bc2a47903c4b4cb9ce7`;
`wallet-broker/src/store.rs` `32a08c17f0cb139aa9b1905e8993b075d357d363cd8a4cf604fec6d2d9ba85aa`;
`wallet-broker/src/session.rs` `40a093c4327e23e41ed0dc7a07315375543d4c3672b4bb36c39ce2e909c1ee81`;
`wallet-broker/src/native.rs` `b3ce8a2dfa2b7a5646823b6d9f35de56e43a85dae4702b733376881fa40d6610`;
`wallet-broker/src/native_ui.rs` `3a255c443eeabb0ea0e04e32815eba499ada940186769fbf1257bcc62579d9dc`;
`wallet-broker/src/hygiene.rs` `ea600c1a4d4f178570237c63892fd5de450ce6166a48117f5f86e3ce7da06dfe`;
`deny.toml` `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8`;
`scripts/security-policy.js` `486196e8b1791b522efee2ec36e1563e108ff83198b59a696273cbfbdffa0dda`;
`scripts/validate-rust-sbom.js` `6b90e1a5dcc423a6a891cbf3d5964536e00772fb6549e64d82bce3fcec84b4a0`;
`package.json` `9b8b03edc602554e98266d7a79168eacbffe4243a686b52d92bc1dd8a52e3893`;
`.github/workflows/social.yml` `c2d7e2cca231d6b55b7403e756b39e2855421c5407d10fb2146d7493650f96a3`;
`.github/workflows/security.yml` `64421f333299861103fdd8d3eee0df35414a40e45b5eef4f05d83cd1ebe3159a`;
`.github/workflows/sbom.yml` `8407f00fc0ed9ad7bd88c726d64e5cd02a61922653991f9cf4b7cf8bea528824`.
