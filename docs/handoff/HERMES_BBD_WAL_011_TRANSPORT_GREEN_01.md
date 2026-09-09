# WAL-011 transport validation and integration 01

Actor: Hermes. Protected parent: reviewer publication directly after 434ea35b, one
CURRENT-only launch allowed. Read AGENTS.md, TESTING.md, active CURRENT/ticket,
GROK_BBD_WAL_011_TRANSPORT_PRODUCTION_01.md collected review and this task.
No historical reload. Source actors closed. High remains sufficient.

Verify HEAD/status and the four exact source identities in the collected review,
plus frozen protocol.js 79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4.
Stop on mismatch. Preserve unrelated npm/policy files and prior WAL-009 evidence.
Record `hermes --version` and your session ID/provider/model using narrow session
metadata; do not use full status/config output. Previous red session actually used
provider nous, model poolside/laguna-s-2.1:free, session 20260909_135906_d3d5a1.

## Exact falsification, then green

Run this single restoration-safe falsification command from the repository root.
It suppresses only matching successful public-response settlement, preserving the
real handshake/bootstrap. It must fail the primary real-child result assertion.
The wrapper must exit 0 only on the exact expected failure and restored hash.

```bash
python3 - <<'WAL011_FALSIFY_PUBLIC_SETTLEMENT'
from pathlib import Path
import hashlib, subprocess
p = Path('wallet-broker/supervisor.js')
original = p.read_bytes()
expected = '1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8'
assert hashlib.sha256(original).hexdigest() == expected
needle = b'    settle(entry, null, publicResult(entry.method, value.result));'
assert original.count(needle) == 1
javascript = """const tests = require('./test/walletSupervisorTransport.node.js').tests;
const selected = tests.filter(t => t.name === 'transport: real framed hello, ack, bootstrap and a subsequent request return a distinct fixture result');
if (selected.length !== 1) throw new Error('missing unique primary test');
selected[0].fn().then(() => { console.log('UNEXPECTED MUTANT PASS'); process.exitCode = 0; }, error => { console.error(error.stack || error); process.exitCode = 1; });"""
try:
    p.write_bytes(original.replace(needle, b'    void entry; // temporary settlement falsification'))
    result = subprocess.run(['node', '-e', javascript], stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, text=True, timeout=15)
    print(result.stdout, end='')
    print('MUTANT_EXIT=' + str(result.returncode))
    assert result.returncode == 1
    assert 'subsequent request did not settle; missing framed matching reply' in result.stdout
finally:
    p.write_bytes(original)
    assert hashlib.sha256(p.read_bytes()).hexdigest() == expected
    print('RESTORED_SHA256=' + expected)
WAL011_FALSIFY_PUBLIC_SETTLEMENT
```

After successful falsification/restoration, run each command once, in this order.
Stop at the first unexpected failure; no retry, repair or broader suite.

1. `node test/walletSupervisorTransport.node.js` — all seven groups pass.
2. `node test/walletSupervisor.node.js` — all thirteen groups pass.
3. `node test/walletBrokerProtocol.node.js`
4. `node test/walletPreload.node.js`
5. `node test/walletPay.node.js`
6. `node test/electronSecurity.node.js`

Use a 45-second tool timeout for each suite; record exact exit codes and counts.
No npm, policy tests/audits, Rust, scanner, build, network or unrelated commands.
Only Git push below may contact the existing remote. The stopped package work stays
stopped; these focused results do not satisfy final app/release security acceptance.

## Evidence and integration

Write docs/testing/BBD-WAL-011-TRANSPORT-GREEN-01.md with runtime identity, baseline,
exact falsification outcome/restored hash, each executed command/result/count and
any skipped commands. Name the source review's fallback-cleanup limitation; do not
claim a complete wallet, general leak-free shutdown or final security acceptance.
Sanitize local absolute paths in records. Do not infer results from expected values.

Correct docs/testing/BBD-WAL-011-TRANSPORT-RED-01.md alongside this work: runtime
model/session, actual timeout wrapper and captured test exit, and failure causes
from its collected review. Remove unsupported independent-cleanup claims. Preserve
the actual six failures and original input hashes; no red rerun. No other evidence.

On any unexpected failure, verify source restoration, record results, then stop
without Git mutation or source edits. On complete success, verify all five source
hashes/status once and run `git diff --check` limited to the four source and two
evidence paths below. Stage exactly:

- wallet-broker/supervisor.js
- test/walletSupervisorTransport.node.js
- test/fixtures/wallet-broker/transport-child.js
- test/walletSupervisor.node.js
- docs/testing/BBD-WAL-011-TRANSPORT-RED-01.md
- docs/testing/BBD-WAL-011-TRANSPORT-GREEN-01.md

Inspect `git diff --cached --stat` to confirm only those six paths. Commit with
`git commit -m "Wire live wallet broker transport with correlated replies"`, then
`git push`. If push fails, report it; no unrelated repair or force push. Do not edit
CURRENT, ticket, other docs, manifests or production beyond the temporary restored
falsification. Report commit/push and stop immediately; no post-success extra checks.
