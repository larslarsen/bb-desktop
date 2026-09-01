# Codex Sol Handoff — BBD-WAL-006 Prepare Format Correction 03

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. Apply only the four captured
rustfmt hunks below.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Use `apply_patch` only. Edit only `wallet-broker/src/zec/prepare.rs` and
`wallet-broker/src/zec/test_support.rs`. Apply these exact replacements:

```diff
-        if &artifact.binding != binding || artifact.binding.session_id.as_str() != session.as_str() {
+        if &artifact.binding != binding || artifact.binding.session_id.as_str() != session.as_str()
+        {
```

```diff
-        || bytes
-            .iter()
-            .enumerate()
-            .any(|(index, byte)| !matches!(index, 4 | 7 | 10 | 13 | 16 | 19) && !byte.is_ascii_digit())
+        || bytes.iter().enumerate().any(|(index, byte)| {
+            !matches!(index, 4 | 7 | 10 | 13 | 16 | 19) && !byte.is_ascii_digit()
+        })
```

```diff
-        AddressAccount::open_viewing_with_network(root.inner, account_id, network).map(
-            |inner| Self {
+        AddressAccount::open_viewing_with_network(root.inner, account_id, network).map(|inner| {
+            Self {
                 inner,
                 prepare: PrepareState::viewing_only(),
-            },
-        )
+            }
+        })
```

```diff
-    pub fn inspect_prepared_for_test(
-        &self,
-        handle: &str,
-    ) -> Result<PreparedInspection, ZecError> {
-        self.prepare.inspection(handle).map(PreparedInspection::from)
+    pub fn inspect_prepared_for_test(&self, handle: &str) -> Result<PreparedInspection, ZecError> {
+        self.prepare
+            .inspection(handle)
+            .map(PreparedInspection::from)
     }
```

Do not inspect or change another site. Preserve `zec.rs` and `store.rs` byte-exact. Do not run any
formatter, compiler, Cargo, test, Clippy, Node, Git, network, or other command. Do not edit another
path, stage, commit, or push.

Return all four source line counts/SHA-256 and confirm only these four captured formatter hunks
changed. Hermes remains the sole execution/evidence/integration/Git actor.
