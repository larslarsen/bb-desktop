# Hermes — WAL-019 records and publication 01

Actor: Hermes, free Nous Portal model, manually relayed by owner. Read AGENTS.md,
TESTING.md, CURRENT_TASK.md, WAL-019 ticket and
docs/testing/BBD-WAL-019-GREEN-REVIEW-01.md. Runtime and local wallet refresh are
reviewer accepted. This handoff closes records and publishes the bounded change.

## Preflight and invariants

Require local master HEAD `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` and an empty
staging index. Verify all ten pins in HERMES_BBD_WAL_019_SYNC_GREEN_01.md and both
artifact hashes in the accepted green review. If any mismatch, stop. Do not rebuild
or rerun tests to fix records. Preserve all unrelated working-tree files/captures.

Read-only preflight commands are authorized: git status --short, git rev-parse HEAD,
git diff --cached --name-only, git branch --show-current, git remote -v, file
hash/size reads, and git ls-remote origin refs/heads/master. Require origin to be
the larslarsen/bb-desktop repository and remote master to equal the baseline before
committing. On drift, stop; do not pull, merge, force-push or change remotes.

## Authorized record corrections

Edit only these existing records:

- `docs/testing/BBD-WAL-019-SYNC-GREEN-01.md`: apply every correction in the green
  review. Copy actual argv/exits from raw JSON, distinguish restoration verification
  from an executed command, include full mutant/binary/manifest hashes, exact byte
  sizes, source restoration proof, and clickable relative raw links with hashes.
  Say account_native_ui target, not the entire native UI/Rust suite. Report that the
  developer resource was refreshed, the app was not restarted and the daemon was
  unchanged. Keep session identity explicitly unavailable if unrecoverable.
- `docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md`: clarify that production became
  authorized only by the later expected-red review; make raw artifact links clickable.
  Preserve all actual red result values and captures.

Create `docs/testing/BBD-WAL-019-PUBLICATION-01.md` with actual actor/version/model,
baseline and pin verification, the exact staged path list, scanner executable
identity, command/exits and any anomalies. Record test acceptance by linking the
green review, without claiming a new execution. No control-plane/source edits.

## Exact publication path set

The commit must contain exactly these 20 paths; no broad git add. Whole accepted
test-file bytes include the previously reviewed mechanical lint corrections already
present in its pinned starting state. Do not include other WAL-009 changes.

```text
wallet-broker/src/account_ui.rs
wallet-broker/tests/account_native_ui.rs
tickets/BBD-WAL-019.md
docs/handoff/CURRENT_TASK.md
docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md
docs/architecture/BBD-WAL-MOBILE-DESIGN-DIRECTION-01.md
docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_TESTS_01.md
docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_TESTS_CORRECTION_01.md
docs/handoff/GROK_BUILD_BBD_WAL_019_SYNC_PRODUCTION_01.md
docs/handoff/HERMES_BBD_WAL_019_SYNC_EXPECTED_RED_01.md
docs/handoff/HERMES_BBD_WAL_019_SYNC_GREEN_01.md
docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_01.md
docs/testing/BBD-WAL-019-TEST-SOURCE-REVIEW-01.md
docs/testing/BBD-WAL-019-TEST-SOURCE-REVIEW-02.md
docs/testing/BBD-WAL-019-EXPECTED-RED-REVIEW-01.md
docs/testing/BBD-WAL-019-PRODUCTION-SOURCE-REVIEW-01.md
docs/testing/BBD-WAL-019-GREEN-REVIEW-01.md
docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md
docs/testing/BBD-WAL-019-SYNC-GREEN-01.md
docs/testing/BBD-WAL-019-PUBLICATION-01.md
```

No generated executables/manifests, ignored captures, root private planning notes,
AGENTS.md, TESTING.md, dependency/policy files or other source enter this commit.

## Scan, commit and push

Verify the existing scanner `target/security-tools/gitleaks-v8.30.1/gitleaks` has
size 21958840 and SHA-256
`88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`.
Run its `version` command and record the actual result. Do not fetch/replace tools
or execute historical full-tree scanner commands from other handoffs.

Stage only the exact 20 paths with `git add --` and their explicit names. Run
`git diff --cached --check` and inspect `git diff --cached --name-only` against the
exact set. Verify staged source bytes match the accepted hashes by hashing the
output of `git show :wallet-broker/src/account_ui.rs` and
`git show :wallet-broker/tests/account_native_ui.rs` without altering line endings.

Run the following staged-content scan and capture its real output/exit in a new
ignored directory `wallet-broker/target/wal019-publication-01`. Do not overwrite
prior captures. Standard-library capture code may create only that directory and
its metadata/logs; it may not modify the command, ignore findings or rerun tests.

```bash
target/security-tools/gitleaks-v8.30.1/gitleaks git --pre-commit --staged --redact=100 --ignore-gitleaks-allow --no-banner .
```

Require exit 0 and zero findings. Any finding, including a suspected fixture/hash
false positive, stops publication for reviewer classification; no suppression,
allowlist, baseline bypass or source edit is authorized. This is a scoped new-content
scan, not a clean full-history/security-release claim. Inherited blockers remain.

Add the scan's actual result to the publication report, restage that report, and
repeat the same scan on the final staged bytes. Require a second exit 0; no edits
after this final scan. Recheck exact staged paths, whitespace, accepted source hashes
and HEAD. Then execute:

```bash
git commit -m "Make native wallet sync start with one click"
git push origin HEAD:master
git rev-parse HEAD
git show --format=fuller --stat HEAD
git ls-remote origin refs/heads/master
git status --short
```

No amend, force push, unrelated commit, cleanup or process restart. If push fails,
report the local commit and actual error; do not change history. Read-only GitHub
Actions inspection for the resulting commit is permitted; record exact workflow
head/status/conclusion without rerunning workflows or claiming unrelated CI green.
Return the feature commit, verified remote ref, report path and remaining dirty
paths. Commit/remote results may be reported in the completion message and ignored
capture; do not amend the scanned commit to insert its own hash. Codex performs
final control-plane closeout after verifying publication.
