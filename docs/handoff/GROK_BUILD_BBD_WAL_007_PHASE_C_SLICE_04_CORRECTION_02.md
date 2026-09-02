# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Correction 02

You are **Sr Dev — Grok Build using Grok 4.6 High**. This durable handoff is
authoritative. Do not delegate this source work to Sol.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff; the rejected
Correction-01 source drop remains unstaged above that parent.

Read completely before editing: `AGENTS.md`, `TESTING.md`, the development-role policy,
`tickets/BBD-WAL-007.md`, Test-Source Review 04, Expected Red 02, Slice-3 Acceptance 01,
the Slice-3 Upstream RPC Decision, both Slice-4 source reviews, both prior Slice-4
handoffs, the complete accepted XMR and WAL-004 vault/store/process/RPC source,
`wallet-broker/tests/xmr_account.rs`, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and exact source identities

Correct the rejected Slice-4 account-custody/recovery drop. Edit only:

- `wallet-broker/src/vault.rs`;
- `wallet-broker/src/xmr.rs`;
- `wallet-broker/src/xmr/account.rs`;
- `wallet-broker/src/xmr/store.rs`;
- `wallet-broker/src/xmr/process.rs`;
- `wallet-broker/src/xmr/rpc.rs`; and
- `wallet-broker/src/xmr/test_support.rs`.

Require these exact starting identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `9bdcef746110c164726a7baafeb7d7d123c24d12b96f9187ca0a21f965e36590` |
| `wallet-broker/src/xmr/account.rs` | 2,254 | `b7f235791bfed485c92ee7dde7906d87bdf2df28f9c7faf207776609a2be3c64` |
| `wallet-broker/src/xmr/store.rs` | 702 | `11a279af1770e607cd42ad8e1680956db3db01f165f9ec04e1105754d59d860c` |
| `wallet-broker/src/xmr/process.rs` | 1,662 | `a1d6b38d1056685b1833a4612a74f29f08e5fd0ac957fe6f885126156e4a6608` |
| `wallet-broker/src/xmr/rpc.rs` | 2,324 | `7b07e261330a0b1c5b905c42b64e297f1a628420fdb073ea3312048d74b38304` |
| `wallet-broker/src/xmr/test_support.rs` | 3,772 | `004413c04179a94ba7cb84ad2219d51301830faa9ec3fde6432fef47f35abb42` |

If any identity or scope differs, stop without editing. Preserve every test, manifest,
lockfile, source outside these seven paths, policy, workflow, documentation, fixture,
and other repository byte-exact.

Do not run Cargo, rustfmt, tests, Clippy, Node/npm, policy/security commands, builds,
binaries, package managers, network, Git, GitHub, staging, commit, push, or evidence.
Correction 01 used read-only Git despite this rule; do not repeat it. Read source and
report line/hash results without Git. Leave the corrected drop unstaged for XHigh review.

## 1. Close the pinned RPC/process authority

Serialize the exact pinned v0.18.5.1 request members. `get_address` includes the fixed
empty `address_index` vector. Watch-only `generate_from_keys` includes the fixed empty
`spendkey`, fixed empty `language`, exact filename/password/address/viewkey/height, and
no optional authority expansion. Deterministic restore includes fixed empty
`seed_offset`, English language, and no multisig authority. Preserve safe upstream
optional defaults for autosave. The frozen high-level account observer continues to
show only its exact tested logical fields and must not expose these fixed internal
members as caller authority.

Make account RPC authority stateful and phase-bound on the exact authenticated owned
session. Mnemonic query succeeds at most once and only immediately after successful
fresh software creation. Create/open/generate/restore cannot be used as an account
switch or arbitrary reusable RPC façade. Keep requests closed/non-`Debug`, serialized
secret buffers zeroizing, and the accepted Digest/HTTP/node behavior intact. Narrow
new secret-bearing pool/control visibility wherever it is not required by the closed
broker account owner.

Use one exact-owned teardown. Lock and every failure must close the wallet where
applicable, stop, wait or kill, reap, close sockets, remove runtime secrets, and wipe
credentials exactly once. Do not issue one close/stop pair in the account layer and a
second pair through `stop_account`. Early RPC failure cannot skip reap/wipe; required
cleanup uncertainty compounds to `INTERNAL`.

## 2. Use honest live identity evidence

Do not place requested account/network/kind/height/filename values in a structure named
or treated as RPC observation. The live account proof is:

- the authenticated WAL-004 metadata equals exact account ID, asset XMR, selected
  network, and epoch;
- the decrypted closed secret and exact reopened durable identity agree on kind,
  restore height, account/network, and primary;
- the wallet filename is derived from the validated account ID and the process manager
  proves the exact owned network/account session;
- `get_address` independently returns the same primary and the closed
  `validate_address` call proves exact nettype, valid, non-integrated, and
  non-subaddress; and
- recovery sends the exact sealed kind/height/material through the corresponding closed
  restore or empty-spend-key generation request.

