# Codex Sol Handoff — BBD-WAL-006 Prepare Stage Diagnostic 01

You are **Principal Dev — Codex Sol**. Own only temporary closed stage instrumentation. Do not
execute any command or use Git.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely: `AGENTS.md`, `TESTING.md`, Prepare Gate Targeted Review 01,
`wallet-broker/src/zec/store.rs`, and `CURRENT_TASK.md`.

Use `apply_patch` to modify only `wallet-broker/src/zec/store.rs`, currently 2,048 lines and
SHA-256 `f9f66f98f33b8457c955125b77453be018397ab120f78618d52ed817200fcf34`.

At exactly the four erased post-proposal error mappings, add a fixed `eprintln!` immediately before
returning the unchanged `ZecError::internal()`:

- `BBD-WAL-006-DIAGNOSTIC:create-pczt`
- `BBD-WAL-006-DIAGNOSTIC:serialize-pczt`
- `BBD-WAL-006-DIAGNOSTIC:own-secret-bytes`
- `BBD-WAL-006-DIAGNOSTIC:parse-pczt`

Do not print or interpolate the error or any other value. Do not change control flow, public
errors, types, inspection, redaction, or any other line. This instrumentation is temporary and
must never be staged or committed. Do not modify tests or another path. Do not run formatter,
compiler, tests, Node, network, or any other command. Report direct patch accounting; reviewer
will verify the resulting identity.
