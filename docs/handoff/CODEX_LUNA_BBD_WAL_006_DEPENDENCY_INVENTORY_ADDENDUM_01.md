# Codex Luna Handoff — BBD-WAL-006 Dependency Inventory Addendum 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, the ticket, dependency correction gate evidence/review,
and current integrated manifest/lock. Require protected `HEAD == origin/master`, clean
index, `git diff --check`, and exactly the six accepted untracked ZEC tests. Require the
committed 5,367-line lock at SHA-256
`bf9a36a70c96afe3c0b68355f082342c3e625e4764782228be9f4663b635cd83`
and the 55-line evidence at SHA-256
`27c77f4e240709aaef74c663f82b2006f996752b7b459dc6cf1a81f8ba67f6b8`.

Revalidate the ignored, real, disk-backed `wallet-broker/target` boundary. Run exactly
one Cargo command, locked and offline, writing only this ignored scratch file:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 metadata --manifest-path wallet-broker/Cargo.toml --locked --offline --format-version 1 > wallet-broker/target/wal006-metadata-inventory.json
```

It must exit 0 without changing a tracked/untracked source, test, manifest, policy, lock,
or fixture byte. Run only these read-only inventory queries over that exact capture:

```text
jq -r '[.packages[] | .license // "NONE"] | unique[]' wallet-broker/target/wal006-metadata-inventory.json
jq -r '.packages[] | select(.license == null) | [.name, .version, (.source // "workspace")] | @tsv' wallet-broker/target/wal006-metadata-inventory.json | sort -u
jq -r '[.packages[] | .source // "workspace"] | unique[]' wallet-broker/target/wal006-metadata-inventory.json
jq -r '. as $root | .resolve.nodes[] | . as $node | $root.packages[] | select(.id == $node.id) | select(.name == "zcash_client_backend" or .name == "zcash_client_sqlite" or .name == "pczt" or .name == "zcash_primitives" or .name == "zcash_protocol" or .name == "zcash_keys") | [.name, .version, (.source // "workspace"), (.checksum // "NONE"), ($node.features | sort | join(","))] | @tsv' wallet-broker/target/wal006-metadata-inventory.json | sort -u
jq -r '.packages[] | select(any(.targets[]; any(.kind[]; . == "custom-build"))) | [.name, .version, (.source // "workspace")] | @tsv' wallet-broker/target/wal006-metadata-inventory.json | sort -u
jq -r '.packages[] | select(any(.targets[]; any(.kind[]; . == "proc-macro"))) | [.name, .version, (.source // "workspace")] | @tsv' wallet-broker/target/wal006-metadata-inventory.json | sort -u
```

Record every output row without elision. The source inventory may contain only the
workspace root and crates.io registry. Each direct Zcash version/checksum must match the
architecture review and its enabled features must be explainable by the accepted direct
feature requests plus Cargo unification. Any `NONE` license for a non-workspace package,
git/path/patched source, unexpected direct version/checksum, or package target that
contradicts the architecture review is a blocker; stop without evidence/Git/fixture or
repair.

On exact success append a clearly titled supply-chain inventory addendum to
`docs/testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-01.md` containing the metadata
command/status, scratch hash/line-or-byte count, complete outputs of all six queries,
and the reviewer-facing distinction between compiled transitive capability and BitBook
authority. Update only `docs/handoff/CURRENT_TASK.md` to
`DEPENDENCY INVENTORY ADDENDUM RECORDED — REVIEW REQUIRED`.

Run `git diff --check`; stage only the evidence and `CURRENT_TASK.md`; inspect. Commit
`docs: complete WAL-006 dependency inventory` and push. Require final
`HEAD == origin/master`, clean index, and exactly six accepted ZEC tests. Leave the
ignored metadata capture in place. Run no test/tree/formatter/Node/npm/security command,
resolution, fetch, network, fixture generation, wallet/node/device action, source/test/
manifest/policy/lock edit, cleanup, package, SBOM, or other-repository action. Stop and
report exact results.
