# WAL-009 native runtime decision and library checkpoint 01

Reviewer: Codex; owner selected XHigh for this architecture decision.
Actor: Jr Dev — Hermes, evidence normalization and source integration only.
Protected parent: the reviewer commit publishing this handoff; one subsequent
reviewer launch checkpoint may change only CURRENT_TASK.md. Record actual HEAD.
Previous governance: c50b662fb3bb26c236d61eb73a4ec7cb1f27424a.

## Architecture decision — 2026-09-09

Keep the existing synchronous native review component. Pinned eframe 0.36.1
src/native/run.rs:51-74 caches one event loop per thread specifically to support
successive windows; run_glow routes run_and_return through that cached loop.
The earlier finding must not be read as creation of a fresh OS event loop for
every dialog. Pinned winit 0.30.13 src/event_loop.rs:83-95 requires the process
main thread for cross-platform event-loop creation. No UI rewrite is justified
solely by repeated run_native calls.

The broker executable's process main thread will own native UI. Protocol I/O and
cryptographic work must remain responsive on worker threads. One immutable review
may be displayed at a time; a bounded private in-process request channel carries
its exact account/session/request/handle/intent/review bindings and expiry, never
seed/key/PCZT bytes. The UI owner stays on the main thread. Thread names or a
platform-specific any_thread switch are not proof of that ownership.

An authenticated cancellation, lock, account/session replacement, pipe EOF, broker
shutdown, user cancellation, window close or expiry invalidates the pending review.
Control processing must continue while a dialog is open. Native completion returns
only to the matching private request; a cancelled/replaced/expired request drops
late results. Recheck authoritative state before seed access and before verified
publication. A native capability remains one-shot, non-serializable and process-
local; neither an IPC boolean nor Electron can mint it. Cancel/close wins a
simultaneous user confirmation. No automatic replay after process restart.

These are constraints for the executable/runtime implementation, not claims about
current code. The current library has no executable entry point, live protocol
framing or Electron startup connection. WAL-001 section 4.2 explicitly reserves
packaged pins/spawn for WAL-011 and fixture-only operation beforehand. Therefore:

- WAL-009 Phase A1 retains its local fixture signing/independent-verification scope.
  Existing native dialog/App/capability tests establish their component behavior.
  Another simulated frame test or throwaway GUI executable would not close the
  production process boundary and is not the next task.
- WAL-010 owns the usable native onboarding/review and Pay flow. WAL-011 owns the
  actual broker entry point, inherited byte-stream framing, supervisor startup,
  binary pins/packaging and tests through that process boundary. Create its ticket
  before implementing those paths; no runtime source is authorized by this record.
- OS acceptance must exercise the main-thread entry point, successive dialogs,
  real input/close, cancellation while open, stale replies and child exit on each
  supported release platform. Current egui tests are not that evidence.

Read-only secret-owner review also identifies a dependency limitation:
zcash_keys 0.16.1 UnifiedSpendingKey contains private key fields; orchard 0.15.5
SpendingKey is a Copy [u8;32] wrapper, and SpendAuthorizingKey wraps another private
signing type. Their inspected definitions do not implement Zeroize or custom Drop.
pczt 0.9.3 consumes and reconstructs private typed data through signing/proving.
The broker's drop(usk), ordinary typed destruction, or wiping the serialized
SecretBytes cannot establish erasure of all those copies/prover allocations.
This is not repaired by adding more observer counters. Full typed-secret erasure
remains an explicit Phase-A1/final-release blocker; no requirement is waived.
Before real custody/production activation, authorize a separate dependency-level
hardening decision with achievable guarantees and evidence. No unsafe memory
inspection, custom cryptography or dependency fork is authorized here.

## Purpose and limits of this integration

Publish the already reviewed, locally validated library work as a partial source
checkpoint. This is neither full Phase-A1 acceptance nor completion of WAL-009,
a usable app wallet, runtime integration or release approval. Previously accepted
signing/decoded-effects/native-component/cleanup/panic results remain valid at
their recorded scope. Do not repeat them. The exact 39 input identities below were
independently verified by the reviewer; 20 source/manifest/lock paths and 18 evidence
records are dirty, while tests/native_surface.rs is an unchanged regression input.
The checkpoint makes source progress durable without claiming the remaining gates.

