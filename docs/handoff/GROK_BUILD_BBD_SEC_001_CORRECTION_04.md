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
