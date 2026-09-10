# WAL-013 actual native runtime/build/window green — authorized

Source UI03044aca/runtime67889145/libe37713c4 formatted diff reviewed and accepted.
Widget run6pass3fail traced to testhelper center-distance selecting narrow submit
button instead of fullwidthinput; Sol7820 corrects only account_native_ui.rs. That
one testsource unpinned here, never compiled by commands below. Production FROZEN.
No sourceactor writing production; allJS92groups+menu/EOFfalsifications accepted.

Hermes nous/poolside/laguna-s-2.1:free only. Build/stage actual native-feature broker
with fixed builder, record actualbytes/pin/inventory/mode. Execute actualRustruntime10,
actualappstartup smoke1 (explicitheadless), privateXvfb actualwindow lifecycle1.
No packages/installers/dependencies/network. Existingext4target reused, no newlarge
/tmp artifacts. Tests create only owned0700root and verifychildclose beforecleanup.
No sourceedits/formatters/falsification inthisrun. No Git/extraevidence/investigation/
preflight/delegation. Stopfirstunexpected, preservecompletedsteps. Actualwindow later
falsification authorized separately after its firstgreen. Readonlythishandoff, run
exactlauncher explicitrepoworkdir backgroundtrue, notifycompletion, wait<=60 toEXIT.

```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_NATIVE_RUNTIME_GREEN_01.md").read_text(); code=s.split("```python\n# WAL013_NATIVE_RUNTIME_GREEN_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_NATIVE_RUNTIME_GREEN_01","exec"))'
```

```python
# WAL013_NATIVE_RUNTIME_GREEN_01
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

pins = {'social-main.js': '29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30', 'wallet-broker/supervisor.js': '1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'test/walletAccountManagement.node.js': '94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea', 'test/walletStartup.node.js': '4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/accounts.rs': 'f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1', 'wallet-broker/tests/account_management.rs': '7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/Cargo.toml': 'c4e5700db87b30cd2da488fcc68c7305f8b59e2dd1b16f10da2fd6e59ab28cda', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/account_ui.rs': '03044acaea68d6ddbc2dbe91039e7ffd425c110c29aaa5f88011578a653cd92f', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'test/walletBrokerRuntime.node.js': '989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': 'e4bc03f16d5c545c73298c02242011d70af1051a90c9dea29be1305aa9b381bd', 'test/fixtures/wallet-broker/x11-window.py': '040d8214487216e6b91ec514c9473e95cd66811c26306792f5ca7b081696b5d4'}
raw_path=Path('wallet-broker/target/wal013-native-runtime-green-01.json')
evidence_path=Path('docs/testing/BBD-WAL-013-NATIVE-RUNTIME-GREEN-01.md')
try:
    metadata()
    env=dict(os.environ)
    env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','/usr/bin:/bin')
    required(['node','scripts/build-wallet-broker.js'],300,env)
    binary=Path('wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker')
    manifest=binary.parent/'manifest.json'
    info=binary.lstat()
    assert binary.is_file() and not binary.is_symlink() and info.st_mode & 0o777==0o755
    record['artifact']={'path':str(binary),'sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'size':info.st_size,'mode':oct(info.st_mode & 0o777)}
    record['staged_inventory']={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in binary.parent.iterdir() if p.is_file() and not p.is_symlink()}
    for name in ['walletBrokerRuntime','walletStartupSmoke']:
        row=command(['node','test/'+name+'.node.js'],150)
        assert row['exit']==0 and 'not ok' not in row['output'], 'real broker failure '+name
        assert 'ok ' in row['output'], 'missing real broker proof'
    row=command(['xvfb-run','-a','-s','-screen 0 1024x768x24','node','test/walletAccountWindowSmoke.node.js'],90)
    assert row['exit']==0 and 'tests passed (1)' in row['output'], 'actual native window not green'
    record['accepted']=True
except Exception as exc:
    record['stop']=str(exc)
finally:
    finish('WAL-013 actual native broker and window lifecycle green')
sys.exit(0 if record['accepted'] else 1)

```
