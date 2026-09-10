# WAL-014 accepted receive source integration

Reviewer accepts the exact six source/test files and four execution records below.
Inspected all production changes and both host screenshot artifacts: real Receive
button is visible in the isolated empty account window, initial and reopened frames
match c05a2423a15eee8593a40a0e7d48fdaf18c6d6ec9c51c23b42a0da6c7ab014d0.
The real-manager native pointer test proves account creation/unlock/Receive/Copy,
independent address decoding/derivation and persisted issuance. Verified 82 Rust tests,
28 distinct JS groups (native smoke repeated on virtual+actual display), production
Clippy, origin/UFVK falsifications and exact restoration, fixed build-runner PATH
correction, manifest pin and Gitleaks directory result. Source/evidence pins match.

GREEN01 full test-target Clippy failed on seven byte-identical inherited helper warnings;
GREEN02 passed established production Clippy and falsifications then lacked rustup on
runner PATH; GREEN03 proved ENOENT, corrected PATH for build command only and completed
remaining checks. Preserve those actual records; do not describe failed commands as
passing. Six inherited policy failures and seven inherited test-helper lint warnings
remain. No installer/mainnet/live-sync/payment claim; balance explicitly unsynced.

Final staged binary SHA256 5ed35fe09b1172b2639faa6170621fca60fe99825636398b78fe1bd382509ae4,
496341880 bytes. Reviewer independently verified current hashes, actual Hermes session
metadata and tool calls. GREEN02 added one read-only evidence-file read after execution;
no source or extra acceptance-command deviation. All source/execution actors exited.

Hermes may execute ONLY embedded launcher once through terminal with explicit repo
workdir, background=true notify_on_complete=true; process.wait<=60 until actual EXIT.
No execute_code or extra commands. Driver checks empty index/master/origin, stages ONLY
10 enumerated accepted source/test/evidence paths, commits, verifies committed blobs,
runs pinned committed-content Gitleaks, pushes and verifies remote ref. It then writes,
commits and pushes ONLY its integration evidence file. No unrelated pending npm/policy
or historical draft paths. No new tests, formatting, source edits, user app/profile or
package work. Stop on drift/failure. Reviewer owns final ticket/handoff closure.

Exact launcher:
```text
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_014_RECEIVE_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL014_INTEGRATION_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL014_INTEGRATION_DRIVER","exec"))'
```

