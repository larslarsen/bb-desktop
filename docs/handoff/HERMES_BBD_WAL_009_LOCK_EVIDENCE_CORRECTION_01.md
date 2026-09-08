# WAL-009 lock evidence correction — documentation only

Actor: Hermes Jr Dev. Repository: `/home/lars/OpenBazaar/bb-desktop`.
Governance parent: the commit containing this handoff.

Read AGENTS.md, this handoff, the leading CURRENT_TASK section, and
docs/testing/BBD-WAL-009-LOCK-SYNC-01.md. Do not reload historical files.

The reviewer accepts the one-line lockfile outcome provisionally; the evidence needs
two corrections. Edit only docs/testing/BBD-WAL-009-LOCK-SYNC-01.md:

1. Replace its incorrect package-lock.json hash with the verified value
   `5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc`.
2. The runtime identity must describe the ORIGINAL lock-sync run (session 47880),
   not this correction run or the adoption-time configuration in the routing policy.
   Inspect that run's saved session metadata/transcript read-only, extracting only
   session ID, version, provider, and model as needed; do not expose credentials or
   full configuration. Record supported actual fields and the provenance. If an
   actual model/provider cannot be recovered, explicitly say it is unverified and
   remove the adoption-time value as a substitute. Do not invent a runtime identity.
   Limit identity recovery to five targeted read-only lookups in recent session
   metadata; do not scan historical request dumps. If still unavailable, record the
   uncertainty and finish the correction.

Execution parent remains `7b11bc61879e1097aaf8cf1dfe453ee716098470`.
Resulting Cargo.lock remains
`b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71`.
The original exit 0 and exact one-line lock diff must not change. Add a brief note
that this correction used read-only identity checks and did not repeat cargo update.

Run no Cargo, formatter, build, tests, policy, audit, scanner, npm, network, product,
or other actor. Do not touch source, lockfiles, packages, CURRENT_TASK, or other docs.
Do not stage, commit, or push. Report evidence line count/SHA-256 and stop.
