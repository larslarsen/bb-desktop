# WAL-011 async Electron IPC focused green 01

Actor: Hermes, execution/evidence only. Grok is closed. No integration authorized.
Protected parent: reviewer publication following 24d5662b, with one CURRENT-only
launch commit allowed. Read CURRENT lines 1–44 only and this handoff. Standing
AGENTS.md, TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed.

Submit the short launcher below exactly once from the repository root via terminal
background=true, notify_on_complete=true. Wait only on that same process, at most
60 seconds per wait, then report the actual result and stop. No independent
preflight, source discovery, manual evidence transcription, repairs or retry.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_ASYNC_IPC_GREEN_01.md").read_text(); code=s.split("```python\n# WAL011_ASYNC_IPC_GREEN_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_ASYNC_IPC_GREEN_01_DRIVER","exec"))'
```

Four stages: Electron security green (23 groups), preload green (six groups),
one-line reversal falsification (21 ok/two expected failures), exact restoration
and Electron green again (23 groups). No other suites, builds, npm/Cargo/scans,
dependencies, network or Git mutations. Stop at the first unexpected result;
the driver must still restore its own mutant in finally. Never rerun the driver.
This proves the registered main handler's async boundary with controlled fixtures;
it does not prove startup, real Electron-to-Rust composition or native flows.

Only writes: the temporary exact social-main.js reversal and restoration,
wallet-broker/target/wal011-async-ipc-green-01.json and
docs/testing/BBD-WAL-011-ASYNC-IPC-GREEN-01.md. Target filesystem was verified ext4.
Keep all unrelated pending npm/policy and evidence files unchanged. The existing
package-policy failure is neither altered nor waived; final release gates remain
separate. No credential/config discovery. Metadata is one exact environment key
and its session row, without fallback.

```python
# WAL011_ASYNC_IPC_GREEN_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-async-ipc-green-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-ASYNC-IPC-GREEN-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'social-main.js': '2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4',
    'test/electronSecurity.node.js': 'df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3',
    'wallet-broker/supervisor.js': '1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8',
    'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
    'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e',
    'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df',
    'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e',
    'test/fixtures/wallet-pay/snapshots-v1.json': 'bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252',
    'docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md': 'fa17e0060ca058560fde00309d6b4c5265c0493ee11c9e327cb150a6ef602435',
}
record = {'commands': [], 'stages': [], 'success': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
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
    assert row['output'].splitlines()[-1] == footer, name+': incomplete footer; stop'
try:
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
    assert record['input_hashes'] == pins, 'source identity mismatch; no execution'
    green('electron-green', 'test/electronSecurity.node.js', 23, 'BitBook electron security tests passed (23).')
    green('preload-green', 'test/walletPreload.node.js', 6, 'BitBook wallet preload tests passed (6).')
    assert hashes() == pins, 'inputs changed before falsification; stop'
    main = Path('social-main.js')
    original = main.read_bytes()
    fixed = b'    return Promise.resolve(result).then(cloneBoundary);\n'
    broken = b'    return cloneBoundary(result);\n'
    assert original.count(fixed) == 1, 'mutation anchor mismatch'
    mutant = original.replace(fixed, broken, 1)
    record['mutant_sha256'] = hashlib.sha256(mutant).hexdigest()
    assert record['mutant_sha256'] == 'b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df'
    try:
        main.write_bytes(mutant)
        result = command(['node', 'test/electronSecurity.node.js'], 'settlement-falsification')
    finally:
        assert main.read_bytes() in (mutant, original), 'unexpected concurrent main edit; preserve and stop'
        main.write_bytes(original)
        record['restored_sha256'] = hashlib.sha256(main.read_bytes()).hexdigest()
    assert hashes() == pins, 'restoration or frozen input mismatch; stop'
    lines = result['output'].splitlines()
    assert result['exit'] == 1 and result['counts'] == {'ok': 21, 'not_ok': 2}, 'falsification not established; stop'
    assert [s for s in lines if s.startswith('not ok ')] == [
        'not ok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel',
        'not ok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors',
    ], 'unexpected falsification failures; stop'
    assert lines.count('AssertionError [ERR_ASSERTION]: wallet:snapshot:get did not return a thenable') == 2, 'unexpected falsification reason; stop'
    assert lines[-1] == '2 electron security test(s) failed', 'incomplete falsification footer; stop'
    green('electron-restored-green', 'test/electronSecurity.node.js', 23, 'BitBook electron security tests passed (23).')
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'final input mismatch; stop'
    record['success'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 async IPC focused validation 01\n\n'
        'One driver invocation. Actual results, metadata, complete stage output and restoration identities follow. This is controlled main-handler/preload evidence, not real app startup or release acceptance. No integration is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('VALIDATION_SUCCESS='+str(record['success']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['success'] else 1)
```
