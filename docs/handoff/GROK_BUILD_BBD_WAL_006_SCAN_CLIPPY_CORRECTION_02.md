# Grok Build Handoff — BBD-WAL-006 Scan Clippy Correction 02

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, Scan Truth Correction Review 01, Scan
Compile Correction Review 01, Scan Gate Clippy Review 02, and `docs/handoff/CURRENT_TASK.md`.

## Sole task

Apply exactly this bounded no-suppression correction with `apply_patch`:

1. Immediately after `ScanRequest` in `wallet-broker/src/zec/scan.rs`, define:

```rust
pub(crate) struct ScanPlan<'a> {
    pub(crate) fixture: &'a ValidatedFixture,
    pub(crate) request: ScanRequest,
    pub(crate) fault: Option<ScanFaultPort>,
}
```

2. In `scan::execute`, replace the separate `fixture`, `request`, and `fault` parameters with
   `plan: ScanPlan<'_>`. Use `plan.fixture` in its local network/manifest validation. At each of
   its two `execute_with_params` calls, replace the separate `fixture`, `request`, and `fault`
   arguments with the single `plan` argument in the same position before `metrics`.
3. In `execute_with_params`, replace the separate `fixture`, `request`, and `fault` parameters
   with `plan: ScanPlan<'_>`. As the first statement, destructure exactly:

```rust
    let ScanPlan {
        fixture,
        request,
        fault,
    } = plan;
```

   Leave all later uses and behavior unchanged.
4. Collapse the nested consistency check to exactly:

```rust
    if old_tip != old_identity.tip() && (old_tip.is_some() || !old_identity.rows.is_empty()) {
        return Err(ZecError::state_corrupt());
    }
```

5. In the sole call from `wallet-broker/src/zec/store.rs`, replace the separate `fixture`,
   `request`, and `fault` arguments with:

```rust
            scan::ScanPlan {
                fixture,
                request,
                fault,
            },
```

Do not add an `allow` attribute. Do not rename the struct or fields, move validation, reorder a
predicate, change visibility beyond the exact declaration, or make another cleanup. Do not run
rustfmt, Cargo, Rust, Clippy, tests, Node, npm, policy, a compiler, a linter, Git, network,
fixture, wallet, node, device, cleanup, or deletion command. Do not stage, commit, or push.

## Starting source identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 218 | `ae06d6a9b4448603860b55bf4eef42a1e970c414049a9304f42b436d2bf49cb4` |
| `wallet-broker/src/zec/fixture.rs` | 614 | `318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36` |
| `wallet-broker/src/zec/scan.rs` | 1,399 | `69a89bcd17a3263b8287ac256375cba40f9241b6e3cfda52567c760121ebd80f` |
| `wallet-broker/src/zec/store.rs` | 1,814 | `3b475dadffa6c1adc4c500c020c82d4e0571805c51d049f475ad4ee08ffbf894` |
| `wallet-broker/src/zec/test_support.rs` | 1,220 | `02609ae79bb8d35a5c7fb0058f0ab56fa5deb27c94cfa2ddb81347a2f38ac034` |

Require `wallet-broker/src/zec/prepare.rs` to remain absent. `zec.rs`, `fixture.rs`,
`test_support.rs`, every test, manifest, lockfile, policy, document, workflow, package, and every
other repository path are frozen.

After editing, use only read-only file inspection, `wc -l`, and `sha256sum`. Return both changed
path line counts and hashes, re-prove the three frozen source identities and absent `prepare.rs`,
enumerate every replacement, and confirm no suppression, ambiguity, or broader change remains.
The reviewer will inspect and decide whether Hermes may restart the gate.
