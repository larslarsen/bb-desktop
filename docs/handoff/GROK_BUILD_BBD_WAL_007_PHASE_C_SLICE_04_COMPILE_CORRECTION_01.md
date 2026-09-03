# Grok Build Handoff — BBD-WAL-007 Phase-C Slice 4 Compile Correction 01

Status: AUTHORIZED — EXACT TWO-PATH SOURCE REPAIR

Source actor: Sr Dev — Grok Build using Grok 4.6 High

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`, the development-role policy,
Slice-4 Source Review 06, Format Correction Source Review 01, Green Resume 01 Stop
Review 01, the complete two editable files, and `docs/handoff/CURRENT_TASK.md`.

## Exact starting boundary

Edit only:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/account.rs` | 3,039 | `a014af7f26f257511b534d8dd96d74c0d87c2c15eb09e21b8f9f0ed217db7499` |
| `wallet-broker/src/xmr/test_support.rs` | 3,918 | `20bcf14c992f88733082034de0c7ea5f91ec0f1f77764a576dbef17d8847ec53` |

Freeze byte-for-byte the other five accepted source paths and frozen test:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/vault.rs` | 776 | `bb79b16f24a9abbcd266e856a37ee982bde8fcdc2da3858e75d34bdd34a4af86` |
| `wallet-broker/src/xmr.rs` | 7 | `8b382ef673e82001720727c161f365c0fa3c1401d1bdaa4d20ee2683c001559b` |
| `wallet-broker/src/xmr/store.rs` | 1,380 | `21ef2db4eaf32389809a86bcc3c0c8164ac57763ac7567c35c6f2007abb86749` |
| `wallet-broker/src/xmr/process.rs` | 1,803 | `aec5e5cc8bf93be3ee86888aa1ea5209ceed9a7ce229c3ab2fd9e0935d85688c` |
| `wallet-broker/src/xmr/rpc.rs` | 2,413 | `7f5019c9f4fb668a8f68bdf06f8ad8f20433890cef299b458f00f515b3c89965` |
| `wallet-broker/tests/xmr_account.rs` | 586 | `5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b` |

Every other path is read-only, including all other tests, governance, evidence,
manifests, lockfiles, configuration, and generated/cache state.

## Exact repair

In `wallet-broker/src/xmr/account.rs`, exactly three occurrences currently read:

```rust
                    .map_err(|_| XmrError::state_corrupt)?;
```

or the same expression at the surrounding indentation. Change only the missing call
syntax so all three read:

```rust
                    .map_err(|_| XmrError::state_corrupt())?;
```

The exact occurrences are the artifact validation in `preflight_all_existing` and the
vault/state artifact validations in `preflight_open_existing`. Do not alter the
surrounding fail-closed mapping.

In `RecordingAccountRig::install_sealed` in
`wallet-broker/src/xmr/test_support.rs`, replace only:

```rust
        let mut password = generate_wallet_password(self.manager.port_mut())
            .expect("sealed password")
            .observation()
            .encoded;
```

with exactly:

```rust
        let mut observation = generate_wallet_password(self.manager.port_mut())
            .expect("sealed password")
            .observation();
        let mut password = Zeroizing::new(std::mem::take(&mut observation.encoded));
```

This moves the allocation by replacing the observation field with an empty string,
creates no secret clone, leaves the observation's `Drop` valid, and places the owned
password under a zeroizing guard for normal return and unwind. Retain the existing
explicit `password.zeroize()` after secret construction.

Make no other source, formatting, import, warning, lint, test, or behavioral change.

## Prohibited actions and stop

Do not run rustfmt, Cargo, compiler, tests, Clippy, builds, binaries, Node/npm,
package-manager, policy/security, network, Git, or GitHub commands. Do not stage,
commit, push, edit governance/evidence, or invoke Sol/another actor.

Stop without editing on any parent, index, path, starting/frozen identity, or exact
source-shape mismatch. After the two-path source drop, report the changed paths, line
counts, SHA-256 identities, exact repair, secret-lifetime reasoning, and prohibited-
action compliance. Stop for XHigh source inspection; Hermes remains unauthorized.
