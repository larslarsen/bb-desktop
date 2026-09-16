# Hermes — Messages evidence closeout and security scans 01

CLOSED: reviewed in [review 17](../testing/BBD-PAY-001-MESSAGES-REVIEW-17.md).
New preload IPC rejection requires [Sol policy correction](SOL_BBD_PAY_001_PRELOAD_POLICY_01.md).
No unchanged rerun. Instructions below are historical.
Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[review 16](../testing/BBD-PAY-001-MESSAGES-REVIEW-16.md).
Sole execution authority. Older execution/source handoffs remain closed.

## Scope and baseline

bb-desktop HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Preserve all unrelated dirty work. Require the 21 review-05 inputs, with only the driver
pin overridden by review 15: 1599 lines, SHA-256
`f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54`.
No product/test/policy/dependency edits, Git mutation, native build, live daemon/wallet,
user profile access, funds, process restart or actor launch. No UI or passing Node reruns.

Writable report: docs/testing/BBD-PAY-001-HERMES-SCANS-01.md.
New ignored captures/snapshot: dist/pay001-scans01/ and dist/pay001-scan-input01/.
If either exists, inspect/report rather than overwrite or repeat. Verify disk-backed
storage/free space before copying. The installed runtime remains read-only; no recopy.
Capture tooling under the new ignored directory is allowed; retain its exact source.

## 1. Close UI06 evidence honestly, without rerunning

In the new report, add the corrections from review 16: Enter toggles details, traffic
samples are truncated (12/12/8), all-200 totals lack retained status proof, 154G raw disk
space disagrees with the summary, after/runtime manifests and executor source are absent.
Address the prelaunch 01:45:00Z gate claiming syntax completion recorded at 01:45:01Z.
If actual contemporaneous executor/tool records exist, recover their relevant command,
exit and timestamp captures with provenance into the new directory. Limit any session
inspection to the actor's relevant UI06 execution; exclude unrelated conversation,
credentials, descriptors and environment. Otherwise state unavailable explicitly.
Do not invent times/statuses or overwrite UI06 metadata. A new runtime inventory is a
current observation only, never execution-time recovery. Do not create old before/after
files after the fact. Missing historical records do not authorize unchanged test reruns.

## 2. Prepare actual new execution evidence

Capture real actor version/provider/model, Node/npm versions, cwd, HEAD/index/status;
write source-before.json with 21 actual hashes/line counts. Hash maintained files read
by the policy check as an additional current input inventory, excluding generated caches.
Verify all frozen pins before commands. Save actual UTC start before every spawn, exact
argv (including timeout wrapper), cwd and only relevant environment, raw separate stdout/
stderr, observed wrapper exit/signal/timeout, end after stream closure, both log hashes.
Unknown child status stays null. Metadata must come from the executor's observed process
result and clock, not handwritten estimated times. Retain executor source. On each command
failure finish its metadata and continue only the independent checks authorized below.
Use finally to write source-after.json and HEAD/index/status checks even after scan failure.

## 3. Remaining commands (run once each, sequentially from bb-desktop)

```text
timeout --signal=TERM --kill-after=5s 120s node scripts/security-policy.js
timeout --signal=TERM --kill-after=5s 180s node test/securityPolicy.node.js
timeout --signal=TERM --kill-after=5s 180s npm audit --json
timeout --signal=TERM --kill-after=5s 30s target/security-tools/gitleaks-v8.30.1/gitleaks version
timeout --signal=TERM --kill-after=5s 180s target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact --report-format json --report-path dist/pay001-scans01/gitleaks.json dist/pay001-scan-input01
```

Before the Gitleaks commands, require its installed binary to be a regular non-symlink,
21958840 bytes, SHA-256
`88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`.
Version must output 8.30.1 before scanning. No tool install/update is needed.
Normal tool escalation for npm advisory access is allowed if confinement blocks it;
never run npm audit fix/install or change the lockfile. Record access failure as a blocker.
Timeout/forced termination is failure. Keep raw npm JSON separately; nonzero advisory
exit is a finding to report, not a reason to discard output. No suppressions/allowlist edits.

### Exact scan input

Use only the literal paths in [scan path list](BBD_PAY_001_SCAN_PATHS_01.txt), one relative
path per line. No actor glob expansion or extra dirty files. Every listed path must exist
as a regular non-symlink file inside bb-desktop; missing/extra/changed inputs stop copying.
Copy the unchanged bytes with repository-relative paths into the new snapshot, rejecting
symlinks/special files. Retain scan-source-manifest.json and scan-copy-manifest.json with
relative path/size/SHA-256; require exact path-set and hash equality before the scan, and
hash the path list itself. Recheck the snapshot and original listed files after scanning.
Never place captures, installed dependencies, private runtime profiles or UI home files
in the snapshot. Do not stage anything.

This list is the prospective PAY feature/governance scan set only. Shared dirty package,
policy and agent-role files are preserved validation inputs, not silently selected for
publication. Final Git scope/dependency review is still a reviewer decision. New reports
and later reviewer closeout records require their own final-byte scan before publication;
this candidate scan must not be advertised as covering future bytes or a commit.

### Thresholds and inherited baseline

Require zero Gitleaks findings. Record exact dependency advisories/severity/affected
versions and distinguish newly observed advisories from unchanged dependency bytes.
Any potentially exploitable advisory or new policy failure requires reviewer disposition.
The inherited policy result is 88 pass / 4 fail (92 total), with these exact failed names:

- committed workflows satisfy the fail-closed checker;
- strict nine-line reviewed Gitleaks ratchet bytes and content are enforced;
- WAL-004 Rust source inventory is exported closed and enumerated by repository policy;
- BBD-WAL-008 closes the hardware target and current fourteen-path WAL-008/WAL-009 ZEC policy inventory.

Baseline: docs/testing/BBD-WAL-009-EXTRACTOR-POLICY-ACCEPTANCE-01-REVIEW.md.
Checker failure and these failures remain release blockers, not green gates or waived
findings. Compare exact reasons, not only counts; any extra failure is a new blocker.
Independent npm/Gitleaks scans may still run after policy findings to complete the report.
Do not fix source, expand scans to unrelated Rust/network suites or infer acceptance.

## Finish

The report must link actual metadata/raw logs, snapshot manifests, findings, source
before/after hashes, actor/version output and the UI06 evidence addendum. Include actual
command results, exact failure names and missing files. Preserve originals and return
the report pointer. Codex will decide final acceptance and publication scope; neither
is granted here. No owner manual test is required for these remaining checks.
