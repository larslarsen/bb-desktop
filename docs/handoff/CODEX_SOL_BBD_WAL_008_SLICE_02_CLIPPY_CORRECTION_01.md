# Codex Sol Handoff — BBD-WAL-008 Slice-02 Clippy Correction 01

Status: AUTHORIZED — ONE STORE REGION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Grok remains unavailable because the owner reports its weekly usage exhausted; this is
the documented Sol fill-in condition.

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, Slice-02 Source Review 01, Format Correction Source
Review 01, Green Resume 02 Stop Review 01, and the complete frozen `store.rs`.

The only mutable path is `wallet-broker/src/zec/store.rs`, starting at 2,848 lines and
SHA-256 `f552a17c91b5c025f102b22a10d613693c86f540483bd920e9309b056f3c1b8a`.
Freeze `hardware.rs` at `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760`
and `test_support.rs` at
`e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82`.

Replace only this region:

```rust
if let Some(prior) = &prior {
    if !hardware::decision_narrows(&prior.decision, decision) && !expansion_authorized {
        return Err(HardwareError::state_corrupt());
    }
}
```

with the Rust 1.98 Clippy-prescribed equivalent:

```rust
if let Some(prior) = &prior
    && !hardware::decision_narrows(&prior.decision, decision)
    && !expansion_authorized
{
    return Err(HardwareError::state_corrupt());
}
```

Do not alter either condition, error, or surrounding behavior. Do not run a formatter,
formatter check, Cargo, compiler, test, Clippy, native, Node, policy, Git, network,
product/device command, or another actor. Stop after editing and report the resulting
line count/hash plus confirmation that every other path stayed unchanged.
