# BBD-WAL-004 CI Gate Red Run 01

Reviewer: Lead Engineer/Reviewer — Codex XHigh

Governance parent: `c93e235deee41ba7bed744e4d1b7bd032e850a65`

Result: **RUNNER COMMAND REJECTED — TEST SOURCE UNCHANGED**

Luna's preflight passed. `HEAD == origin/master` at the governance parent; the sole
unstaged path was the accepted 2,063-line `test/securityPolicy.node.js` with SHA-256
`6b48023598984d91499466869533cf5c4b2d3b6a697cac567753f225dc044493`;
the index was clean and `git diff --check` passed.

The authorized `node --test --test-name-pattern=...` command exited 1, but Node's native
test runner treated this repository's custom test file as one subprocess. It therefore
reported one failed file-level subtest and did not expose the file's 69 registered cases
or their intended failures. This is an invalid runner command, not regression evidence
and not a test-source defect.

Luna stopped without creating evidence, staging, committing, pushing, or changing any
path. The accepted test source remains byte-identical. The corrected command is the
repository's established custom runner invocation:

```text
node test/securityPolicy.node.js
```

That full offline suite must run all 69 cases. Against the current production state,
exactly three cases must be `not ok`: the general exported-constant comparison and the
strict ratchet test for the Gitleaks correction, plus the manual Rust SBOM test for the
missing `--all-features`. The other 66 cases must be `ok`. The corrected execution is
authorized only by the amended Luna handoff.
