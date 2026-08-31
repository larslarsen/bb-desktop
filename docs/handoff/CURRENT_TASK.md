# Current Task

Ticket: BBD-WAL-006

State: SECOND DEPENDENCY CONTRADICTION — ARGON2 TEST CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Accepted correction actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `363c0046`

Test-source review:
[BBD-WAL-006-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md)

Format-correction review:
[BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md](../testing/BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md)

Active handoff:
[CODEX_SOL_BBD_WAL_006_DEPENDENCY_TEST_CORRECTION_02.md](CODEX_SOL_BBD_WAL_006_DEPENDENCY_TEST_CORRECTION_02.md)

Second dependency-resolution review:
[BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-02.md](../testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-02.md)

Dependency production-source review:
[BBD-WAL-006-DEPENDENCY-PRODUCTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-DEPENDENCY-PRODUCTION-SOURCE-REVIEW-01.md)

Dependency expected-red review:
[BBD-WAL-006-DEPENDENCY-TEST-RED-REVIEW-01.md](../testing/BBD-WAL-006-DEPENDENCY-TEST-RED-REVIEW-01.md)

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

The first corrected test/red and manifest/policy source are accepted. Luna proved the
production map now matches, then the next lock attempt stopped before mutation on a
second published same-line conflict: direct `argon2 0.6.0` pulls stable `digest 0.11`
through `blake2 0.11`, while fixed Zcash `bip32` requires exact prerelease
`digest 0.11.0-pre.9`. Sol may change only the Node test-side Argon2 expectation and its
exact mutations under the active handoff. Manifest, production policy, Rust source/tests,
lockfile, fixtures, execution, evidence, integration, and Git are frozen. Phase-B fixture
generation remains suspended.

No production source or policy implementation is authorized. XHigh must accept the
resolved graph, fixture bytes/provenance, and exact expected-red evidence before a
separate Phase-C source handoff exists. No broader tests, falsification, live endpoint,
mainnet, signing, proving, extraction, broadcast, hardware, Electron, package, SBOM, or
other-repository work is authorized in this phase.

This slice remains offline and synthetic. `../go-ipfs` is deprecated and receives no
wallet work.

Dependency correction expected-red evidence:
[BBD-WAL-006-DEPENDENCY-CORRECTION-EXPECTED-RED-01.md](../testing/BBD-WAL-006-DEPENDENCY-CORRECTION-EXPECTED-RED-01.md)
