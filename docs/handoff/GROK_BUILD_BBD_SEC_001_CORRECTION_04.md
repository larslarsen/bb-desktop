# Grok Build Design Review — BBD-SEC-001 Correction 04

You are **Sr Dev — Grok Build (Grok 4.6 High)**. This is design review only. Do not edit,
run commands/tests/scanners, install, use Git, or mutate GitHub.

Read the complete ticket and every BBD-SEC-001 handoff, especially Gitleaks Triage 01.
Return an exact test-first correction proposal; implementation waits for reviewer
authorization.

## Redacted reviewer evidence

Gitleaks v8.30.1 full-history scan now covers 4,550 commits and returns 10
`generic-api-key` findings. `Secret` and `Match` redaction was verified at 100 percent
without displaying either. There are eight unique commit fingerprints; one is emitted
three times. Every unique commit is an ancestor of `upstream/master`, dates from
2016–2018, and predates BitBook's first authored commit in 2026:

```text
988fcc3da2d2b13689fdd98e936df14e2f989709:js/models/order/Case.js:generic-api-key:107
bfd12cbe6e1f586af1f728c6d4e1ba68b8d9d103:js/utils/metrics.js:generic-api-key:13
e30e2ebbe6cc6198ca3c507167d26ff934ef9deb:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65
d38fc4819f1aa16f692394c56acc90db5d4f973a:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65
f83f40146c4bd2eb6da9f7fdd7a8eab8fb660b13:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:63
b0637a03e1eb12e4e5d49c9dfba92dcbf51a0d8c:js/utils/feedback.js:generic-api-key:8
f527597842b38bbe25c36cb42d204f16747e7e72:js/start.js:generic-api-key:409
7f6a71d6d5ec94b0d8ed02a23eddd7d1bfeaf802:index.html:generic-api-key:57
```

A separate 100-percent-redacted `gitleaks dir .` scan reports two current-tree matches:

```text
js/utils/metrics.js:generic-api-key:79
js/utils/feedback.js:generic-api-key:17
```

These are inherited marketplace modules, not imported by the maintained social entry,
but they remain present in the checkout. They must not be hidden by a current-tree/global
fingerprint.

Official Gitleaks documentation supports exact report baselines and fingerprint entries
in `.gitleaksignore`. The latter contains no secret values. Gitleaks' default ignore path
is repository root, so an exact full-history command can remain unchanged while eight
commit-qualified fingerprints ratchet inherited history. Commit-qualified entries do not
match current-tree/global fingerprints.

## Required security outcome

Recommend the smallest correction that:

- removes or neutralizes both current-tree matches without printing, copying, or storing
  their secret/match bodies in tests, reports, patches, handoffs, or output;
- retains the exact complete-history Gitleaks scan and adds an exact blocking current-tree
  `gitleaks dir --redact=100 --no-banner .` scan;
- permits only the eight exact inherited commit-qualified fingerprints above and rejects
  missing, extra, duplicate, malformed, global/current-tree, wrong-path/rule/line/commit,
  unsorted, comment, wildcard, or secret-bearing entries;
- treats the ratchet as temporary until inherited marketplace removal, with owner,
  rationale, and expiry/removal condition in durable docs;
- mutation-tests the ratchet, both exact scan commands/order, path filters, and fail-closed
  behavior;
- adds no dependency, report upload, baseline report, config allowlist, altered exit code,
  range/log option, token, native package, workflow dispatch, `/tmp`, deletion, or cleanup;
- keeps ordinary pushes package-free and security PR/manual only; and
- does not change maintained social runtime/CSP, package files, SBOM, native scripts, or
  unrelated inherited code.

Evaluate whether the two inactive files should receive minimal safe placeholder changes
or whether their unused integrations should be disabled more explicitly. Do not propose
large marketplace deletion inside this ticket. Explain how the implementation can avoid
ever reproducing the detected values in tool output or handoff history.

Return:

1. root-cause/security interpretation of historical versus current findings;
2. exact authorized paths and changes;
3. tests-first order and mutation matrix;
4. exact Gitleaks workflow/checker commands and ordering;
5. safe method for neutralizing current values without disclosure;
6. exact red/green/falsification and acceptance sequence; and
7. ratchet owner, rationale, expiry/removal condition, and residual risks.

## Grok Build design return — Reviewer-preserved

Grok classified the gate as correct and the missing controls as: no reviewed inherited
history ratchet, two retained current-tree integrations, and no blocking directory scan.
It rejected a baseline report because that would store finding bodies and could hide
current-tree matches. It recommended eight exact commit-qualified `.gitleaksignore`
fingerprints, which contain no secret values and cannot match three-part current-tree
fingerprints.

