# WAL-017 — Diagnose near-completion sync failure

Baseline fa2ec32039eff0b27d0d81a78e91c20723d547e9. User reports real sync progressed
almost to target then Failed; retry removed visible previous progress. Read-only minimal
checkpoint inspection found1842420birthday,4308219committed,4338623target. No vault,
keys, addresses, amounts or user database rows other than three heights were read.

Diagnosis authorized: reviewer source review; existing principal Sol core actor read-only
review (prior Grok stopped without usable drop; retained senior corrective escalation).
Hermes executes docs/handoff/WAL017_DIAG.js only: bounded TLS public lightwalletd reads
at https://testnet.zec.rocks:443 (metadata, tree at4308219/4308319, next100compactblocks).
No keys/account identifiers/wallet material sent. No user profile access or mutation by
runner. At most20MiB per response,15second deadline, default TLS verification; no
fallback server or TLS bypass. Save public protobuf replies under disk-backed existing
wallet-broker/target/wal017-diagnostic and safe length/status/time summary; this is a
one-off public-server diagnostic, not an acceptance test dependent on mutable service.
Record actual Hermes version/provider/model/session alongside evidence. No source/test,
dependency, formatter or Git actions yet. Product fix/test contract follows evidence.
Preserve unrelated npm/policy changes and two historical untracked evidence drafts.

Additional bounded diagnosis authorized: Sol may author only temporary
wallet-broker/examples/wal017_diagnostic.rs. This is an offline diagnostic executable,
not product behavior or an acceptance test. It reads only the owned clone at
wallet-broker/target/wal017-diagnostic/viewing-copy.sqlite3 and captured public frames.
Run actual upstream update_chain_tip+scan_cached_blocks for4308220..4308319 with
TestNetwork and copied viewing account; always roll back the owned clone transaction.
Print only fixed stage labels and allowlisted structural error details (block heights,
pool, tree/checkpoint positions, SQLite error codes); never keys/addresses/amounts,
raw CorruptedData/BadAccountData/SQL error strings or account identifiers.
Hermes may create that exact private clone using SQLite backup from the single real
live.sqlite3 opened URI mode=ro (only viewing cache; no vault/seed). Clone directory0700,
file0600; no writes to live profile. Execute cargo run with original locked offline
manifest/native-ui, --example wal017_diagnostic. No live RPC in replay. Explicit output
path and no general file/seed argument. Source reviewer approves before execution;
remove exact temporary example/clone after diagnosis. Disk has3.9GiB available;
no dependency installation or cache cleanup authorized. Production fix remains gated.

Private clone replay authorization withdrawn before execution: reviewer found public
TreeState4308319hash disagrees with CompactBlock4308319hash. Thus actual validator
rejects before commit; scan replay not needed. Hermes may instead execute
node docs/handoff/WAL017_DIAG_SAME_CONNECTION.js with the same five bounded public
requests over one persistent HTTP2 connection, outputs wal017-diagnostic-sameconnection
and DIAGNOSTIC-02.md. No private data copying or example execution authorized.

Same-connection responses are byte-identical for prior/end/range and reproduce mismatch.
Read-only comparison of two operator-published Testnet alternatives authorized through
WAL017_DIAG_ALTERNATES.js zecpro and nighthawk (fixed TLS hosts lwd.testnet.zec.pro:443,
testnet.lightwalletd.com:9067). Same bounded five public reads, stop endpoint on first
error; no wallet/profile traffic or automatic product server change. Hermes may unlink
only the unused wallet-broker/examples/wal017_diagnostic.rs. No clone was created.

Two alternatives failed metadata (ZEC.PRO HTTP521; Nighthawk connection failure).
One final bounded public diagnostic is authorized: WAL017_DIAG_ALTERNATES.js zaino,
fixed https://zaino.testnet.unsafe.zec.rocks:443, operator-advertised Testnet Zaino
endpoint listed by Hosh. No TLS bypass, user wallet traffic, or automatic product
configuration change. Same limits and evidence. No additional endpoint search authorized.

Resolution: inconsistent default-server responses confirmed over both connection
patterns. Native validation refuses final checkpoint hash before committing, exactly
preserving4308219. Alternative operator endpoint Zaino returns matching hashes and
same prior checkpoint. Owner can change Server and resume. This does not claim actual
wallet completion. No product modification required to use the alternative. Generic
failure wording and blank initial retry progress remain known UI limitations.
Reviewer comparison contract WAL017_COMPARE.py checks chain/height/metadata/transaction
lengths/tree-size deltas and contrary end hashes on captured public replies. Hermes
executes it and exact diagnostic-record publication via WAL017_INTEGRATE.json, retaining
unchanged source/dependency/policy pins. Pinned committed Gitleaks uses prior five-public-
checksum classification only. No new credentials allowed. No release or build work.
