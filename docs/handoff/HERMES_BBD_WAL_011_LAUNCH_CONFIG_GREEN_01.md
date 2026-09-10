# WAL-011 launch configuration green and falsification 01

Actor: Hermes, execution/evidence only; Grok closed. No integration yet.
Parent: reviewer publication following fc053abe plus CURRENT-only launch record.
Read this handoff and CURRENT active prefix; AGENTS/TESTING/routing if needed.
Submit the exact extraction launcher once using terminal background=true,
notify_on_complete=true and the exact absolute repository workdir supplied in the
launch prompt. No independent commands, driver transcription, repairs, retries,
config/history discovery, test edits, Git mutation or extra actors. Wait only on
that process in intervals at most 60 seconds; report actual output and stop.
Reviewer stays with execution and collects automatically, no owner done required.

Only writes: two named evidence/raw records and the exact temporary production
falsifications in the driver, restored in finally. No permanent source edits.
Target verified ext4. Existing tests own their small unlink/rmdir temp fixtures.
All 17 frozen files, including pending unrelated work, must remain byte-identical.
The driver records version, exact current session row, HEAD, status and hashes.

Commands in order, each once; stop at unexpected result, restore and record:
1. node test/walletBrokerLaunchConfig.node.js — nine groups/62 authored rows green.
2. node test/walletSupervisor.node.js — 13 affected groups green.
3. Remove only manifest target equality; run only the existing invalid-manifest
   group. Expected platform mismatch: expected UNAVAILABLE assertion; restore.
4. Export the existing unchecked implementation to bypass only normalization;
   run only four existing reflection groups. Expected four code assertions; restore.
5. node test/walletBrokerLaunchConfig.node.js — restored nine groups green.
No suite beyond those, native builds, package execution, policy repair or security
waiver. Module remains disconnected. Source/evidence integration follows review.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_LAUNCH_CONFIG_GREEN_01.md").read_text(); code=s.split("```python\n# WAL011_LAUNCH_CONFIG_GREEN_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_LAUNCH_CONFIG_GREEN_01_DRIVER","exec"))'
```

