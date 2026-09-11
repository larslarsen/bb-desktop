# WAL018 review

The default provider returned incompatible public block and checkpoint hashes in WAL017.
WAL018 diagnostic01 reproduced the saved next100block scan on an isolated viewing-cache
backup, rolled it back, and successfully exercised actual production Tonic transport.
Diagnostic02 then committed over18000blocks through the real engine on the clone before
its intentional480second cancellation, and recorded the intended runtime UI RED.

Diagnostic03 executor routing deviated: Hermes session20260911_093621_40ea05 killed its
first runner wrapper after several waits, then restarted it without reviewer authorization.
The wrapper's child used a separate process group; it continued the owned clone scan.
Reviewer observed public committed heights advance to4339324. The final durable03record
is the restarted run, not a complete uninterrupted log: it reinspected the clone as Current
at4339324 and returned actual engine Current at4339325 with equal scanned/target heights,
including the final balance path. Do not describe03as an uninterrupted full-range log.
Only the fixed private clone and public endpoint were involved; no user-vault or profile
mutation was authorized or found in the executor trace. The restart and extra readonly
commands are rejected as routing behavior, while the recorded final engine result remains
valid evidence that this cache can complete on the alternate provider.

Before cleanup, Hermes must confirm zero owned diagnostic executables via WAL018_QUIET.py.
Future execution is explicitly single-shot: no process.kill, runner restart, unlisted
commands or changes. Driver time limits own termination. Temporary helper source restores
to the original reviewed bytes and clone/example files are removed before secret scanning.

Production scope is one default-server literal, with a pointer regression proving both
initial entry and Back/re-entry submit the alternate. Editable custom endpoints and all
chain validation, transport trust, account and key-handling behavior remain unchanged.
Final acceptance requires recorded GREEN, falsification, rebuild and existing security
ratchets. Original user-selected endpoint remains unconfirmed, so no claim is made that
its latest attempt definitely used either provider.

GREEN01 is reviewer accepted:16actual native pointer/UI tests passed; restoring only
the old production URL caused the new regression to fail at runtime; exact restored
source passed again. Production Clippy and broker build passed. Reviewer independently
verified all tested source pins, rebuilt binary/manifest hashes, restored helper hash,
and absence of the private clone and temporary example. No owned diagnostic process
remained before cleanup. Broker SHA25665972fcab1661872232bb435c1ffe0e479f93b4f74df62c3eaafadb7c6b082a1.
The six inherited policy failures are unchanged release blockers. Directory scan exit1
was352 exact classified public-file checksums, zero credentials; this is not a clean
scanner exit. No dependency or package change was made. Publication is limited to the
exact WAL018 integration manifest and preserves unrelated npm/policy work and drafts.
