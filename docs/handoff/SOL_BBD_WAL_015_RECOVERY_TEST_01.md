# WAL-015 hot rollback journal regression

Principal Dev Sol High, existing escalation. Own only NEW
wallet-broker/tests/zec_live_recovery.rs, plus a tiny refinement to your own
zec_live_validation.rs header test so hash mismatch and previous-hash mismatch are
rejected independently, each with the other field correct. No execution, formatting,
Git, dependencies, helpers/production, evidence or other actors.

Reviewer found that prepare_live_path rejects all sidecars before its later SQLite
recovery code, preventing resume after a real hot-journal crash. Author one real
recovery regression through a proposed missing hidden harness method:
LiveSyncHarness::reopen_via_preparation(restart: LiveRestart, fixture:&FrozenFixture)
 -> Result<LiveSyncHarness,ZecError>. It must eventually call actual prepare_live_path,
not just inspect_live; keep helper missing until Hermes records RED.

Use existing FrozenFixture and LiveSyncHarness to scan through103, capture actual
live and receive bytes/inspection, close harness to its restart capsule. Spawn this
integration-test executable as an OWNED child running an exact ignored helper test
with a task-specific env path to the owned live database (no user data). Child opens
SQLite, sets DELETE/FULL, cache_size=1, cache_spill=ON, BEGIN IMMEDIATE, updates only
ext_bitbook_live_state.ufvk to a large test marker (e.g.256KiB) to force dirty-page
spill/hot journal, then std::process::exit(73) without destructors/commit. No custom
journal bytes. Parent enforces bounded child wait/reap and asserts exact exit73,
existing private regular single-link nonempty journal. Reopen through preparation,
assert journal consumed, exact prior live bytes/inspection/receive bytes preserved,
then resume real scanner through107 and assert Current/known nonzero balances.

Owned child guard must kill/reap on panic; exact current executable and exact test
name only, bounded waits. No persistent private key or external network. Fixed test
marker cannot expose seed/UFVK. Existing harness artifacts live under disk-backed
target. Avoid recursive deletion; if new cleanup is added enumerate only owned known
files and validated parent directories. Do not execute the test yourself.
