# WAL-011 Electron normal quit expected red 01

Actor: Hermes, execution/evidence only. Source and integration actors are closed.
Protected parent: reviewer publication following ca0f945543fdd551207c8dc39586f4596c69c751,
plus one CURRENT-only launch record. Read CURRENT lines 1–40 and this handoff only.
AGENTS.md, TESTING.md and HERMES_JR_DEV_ROUTING.md may be read if needed. No history.

Submit this exact extraction launcher once via terminal background=true,
notify_on_complete=true and repository-root workdir. Do not prefix with cd or
transcribe the driver. Wait only on that same process, at most 60 seconds per wait,
then report actual output and stop. No independent preflight/metadata commands,
source reads, tests, repairs, retries or manual evidence transcription.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_ELECTRON_QUIT_RED_01.md").read_text(); code=s.split("```python\n# WAL011_ELECTRON_QUIT_RED_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_ELECTRON_QUIT_RED_01_DRIVER","exec"))'
```

Only writes: wallet-broker/target/wal011-electron-quit-red-01.json and
docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md. The existing target was verified
ext4 by the reviewer. No source/test/Git mutation, builds, dependencies, scans,
network, extra suites or credential/config discovery. Preserve unrelated pending
npm/policy files and historical evidence. Metadata uses the exact HERMES_SESSION_ID
key and that session row only; no alternative discovery if it is absent.

Run once: node test/electronSecurity.node.js, 45-second limit. Expected exit 1,
23 ok/seven not ok: the seven new quit groups each fail at the explicit absent
before-quit handler assertion, with the complete seven-failure footer. Their later
lifecycle assertions and second rejection row remain unexecuted on this baseline.
A pass, other failure, missing module, syntax error, timeout or incomplete output
is unexpected. The driver records actual output and stops; do not repair or rerun.
Driver exit 0 means expected red recorded, not suite green or production acceptance.

```python
# WAL011_ELECTRON_QUIT_RED_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-electron-quit-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {
    'test/electronSecurity.node.js': '7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c',
    'social-main.js': '2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4',
    'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a',
    'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df',
    'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e',
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
    expected_failures = ['not ok after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit', 'not ok before ready, fulfilled no-child shutdown allows one resumed quit without windows', 'not ok shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills', 'not ok rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE', 'not ok synchronous shutdown throw is contained and keeps quit blocked with one fixed error box', 'not ok rejected shutdown plus throwing error box stays contained without resumed quit', 'not ok window-all-closed uses the host platform branch and the same before-quit gate']
    assert result['exit'] == 1, 'unexpected test exit; stop'
    assert record['counts'] == {'ok': 23, 'not_ok': 7}, 'unexpected group counts; stop'
    assert failures == expected_failures, 'unexpected failed groups; stop'
    assert lines.count('AssertionError [ERR_ASSERTION]: before-quit handler is missing') == 7, 'unexpected diagnostics; stop'
    assert lines[-1] == '7 electron security test(s) failed', 'missing complete suite footer; stop'
    record['expected_red'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 Electron quit expected-red evidence 01\n\n'
        'One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; the seven new lifecycle bodies and second rejection row remain unexecuted on the absent-handler baseline. No integration or validation replay is authorized.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('EXPECTED_RED='+str(record['expected_red']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['expected_red'] else 1)
```
