# Current Task

Ticket: BBD-WAL-004

State: CI GATE PRODUCTION SOURCE ACCEPTED — LUNA GREEN AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Governance parent: the commit containing this handoff

Reviewed CI baseline: `e19af1a50ebc2a6b1f46e504fa02dd168358dbb0`

Production commit: `0e42fb4b477cfe76757ed207d3a561270b9e9efe`

Expected-red commit: `61d2a239a89384885a95cf353f4d3ccc319670a7`

Active handoff: [CODEX_LUNA_BBD_WAL_004_CI_GATE_GREEN.md](CODEX_LUNA_BBD_WAL_004_CI_GATE_GREEN.md)

The encrypted custody core is locally green, integrated, pushed, and independently
falsified by all seven required temporary mutations. GitHub Social client run
`33357371137` passed with package jobs skipped.

Manual Security run `33359184973` failed only on the reviewer-published synthetic HKDF
vector's Gitleaks false positive. Manual SBOM run `33359223628` failed because the Rust
document omitted optional native direct dependencies under default features. The
authoritative diagnosis, accepted test source, exact 66-green/3-red execution, and red
acceptance are linked from the active ticket and preserved under `docs/testing/`.

The reviewer relabeled only the live synthetic vector from `key    =` to `expand =`
without changing its hex bytes. Sol's exact three-file correction is accepted in
[BBD-WAL-004-CI-GATE-PRODUCTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-004-CI-GATE-PRODUCTION-SOURCE-REVIEW-01.md).
It adds only the exact historical fingerprint, matching nine-entry policy semantics,
and `--all-features` to the Rust CycloneDX command. Tests, validators, wallet source,
dependencies, other workflows, and package behavior remain unchanged.

Luna may now run only the bounded local green gate, integrate the exact drop, commit and
push it, then dispatch and wait for only the manual Security and SBOM workflows under the
active handoff. Final acceptance requires both local pinned Gitleaks modes and both fresh
remote runs to succeed. No platform package build is authorized or required.

Grok Build is available and queued for the independent `../bb-go`
exchange-rate/provider work after this desktop security correction. `../go-ipfs` is
deprecated and receives no wallet work.
