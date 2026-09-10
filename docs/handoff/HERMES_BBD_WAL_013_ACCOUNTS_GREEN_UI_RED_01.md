# WAL-013 service green + window expected-red execution

Reviewer accepts source6100b5ae (706lines) and lib96bf49de: missingtrait corrected,
RAII seed beforeentropy, encryptedexport validation, corruptedunlockLOCKED. Allstore/
crypto/session primitives retained, rootadvisoryguard helduntilsessionlock onDrop.
Reviewed service testddb29248 (12), UItestd79859fd (9) accepted. Sol56400 and22459
collected exit0. Source actors cannot modify these paths during execution.
Only JS runtime test correction remains independently active, no input overlap.
Hermes nous / poolside/laguna-s-2.1:free execution/evidence ONLY, no integration.

Authorized exact mechanical formatting of THREE listed Rust files via Rust1.98rustfmt
is execution, not manual source/test design; record fullbefore/afterdiff and hashes.
Reviewer accepts semantic source above and will verify formatter-onlydiff on collection.
Then window red (E0432 missingaccount_ui, E0223 allowed only related unresolved trait
cascade), all84 service/primitive tests, two exact falsifications and restoration.
No manual edits, testdesign, other formatterpaths, retries/repairs, scans/Git/otheractors.
Falsification writes are ONLY the pinned one-site mutations and finally restoration
embedded below. Stop first unexpected result; preserve raw/evidence and restoredsource.
Existing ext4 target reused. No tools/dependencies installed or cachesrelocated.
Read this handoff only. Run launcher once with terminal explicit absolute repo workdir,
background=true notify_on_complete=true, then process.wait <=60seconds until EXIT.
Do not return afterlaunch. Driver handles closedstdout but actor must collectactualexit.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_ACCOUNTS_GREEN_UI_RED_01.md").read_text(); code=s.split("```python\n# WAL013_ACCOUNTS_GREEN_UI_RED_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_ACCOUNTS_GREEN_UI_RED_01","exec"))'
```

```python
# WAL013_ACCOUNTS_GREEN_UI_RED_01
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
import difflib, re
raw_path = Path('wallet-broker/target/wal013-accounts-green-ui-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md')
pins = {'wallet-broker/src/accounts.rs': '6100b5ae6dc9836f661b7da488a0c925bc0ed2dac60556e6858acf2a8e777bbd', 'wallet-broker/tests/account_management.rs': 'ddb292489e73b4f25d72133d98fdf204163f13254f5fa44a3a297bffbc3b6fae', 'wallet-broker/tests/account_native_ui.rs': 'd79859fdd0a8a1fd2707304688dcd325648d5d33a02ab0a6271af6b8c68ee919', 'wallet-broker/src/lib.rs': '96bf49de102f16c026fb3d098cb3b447a269dc5f9176fd29c902ac55d44e7246', 'wallet-broker/Cargo.toml': 'c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
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

rustup = str(Path.home()/'.cargo/bin/rustup')
cargo = [rustup,'run','1.98.0','cargo']
common = ['--manifest-path','wallet-broker/Cargo.toml','--locked','--offline','--no-default-features']
source_path = Path('wallet-broker/src/accounts.rs')
def test_result(row, count, failed=False):
    assert row['exit'] == (101 if failed else 0), 'unexpected test exit'
    assert 'error[E' not in row['output'], 'compile failure instead of behavior'
    expected = 'test result: FAILED. 0 passed; 1 failed;' if failed else 'test result: ok. '+str(count)+' passed; 0 failed;'
    assert expected in row['output'], 'unexpected test count/result'
def falsify(label, old, new, test_name):
    original = source_path.read_bytes()
    text = original.decode()
    assert text.count(old) == 1, 'mutation site not unique: '+label
    try:
        source_path.write_text(text.replace(old,new,1))
        row = command(cargo+['test']+common+['--test','account_management',test_name,'--','--exact'],300)
        test_result(row,1,True)
        record[label+'_detected'] = True
    finally:
        source_path.write_bytes(original)
        record[label+'_restored'] = hashlib.sha256(source_path.read_bytes()).hexdigest() == pins[str(source_path)]
    assert record[label+'_restored'], 'source restoration failed'
    row = command(cargo+['test']+common+['--test','account_management',test_name,'--','--exact'],300)
    test_result(row,1)
try:
    metadata()
    assert not Path('wallet-broker/src/account_ui.rs').exists(), 'UI production unexpectedly present'
    format_paths = ['wallet-broker/src/accounts.rs','wallet-broker/tests/account_management.rs','wallet-broker/tests/account_native_ui.rs']
    before_format = {name:Path(name).read_text() for name in format_paths}
    required([rustup,'run','1.98.0','rustfmt','--edition','2024']+format_paths,60)
    record['format_diffs'] = {name:''.join(difflib.unified_diff(before_format[name].splitlines(True),Path(name).read_text().splitlines(True),fromfile=name+' before',tofile=name+' formatted')) for name in format_paths}
    after = hashes()
    assert all(after[name] == pins[name] for name in pins if name not in format_paths), 'formatter touched frozen input'
    pins.update({name:after[name] for name in format_paths})
    record['formatted_input_hashes'] = dict(pins)
    row = command(cargo+['test']+common+['--features','native-ui','--test','account_native_ui'],600)
    assert row['exit'] == 101 and 'error[E0432]' in row['output'] and 'bitbook_wallet_broker::account_ui' in row['output'], 'wrong UI red'
    record['ui_red_error_codes'] = re.findall(r'error\[(E[0-9]+)\]',row['output'])
    assert set(record['ui_red_error_codes']) <= {'E0432','E0223'}, 'UI draft has additional compiler issue'
    record['ui_expected_red'] = True
    targets = ['account_management','vault_crypto','vault_format','vault_store','vault_session','native_surface']
    argv = cargo+['test']+common
    for target in targets: argv += ['--test',target]
    row = command(argv,900)
    assert row['exit'] == 0, 'service/regression green failed'
    counts = [int(n) for n in re.findall(r'test result: ok\. ([0-9]+) passed; 0 failed;',row['output'])]
    assert len(counts) == 6 and sum(counts) == 84, 'wrong green counts'
    record['distinct_green_tests'] = 84
    falsify('native_origin', 'if origin == ActionOrigin::NativeSurface {', 'if true {', 'nonnative_origins_refuse_privileged_apis_before_side_effects')
    old = '        self.sessions\n            .check_deadlines()\n            .map_err(|_| AccountError::unavailable())?;'
    falsify('list_deadline',old,'        let _ = &self.sessions;','list_does_not_extend_idle_deadline_and_backwards_clock_locks_all')
    assert not Path('wallet-broker/src/account_ui.rs').exists(), 'UI production unexpectedly appeared'
    record['accepted'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    finish('WAL-013 account service green and native-window expected red')
sys.exit(0 if record['accepted'] else 1)
```
