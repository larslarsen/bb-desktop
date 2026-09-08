# WAL-009 Correction 03 — declare the pinned verifier dependency

Actor: Grok Build High. Repository: `/home/lars/OpenBazaar/bb-desktop`.
Governance parent: the commit containing this handoff.

This is a single-line manifest correction, not another implementation pass.
Read AGENTS.md, this handoff, the leading active CURRENT_TASK section, and
docs/testing/BBD-WAL-009-TRANSACTION-CLEANUP-02-REVIEW.md. Read Cargo.toml.
No historical handoff or dependency-tree reading is needed.

Authorized edit: wallet-broker/Cargo.toml only.
Starting identity: 121 lines, SHA-256
71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450.

Add this exact dependency entry alongside pczt in [dependencies]:

```toml
orchard = { version = "=0.15.5", default-features = false, features = ["circuit"] }
```

The reviewer explicitly authorizes this narrow exception to the original frozen
manifest because spend.rs already uses orchard::circuit::VerifyingKey directly.
orchard 0.15.5 is already locked and circuit is already enabled by pczt/prover.
The accepted sign/verify test exercises that verifier; do not change its assertions.
This declaration does not resolve the remaining verifier/oracle/UI issues.

Do not edit Cargo.lock; Hermes must synchronize its root dependency entry later
under an explicit offline handoff, verify no version/package/feature expansion, and
run the dependency/policy gates before any acceptance. No execution is authorized now.

Preserve all other pending source and unrelated package edits. Run no Cargo, npm,
formatter, compiler, test, Git, network, audit, scanner, product, or other actor.
Use apply_patch for the one-line edit. Report the final manifest line count/SHA-256
and stop. Do not broaden the task or inspect all seven Rust files.
