# Current Task

Ticket: BBD-SEC-001

State: AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

[BBD-SEC-001](../../tickets/BBD-SEC-001.md) is the only authorized implementation. It
hardens and tests the maintained Electron entry point, adds pinned pull-request/manual
security gates, makes routine CI package-free and path-filtered, and adds manual SBOM-only
evidence. Native package builds, cross-platform fuse work, the inherited marketplace,
and deprecated `go-ipfs` are out of scope.

Complete source and integration prompts are preserved in
[GROK_BUILD_BBD_SEC_001.md](GROK_BUILD_BBD_SEC_001.md) and
[CODEX_LUNA_BBD_SEC_001.md](CODEX_LUNA_BBD_SEC_001.md). No production work begins until
this governance baseline is committed.

Grok delivered the test-first source drop without execution. Reviewer accepted the
runtime/CSP and SBOM design but rejected the Gitleaks Action because it does not scan full
merge history, and found the Windows build script missing from routine path filters.
Only [Correction 1](GROK_BUILD_BBD_SEC_001_CORRECTION_01.md) is now authorized. Luna must
not execute until the corrected source report is preserved and reviewed.

Correction 1 was delivered test-first and reviewer source inspection accepted the pinned
complete-history CLI scan and both Windows path filters. Codex Luna now owns the exact
integration sequence in [its handoff](CODEX_LUNA_BBD_SEC_001.md): verify hashes,
reconstruct bounded red, restore and falsify, then run scanners and manual SBOM commands
without native packaging. Any failure or finding stops before later work and Git.

Luna verified hashes, reconstructed/restored red, and reached green: Electron 12/12;
policy 47/48. The sole failure is a test oracle that does not match the checker's plural
“summaries” rejection. No later gate ran. Only the one-test-file correction in
[Correction 2](GROK_BUILD_BBD_SEC_001_CORRECTION_02.md) is authorized; Luna resumes after
its report is preserved and reviewed.

Correction 2 was delivered as a one-assertion semantic regex fix and reviewer inspection
accepted the new test-file hash. Luna may now verify that hash, rerun the complete policy
suite, and continue at the ticketed falsification sequence only if all tests pass.

Luna subsequently completed/restored the bounded red and falsification work and passed
Electron 12/12, corrected policy 46/46, the policy checker, and npm audit with zero
vulnerabilities. Electronegativity then hit a network boundary: sandboxed npm returned
`EAI_AGAIN`, while an escalated retry remained silent indefinitely and was interrupted
without changing repository, evidence, artifacts, staging, commits, or the remote.

The durable resume contract is
[Network Boundary 01](CODEX_LUNA_BBD_SEC_001_NETWORK_01.md). Luna resumes at the exact
pinned Electronegativity command with a disk-backed npm cache and finite foreground
timeout. Timeout, network/tool error, or any finding stops the ticket; only a zero-finding
result permits the remaining acceptance and manual-SBOM sequence.

The bounded command later completed with network access but printed three findings and
returned exit 0. Reviewer diagnostics established a broken gate: upstream marks
Electronegativity unmaintained, its release model does not recognize Electron 44, and its
CLI does not fail on findings. A maintained-app-only diagnostic also conflicts with the
intentional configurable daemon endpoint and owner-approved dev sandbox workaround.

Integration is stopped before every later gate and Git. Grok Build owns design review
only under [Correction 03](GROK_BUILD_BBD_SEC_001_CORRECTION_03.md); implementation is not
authorized until the reviewer records the exact correction.

Grok's design review is now reviewer-accepted and fully preserved in Correction 03. The
obsolete non-blocking scanner concept will be removed test-first; it is replaced by the
existing executable Electron boundary/policy suite plus a maintained-source sink test,
while blocking npm audit and complete-history Gitleaks remain. No runtime, CSP, package,
lockfile, SBOM, routine CI, packaging, or inherited-source change is authorized. Grok may
author only the four correction paths and must run nothing; Luna remains stopped until
the reviewer accepts Grok's exact hashes.

Correction 03 source is now delivered and reviewer-accepted. Independent hashes and
counts match the durable source report: Electron 13 tests, policy 47 tests, and only the
checker plus security workflow production paths. Codex Luna may resume under the ticket:
correction-specific bounded red and restore, obsolete-scanner reintroduction
falsification, complete corrected acceptance from the beginning, then the unchanged
manual SBOM sequence. Any failure or Gitleaks finding stops before Git.

Luna's exact resumed integration contract is
[Correction 03 Integration](CODEX_LUNA_BBD_SEC_001_CORRECTION_03.md). It prevents repeat
work, supersedes only the obsolete scanner command, and preserves every disk-path,
no-cleanup, no-native-build, hard-stop, evidence, commit, and reviewer-return rule.

Luna completed every corrected gate through the security test suite, including the
required red/restore and obsolete-scanner falsification. Gitleaks then stopped integration
after scanning 4,549 commits and reporting 10 findings. No SBOM/evidence/Git work ran.
Reviewer owns redacted metadata-only triage under
[Gitleaks Triage 01](BBD_SEC_001_GITLEAKS_TRIAGE_01.md); no baseline, suppression, repair,
or resume is yet authorized.

Reviewer triage verified 100-percent redaction, eight unique upstream-history
fingerprints (10 emitted findings), and two retained current-tree matches in inactive
marketplace utility modules. The current values cannot be baselined away. Grok Build now
owns design review only under [Correction 04](GROK_BUILD_BBD_SEC_001_CORRECTION_04.md) for
safe neutralization, an exact commit-fingerprint ratchet, and a required current-tree
scan. No source edit or integration resume is authorized.

Correction 04 design is reviewer-accepted and fully preserved in its handoff. Grok may
now author exactly one test file first (policy total 50), then only the exact eight-line
commit fingerprint ratchet, the two inherited loader neutralizations, the policy checker,
and security workflow. Full-history Gitleaks remains unchanged and a blocking dir scan
is added. Grok runs nothing; Luna stays stopped until exact hashes are reviewed.