The two inactive integrations are not merely inert strings: the inherited metrics module
can initialize Countly and load a remote script, while feedback can initialize Doorbell
and load its remote embed. Grok therefore recommended removing those loader bodies, not
substituting look-alike values or adding `gitleaks:allow`. Large marketplace deletion
remains a separate ticket.

Grok proposed:

- test-first edits to `test/securityPolicy.node.js` only;
- production edits afterward to `.gitleaksignore`, `js/utils/metrics.js`,
  `js/utils/feedback.js`, `scripts/security-policy.js`, and
  `.github/workflows/security.yml` only;
- Electron stays 13 tests; policy grows from 47 to 50;
- exact full-history command remains
  `gitleaks git --redact=100 --no-banner .`;
- exact current-tree command is added immediately after it:
  `gitleaks dir --redact=100 --no-banner .`;
- no baseline/config/report/range/exit/ignore-path flags, Gitleaks Action, token, summary,
  upload, native package, or routine push scan;
- the security PR path filter adds `.gitleaksignore`, `js/utils/metrics.js`, and
  `js/utils/feedback.js`; routine `social.yml` stays unchanged;
- the checker exports and enforces the exact ratchet metadata/body and requires
  install → git scan → dir scan with no intervening step; and
- whole-workflow suppression checks are narrowed only enough to allow the literal
  `.gitleaksignore` path-filter entry, while command flags and all other suppression
  mechanisms remain forbidden.

Its mutation matrix covers missing/extra/duplicate/malformed/global/wrong/shuffled
fingerprints, comments/blanks/wildcards/secret-bearing text, BOM/CRLF/trailing bytes,
missing/reordered/modified scan commands, command-line ignore/baseline/report/exit flags,
missing path filters, and synthetic current-tree key-shaped values. No mutation uses the
inherited values.

Grok's ratchet terms:

- owner: Lead Engineer/Reviewer — Codex;
- rationale: eight inherited 2016–2018 upstream commit fingerprints only; current-tree
  copies are disabled and removed, never ignored; and
- removal condition: delete `.gitleaksignore` when a later authorized ticket removes the
  inherited OpenBazaar marketplace tree (`js/`, old root `index.html`, and its unused
  renderer entry).

Residual risk remains in immutable public upstream history until that tree/history is
retired; any history rewrite or moved finding fails closed as a new fingerprint. Other
unused marketplace code is not comprehensively audited by this bounded correction.

## Reviewer authorization

Accepted with these precision changes:

1. The eight lines must be true lexical order, making the ordering oracle objective:

```text
7f6a71d6d5ec94b0d8ed02a23eddd7d1bfeaf802:index.html:generic-api-key:57
988fcc3da2d2b13689fdd98e936df14e2f989709:js/models/order/Case.js:generic-api-key:107
b0637a03e1eb12e4e5d49c9dfba92dcbf51a0d8c:js/utils/feedback.js:generic-api-key:8
bfd12cbe6e1f586af1f728c6d4e1ba68b8d9d103:js/utils/metrics.js:generic-api-key:13
d38fc4819f1aa16f692394c56acc90db5d4f973a:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65
e30e2ebbe6cc6198ca3c507167d26ff934ef9deb:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:65
f527597842b38bbe25c36cb42d204f16747e7e72:js/start.js:generic-api-key:409
f83f40146c4bd2eb6da9f7fdd7a8eab8fb660b13:js/views/modals/wallet/ReceiveMoney.js:generic-api-key:63
```

2. `js/utils/metrics.js` must preserve its public exports and non-loader behavior, but
   `addMetrics` becomes an immediate no-op and its entire Countly initialization/remote
   loader body is removed, including the detected literal. `js/utils/feedback.js` may be
   reduced to its public `addFeedback` export as an immediate no-op, removing all
   Doorbell setup/remote loader code and imports. Do not leave sensitive literals in dead
   code.
3. For the two sensitive source paths, use `apply_patch` full-file sanitized replacement
   if needed so the patch request contains only post-state and never reproduces an old
   detected value. Never print their Git diff. Tests use only structural identifiers and
   synthetic values.

Grok may now author `test/securityPolicy.node.js` completely first, targeting exactly
50 tests, then only `.gitleaksignore`, `js/utils/metrics.js`, `js/utils/feedback.js`,
`scripts/security-policy.js`, and `.github/workflows/security.yml`. No other source path
may change. It runs nothing, uses no Git, and reports test and production hashes/line
counts separately without file diffs or sensitive bodies.

