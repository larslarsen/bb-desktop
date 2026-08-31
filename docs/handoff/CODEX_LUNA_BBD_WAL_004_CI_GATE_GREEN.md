# Codex Luna Handoff — BBD-WAL-004 CI Gate Green Integration

You are **Jr Dev — Codex Luna**. This durable file is the complete integration prompt;
ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read completely before acting: `AGENTS.md`, `TESTING.md`,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-004.md`, every BBD-WAL-004 CI-gate
review/evidence file, the complete accepted test and three production paths,
`.github/workflows/security.yml`, `package.json`, and the relevant policy runner.

## Preflight

Require `HEAD == origin/master` at the governance parent, clean index, and exactly these
four unstaged accepted paths:

- `.gitleaksignore` — 9 lines —
  `1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b`
- `scripts/security-policy.js` — 2,231 lines —
  `affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`
- `.github/workflows/sbom.yml` — 51 lines —
  `dae5c48985ee9d70ccb06c33483fd13fa1f5351e431d251f6b878d31818a933e`
- `test/securityPolicy.node.js` — 2,078 lines —
  `99caa3f428f808cb53260cffadd4e5e8de7c556a9996075c26e77ab4a6adde47`

Require the ticket vector under `expand =`, no same-hex `key    =` live label, and
`git diff --check` success. Stop on any extra path, index entry, line/hash mismatch,
or unintended diff.

Use only the ignored disk-backed scanner already present at
`target/security-tools/gitleaks-v8.30.1/gitleaks`. Verify it reports version 8.30.1.
Verify its archive is exactly 8,230,402 bytes with SHA-256
`551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb`.
Do not install, move, delete, or clean the tool.

## Corrected-fixture falsification

First run `node test/securityPolicy.node.js`. It must exit 0 with all 69 cases green.
Then use `apply_patch` to temporarily change only the wrong-path branch's exact throw
statement in `scripts/security-policy.js`:

```text
        throw new PolicyError('.gitleaksignore has a wrong path fingerprint');
```

to exactly:

```text
        return;
```

This is an intentional temporary fail-open mutation confined to the already validated
wrong-path branch. Run `node test/securityPolicy.node.js` once. It must exit 1 with
exactly 68 `ok`, one `not ok`, and final summary
`1 security policy test(s) failed`. The sole failure must be
`strict nine-line reviewed Gitleaks ratchet bytes and content are enforced` because its
wrong-path candidate is accepted and `assertRejects` reports that no rejection occurred.
Restore the exact throw immediately with inverse `apply_patch`, require
`scripts/security-policy.js` SHA-256
`affe15ea73706becb6156409afae80ce44076641915457a9e2b9399207ed558f`,
then rerun `node test/securityPolicy.node.js` and require all 69 cases green.

Any other failure, no failure, a fallback generic rejection, restore mismatch, extra
diff, or file change is a stop condition. The earlier block-removal attempt and exact
restoration are recorded in `docs/testing/BBD-WAL-004-CI-GATE-GREEN-RUN-02.md`; do not
repeat it. Do not commit the temporary mutation or use Git restore/reset/checkout.

## Remaining exact local green gate

Run separately and in this order, recording output and exit status:

```text
node scripts/security-policy.js
npm run build
npm test
npm audit --audit-level=low
target/security-tools/gitleaks-v8.30.1/gitleaks git --redact=100 --no-banner .
target/security-tools/gitleaks-v8.30.1/gitleaks dir --redact=100 --no-banner .
git diff --check
```

Every remaining command must exit 0. The direct policy file must report success. The
complete npm suite must retain Electron security 19, security policy 69, wallet contract
48, broker protocol 11, supervisor 11, and preload 6 with no failure. Npm audit must
report zero vulnerabilities. Both pinned Gitleaks modes must report no unsuppressed
finding. The full-history mode may acknowledge only the exact reviewed fingerprints;
the directory mode must find no live secret. No secret/match body may be printed or
recorded.

Do not run Rust/Cargo, SBOM generation locally, cargo-cyclonedx installation, package or
platform builds, Electron, wallets, nodes, hardware, devices, root, `sudo`, `/tmp`,
cleanup, deletion, or any unlisted command. The manual GitHub job installs the pinned
SBOM generator and is the real generated-document proof.

## Local evidence and integration

If and only if every local result is exact, create only
`docs/testing/BBD-WAL-004-CI-GATE-GREEN.md` with the governance parent, versions,
commands/statuses, exact green and falsification counts, restoration hash, scanner
results, accepted pre/post hashes, no-secret statement, and confirmation that no package
build or Rust/wallet path ran. Update only
`docs/handoff/CURRENT_TASK.md` to `CI GATE LOCAL GREEN INTEGRATED — MANUAL WORKFLOWS
RUNNING` and link the local evidence.

Run `git diff --check`. Stage only the four accepted source/test paths, local evidence,
and `CURRENT_TASK.md`; inspect staged names/diff. Commit once as:

```text
fix: close WAL-004 CI security gates
```

Push `master`. Require `HEAD == origin/master` and clean worktree before remote work.

## Manual non-packaging workflows

Dispatch exactly these two workflows at the pushed commit, recording their new run IDs:

```text
gh workflow run security.yml --repo larslarsen/bb-desktop --ref master
gh workflow run sbom.yml --repo larslarsen/bb-desktop --ref master
```

Wait in the foreground for both new runs to complete and report their exact commit,
job/step conclusions, run IDs, and URLs. Both must succeed. Do not rerun, cancel, edit,
download artifacts, dispatch packaging, or improvise if either fails. A failure is a stop
condition for reviewer diagnosis, not authorization to change source.

Report the integration commit, evidence line count/hash, all local counts/results, both
remote run results, accepted production hashes, and final status. Do not create a second
commit after the workflows; final acceptance records remain reviewer-owned.
