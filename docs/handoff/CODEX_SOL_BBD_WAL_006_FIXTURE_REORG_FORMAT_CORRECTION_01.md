# Codex Sol Handoff — BBD-WAL-006 Fixture Reorg Format Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This durable file is
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, the ticket, fixture reorg source/format reviews, and the authorized
test. Require protected `HEAD == origin/master`, clean index, exactly six untracked ZEC
tests, absent fixture output, `git diff --check`, and 928-line
`wallet-broker/tests/zec_fixture_builder.rs` at SHA-256
`4b1efec59f81761e2c713587c0a4f3e7b8c545f7b85cc35c90949c5dedbca4bc`.

Edit only that test with `apply_patch`. Replace exactly:

```text
            let duplicate_height = u32::try_from(block.height)
                .expect("duplicate compact-block height exceeded u32");
```

with:

```text
            let duplicate_height =
                u32::try_from(block.height).expect("duplicate compact-block height exceeded u32");
```

Preserve every other byte. Run no executable, formatter, Cargo, Rust, Node, npm, test,
fixture, network, Git, cleanup, wallet/node/device, or other-path action. After editing,
only read-only line/hash/diff inspection and `git diff --check` over the authorized test
are allowed. Report exact count/hash and frozen integrity. Luna owns the formatter rerun,
execution, evidence, integration, and Git.
