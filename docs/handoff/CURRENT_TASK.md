# Current Task

Ticket: BBD-WAL-006

State: FIXTURE AND EXPECTED RED RECORDED — REVIEW REQUIRED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Accepted correction actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `363c0046`

Test-source review:
[BBD-WAL-006-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md)

Format-correction review:
[BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md](../testing/BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md)

Active handoff:
[CODEX_LUNA_BBD_WAL_006_FIXTURE_RED_EVIDENCE_INTEGRATION_01.md](CODEX_LUNA_BBD_WAL_006_FIXTURE_RED_EVIDENCE_INTEGRATION_01.md)

[BBD-WAL-006-EXPECTED-RED-01.md](../testing/BBD-WAL-006-EXPECTED-RED-01.md)

Fixture/expected-red result review:
[BBD-WAL-006-FIXTURE-EXPECTED-RED-REVIEW-01.md](../testing/BBD-WAL-006-FIXTURE-EXPECTED-RED-REVIEW-01.md)

Fixture reorg format-correction review:
[BBD-WAL-006-FIXTURE-REORG-FORMAT-CORRECTION-REVIEW-01.md](../testing/BBD-WAL-006-FIXTURE-REORG-FORMAT-CORRECTION-REVIEW-01.md)

Fixture reorg format review:
[BBD-WAL-006-FIXTURE-REORG-FORMAT-REVIEW-01.md](../testing/BBD-WAL-006-FIXTURE-REORG-FORMAT-REVIEW-01.md)

Fixture reorg source review:
[BBD-WAL-006-FIXTURE-REORG-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-FIXTURE-REORG-SOURCE-REVIEW-01.md)

Fixture-generation review:
[BBD-WAL-006-FIXTURE-GENERATION-REVIEW-01.md](../testing/BBD-WAL-006-FIXTURE-GENERATION-REVIEW-01.md)

Accepted dependency correction gate review:
[BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-REVIEW-02.md](../testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-REVIEW-02.md)

Dependency correction gate review:
[BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-REVIEW-01.md](../testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-REVIEW-01.md)

AEAD API production-source review:
[BBD-WAL-006-AEAD-API-PRODUCTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-AEAD-API-PRODUCTION-SOURCE-REVIEW-01.md)

AEAD API expected-red review:
[BBD-WAL-006-AEAD-API-TEST-RED-REVIEW-02.md](../testing/BBD-WAL-006-AEAD-API-TEST-RED-REVIEW-02.md)

Complete AEAD API test-source review:
[BBD-WAL-006-AEAD-API-TEST-SOURCE-REVIEW-02.md](../testing/BBD-WAL-006-AEAD-API-TEST-SOURCE-REVIEW-02.md)

API red/correction review:
[BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-05.md](../testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-05.md)

AEAD API test-source review:
[BBD-WAL-006-AEAD-API-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-AEAD-API-TEST-SOURCE-REVIEW-01.md)

Resolved-graph/API review:
[BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-04.md](../testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-04.md)

AEAD production-source review:
[BBD-WAL-006-AEAD-PRODUCTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-AEAD-PRODUCTION-SOURCE-REVIEW-01.md)

AEAD expected-red review:
[BBD-WAL-006-AEAD-TEST-RED-REVIEW-01.md](../testing/BBD-WAL-006-AEAD-TEST-RED-REVIEW-01.md)

AEAD test-source review:
[BBD-WAL-006-AEAD-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-AEAD-TEST-SOURCE-REVIEW-01.md)

Third dependency-resolution review:
[BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-03.md](../testing/BBD-WAL-006-DEPENDENCY-RESOLUTION-REVIEW-03.md)

Argon2 production-source review:
[BBD-WAL-006-ARGON2-PRODUCTION-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-ARGON2-PRODUCTION-SOURCE-REVIEW-01.md)

Argon2 expected-red review:
[BBD-WAL-006-ARGON2-TEST-RED-REVIEW-01.md](../testing/BBD-WAL-006-ARGON2-TEST-RED-REVIEW-01.md)

Argon2 test-source review:
[BBD-WAL-006-ARGON2-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-ARGON2-TEST-SOURCE-REVIEW-01.md)

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

The corrected graph resolved. Luna's first API expected-red run was rejected because
the reviewer contract omitted stable AEAD 0.10's GenericArray slice-constructor errors;
no evidence/integration followed. Sol completed the independent vector test correction,
which XHigh accepted at 394 lines and SHA-256
`a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b`.
The exact locked/offline custody target then exited 101 with zero tests and only the
nine accepted frozen-production API diagnostics. Sol's exact `vault.rs` compatibility
drop is reviewer-accepted at 759 lines and SHA-256
`89d8ada8ad7050910a92a9daa38f9d93a18b86c4b7d1bde7a7e9b4a8adf8b62b`.
The dependency/custody commands passed and were integrated at `6f0a5305`; the corrected
supply-chain inventory is complete at `7ee9eb4e` and reviewer-accepted. The first fixture
run exposed a test-only misuse of upstream wallet rewind before scan; no fixture/evidence
was created. Sol's direct parent-state reorg correction is reviewer-accepted at 928 lines
and SHA-256 `4b1efec59f81761e2c713587c0a4f3e7b8c545f7b85cc35c90949c5dedbca4bc`.
The formatter found one mechanical wrap before execution; Sol's exact correction is
reviewer-accepted at SHA-256
`40cc2b56132b42a765c86482e9915b0422adc0154c1e2edcfda4623760ec5d09`.
The formatter, two fixture runs, exact freeze, and Node red succeeded. The focused Rust
red contained only the absent production `zec` module, classified as `E0433` and `E0432`;
the reviewer accepts the result without rerun. Luna may integrate only the accepted
tests/fixture/evidence under the active handoff. ZEC production remains frozen.

No ZEC production source or policy implementation is authorized. The sole production
exception is the accepted `vault.rs` AEAD API compatibility edit in the active gate.
XHigh must accept the resolved graph, fixture bytes/provenance, and exact expected-red
evidence before a separate Phase-C ZEC source handoff exists. No broader tests,
falsification, live endpoint, mainnet, signing, proving, extraction, broadcast, hardware,
Electron, package, SBOM, or other-repository work is authorized in this phase.

This slice remains offline and synthetic. `../go-ipfs` is deprecated and receives no
wallet work.

Dependency correction expected-red evidence:
[BBD-WAL-006-DEPENDENCY-CORRECTION-EXPECTED-RED-01.md](../testing/BBD-WAL-006-DEPENDENCY-CORRECTION-EXPECTED-RED-01.md)

AEAD correction expected-red evidence:
[BBD-WAL-006-AEAD-CORRECTION-EXPECTED-RED-01.md](../testing/BBD-WAL-006-AEAD-CORRECTION-EXPECTED-RED-01.md)

AEAD API correction expected-red 02 evidence:
[BBD-WAL-006-AEAD-API-CORRECTION-EXPECTED-RED-02.md](../testing/BBD-WAL-006-AEAD-API-CORRECTION-EXPECTED-RED-02.md)

Argon2 correction expected-red evidence:
[BBD-WAL-006-ARGON2-CORRECTION-EXPECTED-RED-01.md](../testing/BBD-WAL-006-ARGON2-CORRECTION-EXPECTED-RED-01.md)

Dependency correction gate evidence:
[BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-01.md](../testing/BBD-WAL-006-DEPENDENCY-CORRECTION-GATE-01.md)
