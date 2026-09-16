# BBD-PAY-001 Grok UI finish 01

Actor: Grok Build 4.6 High (xAI), owner-relayed. Reviewer: Codex.
Handoff: [GROK_BUILD_BBD_PAY_001_UI_FINISH_01.md](../handoff/GROK_BUILD_BBD_PAY_001_UI_FINISH_01.md).
Review: [BBD-PAY-001-SOURCE-REVIEW-03.md](BBD-PAY-001-SOURCE-REVIEW-03.md).
Correction-02 transport/Node work stays accepted and was not rerun.
Prior reports/captures were not overwritten.

Desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
All nineteen source/input hashes in
[BBD-PAY-001-GROK-CORRECTION-02.md](BBD-PAY-001-GROK-CORRECTION-02.md)
matched before this pass. Capture filesystem `dist/pay001-ui-finish01` is
`ext2/ext3`. Unrelated dirty work was left untouched.

This actor has **no** `require_escalated` tool. Host uid-0 helper checks below
were made in the ordinary tool shell (runner uid 1000, single `NSpid`,
`Seccomp 0`, `NoNewPrivs 0`) and returned real uid/gid 0, not a namespaced
"nobody" mapping.

## Host preflight

Read-only record: [preflight.json](../../dist/pay001-ui-finish01/preflight.json)
SHA-256 `f205910efa154628344bf18b49f9d4be08e5db9e624f3e9b32085a1516b3754a`.

| Check | Result |
| --- | --- |
| `stat -Lc '%u %g %a %s %n'` helper | `0 0 4755 15232 /opt/google/chrome/chrome-sandbox` |
| helper type | regular file, not a symlink; `readlink -f` is the same path |
| helper SHA-256 | `c100b678a8c171ad0733e51b6f18d98d936d38ab945681c41da00f2ee22e7571` |
| parent dirs `/opt/google/chrome`, `/opt/google`, `/opt`, `/` | uid/gid 0/0, mode `0755`, not group/other writable, not symlinks |
| Electron dist SHA-256 | `180eea79cebc825a7e7291b8c5ef387760773f2b03596751bda8e6e532a8b6c8` |

Prerequisite verified. The unpackaged sibling
`node_modules/electron/dist/chrome-sandbox` remains uid 1000 mode `0755` and
was not chmod'd, copied, or replaced.

## Smoke run-01 — helper route failed

Existing harness (correction-02 hash) was run once after preflight, without
`--disable-setuid-sandbox` or `--no-sandbox`.

Allowlisted command-local environment:

- `CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox`
- `BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-finish01/run-01`

`xvfb-run -a` assigned the child `DISPLAY`. Full environment was not dumped.

```
CHROME_DEVEL_SANDBOX=/opt/google/chrome/chrome-sandbox
BBD_PAY001_ARTIFACT_DIR=dist/pay001-ui-finish01/run-01
xvfb-run -a node_modules/.bin/electron test/paymentInbox.electron.js
```

Capture driver: [capture-run.js](../../dist/pay001-ui-finish01/capture-run.js)
SHA-256 `aaeff9603565fa5393b9ab0495abefcecc12bcc0a2fbc47879eb565d2a242ca3`.
Exclusive new directory, waits for stdout/stderr `end`, records spawn
errors/timeouts.

| Field | Value |
| --- | --- |
| metadata | [smoke.json](../../dist/pay001-ui-finish01/run-01/smoke.json) SHA `26127eb9c4ec17cae472230c0909f2fca8ae4e30dc24e783aaab47988283e0d4` |
| started_utc | `2026-09-15T22:00:20.223Z` |
| ended_utc | `2026-09-15T22:00:20.484Z` |
| exit_code | 1 (`xvfb-run`; child SIGTRAP) |
| stdout SHA | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| stderr | [smoke.stderr.log](../../dist/pay001-ui-finish01/run-01/smoke.stderr.log) SHA `0e18ca59f18f39d1b468d1467bbfa4408966f8c8ee1c4af286f318a7ca999da6` |

