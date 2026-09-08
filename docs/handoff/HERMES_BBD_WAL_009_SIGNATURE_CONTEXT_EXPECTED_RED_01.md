# WAL-009 signature-context expected red 01

Actor: Hermes Jr Dev. Governance parent: the commit containing this handoff.
Run from the bb-desktop repository root. Grok's source authorizations are closed.
This supersedes all previous execution permissions: one targeted command only.

Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md,
CURRENT_TASK.md's leading active section, this handoff, and
docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-TEST-CORRECTION-01-REVIEW.md only.
Do not reload historical evidence, Rust implementations, or dependency sources.
Do not perform filesystem discovery or search unrelated sessions.

## Preflight and frozen identities

Record the actual version from one `hermes --version` call. Record this actor's
session ID, actual provider/model, governance parent, and observed HEAD. Historical
routing-adoption values are not runtime identity. The launch supplies the current
Hermes session ID; query only that ID if session metadata is needed. Never dump
configuration, environment, credentials, prompts, or other sessions.

Permitted read-only Git preflight: `git rev-parse HEAD`,
`git status --short --untracked-files=all`, and `git diff --cached --name-only`.
The index must be empty. The sixteen frozen paths below are the exact expected
dirty/untracked file set before this run. The new evidence path must be absent.
Do not run Git log or broaden repository inspection. The reviewer may add only
the launch checkpoint after the governance commit; that does not change these
frozen source identities.

Measure SHA-256 and line counts from all sixteen actual files before the command.
Preserve the measurements as parsed tool data, preferably one JSON object emitted
by a bounded Python/hashlib read of this exact path list. Do not reconstruct the
measurement table by manually retyping the expected values from this handoff.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| wallet-broker/src/zec/test_support.rs | 4364 | 6822e458cc8475155e6c2d3bbdef592e1af2a339100f29b3e4f7359a027886ce |
| wallet-broker/src/native_ui.rs | 201 | 600e2fb34134a276d31da9c5e746cc6478337bae73590fd742cf2d36ff785524 |
| wallet-broker/src/native_ui/zec_review_tests.rs | 233 | 2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf |
| wallet-broker/src/native.rs | 489 | 992138f18cbb0b969296109de3186aab923500c41ed2c0d41cf9c0696d3c3acc |
| wallet-broker/Cargo.toml | 122 | 73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503 |
| wallet-broker/Cargo.lock | 5395 | b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71 |
| package.json | 42 | 84b30b6860441a100588b5bdb92b37ddc45fb7610d48ee6f0e51c30ea0717780 |
| package-lock.json | 396 | 5e1122f32b0db42eb4386d4d0f0c47a4160d23cb4ae790bbb3600b5d3a5bddfc |
| wallet-broker/src/zec.rs | 274 | 045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b |
| wallet-broker/src/zec/prepare.rs | 1238 | 44c0783fa3d75867599092d25912032b665838ed7a73c81a16b0eec2b26ffb07 |
| wallet-broker/src/zec/store.rs | 2872 | 531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90 |
| wallet-broker/src/zec/spend.rs | 883 | c646ea835eceac69f899dea111b19ae7fb3354ecc682b73b08b598138b0d5b0d |
| docs/testing/BBD-WAL-009-LOCK-SYNC-01.md | 120 | ab25ffd2ac6ec7c26f7c21e42978697ec10da7ff395a9569afd9e78d1cd27ed2 |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-EXPECTED-RED-01.md | 199 | 42825e83ba6e98471c18b0c63d246e9a8790c25591e395cb6a0934c8d7bee6aa |
| wallet-broker/src/zec/spend/verification_context_tests.rs | 528 | 78e7fe50c3bd213f8d0067957bf1771bd42137d46b9e0e8e7adbcb50d8330dec |
| docs/testing/BBD-WAL-009-NATIVE-REVIEW-PREREQUISITE-02.md | 181 | 33d488dfcc88eb684d316bcffc547c051be412984d5633c54da76c1cef19dbe2 |

