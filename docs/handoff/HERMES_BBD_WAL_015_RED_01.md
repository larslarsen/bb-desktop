# WAL-015 live sync initial expected red

Reviewer inspected Principal Dev Sol's complete three-file test-only drop: 9 core,
3 manager and 2 native tests, including corrections to hostname bounds, real batch
budget validation, account/job binding and decimal ZEC display. Production is frozen.
Hermes alone may execute the embedded driver once through terminal in this repository,
background=true/notify_on_complete=true, process.wait <=60 seconds until actual EXIT.
No execute_code, extra commands, source edits, formatters, dependencies or Git writes.
Disk-backed ext4 target is confirmed. No user account or running user app access.
Expected missing API compile errors; reviewer must inspect actual errors before green.

Exact launcher:
```text
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_015_RED_01.md").read_text(); code=s.split("```python\n# WAL015_RED_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL015_RED_DRIVER","exec"))'
```

```python
# WAL015_RED_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys, signal
record = {'commands': [], 'accepted': False}
def hashes():
    return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in pins}
def emit(value):
    try: print(value, flush=True)
    except BrokenPipeError: pass
def command(argv, timeout=60, env=None):
    process = subprocess.Popen(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, env=env, start_new_session=True)
    timed_out = False
    try:
        output, _ = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(process.pid, signal.SIGTERM)
        try: output, _ = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            output, _ = process.communicate(timeout=5)
    row = {'argv': argv, 'exit': process.returncode, 'output': output}
    if timed_out: row['timeout'] = True
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
    assert record['head'] == 'e85dfdc8b6e06fdd09f84cee3c6ac37037448437', 'HEAD drift'
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


pins={'wallet-broker/src/account_ui.rs': 'fc671ebf02ecb92ef28101540ad78fad15a1b326cd198e41b85ede91be52be99', 'wallet-broker/src/accounts.rs': '5768cef932b8926bdfc80810d799e81e99ba63370ad8f06a48fb7b7563a566e1', 'wallet-broker/src/hygiene.rs': '7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/src/main.rs': '19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'wallet-broker/src/session.rs': '9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/xmr.rs': '78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce', 'wallet-broker/src/xmr/account.rs': '5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb', 'wallet-broker/src/xmr/distribution.rs': '163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8', 'wallet-broker/src/xmr/model.rs': '2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb', 'wallet-broker/src/xmr/process.rs': '66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d', 'wallet-broker/src/xmr/receiver.rs': 'daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9', 'wallet-broker/src/xmr/rpc.rs': '1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33', 'wallet-broker/src/xmr/store.rs': '3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4', 'wallet-broker/src/xmr/test_support.rs': '18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc', 'wallet-broker/src/zec.rs': '2544d86b3022aacea6e64f75ce37c38da0ce27cb6f555e085fc76450b5760d1e', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/fixture.rs': '318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36', 'wallet-broker/src/zec/hardware.rs': '9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760', 'wallet-broker/src/zec/prepare.rs': '365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468', 'wallet-broker/src/zec/scan.rs': '54a890fd0b5a8ca8ce78dfb59c13424407341df25e47a5eacd2de8d6cc3317ad', 'wallet-broker/src/zec/spend.rs': '086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85', 'wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs': '60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66', 'wallet-broker/src/zec/spend/effects.rs': '54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72', 'wallet-broker/src/zec/spend/external_binding_tests.rs': 'bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d', 'wallet-broker/src/zec/spend/verification_context_tests.rs': '25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0', 'wallet-broker/src/zec/store.rs': '531d0a6171ecd9b5012602ccb870f16eab96888c809b3ffdf4e6303576f17c90', 'wallet-broker/src/zec/test_support.rs': 'bd005c21c7a601fe9edd2357525d4f1e8eb29d0b799e57a672433b591581f12d', 'wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs': '5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca', 'wallet-broker/tests/zec_live_sync.rs': 'ba5abca785c2fa40139afae74948174b1fdcdbd94c2301ed7f9280f03e9f99bf', 'wallet-broker/tests/account_management.rs': '22a31f33cc589c99514b2e215596c5f843c02bfe28866eb6d5ae286743fa5773', 'wallet-broker/tests/account_native_ui.rs': 'ecdebfa97e0685df590cf69bbfcca5a04c6313da087d1190744667933aac98e6', 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503', 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c'}

raw_path=Path('wallet-broker/target/wal015-live-red-01.json')
evidence_path=Path('docs/testing/BBD-WAL-015-LIVE-RED-01.md')
if raw_path.exists() or evidence_path.exists(): raise RuntimeError('existing result; no replay')
try:
    metadata()
    cargo=[str(Path.home()/'.cargo/bin/rustup'),'run','1.98.0','cargo','test','--manifest-path','wallet-broker/Cargo.toml','--locked','--offline','--no-default-features']
    for extra in [['--test','zec_live_sync'],['--test','account_management','wal015_'],['--features','native-ui','--test','account_native_ui','wal015_']]:
        row=command(cargo+extra,180)
        assert row['exit']==101 and ('LiveSync' in row['output'] or 'live_sync' in row['output']), 'missing live-sync API red absent'
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally: finish('WAL-015 live sync initial expected red')
sys.exit(0 if record['accepted'] else 1)

```
