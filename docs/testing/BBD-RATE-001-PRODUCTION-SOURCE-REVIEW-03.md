# BBD-RATE-001 Production-Source Review 03

Reviewer: Lead Engineer/Reviewer — Codex at High

Governance HEAD at review: `3f127b4e`

Result: **ACCEPTED FOR HERMES FALSIFICATION AND GREEN GATE**

Accepted production identity:

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `quote-worker/providers.js` | 50 | `e473fb6d32f6dcaa19f8f5825ef47c0a63ca068767f0b75960a2c65d9102470e` |
| `quote-worker/model.js` | 490 | `5e00387eb93d2c2a8e7407e262400175c2aaa37006c35ad7a63daca1fb5969fa` |
| `quote-worker/framing.js` | 460 | `abb27a761e7ba42157ced917ee0da4409c9cd97e5681c83bcb5058ebcf80404e` |
| `quote-worker/worker.js` | 333 | `cbde0dd4242aaf85b3310b149b803e217a34799389edf7b924d0bcb1f7e19674` |
| `quote-worker/supervisor.js` | 286 | `b465c4c5bf3c5226f4e2acf7b555e2b96c90ef043e43ec601423edc831bba825` |
| `package.json` | 38 | `f8b13d53e80c8f91c87a473e3c873999a337078f8eae90779814ac368a10197a` |
| `scripts/security-policy.js` | 2,667 | `f66f6df408d434082b14b8e8a5e1bb61722a7f5bc09c97c7a5e224793b301e7e` |
| `.github/workflows/social.yml` | 153 | `5968dc31bbc72bfc010417381a3b6f83df1f1fa6abf9f71275b007b8254dc9b2` |
| `.github/workflows/security.yml` | 61 | `9b890179bcb5b8ade9503a43ec97c18ed3bca0ab4e2d7e1f0ebcec495225be4e` |

Frozen test identity:

| Path | SHA-256 |
| --- | --- |
| `test/rateWorker.node.js` | `242de6e756e5d4db27d21fda16aedd2d7b7b9183d39a218ffe22c55a949736f8` |
| `test/rateSupervisor.node.js` | `8f3938cc46a86a893760d92a07da9a30b8d11f713514b89d2f8d77d42c866d80` |
| `test/securityPolicy.node.js` | `1bc23cc41fd5c50b855637d80c342e0de3cc9ed8cd48219163f626b06e4391f6` |
| `test/fixtures/rates/provider-bodies-v1.json` | `7179969a17b92c02115ab6224266609c5a29af51c486734c6e321393da52a15b` |

Correction 02 closes both remaining lifecycle defects. The worker's five-second request
timeout now aborts or destroys the active request and enters the existing guarded
single-settlement failure transition. The independent ten-second overall timeout remains
intact. The supervisor now captures and detaches the current child, clears the authoritative
child reference, and best-effort kills only a still-live process while catching signal errors.
Natural child termination therefore cannot leave a reusable dead handle.

The accepted source remains a separate default-off quote process, pins only Coinbase
Exchange ZEC/USD and Kraken XMR/USD, preserves decimal strings without floating-point price
arithmetic, and adds no wallet, Pay, renderer, daemon, or live-default provider authority.
All public exports and all previously frozen production and test bytes remain exact.

`git diff --check` was clean before this governance update. The reviewer ran no Node, npm,
test, provider, package-manager, scanner, or network command. Acceptance is conditional on
Hermes completing the authorized five-mechanism falsification and full local green gate.
