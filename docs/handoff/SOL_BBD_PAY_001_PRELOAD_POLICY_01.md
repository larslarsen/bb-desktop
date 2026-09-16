# Sol — PAY preload policy integration 01

CLOSED: source accepted for verification in [review 18](../testing/BBD-PAY-001-MESSAGES-REVIEW-18.md).
Next authority: [Hermes preload policy verification](HERMES_BBD_PAY_001_PRELOAD_POLICY_VERIFY_01.md).
Instructions below are historical.
Actor: Codex Sol (`gpt-5.6-sol`, High), owner-relayed. Reviewer: Codex.
Grok remains unavailable after weekly usage exhaustion. Read AGENTS.md, TESTING.md,
CURRENT_TASK and [review 17](../testing/BBD-PAY-001-MESSAGES-REVIEW-17.md).

## Two-file scope

HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index.
Only these existing dirty files may change; preserve their previous work:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| scripts/security-policy.js | 2806 | 0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d |
| test/securityPolicy.node.js | 3816 | ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c |

Other 19 review-05 input hashes stay frozen with the review-15 Electron-driver override.
Do not edit preload/main/UI/transport, Rust inventories or capability checks, dependencies,
workflows, ignore files, documentation or reports. No tests/syntax/formatter/build/scans,
Git or agent launches. The old WAL-009 inventory handoff is historical, not authority.

## Exact behavior

The real seven-channel preload must pass its static boundary checker. Existing five
wallet channels plus only `payment:inbox:get` and `payment:inbox:connect` are allowed.
Every channel must still occur exactly once. Computed channels, arbitrary payment/wallet
channels, missing or duplicate invokes, generic IPC and incorrect subscriptions fail.
The existing checker algorithm already enforces this; only its literal list is stale.

## Tests first, then the bounded policy edit

Retained scans01 checker and two integration failures are the understood red evidence:
`wallet-preload.js contains dynamic or unlisted IPC invoke`. Author regression tests
before updating the policy list; do not execute them in this source phase.

1. Extend the test's independent PRELOAD_INVOKE_CHANNELS fixture with the two exact strings
   in the same appended order. Extend the existing valid synthetic preload with one invoke
   each. Preserve every existing import/process/network/subscription negative assertion.
2. Add exactly two top-level groups:
   - `PAY-001 preload policy accepts the exact seven channels and real bridge`
   - `PAY-001 preload policy rejects missing duplicate computed and unlisted channels`
3. First group: assert the exported policy list equals the independent seven-string test
   fixture, length seven, no duplicates; pass both a complete independently built synthetic
   preload and actual wallet-preload.js bytes through checkWalletBoundarySource. Never
   derive the expected fixture from the policy export or alter actual preload bytes.
4. Second group: start from a complete valid synthetic seven-channel preload with correct
   import, exposeInMainWorld, on/removeListener structure. Independently mutate each of
   the seven invoke occurrences: remove it; append a duplicate; replace with a computed
   channel variable. Require PolicyError for each case. Assert each textual mutation
   matched exactly once and actually changed the valid source.
   Also append an unlisted `payment:inbox:other`, `payment:send`, and `wallet:anything`
   invoke in separate cases; append generic send/sendSync and replace the subscription
   removal channel with a mismatched literal. Require rejection for the relevant IPC/
   exact-channel/subscription cause. Keep the rest of the valid scaffold in these cases
   so an unrelated missing-required-channel failure cannot make negatives pass vacuously.
5. Only after authoring those tests, append the two literals to production
   PRELOAD_INVOKE_CHANNELS, after wallet:payee-request:get:
   `payment:inbox:get`, then `payment:inbox:connect`.
   No other change to scripts/security-policy.js. No prefix allowance, regex weakening,
   alternate checker, inventory repair, removed required channel or source skip.

## Verification contract for the next phase

Stop after source edits; reviewer compares the change and pins both files. Hermes will
run syntax, the existing boundary group plus the two new groups, and isolated falsification
that removes ONLY the two new policy literals in a copy and demonstrates the real-bridge
positive regression fails for unlisted IPC. Then restored focused green and one broader
checker/policy diagnostic. Full suite will have 94 registered groups; inherited inventory
failures remain failures even if their names/counts still match old output.
No passing UI/Node suites or npm audit rerun is needed for this list/test-only change.
Fresh final-byte secret coverage is required before future publication. No owner manual
test is required. monerod synced is an owner availability update, not a task change.
