# Current Task

WAL-017 diagnosis is COMPLETE: the default testnet server returns inconsistent block
and checkpoint hashes at4308319, reproducing both with separate and persistent HTTP2
connections. The broker correctly rejects that batch before commit. User's checkpoint
4308219/4338623 remains saved; no wallet data was modified or copied. No product code,
dependencies, tests or binary changed. The unused temporary diagnostic example was
removed without execution.

Operator-advertised alternative https://zaino.testnet.unsafe.zec.rocks:443 returned the
same prior checkpoint and block batch with a matching end checkpoint. User can set the
native Sync screen Server field to this endpoint and Sync to resume. This establishes
consistency for the failing100block batch, not full-wallet sync completion. Other
probed alternatives: ZEC.PRO HTTP521; Nighthawk connection failed.

Exact public comparison and publication are recorded by WAL017_INTEGRATE.json in
docs/testing/BBD-WAL-017-INTEGRATION-01.md. No additional source work is authorized.
Prior completed records below are historical; native binary remains the WAL016 build.

WAL-016 sync interruption correction is COMPLETE and reviewer accepted.
Ordinary 15-minute auto-lock still wipes spending keys; explicitly started viewing-only
sync continues on the same screen through completion. Balances stay hidden until masked
inline unlock reveals the same result without another sync. Explicit cancellation and
manual lock/hide/drop still stop work and clear results. Session timeout is unchanged.

Hermes verified 48 account/native-UI/session tests and production Clippy; a real-engine
Testnet fixture finished after forced expiry. Removing returned-copy redaction caused
the intended runtime failure; exact restoration passed. Native broker rebuilt and both
Xvfb and actual desktop open/hide/reopen/close checks passed using empty test profiles.
Staged broker SHA256 e805ff8b9ecb88f6bab4c0360572b5e1d1261187cdbb0ebd3a154c42f09ff9aa.
No user profile or dependencies changed. The six inherited policy failures remain;
pinned directory scan exit1 contains352 previously classified public-checksum matches,
zero credential findings. See docs/testing/BBD-WAL-016-GREEN-01.md and FINAL-01.md.

Source d9cd7ac4253f5956ce58509cadbe56ad1a6c835a and integration evidence
593ed713f04a39f9e29cef3a9fbfcc8271feca8c are pushed to origin/master. Reviewer
independently verified every integrated blob and unchanged tested working bytes.
Committed scanner exit1 has82 previously classified public-checksum matches and zero
credential findings. All source work and execution are complete; no background work
remains authorized. See docs/testing/BBD-WAL-016-INTEGRATION-01.md.
Fully quit and reopen BitBook to load the rebuilt broker.
The completed WAL015 entry below is historical and its binary hash is superseded.

WAL-015 native live Zcash TESTNET balance synchronization is COMPLETE.
Source edda35308913d59683a08be58052dc7c2d95dbaf and integration evidence
105a2e36af03d26dbc15a3d052901d836b2a63b5 are pushed to origin/master.
No implementation or background actor work remains authorized by this ticket.

Restart BitBook, then Wallet -> Manage accounts -> select/unlock -> Sync balance ->
Sync. The server is editable; default https://testnet.zec.rocks:443 uses direct TLS.
The native screen shows connection disclosure, progress, cancellation and confirmed/
pending received shielded funds. First sync covers history from testnet NU5; restart
resumes committed progress. Receive/Copy and existing account custody remain intact.
No user profile was read or modified. Mainnet and payments remain disabled.

Accepted111distinctRusttests,28appJSgroups,33manifestpolicygroups; productionClippy;
real crash/journal recovery falsification and exactrestore; native UI pointer proof;
Xvfb and actualdesktopwindowproof; dependency audit with no newvulnerabilities.
Final staged Linux x64 broker SHA256:
6c2880cef4eafc5b36e09f47428d4f9ace98deed0f62703c7bab6fd3a7c909f7
552181016bytes. All37integratedblobs independentlyverified. All executionCLIactors exited.

Six inheritedsecurity-policy failures remain releaseblockers;7oldtest-helperlint
warnings remain recorded. Gitleaks exits1 for verified publicfilechecksums in WAL015generatedrecords:
184directory/32committedfindings in integration, zero actualsecretfindings after exact
captured-value comparison to4recomputedpublicfilechecksums at integration. Rules/config/ignore unchanged;
never describe that scan as exit0. See ticket exception and integration evidence.
No installer or release-ready claim. Original npm/policyworkingedits and2historical
untracked evidence drafts preserved; only5WAL015policychanges staged on originalHEAD.

See tickets/BBD-WAL-015.md and docs/testing/BBD-WAL-015-LIVE-INTEGRATION-01.md.
All earlier WAL015handoff authorizations and failures are historical execution records.

Finalgovernance scanner:82public-checksum findings,0actualcredentials after reviewer
verified the fifth referenced publicfile. See
docs/architecture/BBD-WAL-015-FINAL-SCAN-REVIEW.md for exactclassification, final
review and runnertermination. No active/background implementation remains.
