# WAL-013 final actual-host render verification

After resumed Sol source actor is collected closed, reviewer accepts only three test updates then supervisor fixed renderer policy: client windows scoped to owned broker PID via _NET_WM_PID; six copied env fields unchanged; Linux nonempty DISPLAY adds two fixed trusted software-renderer constants, never ambient values. Starting/final source pins below, all other inputs frozen. Main retains the previous XWayland selection correction. Actual-host GLXBadFBConfig was already red; software+Mesa probe initialized and opened the client but old title-only test counted GNOME frame too. No host proof credited yet.

Hermes nous/meituan/longcat-2.0:free runs four targeted JS suites, actual rendered Xvfb smoke and actual host XWayland smoke on DISPLAY=:0. Fresh test store only, all helper capture/close targets restricted to child PID; never alter user's application or profile. Verify final original native binary and manifest unchanged, temporarily suppress fixed LIBGL assignment in supervisor to falsify environment assertions, finally restore exact bytes and rerun7groups, exact inherited policy six failures, pinned directory secret scan. Native blank-UI falsification was already accepted with identical widget source; no repeat Rust build/audits/other tests. Reviewer must inspect BOTH host PNGs after green. Linux native wallet now deliberately requires system XWayland and Mesa software GLX; no dependency installation here, no Electron or sandbox flag change.

TERMINAL tool ONLY, exact launcher ONCE with explicit workdir, background true, notify_on_complete; process.wait <=60 until EXIT. No execute_code, preflight, ls, extra commands, source edits beyond exact temporary assertion falsification/restoration, Git, or other actors. Existing ext4 target, no artifact replay.

```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_BLANK_WINDOW_GREEN_02.md").read_text(); code=s.split("```python\n# WAL013_BLANK_GREEN02\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_BLANK_GREEN02","exec"))'
```

```python
# WAL013_BLANK_GREEN02
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


pins={'social-main.js': 'd959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a', 'wallet-broker/supervisor.js': '115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'test/walletAccountManagement.node.js': '6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475', 'test/walletStartup.node.js': '448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/accounts.rs': 'f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1', 'wallet-broker/tests/account_management.rs': '7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8', 'wallet-broker/tests/account_native_ui.rs': '95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/account_ui.rs': 'f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'test/walletBrokerRuntime.node.js': '989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': '3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a', 'test/fixtures/wallet-broker/x11-window.py': 'd192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3', 'wallet-broker/target/wal011-runtime-package-red-01.json': '5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd'}
raw_path=Path('wallet-broker/target/wal013-blank-window-green-02.json')
evidence_path=Path('docs/testing/BBD-WAL-013-BLANK-WINDOW-GREEN-02.md')
if raw_path.exists() or evidence_path.exists():raise RuntimeError('existing results; no replay')
try:
    metadata()
    env=dict(os.environ);env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','/usr/bin:/bin')
    for name in ['walletAccountManagement','walletStartup','walletSupervisor','walletStartupSmoke']:
        required(['node','test/'+name+'.node.js'],90)
    window=['xvfb-run','-a','-s','-screen 0 1024x768x24','node','test/walletAccountWindowSmoke.node.js']
    required(window,90)
    record['isolated_screenshots']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('wallet-broker/target/wal013-account-window-visible.png'),Path('wallet-broker/target/wal013-account-window-reopened.png')]}
    assert os.environ.get('DISPLAY')==':0' and os.environ.get('XAUTHORITY'), 'host XWayland display context missing'
    required(['node','test/walletAccountWindowSmoke.node.js'],90)
    record['host_xwayland_screenshots']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('wallet-broker/target/wal013-account-window-visible.png'),Path('wallet-broker/target/wal013-account-window-reopened.png')]}
    binary=Path('wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker')
    actual=hashlib.sha256(binary.read_bytes()).hexdigest()
    assert actual=='929751c8f956c8e764466279b9adb4ab8007c391206dd6478b18f1794853a11c'
    assert json.loads((binary.parent/'manifest.json').read_text())=={'v':1,'platform':'linux','arch':'x64','sha256':actual}
    target=Path('wallet-broker/supervisor.js');saved=target.read_bytes()
    original="result.LIBGL_ALWAYS_SOFTWARE = '1';"
    assert saved.decode().count(original)==1, 'software assignment anchor drift'
    try:
        target.write_text(saved.decode().replace(original,'',1))
        row=command(['node','test/walletAccountManagement.node.js'],90)
        assert row['exit']==1 and 'LIBGL_ALWAYS_SOFTWARE' in row['output'], 'software constant falsification not detected'
        record['software_constant_falsification_detected']=True
    finally:
        target.write_bytes(saved)
        assert hashlib.sha256(target.read_bytes()).hexdigest()==pins[str(target)]
    required(['node','test/walletAccountManagement.node.js'],90)
    record['binary_sha256']=actual
    required(['git','diff','--check'])
    old=json.loads(Path('wallet-broker/target/wal013-final-window-security-04.json').read_text())
    row=command(['node','test/securityPolicy.node.js'],120)
    failures=[line for line in row['output'].splitlines() if line.startswith('not ok')]
    assert row['exit']==1 and failures==old['policy_current_failures'], 'policy failure ratchet changed'
    record['policy_current_failures']=failures
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert hashlib.sha256(Path(scanner).read_bytes()).hexdigest()=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'
    required([scanner,'dir','--redact=100','--no-banner','.'],180)
    record['accepted']=True
except Exception as exc:record['stop']=str(exc)
finally:finish('WAL-013 blank window fix: rendered native controls and host XWayland')
sys.exit(0 if record['accepted'] else 1)

```
