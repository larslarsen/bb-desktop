# Hermes Handoff — BBD-WAL-006 Final Policy Focused Gate 01

Use only free Hermes/Nous. Validate the accepted one-path policy implementation with one focused
Node command. Make no file, evidence, or Git mutation.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, `tickets/BBD-WAL-006.md`, the final-policy source
handoff, this handoff, `scripts/security-policy.js`, `test/securityPolicy.node.js`, and
`CURRENT_TASK.md`.

Require `HEAD == origin/master`, exactly one dirty path (`scripts/security-policy.js`), clean
`git diff --check`, and:

- `scripts/security-policy.js`: 2,482 lines,
  SHA-256 `d9b77bf5608e1aa79d565e5c3c9574c1e203286c584c30498dd173796434d7f5`;
- `test/securityPolicy.node.js`: 2,525 lines,
  SHA-256 `2c1bd92c778a975b1218dfd2445b6b1b605bb047032fd5289573cbbb6d0a0169`.

Record Hermes version/provider/model. Run exactly once, unfiltered, from the repository root:

```text
node test/securityPolicy.node.js
```

Require exit 0, exactly 75 `ok`, zero `not ok`, and final line
`BitBook desktop security policy tests passed (75).` Report exact result. Stop without a second
command, edit, evidence, stage, commit, push, cleanup, deletion, network, or device access. Any
other result stops.
