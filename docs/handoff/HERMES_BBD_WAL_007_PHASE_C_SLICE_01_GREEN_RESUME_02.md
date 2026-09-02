# Hermes Handoff — BBD-WAL-007 Phase-C Slice 1 Green Resume 02

You are **Jr Dev — Hermes**. This durable handoff is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `tickets/BBD-WAL-007.md`, the complete
provisional Slice-1 Green 01 evidence, Slice-1 Node Stop Review 01, Slice-1 Native Policy
Source Review 01, both earlier green handoffs, and `docs/handoff/CURRENT_TASK.md`.

## Sole task and precedence

Resume only the two Node commands that were not accepted in Green Resume 01, then finish
the existing evidence and integration on exact success. The formatter check, 17 native
tests, 12 distribution tests, and native-feature compile check are accepted and must not
be rerun. This handoff replaces the two policy identities and evidence corrections from
earlier handoffs; every other role, stop, evidence, Git, redaction, and closed-scope rule
remains mandatory.

You may not edit source, policy, tests, manifests, locks, or governance; run Cargo,
rustfmt, builds, npm, package managers, security tools, network operations, product or
Monero binaries; begin Slice 2; or run the real local-Monero gate.

## Protected preconditions

Require:

- `HEAD == origin/master ==` the protected governance parent;
- a clean index and `git diff --check` clean;
- exactly the nine accepted source/policy paths plus the provisional evidence in the
  worktree; and
- the complete nine-path identities frozen in Native Policy Source Review 01.

In particular, require:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/xmr/distribution.rs` | 914 | `4135db83eca151185d8222498d2435ef56e2d564ff17bd3fa88481260758aed5` |
| `wallet-broker/src/xmr/test_support.rs` | 368 | `7819a2d7f20b49540346a16a53d94ea77e7acd21ceb17e48091e746eadc293b7` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |
| `test/securityPolicy.node.js` | 3,189 | `dd22ab83645d5d5ffb9d695cd21f15175ffe109b6020142e1bff5ca8d60e0497` |
| `docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md` | 87 | `7f13fccc3ac62024bb7a3fb97c42c0974a902189db3c0dbbe9ba56b729334416` |

Re-prove the four frozen Phase-A identities from Green 01. Record the actual resolved
identity again, with provider and model as separate fields from separate commands:

```text
hermes --version
hermes config get model.provider
hermes config get model.default
```

## Exact Node-only resume

Run once each, from the repository root, in this order:

```text
node test/securityPolicy.node.js
node scripts/security-policy.js
```

Require:

1. exit 0, exactly 86 `ok`, no `not ok`, and final line
   `BitBook security policy tests passed (86).`;
2. exit 0 and final line `BitBook desktop security policy checks passed.`

Stop on the first mismatch or failure. Do not repair anything, change a command, create
a second evidence file, stage, commit, or push after a stop.

## Evidence correction and exact-success integration

Only on exact success, update the existing
`docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md` while preserving both prior stop events
and the successful formatter/Rust results. Make these evidence corrections:

- record the post-format line counts as 914 for `distribution.rs` and 368 for
  `test_support.rs`; the existing post-format hashes are already correct;
- state that the Node collision was the legacy generic `monero` token rule applied to
  the exact picker title, not the `eframe` or `rfd` imports;
- record the accepted policy/test identities above and both exact Node results; and
- record version, provider, and model separately from their queried values.

Change the evidence heading/state to complete. Preserve the exact frozen manifest,
lockfile, and test hashes, command chronology, filesystem fact, no-mutation proof,
test-first falsification reference, redactions, and prohibited-action confirmation.

Update `docs/handoff/CURRENT_TASK.md` to
`PHASE C SLICE 1 GREEN COMPLETE — REVIEW REQUIRED`, link the completed evidence, and
retain the ticket, architecture decision, routing, and prior-ticket records.

Recheck final identities, `git diff --check`, exact unstaged/staged scope, and the staged
diff. Stage explicitly only:

- the nine accepted source/policy paths;
- `docs/testing/BBD-WAL-007-SLICE-01-GREEN-01.md`; and
- `docs/handoff/CURRENT_TASK.md`.

Commit exactly:

```text
feat: add BBD-WAL-007 Monero distribution boundary
```

Push `master`, then prove `HEAD == origin/master`, clean index, and clean tracked and
untracked worktree. Stop for reviewer acceptance. Every other path, task, command, and
repository remains unauthorized.
