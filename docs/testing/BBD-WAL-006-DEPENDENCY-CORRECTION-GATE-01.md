# BBD-WAL-006 Dependency Correction Gate 01

Timestamp: 2026-08-31T13:25:48-07:00 (America/Los_Angeles)

Protected governance parent: `63af10cc`

The required policy progression exited `1` for the expected reason: corrected
dependency-map equality passed, then the frozen checker rejected the six WAL-006
manifest additions at `scripts/security-policy.js:1840`.

The formatter check, three locked/offline feature trees, duplicate tree, metadata, and
locked/offline `vault_crypto` target all exited `0`. The custody target compiled and ran
exactly 11 tests: 11 passed, 0 failed, 0 ignored, 0 measured, 0 filtered. The tests
included independent Argon2id RFC 9106, HKDF-SHA256, XChaCha20-Poly1305, and
deterministic encrypted-envelope vectors. No warning, canary, or secret appeared.

Resolved lock inventory: 5,367 lines, SHA-256
`bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`; 513 package
stanzas versus 327 previously (textual diff +2,193/−99; package stanzas +192/−6).
All package sources are crates.io registry sources. Required direct pins/checksums are:

- `argon2 0.5.3` — `3c3610892ee6e0cbce8ae2700349fcf8f98adb0dbfbee85aec3c9179d29cc072`;
- `chacha20poly1305 0.10.1` — `10cd79432192d1c0f4e1a0fef9527696cc039165d729fb41b3f4f4f354c2dc35`;
- `hkdf 0.12.4` — `7b5f8eb2ad728638ea2c7d47a21db23b7b58a72ed6a38256b8a1849f15fbbdf7`;
- `sha2 0.10.9` — `a7507d819769d01a365ab707794a4084392c824f54a7a6a7862f8c3d0892b283`;
- `zcash_client_backend 0.24.0`, `zcash_client_sqlite 0.22.0`, `pczt 0.9.3`,
  `zcash_primitives 0.30.1`, `zcash_protocol 0.10.5`, and `zcash_keys 0.16.1`
  matched the fixed architecture-review versions and checksums.

The graph contains `aead 0.5.2`, `chacha20 0.9.1`, `cipher 0.4.4`, stable
`crypto-common 0.1.7`, Zcash `crypto-common 0.2.0-rc.1`, stable `digest 0.10.7`,
`hmac 0.12.1`, `blake2 0.10.6`, and Zcash `digest 0.11.0-pre.9`,
`hmac 0.13.0-pre.4`, `sha2 0.11.0-pre.4`. Rejected stable AEAD 0.11/aead 0.6,
crypto-common 0.2.1/0.2.2, and stable hmac/sha2 0.13/0.11 are absent. The feature
trees show the reviewed Orchard/PCZT/SQLite/local-consensus union; upstream transitive
PCZT signer/prover/finalizer/extractor capabilities remain compiled but are not exposed
by BitBook. Build-script and proc-macro packages are transitive registry packages only;
no git/path/patched source was selected.

The required disk boundary was the real, ignored ext4 `wallet-broker/target` with the
existing `wal006-tmp` and `wal006-cargo` paths. No fixture, ZEC adapter test, full suite,
live endpoint, wallet, node, device, or other repository was accessed.

Pre/post hashes:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 759 | `89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b` |
| `wallet-broker/tests/vault_crypto.rs` | 394 | `a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b` |
| `wallet-broker/Cargo.toml` | 79 | `89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3` |
| `scripts/security-policy.js` | 2,231 | `3551656d49bfdc717fdff627d137b477582f274565880aae51a20922ebd28e83` |
| `test/securityPolicy.node.js` | 2,388 | `29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573` |
| `wallet-broker/Cargo.lock` | 5,367 | `bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83` |

The six ZEC tests remain at their accepted format-correction hashes.

## Supply-chain metadata inventory addendum

Addendum timestamp: 2026-08-31T13:32:11-07:00 (America/Los_Angeles)

Protected governance parent: `a9bcc070`

