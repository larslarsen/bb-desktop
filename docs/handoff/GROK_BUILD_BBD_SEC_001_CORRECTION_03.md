# Grok Build Design Review — BBD-SEC-001 Correction 03

You are **Sr Dev — Grok Build (Grok 4.6 High)**. This document is authoritative; chat
history is not.

Repository: `/home/lars/OpenBazaar/bb-desktop`

This is a design-review stop. Do not edit source, run commands/tests/scanners, install,
use Git, or mutate GitHub. Read the complete ticket and all prior BBD-SEC-001 handoffs,
then return one narrow test-first correction proposal for reviewer authorization.

## Reviewer evidence

The exact pinned command originally specified by the ticket eventually ran and returned
exit 0 while printing three findings:

- `CSP_GLOBAL_CHECK`: the single-file input omitted `social/index.html`, where the CSP
  actually lives;
- `AUXCLICK_JS_CHECK`: the old AST check does not recognize the modern
  `setWindowOpenHandler(() => ({ action: 'deny' }))` boundary; and
- `REMOTE_MODULE_JS_CHECK`: the tool assumed Electron 0.1.0 because a lone JS input
  provides no version metadata.

This reveals a gate defect independent of those findings: Electronegativity 1.10.3
prints findings but exits successfully. The current workflow therefore does not block
on results.

Reviewer then composed a diagnostic scan directory outside the repository containing
only `package.json`, `social-main.js`, `social/index.html`, `social/app.js`, and
`social/core.js`, and supplied Electron 44.0.0 explicitly. SARIF contained four results:

- `AUXCLICK_JS_CHECK`;
- `AVAILABLE_SECURITY_FIXES_GLOBAL_CHECK`, because the tool could not recognize Electron
  `44.0.x` and its fetched release list stopped at Electron 22;
- `CUSTOM_ARGUMENTS_JSON_CHECK`, because `npm start` intentionally retains the
  owner-approved development-only `--no-sandbox` workaround; and
- `CSP_GLOBAL_CHECK`, because the renderer intentionally accepts an owner-configured
  HTTP(S)/WebSocket daemon endpoint rather than one fixed origin.

The maintained runtime already denies new windows, navigation, redirects, webviews,
permission requests, and permission checks; uses a sandboxed/context-isolated renderer
without Node or preload; and has executable tests over the real Electron boundary. The
strong CSP is independently tested for unsafe script/style/eval and denied
object/frame/base/form capabilities. `npm audit` and complete-history Gitleaks remain
blocking gates.

The pinned tool's installed source confirms that findings merely populate a table/SARIF
and do not set a failure exit. Its upstream README now explicitly warns that the project
is no longer actively maintained.

## Design constraints

Recommend the smallest honest correction that:

- remains test-first and fail closed;
- does not suppress, baseline, or pretend these results are zero;
- does not weaken the maintained Electron boundary or CSP;
- provides meaningful blocking security coverage for maintained source;
- preserves blocking `npm audit` and complete-history Gitleaks;
- remains pull-request/manual only, with no routine push cost;
- does not upload scanner reports or build native packages;
- does not scan/revive inherited marketplace source or deprecated `go-ipfs`;
- adds no application runtime dependency and does not change `package-lock.json`;
- retains the owner-approved dev-only `npm start -- --no-sandbox` behavior and all manual
  native packaging behavior; and
- uses immutable Action/tool pins if it proposes another external tool.

Prefer deleting the invalid gate conceptually from policy over encoding exceptions for
an obsolete scanner. If existing executable boundary/mutation tests plus npm audit and
Gitleaks are the honest minimal replacement, say so and specify exactly how tests and
policy prove the workflow cannot regress. If recommending a replacement SAST tool,
justify its maintenance, exact fail-on-finding mechanism, maintained-path scope,
resource cost, offline/local reproducibility, and immutable pins.

Return:

1. root cause and security interpretation;
2. recommended exact paths and behavior;
3. tests-first order and required mutations;
4. exact acceptance changes;
5. whether `AUXCLICK_JS_CHECK` warrants a defense-in-depth product change despite the
   existing modern deny handler; and
6. risks or residual coverage gaps.

Do not implement until the reviewer records explicit authorization here and in the
ticket.

## Grok Build design return — Reviewer-preserved

Grok concluded that the scanner gate is invalid independently of each printed finding:
the CLI exits nonzero for invocation/tool errors, not findings; the upstream project is
unmaintained; its Electron release knowledge predates 44; and its successor is a
commercial service rather than an immutable local replacement. Wrapping the old tool,
scanning a larger tree, or adding exclusions would either keep a non-blocking control or
encode known false results. Adding Semgrep, CodeQL, or ESLint solely to preserve the SAST
label would add network/rule-pack/dependency cost without improving the already explicit
Electron boundary.

The recommended correction is therefore to remove the Electronegativity policy concept
and keep three complementary blocking controls:

1. executable Electron runtime, CSP, maintained-source sink, and workflow-mutation tests;
2. `npm audit --audit-level=low` over the lockfile-resolved dependency graph; and
3. the exact complete-history Gitleaks command.

