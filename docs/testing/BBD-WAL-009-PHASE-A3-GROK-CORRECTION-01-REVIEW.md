# WAL-009 Phase A3 Grok Correction 01 Review

Result: REJECTED FOR EXECUTION — source preserved for bounded follow-up.

Grok completed session 96181 with exit 0 and reported no test/build/formatter/Git
execution. This review is static inspection; it is not a complete transcript audit or
an exhaustive assessment. No source integration is authorized.

Blocking findings in the resulting source:

1. `zec/test_support.rs:1243` fabricates wipe evidence. `wipe_class` allocates a fresh
   32-byte `0x5a` buffer and wipes it under each sensitive-class label. That does not
   observe destruction of the actual seed, spending authority, PCZT, or proof workspace.
   `touch_canaries` also increments counters independently of operational use.
2. `zec/spend.rs:414` initializes both `actual` and `expected` from the same prepared
   inspection. The verifier compares those copies for receiver, amount, memo, fee,
   request, and intent; it does not recover these effects from the extracted shielded
   transaction. Most mutation hooks alter the expected copy. These tests cannot prove
   detection of changed transaction effects. A real post-sign effect check needs
   independent output recovery or equivalent cryptographic binding, with actual internal
   change ownership verified against the account.
3. `native_ui.rs:112` creates a fresh `egui::Context` and runs one default-input frame.
   This frame has no native window/input integration, so the confirmation button cannot
   receive the owner's click. It does not implement the required production surface.
4. `zec/spend.rs:435` and the subsequent verification call use `?` while transaction
   bytes are held in a plain Vec. Error exits bypass the explicit wipe at the end.
   Cleanup must be owned by an RAII zeroizing buffer on every return/unwind path.
5. `zec/spend.rs:534` names `orchard::circuit::VerifyingKey`, but orchard is not a
   direct dependency or local module. A transitive Cargo.lock entry does not make the
   crate available here. The actor disclosed this unresolved API issue; no compilation
   was performed. Resolve the dependency/API boundary before execution authorization.

There are useful changes, including explicit signature/proof verification calls and
removal of transaction bytes from VerifiedEffects. These do not close the findings.

Verified resulting SHA-256 identities:

| Source | SHA-256 |
| --- | --- |
| `zec.rs` | `045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b` |
| `zec/prepare.rs` | `44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07` |
| `zec/spend.rs` | `dc2773cdf3bcf87f596b8acb3a89f9ea6d0c7479ca5a330ca4374b9aa065a5f9` |
| `zec/store.rs` | `531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90` |
| `zec/test_support.rs` | `21489e5cda159d670fbdd3b96b196bb0e40f8227fa8770a80b6bd7a3e1c23d83` |
| `native.rs` | `992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc` |
| `native_ui.rs` | `04882d21ca5e4ae21f61aa101b1f9a9610baf7eb1a736effcbb44f2b103e0735` |

The accepted Rust manifest/test and unrelated package files retain their frozen hashes.
All source is preserved. Correction 01 is closed; no actor is currently authorized.
The next source authorization should isolate one blocker and specify a narrow source
read set rather than repeat the broad seven-path rewrite and historical-document load.
