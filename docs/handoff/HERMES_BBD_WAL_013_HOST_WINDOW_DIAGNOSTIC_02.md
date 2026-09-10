# WAL-013 host native initialization error isolation

Diagnostic01 actual trace: new test broker exits1 about1sec after spawn, only fixed broker-closed diagnostic, zero matching native windows. Thus this is native initialization failure, not a blank-content or visibility-timeout correction. No new production design authorized. Isolated Xvfb render and restored blank-fault proof remain valid, host acceptance outstanding.

Hermes nous/meituan/longcat-2.0:free only this exact temporary diagnostic instrumentation of runtime's one ui_result error branch to print the native framework error, build/stage, run unchanged test with prior bounded trace on host. Finally restore runtime bytes and rebuild production regardless of outcome. This logging is NEVER retained in production. Only a fresh empty test profile, no actual account data, no other process logs; raw captured to local target and home paths normalized in evidence. No other source edits, environment changes, dependency work, Git or commands. All inputs pinned; source actors closed. TERMINAL ONLY exact launcher once, explicit workdir/background true/notify_on_complete and process.wait <=60 until EXIT; never execute_code or extras.

```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_HOST_WINDOW_DIAGNOSTIC_02.md").read_text(); code=s.split("```python\n# WAL013_HOST_DIAG02\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_HOST_DIAG02","exec"))'
```

```python
# WAL013_HOST_DIAG02
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


pins={'social-main.js': 'd959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a', 'wallet-broker/supervisor.js': '57811beaebdd588f3328925d9b1d1b1d016100bfe1a57000a4ecccba57d60ee0', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'test/walletAccountManagement.node.js': '307480c4620c704af6f5a6f96afda0bf8941d5ee1a7583f6683dc30be0bb62bb', 'test/walletStartup.node.js': '448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/src/accounts.rs': 'f79eb3286cdccae2058e099c2d08b8e5a7167477a687e36609f91df4dc5b51f1', 'wallet-broker/tests/account_management.rs': '7f876b1a9b7c4e65f8a6225d29867b2ae7dc4902f10cbbb829b4012675dff0d8', 'wallet-broker/tests/account_native_ui.rs': '95584435ad547752481bfb2ebefbb83d734a6a0edbffb63a4ef8ec0681d166d5', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/account_ui.rs': 'f8a47778d00be2cd7a448aec33117172880bde6e70e220b9f783dcca1ad8c1c7', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'test/walletBrokerRuntime.node.js': '989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': '5661f8945b7cebc331e4a2a1feb290d9ab0b2e77f4979a9adcaf2ac6cb3cdc94', 'test/fixtures/wallet-broker/x11-window.py': '6a049e8dbcd10522d6a7c65f46792afac2c980aa33a413041e118a9d964aa25b', 'wallet-broker/target/wal011-runtime-package-red-01.json': '5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd'}
trace="const cp=require('child_process'); const original=cp.spawn; const begin=Date.now(); let seq=0; const log=(label,value)=>process.stdout.write(JSON.stringify({ms:Date.now()-begin,label,value})+'\\n'); cp.spawn=function(file,args,options){ const child=original.apply(this,arguments); const id=++seq; const broker=file.endsWith('/bitbook-wallet-broker'); const inspect=args&&args[args.length-1]==='inspect'; log('spawn',{id,pid:child.pid,broker,inspect}); let bytes=0; if(broker&&child.stderr)child.stderr.on('data',data=>{bytes+=data.length;if(bytes<32768)log('broker_stderr',String(data));}); if(inspect&&child.stdout)child.stdout.on('data',data=>log('windows',String(data))); child.once('close',(code,signal)=>log('close',{id,code,signal})); return child; }; require('./test/walletAccountWindowSmoke.node.js').tests[0].fn().then(()=>log('result','passed'),error=>{log('failure',String(error.stack||error));process.exitCode=1;});"
raw_path=Path('wallet-broker/target/wal013-host-window-diagnostic-02.json')
evidence_path=Path('docs/testing/BBD-WAL-013-HOST-WINDOW-DIAGNOSTIC-02.md')
if raw_path.exists() or evidence_path.exists():raise RuntimeError('existing artifacts')
try:
    metadata()
    env=dict(os.environ);env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','/usr/bin:/bin')
    target=Path('wallet-broker/src/runtime.rs');saved=target.read_bytes()
    anchor='if ui_result.is_err() {'
    assert saved.decode().count(anchor)==1
    try:
        target.write_text(saved.decode().replace(anchor,'if let Err(error) = &ui_result { eprintln!("WAL013_NATIVE_INIT: {error}");',1))
        required(['node','scripts/build-wallet-broker.js'],300,env)
        row=command(['node','-e',trace],90)
        assert row['exit']==1 and 'WAL013_NATIVE_INIT:' in row['output'], 'native init diagnostic not observed'
        record['diagnostic_only']=True
    finally:
        target.write_bytes(saved)
        required(['node','scripts/build-wallet-broker.js'],300,env)
        assert hashlib.sha256(target.read_bytes()).hexdigest()==pins[str(target)]
    record['accepted']=True
except Exception as exc:record['stop']=str(exc)
finally:finish('WAL-013 temporary native initialization diagnostic; restored')
sys.exit(0 if record['accepted'] else 1)

```
