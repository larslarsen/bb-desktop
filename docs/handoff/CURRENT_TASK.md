# Current Task

BBD-WAL-014 native Zcash Receive and explicit unsynced balance status are COMPLETE.
Source aad0bdd003af823dd6cbfa55994e4692555b3d43 and integration evidence
648050628bf2916ca7f192e7585ce169e3a76833 are pushed to origin/master.

Owner reconfirmed the original architecture includes a basic Zcash wallet inside the
broker. Wallet → Manage accounts → select/unlock → Receive now produces a durable,
account-bound Zcash TESTNET Orchard-only Unified Address and offers Copy address.
Balance displays “Balance unavailable — not synced”; live chain synchronization is
not connected. Existing encrypted accounts/vault format are compatible. No user
profile was read or changed by tests. Quit and reopen BitBook to load the rebuilt broker.

Accepted 82 Rust tests and 28 distinct JS groups, including actual-manager native UI
pointer/copy/persistence proof and native window smoke on virtual plus actual desktop.
Both high-value falsifications detected and exact source restored. Production Clippy
and pinned directory/committed Gitleaks passed. Reviewer checked both host screenshots,
all ten integrated source/test/execution blobs and all pins. All actors exited.
No background work or further source authoring is authorized.

Final staged Linux x64 binary SHA256:
5ed35fe09b1172b2639faa6170621fca60fe99825636398b78fe1bd382509ae4.
WAL013 X11/XWayland and fixed Mesa software GLX rendering settings remain intact.
The account window is initially hidden and opens through Wallet → Manage accounts.

See tickets/BBD-WAL-014.md and docs/testing/BBD-WAL-014-RECEIVE-GREEN-03.md / INTEGRATION-01.md.
Six inherited security-policy failures remain release blockers; seven pre-existing
test-helper Clippy warnings are also recorded. No installer/mainnet/payment/live-balance
claim. Four unrelated npm/policy edits and two historical evidence drafts are preserved.