The exact locked/offline metadata command was:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 metadata --manifest-path wallet-broker/Cargo.toml --locked --offline --format-version 1 > wallet-broker/target/wal006-metadata-inventory.json
```

Result: exit `0`. Capture path: `wallet-broker/target/wal006-metadata-inventory.json`;
1,305,072 bytes; SHA-256
`634e62b34362f9c042ddba8688fc540c1baaa793e2094d32d94ffb1131d86cee`.

The six direct Zcash lock checksums are:

- `zcash_client_backend 0.24.0`: `07ad58d8ca5daacbc089bf82ec09e45cf2a8be40adeb40b9399b16545c81a528`
- `zcash_client_sqlite 0.22.0`: `dd92e4334619b1e3f67019254049318ec0d0240a3c15d853bdf2ac4bba597078`
- `pczt 0.9.3`: `a5592f4f3eba7f9344cc423f45b6e65911e0630c9b89968d5a20792aadd5a0eb`
- `zcash_primitives 0.30.1`: `403d5be1e96339534be098e3377fb8a78d68ca7585b1780133d884b810277418`
- `zcash_protocol 0.10.5`: `314329b91ec4bbb517441840e47d0b2029bf0b946f086980c96c889c2d92dc5d`
- `zcash_keys 0.16.1`: `def800f128e459eedebc900f36f408eaf0687634128dcf64ecfeaeebc3e16c14`

The six required query outputs were:

```text
--- licenses
(MIT OR Apache-2.0) AND Unicode-3.0
Apache-2.0
Apache-2.0 / MIT
Apache-2.0 OR MIT
Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT
Apache-2.0/MIT
BSD-2-Clause OR Apache-2.0 OR MIT
BSD-3-Clause
CC0-1.0
CC0-1.0 OR MIT-0 OR Apache-2.0
MIT
MIT OR Apache-2.0
MIT OR Apache-2.0 OR LGPL-2.1-or-later
MIT OR Apache-2.0 OR Zlib
MIT/Apache-2.0
Unlicense OR MIT
Zlib
Zlib OR Apache-2.0 OR MIT
Zlib OR MIT OR Apache-2.0
--- none-license
--- sources
registry+https://github.com/rust-lang/crates.io-index
workspace
--- zcash
pczt	0.9.3	registry+https://github.com/rust-lang/crates.io-index	NONE	io-finalizer,orchard,prover,sapling,signer,spend-finalizer,transparent,tx-extractor,zcp-builder
zcash_client_backend	0.24.0	registry+https://github.com/rust-lang/crates.io-index	NONE	orchard,pczt,test-dependencies,transparent-inputs,unstable-serialization,unstable-spanning-tree,zcash_proofs
zcash_client_sqlite	0.22.0	registry+https://github.com/rust-lang/crates.io-index	NONE	orchard,test-dependencies,transparent-inputs
zcash_keys	0.16.1	registry+https://github.com/rust-lang/crates.io-index	NONE	orchard,sapling,std,test-dependencies,transparent-inputs
zcash_primitives	0.30.1	registry+https://github.com/rust-lang/crates.io-index	NONE	circuits,document-features,std,test-dependencies,transparent-inputs
zcash_protocol	0.10.5	registry+https://github.com/rust-lang/crates.io-index	NONE	document-features,local-consensus,std,test-dependencies
--- build
anyhow	1.0.104	registry+https://github.com/rust-lang/crates.io-index
atomic-polyfill	1.0.3	registry+https://github.com/rust-lang/crates.io-index
crossbeam-deque	0.8.7	registry+https://github.com/rust-lang/crates.io-index
crossbeam-epoch	0.9.20	registry+https://github.com/rust-lang/crates.io-index
crossbeam-utils	0.8.22	registry+https://github.com/rust-lang/crates.io-index
crunchy	0.2.4	registry+https://github.com/rust-lang/crates.io-index
defmt	1.1.1	registry+https://github.com/rust-lang/crates.io-index
defmt-macros	1.1.1	registry+https://github.com/rust-lang/crates.io-index
equihash	0.3.0	registry+https://github.com/rust-lang/crates.io-index
generic-array	0.14.7	registry+https://github.com/rust-lang/crates.io-index
getrandom	0.4.3	registry+https://github.com/rust-lang/crates.io-index
heapless	0.7.17	registry+https://github.com/rust-lang/crates.io-index
indexmap	1.9.3	registry+https://github.com/rust-lang/crates.io-index
libc	0.2.189	registry+https://github.com/rust-lang/crates.io-index
libm	0.2.16	registry+https://github.com/rust-lang/crates.io-index
libsqlite3-sys	0.35.0	registry+https://github.com/rust-lang/crates.io-index
num-traits	0.2.19	registry+https://github.com/rust-lang/crates.io-index
portable-atomic	1.15.0	registry+https://github.com/rust-lang/crates.io-index
portable-atomic-util	0.2.7	registry+https://github.com/rust-lang/crates.io-index
prettyplease	0.2.37	registry+https://github.com/rust-lang/crates.io-index
proc-macro2	1.0.107	registry+https://github.com/rust-lang/crates.io-index
quote	1.0.47	registry+https://github.com/rust-lang/crates.io-index
radium	0.7.0	registry+https://github.com/rust-lang/crates.io-index
rayon-core	1.13.0	registry+https://github.com/rust-lang/crates.io-index
rustix	1.1.4	registry+https://github.com/rust-lang/crates.io-index
rustversion	1.0.23	registry+https://github.com/rust-lang/crates.io-index
secp256k1-sys	0.10.1	registry+https://github.com/rust-lang/crates.io-index
serde	1.0.229	registry+https://github.com/rust-lang/crates.io-index
serde_core	1.0.229	registry+https://github.com/rust-lang/crates.io-index
serde_json	1.0.151	registry+https://github.com/rust-lang/crates.io-index
syn	1.0.109	registry+https://github.com/rust-lang/crates.io-index
thiserror	1.0.69	registry+https://github.com/rust-lang/crates.io-index
thiserror	2.0.20	registry+https://github.com/rust-lang/crates.io-index
wasm-bindgen	0.2.127	registry+https://github.com/rust-lang/crates.io-index
wasm-bindgen-shared	0.2.127	registry+https://github.com/rust-lang/crates.io-index
zcash_client_backend	0.24.0	registry+https://github.com/rust-lang/crates.io-index
zerocopy	0.8.56	registry+https://github.com/rust-lang/crates.io-index
zmij	1.0.23	registry+https://github.com/rust-lang/crates.io-index
--- proc
ambassador	0.4.2	registry+https://github.com/rust-lang/crates.io-index
darling_macro	0.23.0	registry+https://github.com/rust-lang/crates.io-index
defmt-macros	1.1.1	registry+https://github.com/rust-lang/crates.io-index
derive-getters	0.5.0	registry+https://github.com/rust-lang/crates.io-index
document-features	0.2.12	registry+https://github.com/rust-lang/crates.io-index
getset	0.1.7	registry+https://github.com/rust-lang/crates.io-index
jiff-static	0.2.35	registry+https://github.com/rust-lang/crates.io-index
prost-derive	0.14.4	registry+https://github.com/rust-lang/crates.io-index
rustversion	1.0.23	registry+https://github.com/rust-lang/crates.io-index
serde_derive	1.0.229	registry+https://github.com/rust-lang/crates.io-index
serde_with_macros	3.22.0	registry+https://github.com/rust-lang/crates.io-index
thiserror-impl	1.0.69	registry+https://github.com/rust-lang/crates.io-index
thiserror-impl	2.0.20	registry+https://github.com/rust-lang/crates.io-index
time-macros	0.2.32	registry+https://github.com/rust-lang/crates.io-index
tracing-attributes	0.1.31	registry+https://github.com/rust-lang/crates.io-index
visibility	0.1.1	registry+https://github.com/rust-lang/crates.io-index
wasm-bindgen-macro	0.2.127	registry+https://github.com/rust-lang/crates.io-index
zerocopy-derive	0.8.56	registry+https://github.com/rust-lang/crates.io-index
zeroize_derive	1.5.0	registry+https://github.com/rust-lang/crates.io-index
```

The none-license query returned no rows; sources are only workspace and crates.io.
The metadata query reports `NONE` for checksums because Cargo metadata omits registry
checksums; exact checksums remain in the committed lockfile. Zcash features are the
accepted direct requests plus Cargo unification. Build scripts/proc macros are
transitive registry capabilities and grant no BitBook signing, proving, extraction,
broadcast, or live-network authority.
