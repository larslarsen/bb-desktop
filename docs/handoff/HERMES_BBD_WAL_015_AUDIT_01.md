# WAL-015 pinned dependency audit

Reviewer verified DEPS01: 31 additions and zero existing version changes, client-only
Tonic/ring/WebPKI features. Input pins match. Hermes DEPS01 made three extra read-only
filename searches for its own evidence after execution; no additional execution or
input mutation occurred. This run must not repeat those searches.

Authorize ONLY embedded driver once via terminal in this repository,
background=true notify_on_complete=true, process.wait <=60s until actual EXIT.
It verifies pinned cargo-audit, fetches advisory data, audits the exact lockfile and
records actual metadata/output. Threshold: zero vulnerabilities, only the inherited
atomic-polyfill RUSTSEC-2023-0089 unmaintained warning. Stop on anything else.
No source/lock changes, tests/builds, formatters, Git or user data. Source actors work
concurrently on their own files; only dependency inputs are pinned for this audit.
After EXIT, return driver result and the evidence paths provided here. Do not search
for them with search_files/read_file or execute extra commands; they are already known.

```text
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_015_AUDIT_01.md").read_text(); code=s.split("```python\n# WAL015_AUDIT_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL015_AUDIT_DRIVER","exec"))'
```

```python
# WAL015_AUDIT_DRIVER
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


pins={'wallet-broker/Cargo.toml': '508d571a3821deb2ae06400ff445be832a3ae1384d0cf4cb7d870f8e16d66b67', 'wallet-broker/Cargo.lock': 'b28a8e55de47130e53276669a4d5f3608755cb5390b2b229908090304f67ca29', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}

raw_path=Path('wallet-broker/target/wal015-live-audit-01.json')
evidence_path=Path('docs/testing/BBD-WAL-015-LIVE-AUDIT-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing result; no replay')
try:
    metadata()
    scanner=Path.home()/'.cargo/bin/cargo-audit'
    digest=hashlib.sha256(scanner.read_bytes()).hexdigest()
    assert digest=='4ac4b8a8d3893109351b2cf3b9a37c7483e63915f994ac841a405248f7a4e7fa', 'scanner drift'
    record['scanner_sha256']=digest
    required([str(scanner),'--version'])
    row=command([str(scanner),'audit','--file','wallet-broker/Cargo.lock','--json'],240)
    assert row['exit']==0, 'audit failed'
    report=json.loads(row['output'])
    assert report['vulnerabilities']['count']==0, 'new vulnerability'
    warnings=report.get('warnings',{})
    for kind, entries in warnings.items():
        for item in entries:
            advisory=item.get('advisory',{})
            assert kind=='unmaintained' and advisory.get('id')=='RUSTSEC-2023-0089' and item.get('package',{}).get('name')=='atomic-polyfill', 'unreviewed warning'
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally: finish('WAL-015 live sync pinned Cargo audit')
sys.exit(0 if record['accepted'] else 1)

```
