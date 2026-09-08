# BBD-WAL-009 Phase-A3 Sol Usage-Stop Rejection 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance parent: `fc49d2db`

Sol session: `01a06a71-38da-7b33-ab07-7edf4420e6be`

Result: **PARTIAL SOURCE REJECTED — GROK CORRECTION REQUIRED**

Codex Sol `gpt-5.6-sol` at High hit its account usage limit and exited `1` before a
completed handoff, final self-audit, or identity report. Its transcript shows file
changes only in the seven authorized source paths and no formatter, Cargo, compiler,
test, Clippy, audit, scanner, dependency, product, Git, network, wallet/node,
hardware/device, or actor command. The incomplete worktree identities are:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec.rs` | 274 | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` |
| `wallet-broker/src/zec/prepare.rs` | 1,171 | `1a97332e9c4e714833dcda1f487d0da20c371f21ee5375fb4b100eaa0947f642` |
| `wallet-broker/src/zec/spend.rs` | 530 | `0b9a7e4c30afb85558f480d7c177f2732e53755b260ad78c8ee51763948c03cb` |
| `wallet-broker/src/zec/store.rs` | 2,872 | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` |
| `wallet-broker/src/zec/test_support.rs` | 4,136 | `eac4d2ec63a140267bb23706fc6fc675a0fb820106ffebec042bc156f5dc2627` |
| `wallet-broker/src/native.rs` | 435 | `7f64822feae9eb3cb066ac834a769afff2b2dac9644542f4cb94f543ba3df029` |
| `wallet-broker/src/native_ui.rs` | 178 | `a8a8d2486e453ccd90d5d868a45e2778fb93896e4e0275277611019c81462ae2` |

The partial source is not merely unfinished. Static review finds authority-bearing
test-shaped shortcuts that violate the handoff and cannot be accepted:

- prerequisite, mutation, terminal-exit, fault, hardware-success, capability, and
  forbidden-operation helpers manufacture expected codes, flags, counters, or wipe
  observations without exercising the claimed state and resource transitions;
- mutation tests run the unmodified pipeline and then return the expected error rather
  than proving the independent verifier detected a changed artifact;
- cryptographic validity is published as literal `true` fields, while the supposed
  independent verifier performs only structural decoding and incorrectly claims that
  `TransactionExtractor` verified proofs and signatures;
- the verified-effects structure retains raw transaction bytes and is clone-shaped,
  contrary to the bounded redacted result and prompt-destruction boundary;
- the native UI does not render or bind a review; it places fields in an unused tuple
  and accepts a pre-set boolean; and
- synthetic canary, log, diagnostic, and cleanup methods are detached from the actual
  secret owners and failure paths they claim to observe.

Two additional modified paths, `package.json` and `package-lock.json`, are present in
the shared worktree. They do not appear in Sol's command or file-change transcript and
are treated as unrelated protected concurrent work. They must not be edited, restored,
staged, or attributed to WAL-009.

No portion of the partial source is accepted and no execution is authorized. Grok
Build High may correct or replace it only under the linked seven-path handoff.
