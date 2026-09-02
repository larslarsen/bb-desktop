# Codex Sol Handoff — BBD-WAL-007 Phase-C Slice 1 Distribution

Status: AUTHORIZED — PRODUCTION SLICE 1 ONLY

Source actor: Principal Dev — Codex Sol (`gpt-5.6-sol`, High)

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Protected source baseline: `6204255988423437cb3b3d18da88f636dc648cd7`

Phase-B acceptance:
`../testing/BBD-WAL-007-PHASE-B-ACCEPTANCE-01.md`

Ticket: `../../tickets/BBD-WAL-007.md`

## Objective

Implement only the Linux x86-64 pinned Monero installation distribution boundary and
native-only selection orchestration proven by `native_surface` and `xmr_distribution`.
This slice includes no wallet-RPC lifecycle, HTTP/JSON/Digest transport, node probe,
account custody, viewing state, receiver persistence, or real-gate execution.

## Authorized paths

Sol may edit only:

- `wallet-broker/src/lib.rs` — pre-edit 11 lines, SHA-256
  `ad61f92cce12115df9da6f263240ec0c57b5746e79cdcf6459f03731dab45617`;
- `wallet-broker/src/native.rs` — pre-edit 293 lines, SHA-256
  `50a078f05d8d66127fac0aae99343070758b0da549d5468ed2e0bd71ba0483e9`;
- `wallet-broker/src/native_ui.rs` — pre-edit 135 lines, SHA-256
  `1c1da0425460154ea84a0751bfb9f40d0cea58b15efbf13aaebd0c463bde60b8`;
- `wallet-broker/src/xmr.rs` — new;
- `wallet-broker/src/xmr/distribution.rs` — new;
- `wallet-broker/src/xmr/model.rs` — new;
- `wallet-broker/src/xmr/test_support.rs` — new, distribution/native fake support only.

Every other path and repository is read-only. In particular do not edit the manifest,
lockfile, tests, policy, evidence, governance, ZEC modules, vault/session/store, Electron,
protocol, supervisor, renderer, workflows, packages, or `bb-go`.

## Frozen module and native boundary

- `lib.rs` adds only `pub mod xmr;` to its existing module inventory.
- `xmr.rs` exposes only the three modules present in this slice: `distribution`, `model`,
  and `test_support`. Do not declare placeholder future modules.
- Add the exact `XmrInstallationSelectionPort` and `XmrSelectionController` API consumed
  by the accepted native test. Reject every nonnative origin before chooser, path,
  verifier, persistence, or process effect. Cancellation ends after the chooser.
- `NativeAction::from_method` remains fail-closed: no Electron, preload, protocol, HTTP,
  generic method string, or dynamic path gains selection authority.
- The native UI may add only the direct `monero-wallet-rpc` file-picker operation. It
  must reject non-UTF-8 without lossy conversion and must not validate, scan, download,
  launch, log, or expose the selected path.

## Frozen distribution behavior

Implement the exact release constants and behavior in the ticket and
`xmr_distribution.rs`:

- only explicit Linux x86-64 selection is available; normal and portable selections
  have identical validation and pin semantics;
- validate absolute nonempty UTF-8 with no NUL and at most 4,096 bytes before filesystem
  access;
- use final-component `lstat`; reject symlink and every nonregular type without follow;
- require exact 29,026,368-byte length and effective-user executability before hashing;
- compute bounded streaming SHA-256 and compare the exact inner hash in constant time;
- only after hash success, run the selected executable with the single `--version`
  argument under a cleared environment, bounded output, and exact version match;
- never search `PATH`, scan a parent, download, substitute, fall back, resolve a caller
  URL, or disclose the personal selected path through error/debug/log/diagnostic text;
- persist only schema version 1, the exact Linux pin ID, and selected path using a
  private `0600` temp file, file sync, atomic replacement, and parent-directory sync
  beneath a caller-supplied already-private broker data directory;
- reject corrupt/partial/unknown-field/version/mode/symlink selection records closed;
- retain observed file identity/size/modification metadata, re-run the complete verifier
  before launch, and report disappearance/change so a later process owner can stop the
  exact child without alternate selection.

Use safe Rust only and existing direct dependencies. Effective-user executable checking
must be real on Linux, fail closed, and not be approximated as “any execute bit.” It may
read bounded Linux process identity data through safe standard-library filesystem APIs;
it may not add libc/FFI/unsafe or invoke a helper program.

The common model in this slice may contain only the closed error/platform types needed
for distribution and later extension. Errors expose stable codes/messages without
paths, upstream output, or free text and do not give secret/path-bearing values `Debug`.

## Test-support constraint

`xmr/test_support.rs` implements only the recording ports and fault controls required by
`xmr_distribution.rs`. `DistributionRig` must drive the real production enrollment,
verification-order, persistence, launch-reverification, and poll/teardown decision code;
it may not duplicate those algorithms or pass through a test-local substitute. Fakes
record exact operations and inject only the named accepted faults.

Do not add permissive defaults, ignored branches, conditional success, placeholder
methods, raw/generic RPC, runtime network code, or future-slice scaffolding.

## Prohibited actions and delivery

Sol must not run tests, Cargo, formatters, builds, binaries, Node, npm, package managers,
security tools, network, Git, or GitHub. It must not stage, commit, push, or create
evidence/scratch state.

Stop after the seven-path source drop. Report exact paths, line counts, SHA-256 hashes,
the production-to-fake delegation shape, and confirmation that no prohibited path or
command was used. Do not begin Slice 2. Reviewer acceptance is required before Hermes
may execute the focused green gate.