```python
# WAL011_LAUNCH_CONFIG_GREEN_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-launch-config-green-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-LAUNCH-CONFIG-GREEN-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a', 'test/walletSupervisor.node.js': 'eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'social-main.js': 'c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3', 'test/electronSecurity.node.js': '7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'test/fixtures/wallet-pay/snapshots-v1.json': 'bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252', 'docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md': '1c98fde59cd882a406b6e73082f5fd5a1d8b7328cadc2a71e3d46941a3ae5485', 'docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md': 'b19697561f63f4690304f0c57a8eef89e86204ba9d57024758bd509f32063523', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md': '1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f', 'docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md': '2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56'}
record = {'commands': [], 'green': False}
module = Path('wallet-broker/launch-config.js')
original = None
mutated = False
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def command(argv, timeout=60):
    try:
        result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout)
        row = {'argv': argv, 'exit': result.returncode, 'output': result.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    record['commands'].append(row)
    print(json.dumps(row), flush=True)
    return row
def required(argv):
    row = command(argv)
    assert row['exit'] == 0, 'metadata command failed; stop'
    return row['output'].strip()
def counts(row):
    lines = row['output'].splitlines()
    return {'ok': sum(x.startswith('ok ') for x in lines), 'not_ok': sum(x.startswith('not ok ') for x in lines)}
def green_suite(argv, count, footer):
    row = command(argv)
    row['counts'] = counts(row)
    assert row['exit'] == 0 and row['counts'] == {'ok': count, 'not_ok': 0}, 'suite failed; stop'
    assert footer in row['output'].splitlines(), 'missing green footer; stop'
    return row
def selected(names):
    js = "const assert=require('assert'); const names="+json.dumps(names)+"; const tests=require('./test/walletBrokerLaunchConfig.node.js').tests.filter(t=>names.includes(t.name)); assert.deepStrictEqual(tests.map(t=>t.name),names); let failed=0; for(const {name,fn} of tests){try{fn(); console.log('ok '+name);}catch(e){failed++; console.error('not ok '+name+'\\n'+e.stack);}} process.exitCode=failed?1:0;"
    return command(['node', '-e', js])
def restore():
    global mutated
    if mutated:
        module.write_bytes(original)
        mutated = False
    assert hashlib.sha256(module.read_bytes()).hexdigest() == pins[str(module)], 'restoration mismatch'
def falsify(old, new, names, diagnostics):
    global mutated
    assert original.count(old) == 1, 'falsification anchor mismatch; stop'
    changed = original.replace(old, new, 1)
    mutated = True
    try:
        module.write_bytes(changed)
        row = selected(names)
        row['counts'] = counts(row)
        row['mutant_sha256'] = hashlib.sha256(changed).hexdigest()
        assert row['exit'] == 1 and row['counts'] == {'ok': 0, 'not_ok': len(names)}, 'falsification not detected'
        assert [x for x in row['output'].splitlines() if x.startswith('not ok ')] == ['not ok '+x for x in names], 'wrong falsification groups'
        for diagnostic in diagnostics: assert diagnostic in row['output'], 'wrong falsification assertion'
    finally:
        restore()
    row['restored_sha256'] = hashlib.sha256(module.read_bytes()).hexdigest()
try:
    record['version'] = required(['hermes', '--version'])
    record['head'] = required(['git', 'rev-parse', 'HEAD'])
    record['initial_status'] = required(['git', 'status', '--short'])
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'current session ID absent; no discovery'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        row = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?', (sid,)).fetchone()
    assert row and all(row), 'session metadata absent; no discovery'
    record['session'] = dict(zip(('id','model','provider'), row))
    record['input_hashes'] = hashes()
    assert record['input_hashes'] == pins, 'input identity mismatch; no execution'
    original = module.read_bytes()
    green_suite(['node','test/walletBrokerLaunchConfig.node.js'], 9, 'BitBook wallet broker launch configuration tests passed (9).')
    green_suite(['node','test/walletSupervisor.node.js'], 13, 'BitBook wallet supervisor tests passed (13).')
    falsify(b'  if (manifest.platform !== platform || manifest.arch !== arch) unavailable();',
            b'  // Reviewer-authorized temporary target-check falsification.',
            ['invalid manifest: reject malformed extra mismatched and hostile digest rows'],
            ['AssertionError [ERR_ASSERTION]: platform mismatch: expected UNAVAILABLE'])
    reflection = ['revoked proxy','getPrototypeOf trap','ownKeys trap','getOwnPropertyDescriptor trap']
    falsify(b'module.exports = { resolveWalletBrokerLaunch };',
            b'module.exports = { resolveWalletBrokerLaunch: resolveWalletBrokerLaunchUnchecked };',
            ['reflection errors: '+x+' is sanitized' for x in reflection],
            ['AssertionError [ERR_ASSERTION]: '+x+': code' for x in reflection])
    green_suite(['node','test/walletBrokerLaunchConfig.node.js'], 9, 'BitBook wallet broker launch configuration tests passed (9).')
    record['final_hashes'] = hashes()
    assert record['final_hashes'] == pins, 'input changed; stop'
    record['green'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    if mutated:
        try: restore()
        except Exception as exc: record['restore_error'] = str(exc); record['green'] = False
    if original is not None: record['final_source_sha256'] = hashlib.sha256(module.read_bytes()).hexdigest()
    raw_path.write_text(json.dumps(record, indent=2)+'\n')
    normalized = json.dumps(record, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    evidence_path.write_text('# WAL-011 launch configuration green evidence 01\n\n'
        'Actual command outputs and restoration hashes. green=true requires nine resolver groups, thirteen supervisor groups, two detected falsifications, exact restoration and final nine-group green. No packaging or main-startup integration is claimed.\n\n'
        '```json\n'+normalized+'\n```\n')
    print('GREEN='+str(record['green']), flush=True)
    print('EVIDENCE='+str(evidence_path), flush=True)
sys.exit(0 if record['green'] else 1)

```

## Green acceptance

Outer 73046 completed exit 0. Accept nine resolver groups (62 authored rows) and
13 supervisor groups, target-equality falsification detected at platform mismatch
expected UNAVAILABLE, normalization-bypass falsification detected at all four code
assertions, exact restoration after each, final nine resolver groups green. All
17 frozen input hashes are unchanged at collection. Evidence/raw JSON agree.
Source remains 68adf92e; tests 90acd32c. No acceptance execution by reviewer.

Session 20260909_230128_986ca6, nous / poolside/laguna-s-2.1:free. Actual Hermes
v0.18.2 (2026.7.7.2), upstream cfdbbb6e/local 10b6d1a9, Python 3.11.15. Record
the actual upstream value, which differs from the preceding red session. HEAD
34e622d2. Exact-session messages 80414–80421 show handoff read, exact launcher
with repository workdir, one wait and report; no extra command or mutation.
Raw 144 lines, SHA-256 d6131b28088999e7e74194995e2d04a9ae5f6b909a92364891525791f0de8421.
Evidence 150 lines, SHA-256 ef19df852f51cfe2d113f584d0f083941eb75b3b1aeb45d6db8037e4c507dea8.
Hermes execution closed. Only the five-path integration handoff is authorized.
