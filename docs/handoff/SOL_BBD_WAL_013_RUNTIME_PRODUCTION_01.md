# WAL-013 runtime/menu production — authorized

Reviewer accepts runtime expected red raw wal013-runtime-js-red-01.json:
Hermes20260910_105327_6c4021, nous/poolside/laguna-s-2.1:free. All7 new groups fail
for absent menu/manage/env/grace behavior; builder has5pass and1fail solely missing
native-ui fixed argv. No loader/syntax failures. All exact pins unchanged. Transcript
includes an unnecessary read-only session/path listing before launcher; recorded here,
no source mutation or result impact. Execution is closed, no reruns authorized.
Sol runtime test correction accepted and frozen.

Sol gpt-5.6-sol High source-only, no subagents. Continue documented escalation because
Grok runtime actor stopped40turns with incomplete unusable drop. Implement exactly:
social-main.js, wallet-broker/supervisor.js, wallet-broker/src/runtime.rs,
scripts/build-wallet-broker.js. No tests, Cargo/lib, other source, Git, execution,
formatters or evidence writes. Native UI actor concurrently owns account_ui.rs/lib;
use its fixed public contract without changing its files. Do not wait on that actor.
Read AGENTS.md, TESTING.md, GROK_BBD_WAL_013_RUNTIME_01.md for all fixed semantics,
original service/UI contracts and affected source/tests. No unrelated history/research.

Baseline:
social-main.js 7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0
wallet-broker/supervisor.js c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a
wallet-broker/src/runtime.rs 968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b
scripts/build-wallet-broker.js c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382

Honor all prior strict protocol/framing/replay errors. Requests need retain only safe
validated account-id params; reject unexpected secret-bearing keys as SCHEMA.
Own-data env filtering in BOTH main and supervisor, exact7GUIkeys/4096UTF8bytes.
Normal EOF grace must survive synchronous close inside stdin.end without late timers.
Poisoned manager recover solely lock_all and return unavailable. No busy mutexblocking
wire worker. Clock deadlines tick even hidden/headless. Native loop on mainthread,
protocol worker owns stdout; no indefinite stdin join. No-display headless unchanged.
Build only native-ui flag addition, no dependencies. Do not expose any secret operation
on wire or renderer. Product flows remain Zcash TESTNET and paymentsdisabled.
Complete source-only and stop with hashes/counts; no execution or Git.
