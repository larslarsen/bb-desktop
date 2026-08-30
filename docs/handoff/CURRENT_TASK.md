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
