# WAL-011 runtime package expected red and policy baseline 01

Actor: Hermes, execution/evidence only; source and integration closed.
Parent: reviewer publication following b78bbc0f plus CURRENT-only launch record.
Read this handoff and CURRENT active prefix only; AGENTS/TESTING/routing if needed.
Submit exact launcher once with terminal background=true, notify_on_complete=true,
workdir explicitly the absolute repository path in the prompt. No cd, independent
commands, transcription, retry, repair, source edits, Git mutations, discovery or
extra actors. Wait only on that process at most 60 seconds per wait; report and stop.
Reviewer collects directly without owner done.

Only writes: wallet-broker/target/wal011-runtime-package-red-01.json and
docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md. Existing target ext4.
One node test/walletRuntimePackage.node.js run: expected exit 1, zero ok/seven
explicit helper-missing assertions. Then once each node test/securityPolicy.node.js
and node scripts/security-policy.js to capture current inherited policy failures.
The latter outputs are baseline only and not release acceptance; all input hashes
must remain unchanged. Stop for any different focused red or baseline timeout.
No packaging, dependency download, native build or production stub authorized.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_RUNTIME_PACKAGE_RED_01.md").read_text(); code=s.split("```python\n# WAL011_RUNTIME_PACKAGE_RED_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_RUNTIME_PACKAGE_RED_01_DRIVER","exec"))'
```

```python
# WAL011_RUNTIME_PACKAGE_RED_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-runtime-package-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {'scripts/build-deb.sh': '3ea4d0eba122eedcf17e8044e25dcb33dba998609c6213c1138233da3535b396', 'scripts/build-macos.sh': '74ea1c30684722dff2446bc6818e7207acc384e7ff7c56e4311a4e9ad47fb285', 'scripts/build-windows.ps1': '49619fae88bc01a3f16728e3f595cd70ff8bee35d43971c5a9b43b8ccae4ac28', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'social-main.js': 'c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3', 'packaging/runtime-package.json.in': '1660c63cf7d36abdfacf47333a69496c3a03121819100cb64a55e2c11a658d9e', 'test/walletRuntimePackage.node.js': '5d8a8bf250a4e664fb15853cfb29deee62130efd72cdcf3acdac1129c2324f25', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md': '1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f', 'docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md': '2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56'}
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
    module = Path('scripts/stage-wallet-runtime.js')
    record['production_absent_before'] = not module.exists() and not module.is_symlink()
    assert record['production_absent_before'], 'production unexpectedly present; stop'
    result = command(['node', 'test/walletRuntimePackage.node.js'])
    record['production_absent_after'] = not module.exists() and not module.is_symlink()
    assert record['production_absent_after'], 'production changed; stop'
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'source changed; stop'
    lines = result['output'].splitlines()
    failures = [line for line in lines if line.startswith('not ok ')]
    record['counts'] = {'ok': sum(line.startswith('ok ') for line in lines), 'not_ok': len(failures)}
    expected_failures = ['not ok staging: exact five-file inventory preserves bytes and excludes native data', 'not ok preflight: missing and hostile source entries leave destination unchanged', 'not ok preflight: existing and hostile destinations are never overwritten', 'not ok arguments: invalid roots fail before staging', 'not ok cli: real checkout stages modules whose nested imports resolve', 'not ok cli: missing and extra arguments fail without staging', 'not ok packagers: shared staging precedes signing and archive with Linux disk storage']
    assert result['exit'] == 1, 'unexpected test exit; stop'
    assert record['counts'] == {'ok': 0, 'not_ok': 7}, 'unexpected group counts; stop'
    assert failures == expected_failures, 'unexpected failed groups; stop'
    assert lines.count('AssertionError [ERR_ASSERTION]: wallet runtime staging helper is missing') == 7, 'unexpected diagnostics; stop'
    assert 'cleanup also failed' not in result['output'], 'unexpected cleanup failure; stop'
    record['policy_baseline'] = []
    for argv in [['node','test/securityPolicy.node.js'], ['node','scripts/security-policy.js']]:
        row = command(argv)
        assert row['exit'] in (0,1) and not row.get('timeout'), 'policy baseline failed to execute'
        record['policy_baseline'].append({'argv': argv, 'exit': row['exit'], 'output': row['output']})
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'baseline changed inputs; stop'
    record['expected_red'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 runtime package expected red and policy baseline 01\n\n'
        'One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; all 44 packaging fixture cases remain unexecuted on the absent-helper baseline. Policy results are a recorded baseline, not waived failures. No integration or validation replay is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('EXPECTED_RED='+str(record['expected_red']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['expected_red'] else 1)
```