The pinned account RPC does not report restore height or wallet kind after
`open_wallet`; do not fabricate those fields. For watch-only generation, successful
closed `generate_from_keys` with an exact private view key and a forced empty spend key
is the reviewed watch-only proof. Adapt the recording faults so the frozen mutation
test changes the corresponding sealed/store/RPC evidence while driving the same
production verification state machine.

Authenticate and validate the envelope plus durable identity before starting a child
on open. Only after that gate may existing files use `open_wallet` or missing files use
recovery. On any post-recovery mismatch/error, reconcile the files created by that
attempt and leave no child/handle.

## 3. Correct operation-scoped secret custody

`SystemAccountPort` must not retain a native vault passphrase, mnemonic, private view
key, primary, or wallet-password call history for the account lifetime. Accept the
vault passphrase and watch-import material through an operation-scoped broker-native
`SecretBytes`/zeroizing boundary; wipe every local/caller-owned value according to the
existing native custody contract on success, error, cancellation, unwind, replacement,
lock, and drop. Wipe the wallet password after RPC open/create/recovery no longer needs
it. The live system port records no secret-bearing observer calls.

Recording-only copies required by the frozen tests may remain inspectable until their
test scope ends, but each must have redacted formatting and guaranteed zeroizing drop.
Wipe evidence must come from the actual nonempty secret instance. Delete the branch
that manufactures a 64-byte all-zero event when no password exists. Keep
`XmrSecretV1` and every serialized plaintext buffer non-revealing and zeroizing.

## 4. Separate strict create from strict reopen

On Linux, determine the effective UID from the followed process identity, not the
`/proc/self` symlink metadata. Gate Linux-only imports and implementation. On every
other target the public system entry compiles and returns `UNAVAILABLE` before any
filesystem or process effect.

Before vault initialization, node probing, child start, SQLite open, or wallet RPC,
validate the absolute broker root and every existing derived component with no symlink
or non-directory traversal, exact owner, and exact `0700` mode. A newly created
directory may be set to `0700`; an existing wrong-mode/owner/type entry is rejected and
never chmodded or replaced. Revalidate final wallet/keys/state/vault entries through
no-follow opened handles with exact owner, type, and `0600` mode where applicable.

Provide distinct production paths:

- create-new requires wallet/keys/state/active-vault absence, uses exclusive creation,
  and never overwrites a prior account artifact;
- open-existing requires the state and active vault to exist and never creates or
  reconstructs missing/version-zero/corrupt state; and
- an existing partial or hostile set fails before child or secret use.

Do not convert a failed schema/PRAGMA query to an initialization candidate. Avoid the
metadata-check-then-following-`Connection::open(path)` gap: bind SQLite to a no-follow,
validated regular file using the safest capability available in the current dependency
set, and revalidate the opened identity.

## 5. Make state/vault durability and rollback real

The schema verifier must prove exact version, table/column types, nullability, primary
key, every integer bound, all four independent receiver uniqueness rules, identity
constraints, and absence of unreviewed schema objects. Independently validate every
loaded identity field. Keep receiver issuance out of this slice.

After state commit, perform real file and directory synchronization, close/reopen the
production DB through the strict existing-file path, repeat schema/identity validation,
and only then report durable success. A sync/reopen/verification failure after commit
is an uncertain persisted artifact, not `state_committed = false`.

Reconcile actual artifacts after every failure edge, including a vault write that
renamed active before directory-sync failure, a committed state identity followed by
file/directory-sync failure, and files produced during failed recovery. Remove only an
exact validated newly-created artifact or move it to a unique, exclusive,
noncolliding destination; never replace an existing/hostile quarantine. Synchronize
the containing directory after rename or deletion. Any inability to establish cleanup
compounds to `INTERNAL` and leaves the account unavailable.

Parse the stored WAL-004 envelope and compare authenticated metadata to the exact
expected account/XMR/network/epoch before plaintext use. Preserve meaningful stable
error distinctions for missing/hostile storage versus failed authentication without
revealing paths or upstream text.

## 6. Repair the recording adapter without weakening tests

`sealed_record` must authenticate/decrypt the retained WAL-004 envelope before passing
plaintext to `XmrSecretV1::decode`; it may not decode ciphertext as the secret frame.
Make `XmrSecretFixture`, retained recording fields, and returned encoded fixture bytes
zeroizing or explicitly zeroized on drop. Keep all 16 frozen `xmr_account` tests and
their meaningful production-core coverage byte-exact. Memory SQLite remains only under
this recording port and cannot substantiate the live durability claims.

Preserve the valid Correction-01 work: `XmrTestnet` vault mapping, closed response
parsing, exact account secret frame, entropy and restore-height rules, stable sanitized
results/errors, capability negatives, and the seven-path system composition. Do not
begin Slice 5/viewing/receiver issuance, local-Monero execution, Electron/UI wiring, or
BBD-WAL-008/009.

If any required correction cannot be completed in these seven paths and existing
dependencies, stop and report the exact missing boundary rather than returning a
simulation or expanding scope.

Return the exact changed paths, resulting line counts and SHA-256 values, a concise map
of each Source Review 02 blocker to its correction, and every residual concern. The
reviewer will inspect source before Hermes may format, compile, test, integrate, or use
Git.
