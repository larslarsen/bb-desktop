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
- **Principal Dev — Codex Sol:** uses `gpt-5.6-sol` at High for the highest-risk
  trust-boundary, custody, cryptography, concurrency, persistence, native-process,
  coin-adapter, and release-gate source and test-source work. It does not execute tests,
  integrate, maintain records, or use Git.
- **Sr Dev — Grok Build:** uses Grok 4.6 High for bounded protocol, rate-worker,
  corrective, and other senior source and test-source work after the reviewer fixes the
  sensitive semantics. It does not execute tests, integrate, maintain records, or use
  Git.
- **Jr Dev — Codex Luna:** uses `gpt-5.6-luna`. It owns source-drop integration, test and
  acceptance-command execution,
  implementation/evidence records, and the corresponding Git, commit, and push work. It
  does not design or author tests.
- **Owner:** makes product decisions and relays one-way prompts, reports, repository
  hashes, URLs, and source drops. The owner is not an engineering acceptance authority.

## Routing

1. The reviewer writes the bounded ticket and selects exactly one source actor.
2. Codex Sol receives the highest-risk trust-boundary and money-safety work.
3. Grok Build receives bounded senior work after security and protocol semantics are
   fixed.
4. Codex Spark receives mechanical work whose design and semantics are already fixed.
5. Codex Luna integrates every developer drop, runs the ticket's commands, records evidence,
   and publishes the resulting Git change.
6. The reviewer alone accepts or rejects the result and authorizes what follows.

Selection is based on engineering risk, reliability, and total usage through an accepted
result—not nominal per-token price. Roles do not widen an active ticket's paths or
authority.

The current wallet routing and dependency order are recorded in
`docs/engineering/WALLET_ROADMAP_ROUTING.md`.
