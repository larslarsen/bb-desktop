# WAL-011 runtime package validation and Linux inventory proof 01

Actor: Hermes, execution/evidence only. Grok and integration closed.
Parent: reviewer publication following 04a11f9b plus CURRENT-only launch record.
Read this handoff and CURRENT active prefix; AGENTS/TESTING/routing if needed.
Submit exact launcher once using terminal background=true, notify_on_complete=true
and explicit absolute repository workdir from prompt. No cd, independent commands,
transcription, repairs, retry, source/test edits except exact temporary preload
falsification in driver, Git mutations, config discovery or additional actors.
Wait only on the same process, at most 60 seconds per wait, report actual output.
Reviewer stays with execution and collects automatically without owner done.

Authorized source state is the reviewed 113-line staging helper and three packager
edits; every permanent source/test/evidence hash is frozen in the driver. Source
roots and package destinations are trusted build roots; no ancestor race defense
is claimed. No other production behavior or native-wallet integration is allowed.

Commands and checks, once in order, stop on unexpected failure and restore helper:
- Seven runtime package groups/44 cases, nine resolver groups, 30 Electron security
  groups; bash -n both changed Bash scripts.
- Remove only preload inventory line; selected exact-inventory group must fail at
  the missing preload. Restore exact helper, then repeat seven package groups.
- Repeat the two policy baseline commands; require identical exit and full output.
  Six known policy groups and Rust source-inventory checker remain release blockers.
- npm audit --audit-level=low; pinned Gitleaks 8.30.1 version, git and dir scans
  with --redact=100 --no-banner. All must exit 0, no leaks or low+ vulnerabilities.
  Only audit registry/advisory access is authorized; no installs or other network.
- Verify ext4 dist, Linux x64 and already-installed Electron 44.0.0. No download.
- Preserve existing dist/bitbook_0.1.0_amd64.deb by exclusive byte-identical backup
  dist/bitbook_0.1.0_amd64.pre-wal011-runtime-package-01.deb, which must be absent.
- bash scripts/build-deb.sh once; retained random staging under project dist,
  maximum 600 seconds. No recursive cleanup. Replaces the current .deb only after
  verified backup; no existing backup may be overwritten.
- dpkg-deb --contents and --extract into new, initially absent
  wallet-broker/target/wal011-runtime-package-extract-01. Verify exact app inventory,
  five copied hashes/modes and main hash, root/root chrome-sandbox mode 4755, and
  actual nested Node imports from extracted modules. No GUI/native-broker launch.
- Verify all frozen hashes and preserved old package. No validation replay.

Only other writes: wallet-broker/target/wal011-runtime-package-green-01.json and
docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-GREEN-01.md. dist and target verified ext4;
90 GiB free at reviewer preflight. Retain package/staging/extraction for inspection,
never recursively delete. Small unit-test fixtures self-clean through unlink/rmdir.
component_green means the inventory change passed its evidence, while release_green
remains false for the known policy blockers. No package release, SBOM acceptance,
macOS/Windows native proof, broker pin or startup claim. No policy/ignore edits.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_RUNTIME_PACKAGE_GREEN_01.md").read_text(); code=s.split("```python\n# WAL011_RUNTIME_PACKAGE_GREEN_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_RUNTIME_PACKAGE_GREEN_01_DRIVER","exec"))'
```

