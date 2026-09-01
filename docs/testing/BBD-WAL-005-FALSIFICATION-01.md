# BBD-WAL-005 Falsification 01 — Evidence

## Identity

- Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)
- Provider/Model: meituan/longcat-2.0:free (Nous Portal)
- Baseline commit: `0b51c73f8a875b69b3b57ad0dfb4740c5d96dc12`

## Baseline hashes (verified before and after every mutation)

```
wallet-pay/model.js       acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e
social-main.js            b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df
wallet-preload.js         3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df
```

## Falsification 1 — privacy !== "private" blocker

**File:** `wallet-pay/model.js`
**Mutation:** Changed `if (account.privacy !== 'private')` → `if (false)` at line 497, bypassing the transparent-only blocker so a `transparent_not_private` hardware account reaches later eligibility.

```diff
-  if (account.privacy !== 'private') {
+  if (false) {
     return blockedRow(account, 'CAPABILITY_MISSING', LABELS.PRIVATE_MISSING);
   }
```

**Focused command:** `node test/walletPay.node.js`
**Result:** exit 1 — test `payer view: fixed blocker precedence covers broker, request, network, privacy, device, migration, and syncing` failed at `walletPay.node.js:382:12`:
```
not ok payer view: fixed blocker precedence covers broker, request, network, privacy, device, migration, and syncing
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
true !== false
```
The transparent-only Zcash hardware account (`privacy: 'transparent_not_private'`) no longer failed at the privacy gate; `view.visible` became `true` where the gate demands `false`.

**Restoration:** `git checkout HEAD -- wallet-pay/model.js`
**Restored hash:** `acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e` ✓ matches baseline

---

## Falsification 2 — ZEC orchard MIGRATION_REQUIRED blocker

**File:** `wallet-pay/model.js`
**Mutation:** Changed `if (account.asset === 'ZEC' && account.restored_pool === 'orchard')` → `if (false)` at line 516, bypassing the restored-orchard `MIGRATION_REQUIRED` blocker.

```diff
-  if (account.asset === 'ZEC' && account.restored_pool === 'orchard') {
+  if (false) {
     return blockedRow(account, 'MIGRATION_REQUIRED', LABELS.MIGRATION_REQUIRED);
   }
```

**Focused command:** `node test/walletPay.node.js`
**Result:** exit 1 — test `payer view: fixed blocker precedence covers broker, request, network, privacy, device, migration, and syncing` failed at `walletPay.node.js:382:12`:
```
not ok payer view: fixed blocker precedence covers broker, request, network, privacy, device, migration, and syncing
AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:
true !== false
```
The orchard-restored ZEC account (`restored_pool: 'orchard'`) no longer failed at the migration gate; `view.visible` became `true` where the gate demands `false`.

**Restoration:** `git checkout HEAD -- wallet-pay/model.js`
**Restored hash:** `acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e` ✓ matches baseline

---

## Falsification 3 — leak input fee_atomic into sanitized preview

**File:** `wallet-pay/model.js`
**Mutation:** Added `fee_atomic: dataValue(descriptors, 'fee_atomic')` to the sanitized preview result returned by `sanitizePreview`, so an attacker-controlled input field leaks into the closed output.

```diff
   return {
     intent_id: intentId,
     state,
     asset,
     network,
     amount_atomic: amount,
     status_label: presentation[0],
     can_cancel: presentation[1],
     error_code: normalizeErrorCode(descriptors),
+    fee_atomic: dataValue(descriptors, 'fee_atomic'),
   };
```

**Focused command:** `node test/walletPay.node.js`
**Result:** exit 1 — test `sanitizer: committed full fixture is reduced to the exact closed snapshot and preview` failed at `walletPay.node.js:205`:
```
not ok sanitizer: committed full fixture is reduced to the exact closed snapshot and preview
AssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:
+ actual - expected
...
+     fee_atomic: '123',
      intent_id: 'ffeeddccbbaa99887766554433221100',
      network: 'zec-testnet',
      state: 'awaiting_confirm',
      status_label: 'Confirm in BitBook Wallet'
```
The preview object gained a `fee_atomic` key (`'123'`) that the closed preview schema must not contain — confirming the input-leakage canary asserts.

**Restoration:** `git checkout HEAD -- wallet-pay/model.js`
**Restored hash:** `acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e` ✓ matches baseline

---

## Falsification 4 — add forbidden wallet:intent:confirm channel

**File:** `social-main.js`
**Mutation:** Added an `ipcMain.handle('wallet:intent:confirm', ...)` registration immediately after the `wallet:payee-request:get` handler, introducing a forbidden confirmation channel outside the exact allowlist.

```diff
   ipcMain.handle('wallet:payee-request:get', walletHandler('wallet:payee-request:get', 'receiver.fresh'));
+  ipcMain.handle('wallet:intent:confirm', walletHandler('wallet:intent:confirm', 'intent.confirm'));
```

**Focused command:** `node test/electronSecurity.node.js`
**Result:** exit 1 — test `wallet IPC registers only the exact renderer channel allowlist` failed at `electronSecurity.node.js:594`:
```
not ok wallet IPC registers only the exact renderer channel allowlist
AssertionError [ERR_ASSERTION]: Expected values to be strictly deep-equal:
+ actual - expected
  [
    'wallet:accounts:list',
    'wallet:intent:begin',
    'wallet:intent:cancel',
+   'wallet:intent:confirm',
    'wallet:payee-request:get',
    'wallet:snapshot:get'
```
The forbidden channel `wallet:intent:confirm` appeared in the registered handler set, which the exact-allowlist assertion rejects.

**Restoration:** `git checkout HEAD -- social-main.js`
**Restored hash:** `b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df` ✓ matches baseline

---

## Restored verification gate

All four source files restored; every baseline hash re-verified. `git status --short` showed no source/test change. Gate results (all exit 0):

| # | Command | Result |
|---|---------|--------|
| 1 | `node test/walletPay.node.js` | 20 passed |
| 2 | `node test/electronSecurity.node.js` | 20 passed |
| 3 | `node test/securityPolicy.node.js` | 78 passed |
| 4 | `node scripts/security-policy.js` | BitBook desktop security policy checks passed |
| 5 | `npm test` | passed |
| 6 | `npm run build` | passed (all `node --check` + `bash -n` clean) |
| 7 | `npm audit --audit-level=high` | found 0 vulnerabilities |
| 8 | `gitleaks git --redact=100 --no-banner .` | no leaks found (4848 commits scanned) |
| 9 | `gitleaks dir --redact=100 --no-banner .` | no leaks found |
| 10 | `git diff --check` | clean |
| 11 | `git status --short` | clean (no source/test change) |

`gitleaks` located at `/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks` (not on `$PATH`).

## Path audit

**Authorized temporary mutation targets (mutated and restored):**
- `wallet-pay/model.js` (falsifications 1, 2, 3)
- `social-main.js` (falsification 4)
- `wallet-preload.js` (not needed; untouched)

**Authorized documentation paths (created/modified this run):**
- `docs/testing/BBD-WAL-005-FALSIFICATION-01.md` (this document — created)
- `docs/handoff/HERMES_BBD_WAL_005_FALSIFICATION_01.md` (state line → COMPLETE)

No other paths were created, edited, or deleted. No temporary mutation was staged, committed, or pushed.
