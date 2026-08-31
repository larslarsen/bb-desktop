# BBD-WAL-006 Dependency Resolution Review 03

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `71dc50b7`

Result: **THIRD LOCK CONTRADICTION — TEST-FIRST AEAD PIN CORRECTION AUTHORIZED**

Luna's exact preflight and focused policy progression passed. The next exact
`cargo generate-lockfile` again exited 101 before lock mutation. Direct
`chacha20poly1305 0.11.0` requires `aead ^0.6`, which requires stable
`crypto-common ^0.2`; fixed `zcash_primitives 0.30.1` had already selected exact
`crypto-common 0.2.0-rc.1`. Cargo offered stable `0.2.1`/`0.2.2` for the direct AEAD
line and could not select both compatible-line stable/prerelease releases. No later
command, evidence, integration, or Git action ran.

The accepted minimal correction moves the direct AEAD to exact, defaults-off stable
`chacha20poly1305 0.10.1` with only `alloc`:

| Crate | Version | Registry checksum | Relevant line |
| --- | --- | --- | --- |
| `chacha20poly1305` | exact `0.10.1` | `10cd79432192d1c0f4e1a0fef9527696cc039165d729fb41b3f4f4f354c2dc35` | `aead ^0.5`, `chacha20 ^0.9`, `cipher ^0.4`, `poly1305 ^0.8` |
| `aead` | resolved `0.5.2` | `d122413f284cf2d62fb1b7db97e02edb8cda96d769b16e443a4f6195e35662b0` | `crypto-common ^0.1.4` |
| `chacha20` | resolved `0.9.1` | `c3613f74bd2eac03dad61bd53dbe620703d4371614fe0bc3b9f04dd36fe4e818` | `cipher ^0.4.4` |
| `cipher` | resolved `0.4.4` | `773f3b9af64447d2ce9850330c473515014aa235e6a783b02db81ff39e4a3dad` | `crypto-common ^0.1.6` |

This stable 0.1 crypto-common line is disjoint from Zcash's required 0.2 prerelease.
The broker's used `AeadInOut`, `KeyInit`, `XChaCha20Poly1305`, `XNonce`, `Tag`, and
detached in-place encrypt/decrypt surface exists on the accepted stable line. API
similarity is not behavioral acceptance: the independent XChaCha20-Poly1305 draft vector,
deterministic encrypted envelope, authenticated-domain failures, and all 11 custody tests
must remain exact after test-first policy red and bounded manifest/policy correction.

No Zcash pin/feature substitution, patch, alias, git/path dependency, or policy weakening
is authorized.
