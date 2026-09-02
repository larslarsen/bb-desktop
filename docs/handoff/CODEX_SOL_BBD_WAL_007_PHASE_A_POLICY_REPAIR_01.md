# Codex Sol Handoff — BBD-WAL-007 Phase-A Policy Repair 01

Status: AUTHORIZED — TWO-FILE POLICY CORRECTION ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected governance parent: the commit containing this handoff

Hermes stop evidence is uncommitted at
`../testing/BBD-WAL-007-EXPECTED-RED-02.md`, 88 lines, SHA-256
`b319588f01e91c17d0574b28d7d7e737f75b8b350d56e588411f4f5356d3bab7`.

## Verified gate state

Hermes proved Rust formatting passes and all seven Rust targets fail for their exact
expected-red missing-production boundary. Only the Node policy gate fails.

Two Phase-A policy defects remain:

1. the committed repository checker freezes the pre-WAL-007 manifest; and
2. the test's case-insensitive `\bPATH\b` pattern treats every ordinary Rust identifier
   named `path` as executable path-search authority.

Correct only those defects. Do not change wallet semantics, test inventory/counts,
Phase-C inventory, other forbidden-authority categories, or execution expectations.

## Authorized paths and exact semantics

Edit only:

- `scripts/security-policy.js` — pre-repair 2,667 lines, SHA-256
  `f66f6df408d434082b14b8e8a5e1bb61722a7f5bc09c97c7a5e224793b301e7e`;
- `test/securityPolicy.node.js` — pre-repair 3,067 lines, SHA-256
  `ca7cc722c058870362bebb8c706e29a28a918711e0feed9680053b4f24e23d9d`.

In `checkWalletBrokerManifest` in `scripts/security-policy.js`:

- add exact required feature line `xmr-local-gate = []` after the native UI feature;
- add exact dependency line
  `md-5 = { version = "=0.11.0-pre.4", default-features = false, features = ["zeroize"] }`
  after `hkdf`; and
- append the seven exact manifest test pairs, in manifest order, to `expectedTests`:
  `xmr_distribution`, `xmr_process`, `xmr_rpc`, `xmr_account`, `xmr_receiver`,
  `xmr_hygiene`, and `xmr_local_gate`, each at `tests/<name>.rs`.

Do not alter the WAL-004 exported dependency component set or any SBOM, ZEC, rate,
license, workflow, package, runtime-source, or repository-check behavior.

In `test/securityPolicy.node.js`, replace only the `path search` forbidden-authority
pattern. It must reject literal `which`/`whereis` commands, Rust `var`/`var_os` access to
the literal `PATH` environment name, and Rust `env!`/`option_env!` access to that literal.
It must not reject an ordinary identifier or type containing or equal to `path`/`Path`.
Keep the existing `std::env::var("PATH")` negative fixture and all other WAL-007
assertions byte-for-byte unchanged.

Every other path and repository is read-only.

## Prohibited actions and delivery

Do not run Node, npm, tests, formatters, builds, Cargo, binaries, package managers,
network, Git, or GitHub. Do not stage, commit, or push.

Stop after the exact two-file source drop. Report both line counts and SHA-256 hashes,
the unchanged 86 Node test count, the exact new regex, and confirmation that no other
path, behavior, or command changed. Reviewer acceptance is required before Hermes
resumes.
