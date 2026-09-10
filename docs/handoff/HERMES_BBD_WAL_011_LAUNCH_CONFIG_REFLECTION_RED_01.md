# WAL-011 reflection expected red 01

Actor: Hermes, execution/evidence only. Source and integration closed.
Parent: reviewer publication following 09aee197 plus CURRENT-only launch record.
Read CURRENT lines 1–40 and this handoff; AGENTS/TESTING/routing if needed.
Submit the exact extraction launcher once via terminal background=true,
notify_on_complete=true, repository-root workdir. No independent commands,
source edits, retries, repairs, Git mutations, config/history discovery or extra
actors. Wait on your process in intervals at most 60 seconds and report actual
output. Reviewer will collect without waiting for owner done.

Only writes: wallet-broker/target/wal011-launch-config-reflection-red-01.json and
docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md. Existing target ext4.
One Node invocation of exactly the four added exported tests, not the five earlier
groups. Expected exit 1, zero ok/four not ok at each named code assertion.
Unexpected result stops without repair or rerun. Metadata exact session key/row only.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_LAUNCH_CONFIG_REFLECTION_RED_01.md").read_text(); code=s.split("```python\n# WAL011_LAUNCH_CONFIG_REFLECTION_RED_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_LAUNCH_CONFIG_REFLECTION_RED_01_DRIVER","exec"))'
```

```python
# WAL011_LAUNCH_CONFIG_REFLECTION_RED_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-launch-config-reflection-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'wallet-broker/launch-config.js': '4a639b1644ff0ea618fc53fd525cbd5c0fde84fbea5905f3f3eb56a5520dc01a',
    'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113',
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
    result = command(['node', '-e', "const assert=require('assert'); const names=['revoked proxy','getPrototypeOf trap','ownKeys trap','getOwnPropertyDescriptor trap'].map(x=>'reflection errors: '+x+' is sanitized'); const tests=require('./test/walletBrokerLaunchConfig.node.js').tests.filter(t=>names.includes(t.name)); assert.deepStrictEqual(tests.map(t=>t.name),names); let failed=0; for(const {name,fn} of tests){try{fn(); console.log('ok '+name);}catch(e){failed++; console.error('not ok '+name+'\\n'+e.stack);}} process.exitCode=failed?1:0;"])
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'source changed; stop'
    lines = result['output'].splitlines()
    failures = [line for line in lines if line.startswith('not ok ')]
    record['counts'] = {'ok': sum(line.startswith('ok ') for line in lines), 'not_ok': len(failures)}
    expected_failures = ['not ok reflection errors: '+name+' is sanitized' for name in ['revoked proxy','getPrototypeOf trap','ownKeys trap','getOwnPropertyDescriptor trap']]
    assert result['exit'] == 1, 'unexpected test exit; stop'
    assert record['counts'] == {'ok': 0, 'not_ok': 4}, 'unexpected group counts; stop'
    assert failures == expected_failures, 'unexpected failed groups; stop'
    for name in ['revoked proxy','getPrototypeOf trap','ownKeys trap','getOwnPropertyDescriptor trap']:
        assert 'AssertionError [ERR_ASSERTION]: '+name+': code' in result['output'], 'unexpected assertion; stop'
    assert 'cleanup also failed' not in result['output'], 'unexpected cleanup failure; stop'
    record['expected_red'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 launch configuration reflection expected-red evidence 01\n\n'
        'One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was unchanged; only four reflection regression groups were executed. No integration or validation replay is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('EXPECTED_RED='+str(record['expected_red']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['expected_red'] else 1)
```

## Launcher stop and resume 01

Outer 55437 completed without driver execution: relative handoff path lookup
failed before extraction. No raw/evidence files exist and the two source/test
hashes remain unchanged. This is not test evidence. Authorize one fresh attempt
of the same exact driver using the explicit absolute repository workdir supplied
in the launch prompt. Set terminal workdir to that exact path; do not infer it.
No driver or test changes, no extra commands.

## Expected-red acceptance

Resume outer 10847 completed exit 0. Node exit 1: zero ok/four not ok, exactly
each named code assertion; frozen hashes unchanged. Raw/evidence JSON equality
and current hashes independently verified. Session 20260909_225544_2c118d,
nous / poolside/laguna-s-2.1:free; Hermes v0.18.2 (2026.7.7.2), upstream
8e85b276/local 10b6d1a9, Python 3.11.15. HEAD 1e69bba5. Exact-session messages
80404–80413 show handoff read, exact launcher with correct workdir, wait and log
collection, report; no extra command or mutation. Actor report understates pending
files; raw git status is authoritative. Raw 68 lines, SHA-256
7e8c27ea8edcf46959f8a29c2e1d8bda375a409c6b84edaf3e00cf0dacd02662.
Evidence 74 lines, SHA-256
b19697561f63f4690304f0c57a8eef89e86204ba9d57024758bd509f32063523.
Hermes closed. Only the Grok reflection correction handoff is authorized.
