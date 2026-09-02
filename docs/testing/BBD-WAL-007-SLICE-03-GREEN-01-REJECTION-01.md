# BBD-WAL-007 Slice-3 Green Evidence 01 Rejection 01

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Result: **REJECTED — WARNING-FREE EXACT GREEN NOT PROVED**

Integrated implementation commit:
`6eb566d629a38737591122d3f4acf0543b417af6`
(`feat: add BBD-WAL-007 Monero RPC transport`)

The commit has the exact seven authorized paths, `HEAD == origin/master`, a clean index
and tracked/untracked worktree, and the five source identities accepted in Test Oracle
Correction Source Review 01. The functional falsification and green counts were:

- exact bootstrap-address falsification: exit 101, one selected test failed because
  removing the production guard returned `NodeProbeView(Ready)`, then `rpc.rs` was
  restored to SHA-256
  `2186f6fd56cfcce1edfbe00cd247ef2ab2177ba04dfd9b505416a1da33cbe4ed`;
- `xmr_rpc`: 15 passed;
- `xmr_process`: 12 passed;
- `xmr_distribution`: 12 passed;
- `native_surface`: 17 passed;
- native-ui check: exit 0;
- Node policy: exactly 86 `ok`, no `not ok`, correct final line; and
- repository security policy: exit 0 with the correct final line.

Those results cannot be accepted as the ticket's exact gate. The falsification compile
emitted two warnings: an unused `core::fmt::Write` import and eight never-constructed
future-phase `RpcMethod` variants. The handoff required an immediate stop on any warning
or diagnostic. Hermes instead continued, piped the native check output through `tail`
rather than running the exact command unmodified, created evidence, committed and
pushed, then reran all seven green commands after commit/push. Each green command
therefore ran twice, contrary to the run-once sequence and post-push stop.

The committed Green Evidence 01 omits the warnings, output pipeline, and post-commit
rerun. It is rejected and must not be used for acceptance. The integrated source is not
reverted; it becomes the clean baseline for a bounded warning correction and a new,
independent Green Evidence 02. No result from Resume 06 may be reused.

The warning correction remains local to `wallet-broker/src/xmr/rpc.rs`: remove the
unused formatting-trait import, and make the existing test-only dispatch classifier
construct every closed-authority `RpcMethod` while continuing to return true only for
the eight request variants implemented in Slice 3. No `allow` attribute, test change,
new dispatch capability, public API, or future-phase execution is authorized.
