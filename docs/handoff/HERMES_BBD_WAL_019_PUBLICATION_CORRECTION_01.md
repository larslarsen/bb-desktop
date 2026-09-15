# Hermes — WAL-019 publication correction 01

Actor: Hermes, free Nous Portal model, manually relayed by owner. Read AGENTS.md,
TESTING.md, CURRENT_TASK.md, HERMES_JR_DEV_ROUTING.md in docs/engineering, and
[publication review 01](../testing/BBD-WAL-019-PUBLICATION-REVIEW-01.md).
Earlier publication authorization is closed. Runtime acceptance stands.

## Baseline and boundaries

Require local master and live origin master at
`f2d9c9a513b3285531352df84ada818fdac0faf6`, parent
`2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`, empty staging index, and origin
`https://github.com/larslarsen/bb-desktop.git`. Stop on drift without pull/merge.
Verify all ten green-handoff source pins, both binary hashes and manifest from the
accepted green review. Preserve all unrelated changes. Read-only Git, file/hash
inspection and exact-commit GitHub Actions inspection are allowed.

No source/test edits, tests, formatting, rebuild, dependency changes, process
restart, wallet-data access, new actors, history rewriting or cleanup. Do not alter
old raw captures. Inspect filesystem type before creating the new exclusive capture
directory `wallet-broker/target/wal019-publication-correction-01`; require a real,
disk-backed target directory. Standard-library capture code may write only this
new directory and the four executor-owned reports below. If it already exists,
stop and report; do not overwrite or silently reuse it.

Capture actual argv, UTC start/end, exit code, combined output, log SHA-256 and
relevant Git identities for every executed gate/publication command. Record actual
Hermes version/provider/model; an unavailable session ID stays unavailable. Never
infer historical results from today's commands or version output.

## Record corrections

Hermes may edit only these existing reports:

1. `docs/testing/BBD-WAL-019-SYNC-GREEN-01.md`: copy the actual mutation digest from
   raw JSON; include complete captured argv arrays (including timeout) and exits
   for every command. Keep restoration/artifact snapshots distinct from commands.
   Generate SHA-256 values for every linked raw file, replacing all placeholders.
   Use clickable relative links, e.g.
   `../../wallet-broker/target/wal019-sync-green-01/04-mutation.json`.
   Preserve correct runtime counts, binary sizes/hashes and process-refresh scope.
2. `docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md`: link every raw capture with
   its computed digest; remove the contradictory premature production authorization
   in Outcome and attribute it solely to the later expected-red review.
3. `docs/testing/BBD-WAL-019-PUBLICATION-01.md`: label historical scan exits and
   second-scan coverage unverified from retained evidence. Describe exactly the
   single retained log and absent metadata. Do not claim a version command ran
   without captured proof. Distinguish original preflight baseline from published
   feature HEAD/remote ref. Add actual feature commit, verification source and
   scoped coverage caveat, linking the new correction report. Correct the copied
   green preflight/restoration wording so it does not imply publication captures.

Create only `docs/testing/BBD-WAL-019-PUBLICATION-CORRECTION-01.md` for new execution
evidence: actor identity, baseline/source/artifact/scanner verification, exact paths,
commands/exits, raw links/digests, actual new scan results and historical limitation.
No new test claim. Keep inherited release blockers and payments status explicit.

The following four reviewer-authored files are frozen; stage their existing bytes
as part of this correction without editing them:

- `docs/testing/BBD-WAL-019-PUBLICATION-REVIEW-01.md`
- `docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_CORRECTION_01.md`
- `docs/handoff/CURRENT_TASK.md`
- `tickets/BBD-WAL-019.md`

## New published-commit scan

Verify scanner `target/security-tools/gitleaks-v8.30.1/gitleaks`: 21958840 bytes,
SHA-256 `88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`.
Run and capture its actual `version` output/exit (report the output literally even
if it differs from the directory name). Do not fetch or substitute tools.
Run from bb-desktop and capture:

```bash
target/security-tools/gitleaks-v8.30.1/gitleaks git --log-opts=2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e..f2d9c9a513b3285531352df84ada818fdac0faf6 --redact=100 --ignore-gitleaks-allow --no-banner .
```

Require exit 0, one commit scanned, zero findings. This is a new scan of the
published feature diff, not a retrospective proof of the historical staged scan
or a full-tree/release scan. Any finding or unexpected scope stops work for Codex;
no bypass, suppression, allowlist or tool/config change is authorized.

## Exact corrective publication

Only these eight paths may enter the corrective commit:

```text
docs/testing/BBD-WAL-019-SYNC-GREEN-01.md
docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md
docs/testing/BBD-WAL-019-PUBLICATION-01.md
docs/testing/BBD-WAL-019-PUBLICATION-CORRECTION-01.md
docs/testing/BBD-WAL-019-PUBLICATION-REVIEW-01.md
docs/handoff/HERMES_BBD_WAL_019_PUBLICATION_CORRECTION_01.md
docs/handoff/CURRENT_TASK.md
tickets/BBD-WAL-019.md
```

Finish reports using actual captured results; verify raw links resolve, digests
match, no placeholders remain, and argv arrays equal their metadata. Stage with
`git add --` and the eight explicit paths. Capture `git diff --cached --check`,
`git diff --cached --name-only` and `git write-tree`. Require clean whitespace and
the exact path set. Capture this staged scan:

```bash
target/security-tools/gitleaks-v8.30.1/gitleaks git --pre-commit --staged --redact=100 --ignore-gitleaks-allow --no-banner .
```

Require exit 0 and zero findings. Add this first staged scan's actual results and
raw hashes to the correction report, restage only that report, then capture the
same scan again on final bytes. Require a second exit 0 and zero findings. Capture
the staged tree before and after this final scan and require equality. Do not edit
reports after the final scan or pre-claim its success inside the report; its result
belongs in ignored capture metadata and the completion message. Recheck whitespace,
eight paths, frozen source/artifacts, HEAD and live remote before committing.

```bash
git commit -m "Correct WAL019 publication evidence"
git push origin HEAD:master
git rev-parse HEAD
git show --format=fuller --stat HEAD
git ls-remote origin refs/heads/master
git status --short
```

Capture actual results and verify committed tree equals the final scanned tree.
No amend or force push. If push fails, retain the local commit and report the error.
Return correction commit/remote, report path, final scan/tree proof and remaining
dirty paths. Stop for Codex verification; do not mark the ticket complete yourself.
