# WAL-011 shutdown expected red 01

Actor: Hermes, execution/evidence only. Source and integration actors are closed.
Protected parent: reviewer publication following c7f83995; one CURRENT-only launch
commit may follow. Read CURRENT lines 1–40 only and this handoff. Standing AGENTS.md,
TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed. No historical reload.

Submit this exact short launcher once via terminal background=true,
notify_on_complete=true, with workdir set to the repository root. Do not prefix
with cd or retype the driver. Wait only on that same process, at most 60 seconds
per wait, then report its actual result and stop. No independent preflight,
source/metadata discovery, command replay, repairs or manual evidence writing.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_SHUTDOWN_RED_01.md").read_text(); code=s.split("```python\n# WAL011_SHUTDOWN_RED_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_SHUTDOWN_RED_01_DRIVER","exec"))'
```

Only evidence writes: wallet-broker/target/wal011-shutdown-red-01.json and
docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md. The existing target filesystem was
verified ext4. The Node suite may create/remove only its small empty temporary
test directories as specified by its accepted source. No source changes, Git
mutations, extra tests, syntax/build commands, dependencies, scans or network.
Preserve every unrelated pending file, including the two untracked test paths.
The driver reads one exact environment key and its session database row; no
credential/config discovery or alternate metadata search is authorized.

Expected command: node test/walletSupervisorShutdown.node.js, once, 45-second limit.
On Linux, expect exit 1, zero ok, six not ok, each at absent shutdown API. A pass,
different failure, timeout, module/syntax error, cleanup failure or skip is
unexpected. Record actual output and stop without repair or rerun. This proves
only the absent API baseline, not shutdown behavior or real-child cleanup.

```python
# WAL011_SHUTDOWN_RED_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-shutdown-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'test/walletSupervisorShutdown.node.js': '69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7',
    'test/fixtures/wallet-broker/shutdown-child.js': '0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb',
    'wallet-broker/supervisor.js': '1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8',
    'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
    'test/fixtures/wallet-broker/transport-child.js': 'ba11ba8a37d4ee7a6df2e3499c40c972ca4a1cfd8ef0eed377afe1c7a1e3ba91',
    'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700',
}
record = {'commands': [], 'expected_red': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def command(argv):
    try:
        p = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=45)
        row = {'argv': argv, 'exit': p.returncode, 'output': p.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    record['commands'].append(row)
    print(json.dumps(row), flush=True)
    return row
def required(argv):
    row = command(argv)
    assert row['exit'] == 0, 'metadata command failed; stop'
    return row['output'].strip()
try:
    assert sys.platform.startswith('linux'), 'this expected-red contract requires Linux'
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
    result = command(['node', 'test/walletSupervisorShutdown.node.js'])
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'input changed; stop'
    lines = result['output'].splitlines()
    failures = [s for s in lines if s.startswith('not ok ')]
    record['counts'] = {'ok': sum(s.startswith('ok ') for s in lines), 'not_ok': len(failures)}
    assert result['exit'] == 1, 'unexpected test exit; stop'
    assert record['counts'] == {'ok': 0, 'not_ok': 6}, 'unexpected group counts; stop'
    assert failures == [
        'not ok shutdown: no-start and failed-start resolve one Promise without spawn, kill, or timers',
        'not ok shutdown: bound cancel precedes SIGTERM and close, not exit, completes one Promise',
        'not ok shutdown: prior quit, protocol failure, and close-before-shutdown keep one termination',
        'not ok shutdown: stubborn child SIGKILL at 250 ms and TIMEOUT at 1500 ms',
        'not ok shutdown: real child normal termination observes close and closed streams',
        'not ok shutdown: real child ignoring SIGTERM terminates with SIGKILL after close',
    ], 'unexpected failed groups; stop'
    assert lines.count('AssertionError [ERR_ASSERTION]: absent shutdown API') == 6, 'unexpected failure reason; stop'
    assert 'cleanup also failed' not in result['output'], 'cleanup failed; stop'
    assert not any(s.startswith('skip ') for s in lines), 'unexpected skip; stop'
    record['expected_red'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 shutdown expected-red evidence 01\n\n'
        'One driver invocation; actual results below. expected_red=true means the reviewed absent-API failure was observed, not suite green or real-child cleanup acceptance. No production change or integration is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('EXPECTED_RED='+str(record['expected_red']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['expected_red'] else 1)
```
