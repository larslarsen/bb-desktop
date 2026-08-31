# BBD-WAL-006 Dependency Resolution Review 02

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `24c1a452`

Result: **SECOND LOCK CONTRADICTION — TEST-FIRST ARGON2 PIN CORRECTION AUTHORIZED**

## Observed stop

Luna's preflight passed with the exact accepted eight-path state. The focused Node case
also progressed exactly as required: the corrected WAL-004 dependency maps matched, and
the still-WAL-004-only production checker rejected the six WAL-006 manifest additions at
`scripts/security-policy.js:1840`.

The next exact `cargo generate-lockfile` exited 101 before lock mutation. Cargo reported
that direct `argon2 0.6.0` selects `blake2 0.11.0`, which requires stable `digest ^0.11`,
while Zcash's `bip32 0.6.0-pre.1` selects exact `hmac 0.13.0-pre.4`, which requires exact
`digest 0.11.0-pre.9`. Cargo offered stable `digest 0.11.2`/`0.11.3` for the first
requirement and could not select both compatible-line stable/prerelease releases. No
fetch, tree, metadata, Rust test, fixture, evidence, integration, or Git action followed.

## Accepted minimal correction

Move the exact, defaults-off direct Argon2 dependency from `0.6.0` to stable `0.5.3`:

| Crate | Exact version | Registry checksum | Relevant published dependency |
| --- | --- | --- | --- |
| `argon2` | `0.5.3` | `3c3610892ee6e0cbce8ae2700349fcf8f98adb0dbfbee85aec3c9179d29cc072` | `blake2 ^0.10.6` |
| `blake2` | resolved `0.10.6` | `46502ad458c9a52b69d4d4d32775c788b7a1b85e8bc9d482d92250fc0e3f8efe` | `digest ^0.10.3` |

This keeps custody on a stable disjoint digest line alongside accepted stable
`hkdf 0.12.4`, `hmac 0.12.1`, and `sha2 0.10.9`, while the fixed Zcash graph retains its
exact prerelease digest/HMAC/SHA-2 line. `argon2 0.5.3` retains the broker's reviewed
`alloc` feature and the used `Params`, `ParamsBuilder`, `Algorithm`, `Version`,
`Argon2::new`, `Argon2::new_with_secret`, and `hash_password_into` surface.

API compatibility is not cryptographic acceptance. After an exact test-first policy red
and bounded manifest/policy correction, Luna must resolve the graph and run the complete
11-test `vault_crypto` target. Its independent Argon2id RFC 9106 vector, HKDF vector,
XChaCha20-Poly1305 vector, and deterministic encrypted envelope must all remain exact.
No Zcash pin/feature substitution, patch, alias, git/path dependency, or policy weakening
is authorized.
