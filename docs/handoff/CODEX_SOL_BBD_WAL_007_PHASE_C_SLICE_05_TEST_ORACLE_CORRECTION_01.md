# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 5 Test-Oracle Correction 01

Status: AUTHORIZED — EXACT ONE-LINE TEST REPAIR

Source actor: Principal Dev — Codex Sol using `gpt-5.6-sol` at High

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

This continues the documented Sol fill-in because Grok Build's weekly usage remains
exhausted.

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role policy,
the Slice-5 Green Resume 02 Rejection 01, the complete editable receiver test, all
production definitions of the sanitized view and capability JSON schema, and
`docs/handoff/CURRENT_TASK.md`.

## Exact starting boundary

Require `HEAD == origin/master ==` the protected governance parent, a clean index, and
the exact accepted/frozen worktree described by the rejection. Edit only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/xmr_receiver.rs` | 588 | `d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622` |

Freeze all eight accepted production source identities and the Hermes stop draft. In
particular, `wallet-broker/src/xmr/store.rs` remains 1,904 lines at
`3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4`,
`wallet-broker/src/xmr/receiver.rs` remains 871 lines at
`4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0`,
and `docs/testing/BBD-WAL-007-SLICE-05-GREEN-01.md` remains 59 lines at
`20f07f0b44dae0f006e192d91b717b541487a857d4d920047bbfd68818705637`.
Every other source, test, manifest, lockfile, document, policy, workflow, fixture,
generated/cache path, repository, and path is read-only.

## Exact repair

In `sanitized_xmr_view_has_exact_fields_and_all_spend_zec_hardware_caps_negative`,
replace only:

```rust
        assert!(!encoded.contains(forbidden));
```

with:

```rust
        assert!(!encoded.contains(&format!("\"{forbidden}\":")));
```

This retains every forbidden field name but checks for an exact serialized JSON object
key token. It therefore still rejects forbidden keys at any nesting depth while no
longer rejecting the required `can_export_viewing_material` capability merely because
it contains the letters `port`. Do not change the forbidden list, expected schema,
production code, another assertion, or any other expression.

## Prohibited actions and stop

Do not run rustfmt, Cargo, compiler, tests, Clippy, builds, binaries, Node/npm,
package-manager, policy/security, network, Git, or GitHub commands. Do not stage,
commit, push, edit governance/evidence, or invoke another actor.

Stop without editing on any parent, index, path, starting/frozen identity, or exact
source-shape mismatch. After the one-line test drop, report the resulting line count
and SHA-256, the exact oracle repair, and prohibited-action compliance. Stop for XHigh
source inspection; Hermes remains unauthorized.
