# WAL-013 service test-source correction 01

Collected Grok48bf76ea / outer45266 exit0; first drop NOT accepted for execution.
Only wallet-broker/tests/account_management.rs writable, current SHA256
791732b8cf28371e3ae2cbd788b94a4c045b1cc7e26cb6a6a81297e6bc3c1fb6 (1274lines,11tests).
Cargo c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda frozen.
Grok grok-4.6 High no subagents; source-only; no tests/build/syntax/format/execution,
Git/evidence/otheractor. Read this plus last concurrency refinement in original account
contract, named test and relevant vault/session APIs only. Finish promptly, no broad
new searches or test expansion beyond exact items below.

1 FixedEntropy::fill currently mixes () branches with Result error. Return Ok(())
after copying and return Err from unsupported branch. This is a source compile bug.
2 AccountManager explicitly must NOT implement Debug. The two with_ports(...).unwrap_err()
require it; use .err().expect("...") or an explicit match. PreparedRestore similarly
has no public Debug contract; change prepare_restore error assertions to err().expect.
Do not make production implement extra visibility for tests.
3 Cleanup helpers currently swallow every error and readdir failure. On normal test
completion verify all owned roots and extras are absent using symlink_metadata, panic
on cleanup failure; never follow symlinks/recursive delete. Don't silently flatten
readdir errors. During unwinding report cleanup failure without a second panic. Keep
explicit known account child then root removal. Tests must detect owned temp leakage.
4 Add ONE focused test for original contract's directory advisory lock refinement:
second manager same root=>UNAVAILABLE, unrelated root works, dropping first releases
lock and reopening returns persisted locked account. No testexecution here.
5 Extend existing hostile test with symlink root and accounts-dir cases (assert target
mode/contents unchanged), and 1024-entry bound using cheap known .stage names: one
active+1023stages allowed, adding one =>LIMIT. Use independent owned scratch objects;
no KDFs needed for these metadata checks. Existing 256cap preserves LIMIT semantics.
6 Extend cap test: prepare one authenticated new backup BEFORE filling catalog256;
then confirm=>LIMIT and unchanged 256 active vaults. This proves commit recheck after
preparation, one KDF seal/open only. Existing capcreate and257 tests retained.
7 Extend entropy failure test after generated ID+seed: script only account-id/account-seed
so vault-salt fails; assert no active write and observed zeroized passphrase+plaintext.
Use separate manager/root to avoid queue collision with duplicate-ID case. Also assert
collision failure wipes passphrase. No additional giant matrices.

Review accepts clock sequencing correction and all-native-origin confirmation coverage
already present. Expected missing-accounts red only after these source issues fixed.
Do not execute any check; report final hash/lines/test count and stop.