Remaining before final acceptance: typed-secret requirements, a consolidated
formatter/lint/regression/security gate against the eventual final source, and
all later durable-send/transport/UX/runtime work separately authorized by ticket.
No network/broadcast/mainnet/hardware/Monero/Electron-send capability is enabled.

## Authorized scope and procedure

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, this handoff,
CURRENT_TASK.md active prefix only, and tickets/BBD-WAL-009.md active prefix only.
The ticket lives at repository-root tickets/. Do not reload historical handoffs
except the two exact acceptance excerpts consumed by the normalization function.
Use explicit repo-root workdir for each command. Record hermes --version once;
report the actual session/provider/model if available, otherwise label unavailable
for reviewer verification. Do not infer an unset terminal variable from execute_code.

No source/test/manifest/lock/fixture edits, formatting, compilation, Cargo/Node
functional tests, proof generation, dependency/advisory updates, additional actors,
product/native launch, hardware, cleanup/deletion or new backup are authorized.
Existing disk-backed target files and every old backup remain untouched.

1. Read-only preflight: require master, an empty index, HEAD at the protected parent
   or its single CURRENT-only launch checkpoint, and no other dirty/untracked paths
   than the 38 changed paths in the frozen inventory. Verify all 39 identities.
   Require both new integration evidence path absent and the unchanged scanner pin
   and ignore file below. Stop on drift; do not repair or rediscover alternatives.
2. Normalize precisely the eighteen evidence records using the function below.
   Verify all originals first. Require each generated result to equal its exact
   post-normalization line count/SHA-256 before writing. No other evidence edit is
   authorized. The two latest reports carry forward their already accepted reviewer
   corrections and fix known identity/log/deviation claims. Preserve all old test
   results; do not reconstruct missing compiler output or label summaries verbatim.
3. Create docs/testing/BBD-WAL-009-LIBRARY-CHECKPOINT-01.md. Record runtime identity,
   actual starting HEAD, unchanged source inventory, before/after evidence hashes,
   partial-checkpoint scope, open blockers, commands/results and tool identifiers.
   State explicitly that no functional tests were run in this integration task.
   Update only the CURRENT active prefix, preserving its historical suffix, to
   CHECKPOINT PREPARED — SECRET SCAN/COMMIT/PUSH PENDING. Keep this handoff/ticket
   and all other governance frozen. The only writes are the 18 transformed evidence
   files, this new integration evidence and the CURRENT active prefix.
4. Check git diff --check; require exit 0. Run the one exact scanner command below
   once, with redacted output. Require exit 0 and zero leaks. No install/update,
   additional scan, suppression or source repair. Retain complete terminal output
   and identifiers; foreground is fine, or background with waits <=60 seconds and
   complete process.log retrieval. The reviewer will not poll this actor.
5. On success, record the scan outcome in the new integration evidence and set the
   CURRENT prefix to PARTIAL LIBRARY CHECKPOINT INTEGRATED — REVIEW PENDING, with
   the remaining blockers and no further actor authorization. Verify all 21 source
   input hashes unchanged, all 18 normalized evidence hashes exact, and no paths
   outside this contract changed. Scan the new/normalized records for local absolute
   home/repository paths; require none. Recheck git diff --check.
6. Stage exactly the 39 inventory paths plus the new integration evidence and
   CURRENT_TASK.md. The unchanged native_surface.rs produces no staged delta.
   Require the staged path set to be exactly the 38 changed inventory paths plus
   those two records (40 changed files); inspect cached diff --check and --stat.
   Never use git add -A, -u, a directory or glob. Do not stage this handoff/ticket,
   targets, backups, actor state or other paths.
