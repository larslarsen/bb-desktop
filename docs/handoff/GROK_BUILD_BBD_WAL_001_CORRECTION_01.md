# Grok Build Handoff — BBD-WAL-001 Correction 01

You are **Sr Dev — Grok Build**, using Grok 4.6 High. This is the complete durable
correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/DEVELOPMENT_ROLES.md`, `tickets/BBD-WAL-001.md`,
`docs/handoff/CURRENT_TASK.md`, `docs/handoff/GROK_BUILD_BBD_WAL_001.md`, and the current
`docs/architecture/BBD-WAL-001-REVIEW.md`.

The current architecture source is 1,245 lines with SHA-256
`d9c0107ee4b7381cf5c70240eeae1d041791d0eaa2f926f9c2abf79e63c3a816`. Codex rejected it
for the seven blockers recorded in the ticket. Correct the document in place. Preserve
sound decisions and the complete required coverage; do not merely append errata.

You may edit only:

- `docs/architecture/BBD-WAL-001-REVIEW.md`

## Verified protocol facts supplied by the reviewer

Use these facts without network access and add a dated authoritative-source snapshot to
the review:

- Zcash NU6.3/Ironwood activated on mainnet on 2026-07-28 at block 3,428,143. The old
  Orchard pool is restricted and funds exit through the turnstile into Ironwood:
  <https://z.cash/upgrade/nu6-3/>.
- NU6.3 adds transaction version 6 and a separate Ironwood bundle/pool. Ironwood reuses
  the Orchard protocol; it is not a new address/receiver type:
  <https://zips.z.cash/zip-0229> and <https://zips.z.cash/zip-0258>.
- The Orchard-protocol receiver and incoming viewing key are scoped to the protocol, not
  a pool, and scan both Orchard and Ironwood under the NU6.3 wallet rules:
  <https://zips.z.cash/zip-0326>.
- Orchard-to-Ironwood migration requires privacy-aware wallet behavior and user consent
  for residual cross-pool amount disclosure: <https://zips.z.cash/zip-0318>.
- Current librustzcash wallet APIs construct v6 PCZTs and Ironwood bundles for
  post-NU6.3 payments, but exact release pins remain a later implementation decision:
  <https://github.com/zcash/librustzcash/blob/main/zcash_client_backend/CHANGELOG.md>.
- Official Monero documentation defines `monero-wallet-rpc --restricted-rpc` as
  restricted to view-only commands. A spending adapter therefore needs authenticated
  full wallet RPC on loopback, contained behind the broker:
  <https://docs.getmonero.org/interacting/monero-wallet-rpc-reference/>.

## Required corrections

1. Replace every future/NU7/approximate Ironwood statement with the verified NU6.3
   activation facts. Replace invented Ironwood receiver wording with accurate
   protocol-receiver versus pool semantics. Capabilities must separately express v6,
   Orchard, Ironwood receive/spend, PCZT version, and Orchard-to-Ironwood migration.
   Restored legacy funds must fail closed or use a later consented migration ticket.
2. Distinguish local `monerod` RPC from wallet RPC. The broker may use restricted node
   RPC where sufficient. It must use authenticated **full** wallet RPC for spending,
   loopback only, random credentials, no renderer visibility, and no public fallback.
   State the residual same-user/local-malware limit honestly.
3. Choose one normative, cross-language canonical representation for signed
   `PaymentRequestV1`, status events, and `intent_hash`. Specify domain separation,
   exact validation, unknown-field behavior, timestamp normalization, Unicode policy,
   immutable initial status, cancellation/status signatures, and independent Go/Rust
   golden vectors. Remove “CBOR/JSON” ambiguity and wrong section references.
4. Redesign the intent flow so an exact prepared review image—including fee and fee
   bound—exists before user confirmation. Define separate prepared, awaiting-confirm,
   signing, signed-unverified, verified, broadcasting, cancelled, expired, failed, and
   crash-recovery behavior. One explicit user confirmation may authorize the normal
   sign→verify→broadcast sequence, but cancellation/expiry must be checked again after
   signing and immediately before broadcast. A signed artifact recovered after crash
   requires fresh broker-owned confirmation and revalidation; never auto-broadcast.
5. Move all spend-authorizing actions and secret entry out of Electron. The broker owns
   a minimal native confirmation/onboarding/unlock/backup surface, or an equivalently
   broker-controlled OS credential agent. Hardware confirmation occurs on the device.
   Electron may supervise, submit a signed social request for review, show sanitized
   previews/status, and cancel. It cannot deliver confirm, unlock credentials, backup
   bytes, raw signer material, or broadcast. Explain that a compromised OS/broker or a
   hardware device that cannot display relevant fields remains outside/inside the
   residual trust boundary as appropriate.
6. Use inherited anonymous bidirectional child pipes/handles for v1 broker IPC; no UDS,
   named-pipe listener, TCP, or HTTP listener. Verify the packaged broker binary before
   spawn, bind the session transcript to both process identities/nonces, and never place
   a secret in argv or environment. Protocol stdout/pipe and redacted diagnostics must
   be separate. State that same-user OS compromise is not solved by filesystem modes.
7. Reconcile data flow, broker method allowlists, UX text, threat table, tests, ticket
   order, and decision register with these changes. Explicitly distinguish forbidden
   product/generic wallet HTTP from the contained authenticated loopback XMR wallet RPC.
   Explain why the Rust sidecar is selected despite adding a toolchain, and reject or
   bound the Go-sidecar and zallet-process alternatives.

The first implementation slice remains offline, deterministic, credential-free, and
incapable of constructing or moving real funds. It must add canonical golden vectors and
the corrected state/capability model, but no Electron UI, Rust installation, coin
dependency, network, device, wallet, node, or real key.

Do not execute tests, builds, installs, formatters, scanners, Git, GitHub, wallet, node,
network, hardware, or package commands. Do not touch a real device or secret. Do not use
`/tmp`, root, deletion, cleanup, `rm`, globs, variables as targets, or unresolved paths.
Do not edit any other file.

In your terminal response report only the authored path, line count, SHA-256, principal
corrections, unresolved owner choices, and confirmation that no out-of-scope action ran.
Codex reviews the corrected source; Codex Luna remains stopped until exact acceptance.
