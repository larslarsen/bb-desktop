# BBD-WAL-006 Dependency Test Expected-Red Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Evidence commit: `99bf5b1d`

Result: **EXPECTED RED ACCEPTED — BOUNDED MANIFEST/POLICY CORRECTION AUTHORIZED**

Luna ran the one authorized focused Node case exactly once. It found the test exactly
once and exited 1 at `test/securityPolicy.node.js:1657`, the first
`assert.deepStrictEqual`, before manifest validation or any mutation.

The diagnostic showed only the intended dependency-map differences: frozen production
expected `hkdf =0.13.0` and `sha2 =0.11.0`; the accepted test expected exact
`hkdf =0.12.4` and `sha2 =0.10.9`. No syntax/load error, manifest-policy error, extra
field difference, abort, signal, canary, secret, or broader command occurred.

The accepted test is committed at 2,374 lines with SHA-256
`f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4`. The manifest,
production policy, lockfile, and six uncommitted Rust tests remained byte-exact. Evidence
is 33 lines with SHA-256
`11bab5e871c0f0be61326979e2ffb650a2f95104003a1fe47bda311fa60287d5`.

Sol may now change only the two direct manifest pins and their four matching
production-policy literals. It may not teach the policy about WAL-006, change a Zcash
pin/feature, edit test source, resolve the graph, or run a command. Luna will own the
subsequent lock resolution, custody-vector gate, evidence, integration, and Git under a
separate handoff after reviewer source acceptance.
