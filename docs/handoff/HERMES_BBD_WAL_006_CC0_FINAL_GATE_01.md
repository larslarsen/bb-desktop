# Hermes Handoff — BBD-WAL-006 CC0 and Final Gate 01

You are **Jr Dev — Hermes** using only the free configured Nous route. Validate and integrate the
accepted test-first CC0-1.0 policy correction, finish the stopped final local gate from its
license step, and write exact consolidated evidence. Do not alter accepted source beyond the three
authorized dirty policy paths.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the CC0 test/expected-red/
production handoffs, the stopped final-local-gate handoff, all prior final gate evidence, this
handoff, all three dirty paths, both lockfiles, `.gitleaksignore`, and `CURRENT_TASK.md`.

## Preconditions

Record Hermes version/provider/model. Require `HEAD == origin/master`, exactly these three dirty
paths, clean `git diff --check`, ext4 Cargo directories, and:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,527 | `d42f58a2487c64ad59d282d93759a6b156239e0a125d399f7821f278a9087faa` |
| `scripts/security-policy.js` | 2,483 | `73a9c99b306259823c9c2931946dd8a974d8fa2c03efda62021f3f9c8c3b7927` |
| `deny.toml` | 40 | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` |

Require exact unchanged lock and scanner identities from the stopped final gate and installed
`cargo-deny 0.20.2`.

The accepted earlier portion of the same final local gate at parent `81f047fe` is:

- cargo-deny 0.20.2 installation succeeded;
- fmt passed with no mutation;
- all 127 Rust integration tests passed, zero failed/ignored;
- all-targets/all-features Clippy passed with warnings denied;
- native-ui check passed;
- npm build passed;
- npm test passed: social, Electron 19, policy 75, wallet 48, protocol 11, supervisor 11,
  preload 6;
- direct repository policy passed;
- npm audit passed with zero vulnerabilities;
- cargo-audit passed with zero vulnerabilities and only the existing non-denying
  `atomic-polyfill` RUSTSEC-2023-0089 unmaintained warning;
- cargo-deny advisories/bans/sources passed but licenses stopped only on the three exact pinned
  CC0-1.0 crates. No evidence/integration was created.

The accepted test-first expected red at parent `47bad440` is exactly 74 passed/one failed because
the frozen test expected CC0-1.0 while the production policy constant and deny allow-list omitted
it. Do not repeat the stale inverted wording from the executor summary.

## Exact commands

Run once each in order; stop on first mismatch.

1. `node test/securityPolicy.node.js`
   — exit 0, exactly 75 `ok`, zero `not ok`, exact final line
   `BitBook security policy tests passed (75).`
2. `node scripts/security-policy.js`
   — exit 0, exact line `BitBook desktop security policy checks passed.`
3. `npm run test:security`
   — exit 0, Electron security 19 and policy 75, zero failure.
4. `/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources`
   — exit 0; the three exact CC0-1.0 crates accepted; no advisory/license/source/ban denial;
   no exception/ignore/bypass.
5. `target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .`
   — exit 0, no unsuppressed finding.
6. `target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .`
   — exit 0, no unsuppressed finding.

After command 6 require both lock hashes unchanged, exact three dirty identities unchanged, and
clean `git diff --check`.

## Evidence and integration

Only on exact success, use `apply_patch` to:

1. create `docs/testing/BBD-WAL-006-CC0-LICENSE-GATE-01.md` with primary local crate-manifest
   license evidence, expected red, exact three-line policy result, green Node/cargo-deny results,
   closed allow-list/no exceptions, identities, and final state;
2. create `docs/testing/BBD-WAL-006-FINAL-LOCAL-GATE-01.md` consolidating the accepted stopped-gate
   commands 0–9 and this resume's commands 1–6, exact counts/results/tool versions, audits/scans,
   all prior prepare/policy/falsification evidence, immutable locks/source, and final Git state;
3. correct only the stale first-command final line in
   `docs/testing/BBD-WAL-006-FINAL-POLICY-GATE-01.md` from
   `BitBook desktop security policy tests passed (75).` to
   `BitBook security policy tests passed (75).`;
4. update only the leading current-task block to `FINAL LOCAL GATE COMPLETE — REVIEWER ACCEPTANCE
   PENDING`, no authorized actors, and list this handoff plus both new evidence files completed.

Stage exactly the three dirty policy paths plus those four documentation paths. Commit exactly
`fix: allow reviewed CC0 Zcash transitives`, push `master`, and prove clean worktree/index with
`HEAD == origin/master`.

Do not run any other command; do not modify wallet source, locks, manifests, workflows, package,
scanner config, or prior evidence beyond the one exact line; do not amend/rebase/merge/force-push,
clean, delete, or access wallet/node/device/mainnet/sign/prove/extract/broadcast. Any mismatch
stops without evidence/integration.
