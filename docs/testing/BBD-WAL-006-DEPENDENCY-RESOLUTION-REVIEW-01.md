# BBD-WAL-006 Dependency Resolution Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `841a432f`

Result: **LOCK RESOLUTION BLOCKED — TEST-FIRST CUSTODY-PIN CORRECTION AUTHORIZED**

## Observed stop

Luna revalidated the accepted eight-path Phase-A drop and the unchanged 3,273-line
`wallet-broker/Cargo.lock` with SHA-256
`1d182550f82b9dc443804d82e709b5d58021d4243bef1612c440aac59df1c2f5`. The exact
formatter command then exited 0 without output or a source change.

The next authorized `cargo generate-lockfile` reached crates.io resolution only after
the approved network retry and exited 101 before changing the lockfile. Cargo reported
that `bip32 0.6.0-pre.1`, selected by `zcash_client_backend 0.24.0`, requires exact
`hmac 0.13.0-pre.4`, while the accepted WAL-004 direct `hkdf 0.13.0` pin had already
selected stable `hmac 0.13.0`. No fixture generation or test execution followed.

## Published graph contradiction

The registry manifests establish both sides of the contradiction:

- `zcash_client_backend 0.24.0` feature `pczt` enables `transparent-inputs` and its
  exact `bip32 0.6.0-pre.1` dependency;
- `bip32 0.6.0-pre.1` pins exact `hmac 0.13.0-pre.4` and exact
  `sha2 0.11.0-pre.4`;
- stable `hkdf 0.13.0` requires `hmac ^0.13`, and the broker directly pins stable
  `sha2 0.11.0`; and
- Cargo cannot select the stable and prerelease releases together within those same
  compatible version lines.

Changing only `hmac` would expose the corresponding `sha2` contradiction next. The
fixed Zcash versions/features cannot be weakened: backend PCZT and SQLite transparent
input support are deliberate upstream compiled capabilities, even though BitBook is
forbidden from exposing signing, proving, extraction, broadcast, or networking.

## Accepted minimal correction

The custody implementation remains on stable RustCrypto releases, but moves to disjoint
stable version lines that can coexist with Zcash's exact prereleases:

| Direct dependency | Corrected exact pin | Required transitive line | Registry checksum |
| --- | --- | --- | --- |
| `hkdf` | `=0.12.4` | `hmac ^0.12.1` | `7b5f8eb2ad728638ea2c7d47a21db23b7b58a72ed6a38256b8a1849f15fbbdf7` |
| `sha2` | `=0.10.9` | `digest ^0.10.7` | `a7507d819769d01a365ab707794a4084392c824f54a7a6a7862f8c3d0892b283` |

Both direct dependencies remain exact and `default-features = false`. This introduces
no loose, git, patched, vendored, network, downloader, OpenSSL, FFI, or first-party
unsafe dependency. No Zcash version or feature substitution is authorized.

The reviewed broker call surface is only `Hkdf::<Sha256>::new(...).expand(...)` and
`Sha256`/`Digest`. API similarity is not acceptance of cryptographic equivalence. Before
fixture generation, Luna must run the complete existing `vault_crypto` target after lock
resolution. It includes the independent RFC 5869 HKDF-SHA256 vector and the exact
deterministic encrypted-envelope bytes. Any compile difference, vector difference,
envelope difference, warning, or unrelated failure rejects the correction.

## Test-first sequence

1. Sol changes only the Node test-side WAL-004 dependency expectation and adds explicit
   mutations proving both corrected exact pins reject the superseded stable pins and
   loose requirements. Manifest and production policy remain frozen.
2. After reviewer source acceptance, Luna executes only that one test case. It must fail
   at the old production-policy dependency map before manifest validation. Luna records
   and integrates the exact red test source.
3. Only after the red is accepted may Sol change the two manifest pins and the matching
   production-policy constants/required lines.
4. After reviewer source acceptance, Luna resolves the graph, inventories all crypto
   duplicates/features/build scripts/sources/licenses, runs `vault_crypto`, and resumes
   the already authorized fixture and expected-red handoff only on exact green.

The remaining six Rust tests, the rest of the Node suite, the current manifest, policy,
lockfile, production source, fixture bytes, workflows, package inputs, and other
repositories remain unauthorized during the test correction.
