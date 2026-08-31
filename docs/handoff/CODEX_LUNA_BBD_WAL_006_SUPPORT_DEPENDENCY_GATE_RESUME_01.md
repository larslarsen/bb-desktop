# Codex Luna Handoff — BBD-WAL-006 Support-Dependency Gate Resume 01

You are **Jr Dev — Codex Luna**. This durable file resumes the stopped gate; ephemeral
chat is not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff.

Read completely: the original support-dependency gate handoff, its review 01,
`CURRENT_TASK.md`, ticket, accepted source reviews, both changed source files, frozen
lock, and accepted Node test.

## Accepted prior result and preflight

Do not rerun Node. Reviewer accepts its exact exit 1, 71 `ok`, three `not ok`, with only
the three deferred ZEC policy groups red. The prior metadata command exited 0 and made no
lock diff; it is not rerun and is not evidence of resolution.

Require `HEAD == origin/master` at this protected governance parent, clean index,
exactly the two accepted modified source paths and hashes from the original handoff,
frozen test hash, frozen lock 5,367-line/hash state, no ZEC source path, and no other
tracked/untracked change.

## Exact resume sequence

Run only these commands, separately, via Rust/Cargo 1.98.0:

```text
/home/lars/.cargo/bin/rustup run 1.98.0 cargo check --manifest-path wallet-broker/Cargo.toml --offline --no-default-features --test vault_crypto
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline -e features -i rand_core@0.6.4
/home/lars/.cargo/bin/rustup run 1.98.0 cargo tree --manifest-path wallet-broker/Cargo.toml --locked --offline -e features -i rusqlite@0.37.0
/home/lars/.cargo/bin/rustup run 1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline --no-default-features --test vault_crypto
```

The first command is intentionally not `--locked`; it is the sole authorized lock
mutation and must otherwise check successfully. Immediately after it, inspect the entire
lock diff before running either tree command. It must add exactly `rand_core` and
`rusqlite` to the `bitbook-wallet-broker` package dependency array. No package block,
version, checksum, source, transitive edge, or other byte may change. If the lock is
still unchanged or differs more broadly, stop without another command/evidence/Git.

After an exact lock diff, apply all remaining feature/checksum/source/no-new-package
requirements from the original handoff. The two tree commands must pass locked/offline,
and the custody target must pass all 11 tests with frozen vectors. No warning or output
may widen the accepted support authority.

## Evidence and integration

If exact, create the same single evidence path authorized by the original handoff,
`docs/testing/BBD-WAL-006-SUPPORT-DEPENDENCY-GATE-01.md`. Include the accepted prior
Node result, the ineffective metadata stop, this resume's command/status results, exact
lock diff/checksums/features, 11-test custody result, no-new-package/build-script/source/
license conclusion, hashes/lines, absent ZEC source, and final state.

Update only `docs/handoff/CURRENT_TASK.md` to `SUPPORT-DEPENDENCY GATE INTEGRATED —
ADDRESS SOURCE REVIEW REQUIRED` and link evidence. Stage/commit/push exactly the five
paths listed in the original handoff, using the same commit message. Require clean
tracked worktree/index and `HEAD == origin/master`; report all original requested facts.

Do not rerun Node/metadata, edit source/tests, run another Cargo/npm/Node command, use
network, or perform any action forbidden by the original handoff.
