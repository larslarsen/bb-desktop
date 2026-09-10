# WAL-011 finish only the missing working-tree secret scan

Prior Hermes outer 2349 / session 20260910_085930_0dff59 returned without the required
process wait. Its output pipe then closed: driver stopped at print AFTER the successful
Git secret scan. This is an orchestration failure, not a source/test failure. Raw and
normalized evidence match; all 59 source inputs remain unchanged/restored. Reviewer
accepts retained 70 unit groups + real-main/native smoke, actual build/pin, both
falsifications/restored green, unchanged policy baseline, zero-vulnerability audit,
and clean Git secret scan. Only working-tree scan remains. Do not repeat prior tests,
build, audit or Git scan. No process or source actor remains active.

Hermes nous / poolside/laguna-s-2.1:free execution/evidence ONLY. Parent publication
following fceb659c. Read this handoff only. Exact launcher ONCE with terminal
workdir explicitly absolute repository path, background=false, timeout=60.
Wait for the foreground result. If tool nevertheless returns a process ID, call
process.wait repeatedly (<=60 seconds) until exit. DO NOT end after launching.
No extra commands, retries, repairs, source edits, Git or other actors. Only writes
new raw/evidence below. Prior results and artifacts preserved. Driver output handles
closed-pipe reporting without losing execution; finish record is durable.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_STARTUP_GREEN_FINISH_01.md").read_text(); code=s.split("```python\n# WAL011_STARTUP_GREEN_FINISH_01\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_STARTUP_GREEN_FINISH_01","exec"))'
```

