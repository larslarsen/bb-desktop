# BBD-SEC-001 — Harden the Maintained Electron Client and Add Security/SBOM Gates

Status: AUTHORIZED

Reviewer: Lead Engineer/Reviewer — Codex

Source actor: Sr Dev — Grok Build (Grok 4.6 High)

Integration actor: Jr Dev — Codex Luna (`gpt-5.6-luna`)

Source baseline: `f40d8404`

## Objective

Make the small maintained Electron social client fail closed at its renderer, navigation,
permission, dependency, secret, and supply-chain boundaries. Normal pushes remain fast
and never build native packages. Security scanning runs only for relevant pull requests
or manual dispatch. SBOM generation is manual and uploads only CycloneDX JSON.

`../go-ipfs` is deprecated and out of scope. The inherited OpenBazaar renderer is
migration reference, not the product entry point, and must not be revived or scanned as
maintained application source by this ticket.

## Verified baseline

- `package.json` selects `social-main.js` and pins current Electron `44.0.0`.
- `npm audit --json` reports 0 total vulnerabilities across 13 dependencies.
- The maintained window already disables Node integration, enables context isolation and
  sandboxing, loads only the local social page, blocks navigation, and denies permission
  requests.
- The renderer CSP uses self-only scripts and the UI writes social content through DOM
  text nodes rather than HTML injection.
- GitHub Dependabot alerts are currently disabled for this repository. This ticket uses
  blocking `npm audit`; enabling repository alerts is a separate GitHub-setting action.

## Exact pins

