# BBD-WAL-007 Slice-4 Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **REJECTED — PRODUCTION BOUNDARY AND SECRET-SAFETY CORRECTION REQUIRED**

Reviewed unstaged Grok 4.6 High drop:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr.rs` | 7 | `9bdcef746110c164726a7baafeb7d7d123c24d12b96f9187ca0a21f965e36590` |
| `wallet-broker/src/xmr/account.rs` | 1,431 | `7f96986460c23cd7ad365a585225109320c794f0fdfd90ec2426735910457c90` |
| `wallet-broker/src/xmr/store.rs` | 326 | `14bed733f62e0d9b49bc213c8a2c5a48e647c542727237d427886e7ff44cb25a` |
| `wallet-broker/src/xmr/test_support.rs` | 3,543 | `66e5ff0f03e9ed1699d72a590b9caf662d7de4bdfaa6e5b70fc49a6a3788ff04` |

The changed-path scope and starting identities are exact, `git diff --check` is clean,
and no test or integration is authorized. The drop is rejected before compilation for
the following source-level blockers:

1. There is no production account port. `AccountManager` is exercised only through
   `RecordingAccountPort`; the new code never selects/owns an accepted process, calls
   `RpcCore`/`SystemWalletRpcControl`, uses `VaultStore` plus `seal_vault`/
   `open_vault_bytes`, or opens a path-backed account-state database. Grok's own report
   confirms there is no `SystemAccountPort`. This violates the handoff's required stop
   on inability to use the existing process/RPC/vault/store boundaries.
2. `TypedWalletCall` is a public raw method-string/field-vector API. Its public fields
   can construct unreviewed methods, and its derived `Debug` prints the stored wallet
   password, mnemonic, or private view key. This directly violates the closed typed RPC
   authority and secret-log contract.
3. Secret values are repeatedly copied into ordinary `String`/`Vec<u8>` values:
   `XmrSecretV1` getters and `encode`, request fields, observations, sealed test records,
   and retained recording calls. Several have no zeroizing drop. The rig then reports
   `creation_secrets_wiped` from a Boolean while its call log still holds raw copies.
   That is not evidence of production wiping.
4. `SqliteSurface` is memory-only; its file and directory sync methods are no-ops.
   Schema verification checks only two table names, not the exact columns, constraints,
   identity row, or schema version. There is no `0600` state file, lstat/owner/mode
   enforcement, path-backed reopen, transaction/quarantine behavior, or durable sync.
5. Hostile wallet paths and file identities are values supplied by the recording port;
   no system implementation performs the required no-follow metadata, owner, mode,
   partial-set, account, or network checks. The production code therefore cannot enforce
   the path claims exposed by `inspect_paths`.
6. `seal_and_persist` marks the vault uncommitted in memory after state failure but has
   no operation that removes, quarantines, or otherwise reconciles a successfully
   persisted vault envelope. Rollback state is represented by flags rather than the
   actual WAL-004 custody store.
7. Watch-only import checks only string length and lowercase view-key hex. It does not
   establish that the primary is valid for the selected network. The live path must use
   the closed authenticated RPC/address-validation semantics while preserving the
   account-level logical call observer.
8. WAL-004 vault metadata has no `XmrTestnet` variant. Avoiding the vault API does not
   solve that conflict; it leaves testnet recovery outside authenticated custody.

The initial four-path reviewer boundary was too narrow: the accepted Slice-3 parser
already has closed result handling for the future account methods, but `RpcRequest` and
the live process control do not yet expose their phase-bound typed requests. A correct
Slice-4 production bridge therefore requires bounded changes to `rpc.rs`, `process.rs`,
and `vault.rs` in addition to the four attempted paths. That boundary correction is a
reviewer responsibility and is explicitly opened in Correction 01.

Grok 4.6 High remains the sole source actor. It may replace this unstaged drop under the
linked correction handoff. Hermes execution/integration, Slice 5, broader acceptance,
and the real offline local-Monero gate remain closed.
