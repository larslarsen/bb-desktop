# WAL-013 reviewed source integration — authorized

Reviewer independently accepts final04 actual execution, all input/final hashes, source restoration, unchanged final staged binary versus final03 successful window run, account12 discovery, exact6 inherited policy failures, no new vulnerability failures or secret findings. Hermes45360/session20260910_114808_10675d collected EXIT0; only read_file, exact terminal launcher, process.wait. Sol86562 source actor closed. All actors closed. No source/test changes remain authorized.

Functional acceptance: 103 distinct Rust tests and 104 JS groups across accepted records; actual native startup/open/hide/reopen/normal EOF and menu/origin/idle/confirmation/EOF/opening suppression fault checks. Failed intermediate evidence is retained with subsequent correction, not counted as green. Current binary 929751c8f956c8e764466279b9adb4ab8007c391206dd6478b18f1794853a11c, 476621128 bytes, 0755; exact manifest pin. No payment capabilities enabled. Existing six policy failures and Rust source inventory are release blockers. Native first blank frame may transiently map; initialized window proven unmapped before manage. No real profile, installer, all-feature/proving, or OS file-picker portal acceptance claim.

Hermes nous/meituan/longcat-2.0:free exact integration only: 19 source/test paths plus the enumerated WAL013 evidence paths below; preserve unrelated npm/policy edits and two historical drafts. Verify pins/index/branch/origin, exact stage and commit, verify committed bytes, pinned committed-content Gitleaks, push origin master, record integration evidence, commit and push that exact evidence path. No broad staging, amendments, resets, source edits, other tests or actors. Git push is authorized by this accepted workflow. No user relay.

TERMINAL TOOL ONLY never execute_code. Exact launcher once, explicit repo cwd, background true, process.wait <=60 until observed EXIT. No preflight or extra commands beyond driver. Stop on unexpected failures; retained raw explains any partial commit/push. Use existing disk-backed target.

```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL013_INTEGRATION01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_INTEGRATION01","exec"))'
```

