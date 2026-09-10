# WAL-011 awaitable shutdown focused validation 01

Actor: Hermes, execution/evidence only. Source actors and integration are closed.
Protected parent: reviewer publication following b16b6aa6, plus one CURRENT-only
launch commit. Read CURRENT lines 1–40 only and this handoff. Standing AGENTS.md,
TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed. No historical reload.

Submit the short launcher once via terminal background=true, notify_on_complete=true
and repository workdir. Do not prefix cd or retype the driver. Wait only on that
same process at most 60 seconds per wait; report actual output and stop. No separate
preflight, discovery, manual evidence transcription, repairs or retries.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_SHUTDOWN_GREEN_01.md").read_text(); code=s.split("```python\n# WAL011_SHUTDOWN_GREEN_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_SHUTDOWN_GREEN_01_DRIVER","exec"))'
```

Five stages: shutdown suite (six Linux groups, both real children), affected live
transport suite (seven groups), supervisor suite (13 groups), selected fake-child
premature-completion falsification, and restored shutdown suite (six groups).
Stop at the first unexpected result; restore the driver-owned mutant in finally.
No other suites, Rust/build/npm/scan commands, dependency changes or Git mutations.

Only production write is the exact temporary supervisor mutation/restoration below.
Only evidence writes: wallet-broker/target/wal011-shutdown-green-01.json and
docs/testing/BBD-WAL-011-SHUTDOWN-GREEN-01.md. Target is verified ext4. Tests may
create their accepted temporary fixture directories/processes and must clean them
according to their source. Preserve every unrelated pending file. No credential or
config discovery; use only the exact session key/row in the driver. No network.

Expected falsification is ERR_ASSERTION with actual resolved and expected pending
in the held-close case. Missing test selection, absent API, syntax/fixture errors,
cleanup failures or a pass do not establish falsification. This validates the
supervisor boundary with Node fixtures, not Electron startup or native descendants.

```python
# WAL011_SHUTDOWN_GREEN_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-shutdown-green-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-SHUTDOWN-GREEN-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a',
    'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
    'test/walletSupervisorShutdown.node.js': '69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7',
    'test/fixtures/wallet-broker/shutdown-child.js': '0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb',
    'test/fixtures/wallet-broker/transport-child.js': 'ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91',
    'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700',
    'test/walletSupervisor.node.js': 'eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c',
    'docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md': '32e6d2b69e05e95def6eaac0ccce6d4ff606b535fc362376934716c028b443ae',
}
record = {'commands': [], 'stages': [], 'success': False}
def hashes():
    return {p: hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in pins}
def command(argv, name=None):
    try:
        p = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=45)
        row = {'argv': argv, 'exit': p.returncode, 'output': p.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    if name is not None:
        row['name'] = name
        lines = row['output'].splitlines()
        row['counts'] = {'ok': sum(s.startswith('ok ') for s in lines), 'not_ok': sum(s.startswith('not ok ') for s in lines)}
        record['stages'].append(row)
    else:
        record['commands'].append(row)
    print(json.dumps(row), flush=True)
    return row
def required(argv):
    row = command(argv)
    assert row['exit'] == 0, 'metadata command failed; stop'
    return row['output'].strip()
def green(name, file, count, footer):
    row = command(['node', file], name)
    assert row['exit'] == 0, name+': unexpected exit; stop'
    assert row['counts'] == {'ok': count, 'not_ok': 0}, name+': unexpected counts; stop'
    assert not any(s.startswith('skip ') for s in row['output'].splitlines()), name+': unexpected skip'
    assert row['output'].splitlines()[-1] == footer, name+': incomplete footer; stop'
try:
    assert sys.platform.startswith('linux'), 'Linux validation contract only'
    record['version'] = required(['hermes', '--version'])
    record['head'] = required(['git', 'rev-parse', 'HEAD'])
    record['initial_status'] = required(['git', 'status', '--short'])
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'current session absent; no discovery'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        row = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?', (sid,)).fetchone()
    assert row and all(row), 'session metadata absent; no discovery'
    record['session'] = dict(zip(('id', 'model', 'provider'), row))
    record['input_hashes'] = hashes()
    assert record['input_hashes'] == pins, 'input identity mismatch; no execution'
    green('shutdown-green', 'test/walletSupervisorShutdown.node.js', 6, 'BitBook wallet supervisor shutdown tests passed (6).')
    green('transport-green', 'test/walletSupervisorTransport.node.js', 7, 'BitBook wallet supervisor transport tests passed (7).')
    green('supervisor-green', 'test/walletSupervisor.node.js', 13, 'BitBook wallet supervisor tests passed (13).')
    assert hashes() == pins, 'inputs changed before falsification; stop'
    source = Path('wallet-broker/supervisor.js')
    original = source.read_bytes()
    anchor = b'      quit();\n      return shutdownPromise;'
    replacement = b'      quit();\n      completeShutdown();\n      return shutdownPromise;'
    assert original.count(anchor) == 1, 'mutation anchor mismatch'
    mutant = original.replace(anchor, replacement, 1)
    record['mutant_sha256'] = hashlib.sha256(mutant).hexdigest()
    assert record['mutant_sha256'] == '25fe467451f58ed2cc12b45206d6471c1c1eb7c7aad3f016d459e4b8f3657a12'
    selected = "const {tests}=require('./test/walletSupervisorShutdown.node.js');const name='shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise';const matches=tests.filter(t=>t.name===name);if(matches.length!==1)throw new Error('falsification selection mismatch');Promise.resolve().then(()=>matches[0].fn()).then(()=>{console.error('FALSIFICATION_UNEXPECTED_PASS');process.exitCode=2;},e=>{console.error(JSON.stringify({code:e.code,actual:e.actual,expected:e.expected,message:e.message,stack:e.stack}));process.exitCode=1;});"
    try:
        source.write_bytes(mutant)
        result = command(['node', '-e', selected], 'premature-completion-falsification')
    finally:
        assert source.read_bytes() in (mutant, original), 'unexpected concurrent source edit; preserve and stop'
        source.write_bytes(original)
        record['restored_sha256'] = hashlib.sha256(source.read_bytes()).hexdigest()
    assert hashes() == pins, 'restoration or frozen input mismatch; stop'
    assert result['exit'] == 1, 'falsification did not fail as required; stop'
    error = json.loads(result['output'].strip())
    assert error['code'] == 'ERR_ASSERTION' and error['actual'] == 'resolved' and error['expected'] == 'pending', 'unexpected falsification reason; stop'
    assert 'cleanup also failed' not in error['message'], 'falsification cleanup failed; stop'
    green('shutdown-restored-green', 'test/walletSupervisorShutdown.node.js', 6, 'BitBook wallet supervisor shutdown tests passed (6).')
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'final inputs changed; stop'
    record['success'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 shutdown focused validation 01\n\n'
        'One driver invocation. Actual metadata, complete stage output and restoration identities follow. This validates supervisor lifecycle with Node fixtures, not app startup, native descendants or release readiness. No integration is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('VALIDATION_SUCCESS='+str(record['success']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['success'] else 1)
```


