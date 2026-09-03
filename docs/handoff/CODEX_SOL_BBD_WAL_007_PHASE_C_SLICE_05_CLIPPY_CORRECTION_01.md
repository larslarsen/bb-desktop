# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 5 Clippy Correction 01

Status: AUTHORIZED — EXACT TWO-PATH SOURCE REPAIR

Source actor: Principal Dev — Codex Sol using `gpt-5.6-sol` at High

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

This continues the documented Sol fill-in because Grok Build's weekly usage remains
exhausted. Read `AGENTS.md`, `TESTING.md`, the development-role policy, Green Resume 03
Rejection 01, both complete editable paths, and `CURRENT_TASK.md` before editing.

Edit only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/receiver.rs` | 871 | `4a1fd4ddd8025ea3c64c443d2746f319e91e609526d00ef912548fbd20d833f0` |
| `wallet-broker/src/xmr/test_support.rs` | 6,019 | `a815ab198559e7942d1c91ce0466a52d3b751631dba6dd80d5044682ec90cf33` |

Every other path and accepted identity is frozen, including the corrected receiver test
and Hermes stop draft.

In `issue_fresh`, delete exactly this unreachable block and nothing else:

```rust
    if next_sequence > MAX_ISSUANCE_SEQUENCE {
        return Err(XmrError::limit());
    }
```

The preceding checked addition remains the overflow/maximum guard. In
`synthetic_subaddress`, replace only:

```rust
    address.extend(core::iter::repeat(digit).take(94));
```

with:

```rust
    address.extend(core::iter::repeat_n(digit, 94));
```

Do not add lint allowances or alter behavior, tests, imports, APIs, or other code. Do
not run rustfmt, Cargo, compiler, tests, Clippy, builds, Node/npm, policy/security,
network, Git, or GitHub. Do not stage, commit, push, or edit governance/evidence.
Stop on any boundary mismatch. Report both resulting line counts/hashes and compliance.