No product runtime or CSP change is recommended. In particular, do not add
`disableBlinkFeatures: 'Auxclick'`: the old heuristic recognizes only that retired Blink
flag and cannot see Electron's modern deny-all `setWindowOpenHandler`. The useful
defense-in-depth is to invoke the live handler with `background-tab`, `foreground-tab`,
and `new-window` dispositions and prove they all return `{ action: 'deny' }` without
reaching `shell.openExternal`.

Grok proposed these exact source paths and test-first behavior:

- tests first: `test/electronSecurity.node.js`, `test/securityPolicy.node.js`;
- production/policy only after tests: `scripts/security-policy.js`,
  `.github/workflows/security.yml`;
- strengthen the existing new-window test without changing its count;
- add one maintained-source test over only `social-main.js`, `social/index.html`,
  `social/app.js`, and `social/core.js`, rejecting `innerHTML`, `outerHTML`,
  `insertAdjacentHTML`, `document.write`, `eval(`, `new Function`, and `javascript:`;
- retain every existing audit, Gitleaks, Action-pin, permissions, path-filter, routine
  packaging, SBOM, and no-suppression mutation;
- remove only the Electronegativity pin/presence expectations;
- reject any reintroduction of Electronegativity/ElectroNG, including the formerly pinned
  command, inherited or maintained scan targets, SARIF/CSV output, `-x`,
  `--exclude-checks`, or `eng-disable` annotations; and
- remove only the Electronegativity constant/command/check/export and its one workflow
  step.

Residuals recorded by Grok: there is no generic third-party JS SAST after this correction;
Electron fuses and packaged sandbox verification remain deferred with native packaging;
the app continues to use `file://`; and the owner-configured daemon endpoint plus
development-only `--no-sandbox` script remain intentional. These do not weaken the
tested production window, permission, navigation, CSP, dependency, or secret boundaries.

## Reviewer authorization

Accepted with one precision correction: the final expected counts are Electron 13/13 and
policy 47/47. Preserve all 46 current policy test cases conceptually, converting the
Electronegativity-specific cases rather than dropping them, and add exactly one distinct
obsolete-scanner report/suppression case.

Grok is authorized to edit only:

- `test/electronSecurity.node.js` first;
- `test/securityPolicy.node.js` first;
- `scripts/security-policy.js` only after both tests are complete; and
- `.github/workflows/security.yml` only after both tests are complete.

Required policy-test conversions:

- constants test drops only the Electronegativity export assertion;
- unpinned-tool test becomes CycloneDX-only;
- missing-Electronegativity test becomes exact pinned Electronegativity plus ElectroNG
  reintroduction rejection;
- inherited-repository test becomes obsolete Electron-SAST input rejection for
  `social-main.js`, `.`, `main.js`, and `js/`;
- the existing suppression test retains all Gitleaks baseline/config/ignore mutations;
  and
- one new test rejects obsolete-scanner SARIF/CSV output, `-x`, `--exclude-checks`, and
  `eng-disable` forms.

The checker must reject the obsolete scanner vocabulary fail closed before checking the
remaining required commands, while retaining every existing Gitleaks and generic
non-blocking/suppression rule. The workflow deletes exactly the one obsolete scanner
step. No other path or behavior changes. Grok runs nothing, uses no Git, and returns
separate ordered hashes for tests and production/policy source.

After reviewer source acceptance, Luna reruns Electron 13/13, policy 47/47, checker, npm
audit, build syntax, social tests, complete security tests, exact Gitleaks, diff check,
and the unchanged local manual SBOM sequence. Luna also falsifies reintroduction of the
formerly pinned scanner before Git.

## Source delivery and reviewer acceptance

Grok authored both tests completely before either production/policy path and ran
nothing. Delivered test source:

```text
c794d5e063bb121f52ae09bd96bb3ced061a02bc38c2f866957fe629d6999089  test/electronSecurity.node.js  508 lines  13 tests
6dbd7d2bf534b587efd0445045a662911dfc83fd6498ae1b048e0439369d22dd  test/securityPolicy.node.js    860 lines  47 tests
```

Delivered production/policy source after the tests:

```text
473ce9702c8d8bd4518a6ef02b8bcb59b8e4cc8f62fdab9406c8990d544f93e3  scripts/security-policy.js       1348 lines
0e4fb13b7efb206137972eb70851703340d8f8f237ba2f64a380d5104365fd83  .github/workflows/security.yml     45 lines
```

Reviewer independently reproduced every hash, line count, and test count and inspected
the fail-closed source. Accepted findings:

- the live deny handler is exercised across background-tab, foreground-tab, and
  new-window dispositions without a runtime change;
- the sink scan reads exactly the four authorized maintained files;
- all 46 prior policy cases remain conceptually represented and the new dedicated
  obsolete-report/suppression case yields 47;
- obsolete scanner vocabulary is rejected before remaining required commands;
- no obsolete scanner constant/function/export remains in production policy;
- the security workflow removed exactly the obsolete step while preserving audit,
  boundary/policy checks, and the exact pinned full-history Gitleaks design; and
- no unauthorized path was changed.

Codex Luna may now integrate these exact hashes under the ticket. It must reconstruct a
bounded correction-specific red without deleting files, restore hashes, run full green,
falsify obsolete-scanner reintroduction, then rerun the complete corrected acceptance and
unchanged manual SBOM sequence. Any failure or Gitleaks finding stops before Git.
