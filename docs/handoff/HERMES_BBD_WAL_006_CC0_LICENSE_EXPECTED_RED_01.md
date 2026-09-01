# Hermes Handoff — BBD-WAL-006 CC0 License Expected Red 01

Use only free Hermes/Nous. Run the one expected-red policy command; make no file or Git mutation.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the CC0 test-source handoff, this
handoff, both policy files, `deny.toml`, and `CURRENT_TASK.md`.

Require `HEAD == origin/master`, exactly one dirty path (`test/securityPolicy.node.js`), clean
`git diff --check`, and:

- `test/securityPolicy.node.js`: 2,527 lines,
  SHA-256 `d42f58a2487c64ad59d282d93759a6b156239e0a125d399f7821f278a9087faa`;
- `scripts/security-policy.js`: 2,482 lines,
  SHA-256 `d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5`;
- `deny.toml`: SHA-256 `3bd1161fbd31552f0c7887ac02d477be71e3c2542d15c1896bd5d037c562d5a8`.

Record Hermes version/provider/model. Run exactly once, unfiltered:

```text
node test/securityPolicy.node.js
```

Require exit 1, exactly 74 `ok`, exactly one `not ok` named
`WAL-004 cargo-deny policy is exact fail-closed and has no bypass lists`, an expected/actual
license-list mismatch containing `CC0-1.0`, and final line `1 security policy test(s) failed`.

Stop without a second command, edit, evidence, stage, commit, push, cleanup, deletion, network, or
device access. Any different result stops.
