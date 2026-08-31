# BBD-WAL-006 Address Gate Format Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `753ec19e`

Result: **EXPECTED SAFE STOP — MECHANICAL FORMAT CORRECTION REQUIRED**

Luna proved `HEAD == origin/master == 753ec19e165074e001d31057bfae4b73a65a6785`, a
clean index, and only the accepted six production paths in the worktree. The first authorized
command, the exact formatter check, exited 1 with only rustfmt layout changes. Luna stopped before
Clippy, Rust tests, Node policy, evidence, staging, commit, or push and made no source repair.

The retained formatter output identifies exactly 14 mechanical hunks in four paths:

- `wallet-broker/src/zec/address.rs`: wrap the testnet `derive_ufvk_for` arm; join the
  `seed.replace` receiver with its call; wrap the testnet `derive_only_for` arm.
- `wallet-broker/src/zec/fixture.rs`: keep the checkpoint checked-add comparison on one line.
- `wallet-broker/src/zec/store.rs`: join the `open_viewing` signature; join two transaction commit
  expressions; wrap the testnet official-wallet arm; format the binding tuple comparison, optional
  SQL row mapping, expected-column tuple, and final poison-recovering mutex expression.
- `wallet-broker/src/zec/test_support.rs`: join the state-root format expression, join the wipe
  predicate, and format the final poison-recovering mutex expression.

These are the formatter's exact replacements:

```diff
-        Network::Testnet => derive_ufvk_for(&zcash_protocol::consensus::Network::TestNetwork, guard),
+        Network::Testnet => {
+            derive_ufvk_for(&zcash_protocol::consensus::Network::TestNetwork, guard)
+        }
-        seed
-            .replace(Vec::new(), "zec-seed", observer)
+        seed.replace(Vec::new(), "zec-seed", observer)
-        Network::Testnet => derive_only_for(
-            &zcash_protocol::consensus::Network::TestNetwork,
-            guard,
-        ),
+        Network::Testnet => {
+            derive_only_for(&zcash_protocol::consensus::Network::TestNetwork, guard)
+        }
-            || self.network.checkpoint_height.checked_add(1)
-                != Some(self.network.birthday_height)
+            || self.network.checkpoint_height.checked_add(1) != Some(self.network.birthday_height)
-    pub(crate) fn open_viewing(
-        root: StateRoot,
-        account_id: AccountId,
-    ) -> Result<Self, ZecError> {
+    pub(crate) fn open_viewing(root: StateRoot, account_id: AccountId) -> Result<Self, ZecError> {
-        transaction
-            .commit()
-            .map_err(|_| ZecError::state_corrupt())
+        transaction.commit().map_err(|_| ZecError::state_corrupt())
-        Network::Testnet => initialize_wallet_for(
-            path,
-            zcash_protocol::consensus::Network::TestNetwork,
-        ),
+        Network::Testnet => {
+            initialize_wallet_for(path, zcash_protocol::consensus::Network::TestNetwork)
+        }
-    transaction
-        .commit()
-        .map_err(|_| ZecError::state_corrupt())
+    transaction.commit().map_err(|_| ZecError::state_corrupt())
-    )
-        != (
-            network.as_str(),
-            i64::from(birthday),
-            i64::from(nu6_3),
-            i64::from(confirmation),
-        )
-        || ufvk.is_empty()
+    ) != (
+        network.as_str(),
+        i64::from(birthday),
+        i64::from(nu6_3),
+        i64::from(confirmation),
+    ) || ufvk.is_empty()
-                sql: row.get::<_, Option<String>>(3)?.map(|sql| normalize_sql(&sql)),
+                sql: row
+                    .get::<_, Option<String>>(3)?
+                    .map(|sql| normalize_sql(&sql)),
-            (
-                (*name).to_owned(),
-                (*kind).to_owned(),
-                *not_null,
-                *primary,
-            )
+            ((*name).to_owned(), (*kind).to_owned(), *not_null, *primary)
-    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
+    mutex
+        .lock()
+        .unwrap_or_else(|poisoned| poisoned.into_inner())
-        let path = state_parent.join(format!(
-            "{label}-{}-{sequence}-{nonce}",
-            std::process::id()
-        ));
+        let path = state_parent.join(format!("{label}-{}-{sequence}-{nonce}", std::process::id()));
-            event.label == label
-                && event.length == length
-                && event.all_zero
-                && event.exit == exit
+            event.label == label && event.length == length && event.all_zero && event.exit == exit
-    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
+    mutex
+        .lock()
+        .unwrap_or_else(|poisoned| poisoned.into_inner())
```

The two identical transaction and mutex replacements apply at the distinct locations named above.
No semantic, test, fixture, manifest, dependency, lock, policy, evidence, or integration change is
authorized. The prior source acceptance is suspended only for exact formatting and hash review.
