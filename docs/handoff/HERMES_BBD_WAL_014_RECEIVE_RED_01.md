# WAL-014 receive expected red

Reviewer reviewed Sol source actor95037, session01a08cff-bb38-7ce1-846e-2d081897b596,
exit0, four service tests and three native UI tests. Source reads/hash reporting only;
no production/test execution by source actor. Parent69523ec41 plus this governance.
Hermes may execute ONLY the embedded driver once through terminal, exact repository
workdir, background=true and notify_on_complete=true. Wait with process.wait <=60s
until actual EXIT. No execute_code, discovery, preflight, source edits, Git integration
or extra commands. Driver collects actual model/provider/version and input hashes.

Compile-only expected missing API red; do not bypass compile failure to run test bodies.
Reviewer requires a small cleanup refinement before GREEN: both new unlink_zec_account
helpers must inspect the network directory itself before constructing descendants, so
an unexpected network symlink cannot be traversed. This does not affect compile red.
Source stays frozen. Target already verified disk-backed ext4; user profiles untouched.

Exact launcher:
```text
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_014_RECEIVE_RED_01.md").read_text(); code=s.split("```python\n# WAL014_RED_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL014_RED_DRIVER","exec"))'
```

```python
# WAL014_RED_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
record = {'commands': [], 'accepted': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def emit(value):
    try: print(value, flush=True)
    except BrokenPipeError: pass
def command(argv, timeout=60, env=None):
    try:
        result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout, env=env)
        row = {'argv': argv, 'exit': result.returncode, 'output': result.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
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


pins={'wallet-broker/tests/account_management.rs': '84ac590cb389aafb3e21b381c890aca9b0f434811c0398e35d515bc8413f8718', 'wallet-broker/tests/account_native_ui.rs': '783fe38024cc59c83304396eb9764f57540b5661b7622ece7712e61346e63ab5', 'wallet-broker/src/accounts.rs': 'f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/zec.rs': '045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b', 'wallet-broker/src/account_ui.rs': 'f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/zec/store.rs': '531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/scan.rs': '54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
raw_path=Path('wallet-broker/target/wal014-receive-red-01.json')
evidence_path=Path('docs/testing/BBD-WAL-014-RECEIVE-RED-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing result; no replay')
try:
    metadata()
    cargo=['/home/lars/.cargo/bin/rustup','run','1.98.0','cargo','test','--manifest-path','wallet-broker/Cargo.toml','--locked','--offline','--no-default-features']
    row=command(cargo+['--test','account_management','wal014_'],180)
    assert row['exit']==101 and 'fresh_receiver' in row['output'], 'expected missing receive API red absent'
    row=command(cargo+['--features','native-ui','--test','account_native_ui','wal014_'],180)
    assert row['exit']==101 and 'receive' in row['output'], 'expected native receive API red absent'
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally: finish('WAL-014 receive focused expected red')
sys.exit(0 if record['accepted'] else 1)

```
