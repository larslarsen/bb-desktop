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


pins={'wallet-broker/src/account_ui.rs': 'ae8aa3b7bd14390244c5e73475195c33a1ba69079ac9ba6b0d86d869941fda75', 'wallet-broker/src/accounts.rs': '34ea1b21527887122bd2933c81c8647dbf59de4cb0a0af7675d142f0393c2c45', 'wallet-broker/src/hygiene.rs': '7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/src/main.rs': '19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'wallet-broker/src/session.rs': '9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/xmr.rs': '78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce', 'wallet-broker/src/xmr/account.rs': '5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb', 'wallet-broker/src/xmr/distribution.rs': '163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8', 'wallet-broker/src/xmr/model.rs': '2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb', 'wallet-broker/src/xmr/process.rs': '66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d', 'wallet-broker/src/xmr/receiver.rs': 'daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9', 'wallet-broker/src/xmr/rpc.rs': '1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33', 'wallet-broker/src/xmr/store.rs': '3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4', 'wallet-broker/src/xmr/test_support.rs': '18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc', 'wallet-broker/src/zec.rs': '95335e042f2ab67368701f049098682697a7d77b29e2315c6e198dffd8c8755c', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/fixture.rs': '318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36', 'wallet-broker/src/zec/hardware.rs': '9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760', 'wallet-broker/src/zec/live.rs': 'c602073f4a926d8fde06b05e6c43297bfbdf6a8a418f1dde97800e2e123fd3c5', 'wallet-broker/src/zec/live_transport.rs': 'a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e', 'wallet-broker/src/zec/prepare.rs': '365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468', 'wallet-broker/src/zec/scan.rs': '0c4b75aa1e5b7b1b63b710a85a128dadea7dde0fb619189a787fa6bf7b42c3f7', 'wallet-broker/src/zec/spend.rs': '086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85', 'wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs': '60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66', 'wallet-broker/src/zec/spend/effects.rs': '54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72', 'wallet-broker/src/zec/spend/external_binding_tests.rs': 'bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d', 'wallet-broker/src/zec/spend/verification_context_tests.rs': '25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0', 'wallet-broker/src/zec/store.rs': 'd98f4641f64e931d79fdcdf5e87713fc3a5d0479eae7210b7c955b9465780910', 'wallet-broker/src/zec/test_support.rs': '284f4a035b0d1a5006483b292237aff83788f873428e67ee47f0677ad3877e64', 'wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs': '5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca', 'wallet-broker/tests/account_management.rs': '03834e502e133f00a374601bffa3730d38177742df69b4a05001fa8a15dcbd11', 'wallet-broker/tests/account_native_ui.rs': '1193b6181fb1f54b491386037d075f0681a1ff3fde155b9889adcdfc52d4e05e', 'wallet-broker/tests/native_surface.rs': '349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d', 'wallet-broker/tests/secret_hygiene.rs': 'dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4', 'wallet-broker/tests/vault_crypto.rs': 'a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b', 'wallet-broker/tests/vault_format.rs': '5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193', 'wallet-broker/tests/vault_session.rs': '67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b', 'wallet-broker/tests/vault_store.rs': '582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5', 'wallet-broker/tests/xmr_account.rs': '5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b', 'wallet-broker/tests/xmr_distribution.rs': '481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8', 'wallet-broker/tests/xmr_hygiene.rs': '3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f', 'wallet-broker/tests/xmr_local_gate.rs': '00a1c7f7e4d01254a94f35b9d38b4a7374d0b74fe3c80d42ef258d7fdcc8728d', 'wallet-broker/tests/xmr_process.rs': '25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833', 'wallet-broker/tests/xmr_receiver.rs': '39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e', 'wallet-broker/tests/xmr_rpc.rs': 'a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b', 'wallet-broker/tests/zec_address.rs': '2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3', 'wallet-broker/tests/zec_fixture_builder.rs': '40cc2b56132b42a765c86482e9915b0422adc0154c1e2edcfda4623760ec5d09', 'wallet-broker/tests/zec_hardware.rs': '32959949c9da01834fe10ab1328777ab906fb9f8c7bc3e8ef66945f6961ad7a7', 'wallet-broker/tests/zec_hygiene.rs': 'aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6', 'wallet-broker/tests/zec_live_sync.rs': '60d0093857a5a22a1cbb113b97fd8504827cc6a4a060b628543d0bccb71b7c8e', 'wallet-broker/tests/zec_live_transport.rs': 'f586257cb8827fcda68e7d38ebe4f4ea52852457e6599b0d231a266687756613', 'wallet-broker/tests/zec_prepare.rs': 'c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa', 'wallet-broker/tests/zec_scan.rs': '87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f', 'wallet-broker/tests/zec_sign_verify.rs': '47d6457f2031132efe09282ca38d2b0141b937a9ec48d6acdecbcb618f244f81', 'wallet-broker/tests/zec_store.rs': '1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225', 'wallet-broker/tests/fixtures/live-grpc-server.js': 'bd208bc877070e6cf27a45e5b7129107190bbd74390ac03c648bf9cd80b64c3b', 'wallet-broker/Cargo.toml': '435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b', 'wallet-broker/Cargo.lock': 'a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': 'fec10759a73a3c675d8206da454abf4b0e8dca6b62da463f39a2db27b30dc078', 'test/securityPolicy.node.js': 'a58d85e5322f0cb01a78b636454f92547bd98e556013cc07712dc0cf64ea565c', 'wallet-broker/tests/zec_live_validation.rs': 'f84e970072700e14cb4ffb1af2330043c315df7c631ac6302efda51a136fd3e7', 'wallet-broker/tests/zec_live_recovery.rs': '86a1ca797cc84e9ce56a8f8ac737015efafca8b217182ad1701c9e0c0335153f'}