```python
# WAL013_INTEGRATION01
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


pins={'social-main.js': '29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30', 'wallet-broker/supervisor.js': '1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'test/walletAccountManagement.node.js': '94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea', 'test/walletStartup.node.js': '4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/accounts.rs': 'f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1', 'wallet-broker/tests/account_management.rs': '7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8', 'wallet-broker/tests/account_native_ui.rs': '95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/account_ui.rs': 'f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'test/walletBrokerRuntime.node.js': '989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': 'c6e6ecc4616a28369f890255be2cc79f0d0d39909c561744f139aa6969549cbd', 'test/fixtures/wallet-broker/x11-window.py': '040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4', 'wallet-broker/target/wal011-runtime-package-red-01.json': '5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd', 'docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md': '4b96ed584da895b7c89a491426c4835eef08164c0eed9d14b2fab64f82eec9ad', 'docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md': 'a20778f6ae1ec8750afe1f511a0a28507f09ddd19357ef3e6c686e40a9850f83', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-01.md': 'f2536dfecc8634079d6e06ee295c9ffd35ed19a441f94d6cfeaa18fdd17c857c', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-02.md': 'd1321c3157bfc2bb0e45ac02561edba14d26351e37d76882bcd229ecde5c0f29', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-03.md': '62b52d38de8023f6fc87d5971c9c8ff4bfcf2aaac10d5cc79367b85b2105d663', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-04.md': 'be331c08170262885b7615a4f0904ffe31c4ea1ee8f88c044eb0f0a4b67778dd', 'docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md': 'c4fd7dc99d964387ad2ae06fd20bc742667e6f1fe1763265bced2e001165b8ed', 'docs/testing/BBD-WAL-013-NATIVE-UI-DIAGNOSTIC-01.md': 'f579a2b668915a699fe405fe3fcfa6ee445df540f5ae4eb69f6660e098292ace', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-01.md': 'de5a5330021442773ffcf8e6aa80c735604be8e735849c9e7eebe8de74c50d1b', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-FINISH-01.md': 'fe6023b5c6fc53da2fbea1f25216c72011cf40f40c453cee0ce30c30851497ff', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-01.md': 'f7a82855015f6c2adbe24eecc6e5b80ff8e6214148497e0fcdd9ec6a84286739', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-02.md': '62e09a336c95e76fc939181bcb235c74d8d28e0afb90157866e533b774b96d73', 'docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md': '8896d01cc12b8a305b1a277aea7087398179950b664196d3bb2b854eb849c0f0', 'docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-FINISH-01.md': '1484b3c90cf8536df818cfc5bc8cd3619a19d55558cd522be3a43c4589f7e730', 'docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md': '6d2ac5e50de00d14fc28611568963db53d14af848d819c29c30559bdac7bfed7', 'wallet-broker/target/wal013-final-window-security-04.json': '45c1abe5ce23f32df5ac848c131f3c49ecb1c238f73604f2a704154eec4e20ed'}
integration_paths=['scripts/build-wallet-broker.js', 'social-main.js', 'wallet-broker/src/accounts.rs', 'wallet-broker/src/account_ui.rs', 'wallet-broker/src/lib.rs', 'wallet-broker/src/runtime.rs', 'wallet-broker/supervisor.js', 'test/electronSecurity.node.js', 'test/walletBrokerBuild.node.js', 'test/walletBrokerRuntime.node.js', 'test/walletStartup.node.js', 'test/walletStartupSmoke.node.js', 'test/walletSupervisor.node.js', 'test/walletSupervisorShutdown.node.js', 'test/walletAccountManagement.node.js', 'test/walletAccountWindowSmoke.node.js', 'test/fixtures/wallet-broker/x11-window.py', 'wallet-broker/tests/account_management.rs', 'wallet-broker/tests/account_native_ui.rs', 'docs/testing/BBD-WAL-013-ACCOUNTS-GREEN-UI-RED-01.md', 'docs/testing/BBD-WAL-013-ACCOUNTS-RED-01.md', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-01.md', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-02.md', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-03.md', 'docs/testing/BBD-WAL-013-FINAL-WINDOW-SECURITY-04.md', 'docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md', 'docs/testing/BBD-WAL-013-NATIVE-UI-DIAGNOSTIC-01.md', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-01.md', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-FINISH-01.md', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-01.md', 'docs/testing/BBD-WAL-013-NATIVE-UI-GREEN-RESUME-02.md', 'docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md', 'docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-FINISH-01.md', 'docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md']
# Integration pins and paths supplied by reviewer after final acceptance.
raw_path=Path('wallet-broker/target/wal013-integration-01.json')
evidence_path=Path('docs/testing/BBD-WAL-013-INTEGRATION-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing artifacts; no replay')
try:
    metadata()
    assert required(['git','branch','--show-current'])=='master'
    assert required(['git','diff','--cached','--name-only'])==''
    assert required(['git','remote','get-url','origin'])=='https://github.com/larslarsen/bb-desktop.git'
    final=json.loads(Path('wallet-broker/target/wal013-final-window-security-04.json').read_text())
    assert final['accepted'] and final['input_hashes']==final['final_hashes']
    required(['git','diff','--check'])
    required(['git','add','--']+integration_paths)
    assert set(required(['git','diff','--cached','--name-only']).splitlines())==set(integration_paths)
    required(['git','diff','--cached','--check'])
    required(['git','commit','-m','Add native testnet wallet account management'])
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
    finish('WAL-013 exact source integration and committed secret scan')
    required(['git','add','--',str(evidence_path)])
    assert required(['git','diff','--cached','--name-only'])==str(evidence_path)
    required(['git','commit','-m','Record account management integration evidence'])
    record['evidence_commit']=required(['git','rev-parse','HEAD'])
    required(['git','push','origin','master'],120)
    assert required(['git','ls-remote','origin','refs/heads/master'],120).split()[0]==record['evidence_commit']
    record['final_status']=required(['git','status','--short'])
    assert hashes()==pins, 'final input drift'
except Exception as exc:
    record['accepted']=False;record['stop']=str(exc)
    if not evidence_path.exists(): finish('WAL-013 integration stopped')
finally:
    raw_path.write_text(json.dumps(record,indent=2)+'\n')
    emit(json.dumps({'accepted':record['accepted'],'stop':record.get('stop'),'source_commit':record.get('source_commit'),'evidence_commit':record.get('evidence_commit')}))
sys.exit(0 if record['accepted'] else 1)

```
