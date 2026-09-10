# WAL-011 launch configuration expected red 01

Actor: Hermes, execution/evidence only. Grok and integration are closed.
Protected parent: reviewer publication following 1df0900726cc4da4cfa61adafd85ae50154d2555,
plus one CURRENT-only launch record. Read CURRENT lines 1–40 and this handoff only.
AGENTS.md, TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed. No history.

Submit the exact launcher once via terminal background=true, notify_on_complete=true,
with repository-root workdir. Do not prefix cd or transcribe its driver. Wait only
on that process, at most 60 seconds per wait, then report actual output and stop.
No independent preflight, source/metadata discovery, repair, retry or manual evidence.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_LAUNCH_CONFIG_RED_01.md").read_text(); code=s.split("```python\n# WAL011_LAUNCH_CONFIG_RED_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_LAUNCH_CONFIG_RED_01_DRIVER","exec"))'
```

Only writes: wallet-broker/target/wal011-launch-config-red-01.json and
docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md. Target was verified ext4.
The accepted Node test owns only small temporary files with unlink/rmdir cleanup;
its absent-resolver baseline should stop each group before fixture construction.
No source/test/Git changes, production stub, extra suites, syntax/build commands,
dependencies, scans, network or credential/config discovery. Metadata is one exact
session key and that row only. Preserve all unrelated pending files.

One command: node test/walletBrokerLaunchConfig.node.js, 45-second limit.
Expected exit 1, zero ok/five not ok, all at the explicit missing-resolver assertion.
There is no failure footer in this runner; exact five named failure groups and
five matching assertions establish the complete expected result. Missing-module
exceptions, syntax errors, other failed assertions, timeout or a pass are unexpected.
Record actual output and stop, never repair or rerun. The 58 fixture rows remain
unexecuted until green. Driver exit 0 means expected red recorded, not suite green.

```python
# WAL011_LAUNCH_CONFIG_RED_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-launch-config-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'test/walletBrokerLaunchConfig.node.js': '1b6a7d8c102360efdbd0920b6211e7b55999fbdfff18aff4eee23cd311d750f8',
    'social-main.js': 'c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3',
    'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a',
    'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
    'test/electronSecurity.node.js': '7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c',
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
    module = Path('wallet-broker/launch-config.js')
    record['production_absent_before'] = not module.exists() and not module.is_symlink()
    assert record['production_absent_before'], 'production unexpectedly present; stop'
    result = command(['node', 'test/walletBrokerLaunchConfig.node.js'])
    record['production_absent_after'] = not module.exists() and not module.is_symlink()
    assert record['production_absent_after'], 'production changed; stop'
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'source changed; stop'
    lines = result['output'].splitlines()
    failures = [line for line in lines if line.startswith('not ok ')]
    record['counts'] = {'ok': sum(line.startswith('ok ') for line in lines), 'not_ok': len(failures)}
    expected_failures = ['not ok valid configuration: six platform identities return frozen supervisor options', 'not ok invalid options: reject missing extra accessor prototype and path identities', 'not ok invalid manifest: reject malformed extra mismatched and hostile digest rows', 'not ok manifest size: 4096 bytes succeed; 4097 and empty reject', 'not ok inventory: missing symlink and directory entries fail closed']
    assert result['exit'] == 1, 'unexpected test exit; stop'
    assert record['counts'] == {'ok': 0, 'not_ok': 5}, 'unexpected group counts; stop'
    assert failures == expected_failures, 'unexpected failed groups; stop'
    assert lines.count('AssertionError [ERR_ASSERTION]: packaged broker launch resolver is missing') == 5, 'unexpected diagnostics; stop'
    assert 'cleanup also failed' not in result['output'], 'unexpected cleanup failure; stop'
    record['expected_red'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 launch configuration expected-red evidence 01\n\n'
        'One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; all 58 fixture case bodies remain unexecuted on the absent-resolver baseline. No integration or validation replay is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('EXPECTED_RED='+str(record['expected_red']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['expected_red'] else 1)
```