Verify the effective build target remains the existing wallet-broker/target on
repository ext4. Filesystem-type inspection of the repository, that target, and
/tmp is allowed; /tmp is tmpfs and must not hold substantial build artifacts.
Do not alter environment/configuration or install tools. An unexpected file,
hash, line count, staged index, or target state stops before Cargo; record the
preflight stop in the one evidence path below.

## Exactly one execution command

Submit this string as the entire terminal command exactly once:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --lib zec::spend::verification_context_tests
```

Use the terminal tool's returned output and exit_code. No shell operators,
pipeline, redirection, environment prefix, wrapper, echo, or version probe may
be added. Do not rerun for clearer output or a status code. Waiting on the SAME
running command is allowed; it cannot launch a second command. Never run two
acceptance tools concurrently. If capture fails, report the limitation and stop.

Four tests are declared in the accepted module. Do not invent a different count
or list. Actual executed count must come from this result, not the declared count.

Classify the actual complete result:

- Exit 101, missing private ShieldedVerificationContext contract only, and zero
  tests executed: intended absent-interface red, pending reviewer acceptance.
- Existing spend.rs authorizing-context/Option/lifetime errors or test_support.rs
  fault-type error, including alongside the missing context: MIXED /
  PREREQUISITE-BLOCKED. The existing unused-mut warning must also be recorded.
  Such a compile stop is not behavioral red and runs zero tests.
- Any additional unexpected source/API diagnostic: unexpected compile blocker;
  retain the exact diagnostic for reviewer triage. Do not hide it among known
  prerequisites or repair it.
- Dependency/environment failure: record that specific stop without dependency,
  network, configuration, toolchain, or lockfile repair.
- Exit 0 or any executed tests: unexpected outcome; report accurately and stop.

Every outcome stops gate execution. No formatter, cargo check, other test, lint,
audit, scanner, falsification, native launch, source edit, integration, or other
actor is authorized. No mainnet, broadcast, device, Monero, or network action.

## Mechanical evidence and final stop

After the result, only these actions remain allowed: the same sixteen-path
read-only measurements, read-only Git status, bounded retrieval of already
captured results, and creation/verification of this one file:

docs/testing/BBD-WAL-009-SIGNATURE-CONTEXT-EXPECTED-RED-01.md.

Render that record programmatically from actual saved tool data. Include:

- actual runtime/session identity, governance parent, observed HEAD, preflight
  state, and build-target filesystem;
- the exact submitted command and actual tool exit_code;
- the COMPLETE returned compiler output in a text code fence, preserving all
  diagnostic text, caret spans, notes, suggestions, warning, and final summary;
- four declared tests versus the actual executed count and result classification;
- all sixteen FULL before/after hashes and line counts, generated from parsed
  measurements; and any actual deviation, without blanket compliance claims.

For compiler output only, replace local repository and Cargo-registry prefixes
with explicit tokens such as <repo>/ and <cargo-registry>/, and state that path
normalization. Preserve every other byte of diagnostic text, including whitespace.
The exact command is recorded separately without normalization. Never manually
retype, summarize, fix, or reconstruct diagnostics or hash measurements.

If tool data must be recovered from Hermes state.db, open that database in SQLite
URI read-only mode. Query only this supplied current session ID. Select only the
session model/billing_provider and the needed tool-call arguments/result content;
resolve the relevant message IDs within this session, then retrieve those exact
IDs. Do not print other tool bodies, prompts, credentials, or unrelated sessions.
Parse the stored JSON and feed its output strings directly into the renderer.
Do not paste a manually supplied copy into the rendering code. If retrieval is
incomplete, report that limitation; do not regenerate execution.

Verify the resulting evidence hash/line count and the rendered data against the
captured originals. Leave the file uncommitted. Do not modify CURRENT_TASK.md,
prior evidence, or any other record; do not stage, commit, push, repair, launch,
or run another gate. Report result classification, declared/executed counts,
runtime/session ID, evidence hash/lines, and stop. The reviewer collects only
after the owner reports done; no polling.
