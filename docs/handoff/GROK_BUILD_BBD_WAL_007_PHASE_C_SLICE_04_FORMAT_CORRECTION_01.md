# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Format Correction 01

Status: AUTHORIZED — FORMATTING-ONLY SOURCE EDIT

Source actor: Sr Dev — Grok Build using Grok 4.6 High

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, Slice-4 Source Review 06, Slice-4 Green Stop
Review 01, Slice-4 Formatter Diff 01, the complete five editable source paths below,
and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Manually reproduce only the Rust 1.98 mechanical layout recorded in
`docs/testing/BBD-WAL-007-SLICE-04-FORMATTER-DIFF-01.md`. Preserve every semantic
token, identifier, literal, type, visibility, expression, statement, item, attribute,
comment, error mapping, and behavior. Import movement is allowed only for the recorded
rustfmt ordering. Do not repair, refactor, simplify, rename, optimize, suppress a lint,
or make any behavioral change.

Edit only these five paths, starting from these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/account.rs` | 3,073 | `864ceeb41d74da04338b2c21f47e8be57f2f95215ff51fbf20b1bdfd4c95e61b` |
| `wallet-broker/src/xmr/store.rs` | 1,316 | `b94e26ef1d8dbcd12e275c1603806700ccb00a6efdb7c17500f3a177be11dfb8` |
| `wallet-broker/src/xmr/process.rs` | 1,808 | `b990de3e80db0a4d354ec6119fbc746b27a8989909e702b63270b6d5b43fd52a` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `5695a67aac219f36e5cd4df156f0708843084c9befb8e396f641c7c3348f966e` |

Freeze byte-for-byte:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |

Every other path is read-only, including all tests, governance, evidence, manifests,
lockfiles, configuration, and generated/cache state.

## Prohibited actions and stop

Do not run rustfmt, Cargo, compiler, tests, Clippy, builds, binaries, Node/npm,
package-manager, policy/security, network, Git, or GitHub commands. Do not stage,
commit, push, or edit governance/evidence. Do not invoke Sol or another actor.

Stop without editing if the protected parent, clean index, worktree scope, or any
starting/frozen identity differs. After the manual five-path drop, report changed
paths, line counts, SHA-256 identities, the 52 reproduced layout regions,
semantic-token preservation, and prohibited-action compliance. Stop for XHigh source
inspection; Hermes remains unauthorized until the reviewer accepts the new identities.
