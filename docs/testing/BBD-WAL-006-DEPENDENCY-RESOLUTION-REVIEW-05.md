# BBD-WAL-006 Dependency Resolution Review 05

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `c5a05b32`

Result: **FIRST API RED REJECTED AS INCOMPLETE — TEST CONSTRUCTOR CORRECTION AUTHORIZED**

Luna's locked/offline custody command exited 101 before tests, but correctly stopped
because the compiler produced more than the handoff's single expected `E0432`. Stable
AEAD 0.10 also lacks `TryFrom<&[u8]>` for its GenericArray `XNonce`/`Tag` aliases and
lacks both `encrypt_inout_detached` and `decrypt_inout_detached`. No evidence or Git
action followed.

The fetched authoritative API uses `XNonce::from_slice`/`Tag::from_slice` and
`AeadInPlace` methods. The test compatibility drop already corrected its trait/method but
still calls `XNonce::try_from`. Sol may change only that constructor to
`XNonce::from_slice` and pass the resulting reference directly. Its fixed 24-byte input
makes the slice-length invariant non-vacuous and unchanged. All vector bytes/assertions
remain immutable.

After this test correction, the expected red must contain only frozen production's
trait, nonce/tag constructor, and two method incompatibilities; the corrected test must
add none. Production remains unauthorized until reviewer accepts that exact compiler
red.
