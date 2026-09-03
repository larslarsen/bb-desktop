# BBD-WAL-007 Slice-4 Source Review 06

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Governance parent: `e90cbb2d`

Result: **ACCEPTED FOR HERMES FOCUSED GREEN**

No formatter, compiler, test, build, binary, Node/npm, policy/security,
package-manager, staging, commit, or push command was run by the reviewer while
reviewing the source drop. `HEAD == origin/master == e90cbb2d`, the index is clean,
the worktree contains exactly the accepted seven-path Slice-4 source drop, and
`git diff --check` is clean. The frozen 586-line account test remains byte-exact at
SHA-256 `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b`.

## Accepted identities

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/account.rs` | 3,073 | `864ceeb41d74da04338b2c21f47e8be57f2f95215ff51fbf20b1bdfd4c95e61b` |
| `wallet-broker/src/xmr/store.rs` | 1,316 | `b94e26ef1d8dbcd12e275c1603806700ccb00a6efdb7c17500f3a177be11dfb8` |
| `wallet-broker/src/xmr/process.rs` | 1,808 | `b990de3e80db0a4d354ec6119fbc746b27a8989909e702b63270b6d5b43fd52a` |
| `wallet-broker/src/xmr/rpc.rs` | 2,418 | `4f52baf1532a374dda748ec03a4a0a4311fd970b441f258f576c26030bb5ac14` |
| `wallet-broker/src/xmr/test_support.rs` | 3,924 | `5695a67aac219f36e5cd4df156f0708843084c9befb8e396f641c7c3348f966e` |

## Accepted semantics

The account core is crate-internal behind the sanitized `SystemAccount` composition.
It validates the account/network/kind and all existing derived storage before node,
directory, child, vault, SQLite, or wallet-RPC effects. Create-new artifacts use exact
per-attempt vault/state/wallet/keys identities; recovery owns only wallet material it
created, and existing-wallet open owns none. A successful operation retires its ledger
once at the returned-success boundary. Error and unwind run the same child teardown,
secret wipe, and exact artifact reconciliation, while any uncertainty latches the
account unavailable. Lock teardown failure now follows that same fail-closed rule.

Active vault and state creation are exclusive. The live port marks provisional
uncertainty immediately after `create_new`, publishes device/inode identity before any
write or SQLite work, and retains uncertainty if identity derivation fails. Wallet and
keys are both inspected after every create/generate/restore outcome, every proven inode
is retained, and success requires a complete owner-`0600` pair in the unchanged private
directory. Hard-link quarantine validates source and destination identities, never
unlinks an unknown destination, rechecks pathname identity immediately before unlink,
and synchronizes the validated parent. An unknowable cleanup result becomes compound
`INTERNAL`, not a guessed deletion.

Open authenticates the WAL-004 vault and validates the exact stored schema/identity
before child start. Sealed and stored primaries must be valid UTF-8/base58 syntax and
match before recovery or open RPC; the independent closed RPC network proof remains.
The state surface retains no-follow state/account-directory handles, compares exact
identities across the accepted rusqlite `SQLITE_OPEN_NOFOLLOW` path open, uses FULL
sync, synchronizes the retained directory, strictly reopens, and revalidates the exact
two-table schema and complete identity row. Existing process-derived directories require
the effective owner, no-follow identity, exact type, and `0700`; new directory entries
have their containing parents synchronized.

Native import/passphrase, mnemonic/view-key, primary, wallet-password, vault plaintext,
RPC, and recording buffers retain zeroizing/redacted ownership. Wallet-password entropy
is held in a `Zeroizing<[u8; 32]>`, borrowed during encoding rather than copied into an
array iterator, and wiped on success, error, or unwind. The unused combined state
constructor and useless signed-64 maximum comparison are gone without lint suppression.

The same-user replacement interval around safe-Rust pathname operations remains the
ticket's disclosed local-malware residual. If a quarantine link cannot be opened and
identified, it may remain while the account is permanently unavailable; deleting an
unknown entry would be less safe. The retained handle plus before/after identity checks
is still the accepted rusqlite 0.37 boundary under repository-wide
`forbid(unsafe_code)`. This review does not claim an unsafe from-fd solution, immunity
from same-user malware, or successful compilation/execution.

Hermes must now prove formatter cleanliness without mutation, the exact lock-teardown
falsification, focused account green, the affected vault and XMR regressions, focused
Clippy with warnings denied, native checks, and policy checks before integration. Slice
5, broader/final acceptance, and the real offline local-Monero gate remain unauthorized.
