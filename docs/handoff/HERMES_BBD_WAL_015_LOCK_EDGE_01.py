from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys, signal
record = {'commands': [], 'accepted': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def emit(value):
    try: print(value, flush=True)
    except BrokenPipeError: pass
def command(argv, timeout=60, env=None):
    process = subprocess.Popen(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, env=env, start_new_session=True)
    timed_out = False
    try:
        output, _ = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(process.pid, signal.SIGTERM)
        try: output, _ = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            output, _ = process.communicate(timeout=5)
    row = {'argv': argv, 'exit': process.returncode, 'output': output}
    if timed_out: row['timeout'] = True
    record['commands'].append(row)
    emit(json.dumps(row))
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
    assert record['head'] == 'e85dfdc8b6e06fdd09f84cee3c6ac37037448437', 'HEAD drift'
    record['input_hashes'] = hashes()
    assert record['input_hashes'] == pins, 'input drift'
def finish(title):
    record['final_hashes'] = hashes()
    if record['final_hashes'] != pins:
        record['accepted'] = False
        record['stop'] = 'input drift'
    raw_path.write_text(json.dumps(record,indent=2)+'\n')
    normalized = json.dumps(record,indent=2).replace(str(Path.cwd()),'<repo>').replace(str(Path.home()),'<home>')
    evidence_path.write_text('# '+title+'\n\nActual bounded execution; inherited release blockers remain.\n\n```json\n'+normalized+'\n```\n')
    emit('ACCEPTED='+str(record['accepted']))


import tomllib
pins={'wallet-broker/Cargo.toml': '435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
raw_path=Path('wallet-broker/target/wal015-live-lock-edge-01.json')
evidence_path=Path('docs/testing/BBD-WAL-015-LIVE-LOCK-EDGE-01.md')
try:
    metadata()
    lock=Path('wallet-broker/Cargo.lock')
    before=tomllib.loads(lock.read_text())['package']
    record['lock_before']=hashlib.sha256(lock.read_bytes()).hexdigest()
    required([str(Path.home()/'.cargo/bin/rustup'),'run','1.98.0','cargo','fetch','--manifest-path','wallet-broker/Cargo.toml','--offline'],120)
    after=tomllib.loads(lock.read_text())['package']
    record['lock_after']=hashlib.sha256(lock.read_bytes()).hexdigest()
    def graph(packages):
        return sorted((p['name'],p['version'],p.get('source'),p.get('checksum'),tuple(p.get('dependencies',[]))) for p in packages if p['name']!='bitbook-wallet-broker')
    assert graph(before)==graph(after), 'transitive graph changed'
    old=next(p for p in before if p['name']=='bitbook-wallet-broker')['dependencies']
    new=next(p for p in after if p['name']=='bitbook-wallet-broker')['dependencies']
    record['root_edges_added']=sorted(set(new)-set(old))
    record['root_edges_removed']=sorted(set(old)-set(new))
    assert record['root_edges_added']==['incrementalmerkletree'] and not record['root_edges_removed']
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally: finish('WAL-015 existing dependency root edge resolution')
sys.exit(0 if record['accepted'] else 1)
