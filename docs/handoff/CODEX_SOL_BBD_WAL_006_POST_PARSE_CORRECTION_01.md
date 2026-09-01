# Codex Sol Handoff — BBD-WAL-006 Post-Parse Correction 01

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. Correct only the
post-parse Ironwood action classification established by the completed fixed diagnostic.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`,
`docs/handoff/HERMES_BBD_WAL_006_POST_PARSE_DIAGNOSTIC_01.md`, and the current
`wallet-broker/src/zec/store.rs` prepare implementation.

## Exact scope

Edit only `wallet-broker/src/zec/store.rs`, starting at 2,126 lines and SHA-256
`c27bc3424e25f5a18d4b31cc0fbd5d510395cdbf8929ec151e645f711cb20134`.

## Established diagnostic truth

The accepted fixed diagnostic produced:

```text
BBD-WAL-006-DIAGNOSTIC:post-parse payment=1 memo_plain=1 v6=1 branch=1 ironwood_actions=2 witnessed_inputs=2 positive_outputs=2 transparent_empty=1 sapling_empty=1 orchard_actions=0 orchard_witness=0 orchard_output=0 real_signature=1 ironwood_proof=0 orchard_proof=0
```

Pinned upstream behavior establishes that IO Finalizer pre-signs protocol-padding dummy
spends and clears their dummy keys; requested real spends remain without a spend authorization
signature until a signer acts. Both finalized actions may retain witnesses, so witness presence
is not a real-input discriminator.

## Mandatory correction

1. Remove the complete temporary `BBD-WAL-006-DIAGNOSTIC:post-parse` marker and no other
   production behavior merely for diagnostics.
2. Classify the two Ironwood actions using `spend_auth_sig`:
   - exactly one authorization-required real input has no spend authorization signature;
   - exactly one finalized protocol-padding input has a spend authorization signature.
3. Require the exact two-action shape as part of fail-closed post-parse validation, alongside
   the existing one-real-input and two-positive-output requirements.
4. Report `inspection.ironwood_inputs` as the authorization-required real-input count.
5. Report `inspection.has_signatures` as false for this unsigned PCZT. The signature placed by
   IO Finalizer on the verified padding action is protocol padding, not a real-spend signature.
   A shape other than one unsigned real action plus one signed padding action must fail closed.
6. Preserve the accepted rollback-only official construction, exact-byte database immutability,
   v6/branch/payment/memo/fee/bundle/proof checks, public error mapping, and all frozen tests.

Use a concise source comment only where needed to prevent future confusion about the upstream
padding signature. Do not hard-code serialized bytes or weaken an assertion.

## Delivery boundary

Use `apply_patch` only. Read-only local inspection is permitted. Do not run a formatter, Cargo,
Rust, Clippy, tests, Node, policy, Git, network, cleanup, deletion, or device command. Do not edit
tests, manifests, lockfiles, docs, policy, or any other source path. Do not stage, commit, or push.

Return the exact changed path with line count/SHA-256, summarize the classification and
fail-closed invariant, and disclose any ambiguity. Hermes remains the execution and Git actor.
