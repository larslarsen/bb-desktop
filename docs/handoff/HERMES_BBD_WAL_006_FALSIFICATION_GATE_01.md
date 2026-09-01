# Hermes Handoff — BBD-WAL-006 Falsification Gate 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Execute exactly five
reviewer-approved, Sol-authored temporary source falsifications, restoring the accepted source
byte-for-byte after each one. No falsification byte may be committed.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, this handoff, both changed
source paths, the three focused test files, and `CURRENT_TASK.md`.

## Preconditions and restoration contract

Record Hermes version/provider/model. Require `HEAD == origin/master`, clean worktree/index,
clean `git diff --check`, ext4 Cargo work directories, and these exact identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/zec/prepare.rs` | 964 | `3cb1096107386bfcd7f30169ade545ec8a7073875dc1d8e631dd0d5104d7c08e` |
| `wallet-broker/src/zec/scan.rs` | 1,661 | `54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad` |
| `wallet-broker/tests/zec_prepare.rs` | 416 | `c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa` |
| `wallet-broker/tests/zec_scan.rs` | 325 | `87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f` |
| `wallet-broker/tests/zec_hygiene.rs` | 375 | `aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6` |

For each falsification: use `apply_patch` for the exact forward patch; verify only its named source
path is dirty; run its one exact command once; require exit 101, exactly one failed named test and
zero passed; then apply the exact reverse patch even if the test result mismatches. Verify the
source identity above is restored, `git diff --check` is clean, and the worktree/index are clean
before continuing. If a patch does not apply exactly or restoration fails, stop after exhausting
safe exact restoration; do not use checkout/reset/clean/copy/temp-file replacement.

### Falsification 1 — accept transparent/Sapling UA composition

Forward patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
-    if address.has_transparent() {
-        return Err(ZecError::transparent_downgrade());
-    }
-    if !address.has_orchard() || address.has_sapling() || !address.unknown().is_empty() {
+    if !address.has_orchard() || !address.unknown().is_empty() {
         return Err(ZecError::protocol_incompatible());
     }
*** End Patch
```

Command:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare receiver_network_and_composition_reject_downgrade_without_fallback -- --exact
```

Require failure in `receiver_network_and_composition_reject_downgrade_without_fallback`, proving an
Orchard+P2PKH or Orchard+Sapling receiver reached spend access or preparation.

Reverse patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
-    if !address.has_orchard() || !address.unknown().is_empty() {
+    if address.has_transparent() {
+        return Err(ZecError::transparent_downgrade());
+    }
+    if !address.has_orchard() || address.has_sapling() || !address.unknown().is_empty() {
         return Err(ZecError::protocol_incompatible());
     }
*** End Patch
```

### Falsification 2 — bypass previous-hash continuity

Forward patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/scan.rs
@@
-            || (index > 0
-                && (row.height
-                    != rows[index - 1]
-                        .height
-                        .checked_add(1)
-                        .ok_or_else(ZecError::limit)?
-                    || block.prev_hash != blocks[index - 1].hash))
+            || (index > 0
+                && row.height
+                    != rows[index - 1]
+                        .height
+                        .checked_add(1)
+                        .ok_or_else(ZecError::limit)?)
@@
-    let states = derive_chain_states(
-        &params_value,
-        fixture.manifest.network.checkpoint_height,
-        &intended_identity.blocks,
-    )?;
+    let states = match derive_chain_states(
+        &params_value,
+        fixture.manifest.network.checkpoint_height,
+        &intended_identity.blocks,
+    ) {
+        Ok(states) => states,
+        Err(error) if error.code() == "PROTOCOL_INCOMPATIBLE" => {
+            remove_candidate(&candidate)?;
+            return Ok(());
+        }
+        Err(error) => return Err(error),
+    };
*** End Patch
```

Command:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_scan malformed_discontinuous_wrong_branch_and_impossible_tree_fail_without_tip_advance -- --exact
```

Require failure in `malformed_discontinuous_wrong_branch_and_impossible_tree_fail_without_tip_advance`
when `wrong-previous-hash` yields apparent success.

