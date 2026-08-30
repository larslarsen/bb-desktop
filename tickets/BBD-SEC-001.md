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
- `gitleaks/gitleaks-action@e0c47f4f8be36e29cdc102c57e68cb5cbf0e8d1e`
  (`v3.0.0`)
- `@doyensec/electronegativity@1.10.3`
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
- a missing `npm audit`, maintained Electron test, policy test, Electronegativity scan, or
  complete-history Gitleaks scan;
- non-blocking scanner behavior, ignore/baseline/suppression flags, report uploads, or
  altered exit behavior;
- Electronegativity scanning the inherited repository instead of the maintained entry
  point; or
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
npx --yes @doyensec/electronegativity@1.10.3 -i social-main.js
npm run build
npm run test:social
npm run test:security
gitleaks git --redact=100 --no-banner .
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
- All acceptance commands pass with zero npm, Electronegativity, or new Gitleaks findings.
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
