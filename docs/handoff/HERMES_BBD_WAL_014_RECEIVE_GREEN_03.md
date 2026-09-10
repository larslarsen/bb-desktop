# WAL-014 build runner diagnostic and remaining validation

Hermes75882 exited1, session20260910_134750_2cf32f. Reviewer verified raw GREEN02:
production Clippy clean; native-origin and wrong-UFVK falsifications detected the
intended runtime assertion failures; exact source restoration; seven feature tests
passed again. Source pins unchanged. Build entrypoint returned only its fixed
“Unable to build wallet broker” message, with no compiler output. No later check ran.

Hermes may execute only embedded launcher once through terminal, explicit repo workdir,
background=true notify_on_complete=true and process.wait<=60 until actual EXIT. No
execute_code/preflight/extras/source/Git. Diagnostic runs Node spawnSync rustup --version,
records only result/error code. IF AND ONLY IF ENOENT proves runner PATH lacks rustup,
prepend the already-used trusted <home>/.cargo/bin for the single existing build entrypoint
command, then finish prior remaining JS/native/policy/secret-scan steps. Otherwise stop
for reviewer with actual diagnostic. No source/env-file changes, user app restart or
user profile access. No repeated passing Rust/lint/mutation suites. All source/evidence
pins embedded. Same isolated native test roots and owned-process cleanup as GREEN01.

Exact launcher:
```text
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_014_RECEIVE_GREEN_03.md").read_text(); code=s.split("```python\n# WAL014_GREEN03_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL014_GREEN03_DRIVER","exec"))'
```

```python
# WAL014_GREEN03_DRIVER
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


pins={'wallet-broker/tests/account_management.rs': '817fa7972be4aebec3799a83ac998e572dd5b6f54c93683c5f16077069382e77', 'wallet-broker/tests/account_native_ui.rs': 'eaa1dbb0a9db410c41be7daaac6663dc07d369a6a0769b6460ecf7983382d560', 'wallet-broker/src/accounts.rs': '5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1', 'wallet-broker/src/session.rs': '9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af', 'wallet-broker/src/zec.rs': '2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e', 'wallet-broker/src/account_ui.rs': 'fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/zec/store.rs': '531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/scan.rs': '54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'wallet-broker/supervisor.js': '115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721', 'social-main.js': 'd959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'test/walletAccountManagement.node.js': '6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475', 'test/walletStartup.node.js': '448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': '3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a', 'test/fixtures/wallet-broker/x11-window.py': 'd192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3', 'wallet-broker/tests/vault_session.rs': '67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b', 'wallet-broker/tests/zec_address.rs': '2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3', 'wallet-broker/tests/zec_store.rs': '1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225', 'wallet-broker/tests/zec_hygiene.rs': 'aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6', 'wallet-broker/tests/native_surface.rs': '349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d', 'wallet-broker/target/wal013-final-window-security-04.json': '45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed', 'wallet-broker/target/wal014-receive-green-01.json': 'f947d626af653d30c7381ccc2cc8ec241b62ea5c1b037b277cf701c1af853212', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md': 'c091d139092d68b8c3c28f74605dced9fca1b86256cff6f77e7dc8bbd9112dc9', 'wallet-broker/target/wal014-receive-green-02.json': '1375570d23b150eed60ca5177f4313eea9056ac30f26862fc6885d37e5b1aedb', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md': 'a6dd88997f3839f3d8c39f99f7b044f05a9107b9aed9f7409c0de729a58f4402'}
raw_path=Path('wallet-broker/target/wal014-receive-green-03.json')
evidence_path=Path('docs/testing/BBD-WAL-014-RECEIVE-GREEN-03.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing result; no replay')
try:
    metadata()
    prior=json.loads(Path('wallet-broker/target/wal014-receive-green-02.json').read_text())
    assert prior['input_hashes']==prior['final_hashes']
    assert all(pins[n]==h for n,h in prior['final_hashes'].items())
    assert prior['native_origin_falsification_detected'] and prior['ufvk_binding_falsification_detected']
    assert prior['reused_rust_test_counts']==[16,13,8,8,8,12,17]
    record['reused_rust_test_counts']=prior['reused_rust_test_counts']
    record['reused_production_clippy_and_falsifications']=True
    diagnostic="const {spawnSync}=require('child_process'); const r=spawnSync('rustup',['--version'],{encoding:'utf8'}); console.log(JSON.stringify({status:r.status,error:r.error&&r.error.code,stdout:r.stdout,stderr:r.stderr}));"
    row=command(['node','-e',diagnostic],60)
    assert row['exit']==0
    observed=json.loads(row['output'])
    record['build_runner_rustup_diagnostic']=observed
    assert observed.get('error')=='ENOENT', 'build failure was not missing rustup on runner PATH; stop for review'
    build_env=dict(os.environ)
    build_env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+build_env.get('PATH','')
    record['build_path_correction']='prepend trusted <home>/.cargo/bin for build command only'
    required(['node','scripts/build-wallet-broker.js'],300,env=build_env)
    binary=Path('wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker')
    actual=hashlib.sha256(binary.read_bytes()).hexdigest()
    assert json.loads((binary.parent/'manifest.json').read_text())=={'v':1,'platform':'linux','arch':'x64','sha256':actual}
    record['binary_sha256']=actual
    for name in ['walletAccountManagement','walletStartup','walletSupervisor','walletStartupSmoke']:
        required(['node','test/'+name+'.node.js'],90)
    required(['xvfb-run','-a','-s','-screen 0 1024x768x24','node','test/walletAccountWindowSmoke.node.js'],90)
    record['xvfb_screenshots']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('wallet-broker/target/wal013-account-window-visible.png'),Path('wallet-broker/target/wal013-account-window-reopened.png')]}
    assert os.environ.get('DISPLAY')==':0' and os.environ.get('XAUTHORITY'), 'host display context missing'
    required(['node','test/walletAccountWindowSmoke.node.js'],90)
    record['host_screenshots']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('wallet-broker/target/wal013-account-window-visible.png'),Path('wallet-broker/target/wal013-account-window-reopened.png')]}
    old=json.loads(Path('wallet-broker/target/wal013-final-window-security-04.json').read_text())
    row=command(['node','test/securityPolicy.node.js'],120)
    failures=[line for line in row['output'].splitlines() if line.startswith('not ok')]
    assert row['exit']==1 and failures==old['policy_current_failures'], 'policy failure ratchet changed'
    record['policy_current_failures']=failures
    row=command(['node','scripts/security-policy.js'],60)
    assert row['exit']==1 and 'wallet Rust source inventory is missing or extra' in row['output'], 'policy baseline drift'
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert hashlib.sha256(Path(scanner).read_bytes()).hexdigest()=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'
    required([scanner,'dir','--redact=100','--no-banner','.'],180)
    required(['git','diff','--check'])
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally: finish('WAL-014 receive validation, native rendering and unchanged policy baseline')
sys.exit(0 if record['accepted'] else 1)

```