- Electron: `44.0.0`
- Node CI: `24`
- `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (`v7.0.1`)
- `actions/setup-node@48b55a011bda9f5d6aeb4c2d9c7362e8dae4041e` (`v6.4.0`)
- `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` (`v7.0.1`)
- Gitleaks CLI: `8.30.1`
- Gitleaks Linux x64 archive SHA-256:
  `551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb`
- Gitleaks Linux x64 archive size: `8230402` bytes
- `@cyclonedx/cyclonedx-npm@6.0.1`

## Authorized paths

Test source, authored first:

- `test/electronSecurity.node.js`
- `test/securityPolicy.node.js`

Production/policy source, authored only after the tests:

- `social-main.js`
- `social/index.html`
- `package.json`
- `.github/workflows/social.yml`
- `.github/workflows/security.yml`
- `.github/workflows/sbom.yml`
- `scripts/security-policy.js`
- `scripts/validate-sbom.js`

Codex Luna may additionally author only:

- `docs/security/BBD-SEC-001-EVIDENCE.md`
- `docs/handoff/CURRENT_TASK.md`

No package-lock, packaging script, native artifact, inherited marketplace source, logo,
daemon, `go-ipfs`, signing, release, or repository-setting change is authorized.

## Maintained runtime invariants

Tests must exercise `social-main.js` through a mocked Electron boundary, not merely search
for success text. They must prove:

- `app.enableSandbox()` is invoked;
- the actual `BrowserWindow` explicitly sets `nodeIntegration: false`,
  `contextIsolation: true`, `sandbox: true`, `webSecurity: true`,
  `allowRunningInsecureContent: false`, and `experimentalFeatures: false`;
- only the repository's `social/index.html` is loaded;
- all renderer navigation and new-window creation are denied;
- `shell.openExternal` is not reachable from renderer-controlled URLs;
- both permission request and permission check handlers deny every permission;
- no preload, webview, IPC bridge, remote content load, or inherited `main.js` entry point
  is introduced; and
- the CSP keeps self-only script/style execution and explicitly denies objects, frames,
  base changes, and form submission without adding unsafe inline/eval directives.

The source change must be minimal: remove renderer-controlled external opening, add the
permission-check denial, make security-relevant web preferences explicit, and strengthen
the existing CSP. It must not change daemon connectivity or social behavior.

## Workflow policy

`scripts/security-policy.js` and its independent mutation tests must fail closed on:

- mutable or wrong Action/tool pins;
- missing least-privilege `contents: read` permissions;
- routine native packaging, artifact upload, or product-binary build;
- security or SBOM workflows running on ordinary pushes;
- missing relevant path filters or documentation-only CI filtering;
- a missing `npm audit`, maintained Electron boundary/sink test, policy test/checker, or
  complete-history Gitleaks scan;
- non-blocking scanner behavior, ignore/baseline/suppression flags, report uploads, or
  altered exit behavior;
- reintroduction of unmaintained Electronegativity/ElectroNG commands, reports,
  exclusions, annotations, or inherited-source scan targets; or
- SBOM output other than one validated CycloneDX JSON artifact.

The existing native Linux/Windows/macOS jobs remain manual-only and are not dispatched.
Normal push/PR validation runs only offline syntax and Node tests over relevant maintained
paths. Security runs on relevant pull requests plus `workflow_dispatch`. SBOM runs only
on `workflow_dispatch`, performs `npm ci`, blocking `npm audit`, pinned CycloneDX
generation and structural validation, and uploads only JSON with 14-day retention.

## Test-first integration

Grok authors both test files before every production/policy path and does not execute
them. It reports test and production hashes separately.

Codex Luna verifies the hashes, reconstructs a bounded test-only red state without
deleting files or using unresolved targets, and records the expected failures for absent
runtime hardening/policy/workflows. It restores the exact source hashes, runs green, then
falsifies at least these mechanisms one at a time with bounded reversible edits:

- sandbox/context isolation;
- permission denial;
- new-window denial;
- CSP unsafe-script rejection;
- immutable Action pin rejection; and
- routine-package-build rejection.

Each falsification must make the relevant test fail for the intended reason, after which
the exact delivered source is restored and hash-checked.

## Acceptance commands

Run in this order and stop before later commands or Git on any failure/finding:

```text
node test/electronSecurity.node.js
node test/securityPolicy.node.js
node scripts/security-policy.js
npm audit --audit-level=low
npm run build
npm run test:social
npm run test:security
gitleaks git --redact=100 --no-banner .
gitleaks dir --redact=100 --no-banner .
git diff --check
```

The local Gitleaks run must use the existing pinned v8.30.1 binary under the explicit
disk-backed `/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829` directory and
write no unredacted report. Any inherited finding stops for reviewer triage; no baseline
or suppression is pre-authorized.

Luna also executes the SBOM workflow commands locally without native packaging. Use only
explicit disk-backed paths beneath `/home/lars/OpenBazaar/.security-artifacts`,
`.security-cache`, and `.security-tmp`; do not use local `/tmp`. Record the CycloneDX
format/spec, root component, component/dependency counts, SHA-256, and byte size without
committing the generated document.

## Resource and safety rules

- Do not install globally, use root, clean, delete, or run `rm` in any form.
- Do not use local `/tmp`; GitHub runner temporary storage is allowed in workflows.
- Do not echo secret values or write/upload unredacted scanner reports.
- Leave local tools, caches, SBOM, and scan artifacts at their exact disk-backed paths for
  owner inspection.
- Do not dispatch native packaging, security, or SBOM workflows during source authoring.
- Grok does not run tests, scanners, installs, Git, or GitHub mutations.
- Luna does not repair findings or design tests; it stops and reports.

## Acceptance criteria

- Red, green, and falsification evidence is non-vacuous.
- All acceptance commands pass with zero npm vulnerabilities or new Gitleaks findings.
- Routine CI remains package-free and documentation-only changes do not consume it.
- The maintained Electron security boundary is executable and fail closed.
- Manual SBOM generation produces one validated CycloneDX JSON and no application binary.
- Reviewer accepts the integrated commit and required remote routine CI passes.

## Correction 1 — Authorized

Reviewer source inspection rejected the Gitleaks Action design before execution. Despite
complete checkout, that action applies event-specific first-parent ranges and does not
execute the ticket's complete-history command, creating a merge-history false-negative
risk. Its default summary behavior also conflicts with the no-finding-body rule. The
routine path filters also omitted `scripts/build-windows.ps1`.

Grok may perform only `docs/handoff/GROK_BUILD_BBD_SEC_001_CORRECTION_01.md`: test-first
replacement of the Action with the official pinned Gitleaks v8.30.1 Linux archive,
verified by exact SHA-256 and size, followed by exact root command
`gitleaks git --redact=100 --no-banner .`; and inclusion of the Windows build script in
both routine path filters. No other source or behavior may change. Luna does not execute
until reviewer accepts the bounded correction source.

Correction 1 was delivered test-first and reviewer source inspection accepted the exact
four-path hashes and fail-closed CLI design preserved in the correction handoff. Codex
Luna may now begin the ticketed red/green/falsification and acceptance sequence. All
original stop, disk-path, no-cleanup, no-native-package, and no-secret-output rules remain
in force.

## Correction 2 — Authorized

Luna's green run passed Electron 12/12 and policy 47/48. The only failure was an overly
narrow test-message regex: the summary mutation was correctly rejected with “summaries,”
while the oracle matched singular `summary` only. No falsification, scanner, SBOM, Git, or
push ran. Grok may edit only `test/securityPolicy.node.js` under
`docs/handoff/GROK_BUILD_BBD_SEC_001_CORRECTION_02.md` to make the existing semantic
oracle recognize the summary-specific rejection without weakening it. Luna resumes only
after reviewer accepts that one-path report.

Correction 2 was delivered as the exact semantic oracle change and reviewer inspection
accepted its hash. Luna may verify the corrected test file, rerun the complete policy
suite, and resume the prior falsification/scanner/SBOM sequence only on full green.

## Network execution correction — Authorized

Luna completed and restored the bounded red/falsification work, then passed the Electron
suite 12/12, corrected policy suite 46/46, repository policy checker, and npm audit with
zero vulnerabilities. The first sandboxed Electronegativity attempt failed with npm
`EAI_AGAIN`; its escalated retry remained silent indefinitely and was interrupted. It
made no repository, evidence, artifact, staging, commit, or push change.

Luna may resume under
`docs/handoff/CODEX_LUNA_BBD_SEC_001_NETWORK_01.md`: exact disk-backed npm cache and
artifacts, exact pinned scanners, and a 180-second foreground timeout for each
network-backed `npx` invocation. A timeout, network/tool error, or finding remains a hard
stop. No source repair, cleanup, deletion, native packaging, workflow dispatch, or `/tmp`
use is authorized.

## Scanner finding stop — Resolved by Correction 3

The bounded Electronegativity command obtained network access and completed, but printed
three findings while returning exit 0. Reviewer diagnostics confirmed that the pinned
tool is no longer actively maintained, does not understand Electron 44, and does not
fail its process when findings exist. A maintained-app-only SARIF diagnostic produced
four results, including tool-age, owner-approved development sandbox, configurable daemon
endpoint, and aux-click heuristics. No later gate, SBOM, source repair, evidence, Git, or
push ran.

Correction design is delegated to Grok Build under
`docs/handoff/GROK_BUILD_BBD_SEC_001_CORRECTION_03.md`. This is design review only; no
source change or integration resumption is authorized until the reviewer accepts and
records an exact test-first replacement.

## Correction 3 — Authorized

Grok Build's design review and reviewer source inspection agree that Electronegativity
1.10.3 cannot serve as a blocking Electron 44 control: upstream marks it unmaintained,
its release model predates Electron 44, and findings do not change the CLI's success
exit. Encoding exclusions or product no-ops would be dishonest. The invalid scanner
concept is removed, not suppressed or baselined.

Grok may now perform only the exact test-first source correction preserved in
`docs/handoff/GROK_BUILD_BBD_SEC_001_CORRECTION_03.md`: strengthen the live window-open
test for aux-click dispositions; add a maintained-source injection/eval sink test;
mutation-test rejection of Electronegativity/ElectroNG and their report/suppression/scope
forms; remove the obsolete checker constant/function/export; and delete its one workflow
step. No runtime, CSP, package, lockfile, SBOM, routine workflow, packaging, inherited
source, or dependency change is authorized. Grok runs nothing and reports exact
test/production hashes separately. Luna resumes only after reviewer source acceptance.

Acceptance now treats the executable Electron boundary/sink suite and policy mutations
as the maintained-source security scan, alongside blocking npm audit and complete-history
Gitleaks. After correction, Luna reruns every acceptance command from the beginning and
adds an obsolete-scanner-reintroduction falsification. The historical network handoff is
superseded for execution but remains preserved as evidence of the stopped attempt.

Correction 3 was delivered test-first at the exact four authorized paths. Reviewer
independently verified the reported hashes and accepted the source: Electron 13 tests,
policy 47 tests, fail-closed obsolete-scanner rejection, and deletion of only the invalid
workflow step. Luna may now resume correction-specific red/restore/falsification and the
complete corrected acceptance sequence.

Luna executes under `docs/handoff/CODEX_LUNA_BBD_SEC_001_CORRECTION_03.md`, which fixes
the exact correction-specific red/green/falsification order and supersedes only the old
Electronegativity command. All other safety, acceptance, SBOM, evidence, and Git rules
remain in force.

## Gitleaks finding stop — Reviewer triage authorized

Correction integration passed bounded red/restore, Electron 13/13, policy 47/47, checker,
npm audit with zero vulnerabilities, obsolete-scanner falsification/restore, build
syntax, social tests, and the complete security suite. Exact Gitleaks v8.30.1 then scanned
4,549 commits, reported 10 findings, and exited 1. Luna stopped before SBOM, evidence,
staging, commit, or push.

Reviewer-only redacted metadata triage is authorized under
`docs/handoff/BBD_SEC_001_GITLEAKS_TRIAGE_01.md`. It authorizes no suppression, baseline,
repair, deletion, Git, or integration resumption.

Triage proved eight unique findings are inherited upstream commit fingerprints; one is
duplicated three times by Gitleaks. A redacted current-tree scan also found two retained
matches in inactive inherited marketplace utility modules. No ratchet may hide those
current-tree matches. Grok Build owns design review only under
`docs/handoff/GROK_BUILD_BBD_SEC_001_CORRECTION_04.md`; no implementation or resume is
authorized yet.

## Correction 4 — Authorized

Grok Build's exact test-first secret-ratchet design is accepted under
`docs/handoff/GROK_BUILD_BBD_SEC_001_CORRECTION_04.md`. It authorizes only one test file
first, then an eight-line lexically sorted commit-fingerprint `.gitleaksignore`, explicit
no-op/removal of the inherited Countly and Doorbell loader bodies in the two exact utility
files, checker enforcement, and a second exact blocking current-tree Gitleaks workflow
step. The full-history command remains unchanged.

The ratchet owner is Lead Engineer/Reviewer — Codex. Its sole rationale is the eight
reviewed 2016–2018 fingerprints proven ancestral to upstream OpenBazaar. It must be
removed when a later authorized ticket removes the inherited marketplace tree, old root
page, and unused renderer entry. It may never contain a global/current-tree fingerprint,
comment, wildcard, extra line, secret value, or finding body.

No baseline report, config/allowlist, command-line ignore, altered exit/range, report,
upload, dependency, package/lockfile, maintained social runtime/CSP, SBOM, routine CI,
native package, unrelated inherited source, deletion/cleanup, scanner execution, Git, or
GitHub mutation is authorized for Grok. Luna resumes only after reviewer hash/source
acceptance.

Reviewer inspection found one vacuous synthetic-loader oracle before execution: its
standalone fixture fails the earlier public-export invariant instead of the intended
remote-loader invariant. Correction 04 Oracle 01 authorizes Grok to edit only
`test/securityPolicy.node.js` as specified in the handoff, preserving 50 tests and all
production hashes. Luna remains stopped.

Correction 04 and Oracle 01 were delivered test-first at the exact authorized paths.
Reviewer independently reproduced every hash/count and accepted the strict eight-line
ratchet, two loader neutralizations, checker, exact dual-scan workflow, 50 policy tests,
and corrected non-vacuous loader mutation. Luna may resume only under
`docs/handoff/CODEX_LUNA_BBD_SEC_001_CORRECTION_04.md`.

Luna's restored green passed Electron 13/13 and policy 49/50. The only failure is an
over-eager checker branch that labels a valid ninth fingerprint as trailing bytes before
the intended extra-count rule. Correction 04 Oracle 02 authorizes Grok to remove only
that premature branch in `scripts/security-policy.js`. Luna remains stopped.

Oracle 02 was delivered and reviewer-accepted at checker SHA-256
`c693e4055c0da3d244603857b44a6ea849d1ebb521624cce698d3bc87894f72a`.
All other accepted hashes remain unchanged. Luna resumes from restored green without
repeating its completed Correction 04 red/restore.

Correction 04 subsequently passed every application/security gate, including exact
full-history Gitleaks over 4,558 commits and current-tree Gitleaks with zero findings. The
manual SBOM sequence stopped at sandboxed `npm ci` on registry DNS `EAI_AGAIN`, before any
SBOM/evidence/Git work. One exact bounded external-network retry is authorized under
`docs/handoff/CODEX_LUNA_BBD_SEC_001_SBOM_NETWORK_01.md`.
