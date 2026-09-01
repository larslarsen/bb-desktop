# BBD-WAL-006 Final Local Gate 01 Evidence

Ticket: BBD-WAL-006

State: FINAL LOCAL GATE 01 COMPLETE — REVIEWER ACCEPTANCE PENDING

Execution timestamp: 2026-09-01

Protected governance parent: `704b11caf135dec04c72232a509db540c71a18be` (commit containing HERMES_BBD_WAL_006_CC0_FINAL_GATE_01.md)

Filesystem type: ext4 (disk-backed, /dev/mapper/ubuntu--vg-ubuntu--lv)

Hermes identity:
- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Accepted stopped-gate portion (commands 0–9, parent 81f047fe)

The accepted earlier portion of the same final local gate at parent `81f047fe` is:
- cargo-deny 0.20.2 installation succeeded;
- fmt passed with no mutation;
- all 127 Rust integration tests passed, zero failed/ignored;
- all-targets/all-features Clippy passed with warnings denied;
- native-ui check passed;
- npm build passed;
- npm test passed: social, Electron 19, policy 75, wallet 48, protocol 11, supervisor 11, preload 6;
- direct repository policy passed;
- npm audit passed with zero vulnerabilities;
- cargo-audit passed with zero vulnerabilities and only the existing non-denying `atomic-polyfill` RUSTSEC-2023-0089 unmaintained warning;
- cargo-deny advisories/bans/sources passed but licenses stopped only on the three exact pinned CC0-1.0 crates. No evidence/integration was created.

## Resume portion (commands 1–6, this execution)

Resume parent: `704b11caf135dec04c72232a509db540c71a18be`. Preconditions verified:
- HEAD == origin/master, exactly three dirty paths, clean `git diff --check`, ext4 Cargo directories.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,527 | `d42f58a2487c64ad59d282d93759a6b156239e0a125d399f7821f278a9087faa` |
| `scripts/security-policy.js` | 2,483 | `73a9c99b306259823c9c2931946dd8a974d8fa2c03efda62021f3f9c8c3b7927` |
| `deny.toml` | 40 | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` |

Immutable lock and scanner identities:

| Path | SHA-256 |
| --- | --- |
| `package-lock.json` | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` |
| `wallet-broker/Cargo.lock` | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |
| `.gitleaksignore` | `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b` |

Tool versions: cargo 1.98.0, cargo-deny 0.20.2, gitleaks v8.30.1.

### Exact execution (each run once, in order, no network)

1. `node test/securityPolicy.node.js`
   - Exit: 0. Exactly 75 `ok`, zero `not ok`, final line `BitBook security policy tests passed (75).`

2. `node scripts/security-policy.js`
   - Exit: 0. Exact line `BitBook desktop security policy checks passed.`

3. `npm run test:security`
   - Exit: 0. Electron security exactly 19 passed (`BitBook electron security tests passed (19).`), policy exactly 75 passed (`BitBook security policy tests passed (75).`), zero failure.

4. `/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources`
   - Exit: 0. Advisories ok, bans ok, licenses ok, sources ok. The three exact CC0-1.0 crates (`bounded-vec 0.9.0`, `secp256k1 0.29.1`, `secp256k1-sys 0.10.1`) accepted; no advisory/license/source/ban denial; no exception/ignore/bypass.

5. `target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .`
   - Exit: 0. 4836 commits scanned, ~27.65 MB in 2.38s, no leaks found.

6. `target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .`
   - Exit: 0. ~1.44 GB in 10.2s, no leaks found.

### Post-conditions (all exact)

After command 6 both lockfile hashes unchanged, exact three dirty identities unchanged, clean `git diff --check` (exit 0).

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,527 | `d42f58a2487c64ad59d282d93759a6b156239e0a125d399f7821f278a9087faa` |
| `scripts/security-policy.js` | 2,483 | `73a9c99b306259823c9c2931946dd8a974d8fa2c03efda62021f3f9c8c3b7927` |
| `deny.toml` | 40 | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` |
| `package-lock.json` | — | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` |
| `wallet-broker/Cargo.lock` | — | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |

## Audits and scans

- npm audit: zero vulnerabilities (stopped-gate command 8).
- cargo-audit: zero vulnerabilities/advisory denial; only existing non-denying `atomic-polyfill` RUSTSEC-2023-0089 unmaintained warning (stopped-gate command 9).
- cargo-deny advisories/bans/licenses/sources: all ok (resume command 4).
- gitleaks git: no leaks found (resume command 5).
- gitleaks dir: no leaks found (resume command 6).

## Prior accepted evidence

The following prior prepare/policy/falsification/CC0 evidence is accepted and incorporated by reference:
- `docs/testing/BBD-WAL-006-PREPARE-ROLLBACK-GATE-01.md` — 127 integration tests, byte-exact DB rollback, one unsigned real Ironwood action, one IO-finalized signed padding action, negative capability.
- `docs/testing/BBD-WAL-006-FALSIFICATION-GATE-01.md` — five independent falsifications (transparent/Sapling UA, chain-continuity, Orchard value, v6 artifact, handle-across-lock); no mutation bytes committed.
- `docs/testing/BBD-WAL-006-FINAL-POLICY-GATE-01.md` — closed WAL-004/WAL-006 Rust source inventories, forbidden-feature/authority enforcement, legitimate offline upstream allowance (corrected first-command final line).
- `docs/testing/BBD-WAL-006-CC0-LICENSE-GATE-01.md` — three exact CC0-1.0 Zcash transitives, closed allow-list, no exceptions.

## Immutable source and locks

All wallet-broker production source, manifests, and lockfiles are unchanged from the stopped final gate. No wallet source, lock, manifest, workflow, package, or scanner configuration was modified.

## Final Git state

Head commit: `704b11caf135dec04c72232a509db540c71a18be`
Push: master
Final state: HEAD == origin/master, clean index, clean tracked worktree.

The reviewer alone accepts the result and authorizes the next task.
