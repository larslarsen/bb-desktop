# Codex Luna Handoff — BBD-WAL-006 Fixture/Red Evidence Integration 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This durable file is authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Protected governance parent: the commit containing this handoff

Read `AGENTS.md`, `TESTING.md`, ticket, Resumes 04–06, and fixture/expected-red review 01.
Do not rerun any formatter, test, Node, Cargo, or generation command. The previously
observed results are reviewer-accepted, including the exact `E0433`/`E0432` pair.

Require protected `HEAD == origin/master`, clean index, `git diff --check`, frozen
committed Resume-04 inputs, and exactly the six accepted ZEC tests plus these 16 frozen
fixture files. Require test hashes from Resume 06/04 and:

| Frozen fixture path | Bytes | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/tests/fixtures/zec/manifest.json` | 8,708 | `0c5836405b29a2af618a9fa2784a973c981e807c3a8d46471aeeb90d8a6fe389` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000100.compact` | 270 | `a228cae6189ccbff51ed901e641dd59916beb223ff7244359a0d3b334fcaad06` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000101.compact` | 74 | `6ef47b02e1324b31b6187b4c96e396f23b20f988f5693deb6caca3fe4d4b1195` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000102.compact` | 74 | `8826eacd4aa8d68b5c7cbcd8dc4afbf13ec74cafeea30ce3d4ae590773e0a964` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000103.compact` | 270 | `b0f51ef571fda2516ecd6982e9523e7567a9d38236bc81a07df56cc3b510fe64` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000104.compact` | 272 | `514f9ebea982e455dcf81576a4f6dd5a144d7399a9af61bbb2497dc7b66f72fb` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000105.compact` | 76 | `74319b469c52e1b6ab876ef04fbf4b38b1b65aaa11c630c37aa52a9f6ace957a` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000106.compact` | 76 | `5cfe279807dbf3cb88d4e6b470b5f5c8f24c59b91f9836248b2e9c5ba9dd8a45` |
| `wallet-broker/tests/fixtures/zec/blocks/canonical-000107.compact` | 272 | `7ae90de483f9d2f0628be98c9129ac36dc6427c70bb62e74b2faae1fecbf1bf6` |
| `wallet-broker/tests/fixtures/zec/blocks/reorg-replacement-000107.compact` | 272 | `503565aeb51f7065a5a614d14cd4e8045fb865b84e480db3ae63ef4a0de800e9` |
| `wallet-broker/tests/fixtures/zec/blocks/discontinuity-wrong-prev-000107.compact` | 272 | `2db71f38015ca0071c462c61a7bbce080da09319e89398db6d711fc3c927c168` |
| `wallet-broker/tests/fixtures/zec/blocks/discontinuity-height-gap-000109.compact` | 272 | `b0eaaf0ba0a881a6ba7fe33def1418e25a62e308a2c1de7922c77b08fc107213` |
| `wallet-broker/tests/fixtures/zec/blocks/impossible-tree-state-000107.compact` | 268 | `67017418f42edaf876ef19df78a495823773b1494d258319f59de3880d294ef0` |
| `wallet-broker/tests/fixtures/zec/blocks/truncated-000107.compact` | 271 | `0edc2de6333f781ebace3f4cd5480c64946fa76c9b583336de3962e4f885d174` |
| `wallet-broker/tests/fixtures/zec/blocks/malformed.compact` | 1 | `76be8b528d0075f7aae98d6fa57a6d3c83ae480a8469e668d7b0af968995ac71` |
| `wallet-broker/tests/fixtures/zec/blocks/corrupt-wire-type-000107.compact` | 272 | `bbfd9bbe5d5f91f3dde5ea436c75378d4a3cdf1174edd90e37f5734e1bf609d4` |

Require byte-for-byte equality with the ignored generated output and exactly 16 frozen
files. Create only `docs/testing/BBD-WAL-006-EXPECTED-RED-01.md` with the complete
evidence contract from Resumes 04–06, including:

- both prior clean stops and their exact causes;
- formatter success and corrected 928-line fixture-test hash;
- two four-pass locked/offline fixture runs and no-wallet-rewind behavior;
- every generated/frozen path, mode, length, hash, manifest linkage/scenario result, and
  repeat-equality result;
- Node 66/7 totals and seven exact names;
- focused Rust exit 101, zero tests, `E0433` line 6 and `E0432` line 9, both solely absent
  production `zec`; and
- pre/post input hashes and all negative-capability/no-canary/no-secret/no-network claims.

Update only `docs/handoff/CURRENT_TASK.md` to
`FIXTURE AND EXPECTED RED RECORDED — REVIEW REQUIRED` and link the evidence.

Run only read-only state/hash/equality inspection and `git diff --check`; do not execute
tests or alter source/test/fixture bytes. Stage with an explicit path list only the six
ZEC tests, 16 frozen fixture files, evidence, and `CURRENT_TASK.md`. Inspect the exact 24
staged paths/content. Commit `test: record WAL-006 fixture and expected red` and push.
Require `HEAD == origin/master`, clean index/worktree except ignored target state. Do not
amend, clean/delete, run an executable/test/generator, edit production/manifest/policy/
lock, access network/live endpoint/wallet/node/device, build/package/SBOM, or touch other
repositories. Stop; XHigh owns evidence/fixture acceptance and Phase-C authorization.
