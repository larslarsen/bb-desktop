# BBD-WAL-009 Phase-A2 Expected-Red Resume 01 Stop Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance parent and execution HEAD: `efd1210dbbf6fbf942b5b00ce2e56def7027703a`

Result: **EXPECTED RED VALID — CONSERVATIVE STOP ACCEPTED**

Hermes `v0.18.2` using `meituan/longcat-2.0:free` passed preflight and the exact Rust
1.98 formatter check with exit `0` and no mutation. The exact focused test then exited
`101` before any test executed. Hermes stopped without editing, staging, committing,
pushing, rerunning, or running another gate because it classified one diagnostic as
outside the handoff.

Reviewer inspection resolves that classification: the absent
`FrozenFixture::expected_destination_receiver_bytes` method is deliberately part of
the new typed BBD-WAL-009 `zec::test_support` contract. It is not a misspelling or a
misuse of a promised existing API. Its purpose is to give the independent decoded
effects oracle exact receiver bytes instead of conflating them with the printable
ZIP-316 address. The source review accepted this new method requirement.

The complete expected-red diagnostic set is therefore in scope:

- absent `src/zec/spend.rs` from the permitted production-inventory assertion;
- unresolved new sign/verify types imported from `zec::test_support`; and
- absent new `FrozenFixture::expected_destination_receiver_bytes` test-support method.

There was no dependency, lockfile, syntax, existing-source, fixture, network, or
unrelated failure. The frozen source identities remain:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 121 | `71e0135dbc2a6086ee6658e173718d2dd8c608a1f6369f732a8979d942ef6450` |
| `wallet-broker/tests/zec_sign_verify.rs` | 1,115 | `80a3a342392f53553950fabae710f2e95082d357c281c6de23b54aedbc85eccd` |

The executed red is accepted without rerun. Hermes alone may transcribe the completed
run into the evidence record and integrate the exact four paths under the linked
documentation/integration handoff. No further execution, source repair, production
source, broadcast/network, real hardware, mainnet, or Monero work is authorized.
