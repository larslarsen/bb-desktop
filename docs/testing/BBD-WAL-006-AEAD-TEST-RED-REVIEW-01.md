# BBD-WAL-006 AEAD Expected-Red Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Evidence commit: `9bfb8762`

Result: **EXPECTED RED ACCEPTED — BOUNDED AEAD MANIFEST/POLICY CORRECTION AUTHORIZED**

Luna's one focused run exited 1 at the first dependency-map assertion before manifest
validation/mutation. The sole difference was production `chacha20poly1305 =0.11.0`
versus accepted test `=0.10.1`; Argon2, HKDF, SHA-2, and every other field matched. No
other command, error class, canary, or secret occurred.

The accepted test is committed at 2,388 lines with SHA-256
`29cbe64a0a217a9a94eca23c52126b06174b79c547f512d8191282c0e00a4573`; evidence is 33
lines with SHA-256 `3eb631a79a00d3f7ab465e3b0ec5f726799e98c7c1f2fd329bd472af68351a09`.
Sol may change only the direct manifest AEAD version and two matching production-policy
literals. All other paths/actions remain frozen.
