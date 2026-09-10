# WAL-011 async Electron IPC expected red 01

Actor: Hermes, execution/evidence only. Source actors and integration are closed.
Protected parent: reviewer publication following 46310708, plus one CURRENT-only
launch commit. Read only the active CURRENT prefix and this handoff. Standing
AGENTS.md/TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed. No history.

Run the short extraction launcher below exactly once from the repository root via
terminal background=true, notify_on_complete=true. Wait on that same process only,
at most 60 seconds per wait, then report its output and stop. No other commands,
source reads, discovery, manual evidence transcription, repair or retries.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_ASYNC_IPC_RED_01.md").read_text(); code=s.split("```python\n# WAL011_ASYNC_IPC_RED_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_ASYNC_IPC_RED_01_DRIVER","exec"))'
```

The driver owns metadata, four source pins, one Node execution and exact evidence.
Only writes: wallet-broker/target/wal011-async-ipc-red-01.json and
docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md. The target directory was inspected as
ext4 by the reviewer. No source/test mutation, Git mutation, dependency/build/scan
commands, other suites, network or credentials/config discovery. All unrelated
pending npm/policy files and evidence remain unchanged. The only session discovery
is the exact HERMES_SESSION_ID key and that row's three fields, without fallback.

Expected red: `node test/electronSecurity.node.js`, exit 1, 21 ok and two not ok.
Both delayed groups must fail at `wallet:snapshot:get did not return a thenable`.
Missing modules, syntax errors, timeout, unhandled-rejection crashes, other failed
groups or a pass are unexpected. Record and stop on any mismatch; never rerun.
This red executes only the first row of each delayed group. All five success/two
error rows must execute at later green. The driver records actual output even on
an unexpected result. Driver exit 0 means expected red recorded, not suite green.

```python
# WAL011_ASYNC_IPC_RED_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-async-ipc-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'test/electronSecurity.node.js': 'df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3',
    'social-main.js': 'b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df',
    'wallet-broker/supervisor.js': '1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8',
    'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
}
record = {'commands': [], 'expected_red': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def command(argv, timeout=45):
    try:
        result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout)
        row = {'argv': argv, 'exit': result.returncode, 'output': result.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes):
            output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    record['commands'].append(row)
    print(json.dumps(row), flush=True)
    return row
def required(argv):
    row = command(argv)
    assert row['exit'] == 0, 'metadata command failed; stop'
    return row['output'].strip()
try:
    record['version'] = required(['hermes', '--version'])
    record['head'] = required(['git', 'rev-parse', 'HEAD'])
    record['initial_status'] = required(['git', 'status', '--short'])
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'current session ID absent; no discovery'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        row = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?', (sid,)).fetchone()
    assert row and all(row), 'session metadata absent; no discovery'
    record['session'] = dict(zip(('id', 'model', 'provider'), row))
    record['input_hashes'] = hashes()
    assert record['input_hashes'] == pins, 'source identity mismatch; no test execution'
    result = command(['node', 'test/electronSecurity.node.js'])
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'source changed; stop'
    lines = result['output'].splitlines()
    failures = [line for line in lines if line.startswith('not ok ')]
    record['counts'] = {'ok': sum(line.startswith('ok ') for line in lines), 'not_ok': len(failures)}
    expected_failures = [
        'not ok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel',
        'not ok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors',
    ]
    assert result['exit'] == 1, 'unexpected test exit; stop'
    assert record['counts'] == {'ok': 21, 'not_ok': 2}, 'unexpected group counts; stop'
    assert failures == expected_failures, 'unexpected failed groups; stop'
    assert sum('AssertionError [ERR_ASSERTION]: wallet:snapshot:get did not return a thenable' == line for line in lines) == 2, 'unexpected diagnostics; stop'
    assert lines[-1] == '2 electron security test(s) failed', 'missing complete suite footer; stop'
    record['expected_red'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 async IPC expected-red evidence 01\n\n'
        'One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; later rows of each failing delayed group remain unexecuted. No integration or validation replay is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('EXPECTED_RED='+str(record['expected_red']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['expected_red'] else 1)
```
