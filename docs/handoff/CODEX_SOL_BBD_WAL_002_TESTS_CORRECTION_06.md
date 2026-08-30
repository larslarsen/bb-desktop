# Codex Sol Handoff — BBD-WAL-002 Test Source Correction 06

You are temporary **Sr Dev — Codex Sol**, using `gpt-5.6-sol` at High. This file is the
complete durable test-correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-002.md`,
`docs/architecture/BBD-WAL-001-REVIEW.md`, `docs/handoff/CURRENT_TASK.md`, Corrections
04–05, the correction-red evidence, Production Correction 01, and the current accepted
test plus current unaccepted production modules.

Production Correction 01 closes its seven demonstrated red causes but is not accepted.
XHigh source review found three residual fail-open paths. Tests must lead. Modify only:

- `test/walletContract.node.js`

Preserve all 45 accepted tests, fixtures, and protected hashes. Add the smallest
behavioral coverage for exactly these requirements:

1. **Durable signed-unverified recovery gate.** Construct a machine from a synthetic
   durable `signed_unverified` state with a valid review, intent hash, confirmation count,
   and signed artifact. A direct `verifySigned()` must fail without calling adapter
   verify, must remain `signed_unverified`, and cannot lead to broadcast. Only explicit
   `crash() -> crash_recovery`, fresh `confirmRecovery()`, and subsequent verification
   may restore `verified`, with confirmation count incremented. Keep the existing durable
   `verified` and `broadcasting` cases unchanged.
2. **Consensus-compatible receive claims.** For both ZEC and XMR watch-only accounts,
   replace the exact synthetic consensus branch with a nonempty lookalike. The capability
   outcome must be `PROTOCOL_INCOMPATIBLE` with both `can_receive` and `can_spend` false.
   Exact fixture branches must retain watch-only receive true/spend false. This enforces
   the architecture rule that UI may claim receive behavior only when the branch matches.
3. **Untrusted error-code normalization.** Inject a synthetic secret-canary string as an
   adapter prepare failure code and separately as a signer failure code. State-machine
   failures and snapshots must normalize each to `UNAVAILABLE`, contain no canary, enter
   `failed`, release the shared account lock, and never broadcast. Existing explicit
   stable error codes such as `DEVICE_DISCONNECTED`, `WATCH_ONLY`, `INTENT_MISMATCH`, and
   `CAPABILITY_MISSING` must remain preserved by their existing tests.

Do not inspect implementation source from assertions. Do not edit production, fixture,
security tests, package/lock files, workflows, policy, evidence, tickets, handoffs,
documentation, or any other path. Do not weaken, remove, rename, or reorder coverage.

Use `apply_patch`. Read-only inspection and `wc -l`/`sha256sum` over the one authorized
path are allowed. Do not execute tests, Node, npm, builds, formatters, scanners, Git,
GitHub, network, wallet, daemon, subprocess, hardware, USB/HID, or device commands. Do
not use `/tmp`, root, `sudo`, deletion, cleanup, `rm`, globs, environment-variable
targets, or unresolved paths.

Stop after authoring. Report the changed path line count/SHA-256, exact added test names
and total count, how each case is non-vacuous, expected red against Production Correction
01, protected hashes, unchanged paths, and confirmation that nothing ran. Reviewer
XHigh must inspect the test source before Luna executes anything.
