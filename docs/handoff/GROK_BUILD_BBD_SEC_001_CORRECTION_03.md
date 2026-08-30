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