7. Commit exactly `wallet: checkpoint reviewed WAL-009 local signing pipeline`.
   Push master once. Do not retry a failed push or modify/remake the commit.
   Run only git status --short, git log -1 --format='%H %s', and git rev-parse
   origin/master for final-state proof, then stop. No CI polling or follow-up work.
   Report commit, push result, scanner result, changed paths/counts and open scope.

On any mismatch, stop the sequence without committing/pushing (a failure after a
commit preserves that commit). Record a concise failure in the new integration
record and CURRENT prefix if those writes are already reached; otherwise report
it directly. Never revert someone else's files, normalize a drifted input, broaden
commands or repair tests. No complete/accepted status may be substituted for partial.

Scanner: target/security-tools/gitleaks-v8.30.1/gitleaks, 21958840 bytes,
SHA-256 88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509.
.gitleaksignore SHA-256 1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b.
Exact scan command (repo-root workdir, no wrapper/redirection):

```text
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
```

## Evidence transformation

Use Python with pathlib.Path and re imported. This function computes the only
permitted result; it is not permission to rewrite historical reviewer handoffs.
Construct all outputs in memory, check the after table, then write the 18 records.

```python
def normalized_evidence(path, original):
    text = original.decode("utf-8")
    title, body = text.split("\n", 1)
    note = (
        "\nIntegration note (2026-09-09): local repository and home prefixes are "
        "rendered as <repo> and <home>. Earlier hashes/counts describe execution-time "
        "files. This is an archival evidence record; acceptance limits and reported "
        "deviations remain governed by the corresponding reviewer handoffs. "
        "No test was repeated for this source checkpoint.\n\n"
    )
    corrections = ""
    if path.endswith("CLEANUP-LIFECYCLE-VALIDATION-01.md"):
        review = Path("docs/handoff/HERMES_BBD_WAL_009_CLEANUP_LIFECYCLE_VALIDATION_01.md").read_text()
        corrections = review[review.index("## Collected validation acceptance — 2026-09-08"):]
        body = body.replace("poolside/laguna-s-1.2:free", "poolside/laguna-s-2.1:free")
        before, rest = body.split("## Deviations\n", 1)
        _, after = rest.split("## Summary\n", 1)
        body = before + "## Deviations\n\nThe reviewer correction above supersedes the original no-deviation claim.\n\n## Summary\n" + after
        body = body.replace("inline inventory script, verbatim from handoff", "abbreviated reference to the handoff inventory script")
    elif path.endswith("PIPELINE-PANIC-VALIDATION-01.md"):
        review = Path("docs/handoff/HERMES_BBD_WAL_009_PIPELINE_PANIC_VALIDATION_01.md").read_text()
        corrections = review.split("## Collected validation acceptance — 2026-09-09", 1)[1].split("## Next-step native integration findings", 1)[0]
        corrections = "## Collected validation acceptance — 2026-09-09" + corrections
        body = body.replace("HERMES_SESSION_ID: UNAVAILABLE (environment variable not set; reported as unavailable for reviewer verification)", "HERMES_SESSION_ID: unavailable to the actor check; saved runtime session is 20260909_000439_f17d68. The check does not establish that the terminal environment variable was unset.")
        body = body.replace("PROVIDER: UNAVAILABLE", "PROVIDER: nous (saved runtime database)")
        body = body.replace("MODEL: UNAVAILABLE", "MODEL: poolside/laguna-s-2.1:free (saved runtime database)")
        body = body.replace("Provider/model from actual runtime could not be determined; no cargo --version or other discovery command was run per handoff.", "The actor reported identity unavailable; the reviewer resolved the actual session/provider/model from the saved runtime database.")
        body = body.replace("Full output:", "Historical summarized output (not a complete verbatim transcript):")
        body = body.replace("Guarded mutation script (verbatim from handoff)", "Abbreviated reference to the handoff mutation script")
    if corrections:
        note += "The following previously accepted reviewer correction governs the historical actor narrative below. Its references to uncommitted state describe the original review date, before this checkpoint.\n\n"
        note += corrections.rstrip() + "\n\n## Historical actor narrative (subject to the corrections above)\n"
    result = title + "\n" + note + body
    result = result.replace(str(Path.cwd()), "<repo>").replace(str(Path.home()), "<home>")
    assert not re.search(r"/(?:home|Users|tmp|run/user)/", result), path
    return result.encode("utf-8")
```

