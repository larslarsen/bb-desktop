# Grok Build Handoff — BBD-RATE-001 Fixture Correction 01

State: ACTIVE

Actor: Sr Dev — Grok Build (`Grok 4.6`, High)

Governance baseline: the commit containing this handoff

Read `AGENTS.md` and
`docs/testing/BBD-RATE-001-PRODUCTION-GATE-STOP-REVIEW-01.md` completely before editing.

## Authorized path

Edit only:

- `test/fixtures/rates/provider-bodies-v1.json`

Every production path, test source, other fixture, evidence, handoff, ticket, current-task,
package/lockfile, workflow, policy, dependency, wallet, renderer, Rust, daemon, Git, and
unlisted path is frozen. Preserve the uncommitted Hermes stop evidence exactly. Run no
command, test, formatter, parser, script, network/provider access, package manager, or Git.

## Exact correction

Change exactly these three JSON string values and nothing else:

1. Set `bodies.depth_eight_ok.raw_json` to:

```text
{"price":"42.1","time":"2026-09-01T12:00:00Z","x":{"a":{"b":{"c":{"d":{"e":{"f":1}}}}}}}
```

2. Set `bodies.depth_nine_fail.raw_json` to:

```text
{"price":"42.1","time":"2026-09-01T12:00:00Z","x":{"a":{"b":{"c":{"d":{"e":{"f":{"g":1}}}}}}}}
```

3. Set `canaries.amount_atomic` to the valid, unique u64 decimal string:

```text
18446744073709551614
```

Preserve all formatting, ordering, remaining fixture values, and the public fixture name.
Stop and report the final fixture SHA-256/line count, the three exact before/after values,
and that no command/test/network/Git or other edit occurred. Do not begin integration,
edit Hermes evidence, or authorize a resumed gate.
