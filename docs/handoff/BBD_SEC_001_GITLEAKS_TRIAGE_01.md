# Reviewer Triage — BBD-SEC-001 Gitleaks 01

Owner: Lead Engineer/Reviewer — Codex

Integration is stopped. Luna's exact Gitleaks v8.30.1 complete-history command scanned
4,549 commits, reported 10 findings, and exited 1. No finding body was inspected or
recorded. No later SBOM/evidence/Git work ran.

This document authorizes reviewer triage only. It does not authorize a baseline,
suppression, source repair, deletion, cleanup, commit, push, or resumed acceptance.

Run the same full-history scanner with 100 percent redaction and write exactly one report:

```text
/home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/gitleaks-review-01.json
```

Exact command:

```sh
/home/lars/OpenBazaar/.security-tools/bbgo-sec-tools-20260829/gitleaks git \
  --redact=100 --no-banner --report-format json \
  --report-path /home/lars/OpenBazaar/.security-artifacts/bbd-sec-001-20260829/gitleaks-review-01.json \
  .
```

The expected nonzero finding exit is not acceptance. Afterward, inspect only safe
metadata: finding count, rule ID, repository-relative path, commit ID/date, line numbers,
and fingerprint. Do not print/read `Secret` or `Match` fields; verify only that they are
redacted. Determine whether every match is inherited and immutable, or whether any match
was introduced/retained by BitBook. Stop for a new correction if any active or BitBook
finding exists.

If all findings are inherited historical material, reviewer may propose an exact
time-bounded ratchet for separate authorization. It must preserve full-history scanning,
must fail on any additional fingerprint or count change, must not hide current-tree
secrets, must be local/offline and mutation-tested, and must keep its rationale and
expiry/removal condition in the durable ticket. No such ratchet is authorized by this
triage document.

Use no `/tmp`, root, `rm`, deletion, cleanup, or unresolved targets. Leave the redacted
artifact at its exact disk-backed path for owner inspection.
