# Codex Sol Handoff — BBD-WAL-003 Production Correction 03

You are **Principal Dev — Codex Sol**, using `gpt-5.6-sol` at High. This is the complete
durable correction prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance baseline: `201d129f`

Read the production handoff, Corrections 01–02, green handoff, ticket, current task,
accepted protocol tests, and `wallet-broker/protocol.js`. Correction 02's implemented
guards are sound but its same-family audit omitted the exported `computeSessionId()`
boundary. That function still interpolates `parent_pid`, `child_pid`, `parent_nonce`, and
`child_nonce` directly and can therefore invoke hostile string coercion when called
outside the already-validated supervisor path.

Modify only `wallet-broker/protocol.js`. Before constructing the preimage,
`computeSessionId()` must fail `SCHEMA` unless its input is a plain/null-prototype data
object with no accessors and all four named inputs are primitive strings matching their
existing canonical PID or 32-lowercase-hex nonce regexes. The accepted fixture contains
additional transcript fields, so do not impose an exact-key schema on the outer fixture.
Do not invoke `toString`, getters, or any other coercion. Preserve the exact preimage and
hash for valid input, all Correction 02 guards, tests, and every other path byte for byte.

Use `apply_patch`. Only read-only inspection and final `wc -l`/`sha256sum` for the one
authorized path are allowed. Do not execute Node, npm, tests, builds, formatters,
scanners, Electron, Git, GitHub, network, child processes, wallets, nodes, hardware, or
devices. Do not install anything or use root, `sudo`, `/tmp`, deletion, cleanup, `rm`,
globs, or unresolved destructive targets.

Stop after authoring and report the exact validation, line count, SHA-256, preserved valid
preimage, and confirmation that nothing ran. Reviewer XHigh must accept the corrected
source before Codex Luna resumes the green handoff from its first targeted command.
