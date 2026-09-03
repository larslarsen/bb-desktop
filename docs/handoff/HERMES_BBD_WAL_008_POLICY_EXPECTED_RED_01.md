# Hermes Handoff — BBD-WAL-008 Policy Expected Red 01

You are **Jr Dev — Hermes**. Repository: `/home/lars/OpenBazaar/bb-desktop`.

Protected governance parent: the commit containing this handoff

Read completely: `AGENTS.md`, `TESTING.md`, both role/routing policies,
`docs/handoff/CURRENT_TASK.md`, `tickets/BBD-WAL-008.md`, Policy Test Source Review 01,
Resume-03 Stop Review 01, `test/securityPolicy.node.js`, and
`scripts/security-policy.js`.

Record actual Hermes version, provider, and model. Preflight must prove:

- branch `master` and exact `HEAD == origin/master` at the protected parent;
- a clean index and exactly four dirty worktree paths: the accepted policy test plus
  the three accepted Slice-02 Rust paths below;
- all four frozen identities, unchanged manifest/lockfile/production policy, and clean
  `git diff --check`; and
- the repository and Cargo work directories remain on disk-backed ext4.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/securityPolicy.node.js` | 3,358 | `464a576803678a12165609a51df14a43630dbf52925ecb6cb22842e6fa226e07` |
| `wallet-broker/src/zec/hardware.rs` | 924 | `9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760` |
| `wallet-broker/src/zec/store.rs` | 2,849 | `1ab107367c3f7af4581bba770dbb1943955b6ebe5ad0303d31c512fdab3e4b2a` |
| `wallet-broker/src/zec/test_support.rs` | 2,500 | `e3edc609dc6e3eaa80de52b6269caa0a84525e91e761483013b92affe5048f82` |

Frozen unchanged boundaries:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/Cargo.toml` | 117 | `7d89a7e98fde65c69392a91e633436ccf93b65a2f8e826e22b5eec27804da530` |
| `wallet-broker/Cargo.lock` | 5,394 | `29a5e4a8b9fa40ca612f3b7c3b1f90d527544a519c06e50bc11aae7803f57420` |
| `scripts/security-policy.js` | 2,689 | `6aba356098134e90731928a2a1083565d52a6d000c213b89549ba231997f5626` |

Stop on any preflight mismatch. Otherwise submit this command byte-for-byte, alone,
once, with no `cd`, wrapper, redirection, pipeline, environment prefix, or appended
shell text:

```text
node test/securityPolicy.node.js
```

Require exit 1, exactly 80 `ok`, exactly seven `not ok`, and final line
`7 security policy test(s) failed`. The seven failing groups must be exactly:

1. `committed workflows satisfy the fail-closed checker`
2. `strict nine-line reviewed Gitleaks ratchet bytes and content are enforced`
3. `WAL-004 Rust test-harness manifest pins the exact toolchain, dependencies, and native features`
4. `WAL-006 manifest requires six exact defaults-off pins and the minimum direct feature union`
5. `WAL-006 manifest pins the exact production support APIs for RNG and SQLite`
6. `WAL-006 prepare NFC dependency is one exact defaults-off Unicode normalization pin`
7. `BBD-WAL-008 closes the hardware target and current eight-path ZEC policy inventory`

The first six must fail only because the frozen production manifest checker omits the
accepted `zec_hardware` target. The seventh must fail only because the frozen production
policy lacks the new WAL-008 export/checker contract. The renamed historical WAL-006
inventory group must be `ok`. No syntax, module, fixture, exception, unrelated group,
or production-source failure is accepted.

On exact success, create `docs/testing/BBD-WAL-008-POLICY-EXPECTED-RED-01.md` and update
`docs/handoff/CURRENT_TASK.md` to await reviewer acceptance. Record the full literal
command, exit/counts, exact seven groups/causes, all identities, unchanged boundaries,
and scope. Stage exactly the accepted policy test plus those two documentation paths;
leave all three Rust paths unstaged. Commit exactly
`test: require WAL-008 policy inventory`, push `master`, prove clean index,
`HEAD == origin/master`, and exactly the three frozen Rust paths remain dirty, then
stop.

Do not edit developer source, tests, manifests, lockfiles, production policy, workflows,
or evidence outside the two named records. Do not repair the red, run another command,
use Grok, invoke another actor, touch Monero/WAL-007, or start transport/signing/device
work. On any mismatch, stop without evidence, integration, commit, push, or post-stop
verification.
