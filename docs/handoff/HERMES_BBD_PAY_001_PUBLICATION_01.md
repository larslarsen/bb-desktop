# Hermes — PAY publication and closeout 01

CLOSED — feature `f23950245` published. No further execution authorized.

## Frozen baseline and scope

Require branch master, HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Origin must be exactly https://github.com/larslarsen/bb-desktop.git; never use upstream.
Require `git ls-remote origin refs/heads/master` to equal that starting HEAD. Any remote
movement stops before staging; no fetch/merge/rebase/reset/force-push or automatic repair.
Normal tool escalation for already-authorized Git/network access remains available.

Require all 21 review-05 source hashes with review-15 driver and review-18 policy/test
overrides. Reference capture source-before.json in dist/pay001-policy-verify01/ has all
21 accepted hashes/line counts. Independently compare to the reviews/current files.
The accepted 95-file manifest is dist/pay001-publication-prep01/candidate-copy.json,
SHA-256 `dae69a08f036bfd3e7ef244658248a8109da426606c09b6e3696be0be7980ee3`.
Compare current original files with it. Only the four governance changes listed in
review 20 are allowed differences; source drift is never allowed.

Feature staging scope: ONLY the literal paths in
[BBD_PAY_001_PUBLICATION_PATHS_02.txt](BBD_PAY_001_PUBLICATION_PATHS_02.txt).
No glob expansion, git add -A, git add ., partial hunk improvisation or unrelated files.
The 17 product/test/package/policy inputs are the complete reviewed current bytes;
selection does not authorize source changes. Preserve unrelated tracked/untracked work.
No tests, UI runs, audit reruns, source repair, dependency install, native build, process
restart, wallet/RPC/funding operation or release. No new actor launch.

New ignored capture directory: dist/pay001-publication01/. If it already contains
execution records, inspect and stop rather than overwrite/repeat. Confirm disk-backed
storage and free space first. Retain the exact capture executor used.
New durable report: docs/testing/BBD-PAY-001-PUBLICATION-01.md (phase B only).

## Automatic capture requirement

Use the working Python `run` capture function from
`dist/pay001-publication-prep01/executor.py` verbatim: actual start saved before spawn,
exact argv/cwd, observed timeout-wrapper status, end after streams close and raw-log
hashes. Copy that function into a new executor; do NOT import/run the old executor.
All Git mutations and scanner commands must go through it with unique stage labels.
Use argument arrays, no shell interpolation. Reuse its manifest/Git helpers as needed;
new orchestration is allowed only for the exact checks/mutations in this handoff.
No tests or application logic may be authored. Preserve all command metadata, failed
commands and manifests. Do not handwrite guessed metadata, reuse old exits/times, or
claim a later stage ran after early failure. Report actual Hermes version/provider/model.
Capture read-only Git checks and scanner identity along with outputs.

Before any staging, record baseline HEAD/index/status and hashes/types/modes of all dirty
tracked/untracked files outside the feature and phase-B paths. Never read ignored user
profiles or credentials. Recheck those outside files after each phase to prove unrelated
work was preserved. Abort on staged files at entry. On later failure retain owned staged
state and report it; no broad reset/cleanup. Never overwrite original captures.

## Phase A — scan and publish the feature

1. Validate every listed file is regular, non-symlink, repository-relative and resolves
   inside this repository. Freeze source manifest with path/size/mode/SHA-256, then copy
   exactly those bytes to dist/pay001-publication01/feature-snapshot/ with relative paths.
   Check exact file-set equality and source/copy hashes. Hash the literal path list.
2. Verify scanner regular-file size 21958840 and SHA-256
   `88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`.
   Use the exact commands below through the capture function. Version must be 8.30.1;
   dir scan must exit 0 and its JSON must be []. No suppressions or fallback scanner.

```text
timeout --signal=TERM --kill-after=5s 30s target/security-tools/gitleaks-v8.30.1/gitleaks version
timeout --signal=TERM --kill-after=5s 180s target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner --report-format json --report-path dist/pay001-publication01/feature-gitleaks.json dist/pay001-publication01/feature-snapshot
```

