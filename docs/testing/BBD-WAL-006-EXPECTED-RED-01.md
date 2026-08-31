# BBD-WAL-006 Fixture and Expected-Red Evidence 01

Reviewer-authorized integration record for the fixture and expected-red phase.

Protected governance parent: `26de9506`

## Prior accepted stops

Resume 04 stopped after the preflight because the previous fixture attempt was
not accepted: two tests passed and two failed with
`RequestedRewindInvalid { safe_rewind_height: None, requested_height: BlockHeight(106) }`
at upstream `zcash_client_backend-0.24.0/src/data_api/testing.rs:970:54`.
No source repair was performed by Luna.

Resume 05 stopped on the exact formatter rejection in
`wallet-broker/tests/zec_fixture_builder.rs:69`:

```diff
-            let duplicate_height = u32::try_from(block.height)
-                .expect("duplicate compact-block height exceeded u32");
+            let duplicate_height =
+                u32::try_from(block.height).expect("duplicate compact-block height exceeded u32");
```

The corrected accepted test is 928 lines,
SHA-256 `40cc2b56132b42a765c86482e9915b0422adc0154c1e2edcfda4623760ec5d09`,
and its formatter gate passed.

## Fixture generation and freeze

The locked/offline builder was run twice under the accepted contract. Each run
reported exactly:

```text
running 4 tests
test fixture_output_ancestor_policy_rejects_symlink_and_nondirectory_without_io ... ok
test fixture_builder_refuses_every_other_output_location ... ok
test fixture_builder_writes_or_verifies_only_the_fixed_target_path ... ok
test upstream_fixture_oracle_is_deterministic_and_closed ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The second run verified the existing output byte-for-byte and performed no wallet
rewind. Generated output and frozen files are equal for every path (`cmp`), the
manifest is 8,708 bytes, and all generated/frozen files are private (`0600`; the
two directories are `0700`). The frozen inventory is exactly the following 16
files; each row records the manifest linkage/scenario and frozen bytes/hash.

| Path | Manifest linkage/scenario | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `manifest.json` | fixture format/closed inventory | 8708 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |
| `blocks/canonical-000100.compact` | canonical height 100 | 270 | `a228cae6189ccbff51ed901e641dd59916beb223ff7244359a0d3b334fcaad06` |
| `blocks/canonical-000101.compact` | canonical height 101 | 74 | `6ef47b02e1324b31b6187b4c96e396f23b20f988f5693deb6caca3fe4d4b1195` |
| `blocks/canonical-000102.compact` | canonical height 102 | 74 | `8826eacd4aa8d68b5c7cbcd8dc4afbf13ec74cafeea30ce3d4ae590773e0a964` |
| `blocks/canonical-000103.compact` | canonical height 103 | 270 | `b0f51ef571fda2516ecd6982e9523e7567a9d38236bc81a07df56cc3b510fe64` |
| `blocks/canonical-000104.compact` | canonical height 104 | 272 | `514f9ebea982e455dcf81576a4f6dd5a144d7399a9af61bbb2497dc7b66f72fb` |
| `blocks/canonical-000105.compact` | canonical height 105 | 76 | `74319b469c52e1b6ab876ef04fbf4b38b1b65aaa11c630c37aa52a9f6ace957a` |
| `blocks/canonical-000106.compact` | canonical height 106 | 76 | `5cfe279807dbf3cb88d4e6b470b5f5c8f24c59b91f9836248b2e9c5ba9dd8a45` |
| `blocks/canonical-000107.compact` | canonical height 107 | 272 | `7ae90de483f9d2f0628be98c9129ac36dc6427c70bb62e74b2faae1fecbf1bf6` |
| `blocks/reorg-replacement-000107.compact` | one-block reorg replacement | 272 | `503565aeb51f7065a5a614d14cd4e8045fb865b84e480db3ae63ef4a0de800e9` |
| `blocks/discontinuity-wrong-prev-000107.compact` | wrong-prev discontinuity | 272 | `2db71f38015ca0071c462c61a7bbce080da09319e89398db6d711fc3c927c168` |
| `blocks/discontinuity-height-gap-000109.compact` | height-gap discontinuity | 272 | `b0eaaf0ba0a881a6ba7fe33def1418e25a62e308a2c1de7922c77b08fc107213` |
| `blocks/impossible-tree-state-000107.compact` | impossible tree state | 268 | `67017418f42edaf876ef19df78a495823773b1494d258319f59de3880d294ef0` |
| `blocks/truncated-000107.compact` | truncation corruption | 271 | `0edc2de6333f781ebace3f4cd5480c64946fa76c9b583336de3962e4f885d174` |
| `blocks/malformed.compact` | malformed corruption | 1 | `76be8b528d0075f7aae98d6fa57a6d3c83ae480a8469e668d7b0af968995ac71` |
| `blocks/corrupt-wire-type-000107.compact` | corrupt wire type | 272 | `bbfd9bbe5d5f91f3dde5ea436c75378d4a3cdf1174edd90e37f5734e1bf609d4` |

The manifest links canonical heights 100–107, a common height-106 parent for the
two height-107 branches, and all listed closed corruption/discontinuity scenarios.
It contains no extra file, symlink, secret, mnemonic, mainnet, endpoint, or URL
text. Generator versions are zcash_client_backend 0.24.0, zcash_client_sqlite
0.22.0, zcash_pczt 0.9.3, zcash_primitives 0.30.1, zcash_protocol 0.10.5, and
zcash_keys 0.16.1.

## Expected-red results

The complete Node policy command produced exactly 66 `ok`, 7 `not ok`, and the
final line `7 security policy test(s) failed`. The seven authorized failures were:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`
5. `WAL-006 feature policy distinguishes compiled upstream PCZT capability from BitBook authority`
6. `WAL-006 Rust ZEC product source inventory remains empty during test-only Phase A`
7. `WAL-006 policy rejects live-network and authority-bearing Rust snippets without denying upstream transitives`

The focused Rust adapter command exited 101 before test execution: zero tests
ran, with only the absent production `zec` module diagnostics `E0433` at
`tests/zec_address.rs:6` (`zec::test_support`) and `E0432` at line 9
(`zec`). This is the accepted reviewer exception to the earlier E0433-only
wording; both errors arise solely from the intentionally absent production module.
There were no fixture, upstream API/type, manifest/lock, library, linker,
runtime, warning, canary, secret, or other-path failures.

## Integrity and negative-capability record

Before and after the accepted run, the committed inputs remained unchanged:

| Input | SHA-256 |
| --- | --- |
| `wallet-broker/Cargo.toml` | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `wallet-broker/Cargo.lock` | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |
| `scripts/security-policy.js` | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/src/vault.rs` | `89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b` |

The six accepted tests retained their reviewed hashes; no production Rust,
manifest, policy, lockfile, or fixture-generator source was changed. No
network/live endpoint, wallet/node/device, secret, mnemonic, canary, mainnet,
authority-bearing production `zec` capability, or generated output outside the
fixed ignored target was accessed. This record is evidence only; Phase C remains
unauthorized pending XHigh review.
