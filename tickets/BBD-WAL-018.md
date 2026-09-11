# WAL-018 — Reproduce scanner failure using isolated viewing-cache copy

Baseline acf53cf26c41992f198bdb6a4cde5fb04ccffb8b. User again reports Sync failed
after alternate-server guidance. Minimal readonly checkpoint query remains4308219/
4338623. Prior server response diagnosis did not prove local scan completion.

Retain principal Sol core actor (prior Grok stopped without usable drop; established
senior corrective escalation). Source-only author ONLY temporary
wallet-broker/examples/wal018_diagnostic.rs; no execution/format/Git. Hardcoded owned
wallet-broker/target/wal018-diagnostic/viewing-copy.sqlite3 only; captured public frames
from target/wal017-diagnostic-zaino. Decode actual upstream TreeState/CompactBlock,
print only public heights/hash-equality/tree sizes and fixed stage labels. Open owned
clone with WalletDb TestNetwork, actual upstream update_chain_tip+scan_cached_blocks
for4308220..4308319 in transaction then unconditional rollback. Print allowlisted
structural ScanError and SQLite/tree error fields only; no raw private key/address/
amount/account IDs, CorruptedData/BadAccountData/SQL text or arbitrary error Debug.
No vault, seed, user profile or network access from the diagnostic executable.

Hermes alone may create target/wal018-diagnostic0700 and viewing-copy.sqlite30600
using SQLite backup from the single original viewing-cache database URI mode=ro.
Never mutate original user profile or read vault/seed; clone stays private/ignored.
Then rustup run1.98.0 cargo run --manifest-path wallet-broker/Cargo.toml --locked
--offline --no-default-features --features native-ui --example wal018_diagnostic;
existing disk-backed target only, no dependency changes,120second runtime bound.
Record actual Hermes version/provider/model/session and safe output. Remove exact
owned clone/temporary source after diagnosis. No blanket cleanup or disk-cache deletion.
Production/test edit contract follows concrete result, tests first and falsification.
Preserve unrelated npm/policy working changes and historical drafts. No acceptance
of a sample RPC batch as proof of end-to-end user-wallet sync.

If offline replay succeeds, diagnostic may additionally call the existing public
probe_live_transport_for_test on the fixed Zaino Testnet endpoint with default trust
(None CA),15second deadline and fresh cancellation token. This uses actual production
Tonic transport to read public tip metadata/tree/one compact block only; no wallet
material or clone data enters RPCs. Print only error code or tip/block count. This
narrow exception supersedes the earlier blanket no-network diagnostic clause.

## Full engine follow-up authorization

Diagnostic01 proved isolated scanner batch and native Tonic probe both successful.
Authorize the same Sol actor to append ONE temporary helper to
wallet-broker/src/zec/test_support.rs and replace temporary example with a caller.
Helper hardcodes target/wal018-diagnostic/root/network/account/live.sqlite3; readonly
extract account_id/ufvk from that private clone internally, never print either;
construct actual LivePrepared Testnet/current uid, call actual inspect_live then
TonicLiveSource::connect and run_live_sync_with_hooks with fixed Zaino endpoint.
Report fixed stage/code and public scanned/target heights only, no amounts, arbitrary
Debug or private fields. Print progress every1000blocks and final current/failed.
Run full sync only on clone; clone commits are explicitly allowed. Use bounded
cancellation after480seconds with a joined timer which wakes promptly on completion.
No user-profile access from Rust, no key derivation, no transfer/sign/broadcast.
Hermes may move the existing private clone into the exact new0700 hierarchy above;
no overwrite. Build/run same Cargo command,600seconds process bound. Restore exact
original test_support.rs after diagnostic; never stage helper or private clone.
These are diagnostic source additions, not product behavior changes; deterministic
regression tests remain required before any actual production correction.

## Test-only default-server correction

Independent source-only work may add ONE pointer-flow regression in
wallet-broker/tests/account_native_ui.rs: entering Sync from the account list must
show and submit https://zaino.testnet.unsafe.zec.rocks:443 by default; after Back and
re-entering it must not silently revert to the known inconsistent original server.
No custom endpoint behavior changes. Existing suggested-endpoint test constant stays
unchanged until production is authorized. Test asserts literal external contract and
actual FakePort start-call endpoint through pointer clicks; no network.
Targeted RED/GREEN command: rustup run1.98.0 cargo test --manifest-path
wallet-broker/Cargo.toml --locked --offline --no-default-features --features native-ui
--test account_native_ui wal018_default -- --exact (use actual full test name in spec).
Expected RED is runtime visible/submitted endpoint mismatch, not compilation error.
Production default change remains gated on full clone-engine result. Broader acceptance
would be complete account_native_ui test binary plus lib/bin production Clippy, existing
policy ratchet and pinned secrets classification, followed by native build staging.
Falsification: restore original endpoint literal only in production; new regression
must fail at runtime; restore exact accepted source bytes afterward.

If the full engine reaches its intentional480second timer while making committed
progress, authorize one further invocation of the same frozen diagnostic against the
same clone, without relocation or another original-cache backup. Each invocation
retains its600second process bound and480second cancellation. Record timer cancellation
as an incomplete diagnostic, never a product failure or sync completion. Production
correction remains gated on actual Current output with equal scanned/target heights.

Cleanup is exactly WAL018_RESTORE.py then WAL018_CLEANUP.py. In addition to the
private clone and temporary source, remove the four explicitly named WAL018 diagnostic
example binaries/depfiles in CLEANUP.py after the diagnostic exits, to recover their
build space. No other build artifacts or caches may be deleted.

## Bounded production correction (only after DIAGNOSTIC03 Current acceptance)

Authorize existing Sol test actor to change exactly the default endpoint string in
wallet-broker/src/account_ui.rs to https://zaino.testnet.unsafe.zec.rocks:443 and the
existing SUGGESTED_ENDPOINT test constant in wallet-broker/tests/account_native_ui.rs
to match. Preserve the independent literal assertions in the new WAL018 regression.
No fallback, trust, validation, account, cache, or key-handling changes are authorized.
Hermes restores/removes temporary diagnostic material first, then executes the complete
account_native_ui binary, falsifies only the production endpoint literal, confirms
runtime RED, restores exact bytes and reruns the new regression, production lib/bin
Clippy with -Dwarnings, node scripts/build-wallet-broker.js, both existing policy
commands with the inherited six-failure baseline, and pinned directory secrets scan.
No dependency change; reuse unchanged dependency-audit evidence. No GUI lifecycle or
packaging behavior changed, so prior native window checks remain applicable; actual
pointer UI tests and the rebuilt staged broker are the required boundary evidence.
GREEN execution spec enumerates exact commands/pins/timeouts. Integration follows
reviewer inspection with exact paths, committed scan classification, commit and push.
