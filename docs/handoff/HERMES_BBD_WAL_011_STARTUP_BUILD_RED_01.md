# WAL-011 build helper expected red

Hermes execution/evidence only, nous / poolside/laguna-s-2.1:free. Corrected builder
test source accepted at 614e3ad8. Six absent-helper assertions expected. Only reads
this handoff; only writes raw/evidence below. Main production actor is independent;
do not read or modify main. Run exact launcher once in absolute repository workdir,
background=true, notify_on_complete=true. No extra commands, retries, repair, Git,
production or other actors. Wait same process at most 60 seconds, report and stop.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_STARTUP_BUILD_RED_01.md").read_text(); code=s.split("```python\n# WAL011_STARTUP_BUILD_RED_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_STARTUP_BUILD_RED_01","exec"))'
```

```python
# WAL011_STARTUP_BUILD_RED_01
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-startup-build-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-STARTUP-BUILD-RED-01.md')
pins = {'test/walletBrokerBuild.node.js': '614e3ad8dde731c344d23353076672e0ba046a3e9d4919e303b5601350b97035', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'test/walletStartupSmoke.node.js': '35d87dc7c3f8f9f919277e5c16cc66e22fdc7228718d62ef455652b32dc3f09f', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71'}
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
    assert not Path('scripts/build-wallet-broker.js').exists(), 'builder unexpectedly present'
    row = command(['node','test/walletBrokerBuild.node.js'])
    failed = [line[7:] for line in row['output'].splitlines() if line.startswith('not ok ')]
    passed = [line for line in row['output'].splitlines() if line.startswith('ok ')]
    assert row['exit'] == 1 and failed == ['module: exports synchronous stageWalletBroker', 'stage: copied bytes pin independently, modes match identity, resolver accepts layout', 'stage: repeat replacement preserves unrelated files and retains staging leftovers', 'preflight: invalid source, roots, and identities leave destination bytes unchanged', 'preflight: destination directory and pin links leave bytes unchanged', 'cli: vm-mocked cargo command is exact and nonzero skips staging'] and not passed, 'unexpected builder red'
    assert row['output'].count('AssertionError [ERR_ASSERTION]: wallet broker build helper is missing') == 6, 'wrong red diagnostic'
    assert 'cleanup also failed' not in row['output'], 'fixture cleanup failure'
    assert not Path('scripts/build-wallet-broker.js').exists(), 'builder unexpectedly changed'
    record['counts'] = {'ok':len(passed),'not_ok':len(failed)}
    record['accepted'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    finish('WAL-011 build command expected red 01')
sys.exit(0 if record['accepted'] else 1)
```

## Collected acceptance

Hermes outer 85502 closed, session 20260910_085135_5a8edd; nous /
poolside/laguna-s-2.1:free, v0.18.2 upstream 67764dc0/local 10b6d1a9.
Six explicit missing-helper assertions, Node exit 1, no fixture cleanup failure.
All six frozen inputs unchanged; normalized evidence/raw match independently.
Expected red accepted. Only builder production is now authorized; no replay.
