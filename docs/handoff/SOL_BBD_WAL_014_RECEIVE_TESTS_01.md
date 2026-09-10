# WAL-014 receive tests — documented source escalation

Grok grok-4.6 High session 01a08cf5-b5e8-7f21-b00f-a0f774282253 / actor15924 exited0
after source/API reads and an Edit attempt, leaving neither authorized test file
changed. Reviewer collected exit and verified status/diff: no usable source drop.
Escalate to Principal Dev Codex Sol, gpt-5.6-sol High, under AGENTS.md.

You ARE the appointed Principal Dev source actor; parent Codex remains reviewer.
Execute the two-file TEST SOURCE task in GROK_BBD_WAL_014_RECEIVE_TESTS_01.md.
All semantics and path restrictions there apply. Production frozen, no tests/builds/
formatters/checks/Git/evidence/other actors. No Cargo changes. Complete usable edits;
do not stop at a plan or misidentify yourself as reviewer. Reuse existing test helpers.

Efficiency notes from reviewer: actual FreshReceiverV1 is Clone/Debug/Eq. Upstream
oracle is UnifiedSpendingKey::from_seed(TestNetwork,seed,Default::default()), then
to_unified_full_viewing_key().find_address(index.into(),UnifiedAddressRequest::ORCHARD)
and address.encode(TestNetwork); see existing zec/address.rs for exact borrowed args.
rusqlite already direct-pinned. Replace UFVK via parameterized SQL in real initialized
test-owned wallet DB using independently derived foreign UFVK; no new test_support API.
Existing UI click helper returns a settled repaint, which loses release-frame clipboard
output: capture release output explicitly for Copy assertions. Preserve actual pointer
events, settled paint, and no auto-copy/issuance. Cleanup only explicit known owned ZEC
network/account directories and files, never recursive deletion or symlink following.
No full-file repeated reads when focused spans suffice. About four service and three
native UI tests can cover the contract; avoid inflating the test suite needlessly.