## Frozen input inventory (39 paths)

| Path | Lines | SHA-256 |
| --- | --- | --- |
| wallet-broker/src/zec/spend.rs | 1160 | 9c41f98467ac4444fb75db50c2f319226bf3fbdd53b86b73ba3c5c7104f5caa9 |
| wallet-broker/src/zec/test_support.rs | 4479 | 255600063e348a54654cd5923496a9bba5e2dfa447222997c6bfc79b2d592c27 |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 768 | 36599689a0dd3a45c9ef93dc8ef59947bdf10a81e8efd6a8ea7cfa8b747df110 |
| wallet-broker/src/native_ui.rs | 321 | d132a164a6413165291abd0582abf3eca4cacd3f33ce6eb4ac2b467e8774d960 |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf |
| wallet-broker/src/native.rs | 489 | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |
| wallet-broker/Cargo.toml | 122 | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | 5395 | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |
| package.json | 42 | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |
| package-lock.json | 396 | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| wallet-broker/src/zec.rs | 274 | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 2872 | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 120 | ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 199 | 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 181 | 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2 |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md | 216 | 79b6e97f5a6a4ca4903d9db7b40233ecf0f2ca931116e0d232853f1b842d1ff2 |
| wallet-broker/tests/zec_sign_verify.rs | 1338 | 99c0d053c247b21af1ebc090e6b284bfdc0c0e4f2b559e3720939a591411196b |
| wallet-broker/src/zec/spend/external_binding_tests.rs | 114 | bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 677 | 6a23bcb2943df54953d1acddf85a2ec9cfc41b92e84c12856a48b7d07be53c58 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 358 | d51cc09e82f60a02a1493338d0af249dd25f046e2b5ca75e46a0fe4019269ed1 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 560 | 62bb1ffd672c460f6fac6439abbc8d555683342e5410ad7098d2339bf509e5d0 |
| wallet-broker/tests/native_surface.rs | 664 | 349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d |
| wallet-broker/src/native_ui/zec_native_app_tests.rs | 376 | 485ce9691f873af9304e1db36de74d49627279abf9c550ccce4c74bada22a5a4 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 503 | 3a734781d0ad2c31989318373272f13ba89654ded2ab3a6b90f83ea322821de1 |
| docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 184 | e749d709d8d2bd8f5e72a8509c156a2053dbed5983849964348eefbc3429733b |
| wallet-broker/src/zec/spend/effects.rs | 379 | cfa6a86f35d6772053025ca1c9bbb3154d604f79c90d031af261787938576035 |
| docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md | 354 | ffcfc14837ff02aba33a5a24fa6a58d610055b18b4efd150787c6b8acba7dc43 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md | 196 | a7def01abd54eae10e663e341ce8b253992d883863a1410c93278fa698d78431 |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md | 165 | fe82cce82ebba2595747a411a5ffd2258d86abed36dc9b3de60517193d4babad |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md | 345 | 02d078de8a0269ba2dc41664450965a3eec1036ccb5343db6bb8f90518a64f5a |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md | 177 | 996f26aeea6b1cb27799200b24258e0e41defaa05505a0ee2b704c60471c2e9e |
| wallet-broker/src/vault.rs | 794 | f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49 |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md | 564 | 5bfed1076b4668f2df03cc2766983989323a8c0332ada7b4f43db34f2c8868a1 |
| wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs | 452 | b280e7dfa0eb3f65360b33d6f2f5e7debb3fe53c1d4ea4ea8ba1fcfd54500c06 |
| wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs | 174 | ca7a373f8d009e9fc66e65ecfb9d63b4b1483fb924ab9f66dcba511a3f904ccc |
| docs/testing/BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md | 400 | 46415af46bd4313598c9c30caea3c07b06cb2fdb5d0d6c2ce479919a7c58a869 |
| docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md | 100 | acda820c0aa1b6be7cb8a8eea7a1310dc6b588502c1067d381ea4691795958c3 |
| docs/testing/BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md | 343 | a25a5868749a0ac9458712ff91c849f02bd29d483cc3c6ea49a1d1fee531efb4 |

