# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 5 Compile Correction 01

Status: AUTHORIZED — EXACT ONE-LINE SOURCE REPAIR

Source actor: Sr Dev — Grok Build using Grok 4.6 High

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role policy,
Slice-5 Correction-04 Source Review 01, Format-Correction-01 Source Review 01, Green
Resume 01 Stop Review 01, the complete editable source path, and
`docs/handoff/CURRENT_TASK.md`.

## Exact starting boundary

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and
the exact accepted/frozen worktree described by the stop review. Edit only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/store.rs` | 1,904 | `08f7678c8fa5ce85d313c28e9b1ac79b42698ae6ead80c5e7e994f878d6069cd` |

Every other source path is frozen, including:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 8 | `78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce` |
| `wallet-broker/src/xmr/model.rs` | 214 | `2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb` |
| `wallet-broker/src/xmr/account.rs` | 3,375 | `5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb` |
| `wallet-broker/src/xmr/process.rs` | 1,964 | `66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d` |
| `wallet-broker/src/xmr/rpc.rs` | 2,576 | `1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33` |
| `wallet-broker/src/xmr/receiver.rs` | 871 | `4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0` |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `a815ab198559e7942d1c91ce0466a52d3b751631dba6dd80d5044682ec90cf33` |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |
| `docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md` | 59 | `20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637` |

Every test, manifest, lockfile, document, policy, workflow, fixture, generated/cache
path, repository, and other path is read-only.

## Exact repair

In `AccountStore::persist_receiver`, replace only:

```rust
        let result = (|| {
```

with:

```rust
        let result: Result<(), XmrError> = (|| {
```

This constrains the closure's existing error parameter to the error type already
returned by both fallible body operations and by `persist_receiver`. Do not alter the
closure body, transaction boundaries, commit/rollback behavior, corruption mapping,
durability synchronization, imports, APIs, tests, or any other expression.

## Prohibited actions and stop

Do not run rustfmt, Cargo, compiler, tests, Clippy, builds, binaries, Node/npm,
package-manager, policy/security, network, Git, or GitHub commands. Do not stage,
commit, push, edit governance/evidence, or invoke another actor.

Stop without editing on any parent, index, path, starting/frozen identity, or exact
source-shape mismatch. After the one-line source drop, report the resulting line count
and SHA-256, the exact repair, and prohibited-action compliance. Stop for XHigh source
inspection; Hermes remains unauthorized.
