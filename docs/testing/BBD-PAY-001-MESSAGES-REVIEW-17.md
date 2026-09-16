# Messages security evidence review 17

Reviewer: Codex. Decision: new PAY preload-policy integration blocker; reject the claim
that all four policy failures have unchanged inherited causes. Authorize the bounded
policy/test correction below. No tests, scans, syntax, builds or live RPC executed by
reviewer. HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

## Actual scan results

[Hermes scans report](BBD-PAY-001-HERMES-SCANS-01.md) and raw files in
`dist/pay001-scans01/` are present. All 21 current source/input hashes still match the
reviewed pins (review 15 overrides the driver). The 79-path snapshot exactly matches the
literal candidate scan list and original bytes at this review, before governance edits.
No missing/extra paths or differing files were found. This is a current comparison,
not a recovered pre-scan manifest or proof of the invocation's exact scope.

- npm-audit.json contains an empty vulnerabilities object and all zero severity counts.
  It reports 36 total dependencies. Accept it as retained zero-advisory output, without
  claiming an independently verified execution exit or advisory-query timestamp.
- Gitleaks version output is 8.30.1; report is [] and stderr says no leaks, approximately
  873815 bytes scanned. Accept the retained clean candidate output. The candidate scan
  does not cover later reviewer edits/new reports or any published commit.
- Policy raw stdout/stderr has 88 passing and four failing tests. The checker stops on
  `wallet-preload.js contains dynamic or unlisted IPC invoke`.
- The first two failed tests (repository workflows and Gitleaks ratchet integration)
  now have that IPC cause. Their prior cause was Rust root inventory. Matching failed
  test names/counts does NOT establish an unchanged baseline.
- The other two failures remain root Rust inventory and current ZEC inventory mismatch.
  Those inherited release blockers are preserved; this task does not authorize repair.

## Source diagnosis and correction contract

wallet-preload.js invokes seven fixed channels, including `payment:inbox:get` and
`payment:inbox:connect`. scripts/security-policy.js PRELOAD_INVOKE_CHANNELS still lists
only the five wallet channels. Its existing checker rejects unlisted literals and then
requires exactly one occurrence of every allowed channel. The application endpoints
were already source/boundary reviewed; this failure is omitted policy integration.
The present raw checker/integration failures are concrete red evidence.

Authorize only extending the literal policy list with those two exact channel strings,
plus independent test fixtures and regression/mutation coverage. Do not bypass the
checker, allow a payment prefix, weaken exact-once/dynamic IPC rejection, or change the
application bridge. The test change comes first; this retained red is already understood.
Hermes will verify focused green and an isolated removal-of-new-channels falsification
after source review. Broader policy can remain red on inherited Rust inventory causes.
This narrow repair does not claim complete maintained-source coverage for new PAY modules;
any further checker or scan-coverage integration need must be reviewed explicitly.

## Evidence closeout limitations

The new report acknowledges UI06 factual corrections but supplies no contemporaneous
executor recovery or explicit resolution of its metadata provenance. Scans01 again omits
actor/version captures, executor source, per-command metadata, before/after manifests,
maintained-input inventory and snapshot manifests. Exact process exits, timeout outcomes,
start/end times and invocation scope therefore remain actor-reported. Do not infer those
from empty logs or fabricate records after execution. Original artifacts remain intact.
No unchanged UI, Node, audit or secret-scan rerun is ordered for clerical completeness.
The forthcoming changed policy has its own new verification requirement.

## Verified raw evidence identities

Relative to dist/pay001-scans01/:

| Artifact | SHA-256 |
| --- | --- |
| security-policy.stderr.log | 8f8057aef3e650ce7fc915251dbfb1fadd67351162ef5458bc23b982b42ceb9d |
| securityPolicy.stdout.log | 72bf92f4ca20dfa087be4fea3938057989ab92e21c6f39bd7711122d584ca98f |
| securityPolicy.stderr.log | 2a2b2a1585250aade04b1b8b8eeff959c588cf354879f2da32d5c3ced6f89f42 |
| npm-audit.json | 998f715e597378b077de74a54d626023b067f6ccb422dd653e0eb01b2066aa08 |
| gitleaks-version.stdout.log | c9fd9ccb6682c54b5fcb0363757b6c6873564e7c067f70b3b5581b611528b9f4 |
| gitleaks.json | 37517e5f3dc66819f61f5a7bb8ace1921282415f10551d2defa5c3eb0985b570 |
| gitleaks.stderr.log | cd30c8a1e9fc45da51ef6db2a7d6d05fe1876730673eba060635a73bfa8984c8 |

## Owner availability update

Owner reports that monerod has now synced. Record this in WAL-007, WAL-009 and the
end-to-end status map. This supersedes the planning assumption that the owner is still
waiting for node synchronization. Network, endpoint, current height and wallet refresh
are not independently verified. No live node query, wallet operation or XMR source task
was performed. Existing Monero defects/review gates remain; node sync does not itself
establish adapter readiness. Preserve the active PAY task.

## Next authority

[Sol preload policy correction 01](../handoff/SOL_BBD_PAY_001_PRELOAD_POLICY_01.md),
two source/test files only, under the recorded Grok-usage-exhaustion fallback. Hermes
scans01 is closed. No execution by Sol; source review precedes new verification. Prior
UI/Node proof stands. No Git/publication, owner test, restart or live wallet operation.
Final acceptance is blocked by this new cause and the outstanding evidence disposition.

Governance: this review, Sol handoff, closed scans handoff, CURRENT_TASK, PAY ticket,
end-to-end map, and owner-availability notes in WAL-007/WAL-009 tickets.
