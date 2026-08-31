# Current Task

Ticket: BBD-WAL-006

State: AEAD API EXPECTED RED 02 ACCEPTED — PRODUCTION CORRECTION AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Accepted correction actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Authorized integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `363c0046`

Test-source review:
[BBD-WAL-006-TEST-SOURCE-REVIEW-01.md](../testing/BBD-WAL-006-TEST-SOURCE-REVIEW-01.md)

Format-correction review:
[BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md](../testing/BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md)

Active handoff:
[CODEX_SOL_BBD_WAL_006_AEAD_API_PRODUCTION_CORRECTION_01.md](CODEX_SOL_BBD_WAL_006_AEAD_API_PRODUCTION_CORRECTION_01.md)

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
nine accepted frozen-production API diagnostics. Sol may change only
`wallet-broker/src/vault.rs` under the active handoff. Manifest, policy, lock, ZEC tests,
fixtures, execution, evidence, integration, and Git remain frozen.

No ZEC production source or policy implementation is authorized. The sole production
exception is the exact `vault.rs` AEAD API compatibility edit in the active handoff.
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