## Collected validation acceptance — 2026-09-09

Accept focused lifecycle validation and premature-completion falsification; no
rerun. Outer 1099 collected on owner done with exit 0. Actual session
20260909_195724_724f82, provider nous, model poolside/laguna-s-2.1:free.
Hermes v0.18.2 (2026.7.7.2), upstream 8e85b276/local 10b6d1a9, Python 3.11.15.
Observed HEAD 4ec2cf351c9ddee96d7ba0f4104a955ca48feb82.

| Stage | Exit | Outcome |
| --- | --- | --- |
| Shutdown green | 0 | six ok, zero not ok, no skips |
| Transport regression | 0 | seven ok, zero not ok |
| Supervisor regression | 0 | 13 ok, zero not ok |
| Premature-completion falsification | 1 | ERR_ASSERTION, actual resolved, expected pending |
| Restored shutdown green | 0 | six ok, zero not ok, no skips |

The six shutdown groups execute both real-child cases: normal termination and
SIGTERM-ignore/SIGKILL termination, with observed closure and accepted cleanup.
Fake-clock deadline/ordering and repeated-call cases pass. These are 26 distinct
passing groups; the final six-group pass repeats the focused suite after restoration.
The selected falsification fails at the held-close assertion (test line 670), with
no cleanup failure. Its JSON output uses no ok/not-ok lines, so the raw stage's
zero/zero counters do not describe a skipped or passing test.

Restored source is c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a.
All eight input hashes agree before, after and at reviewer collection. Evidence
JSON exactly matches normalized raw JSON. Identities:

- wallet-broker/target/wal011-shutdown-green-01.json: SHA-256
  bda8e5a9a421a115c426ec431117ad61b9c691767027921b9b393c8fea5b4177.
- docs/testing/BBD-WAL-011-SHUTDOWN-GREEN-01.md: 135 lines, SHA-256
  a34373da4315d49f95953d996e3a573b468513a512af19abfb2e80137b838248.

Exact-session inventory (80342–80349) shows bounded CURRENT/handoff reads, one
exact extraction launcher with correct workdir, one process wait and final report.
No extra commands, repair, discovery, source/test redesign or Git mutations appear.
Reviewer ran no tests/builds. This accepts the Node supervisor lifecycle boundary,
not Electron app lifecycle wiring, native descendants or package/release readiness.

Execution is closed. Only the linked
[five-path integration driver](HERMES_BBD_WAL_011_SHUTDOWN_INTEGRATION_01.md) is
authorized. Preserve unrelated npm/policy changes and other untracked evidence.
