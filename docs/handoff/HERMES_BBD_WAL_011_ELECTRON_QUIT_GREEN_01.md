# WAL-011 Electron normal quit focused validation 01

Actor: Hermes, execution/evidence only. Grok and integration are closed.
Protected parent: reviewer publication following 00e7be5d4f1f97b06f842ff16b06754fca687741,
plus one CURRENT-only launch commit. Read CURRENT lines 1–40 and this handoff only.
AGENTS.md, TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed. No history.

Submit this exact extraction launcher once via terminal background=true,
notify_on_complete=true, with repository-root workdir. Do not prefix cd or retype
its driver. Wait only on the same process, at most 60 seconds per wait, then report
actual output and stop. No independent preflight, source discovery, repair,
manual evidence transcription or retry.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_ELECTRON_QUIT_GREEN_01.md").read_text(); code=s.split("```python\n# WAL011_ELECTRON_QUIT_GREEN_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_ELECTRON_QUIT_GREEN_01_DRIVER","exec"))'
```

Four stages, 45-second limit per command:

1. node test/electronSecurity.node.js: exit 0, 30 ok/zero not ok, complete footer.
2. node test/walletPreload.node.js: exit 0, six ok/zero not ok, complete footer.
3. Temporarily insert approveNormalQuit() immediately after the shutdown Promise
   subscription. Run only the first new quit test via the exact node -e expression
   below. It must reject at the initial zero-quit assertion: actual 1, expected 0,
   ERR_ASSERTION, not timeout/cleanup error or an unexpected pass. No test edit.
4. Restore exact main bytes in finally, verify all inputs, then repeat the full
   30-group Electron green. Stop at any unexpected result; restore the driver's
   own mutant even on failure. Never rerun the driver or repair source.

The selected assertion is at frozen electronSecurity.node.js line 1419.
Mutant SHA-256: 12896f8cd1e2fa3ed089028c845633adcfe311b14d31370334577b11071256f6.
Restored source: c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3.
All eight authored new cases must complete in the full green run; the host branch
is exercised without claiming native cross-platform execution.

Only writes: this exact temporary social-main.js mutant/restoration,
wallet-broker/target/wal011-electron-quit-green-01.json and
docs/testing/BBD-WAL-011-ELECTRON-QUIT-GREEN-01.md. Target was verified ext4.
No Git integration, extra tests, syntax/build commands, dependencies, scans,
network or configuration/credential discovery. Metadata is the exact session key
and row only. Preserve all unrelated pending files and accepted red evidence.

This validates registered main callbacks with controlled Electron/supervisor
fixtures; startup, packaging pins, real app-to-Rust composition, native flows and
release acceptance remain future work. Existing package-policy failures are not
waived. No supervisor/transport/Rust replay is authorized.

```python
# WAL011_ELECTRON_QUIT_GREEN_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-electron-quit-green-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-ELECTRON-QUIT-GREEN-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'social-main.js': 'c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3',
    'test/electronSecurity.node.js': '7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c',
    'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a',
    'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
    'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e',
    'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df',
    'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e',
    'test/fixtures/wallet-pay/snapshots-v1.json': 'bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252',
    'docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md': '229c1a97b501dcf0a1f60865245e76c644520776500b06f3c3ff5d0ddedf167b',
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
    green('electron-green', 'test/electronSecurity.node.js', 30, 'BitBook electron security tests passed (30).')
    green('preload-green', 'test/walletPreload.node.js', 6, 'BitBook wallet preload tests passed (6).')
    assert hashes() == pins, 'inputs changed before falsification; stop'
    main = Path('social-main.js')
    original = main.read_bytes()
    fixed = b'    walletSupervisor.shutdown().then(approveNormalQuit, failNormalQuit).then(() => {}, () => {});\n'
    broken = b'    walletSupervisor.shutdown().then(approveNormalQuit, failNormalQuit).then(() => {}, () => {});\n    approveNormalQuit();\n'
    assert original.count(fixed) == 1, 'mutation anchor mismatch'
    mutant = original.replace(fixed, broken, 1)
    record['mutant_sha256'] = hashlib.sha256(mutant).hexdigest()
    assert record['mutant_sha256'] == '12896f8cd1e2fa3ed089028c845633adcfe311b14d31370334577b11071256f6'
    try:
        main.write_bytes(mutant)
        result = command(['node', '-e', "const assert = require('assert'); const name = 'after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit'; const selected = require('./test/electronSecurity.node.js').tests.filter(t => t.name === name); assert.strictEqual(selected.length, 1); selected[0].fn().then(() => { console.log(JSON.stringify({unexpectedPass:true,name})); process.exitCode=2; }, error => { console.log(JSON.stringify({name,code:error.code,actual:error.actual,expected:error.expected,message:error.message,stack:error.stack})); process.exitCode=1; });"], 'early-quit-falsification')
    finally:
        assert main.read_bytes() in (mutant, original), 'unexpected concurrent main edit; preserve and stop'
        main.write_bytes(original)
        record['restored_sha256'] = hashlib.sha256(main.read_bytes()).hexdigest()
    assert hashes() == pins, 'restoration or frozen input mismatch; stop'
    assert result['exit'] == 1, 'falsification did not reject; stop'
    detail = json.loads(result['output'].strip())
    assert detail['name'] == 'after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit', 'wrong selected test; stop'
    assert detail['code'] == 'ERR_ASSERTION' and detail['actual'] == 1 and detail['expected'] == 0, 'wrong falsification assertion; stop'
    assert 'electronSecurity.node.js:1419:' in detail['stack'], 'wrong falsification location; stop'
    record['falsification'] = detail
    green('electron-restored-green', 'test/electronSecurity.node.js', 30, 'BitBook electron security tests passed (30).')
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'final input mismatch; stop'
    record['success'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 Electron quit focused validation 01\n\n'
        'One driver invocation. Actual results, metadata, complete stage output and restoration identities follow. This is controlled normal-quit/main-handler/preload evidence, not real app startup or release acceptance. No integration is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('VALIDATION_SUCCESS='+str(record['success']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['success'] else 1)
```
