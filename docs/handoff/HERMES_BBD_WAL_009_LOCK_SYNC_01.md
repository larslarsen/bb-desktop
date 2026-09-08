# WAL-009 offline root lock synchronization 01

Actor: Hermes Jr Dev. Repository: `/home/lars/OpenBazaar/bb-desktop`.
Governance parent: the commit containing this handoff.

Read AGENTS.md, TESTING.md, HERMES_JR_DEV_ROUTING.md, this handoff, the leading active
CURRENT_TASK section, and BBD-WAL-009-ORCHARD-DECLARATION-03-REVIEW.md.
Use the engineering/testing directories for the two referenced records.
Do not reload historical handoffs or Rust implementations.

Preflight: record actual Hermes version/provider/model and HEAD; verify no staged
changes. The protected manifest is 122 lines, SHA-256
73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503.
Starting wallet-broker/Cargo.lock SHA-256 is
29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420.
Expected existing dirty files are package.json, package-lock.json, the Rust manifest,
native.rs, native_ui.rs, zec.rs, zec/prepare.rs, zec/store.rs, zec/test_support.rs,
and untracked zec/spend.rs. Preserve all of them exactly; hash before and after.

Only this execution is authorized, once, as the exact terminal command:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo update --manifest-path wallet-broker/Cargo.toml --offline --workspace
```

Required outcome: exit 0 and the only lockfile difference is adding "orchard" to
bitbook-wallet-broker's root dependency list. No package/version/checksum/source change,
new resolved package, manifest edit, network access, test/build, or policy run is
authorized. Inspect the lock diff and protected hashes after execution.
If another difference appears or execution fails, report the actual output and diff
and stop; do not repair, restore, rerun, or integrate it.

On exact success, create only
docs/testing/BBD-WAL-009-LOCK-SYNC-01.md with runtime identity, actual command/exit,
starting/resulting lockfile SHA-256, exact diff, and protected-file preservation.
Report that source, policy, cryptographic and native-UI gates remain pending.
Do not update CURRENT_TASK, stage, commit, push, or run another actor. Leave lockfile
and evidence uncommitted for reviewer inspection. This is lock synchronization only.
