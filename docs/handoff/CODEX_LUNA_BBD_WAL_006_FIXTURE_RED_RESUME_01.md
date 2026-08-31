# Codex Luna Handoff — BBD-WAL-006 Fixture and Expected Red Resume 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable resume supplements
`CODEX_LUNA_BBD_WAL_006_FIXTURE_RED.md`; its new hashes and protected parent supersede
that handoff's initial preflight only. Every other command, expected result, path,
stop condition, evidence requirement, Git boundary, and prohibition remains exact.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read the original Phase-B handoff, formatter capture handoff, Sol format-correction
handoff, `docs/testing/BBD-WAL-006-FORMAT-CORRECTION-REVIEW-01.md`, and every accepted
path completely.

Require `HEAD == origin/master` at the protected parent, a clean index, exactly the eight
uncommitted paths/line counts/hashes in the format-correction review, `git diff --check`,
and the unchanged pre-resolution lockfile. Revalidate that the ignored target/temp/Cargo
directories remain real and disk-backed. Stop on mismatch.

Run the original exact `cargo fmt --check` command once. It must now exit 0 with no source
change or output diff. Any nonzero exit or changed accepted hash is rejection; stop.

If and only if formatting passes, resume the original Phase-B handoff at **Toolchain,
formatting, and resolution** immediately after its formatter paragraph: resolve the
lockfile, inventory the graph, run the upstream fixture gate twice, freeze all 16 exact
fixture paths, run the one exact Node policy command and one exact Rust expected-red
command, record evidence/current state, integrate the named paths, commit, push, and
stop. Do not repeat the already successful tool-version commands, and do not run or edit
anything outside the original handoff.

The evidence must cite both the original protected parent `a13dc587` and this resume's
protected parent, plus the formatter rejection/capture/correction records. The final
accepted source hashes are those in the format-correction review, not the superseded
pre-format hashes. Production and production policy remain unauthorized.
