# BBD-WAL-008 Final Security Gate 01 Evidence Review 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Integrated evidence commit: `404a438e0f7e461ec064eb20005fc03b40aaf40c`

Hermes session: `20260903_162048_093801`

Result: **FINAL SECURITY RESULTS VALID — EVIDENCE CORRECTION REQUIRED**

The five final security results are technically valid. The stopped parent proved npm
audit and cargo-audit; Resume 01 proved cargo-deny advisories, bans, licenses, and
sources plus both Gitleaks scans. The three Resume-01 gate commands were submitted
exactly once, in order, and exited 0. The four immutable inputs remained exact, and
commit `404a438e` contains only the specified evidence and current-task records.

The integrated evidence has three material metadata errors:

1. its protected governance parent is `6503959d...`; the Resume-01 protected parent
   actually proved by preflight was
   `c3997ab63e109d2e6536ffe4b411b6a28e03a8b1`;
2. Hermes upstream is mistyped as `63239301`; `hermes version` reported `63279301`;
3. provider/model is falsely recorded as `openai/gpt-5.6-sol (Nous)`; the session
   database records provider `nous` and model `meituan/longcat-2.0:free`.

Transcript audit also found read-only/process deviations that do not invalidate the
security results but must be disclosed. Hermes attempted three nonexistent shorthand
evidence paths before locating the exact files, used repeated searches and directory
listings to find handoffs, ran the unrequested read-only
`git show 6503959d08332802f90f8832b5af2652035f46ed --stat | head -30`, and added
`git log --oneline -1` to the final Git proof. Its post-gate tool-identity capture and
integration were compound shell submissions rather than individually submitted
commands. The session did not issue separate reads for the two named role/routing
policy documents; `AGENTS.md` was already injected as project context. None of these
operations changed source or gate inputs, and none reran a security gate.

Hermes alone may correct
`docs/testing/BBD-WAL-008-FINAL-SECURITY-GATE-01.md` and
`docs/handoff/CURRENT_TASK.md` under the separate documentation-only handoff. No gate
may be rerun. No source, test, policy, manifest, lockfile, workflow, deny/ignore rule,
ticket, review, or other path may change.
