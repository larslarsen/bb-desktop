# BBD-WAL-006 Address Test-Root Correction Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `0339ffe0`

Result: **SOURCE ACCEPTED — FRESH GATE RESUME AUTHORIZED**

Sol changed only `wallet-broker/src/zec/test_support.rs`, now 389 lines with SHA-256
`f7fa31df8f707ead35bba1cd3904c7a4d4b0610bcfe860d710e57a4c12d44ca5`.
The complete six-path source is 1,854 lines and source-only `git diff --check` is clean.

The accepted correction uses `DirBuilderExt` to create a missing shared ancestor atomically with
mode `0700`. A concurrent `AlreadyExists` result triggers fresh `symlink_metadata` and proceeds
only for a real nonsymlink directory; other errors and hostile types remain fail-closed. Existing
repository/build ancestors are not chmodded, and unique state-root allocation plus production path
validation are unchanged.

No command was executed by the source actor or reviewer. Luna must restart the complete gate at
the formatter and reuse no earlier formatter, Clippy, or test result.
