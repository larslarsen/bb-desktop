# Current Task

Ticket: BBD-WAL-006

State: DEPENDENCY TEST SOURCE ACCEPTED — FOCUSED EXPECTED RED AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Accepted correction actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `363c0046`

Test-source review:
[BBD-WAL-006-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md)

Format-correction review:
[BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md](../testing/BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md)

Active handoff:
[CODEX_LUNA_BBD_WAL_006_DEPENDENCY_TEST_RED_01.md](CODEX_LUNA_BBD_WAL_006_DEPENDENCY_TEST_RED_01.md)

Dependency test-source review:
[BBD-WAL-006-DEPENDENCY-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-DEPENDENCY-TEST-SOURCE-REVIEW-01.md)

Dependency-resolution review:
[BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-01.md](../testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-01.md)

Architecture:
[BBD-WAL-006-UPSTREAM-REVIEW.md](../architecture/BBD-WAL-006-UPSTREAM-REVIEW.md)

Ticket: [BBD-WAL-006.md](../../tickets/BBD-WAL-006.md)

BBD-WAL-004 remains complete and reviewer-accepted at `e8894a44`. BBGO-PAY-001 remains
complete in `../bb-go` at production `6bbb0629` and final evidence `801f5d55`.

The exact eight uncommitted Phase-A test/manifest paths passed semantic source review and
the corrected formatter gate exited 0. The next lock-resolution command stopped before
lock mutation: Zcash's exact `bip32 0.6.0-pre.1` requires prerelease
`hmac 0.13.0-pre.4` and `sha2 0.11.0-pre.4`, which Cargo cannot resolve beside the
accepted stable WAL-004 `hkdf 0.13.0`/`sha2 0.11.0` graph.

Sol's one-path dependency test correction is accepted at 2,374 lines and SHA-256
`f4f6defd5d55212c480a7a1a87d37edea85786b6125cad29467ae5e1f3480ee4`, with all 73
test cases retained. Luna is authorized to run only the focused existing WAL-004
manifest-policy test and integrate the test/evidence/current-task paths on the exact old
production-map failure. Manifest, production policy, Rust source/tests, lockfile,
fixtures, and all broader execution remain frozen. The Phase-B fixture resume is
suspended until this red is reviewer-accepted, followed by a separate manifest/policy
correction and exact custody-vector gate.

No production source or policy implementation is authorized. XHigh must accept the
resolved graph, fixture bytes/provenance, and exact expected-red evidence before a
separate Phase-C source handoff exists. No broader tests, falsification, live endpoint,
mainnet, signing, proving, extraction, broadcast, hardware, Electron, package, SBOM, or
other-repository work is authorized in this phase.

This slice remains offline and synthetic. `../go-ipfs` is deprecated and receives no
wallet work.
