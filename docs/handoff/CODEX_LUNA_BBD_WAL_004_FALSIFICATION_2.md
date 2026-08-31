# Codex Luna Handoff — BBD-WAL-004 Corrected Isolated Falsification

You are **Jr Dev — Codex Luna**. This durable file and the original falsification handoff
are the complete prompt; ephemeral chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely: `AGENTS.md`, `TESTING.md`, roles, `tickets/BBD-WAL-004.md`,
`docs/testing/BBD-WAL-004-FALSIFICATION-RUN-01.md`, the original
`CODEX_LUNA_BBD_WAL_004_FALSIFICATION.md`, every path/test it names, and `CURRENT_TASK.md`.

Require `HEAD == origin/master` at the governance parent, clean worktree/index, and all
green-evidence hashes. Apply all original universal isolation, inverse-`apply_patch`,
hash, exact-failure-reason, disk-backed target, and stop rules.

Replace only original case 1 with this corrected pairing:

1. In `SecretBytes::wipe_with` in `wallet-broker/src/vault.rs`, temporarily remove only
   `bytes.zeroize();`, leaving the observer and its post-wipe `all_zero` scan intact. Run:

   ```text
   /home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test secret_hygiene panic_unwind_zeroizes_secret_before_control_returns -- --exact
   ```

   Require nonzero exit because `decrypt-plaintext` lacks a true post-zeroize observation.
   A panic caught only by the inner test is not enough; the integration test itself must
   fail at the all-zero assertion. Immediately restore and verify the exact vault hash.

After corrected case 1 restores exactly, execute original cases 2–7 literally and in
order. Do not rerun the incorrect drop-test pairing. Never overlap mutations.

After all seven intended failures and exact restorations, follow the original evidence
and Git section literally: create only `docs/testing/BBD-WAL-004-FALSIFICATION.md`, update
only `CURRENT_TASK.md` to `FALSIFICATION GREEN — CI SECURITY/SBOM GATES PENDING` with
active handoff `NONE — REVIEWER CI GATE REVIEW`, stage only those two docs, commit once
as `test: falsify wallet custody invariants`, and push. Report all exact failures,
restored hashes, evidence count/hash, commit, clean state, and push. Do not dispatch
workflows; reviewer owns them.