raw_path=Path('wallet-broker/target/wal015-live-green-01.json')
evidence_path=Path('docs/testing/BBD-WAL-015-LIVE-GREEN-01.md')
try:
    metadata()
    record['formatted_files']=[]
    for name in ['wallet-broker/src/zec/live.rs','wallet-broker/src/zec/scan.rs']:
        result=subprocess.run([str(Path.home()/'.cargo/bin/rustup'),'run','1.98.0','rustfmt','--edition','2024','--emit','stdout'],input=Path(name).read_bytes(),stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=60)
        assert result.returncode==0 and result.stdout, 'formatter failed: '+name
        Path(name).write_bytes(result.stdout)
        pins[name]=hashlib.sha256(result.stdout).hexdigest()
        record['formatted_files'].append(name)
    record['formatted_hashes']=hashes()
    base=[str(Path.home()/'.cargo/bin/rustup'),'run','1.98.0','cargo']
    args=['--manifest-path','wallet-broker/Cargo.toml','--locked','--offline','--no-default-features','--features','native-ui']
    cargo=base+['test']+args
    for target in ['zec_live_sync','account_management','account_native_ui','vault_session','native_surface','zec_scan','zec_address','zec_store','zec_hygiene']:
        required(cargo+['--test',target],300)
    required(base+['clippy']+args+['--lib','--bin','bitbook-wallet-broker','--','-D','warnings'],300)
    # Falsify real SQLite interrupted-transaction recovery, then restore exact bytes.
    path=Path('wallet-broker/src/zec/live.rs'); original=path.read_bytes()
    old=b'    validate_dir(&directory, owner)?;\n    match fs::symlink_metadata(&path) {'
    new=b'    validate_dir(&directory, owner)?;\n    validate_sidecars(&directory)?;\n    match fs::symlink_metadata(&path) {'
    assert original.count(old)==1
    try:
        path.write_bytes(original.replace(old,new))
        row=command(cargo+['--test','zec_live_recovery','preparation_recovers_owned_hot_journal_without_losing_committed_state','--','--exact'],180)
        assert row['exit']==101 and 'FAILED' in row['output'] and 'STATE_CORRUPT' in row['output'] and 'could not compile' not in row['output'], 'recovery falsification not detected'
        record['hot_journal_falsification_detected']=True
    finally:
        path.write_bytes(original)
        assert hashlib.sha256(path.read_bytes()).hexdigest()==pins[str(path)]
    required(cargo+['--test','zec_live_recovery'],180)
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally: finish('WAL-015 broad Rust checks, production lint and recovery falsification')
sys.exit(0 if record['accepted'] else 1)
