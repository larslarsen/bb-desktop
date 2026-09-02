# Development Roles and Routing Policy

This document records the agent roles used for BitBook desktop work. It is governance
only and changes no client code, package, native artifact, or acceptance state.

## Roles

- **Lead Engineer/Reviewer — Codex:** fixes architecture and task boundaries, selects the
  minimum-usage capable source actor, reviews integrated work, accepts or rejects it, and
  authorizes the next ticket. It may directly publish only a small reviewer-authored
  governance/review change with exact authorized paths.
- **Implementation Dev — Codex Spark:** uses GPT-5.3-Codex-Spark High for bounded
  boilerplate, fixture/table plumbing, schema scaffolding, and later UI wiring whose
  semantics are already fixed. It does not decide architecture, trust boundaries,
  cryptography, custody, concurrency, or persistence and does not execute tests,
  integrate, maintain records, or use Git.
- **Sr Dev — Grok Build:** uses Grok 4.6 High and is the default senior source actor for
  reviewer-bounded source and test-source work after the reviewer fixes sensitive
  semantics. This includes protocol, native-process, coin-adapter, corrective, custody,
  concurrency, persistence, and release-gate implementation. It does not execute tests,
  integrate, maintain records, or use Git.
- **Principal Dev — Codex Sol:** uses `gpt-5.6-sol` at High only as a fill-in escalation
  when the reviewer records that Grok is not strong enough for the bounded task or Grok
  has stopped without a usable drop. It does not execute tests, integrate, maintain
  records, or use Git.
- **Jr Dev — Hermes:** uses the locally installed Hermes Agent. It owns source-drop
  integration, test and acceptance-command execution,
  implementation/evidence records, and the corresponding Git, commit, and push work. It
  does not design or author tests. Each run records its actual Hermes version, provider, and
  model under `HERMES_JR_DEV_ROUTING.md`.
- **Owner:** makes product decisions and relays one-way prompts, reports, repository
  hashes, URLs, and source drops. The owner is not an engineering acceptance authority.

## Routing

1. The reviewer writes the bounded ticket and selects exactly one source actor.
2. Grok Build receives senior source/test-source work by default once security and
   protocol semantics are fixed by the reviewer.
3. Codex Sol is used only after a documented Grok insufficiency or stopped attempt.
4. Codex Spark may receive explicitly selected mechanical work whose design and
   semantics are already fixed.
5. Hermes integrates every developer drop, runs the ticket's commands, records evidence,
   and publishes the resulting Git change.
6. The reviewer alone accepts or rejects the result and authorizes what follows.

Selection starts with Grok for senior source edits and escalates to Sol only under the
recorded exception above. Reliability and total usage through an accepted result still
govern bounded Spark work and any escalation. Roles do not widen an active ticket's
paths or authority.

The current wallet routing and dependency order are recorded in
`docs/engineering/WALLET_ROADMAP_ROUTING.md`.
