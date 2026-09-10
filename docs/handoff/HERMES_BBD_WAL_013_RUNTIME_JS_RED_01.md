# WAL-013 runtime/menu expected red — execution authorized

Hermes only, provider nous model poolside/laguna-s-2.1:free. Source reviewed;
Sol runtime correction97914 complete and accepted. Only execute the exact embedded
driver once, record actual session/version, two Node expected-red outputs and hashes.
No source edits, Git, broader checks, other actors or investigation. Native UI source
and independent native-window smoke source may proceed in disjoint unpinned paths.
Use terminal foreground timeout60; if background, process.wait <=60 until actual exit.
Stop on unexpected failure; report the raw result. No replay of completed commands.
Reviewer will inspect exact behavioral failures before authorizing production.

Extraction launcher:
```sh
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_013_RUNTIME_JS_RED_01.md").read_text(); code=s.split("```python\n# WAL013_RUNTIME_JS_RED_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL013_RUNTIME_JS_RED_01","exec"))'
```

```python
# WAL013_RUNTIME_JS_RED_01
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

pins = {'test/walletAccountManagement.node.js': '94a40ddbedea2298f707bb942218255ab6f55e7db9903c9401d18190f57eaaea', 'test/walletBrokerBuild.node.js': 'b1ac8ae3a1ca76c9580c92fec7cd1cbca0edb482246f072afcafb22a79e689de', 'test/walletBrokerRuntime.node.js': '989c5f64716d22e8b2903d6bf736fdadcae253d9ed11c6c648aec0fc2de14592', 'test/walletStartup.node.js': '4a64be8628ff54e74f282703abfcaf286c9ae9cec96a15e1d13561a4fb0d501e', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletSupervisorShutdown.node.js': 'f1156d8bf81a03a40ec9eb235666275ce85a3394b12f7b567bbb9da20a7543ef', 'test/electronSecurity.node.js': 'd70d6337ffbe97dfeee1894895c757b804dc18ae819219e8742adda5358ab482', 'social-main.js': '7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0', 'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a', 'wallet-broker/src/runtime.rs': '968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b', 'scripts/build-wallet-broker.js': 'c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}
raw_path = Path('wallet-broker/target/wal013-runtime-js-red-01.json')
evidence_path = Path('docs/testing/BBD-WAL-013-RUNTIME-JS-RED-01.md')
try:
    metadata()
    for name in ['walletAccountManagement','walletBrokerBuild']:
        row=command(['node','test/'+name+'.node.js'],60)
        assert row['exit']==1, 'expected behavior red'
        assert 'not ok' in row['output'], 'missing assertion failure'
        assert 'SyntaxError' not in row['output'] and 'MODULE_NOT_FOUND' not in row['output'], 'source/loader error'
    record['accepted']=True
except Exception as exc:
    record['stop']=str(exc)
finally:
    finish('WAL-013 runtime and menu expected red')
sys.exit(0 if record['accepted'] else 1)

```
