# WAL-011 main startup expected-red execution

Startup test source bf72c9f2 (630 lines, six groups) and Electron mock-only delta
accepted by reviewer. Six main groups prove missing startup on c7687b52 baseline.
Source Grok 09da707c (outer 45126) collected and closed. Build-test correction and
smoke correction are independent active source work; Hermes must not read/change
those files. No production or source integration is authorized.

Hermes nous / poolside/laguna-s-2.1:free. Parent reviewer publication following
6f33f3f3; only this handoff and CURRENT prefix reads. Only writes the raw/evidence
paths embedded below. Run launcher once in absolute repo workdir, background=true,
notify_on_complete=true. No independent commands, retries, fixes, Git or other actors.
Wait on that process in at most 60-second increments, report and stop. Reviewer
collects directly. Existing target is ext4; no substantial temporary files needed.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_STARTUP_MAIN_RED_01.md").read_text(); code=s.split("```python\n# WAL011_STARTUP_MAIN_RED_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_STARTUP_MAIN_RED_01","exec"))'
```

```python
# WAL011_STARTUP_MAIN_RED_01
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-startup-main-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md')
pins = {'social-main.js': 'c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3', 'test/walletStartup.node.js': 'bf72c9f276938b8b2e538bbb7631303837d6f9da998f3a4c279d397a02f1bae3', 'test/electronSecurity.node.js': 'df81caae58607184a0c407f5aa845aa90a2f729c7fa8a9ac0ec9edbc366e8eea', 'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
record = {'commands': [], 'accepted': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def command(argv, timeout=60, env=None):
    try:
        result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout, env=env)
        row = {'argv': argv, 'exit': result.returncode, 'output': result.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    record['commands'].append(row)
    print(json.dumps(row), flush=True)
    return row
def required(argv, timeout=60, env=None):
    row = command(argv, timeout, env)
    assert row['exit'] == 0, 'command failed: '+str(argv)
    return row['output'].strip()
def metadata():
    assert not raw_path.exists() and not evidence_path.exists(), 'existing result; no replay'
    record['version'] = required(['hermes', '--version'])
    record['head'] = required(['git','rev-parse','HEAD'])
    record['initial_status'] = required(['git','status','--short'])
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'missing current session; no discovery'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        row = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?',(sid,)).fetchone()
    assert row and all(row), 'missing metadata'
    record['session'] = dict(zip(('id','model','provider'),row))
    record['input_hashes'] = hashes()
    assert record['input_hashes'] == pins, 'input drift'
def finish(title):
    record['final_hashes'] = hashes()
    if record['final_hashes'] != pins:
        record['accepted'] = False
        record['stop'] = 'input drift'
    raw_path.write_text(json.dumps(record,indent=2)+'\n')
    normalized = json.dumps(record,indent=2).replace(str(Path.cwd()),'<repo>').replace(str(Path.home()),'<home>')
    evidence_path.write_text('# '+title+'\n\nActual bounded execution; release blockers are not waived.\n\n```json\n'+normalized+'\n```\n')
    print('ACCEPTED='+str(record['accepted']),flush=True)

try:
    metadata()
    row = command(['node','test/walletStartup.node.js'])
    failed = [line[7:] for line in row['output'].splitlines() if line.startswith('not ok ')]
    passed = [line for line in row['output'].splitlines() if line.startswith('ok ')]
    assert row['exit'] == 1 and failed == ['ready-only packaged and development paths resolve once with subscribe-before-start', 'unbound snapshot get returns cached down clones then live status after handshake', 'resolver or start failure leaves social usable; fallback still denies sender and payload', 'snapshot cache updates while the window is closed and clones stay isolated', 'pre-ready quit and reentrant quit during resolver never spawn; activate stays idle-gated', 'configured supervisor shutdown is awaited on the replaced instance'] and not passed, 'unexpected startup red'
    assert 'SyntaxError' not in row['output'] and 'MODULE_NOT_FOUND' not in row['output'], 'fixture/load failure'
    record['counts'] = {'ok':len(passed),'not_ok':len(failed)}
    record['accepted'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    finish('WAL-011 main startup expected red 01')
sys.exit(0 if record['accepted'] else 1)
```

## Collected acceptance

Hermes outer 68299 closed; session 20260910_084936_e50477, nous /
poolside/laguna-s-2.1:free, Hermes v0.18.2 upstream 67764dc0/local 10b6d1a9.
Six expected missing-startup failures, Node exit 1; unchanged ten frozen inputs.
The two TypeErrors are missing configured supervisor on the old main, not load
failures; other groups directly prove absent resolver and wrong fallback/quit behavior.
Raw and normalized evidence independently match. Expected red accepted; no replay.
Only social-main.js production now authorized under original fixed startup contract.
