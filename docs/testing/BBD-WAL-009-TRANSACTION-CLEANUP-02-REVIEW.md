# WAL-009 Transaction Cleanup 02 Review

Result: source correction accepted for finding 4; full A3 remains rejected.

Grok session 34001 completed with exit 0. The resulting spend.rs is 880 lines,
SHA-256 `ae665e4742925d88e1f7cf6a66172e89a268f613a5fd0efc0b3cc3581ac4cdf4`.
The other six pending Rust files, accepted Rust manifest/test, and protected package
files retain their Correction-01 identities. Source inspection confirms the owner is
created before serialization, survives all subsequent fallible calls, and zeroizes
the actual live buffer before observing its wipe in Drop. The malformed-state hook
wipes the discarded tail before truncation. Explicit drop precedes success return.
This closes the reviewed early-return/unwind gap for that owned allocation; it does
not establish erasure of previous Vec allocations or all third-party parsed objects.

No formatter, compilation, or test result is claimed. The actor reported no such
execution. Dummy harness wipe observations, copied verification metadata, native UI,
and dependency visibility remain unresolved. Source is not integrated.

Next bounded correction: expose the already locked orchard 0.15.5 crate directly to
the broker, with default-features=false and circuit enabled. Local orchard source
confirms VerifyingKey::build and Bundle::verify_proof; pczt's existing prover feature
already enables orchard/circuit. This is a direct-dependency declaration, not a
version upgrade or permission to expand the enabled feature set. The root lockfile
dependency list will require a separate Hermes synchronization and dependency/policy
gate. Full cryptographic correctness remains subject to review and tests.
