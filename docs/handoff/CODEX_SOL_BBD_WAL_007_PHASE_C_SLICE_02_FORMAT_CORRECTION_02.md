# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 2 Format Correction 02

Status: AUTHORIZED — FOUR FORMATTING REGIONS ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex; XHigh review required before execution

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, Slice-2 Source Review 02,
both Slice-2 green-stop reviews, Format-Correction Source Review 01, the complete three
paths below, and `docs/handoff/CURRENT_TASK.md`.

## Exact path and byte boundary

Edit only these three paths from these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/process.rs` | 1,189 | `6e47fa9a6d07f4028331b8e9f3b859c54c2507ab78fb669856fb495d22714712` |
| `wallet-broker/src/xmr/test_support.rs` | 1,157 | `8e4720f77e60f35b8b40783e5957b2a48c0e5a1ab675bfb04fd5c1b5c11727ca` |
| `wallet-broker/tests/xmr_process.rs` | 455 | `395496959636b78f9896bec3b47e58c89b41fa70f1156c279de0a73931d617f7` |

Every other path is read-only, including `xmr.rs`, `model.rs`, evidence/governance,
manifests, lockfiles, and all other tests/source.

Make only the four mechanical Rust 1.98 rustfmt layout corrections reported by Hermes:

1. Collapse the two-line `let (mut rpc_password, mut password_origin) =
   random_secret(...)` assignment to rustfmt's one-line layout.
2. Rewrap only the `self.port.readiness(...)` call in `start_prepared` to rustfmt's
   fewer-line layout.
3. Rewrap/reorder only names within the existing XMR import block at the top of
   `test_support.rs` to rustfmt's layout.
4. Collapse only the `account_spawn_count(&format!(...)) == 1` assertion immediately
   after `poll_account_health` to rustfmt's one-line layout.

Preserve every semantic token, import target, identifier, literal, type, visibility,
expression, statement, item, attribute, test name/count, comment, and behavior. Do not
repair, refactor, simplify, rename, or optimize anything.

Do not run Cargo, rustfmt, tests, builds, binaries, Node/npm, package managers, security
tools, network, Git, or GitHub. Do not stage, commit, push, or edit evidence/governance.
Stop after the exact four-region formatting drop and report line counts/hashes,
before→after text for all four regions, and confirmation that no semantic token changed
and no prohibited action ran.
