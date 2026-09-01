# BBD-WAL-006 CC0 License Gate 01 Evidence

Ticket: BBD-WAL-006

State: CC0 LICENSE GATE 01 COMPLETED

Execution timestamp: 2026-09-01

Protected governance parent: `704b11caf135dec04c72232a509db540c71a18be` (commit containing HERMES_BBD_WAL_006_CC0_FINAL_GATE_01.md)

Filesystem type: ext4 (disk-backed, /dev/mapper/ubuntu--vg-ubuntu--lv)

Hermes identity:
- Version: Hermes Agent v0.18.2 (2026.7.7.2) · upstream f709bd88 · local 10b6d1a9 (+1 carried commit)
- Provider: nous
- Model: meituan/longcat-2.0:free

## Protected preconditions (all exact)

HEAD == origin/master == 704b11caf135dec04c72232a509db540c71a18be, exactly three dirty paths, clean `git diff --check` (exit 0, no conflict markers or whitespace errors):

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,527 | `d42f58a2487c64ad59d282d93759a6b156239e0a125d399f7821f278a9087faa` |
| `scripts/security-policy.js` | 2,483 | `73a9c99b306259823c9c2931946dd8a974d8fa2c03efda62021f3f9c8c3b7927` |
| `deny.toml` | 40 | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` |

Immutable lock and scanner identities (unchanged from the stopped final gate):

| Path | SHA-256 |
| --- | --- |
| `package-lock.json` | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` |
| `wallet-broker/Cargo.lock` | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |
| `.gitleaksignore` | `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b` |

Tool versions: cargo 1.98.0, cargo-deny 0.20.2, gitleaks v8.30.1.

## Primary local crate-manifest license evidence

The three exact pinned CC0-1.0 transitive crates declared in the locally cached manifests (authoritative from the stopped final gate):
- `bounded-vec 0.9.0` — `license = "CC0-1.0"`
- `secp256k1 0.29.1` — `license = "CC0-1.0"`
- `secp256k1-sys 0.10.1` — `license = "CC0-1.0"`

These are Zcash transitives required by the WAL-006 wallet broker. No other crate in the locked graph declares CC0-1.0.

## Accepted test-first expected red (parent 47bad440)

The frozen test `WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists` expected `CC0-1.0` in `WAL004_ALLOWED_LICENSES` while the production policy constant (`scripts/security-policy.js`) and the deny allow-list (`deny.toml`) omitted it. Result: exactly 74 passed, one failed, final line `1 security policy test(s) failed`. The expected/actual license-list mismatch contained `CC0-1.0`.

## Exact three-line policy correction

The accepted CC0-1.0 production correction added exactly one `CC0-1.0` entry immediately after `BSL-1.0` in each closed license list:
- `'CC0-1.0',` in `WAL004_ALLOWED_LICENSES` (`scripts/security-policy.js`)
- `  "CC0-1.0",` in `[licenses].allow` (`deny.toml`)

No crate exceptions, ignores, source bypasses, license confidence changes, or broader licenses were added. The expected-red test's exact removal mutation remains capable of rejecting omission.

## Exact execution (each run once, in order, no network)

1. `node test/securityPolicy.node.js`
   - Exit: 0. Exactly 75 `ok`, zero `not ok`, final line `BitBook security policy tests passed (75).`

2. `node scripts/security-policy.js`
   - Exit: 0. Exact line `BitBook desktop security policy checks passed.`

3. `npm run test:security`
   - Exit: 0. Electron security exactly 19 passed (`BitBook electron security tests passed (19).`), policy exactly 75 passed (`BitBook security policy tests passed (75).`), zero failure.

4. `/home/lars/.cargo/bin/cargo-deny --manifest-path wallet-broker/Cargo.toml --all-features check advisories bans licenses sources`
   - Exit: 0. Advisories ok, bans ok, licenses ok, sources ok. The three exact CC0-1.0 crates accepted; no advisory/license/source/ban denial; no exception/ignore/bypass.

5. `target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .`
   - Exit: 0. 4836 commits scanned, ~27.65 MB in 2.38s, no leaks found.

6. `target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .`
   - Exit: 0. ~1.44 GB in 10.2s, no leaks found.

## Closed allow-list, no exceptions

The deny.toml `[licenses].allow` and the `WAL004_ALLOWED_LICENSES` array are closed lists. The only change is the exact `CC0-1.0` addition. No crate exceptions, no `license-exceptions`, no `ignore`, no `skip`, no source bypass. The three CC0-1.0 crates remain governed by the existing closed graph/source policy.

## Final state

After command 6 both lockfile hashes unchanged, exact three dirty identities unchanged, clean `git diff --check` (exit 0).

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 2,527 | `d42f58a2487c64ad59d282d93759a6b156239e0a125d399f7821f278a9087faa` |
| `scripts/security-policy.js` | 2,483 | `73a9c99b306259823c9c2931946dd8a974d8fa2c03efda62021f3f9c8c3b7927` |
| `deny.toml` | 40 | `e208918db0faa0b059c8a3a5bf581e10752c6384decfde71a23104ae2eb60b9a` |
| `package-lock.json` | — | `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413` |
| `wallet-broker/Cargo.lock` | — | `5c063b11e2882ab61bf126804cf8c3acef80ff5b4af16ac2fa59fe0ba6c85e01` |

The reviewer alone accepts the result and authorizes the next task.
