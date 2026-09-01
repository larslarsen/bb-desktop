# BBD-WAL-006 Prepare Serde Feature Expected-Red Evidence Review 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Reviewed integration: `9b3ab712`

Result: **EVIDENCE CORRECTION REQUIRED — PRODUCTION STILL PAUSED**

Free Jr Dev — Hermes (`nous`, `meituan/longcat-2.0:free`) integrated exactly the authorized test,
evidence, and task paths at `9b3ab712`; the four production paths remain unstaged and uncommitted,
and `HEAD == origin/master`. The accepted 68/7 red itself remains valid.

The evidence document has three record defects:

1. It states the sole command was `npm test` and describes the full npm script. The only executed
   command was exactly `node test/securityPolicy.node.js`.
2. Its protected-identity block records only untracked `prepare.rs` without the exact identities of
   all four accepted production paths. It must record the four line counts/hashes from Prepare
   Format Correction Review 03.
3. It calls the worktree diff inventory exactly the one changed test. The test-only source delta is
   5 insertions/5 deletions, but the pre-integration worktree inventory was five paths: that test
   plus the four accepted production paths. Those facts must be distinguished.

`CURRENT_TASK.md` also leaves Hermes as the authorized integration actor after its completed commit.
The correction must set both actors to none and make the expected-red evidence correction complete,
review required. No command rerun or other edit is authorized.
