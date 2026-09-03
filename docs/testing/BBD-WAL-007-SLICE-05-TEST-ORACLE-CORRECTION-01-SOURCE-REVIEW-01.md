# BBD-WAL-007 Slice-5 Test-Oracle Correction 01 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `bbea4ae5`

Result: **ACCEPTED FOR FRESH HERMES EXECUTION**

Codex Sol High changed exactly one assertion in the frozen receiver test:

```rust
assert!(!encoded.contains(&format!("\"{forbidden}\":")));
```

The resulting `wallet-broker/tests/xmr_receiver.rs` remains 588 lines and has SHA-256
`39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e`.
Reviewer inverse reconstruction of that exact line produces the prior frozen SHA-256
`d8656c533d7b2d08eb9fa8831cd70da6e1da632ed9563fc2af569cab754d3622`.
All eight accepted production identities and the frozen Hermes stop draft remain exact,
the index is clean, and `git diff --check` is clean.

The corrected oracle still rejects every named forbidden JSON key at any nesting depth
by searching for its exact compact serialized key token. It no longer rejects the
required `can_export_viewing_material` key merely because that legitimate name contains
the substring `port`. The forbidden list, expected schema, production behavior, and all
other assertions remain unchanged. Sol ran no formatter, compiler, test, Clippy, build,
product, Node/npm, package-manager, policy/security, network, Git, integration,
governance, or evidence action.

This is test-source acceptance, not execution evidence. Hermes alone may run the linked
fresh formatter, durable-replay falsification, full focused and affected-regression
sequence, warning-denied Clippy, native check, and policy checks and may integrate the
nine-path source/test drop only after exact success. Broader/final acceptance and the
real offline local-Monero gate remain unauthorized.
