# BBD-WAL-006 Support-Dependency Gate Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `2c0a32ec`

Result: **EXPECTED POLICY RESULT ACCEPTED — LOCK RESOLUTION RESUME REQUIRED**

Luna's focused Node command matched the contract exactly: exit 1, 71 `ok`, three
`not ok`, and only the intentionally deferred ZEC feature-authority, exact source
inventory, and ZEC source-screening groups remained red. The manifest/workflow/Gitleaks,
WAL-004, six-Zcash-pin, and support-pin groups all advanced to green.

The next offline `cargo metadata --no-deps` command exited 0 but did not update
`Cargo.lock`. The root `bitbook-wallet-broker` dependency list therefore still omitted
the new direct `rand_core` and `rusqlite` names. This violates the handoff's required
lock diff, so Luna correctly stopped before feature trees, custody tests, evidence, or
Git. The accepted manifest/policy remain the only modified paths; the lock is still the
frozen pre-correction hash.

The failure is in the reviewer-selected resolution command, not the accepted source.
`--no-deps` does not force the required root lock refresh. The resume replaces that step
with one unlocked, offline `cargo check` limited to the existing `vault_crypto` target,
then requires the same exact root-only lock diff before any further command. No source
repair, broader target, or network is authorized.