Chromium fatal from `setuid_sandbox_host.cc`: the SUID helper **was found**
but is not configured correctly; it named
`node_modules/electron/dist/chrome-sandbox` (uid 1000, mode `0755`), not the
verified `/opt/google/chrome/chrome-sandbox`.

This matches Chromium's `GetSandboxBinaryPath`: if
`$DIR_EXE/chrome-sandbox` exists, that sibling is used; `CHROME_DEVEL_SANDBOX`
is consulted only when the sibling is absent. Electron always ships that
sibling, so the documented devel-helper variable cannot select the installed
Chrome helper. The two binaries are not identical (Electron sibling SHA-256
`a4f6dfd7325ddd55f94ddc0c487d22726d40ef099ded6475d5dffe236e277896`).

No PNG, `sandbox-status.json`, or `electron-smoke.json` was written. This is
a helper-route incompatibility, not a layout assertion. Environment attempts
stop here: no chmod/chown/copy, no AppArmor/sysctl change, no
`--disable-setuid-sandbox`, no `--no-sandbox`, no second helper install.

## Harness correction (not UI-green)

The smoke never reached DOM assertions. The collapsed-details geometry defect
from review 03 was still corrected in the harness only, as specified:

- `layoutOk` requires nonzero geometry for the matched control and each laid-out
  child; `scrollWidth <= clientWidth + 1`; no viewport overflow; no sibling
  overlap.
- Descendants of a **closed** `<details>` other than `summary` are excluded
  (those boxes are intentionally zero). Missing/hidden required controls still
  fail.
- Every fixture row's summary is asserted visible, then opened with keyboard
  Enter, then the full Request ID / Payee / Created / Expires / Digest values
  are asserted and the open details geometry is checked, then closed with
  Enter.
- Nonempty screenshots remain specified against the collapsed layout.

`social/styles.css` was not edited: no inbox CSS failure was demonstrated by a
running smoke.

`node --check test/paymentInbox.electron.js` after the final harness:

| Field | Value |
| --- | --- |
| metadata | [check.json](../../dist/pay001-ui-finish01/run-03-check/check.json) SHA `052a7785b27db2bd3c4031ddfd7444fa1fe06fe3ae1597da8cbe140ba9e4a4cc` |
| started_utc | `2026-09-15T22:04:15.523Z` |
| ended_utc | `2026-09-15T22:04:15.547Z` |
| exit_code | 0 |
| stdout/stderr SHA | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

An earlier syntax check of an intermediate harness revision is preserved at
[run-02-check](../../dist/pay001-ui-finish01/run-02-check/) and was not
overwritten.

## Hashes after this pass

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| test/paymentInbox.electron.js | 580 | `89d82e8ad73da7d08e3f03a1e9d953521a75a4f55803d96f1265f87bd8c8ce54` |
| social/styles.css | 985 | `4673ece3b62454458c0cc2f538a120073b8d9e0d73f83834c1d63716c5a31b31` |

`styles.css` is unchanged from correction 02. All other eighteen listed
source/input hashes remain as in the correction-02 tables.

## UI artifacts

| Artifact | Status |
| --- | --- |
| fixture-nonempty-1180x780.png | **unavailable** |
| fixture-nonempty-860x620.png | **unavailable** |
| fixture-empty-1180x780.png | **unavailable** |
| fixture-unavailable-860x620.png | **unavailable** |
| sandbox-status.json | **unavailable** |
| live webPreferences / OS Seccomp 2 proof | **unavailable** |

Screenshot data, had it been produced, would have been in-process HTTP fixtures,
not a live wallet or owner daemon. A failed helper abort is not a passed smoke.

## Blocker

Exact failure: Chromium selected the unpackaged sibling
`node_modules/electron/dist/chrome-sandbox` and aborted because that file is
not a root-owned mode-4755 helper. The verified installed Chrome helper cannot
be selected via `CHROME_DEVEL_SANDBOX` while that sibling exists. Making the
sibling usable requires forbidden host permission changes. Node suites were
not rerun.

Stop for Codex source/execution review.
