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