```python
# WAL011_STARTUP_GREEN_FINISH_01
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
raw_path = Path('wallet-broker/target/wal011-startup-green-finish-01.json')
evidence_path = Path('docs/testing/BBD-WAL-011-STARTUP-GREEN-FINISH-01.md')
pins = {'social-main.js': '7047253ee27955b0838ec226cd1e49c4a7dca8c3cc2a54a776fb1211c8a820d0', 'scripts/build-wallet-broker.js': 'c96684b9d7242a020a93c0ea8a96d590c7a262482f44726ac12ce14ab618e382', 'test/walletStartup.node.js': 'bf72c9f276938b8b2e538bbb7631303837d6f9da998f3a4c279d397a02f1bae3', 'test/walletBrokerBuild.node.js': '614e3ad8dde731c344d23353076672e0ba046a3e9d4919e303b5601350b97035', 'test/walletStartupSmoke.node.js': '35d87dc7c3f8f9f919277e5c16cc66e22fdc7228718d62ef455652b32dc3f09f', 'test/electronSecurity.node.js': 'df81caae58607184a0c407f5aa845aa90a2f729c7fa8a9ac0ec9edbc366e8eea', 'wallet-broker/supervisor.js': 'c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a', 'wallet-broker/launch-config.js': '68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8', 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4', 'wallet-pay/model.js': 'acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e', 'wallet-preload.js': '3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df', 'test/walletPreload.node.js': '60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e', 'test/walletSupervisor.node.js': 'eaaa63777c8ded89ea6e9b2a07912bd3a680953bdb989a4ff0d6894e5df6ce5c', 'test/walletBrokerLaunchConfig.node.js': '90acd32c9863df0206364eb04f707d92a5979ee66c7e15b2ec7b8250a5fc2113', 'test/fixtures/wallet-pay/snapshots-v1.json': 'bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md': '1e548d364ac3a39e9bb1941a5c89b0ccb45143d9c8588e35e42f12434120799f', 'docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md': '2a6abd0a41a93028ce63fce8b3f385436623a416dfcdc26565b0277e7875bc56', 'docs/testing/BBD-WAL-011-STARTUP-MAIN-RED-01.md': '89bc8c6a6619ff848d7f632f3f36c474b5d2f5b8f1e3104a8f9fcc53073bf5f7', 'docs/testing/BBD-WAL-011-STARTUP-BUILD-RED-01.md': 'efd7134cd477ddaa853b9f2e0a7bbf976d495ae3e0120cfec1a1bc4b3d879177', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/src/hygiene.rs': '7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf', 'wallet-broker/src/lib.rs': '08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925', 'wallet-broker/src/main.rs': '19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'wallet-broker/src/runtime.rs': '968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b', 'wallet-broker/src/session.rs': '42e4f335bb4080ad530d93dcc04d824b4ab54835be7f6c7cd68feba3f20ee227', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/xmr.rs': '78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce', 'wallet-broker/src/xmr/account.rs': '5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb', 'wallet-broker/src/xmr/distribution.rs': '163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8', 'wallet-broker/src/xmr/model.rs': '2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb', 'wallet-broker/src/xmr/process.rs': '66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d', 'wallet-broker/src/xmr/receiver.rs': 'daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9', 'wallet-broker/src/xmr/rpc.rs': '1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33', 'wallet-broker/src/xmr/store.rs': '3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4', 'wallet-broker/src/xmr/test_support.rs': '18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc', 'wallet-broker/src/zec.rs': '045cdc51f26ac8b9b1283cee5f995d60ceda38a25577aa7a937f5d07f177b90b', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/fixture.rs': '318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36', 'wallet-broker/src/zec/hardware.rs': '9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760', 'wallet-broker/src/zec/prepare.rs': '365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468', 'wallet-broker/src/zec/scan.rs': '54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad', 'wallet-broker/src/zec/spend.rs': '086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85', 'wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs': '60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66', 'wallet-broker/src/zec/spend/effects.rs': '54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72', 'wallet-broker/src/zec/spend/external_binding_tests.rs': 'bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d', 'wallet-broker/src/zec/spend/verification_context_tests.rs': '25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0', 'wallet-broker/src/zec/store.rs': '531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90', 'wallet-broker/src/zec/test_support.rs': 'bd005c21c7a601fe9edd2357525d4f1e8eb29d0b799e57a672433b591581f12d', 'wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs': '5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca', 'wallet-broker/target/wal011-startup-green-01.json': '9035d3ee833ed2aa53f15a6844ffbb4cf701c14388999586fce5a27343eed688', 'docs/testing/BBD-WAL-011-STARTUP-GREEN-01.md': 'c2aa40bd4a44ead2dee3382a9a1bcb9e6886d4724e049bbc0ad86141fd0c7377'}
record = {'commands': [], 'accepted': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def command(argv, timeout=60, env=None):
    try:
        result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout, env=env)
        row = {'argv': argv, 'exit': result.returncode, 'output': result.stdout}
    except subprocess.TimeoutExpired as exc:
        output = exc.stdout or b''
        if isinstance(output, bytes): output = output.decode('utf-8', errors='replace')
        row = {'argv': argv, 'exit': None, 'output': output, 'timeout': True}
    record['commands'].append(row)
    try: print(json.dumps(row), flush=True)
    except BrokenPipeError: pass
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
    evidence_path.write_text('# '+title+'\n\nActual bounded execution; release blockers are not waived.\n\n```json\n'+normalized+'\n```\n')
    try: print('ACCEPTED='+str(record['accepted']),flush=True)
    except BrokenPipeError: pass

try:
    metadata()
    previous = json.loads(Path('wallet-broker/target/wal011-startup-green-01.json').read_text())
    assert previous['stop'] == '[Errno 32] Broken pipe' and not previous['accepted'], 'wrong retained run'
    assert previous['input_hashes'] == previous['final_hashes'], 'retained source drift'
    assert previous['main_falsification_restored'] and previous['build_falsification_restored'], 'restoration absent'
    assert previous['policy_baseline_unchanged'] and previous['release_green'] is False, 'policy baseline absent'
    assert len(previous['commands']) == 23, 'unexpected retained command count'
    assert previous['commands'][-1]['argv'] == ['target/security-tools/gitleaks-v8.30.1/gitleaks','git','--redact=100','--no-banner','.'] and previous['commands'][-1]['exit'] == 0, 'git scan proof absent'
    artifact = previous['artifact']
    assert hashlib.sha256(Path(artifact['binary']).read_bytes()).hexdigest() == artifact['sha256'], 'binary drift'
    assert hashlib.sha256((Path(artifact['binary']).parent/'manifest.json').read_bytes()).hexdigest() == artifact['manifest_sha256'], 'manifest drift'
    record['retained_proof'] = {'raw':'wallet-broker/target/wal011-startup-green-01.json','sha256':pins['wallet-broker/target/wal011-startup-green-01.json'],'session':previous['session'],'passing_distinct_groups':71,'artifact':artifact,'both_falsifications_restored':True,'policy_baseline_unchanged':True,'audit_clean':True,'git_secret_scan_clean':True}
    scanner = 'target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert hashlib.sha256(Path(scanner).read_bytes()).hexdigest() == '88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509', 'scanner pin mismatch'
    required([scanner,'dir','--redact=100','--no-banner','.'],180)
    record['release_green'] = False
    record['accepted'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    finish('WAL-011 completed application startup validation')
sys.exit(0 if record['accepted'] else 1)
```

## Collected final validation acceptance

Hermes outer 79760 collected/closed; session 20260910_090154_3794cc, nous /
poolside/laguna-s-2.1:free, v0.18.2 upstream 67764dc0/local 10b6d1a9.
Foreground launcher completed; exact-session messages 80480–80485 contain handoff
read, exact launcher and final only. Working-tree Gitleaks scanned 1.94 GB in
16.2 seconds, exit 0/no leaks. All 61 frozen inputs unchanged/current; normalized
evidence/raw equality verified. Raw SHA256 c7c7ec41d475df89d4c6c92a2f02b4ff1c9ec459bc2f290420136b79821f3de8.
Together with retained prior results this completes local startup acceptance.
Only HERMES_BBD_WAL_011_STARTUP_INTEGRATION_01.md is authorized next; no replay.
