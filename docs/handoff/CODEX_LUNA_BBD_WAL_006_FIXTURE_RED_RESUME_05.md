# Codex Luna Handoff — BBD-WAL-006 Fixture and Expected Red Resume 05

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, ticket, fixture-generation review 01, fixture reorg source
review 01, and Resume 04 completely. Resume 04 remains the complete fixture/expected-red/
evidence/integration contract except where this file supersedes it.

Require protected `HEAD == origin/master`, clean index, `git diff --check`, absent
`wallet-broker/target/wal006-fixture-build`, and exactly six untracked ZEC tests. Replace
Resume 04's fixture-builder row with:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/zec_fixture_builder.rs` | 928 | `4b1efec59f81761e2c713587c0a4f3e7b8c545f7b85cc35c90949c5dedbca4bc` |

All five sibling tests and all committed manifest/lock/policy/Node/vault inputs must
match Resume 04 exactly. Revalidate the same ignored disk-backed target and exact
TMPDIR/CARGO_TARGET_DIR.

Because the corrected test source is new, first run exactly:

```text
env TMPDIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-tmp CARGO_TARGET_DIR=/home/lars/OpenBazaar/bb-desktop/wallet-broker/target/wal006-cargo /home/lars/.cargo/bin/cargo +1.98.0 fmt --manifest-path wallet-broker/Cargo.toml --check
```

It must exit 0 without changing a byte. Any formatting diff rejects the source; stop
without formatting, fixture generation, evidence, integration, or Git.

On formatter success, execute Resume 04 from its first fixture-builder invocation onward
exactly. The corrected source receives two fresh locked/offline builder invocations: each
must report exactly four passed, and the second must verify byte-for-byte deterministic
output. Apply every inspection, exact 16-path freeze, Node 66/7 expected red, focused
`zec_address` E0433-only red, evidence, explicit staging, commit/push, and prohibition in
Resume 04 without alteration.

In `BBD-WAL-006-EXPECTED-RED-01.md`, additionally record the prior rejected run's two-pass/
two-fail `RequestedRewindInvalid` cause, the accepted correction source hash, formatter
result, and proof that corrected generation used no wallet rewind. Do not create a
separate evidence path.

Run no resolution/tree/metadata/fetch/network, other target/suite, source/test repair,
production edit, cleanup, wallet/node/device, package/SBOM, or other-repository action.
Stop on any mismatch; XHigh owns fixture/red acceptance and Phase-C authorization.