## Expected normalized evidence (18 paths)

| Path | Lines | SHA-256 |
| --- | --- | --- |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 123 | bb4da47067749e41de359b9b695c9a05fe1e7a4982821329fab4344139abbe4e |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 202 | 8bc81940922e5aecd998f8f900c5a50183524007506b31f8a11c8399e43fdc57 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 184 | aa4f845242576d5d08bfb031a40018a96a85e510bb5a463cbeb92262be1e03e9 |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md | 219 | 4284b4ea3159e453e1531852222de928ae4669a36b6ecb367ef07c2c5b2d6358 |
| docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-VALIDATION-01.md | 680 | 6c49881b3ffff9e919d309c060c53a1e095d47c72854fbe2b1801cd4b61d1348 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-01.md | 361 | 84a4557cb4ab940de2f101c847fc14f26075bd0c4cdc245f5472b81ce0ffe345 |
| docs/testing/BBD-WAL-009-RETAINED-SPEND-VALIDATION-RESUME-01.md | 563 | 323eccb0428b6e8ae530c1f3131d02c6620fc16161a57ad44812ea06c8bf396b |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-VALIDATION-01.md | 506 | 0e9616da6def1a409e5bd2126f912394b321e5a1464ae5011a10ad901a1bb1dc |
| docs/testing/BBD-WAL-009-NATIVE-APP-EXPECTED-RED-01.md | 187 | 7141eedc9a1529618689c64684863c7337b1d64b626d62c5c7fa850cf9750acb |
| docs/testing/BBD-WAL-009-NATIVE-LAYOUT-VALIDATION-01.md | 357 | db643c2c36f4b1ac6502b13216f7b3e59d8b70553d937ff7d3264264f5c6da9c |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-EXPECTED-RED-01.md | 199 | 5acc080cb82e7dcff1d847b770f258ffb90538aed8753c5752b51405f85a0fea |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-01.md | 168 | 89316ddd1fea15f9c7e2c147554f68b408f59da6936038b99fee71f2991ec26a |
| docs/testing/BBD-WAL-009-DECODED-EFFECTS-VALIDATION-RESUME-01.md | 348 | 5ecb076a3ba4f358d7a1376d083bab56e1e30fa819e5be87374d8f3d89204acd |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-EXPECTED-RED-01.md | 180 | 6c6d0548c331e543b07ca3cfc64dffaa33e225512d46fbcbe65a089e11499d02 |
| docs/testing/BBD-WAL-009-CLEANUP-OBSERVATIONS-VALIDATION-01.md | 567 | bf36cb97180535710df247b9c60511161442245dfe64f8962f032a678ec1d970 |
| docs/testing/BBD-WAL-009-CLEANUP-LIFECYCLE-VALIDATION-01.md | 494 | f4311ef9111e7876be27b12da284148b2c46612fdde6cd4bae88e555e9295567 |
| docs/testing/BBD-WAL-009-PIPELINE-PANIC-EXPECTED-RED-01.md | 103 | fee847fa919b7414c8e5a43161406d7d837cd4436f403ecbd0dab8a9b28386c6 |
| docs/testing/BBD-WAL-009-PIPELINE-PANIC-VALIDATION-01.md | 431 | 4f7f4be2a30902ee752b562a0eb253081bfd6aa0d01384b316f09e5dbc407646 |

Reviewer publication scope for this authorization: this handoff,
docs/handoff/CURRENT_TASK.md and tickets/BBD-WAL-009.md only. Reviewer does not
integrate the source/evidence drop or execute the scan. High is sufficient for the
next bounded checkpoint/transcript review; no further XHigh decision is needed now.
