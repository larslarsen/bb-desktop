# Current Task

Ticket: BBD-WAL-004

State: CI GATE GREEN RUN 02 — CORRECTED FALSIFICATION RESUME AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Governance parent: the commit containing this handoff

Expected-red commit: `61d2a239a89384885a95cf353f4d3ccc319670a7`

Active handoff: [CODEX_LUNA_BBD_WAL_004_CI_GATE_GREEN.md](CODEX_LUNA_BBD_WAL_004_CI_GATE_GREEN.md)

The custody core remains green and independently falsified by its seven original
mutations. The accepted CI-gate test and three-file production corrections remain the
only four unstaged paths at their frozen hashes.

Green Run 01 exposed and stopped on three no-op mutation fixtures; Sol's exact test-only
correction is reviewer-accepted. Green Run 02 then proved all 69 cases green. Its first
falsification removed the path-specific rejection, but the generic exact-nine terminal
check still rejected the bad fingerprint, so the strict test failed on a diagnostic
mismatch rather than a real acceptance bypass. Luna restored the source exactly and
stopped before any later gate or integration. The full record is
[BBD-WAL-004-CI-GATE-GREEN-RUN-02.md](../testing/BBD-WAL-004-CI-GATE-GREEN-RUN-02.md).

The amended handoff now temporarily changes only the wrong-path branch's throw to a
fail-open return. The strict ratchet test must be the sole failure because the bad path
is actually accepted. Luna must restore the exact production hash, repeat the 69-case
green proof, then continue the remaining local gates, four-path integration/push, and
only the manual non-packaging Security/SBOM workflows. No source/test change is
authorized.

No platform package build is authorized or required. Grok Build remains available and
queued for the independent `../bb-go` exchange-rate/provider work after this desktop
security correction. `../go-ipfs` is deprecated and receives no wallet work.