Required new tests are exactly: strict eight-line ratchet bytes/content; exact dir-scan
presence/order/flags; and structural neutralization/no-loader/no-maintained-import. All
47 prior concepts remain, with existing Gitleaks cases converted for the second scan and
approved default ignore filename. Checker validation must reject every mutation listed
above and must read the committed `.gitleaksignore` in `checkRepository`.

## Oracle correction 01 — Authorized

Reviewer source inspection rejected one test mutation before execution. The synthetic
metrics-loader fixture is a standalone module containing only `addMetrics`; production
`checkInheritedLoaderNeutralization` correctly checks required public exports first, so
the test would fail for a missing export rather than the intended Countly/remote-loader
reason. This is vacuous for the mechanism it claims to prove.

Grok may edit only `test/securityPolicy.node.js`. Replace only that synthetic fixture so
it starts from the delivered `metricsSource` and structurally replaces the exact
`export function addMetrics() {\n}` no-op with the synthetic loader body. All public
exports must remain, causing the checker to reach and reject the loader-specific rule.
Keep exactly 50 tests. Do not change the synthetic token, any other test, production,
ratchet, workflow, sensitive source, or behavior. Run nothing and report the new test
hash and line count only.

## Oracle correction 02 — Authorized

Luna's restored green reached policy 49/50. The sole failure is semantic ordering in
`checkGitleaksRatchetBytes`: an appended valid ninth commit-qualified fingerprint is
correctly rejected, but the early exact-prefix/longer-buffer shortcut labels it
“trailing bytes” before parsing/counting can reach the intended “extra fingerprint”
invariant. The test oracle accepts only the mechanism-specific extra rejection.

Grok may edit only `scripts/security-policy.js`. Remove only the premature condition that
classifies any buffer longer than the exact body with the exact body as a prefix as
trailing bytes. Preserve every other validation and its order. A valid ninth newline-
terminated fingerprint must then parse and fail the existing extra-count rule; a
no-final-newline suffix must still fail trailing bytes, and blanks/malformed content must
still fail their existing rules. Do not edit tests, ratchet, workflow, loaders, or any
other path. Run nothing. Report only the new checker hash/line count and safe summary.

## Source delivery and reviewer acceptance

Grok authored the policy test file completely before all production paths and ran
nothing. Oracle 01 then changed only that test file. Final accepted hashes:

```text
3aea90640053e02dc8f1e4cbd1f9be257cc50010d132dd4f1c934236125c2d01  test/securityPolicy.node.js       1214 lines  50 tests
306def13efe09ee6d435dc5ad0b206e65731c3e536aa6e708a67c340dbb81fce  .gitleaksignore                       8 lines
c81417697b3a01dfbbd7c4ef71b7dd21665e999bc49dfdfeced02e276d7bc0c6  js/utils/metrics.js                 166 lines
376b0ff528931d4cb9db3c427600e5e4d89aa0218f4cff0e469692bb594e244d  js/utils/feedback.js                  2 lines
3a100adc33b7d95cc2e51c158d13d2640a2fd7baae7fe5fdca789d3e82858a09  scripts/security-policy.js          1587 lines
38836e829d2edcb9941fb40139d5e32146e781b5447b542791b8cbd64f5a3ca7  .github/workflows/security.yml        49 lines
```

Reviewer independently reproduced every hash, line count, and test count without
displaying sensitive-file diffs. Accepted source findings:

- fingerprint file is exactly eight lexically sorted commit-qualified lines and contains
  no global fingerprint, comment, wildcard, or secret body;
- metrics preserves its public exports while `addMetrics` is an empty no-op with no
  loader body; feedback is exactly its empty public `addFeedback` export;
- checker validates exact bytes, metadata, neutralized loaders, security path filters,
  and install → exact git scan → exact dir scan ordering;
- workflow preserves the pinned install/full-history scan and adds only the exact
  current-tree scan plus three path filters;
- the corrected synthetic mutation preserves all metrics exports and therefore reaches
  the intended remote-loader rejection; and
- no unauthorized path was changed.

Codex Luna may now integrate these exact hashes under Correction 04. It must perform only
Correction-04-specific red/falsifications, restore and hash-check after every bounded
patch, rerun all corrected gates, require both exact Gitleaks commands to return zero,
then perform the unchanged manual SBOM/evidence/commit/push sequence. Any failure or
finding stops before later work and Git.
