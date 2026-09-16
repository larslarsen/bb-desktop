# PAY preload policy source review 18

Reviewer: Codex. Decision: bounded correction accepted for focused verification.
No tests, syntax checks, scans, builds or live RPC executed by reviewer.
HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`; index empty.

| Changed path | Lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2808 | c5431ee340564af26f0d3d2f5f19a3aa72869a45d9b3b0d64353a7da266732cb |
| test/securityPolicy.node.js | 3922 | 29e4e213c803ca7792fae51cff99aaa61f96abdf43830157fda50c89292799fd |

Removing only the two appended payment channel literals from the policy reproduces
its authorized before hash `0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d`.
Reversing the test fixture additions, new scaffold/mutation helper and two new groups
reproduces its before hash `ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c`.
These reversals were in-memory text/hash comparisons only; source was not modified or
executed. Older dirty policy/test work is preserved exactly. The other 19 input pins
are unchanged, including review-15's Electron driver.

The policy's sole change appends payment:inbox:get and payment:inbox:connect to the fixed
invoke list. Existing exact-once, dynamic/unlisted invoke, generic IPC, subscription,
network/process/import and authority rejection logic is unchanged.

Tests independently list seven channels and provide a complete synthetic bridge. The
new positive group checks both that scaffold and actual wallet-preload.js bytes. The
negative group first validates its scaffold; for each of seven channels it independently
removes, duplicates and computes the invocation. It also rejects three unlisted literals,
generic send/sendSync and mismatched subscription removal. Mutation-target uniqueness,
changed-byte checks, PolicyError type and specific rejection messages prevent unrelated
failures from satisfying those assertions. All older negative cases remain. Source has
94 registered groups, exactly two more than before; no runtime count is claimed yet.

## Verification detail

[Review 17](BBD-PAY-001-MESSAGES-REVIEW-17.md) retains the original unlisted-IPC red.
Authorize focused green, isolated old-list falsification, restored focused green and
one broader checker/policy diagnostic. The new positive group checks the channel list
before reading the real bridge, so removing the two literals makes that group fail at
its independent list comparison first. Also invoke the copied checker directly on the
unchanged real preload copy to prove the specific unlisted-IPC failure. Do not claim the
group reached its later real-source check when its first assertion failed.

The full suite is expected to have 94 groups; four inherited inventory failures may
remain (90 pass / 4 fail), but compare reasons, not counts. The payment IPC rejection
must disappear. Other newly exposed failures stop acceptance rather than expanding scope.
This correction does not resolve the separate new-module scan-coverage question or
historical execution-metadata gaps. Final acceptance/publication remains pending.

Next authority: [Hermes preload policy verification 01](../handoff/HERMES_BBD_PAY_001_PRELOAD_POLICY_VERIFY_01.md).
Sol policy source task is closed. The handoff supplies a capture executor to record real
process metadata automatically. No source repair, UI/unchanged Node/audit rerun, broader
scan, Git, owner testing or user restart. Prior UI/audit/secret output remains as reviewed.
Owner-reported monerod sync and identity/WebRTC/video requirements remain recorded.

Governance paths: this review, new Hermes verification handoff, closed Sol handoff,
CURRENT_TASK, PAY ticket and end-to-end status map.
