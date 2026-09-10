# WAL-013 native empty-window correction — reviewed integration authorized

Reviewer accepts green02 actual execution and hashes after Hermes33706/session20260910_125704_54c532 exited0; only exact launcher/wait plus read-only result file. Source actor30964/session01a08cdf-eb69-73c1-b871-5a07bc40870a also exited0 after four authorized edits. All actors closed. No further source authoring authorized.

Actual host XWayland and Xvfb both passed native first open/render/hide/reopen/render/normal EOF with no forced signal and owned child/stdio cleanup. Reviewer inspected both host PNGs, hash 070924bb3e51e5cc75dcd93c5866057751475b9e3353c1e6d53fc0355727ae25: readable BitBook accounts heading, testnet/payment notices, Create account and Restore backup, disabled Unlock/Lock/Export for empty store. Screenshot is of actual owned native client on owner's desktop, not a mock. 27 other focused JS groups passed. Fixed renderer setting falsification detected/restored/7green; prior native blank-UI falsification remains valid for unchanged Rust. Native binary and manifest exact original hashes. Directory Gitleaks no leaks and six inherited policy failures unchanged. Intermediate failed diagnostics are retained, not counted green.

Fix: both JS env allowlists omit WAYLAND_DISPLAY so broker uses X11/XWayland; supervisor uses fixed Mesa software GLX on Linux with nonempty DISPLAY, independent of ambient driver settings. Native window tests verify painted pixels and scope X11 title matching to owned broker PID, excluding GNOME frames. Linux native window requires XWayland plus system Mesa software GLX. No Electron launch flag, driver/package, vault, or other Rust changes. No user profile touched; existing application must be restarted to load changed JS.

Hermes nous/meituan/longcat-2.0:free exact integration driver only: six source/test paths plus six enumerated correction evidence files, exact stage/commit, committed-blob hash verification, pinned committed-content Gitleaks, origin/master push, exact integration evidence commit and push. Preserve unrelated four npm/policy edits and two historical drafts. No broad stage, source edits, additional tests/commands, other actors or output replay. Git pushes explicitly authorized.

TERMINAL ONLY exact launcher once, explicit workdir/background true/notify_on_complete, process.wait <=60 until EXIT. Never execute_code or preflight. Existing disk-backed target.

```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_BLANK_WINDOW_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL013_BLANK_INTEGRATION01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_BLANK_INTEGRATION01","exec"))'
```

```python
# WAL013_BLANK_INTEGRATION01
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


pins={'social-main.js': 'd959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a', 'wallet-broker/supervisor.js': '115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'test/walletAccountManagement.node.js': '6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475', 'test/walletStartup.node.js': '448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/accounts.rs': 'f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1', 'wallet-broker/tests/account_management.rs': '7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8', 'wallet-broker/tests/account_native_ui.rs': '95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/account_ui.rs': 'f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'test/walletBrokerRuntime.node.js': '989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': '3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a', 'test/fixtures/wallet-broker/x11-window.py': 'd192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3', 'wallet-broker/target/wal011-runtime-package-red-01.json': '5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd', 'docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md': 'cc484ec6e0db26767685d5a3e27ced16809a971b024d0192c8ac4c67fe0f55c4', 'docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md': '4348ede94df977d023cbcff96265dc9072c49366c75e9fb9c7b024818e34e2b3', 'docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md': '2ff17248b8f57e00325b75ae0ef5d7d0bd89a9fe4ba80a3acf0828285647934b', 'docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md': 'd288a0d090cd80c6c448dc52c4ac58cb4a7b7d33312bc979150b8e37b6b017c7', 'docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md': '3f48415d5b4ed1e4326e6ccbeb1776022b628b9b8d471f42725c9972c14a1bbc', 'docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md': 'c7b45b958df6a01ed158feb2cc6366dbda2055a53aca077ec6be3811ee87e00c', 'wallet-broker/target/wal013-blank-window-green-02.json': 'eb63c8e3fbd69b72796c88fb6cf257e863f8819925ff7002a92303b8cd10ead0'}
integration_paths=['social-main.js', 'wallet-broker/supervisor.js', 'test/walletAccountManagement.node.js', 'test/walletStartup.node.js', 'test/walletAccountWindowSmoke.node.js', 'test/fixtures/wallet-broker/x11-window.py', 'docs/testing/BBD-WAL-013-BLANK-WINDOW-RED-01.md', 'docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-01.md', 'docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md', 'docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-01.md', 'docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md', 'docs/testing/BBD-WAL-013-HOST-SOFTWARE-DIAGNOSTIC-01.md']
# Integration pins and paths supplied by reviewer after final acceptance.
raw_path=Path('wallet-broker/target/wal013-blank-window-integration-01.json')
evidence_path=Path('docs/testing/BBD-WAL-013-BLANK-WINDOW-INTEGRATION-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing artifacts; no replay')
try:
    metadata()
    assert required(['git','branch','--show-current'])=='master'
    assert required(['git','diff','--cached','--name-only'])==''
    assert required(['git','remote','get-url','origin'])=='https://github.com/larslarsen/bb-desktop.git'
    final=json.loads(Path('wallet-broker/target/wal013-blank-window-green-02.json').read_text())
    assert final['accepted'] and final['input_hashes']==final['final_hashes']
    required(['git','diff','--check'])
    required(['git','add','--']+integration_paths)
    assert set(required(['git','diff','--cached','--name-only']).splitlines())==set(integration_paths)
    required(['git','diff','--cached','--check'])
    required(['git','commit','-m','Fix native wallet windows on Wayland and NVIDIA desktops'])
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
    finish('WAL-013 blank-window correction integration and committed secret scan')
    required(['git','add','--',str(evidence_path)])
    assert required(['git','diff','--cached','--name-only'])==str(evidence_path)
    required(['git','commit','-m','Record native wallet rendering fix integration'])
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
