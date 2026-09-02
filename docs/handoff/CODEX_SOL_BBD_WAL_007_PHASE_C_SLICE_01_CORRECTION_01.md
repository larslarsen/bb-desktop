# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 1 Correction 01

Status: AUTHORIZED — THREE-PATH CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Review: `../testing/BBD-WAL-007-SLICE-01-SOURCE-REVIEW-01.md`

## Authorized paths

Edit only:

- `wallet-broker/src/xmr/distribution.rs` — 864 lines, SHA-256
  `6ff5b1d37f2dd8d073948eae61e5747f9d95f5b08ce09231c25171f100fa2e80`;
- `scripts/security-policy.js` — 2,676 lines, SHA-256
  `a9274004a2fd80674f833fe493e7007ff3e90fe08d1c7d3345648f1dec185da3`;
- `test/securityPolicy.node.js` — 3,067 lines, 86 tests, SHA-256
  `c7d8f7a16b58ffa4224ee9975829d394f8a290b72fde55d14942580cf1c6905c`.

The other six Slice-1 source files and every manifest/lock/test/evidence/governance path
are frozen at Source Review 01. Every other repository is read-only.

## Distribution correction

Add the exact public API already consumed by the accepted real gate:

```text
InstallationVerifier::linux_x86_64()
InstallationVerifier::verify_selected(&Path) -> Result<VerifiedExecutable, XmrError>
```

It must validate path syntax, supported platform, final-component metadata,
effective-user executability, exact size, bounded SHA-256, and exact bounded version by
delegating to the existing centralized production verification implementation. It must
not persist a selection, scan/fallback/download, duplicate the algorithm, disclose the
path, or add process/RPC/account behavior. On a non-Linux-x86-64 build it fails
`UNAVAILABLE` before filesystem/process effects.

Make `VerifiedExecutable::selected_path` crate-private. Do not otherwise alter
distribution behavior or its test-support contract.

## Exact cumulative inventory correction

Keep `WAL007_PHASE_C_XMR_PATHS` as the exact final nine-file inventory. Add one closed
ordered collection of the only accepted cumulative XMR inventories:

1. Slice 1: `xmr.rs`, `distribution.rs`, `model.rs`, `test_support.rs`;
2. Slice 2: Slice 1 plus `process.rs`;
3. Slice 3: Slice 2 plus `rpc.rs`;
4. Slice 4: Slice 3 plus `account.rs` and `store.rs`;
5. Slice 5: Slice 4 plus `receiver.rs`, producing the frozen final inventory.

All paths use the existing `wallet-broker/src/...` form and each inventory is in the
same sorted order returned by `collectWal007RustSourcePaths`. The existing Node test must
accept only Phase A's empty inventory or one of those five exact inventories. It must
prove the stages are unique, strictly cumulative in that order, contain no unreviewed
path, and end at `WAL007_PHASE_C_XMR_PATHS`. Keep the test count at 86 and preserve every
other WAL-007 authority, runtime scan, negative fixture, and final inventory assertion.

In `scripts/security-policy.js`, extend `checkRustWalletSourceInventory` to accept only
the exact existing WAL-006 top-level inventory plus
`wallet-broker/src/xmr.rs`, while continuing to accept the two historical shapes and
reject duplicates, missing paths, every other top-level file, and every malformed value.
Add a call from the existing WAL-007 Node test proving that exact cumulative top-level
shape is accepted. Do not change dependency, SBOM, ZEC, workflow, scanner, license, or
other policy behavior.

## Prohibited actions and delivery

Do not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. Do not stage, commit, or push. Do not begin
Slice 2.

Stop after the three-path correction. Report exact line counts/hashes, the five frozen
inventories, the verifier delegation point, unchanged 86-test count, and scope/command
confirmation. Reviewer acceptance is required before Hermes execution.