```python
# WAL011_RUNTIME_PACKAGE_GREEN_01_DRIVER
from pathlib import Path
import hashlib, json, os, shutil, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-runtime-package-green-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-GREEN-01.md')
assert not raw_path.exists() and not evidence_path.exists(), 'record exists; no replay'
pins = {'scripts/build-deb.sh': '6ccaeed5fe31c487ed15d1d6424de1d9f2ef82f3b9ef79f90e4c4329e5142aa8', 'scripts/build-macos.sh': '29e6ede69b2db49e780599af5a6a51cc4d263d3083683e6ea13c24321811f171', 'scripts/build-windows.ps1': '8ce8faaff9e9a772540e63165122be9e30f78186f035570bbc63f6dd366d16c1', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'social-main.js': 'c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3', 'packaging/runtime-package.json.in': '1660c63cf7d36abdfacf47333a69496c3a03121819100cb64a55e2c11a658d9e', 'test/walletRuntimePackage.node.js': '5d8a8bf250a4e664fb15853cfb29deee62130efd72cdcf3acdac1129c2324f25', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md': '1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f', 'docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md': '2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56', 'scripts/stage-wallet-runtime.js': '97d04b4434e498bf4a2dd70d1ec69881221d8466664656e87fbffdd1c0694a70', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'test/electronSecurity.node.js': '7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c', 'docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md': 'd6c1d5f642b23fa31c13ae933bb661359b123932c6d0541f53b66a9d6b6fa791', 'node_modules/electron/package.json': '3280e83f3a5a5d0825f44708a12c7c695b03034dd0e07356790c655de67cbedf', '.gitleaksignore': '1e239ec10a1f2ccf59711258fe514f827727e984ca063a6a685ab325313b563b'}
record = {'commands': [], 'component_green': False, 'release_green': False}
helper = Path('scripts/stage-wallet-runtime.js')
original = None
mutated = False
inventory = ['wallet-preload.js','wallet-broker/protocol.js','wallet-broker/supervisor.js','wallet-broker/launch-config.js','wallet-pay/model.js']
def sha(p):
    digest = hashlib.sha256()
    with Path(p).open('rb') as f:
        for chunk in iter(lambda: f.read(1024*1024), b''): digest.update(chunk)
    return digest.hexdigest()
def hashes(): return {name: sha(name) for name in pins}
def command(argv, timeout=60):
    try:
        result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout)
        row = {'argv': argv, 'exit': result.returncode, 'output': result.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8',errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    record['commands'].append(row)
    print(json.dumps(row), flush=True)
    return row
def required(argv, timeout=60):
    row = command(argv,timeout)
    assert row['exit'] == 0, 'required command failed: '+str(argv)
    return row['output'].strip()
def counts(row):
    return {'ok':sum(x.startswith('ok ') for x in row['output'].splitlines()),'not_ok':sum(x.startswith('not ok ') for x in row['output'].splitlines())}
def green(argv,n,footer):
    row=command(argv)
    row['counts']=counts(row)
    assert row['exit']==0 and row['counts']=={'ok':n,'not_ok':0}, 'suite failed: '+str(argv)
    assert footer in row['output'].splitlines(), 'green footer absent'
def restore():
    global mutated
    if mutated: helper.write_bytes(original); mutated=False
    assert sha(helper)==pins[str(helper)], 'restore mismatch'
try:
    record['version']=required(['hermes','--version'])
    record['head']=required(['git','rev-parse','HEAD'])
    record['initial_status']=required(['git','status','--short'])
    sid=os.environ.get('HERMES_SESSION_ID'); assert sid, 'session key absent; no discovery'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro',uri=True) as db:
        row=db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?',(sid,)).fetchone()
    assert row and all(row), 'exact session metadata absent'
    record['session']=dict(zip(('id','model','provider'),row))
    record['input_hashes']=hashes(); assert record['input_hashes']==pins, 'input hash mismatch'
    baseline_path=Path('wallet-broker/target/wal011-runtime-package-red-01.json')
    assert sha(baseline_path)=='5608cd092ae81d83e658aa9231e2751378f5f15fbde3a60c2b9dc176d5a370bd', 'baseline raw changed'
    baseline=json.loads(baseline_path.read_text())
    original=helper.read_bytes()
    green(['node','test/walletRuntimePackage.node.js'],7,'BitBook wallet runtime package tests passed (7).')
    green(['node','test/walletBrokerLaunchConfig.node.js'],9,'BitBook wallet broker launch configuration tests passed (9).')
    green(['node','test/electronSecurity.node.js'],30,'BitBook electron security tests passed (30).')
    required(['bash','-n','scripts/build-deb.sh'])
    required(['bash','-n','scripts/build-macos.sh'])
    old=b"  'wallet-preload.js',\n"
    assert original.count(old)==1, 'preload falsification anchor mismatch'
    mutated=True
    try:
        helper.write_bytes(original.replace(old,b'',1))
        js="const assert=require('assert'); const name='staging: exact five-file inventory preserves bytes and excludes native data'; const tests=require('./test/walletRuntimePackage.node.js').tests.filter(t=>t.name===name); assert.strictEqual(tests.length,1); try{tests[0].fn(); console.log('ok '+name);}catch(e){console.error('not ok '+name+'\\n'+e.stack);process.exitCode=1;}"
        row=command(['node','-e',js]);row['counts']=counts(row)
        assert row['exit']==1 and row['counts']=={'ok':0,'not_ok':1}, 'missing-preload falsification not detected'
        assert 'AssertionError [ERR_ASSERTION]' in row['output'] and 'wallet-preload.js' in row['output'], 'wrong falsification diagnostic'
        assert 'cleanup also failed' not in row['output'], 'falsification cleanup failed'
        row['mutant_sha256']=sha(helper)
    finally: restore()
    row['restored_sha256']=sha(helper)
    green(['node','test/walletRuntimePackage.node.js'],7,'BitBook wallet runtime package tests passed (7).')
    record['policy_comparison']=[]
    for prior in baseline['policy_baseline']:
        row=command(prior['argv'])
        assert row['exit']==prior['exit'] and row['output']==prior['output'], 'policy baseline changed; review required'
        record['policy_comparison'].append({'argv':row['argv'],'exit':row['exit'],'identical_to_baseline':True})
    record['known_release_blockers']=['Six inherited security-policy test failures','Repository Rust source inventory mismatch']
    required(['npm','audit','--audit-level=low'],timeout=120)
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert sha(scanner)=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509', 'scanner identity mismatch'
    record['gitleaks_version']=required([scanner,'version'])
    assert record['gitleaks_version']=='8.30.1', 'scanner version mismatch'
    required([scanner,'git','--redact=100','--no-banner','.'],timeout=180)
    required([scanner,'dir','--redact=100','--no-banner','.'],timeout=180)
    assert required(['findmnt','-T','dist','-n','-o','FSTYPE'])=='ext4','dist not reviewed disk filesystem'
    assert required(['uname','-m'])=='x86_64','only Linux x64 package proof authorized'
    pkg=json.loads(Path('package.json').read_text()); epkg=json.loads(Path('node_modules/electron/package.json').read_text())
    assert pkg['version']=='0.1.0' and epkg['version']=='44.0.0', 'package metadata differs'
    assert Path('node_modules/electron/dist/electron').is_file() and os.access('node_modules/electron/dist/electron',os.X_OK), 'existing Electron unavailable; no download'
    assert Path('node_modules/electron/dist/chrome-sandbox').is_file(), 'existing sandbox helper unavailable'
    output=Path('dist/bitbook_0.1.0_amd64.deb')
    backup=Path('dist/bitbook_0.1.0_amd64.pre-wal011-runtime-package-01.deb')
    extract=Path('wallet-broker/target/wal011-runtime-package-extract-01')
    assert not backup.exists() and not backup.is_symlink() and not extract.exists() and not extract.is_symlink(), 'artifact destinations already exist; no overwrite/replay'
    assert output.is_file() and not output.is_symlink(), 'expected existing package absent/type mismatch'
    old_hash=sha(output)
    with output.open('rb') as src, backup.open('xb') as dst: shutil.copyfileobj(src,dst,1024*1024)
    assert sha(backup)==old_hash, 'old package backup mismatch'
    record['preserved_old_package']={'path':str(backup),'sha256':old_hash}
    build=required(['bash','scripts/build-deb.sh'],timeout=600)
    stages=[x.removeprefix('Staging retained at ') for x in build.splitlines() if x.startswith('Staging retained at ')]
    assert len(stages)==1, 'missing retained staging location'
    stage=Path(stages[0]); assert stage.parent==Path.cwd()/'dist' and stage.name.startswith('bitbook-deb.') and stage.is_dir() and not stage.is_symlink(), 'unexpected staging path'
    record['retained_staging']=str(stage)
    record['package']={'path':str(output),'sha256':sha(output),'bytes':output.stat().st_size}
    listing=required(['dpkg-deb','--contents',str(output)])
    sandbox=[x for x in listing.splitlines() if x.endswith(' ./usr/lib/bitbook/chrome-sandbox')]
    assert len(sandbox)==1 and sandbox[0].split()[:2]==['-rwsr-xr-x','root/root'], 'sandbox mode/ownership differs'
    record['sandbox_entry']=sandbox[0]
    required(['dpkg-deb','--extract',str(output),str(extract)],timeout=120)
    app=extract/'usr/lib/bitbook/resources/app'
    expected=['package.json','social-main.js','wallet-preload.js','wallet-broker','wallet-pay','social','imgs']
    assert sorted(x.name for x in app.iterdir())==sorted(expected), 'unexpected packaged app inventory'
    assert sorted(x.name for x in (app/'wallet-broker').iterdir())==['launch-config.js','protocol.js','supervisor.js'], 'native/source files leaked into runtime inventory'
    assert sorted(x.name for x in (app/'wallet-pay').iterdir())==['model.js'], 'unexpected Pay inventory'
    record['packaged_hashes']={}
    for rel in inventory:
        dest=app/rel; assert dest.is_file() and not dest.is_symlink(), 'packaged file type mismatch'
        assert sha(dest)==pins[rel], 'packaged byte mismatch: '+rel
        assert dest.stat().st_mode & 0o777 == 0o644, 'packaged mode mismatch: '+rel
        record['packaged_hashes'][rel]=sha(dest)
    assert sha(app/'social-main.js')==pins['social-main.js'], 'main package mismatch'
    probe="const path=require('path');const root=process.argv[1];for(const rel of ['wallet-broker/supervisor.js','wallet-broker/protocol.js','wallet-broker/launch-config.js','wallet-pay/model.js'])require(path.resolve(root,rel));console.log('packaged nested imports resolved');"
    assert required(['node','-e',probe,str(app)])=='packaged nested imports resolved', 'packaged imports failed'
    assert sha(backup)==old_hash, 'preserved old artifact changed'
    record['final_hashes']=hashes(); assert record['final_hashes']==pins, 'inputs changed during validation'
    record['component_green']=True
except Exception as exc:
    record['stop']=str(exc)
finally:
    if mutated:
        try: restore()
        except Exception as exc: record['restore_error']=str(exc)
    if original is not None: record['final_helper_sha256']=sha(helper)
    raw_path.write_text(json.dumps(record,indent=2)+'\n')
    normalized=json.dumps(record,indent=2).replace(str(Path.cwd()),'<repo>').replace(str(Path.home()),'<home>')
    evidence_path.write_text('# WAL-011 runtime package validation 01\n\nComponent validation, actual package proof and preserved policy baseline below. release_green remains false while the recorded inherited policy blockers stand. No native broker pin, main startup, other-platform execution or release publication is claimed.\n\n```json\n'+normalized+'\n```\n')
    print('COMPONENT_GREEN='+str(record['component_green']),flush=True)
    print('RELEASE_GREEN='+str(record['release_green']),flush=True)
    print('EVIDENCE='+str(evidence_path),flush=True)
sys.exit(0 if record['component_green'] else 1)

```