Reverse patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/scan.rs
@@
-            || (index > 0
-                && row.height
-                    != rows[index - 1]
-                        .height
-                        .checked_add(1)
-                        .ok_or_else(ZecError::limit)?)
+            || (index > 0
+                && (row.height
+                    != rows[index - 1]
+                        .height
+                        .checked_add(1)
+                        .ok_or_else(ZecError::limit)?
+                    || block.prev_hash != blocks[index - 1].hash))
@@
-    let states = match derive_chain_states(
+    let states = derive_chain_states(
         &params_value,
         fixture.manifest.network.checkpoint_height,
         &intended_identity.blocks,
-    ) {
-        Ok(states) => states,
-        Err(error) if error.code() == "PROTOCOL_INCOMPATIBLE" => {
-            remove_candidate(&candidate)?;
-            return Ok(());
-        }
-        Err(error) => return Err(error),
-    };
+    )?;
*** End Patch
```

### Falsification 3 — mark Orchard value spendable

Forward patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
         if self.orchard >= required
             || self
                 .orchard
                 .checked_add(self.ironwood_spendable)
                 .is_some_and(|value| value >= required)
         {
-            return Err(ZecError::migration_required());
+            return Ok(());
         }
*** End Patch
```

Command:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare pool_outcome_table_never_substitutes_total_for_ironwood_spendable -- --exact
```

Require failure in `pool_outcome_table_never_substitutes_total_for_ironwood_spendable` when an
Orchard-funded row no longer returns `MIGRATION_REQUIRED`.

Reverse patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
         if self.orchard >= required
             || self
                 .orchard
                 .checked_add(self.ironwood_spendable)
                 .is_some_and(|value| value >= required)
         {
-            return Ok(());
+            return Err(ZecError::migration_required());
         }
*** End Patch
```

### Falsification 4 — report a non-v6 artifact

Forward patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
-        tx_version: "6".to_owned(),
+        tx_version: "5".to_owned(),
*** End Patch
```

Command:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_prepare sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt -- --exact
```

Require failure in `sufficient_confirmed_ironwood_prepares_sanitized_v6_handle_and_decoded_pczt`
at the exact `prepared.tx_version == "6"` assertion, observing `"5"`.

Reverse patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
-        tx_version: "5".to_owned(),
+        tx_version: "6".to_owned(),
*** End Patch
```

### Falsification 5 — retain prepared handle across lock

Forward patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
 fn invalidate_inner(inner: &mut PrepareInner, edge: HandleInvalidation) {
+    if edge == HandleInvalidation::Lock {
+        return;
+    }
     let mut observer = PrepareObserver {
*** End Patch
```

Command:

```text
TMPDIR="$PWD/wallet-broker/target/wal006-tmp" CARGO_TARGET_DIR="$PWD/wallet-broker/target/wal006-cargo" /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test zec_hygiene every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state -- --exact
```

Require failure in `every_named_lifecycle_edge_invalidates_handle_and_wipes_prepared_state` for
`edge Lock`, proving the retained handle/raw state is detected.

Reverse patch:

```diff
*** Begin Patch
*** Update File: /home/lars/OpenBazaar/bb-desktop/wallet-broker/src/zec/prepare.rs
@@
 fn invalidate_inner(inner: &mut PrepareInner, edge: HandleInvalidation) {
-    if edge == HandleInvalidation::Lock {
-        return;
-    }
     let mut observer = PrepareObserver {
*** End Patch
```

## Evidence and integration

Only after all five expected failures and five exact restorations, use `apply_patch` to create
`docs/testing/BBD-WAL-006-FALSIFICATION-GATE-01.md` and update only the leading current-task block.
Record each mutation, exact command, failure location/result, restoration identity, no committed
mutation, final clean state, and Hermes identity.

Stage only those two documentation paths. Commit exactly `test: record WAL-006 falsification gate`,
push `master`, and prove clean worktree/index with `HEAD == origin/master`.

Do not run any other test/build/policy/audit/scanner command, edit accepted source beyond the exact
temporary patches, stage source, use Git restoration/cleanup, amend/rebase/merge/force-push, access
network/device/wallet/node, or delete anything. Any mismatch stops after exact restoration.