```python
# WAL014_INTEGRATION_DRIVER
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


pins={'wallet-broker/tests/account_management.rs': '817fa7972be4aebec3799a83ac998e572dd5b6f54c93683c5f16077069382e77', 'wallet-broker/tests/account_native_ui.rs': 'eaa1dbb0a9db410c41be7daaac6663dc07d369a6a0769b6460ecf7983382d560', 'wallet-broker/src/accounts.rs': '5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1', 'wallet-broker/src/session.rs': '9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af', 'wallet-broker/src/zec.rs': '2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e', 'wallet-broker/src/account_ui.rs': 'fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/zec/store.rs': '531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/scan.rs': '54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'wallet-broker/supervisor.js': '115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721', 'social-main.js': 'd959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'test/walletAccountManagement.node.js': '6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475', 'test/walletStartup.node.js': '448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': '3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a', 'test/fixtures/wallet-broker/x11-window.py': 'd192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3', 'wallet-broker/tests/vault_session.rs': '67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b', 'wallet-broker/tests/zec_address.rs': '2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3', 'wallet-broker/tests/zec_store.rs': '1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225', 'wallet-broker/tests/zec_hygiene.rs': 'aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6', 'wallet-broker/tests/native_surface.rs': '349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d', 'wallet-broker/target/wal014-receive-green-03.json': '45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed', 'wallet-broker/target/wal014-receive-green-01.json': 'f947d626af653d30c7381ccc2cc8ec241b62ea5c1b037b277cf701c1af853212', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md': 'c091d139092d68b8c3c28f74605dced9fca1b86256cff6f77e7dc8bbd9112dc9', 'wallet-broker/target/wal014-receive-green-02.json': '1375570d23b150eed60ca5177f4313eea9056ac30f26862fc6885d37e5b1aedb', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md': 'a6dd88997f3839f3d8c39f99f7b044f05a9107b9aed9f7409c0de729a58f4402', 'docs/testing/BBD-WAL-014-RECEIVE-RED-01.md': 'ebe38f5d7e8a1efbf3d3bd408a425858ecf80e15f53ecda5eb9a705d21b0ce86', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-03.md': 'f21fd93c3d32354f528f928ab031348c0366ef79b521f1babe087c02d0b2c0c3', 'wallet-broker/target/wal014-receive-green-03.json': '2caff230f91fe6c2fe03240c19e49ccdde4b1a604bb95cc68a867229f72001eb'}
integration_paths=['wallet-broker/src/accounts.rs', 'wallet-broker/src/session.rs', 'wallet-broker/src/zec.rs', 'wallet-broker/src/account_ui.rs', 'wallet-broker/tests/account_management.rs', 'wallet-broker/tests/account_native_ui.rs', 'docs/testing/BBD-WAL-014-RECEIVE-RED-01.md', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-01.md', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-02.md', 'docs/testing/BBD-WAL-014-RECEIVE-GREEN-03.md']
# Integration pins and paths supplied by reviewer after final acceptance.
raw_path=Path('wallet-broker/target/wal014-receive-integration-01.json')
evidence_path=Path('docs/testing/BBD-WAL-014-RECEIVE-INTEGRATION-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing artifacts; no replay')
try:
    metadata()
    assert required(['git','branch','--show-current'])=='master'
    assert required(['git','diff','--cached','--name-only'])==''
    assert required(['git','remote','get-url','origin'])=='https://github.com/larslarsen/bb-desktop.git'
    final=json.loads(Path('wallet-broker/target/wal014-receive-green-03.json').read_text())
    assert final['accepted'] and final['input_hashes']==final['final_hashes']
    required(['git','diff','--check'])
    required(['git','add','--']+integration_paths)
    assert set(required(['git','diff','--cached','--name-only']).splitlines())==set(integration_paths)
    required(['git','diff','--cached','--check'])
    required(['git','commit','-m','Add native Zcash receive addresses and unsynced balance status'])
    record['source_commit']=required(['git','rev-parse','HEAD'])
    assert set(required(['git','diff-tree','--no-commit-id','--name-only','-r','HEAD']).splitlines())==set(integration_paths)
    for name in integration_paths:
        content=subprocess.run(['git','show','HEAD:'+name],check=True,stdout=subprocess.PIPE).stdout
        assert hashlib.sha256(content).hexdigest()==pins[name], 'committed source mismatch: '+name
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert hashlib.sha256(Path(scanner).read_bytes()).hexdigest()=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'
    required([scanner,'git','--redact=100','--no-banner','.'],180)
    required(['git','push','origin','master'],120)
    remote=required(['git','ls-remote','origin','refs/heads/master'],120).split()[0]
    assert remote==record['source_commit']
    record['source_pushed']=True
    record['accepted']=True
    finish('WAL-014 exact source integration and committed secret scan')
    required(['git','add','--',str(evidence_path)])
    assert required(['git','diff','--cached','--name-only'])==str(evidence_path)
    required(['git','commit','-m','Record native receive integration evidence'])
    record['evidence_commit']=required(['git','rev-parse','HEAD'])
    required(['git','push','origin','master'],120)
    assert required(['git','ls-remote','origin','refs/heads/master'],120).split()[0]==record['evidence_commit']
    record['final_status']=required(['git','status','--short'])
    assert hashes()==pins, 'final input drift'
except Exception as exc:
    record['accepted']=False;record['stop']=str(exc)
    if not evidence_path.exists(): finish('WAL-014 integration stopped')
finally:
    raw_path.write_text(json.dumps(record,indent=2)+'\n')
    emit(json.dumps({'accepted':record['accepted'],'stop':record.get('stop'),'source_commit':record.get('source_commit'),'evidence_commit':record.get('evidence_commit')}))
sys.exit(0 if record['accepted'] else 1)

```
