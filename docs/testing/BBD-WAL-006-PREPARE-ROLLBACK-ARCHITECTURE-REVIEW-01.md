# BBD-WAL-006 Prepare Rollback Architecture Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected parent: `2bf87f0a`

Result: **ROLLBACK-SCOPED PRODUCTION CORRECTION AUTHORIZED**

Free Hermes confirmed the focused happy path remains expected red: exit 101, zero passed and one
failed, with public `INTERNAL` returned by the official PCZT construction call. The accepted test
source now also requires exact wallet SQLite bytes to match immediately before and after a
successful prepare.

The production correction uses the existing validated wallet file with read-write, no-create
SQLite access solely so the pinned upstream witness-cache APIs can operate. PCZT construction must
run inside one outer `WalletDb::transactionally` scope and return a private success sentinel after
capturing the PCZT, forcing the outer SQLite transaction to roll back. No upstream cache write may
commit. Construction errors remain erased to the existing stable public code, and the private
sentinel and SQLite errors must not enter public diagnostics.

After construction, a real Ironwood input is identified by its retained spend witness, not by
`dummy_sk`, which the IO Finalizer clears after signing padding spends. The unsigned check applies
only to witnessed real spends; protocol-padding signatures are not wallet authorization.

Principal Dev — Codex Sol may edit exactly `wallet-broker/src/zec/store.rs`. It must not edit the
accepted test, any other source, dependency, evidence, or governance path; execute commands; or use
Git. Jr Dev — Hermes will run the focused and full gates after reviewer source acceptance.
