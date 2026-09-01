# BBD-RATE-001 Test-Source Review 03

Decision: **ACCEPTED FOR EXPECTED-RED EXECUTION**

Reviewer: Lead Engineer/Reviewer — Codex at XHigh

Reviewer governance HEAD before the final drop: `c8eefb55`

## Accepted source identity

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `test/rateWorker.node.js` | 1,156 | `242de6e756e5d4db27d21fda16aedd2d7b7b9183d39a218ffe22c55a949736f8` |
| `test/rateSupervisor.node.js` | 549 | `8f3938cc46a86a893760d92a07da9a30b8d11f713514b89d2f8d77d42c866d80` |
| `test/securityPolicy.node.js` | 2,844 | `1bc23cc41fd5c50b855637d80c342e0de3cc9ed8cd48219163f626b06e4391f6` |
| `test/fixtures/rates/provider-bodies-v1.json` | 263 | `7179969a17b92c02115ab6224266609c5a29af51c486734c6e321393da52a15b` |

The accepted inventory is 20 worker tests, 16 supervisor tests, and four RATE-001 policy
tests within 82 total policy tests. Only these four authorized paths differ from
governance HEAD, and `git diff --check` passes.

## Review result

The source now provides an offline, non-vacuous contract for the simplified decision:
Coinbase Exchange ZEC/USD, Kraken XMR/USD, USD only, one visibly labeled source, and all
provider traffic default-off. It independently covers exact decimal strings and
half-even conversion; strict timestamp, JSON, depth, size, duplicate-key, pair, and
product rejection; fresh-only snapshots; exact framing; clean child process isolation;
request pins; separate connect and overall timeouts; one in-flight request; causal capped
backoff; five-minute cache expiry; and complete private-context canaries.

The supervisor cases independently prove clean spawn inputs, exact request/response
frames, unknown/duplicate provider rejection, pending-request bounds, protocol-failure
termination, and diagnostic containment. The real zero-provider child entry is exercised
through an offline Node subprocess. The policy tests reserve exact package/build/CI paths,
source import allowlists, exact provider URLs, rejected alternate hosts/paths, and no
renderer CSP grant.

Correction 02 removed the last contradictory raw-stderr assertion, split connect from
overall timeout behavior, stopped requiring redundant abort after completed failures,
restored the in-flight oracle, and replaced the fake child-entry claim with the actual
entry boundary. No production source, dependency, lockfile, wallet, Pay, renderer, or
other repository path is authorized by this acceptance.

The reviewer executed no Node, npm, test, provider, network, or integration command.
Passing behavior is not claimed. Hermes alone may integrate these exact bytes and record
the expected red under `docs/handoff/HERMES_BBD_RATE_001_EXPECTED_RED_01.md`.
