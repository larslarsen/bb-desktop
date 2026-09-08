# WAL-009 retained-spend API review

Decision: accept Grok's no-edit stop and correct the API route. Reopen the same
combined four-path test/production task as repair 02; no predicate is removed.
Reviewer: Codex at XHigh, baseline 4b63a0fc671d7a74ab5288d700cfee8254f6816a.

Grok session 3026df1d-8a5c-4f63-b2a8-0cf1d279e5f4, outer 71820, was collected
once after done, exit 0. Runtime grok-4.6-build, High; CLI grok-4.6.
All three writable starting hashes and the frozen context-test hash still match
repair 01; the new module is absent. No test or production edit was delivered.

The reviewer contract omitted the necessary public API route. Grok correctly
identified that pczt 0.9.3's wire Spend::value field is private. Its broader
conclusion that a dependency change is required is resolved by the existing
Verifier role, rather than by weakening the value checks or escalating models.

Verified local pinned source:
- pczt 0.9.3 src/roles.rs: Verifier is exported without a new feature.
- src/roles/verifier/mod.rs: new consumes Pczt; finish returns Pczt.
- src/roles/verifier/orchard.rs: with_ironwood exposes the parsed
  &orchard::pczt::Bundle to a fallible closure, then reserializes it internally.
  It requires only the orchard feature, already enabled by the existing backend
  pczt/signer feature dependency.
- Orchard 0.15.5 src/pczt.rs: Bundle, Action, and Spend have public getters;
  the parsed spend value is Option<NoteValue>. src/value.rs exposes inner().

Use that role by ownership before signing and after finalization. Capture the
real unsigned action's retained identity and actual positive value, require the
other spend's actual value is zero, and recheck both values after proving.
Run existing prepared/output checks before the additional role round trip,
because parsing can expand compact output representations. No new PCZT/secret
clones, library patch, codec, inferred values, or cryptographic bypass is allowed.
Return the role's Pczt to the next existing stage. Full recovered effects and
actual-secret cleanup remain separate unaccepted requirements.

Open [repair 02](../handoff/GROK_BUILD_BBD_WAL_009_RETAINED_SPEND_REPAIR_02.md).
The tests-first combined repair, four writable paths, actual external-index
propagation, existing understood integration red, later grouped Hermes validation,
and falsification obligations remain. No tests were executed by the reviewer.
No actor polling. XHigh remains appropriate. Previous execution/evidence
acceptance and deferred minor report errata are unchanged.

Reviewer publication scope: this review, the linked repair-02 handoff,
docs/handoff/CURRENT_TASK.md, and tickets/BBD-WAL-009.md only.
