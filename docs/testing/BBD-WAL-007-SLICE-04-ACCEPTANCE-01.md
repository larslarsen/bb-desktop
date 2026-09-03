# BBD-WAL-007 Slice-4 Acceptance 01

Reviewer: Lead Engineer/Reviewer — Codex at High

Result: **ACCEPTED**

Accepted implementation commit:
`3aed346e801600826852500c0a07bf8567a7e972`
(`feat: add BBD-WAL-007 Monero account custody`)

Accepted evidence-correction commit:
`118cd61ae7141af0997ff678bccfc16617a1d912`
(`docs: correct WAL-007 slice 4 green evidence`)

`HEAD == origin/master == 118cd61a`; the index and tracked/untracked worktree are
clean. The implementation commit contains exactly the eight accepted XMR source paths,
Green Evidence 01, and `CURRENT_TASK.md`. The later correction commit changes exactly
Green Evidence 01 and `CURRENT_TASK.md`. The accepted source identities are recorded in
the corrected Green Evidence 01.

The reviewer reran no formatter, test, check, Clippy, Node, policy, build, or product
binary. The retained Hermes transcript and corrected evidence establish:

- the Rust 1.98 formatter check exited 0;
- the exact lock falsification exited 101 on the intended assertion with exactly 0
  passed, 1 failed, 0 ignored, 0 measured, and 15 filtered, after which the accepted
  account source identity was restored;
- `xmr_account`, `xmr_hygiene`, `xmr_rpc`, `xmr_process`, `xmr_distribution`,
  `vault_crypto`, `vault_format`, `vault_store`, `vault_session`, `secret_hygiene`, and
  `native_surface` passed exactly 16, 9, 15, 12, 12, 11, 11, 20, 13, 11, and 17 tests,
  with zero failures;
- warning-denied Clippy and the native-ui check exited 0;
- the Node policy suite produced exactly 86 `ok`, no `not ok`, and the correct final
  line; and
- the security-policy script exited 0 with the correct final line.

The implementation/execution transcript was not wholly protocol compliant. It used a
wrapped filesystem probe, attempted the wrong push target before the successful exact
push, continued after that mismatch, captured only eight staged names, and did not
inspect the complete staged diff before commit. Reviewer inspection independently
established the exact ten-path commit, accepted the tested source and technical gate,
and required the bounded evidence correction. Commit `118cd61a` accurately discloses
all four deviations and removes the invalid exact-protocol claims.

The evidence-correction transcript is accepted for its documentation-only purpose. It
edited and staged exactly the two authorized paths, inspected the complete staged diff
and name list, used the exact commit message and `git push origin master`, and performed
only read-only Git proof afterward.

Slice 4's account custody, durable identity/state, software creation, watch-only import,
authenticated recovery/open, lock behavior, rollback, cleanup, and secret-wiping
boundaries are accepted. This acceptance authorizes only the linked Slice-5 Grok
viewing/fresh-receiver source handoff. Hermes execution/integration, broader/final
acceptance, and the real offline local-Monero gate remain closed.
