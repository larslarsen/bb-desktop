# Hermes Handoff — BBD-WAL-008 Final Security Evidence Correction 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`docs/engineering/HERMES_JR_DEV_ROUTING.md`, `docs/handoff/CURRENT_TASK.md`,
`tickets/BBD-WAL-008.md`, Final Security Gate 01 evidence, and Final Security Gate 01
Evidence Review 01.

This is documentation correction only. Run no formatter, test, build, lint, audit,
scanner, policy, product, device, network, or actor command.

Preflight may verify only branch `master`, exact `HEAD == origin/master` at the
protected parent, clean index/worktree, and these frozen record identities:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `docs/testing/BBD-WAL-008-FINAL-SECURITY-GATE-01.md` | 109 | `552401b5b2e07b019b4a454e0c0062008da9114e9e37d5debfe949de1e315817` |
| `docs/handoff/CURRENT_TASK.md` | 526 | `bc4728b55afbe50e88958f9542bceac7a0f94499d5181c1bd7cfc4eabdc57d8e` |

Edit only those two files. In Final Security Gate 01 evidence:

1. add Hermes session `20260903_162048_093801`;
2. correct the protected governance parent to
   `c3997ab63e109d2e6536ffe4b411b6a28e03a8b1`;
3. correct upstream to `63279301`;
4. correct provider/model to `nous` / `meituan/longcat-2.0:free`; and
5. add a transcript-deviations section preserving the exact findings from Evidence
   Review 01: three failed shorthand-path reads and subsequent discovery operations;
   the unrequested `git show ... --stat | head -30`; the missing separate reads of the
   two role/routing policy documents; compound post-gate tool-identity and integration
   submissions; and the extra final `git log --oneline -1`.

State plainly that the deviations were read-only or authorized integration activity,
did not mutate gate inputs or source, and did not rerun any security gate. Preserve all
five accepted security results, literal gate commands, immutable hashes, and the
WAL-007/XMR boundary.

Update only the leading active block of `CURRENT_TASK.md` to await reviewer acceptance
of Final Security Evidence Correction 01 and link the corrected evidence plus Evidence
Review 01. Do not rewrite historical content.

Stage exactly the two corrected records. Commit exactly
`docs: correct WAL-008 final security evidence`, push `master`, then run only the
minimum Git final-state proof authorized by this handoff and stop. Do not amend or
recreate `404a438e`; do not edit source/test/review/ticket/other documentation; do not
rerun any gate; do not use Grok or invoke another actor. On mismatch, stop without
mutation, commit, push, or post-stop command.
