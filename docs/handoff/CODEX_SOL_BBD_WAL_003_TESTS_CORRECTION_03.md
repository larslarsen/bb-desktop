# Codex Sol Handoff — BBD-WAL-003 Test Source Correction 03

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `3509888ebace14e334e514b7d18dbc2c05f829ec`

Read the ticket, current task, architecture §§4.3 and 5.3, all prior Sol handoffs, and all
six current test paths. Corrections 01–02 remain reviewer-rejected before execution.
Preserve their coverage. Modify only the same six test/fixture paths for exactly these
three remaining gaps:

1. **Positive correlation and cancellation.** Extend the protocol session tests with a
   valid parent request followed by a child response using the same ID, child sequence 1,
   and the bound session. Prove a wrong response ID, duplicate/late response, wrong
   direction, or second response fails closed. Add one valid cancel of a known in-flight
   nonterminal request plus the existing unknown-cancel rejection. Assert rejected frames
   do not advance sequence/correlation state. Do not rely only on invalid examples while
   claiming request/response correlation.
2. **Dual-coin receiver schema.** Add a valid XMR `receiver.fresh` dispatch with an XMR
   network, alongside the valid ZEC row, and assert both send exactly once. Add invalid
   XMR↔ZEC network crossovers. Update send-count baselines accordingly; all forbidden and
   invalid calls must still prove zero additional sends.
3. **Deep, not shallow, main-process clone.** In the Electron positive dispatch test,
   assert the nested `payment_request` received by the supervisor is not the renderer
   input's nested object. Mutate that received nested object and prove the renderer input
   remains byte-for-byte unchanged. Likewise assert the returned result's nested `value`
   and `payment_request` are not shared with the raw supervisor result; mutate them and
   prove the raw result is unchanged. Retain the existing top-level, accessor, size,
   subscription, and one-call-per-channel proofs.

Keep every inherited assertion, fixture hash, and other correction result. Do not add
production, package/workflow/policy implementation, evidence, or new paths.

Use `apply_patch`. Only read-only inspection and final `wc -l`/`sha256sum` over the six
paths are allowed. Do not execute Node, npm, tests, builds, formatters, scanners, Git,
GitHub, network, Electron, child processes, wallets, nodes, hardware, or devices. Do not
install anything or use root, `sudo`, `/tmp`, deletion, cleanup, `rm`, globs, or unresolved
destructive targets.

Stop after authoring and report changed tests, totals, six hashes/line counts,
non-vacuity, expected red, preserved counts, and confirmation that nothing ran. Reviewer
XHigh must accept the corrected source before Codex Luna executes it.
