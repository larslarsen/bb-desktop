# BBD-WAL-006 Fixture Reorg Source Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Protected governance parent: `49c56241`

Result: **TEST-ONLY REORG CORRECTION ACCEPTED FOR FIXTURE-RUN RESUME**

Sol changed only `wallet-broker/tests/zec_fixture_builder.rs`. The accepted file is 928
lines with SHA-256
`4b1efec59f81761e2c713587c0a4f3e7b8c545f7b85cc35c90949c5dedbca4bc`.
All four test names and assertions remain.

The recording cache now detects only an exact duplicate block height, checked-converts
that height, and truncates its inner upstream SQLite cache to the predecessor before
insertion. Its independent observation vector remains append-only, retaining canonical
height 107 followed by replacement height 107. Sequential canonical inserts are
unchanged.

The generator no longer asks the unscanned wallet database to rewind. It finds the
recorded canonical height-106 block, requires a valid 32-byte hash and chain metadata,
and calls upstream `generate_block_at` for height 107 with that exact parent hash and
Sapling/Orchard/Ironwood tree sizes, the same Ironwood recipient/value, and broken hash
chains forbidden. It then requires the recorded replacement height before assembly.

No generator version, network activation, seed/account/receiver, amount, canonical RNG
sequence, scenario, manifest, encoder, path/mode, hygiene, production import, dependency,
feature, unsafe code, skip, or other path changed. Reviewer verification confirms
`HEAD == origin/master == 49c56241`, a clean index, exactly six untracked ZEC tests,
frozen sibling/committed hashes, absent fixture output, and `git diff --check`. Sol ran
no executable, formatter, Cargo, test, fixture, network, or Git command.

Luna may run the formatter and corrected fixture/expected-red resume in the active
handoff. ZEC production remains unauthorized.
