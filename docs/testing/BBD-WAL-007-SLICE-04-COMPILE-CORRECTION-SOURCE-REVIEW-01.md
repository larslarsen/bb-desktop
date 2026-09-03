# BBD-WAL-007 Slice-4 Compile Correction Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `c2bf9f0c`

Result: **ACCEPTED FOR HERMES GREEN RESUME 02**

Grok 4.6 High made only the exact two-path correction authorized after the first
compile stop. No formatter, Cargo, compiler, test, Clippy, build, binary, Node/npm,
policy/security, network, Git, GitHub, evidence, governance, Sol, or other actor was
used.

Reviewer inspection confirms:

- the three artifact-validation closures in `account.rs` now call
  `XmrError::state_corrupt()` and preserve the surrounding fail-closed mapping;
- `RecordingAccountRig::install_sealed` takes the password allocation from a mutable
  `WalletPasswordObservation` via `std::mem::take`, leaving an empty valid field for
  its `Drop` implementation;
- the taken password is immediately owned by `Zeroizing<String>`, creates no clone,
  stays covered on unwind, and retains the explicit wipe after secret construction;
- no other source shape or accepted/frozen identity changed; and
- `git diff --check` is clean.

Accepted identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,039 | `c3ae5b07174a9d1732ab3ec7ee2628f8a7f2c394d9af875026fa57d594d311ee` |
| `wallet-broker/src/xmr/store.rs` | 1,380 | `21ef2db4eaf32389809a86bcc3c0c8164ac57763ac7567c35c6f2007abb86749` |
| `wallet-broker/src/xmr/process.rs` | 1,803 | `aec5e5cc8bf93be3ee86888aa1ea5209ceed9a7ce229c3ab2fd9e0935d85688c` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `7f5019c9f4fb668a8f68bdf06f8ad8f20433890cef299b458f00f515b3c89965` |
| `wallet-broker/src/xmr/test_support.rs` | 3,918 | `b359256394de4dcb2cb0788aa558c381c8f6e1a5733aa52a462b41b7b7018bb4` |

The frozen 586-line account test remains byte-exact at SHA-256
`5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`.
This review does not claim formatter, compiler, test, Clippy, Node, policy, or runtime
success. Hermes must prove those gates under the new identities.
