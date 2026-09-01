# BBD-WAL-006 Prepare NFC Dependency Expected-Red Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Integrated commit: `29f3bc9abacfa2d32b72f3b9a7104115a9c38451`

Result: **ACCEPTED — DEPENDENCY PRODUCTION CORRECTION AUTHORIZED**

The integrated commit contains exactly the accepted one-file test source, Hermes evidence, and
`CURRENT_TASK.md`. `HEAD` equals `origin/master` and the worktree/index are clean.

Hermes Agent v0.18.2, provider `nous`, model `meituan/longcat-2.0:free` ran exactly
`node test/securityPolicy.node.js`. It exited 1 with exactly 68 `ok`, exactly seven `not ok`, and
final line `7 security policy test(s) failed`. The six prior partial-red names were unchanged. The
only new failure was the exact NFC dependency test, at its first prerequisite because the real
manifest contained zero copies of the required exact declaration. No missing-export, syntax,
load, fixture, or unrelated error substituted for the intended red.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,525 | `a70d3491c359b8d59a3b7cdb1307ee549b9c0cf8dad310570a47157d066d68ba` |
| expected-red evidence | 60 | `7dfa25128ddf63bd0423e8fe9c112f6515cdfc6cfee3eedb77159cd97f490744` |

The exact-red evidence is sufficient to authorize only the manifest and manifest-policy
implementation. `Cargo.lock`, Rust prepare source/tests, fixtures, and all other paths remain
frozen until that dependency correction is independently reviewed and integrated.

