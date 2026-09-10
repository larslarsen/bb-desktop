# WAL-013 account service source correction — Sol escalation

Reviewer records Grok source insufficient for bounded custody drop: despite explicit
Zeroizing requirement, seed used unguarded stackarray/manualwipes only, leaving panic
path uncovered; missing StorePort trait prevents compile. Grok6481c6a3/outer92553
collected exit0. AGENTS permits Sol gpt-5.6-sol High fill-in. Source-only no subagents.
ONLY wallet-broker/src/accounts.rs writable, baseline
0250afacbe3837994eaa6f128fc4241bb75a3fc0238c74be20bdf15b41f8bd0c (704lines).
lib.rs 96bf49de102f16c026fb3d098cb3b447a269dc5f9176fd29c902ac55d44e7246 frozen.
Read this, AGENTS/TESTING, original account contract as needed; only relevant target
spans/vault/store signatures. No broad research/wholeoldtest reread. No tests/Cargo,
execution/checks/build/formatters, Git/evidence/otheractors. Four exactcorrections:

1 Import crate::store::StorePort for LinuxStorePort trait methods inspect,
read_bounded,ensure_directory. Existing APIs need trait in scope.
2 Wrap fresh 32-byte account seed scratch in zeroize::Zeroizing<[u8;32]> BEFORE
EntropyPort.fill. Pass &mut *seed_bytes. Retain prompt explicitwipe after SecretBytes
copy/error; RAII wipes unwind/earlyreturn. No ordinary unguarded seedarray orreallocation.
3 export_encrypted currently copies unvalidated activecontents. Before existing
VaultStore export, call existing read_active_account to validate canonical expected
profile/filename binding, map malformedactive to closedUNAVAILABLE. Never decrypt.
Rootguard serializes cooperatingmanager writes; don't claim protection against a
malicioussame-user process bypassingadvisorylock. No newstore/path/dependency.
4 Original unlock contract returns LOCKED for corrupt encrypted envelope as well as
AEADbadpassword/non32body. In private read_active_account and unlock parse_vault error
branches map parse failure to LOCKED, retaining UNAVAILABLE for file/path/permissions
or wrongprofile/headerbinding. Catalog parsing stays UNAVAILABLE, restoreparseSCHEMA.
Existing reviewed tests and initial red cover newAPI; this is preacceptance contract
completion, no broader behavior/test redesign. Finish exacthash/lines; noexecution.
