# WAL-013 service expected-red execution

Reviewer accepts corrected12-test service source ddb29248 (1508lines), Cargo target
only c4e5700d. Grok6481c6a3 outer65845 collected exit0. Fixed helper compile errors,
no extra Debug contract, explicit cleanup proof, directory exclusion, restorecommit
cap recheck, post-seed entropy erasure. Production remains absent. Hermes may execute
ONLY launcher below once, record raw/evidence. No source edits, formatting, Git,
retries, repairs, unrelated commands or other actors. Read only this handoff.
Hermes nous / poolside/laguna-s-2.1:free, actualmetadata recorded. Explicit terminal
workdir absolute repo, background=false timeout60. If process ID returned, wait
<=60second increments until completion; never return merely after launch.
Independent JS/UI source actors write their named test paths only; no input overlap.
Existing target verified ext4; no new tools/dependencies/large temporary paths.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_ACCOUNTS_RED_01.md").read_text(); code=s.split("```python\n# WAL013_ACCOUNTS_RED_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_ACCOUNTS_RED_01","exec"))'
```

```python
# WAL013_ACCOUNTS_RED_01
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal013-accounts-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md')
pins = {'wallet-broker/tests/account_management.rs': 'ddb292489e73b4f25d72133d98fdf204163f13254f5fa44a3a297bffbc3b6fae', 'wallet-broker/Cargo.toml': 'c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/lib.rs': '08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
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

try:
    metadata()
    assert not Path('wallet-broker/src/accounts.rs').exists(), 'unexpected production accounts'
    rustup = str(Path.home()/'.cargo/bin/rustup')
    row = command([rustup,'run','1.98.0','cargo','test','--manifest-path','wallet-broker/Cargo.toml','--locked','--offline','--no-default-features','--test','account_management'],300)
    assert row['exit'] == 101, 'expected Rust compile red'
    assert 'error[E0432]' in row['output'] and 'bitbook_wallet_broker::accounts' in row['output'], 'wrong red cause'
    import re
    record['error_codes'] = re.findall(r'error\[(E[0-9]+)\]',row['output'])
    assert set(record['error_codes']) == {'E0432'}, 'additional compile issue requires review'
    assert not Path('wallet-broker/src/accounts.rs').exists(), 'unexpected production accounts'
    record['test_source_count'] = 12
    record['accepted'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    finish('WAL-013 account service expected red')
sys.exit(0 if record['accepted'] else 1)
```