3. Recheck source and copy hashes against the scanned manifest. Recheck HEAD/master,
   empty index and origin/master baseline. Stage with argv `['git','add','--', ...paths]`
   (120-second limit). Compute the expected changed path set by comparing the listed
   current bytes/modes with starting HEAD, including new files. Require staged path set
   to equal it; for EVERY listed path require staged blob bytes/modes to match the scanned
   manifest, including listed files already unchanged in HEAD. Keep staged evidence.
   Run `git diff --cached --check` through the recorder, require exit 0.
4. Commit via `git commit -m "feat: show payment requests in automatically updated Messages"`
   (120-second limit). Require exactly one parent equal to starting HEAD; verify committed
   changed paths equal the expected staged set, and every listed committed blob matches
   the scanned bytes/modes. Record commit and tree IDs, manifests and actual Git output.
   On hook failure or unexpected mutation stop; never bypass hooks or amend.
5. Confirm origin/master still equals the starting parent. Push with
   `git push origin HEAD:refs/heads/master` (180-second limit), without force. Verify
   `git ls-remote origin refs/heads/master` equals the feature commit and local HEAD is
   unchanged. Record branch, remote URL, exact remote hash and empty index. Only now is
   feature publication established. This is not release/CI-green proof.

## Phase B — actual completion records, scan and publish them

After phase A succeeds, write ONLY these five paths:

- docs/testing/BBD-PAY-001-PUBLICATION-01.md (new actual report);
- docs/handoff/CURRENT_TASK.md (replace active prefix with completed PAY status);
- tickets/BBD-PAY-001.md (completed/published status prefix);
- docs/architecture/BBD-PAY-END-TO-END-STATUS-01.md (received-request status);
- docs/handoff/HERMES_BBD_PAY_001_PUBLICATION_01.md (mark this handoff CLOSED).

Report the real feature commit/parent/tree, origin/master verification, exact feature
path count and manifest hash, actual scan result and capture links. Include acceptance
references (reviews 16, 19, 20); explain read-only received requests in Messages and
automatic updates. Retain four inherited Rust policy release blockers and historical
UI/audit metadata limits. Do not imply request creation, native approval, money transfer,
Monero adapter readiness or a release is complete. Preserve the owner-reported synced
monerod and identity/readable-name/WebRTC/video direction and the requirement to settle
account/device addressing before next creation/approval contract or mobile enrollment.
No next implementation is automatically authorized by closure.

The report records already-observed feature publication. Do not put this documentation
commit's own future hash, scan success or push result in its pre-commit body. State that
its final scan/commit/push evidence will be in the ignored publication capture. No circular
claim that the report contains its own final hash. Do not edit the report after scanning.

Capture the exact five final records in closeout-snapshot/ and source/copy manifests.
Scan them with the same pinned scanner (180-second limit):

```text
timeout --signal=TERM --kill-after=5s 180s target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner --report-format json --report-path dist/pay001-publication01/closeout-gitleaks.json dist/pay001-publication01/closeout-snapshot
```

Require exit 0, [] and unchanged source/copy hashes. Require HEAD and origin/master
still equal the verified feature commit and an empty index. Stage ONLY the five paths;
require exact expected changed-set and scanned/staged blob/mode equality as in phase A.
Run staged diff check, then commit:
`git commit -m "docs: close received payment request publication"` (120 seconds).
Verify the single parent is the feature commit; exact changed set and all five committed
files must match the scanned closeout manifest. Recheck remote baseline, push normally
with the same command, and verify origin/master equals this documentation commit.
On failure stop with retained evidence; do not force, amend or expand scope.

## Final verification and return

Retain result.json with both actual commit/tree IDs, both manifest hashes, both actual
zero-finding scan results, final remote hash, index state and unchanged-outside-scope
verification. Confirm final history has exactly these two new commits from the starting
parent. For all feature-list files not changed in phase B verify final committed bytes
still match the feature manifest; for the five phase-B files use the closeout manifest.
Confirm all 21 product/test/input pins remain unchanged in the working tree. Preserve
unrelated dirty work. Any branch/remote drift or failed verification is a blocker.

Return the durable publication report pointer and the actual two commit IDs. No new
handoff or owner confirmation is needed for any successful step already authorized here.
Reviewer checks the finished evidence; no more routine execution is expected. Do not
claim a release, clean global policy, fresh advisory scan or running-process refresh.
