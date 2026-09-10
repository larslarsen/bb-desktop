# WAL-013 JavaScript green — authorized

Reviewer source review accepts Sol72948 four-path runtime/menu production for bounded
execution; actor collectedexit0. Main menu closedcallback/quitgating and independent
7key own-data filters reviewed; shutdown cancels beforeEOF, observesclose, fallback
and1000/250/1500timers. Runtime needs combined Rust/UI execution later, not claimedgreen.
Hermes only nous/poolside/laguna-s-2.1:free. Execute embedded driver once. Exact9Node
suites, sourceinputs pinned. UIactor41323 may continue disjoint account_ui.rs/lib.
No source edits, formatter, Rust build, additional investigation, Git or otheractors.
Only read this handoff then terminal launcher (no preflight commands), collectactual
exit via process.wait<=60 if background. Stopfirstfailure, no reruns. Driver ownsraw
and normalizedevidence; no extra records or unauthorized evidence modifications.

```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_RUNTIME_JS_GREEN_01.md").read_text(); code=s.split("```python\n# WAL013_RUNTIME_JS_GREEN_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_RUNTIME_JS_GREEN_01","exec"))'
```

```python
# WAL013_RUNTIME_JS_GREEN_01
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

pins = {'social-main.js': '29fa41d88cb75d96b00bc2fe740d8701c9ddfbaf4ba2ae71be84b8755e192d30', 'wallet-broker/supervisor.js': '1258444add1741dd6ba850c241871b8e68be71e8550dac14b0e8af6959b9f7a2', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'wallet-broker/src/runtime.rs': 'da1d1484a0e89bf824ba4548e5abf4319178c36a787dc3c6b9470345d834447d', 'test/walletAccountManagement.node.js': '94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea', 'test/walletStartup.node.js': '4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/walletSupervisorTransport.node.js': 'e69fdcfa4aaeeca03b4abf8ac100888ff0b42fbb6001f53e461a35a5b7550700', 'test/walletSupervisor.node.js': 'eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
raw_path=Path('wallet-broker/target/wal013-runtime-js-green-01.json')
evidence_path=Path('docs/testing/BBD-WAL-013-RUNTIME-JS-GREEN-01.md')
try:
    metadata()
    for name in ['walletAccountManagement','walletStartup','walletBrokerBuild','walletSupervisorShutdown','walletSupervisorTransport','walletSupervisor','electronSecurity','walletPreload','walletBrokerLaunchConfig']:
        row=command(['node','test/'+name+'.node.js'],90)
        assert row['exit']==0, 'unexpected failure '+name
        assert 'not ok' not in row['output'] and 'ok ' in row['output'], 'missing group proof '+name
    record['accepted']=True
except Exception as exc:
    record['stop']=str(exc)
finally:
    finish('WAL-013 JavaScript account menu and lifecycle green')
sys.exit(0 if record['accepted'] else 1)

```
