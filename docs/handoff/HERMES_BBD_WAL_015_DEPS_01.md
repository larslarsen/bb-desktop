# WAL-015 dependency resolution only

Reviewer inspected the exact manifest candidate: unchanged prior pins plus backend
lightwalletd-tonic and tonic/tokio/prost as reviewed. Authorize cargo fetch/lockfile
resolution and the exact feature-tree reads embedded below. Network fetching of
crates/index is authorized. No tests/builds/formatters/audits or other source edits.
Source actor is concurrently writing the listed production paths; do not inspect or
modify those incomplete files. Only Cargo.lock may change through the listed Cargo
command. Manifest and unrelated npm/policy edits are pinned and must stay unchanged.
Hermes owns docs/testing/BBD-WAL-015-LIVE-DEPS-01.md and ignored raw target evidence.
Disk-backed ext4 target verified. No user data or application process access.

Run exact launcher once through terminal, explicit repository workdir,
background=true notify_on_complete=true, process.wait <=60s until actual EXIT.
No execute_code, extra preflight, retries, alternate commands or Git writes.

```text
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_015_DEPS_01.md").read_text(); code=s.split("```python\n# WAL015_DEPS_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL015_DEPS_DRIVER","exec"))'
```

```python
# WAL015_DEPS_DRIVER
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


pins={'wallet-broker/Cargo.toml': '508d571a3821deb2ae06400ff445be832a3ae1384d0cf4cb7d870f8e16d66b67', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}

raw_path=Path('wallet-broker/target/wal015-live-deps-01.json')
evidence_path=Path('docs/testing/BBD-WAL-015-LIVE-DEPS-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing result; no replay')
try:
    metadata()
    lock=Path('wallet-broker/Cargo.lock')
    record['lock_before']=hashlib.sha256(lock.read_bytes()).hexdigest()
    assert record['lock_before']=='b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'initial lock drift'
    cargo=[str(Path.home()/'.cargo/bin/rustup'),'run','1.98.0','cargo']
    required(cargo+['fetch','--manifest-path','wallet-broker/Cargo.toml'],240)
    record['lock_after']=hashlib.sha256(lock.read_bytes()).hexdigest()
    required(['git','diff','--','wallet-broker/Cargo.lock'])
    for package in ['tonic','tokio','rustls']:
        required(cargo+['tree','--manifest-path','wallet-broker/Cargo.toml','--locked','--offline','--no-default-features','--features','native-ui','-e','features','-i',package],90)
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally:
    record['lock_final']=hashlib.sha256(Path('wallet-broker/Cargo.lock').read_bytes()).hexdigest()
    finish('WAL-015 live sync dependency resolution')
sys.exit(0 if record['accepted'] else 1)

```
