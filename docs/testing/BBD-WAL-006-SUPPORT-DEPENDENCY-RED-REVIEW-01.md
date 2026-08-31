# BBD-WAL-006 Support-Dependency Expected-Red Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Reviewed integration commit: `1f24a22191c25a049ea690170b751be85eef4308`

Result: **ACCEPTED — MANIFEST AND NARROW POLICY SOURCE AUTHORIZED**

The focused Node policy file exited 1 with exactly 66 `ok`, eight `not ok`, and
`8 security policy test(s) failed`. The seven previously accepted groups retained their
causes. The new support-dependency group failed first at the real-manifest exact-line
assertion: both accepted support lines had occurrence count zero, before the policy
export or checker ran. No prior `ok`, syntax, fixture, module, or unrelated path failed.

The accepted test is 2,454 lines, 74 named tests, SHA-256
`f8340ae965a1f59e27594153cd3c9f3eca5ee1d8a78ed0a9a685faf6f1dc2647`.
The evidence is 58 lines, SHA-256
`6f6d313743995238a2e0fa7e97f075600770937ae1658028fe86acba744c318d`.
The manifest remains SHA-256
`89f02a18de21114d96c9f7231a36d8b60434c757e996314cb7f33403ae51c6a3`
and the lockfile remains SHA-256
`bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`.
`HEAD == origin/master` at the reviewed commit with a clean tracked worktree/index and
no ZEC production source.

Sol may now add exactly the two tested manifest lines and the manifest-policy constants,
exact inventories, exports, and checker handling needed to accept the already-committed
six Zcash pins/test targets plus the two support pins. This is not authorization for
feature-authority policy, ZEC source-inventory policy, ZEC source screening, lockfile
mutation, or Rust production source. Luna alone will resolve and inspect the lock.
