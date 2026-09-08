# WAL-009 lock synchronization review

Result: ACCEPTED for lock synchronization and corrected evidence only.

Reviewed against governance parent `7af2adbdd2ef3f8a2a929e1b5e24dea0a732ad25`.
The worktree diff adds only `"orchard"` to the broker's root dependency list;
no resolved package, version, source, or checksum changes. Resulting Cargo.lock
SHA-256: `b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71`.

Corrected evidence: [BBD-WAL-009-LOCK-SYNC-01.md](BBD-WAL-009-LOCK-SYNC-01.md),
SHA-256 `ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2`.
The package-lock hash now matches the actual file. Runtime attribution records
Hermes v0.18.2, provider `nous`, model `poolside/laguna-s-2.1:free`, recovered from
the original Hermes session `20260907_204336_5c6109`. Outer terminal session 47880
and the Hermes session ID are different identifier namespaces, not conflicting
execution identities. The evidence correction did not rerun cargo update.

This is static diff/hash and evidence review, not a full transcript audit or a
claim that signing compiles or passes tests. Cargo.lock and implementation evidence
remain uncommitted for later Hermes integration. The reviewer publishes only this
review, CURRENT_TASK.md, and tickets/BBD-WAL-009.md.

Correction 02's actual transaction-buffer ownership repair and Correction 03's
direct Orchard declaration remain source-accepted. A3 remains unaccepted because
actual-secret cleanup observations, independent recovered transaction effects,
and a live native confirmation event path are still unresolved.

The next UI slice needs a reviewer-fixed event/lifetime contract and regression
tests exercising real egui input before production changes. The current one-frame,
default-input implementation cannot receive a native click. Merely storing a
confirmation flag or polling a bool without defining the caller lifecycle would
not resolve that boundary. No new source actor or execution is authorized here.
All pending source, unrelated package changes, and parked Monero work are preserved.
