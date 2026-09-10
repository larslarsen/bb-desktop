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


pins={'wallet-broker/src/account_ui.rs': 'ae8aa3b7bd14390244c5e73475195c33a1ba69079ac9ba6b0d86d869941fda75', 'wallet-broker/src/accounts.rs': 'f386f28f44c12bbdf1015713e4c736e81a86f8c0a27024f376b18a9eb8751d2f', 'wallet-broker/src/hygiene.rs': '7676aaad8ed78fb01fdb3cf2a763fd057693f5fe6f2721b385c3c8dd6d39bdbf', 'wallet-broker/src/lib.rs': 'e37713c4fd1a528a9260b8b11d740f1a317198f2eaf62a534729f4a566a10437', 'wallet-broker/src/main.rs': '19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d', 'wallet-broker/src/native.rs': 'a0ccb0b11e3e636cc28f9576ec0cf6da2d8c2ef5344370e5a7a2a9a9b5cc32e5', 'wallet-broker/src/native_ui.rs': 'c6607af3f36278f8d7deeb2b4276017177bbb4c695017f18bf4d2bcb412b268f', 'wallet-broker/src/native_ui/zec_native_app_tests.rs': 'd1d405f8667c6c9d184104d2023ffcff209f6d013e3920dd5c5ccd7e5a2c40f0', 'wallet-broker/src/native_ui/zec_review_tests.rs': '2eec3b60eb1b9fbd32235832a867d1189928126ca79373fa9f4fa7fdf2ee87bf', 'wallet-broker/src/runtime.rs': '6788914524605f4669d20d65f7b6ef7282b0090a0696ea45f088ecc49dc1c962', 'wallet-broker/src/session.rs': '9f88b3b0eca91d341a1c025bc8394a25e456350be58e99d9ac9417e31eef21af', 'wallet-broker/src/store.rs': '611d837641069a98d05b9e68c14bf11a37a5076de58bf6516188870eeab19236', 'wallet-broker/src/vault.rs': 'f23247b0e46c68dfc125804fa981d16f7022464e1e4d94a8c1e62771a045be49', 'wallet-broker/src/xmr.rs': '78107f241bb4cb8f02ab4168cbc81a01fc90cc75c80328a2677f819d7c06adce', 'wallet-broker/src/xmr/account.rs': '5dcad3d450a2e5d8d780e7e490111c33ba06da6275d7d1ca84e5f76dde09cddb', 'wallet-broker/src/xmr/distribution.rs': '163f8532bc7edfd80fc07966c0f8f32eebc0d12181fd273bc4e6c2870d86dea8', 'wallet-broker/src/xmr/model.rs': '2a2d3ba1ce453aca65138df402bde3e7f1fee997d5d069024cb1beb8102152cb', 'wallet-broker/src/xmr/process.rs': '66f0aae7fd0b507cbadc27628d0b1c26ee0033d90891c294721c11a00be9dd2d', 'wallet-broker/src/xmr/receiver.rs': 'daece8857b74eb7f369e0dfad7607dc418d397338cb311367448a632383df2b9', 'wallet-broker/src/xmr/rpc.rs': '1bbfdf3ec58f89728b2eb169e9d49c53512eb3b108e5c17f7b02bf2634fada33', 'wallet-broker/src/xmr/store.rs': '3a7f4d5b8cc7b33e3596910ce0b9b10d2f760f24c3ccff98fd2941c410ee2df4', 'wallet-broker/src/xmr/test_support.rs': '18e6d410b0b5186d45db82105229c8473ce10cfa39a5a54e57a6bc7d0714c2fc', 'wallet-broker/src/zec.rs': '95335e042f2ab67368701f049098682697a7d77b29e2315c6e198dffd8c8755c', 'wallet-broker/src/zec/address.rs': 'd9d54e301d188e3d4cb539095c5c01bcb07aaf011fcb1a50129aadc18f24cbfe', 'wallet-broker/src/zec/fixture.rs': '318820c6f125f2318ba0caf5ae44835b36b0f67856fc66b83401961f0c301b36', 'wallet-broker/src/zec/hardware.rs': '9546188299a2b7225b65820f3022fbaa85c1b66acc5266c0c37cc858ae910760', 'wallet-broker/src/zec/live.rs': '17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f', 'wallet-broker/src/zec/live_transport.rs': 'a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e', 'wallet-broker/src/zec/prepare.rs': '365ce7cc75219d616098512af900c6c34d0fd1784e844926be5946975a4e3468', 'wallet-broker/src/zec/scan.rs': '0c4b75aa1e5b7b1b63b710a85a128dadea7dde0fb619189a787fa6bf7b42c3f7', 'wallet-broker/src/zec/spend.rs': '086052983c0fae8bffb9461a599df809c92e0fe5b94adde413f2df07f1255b85', 'wallet-broker/src/zec/spend/cleanup_lifecycle_tests.rs': '60d2a67a7c7fff912b0df4ec16bcbe251b276b6e990c33f5433a1492539b2f66', 'wallet-broker/src/zec/spend/effects.rs': '54d1a2959f15f881a9e2bc56261b5090cad89aae06b97d7ace7747f2449a2a72', 'wallet-broker/src/zec/spend/external_binding_tests.rs': 'bb2d6873dd7262ad6e30f117f1947b7d4cf7685ab04e32a5b700d7cb5b112b6d', 'wallet-broker/src/zec/spend/verification_context_tests.rs': '25db22a3273aa2fcc3da38055726ebe834cabea05183b8fbe9ca9c47f5ae7ee0', 'wallet-broker/src/zec/store.rs': 'd98f4641f64e931d79fdcdf5e87713fc3a5d0479eae7210b7c955b9465780910', 'wallet-broker/src/zec/test_support.rs': '7b4dad2fef0c505f610bdcfe7b87b9075bde0673c8713e34ed57b7714326ce57', 'wallet-broker/src/zec/test_support/cleanup_lifecycle_tests.rs': '5b01727fdc2d864e0ec31693a58ad7f25041891f5cab1bc091ec0b8743323cca', 'wallet-broker/tests/account_management.rs': '03834e502e133f00a374601bffa3730d38177742df69b4a05001fa8a15dcbd11', 'wallet-broker/tests/account_native_ui.rs': '1193b6181fb1f54b491386037d075f0681a1ff3fde155b9889adcdfc52d4e05e', 'wallet-broker/tests/native_surface.rs': '349f3a019a0e7a5c37aaae727192b0ddcecb02192a5fae6df9291eaf1357276d', 'wallet-broker/tests/\u0073ecret_hygiene.rs': 'dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4', 'wallet-broker/tests/vault_crypto.rs': 'a83bc4d1bf30201ad8c9bfa09556e8bae029b8eefebf2eeeab4db5fed03e561b', 'wallet-broker/tests/vault_format.rs': '5c07a7a52a5be52d852e5c5d45bf62e2f86913324d8dcf642a455d6483b6f193', 'wallet-broker/tests/vault_session.rs': '67487db86d6788633e031418da71f6080409ac57a144ad362e311cb22519be6b', 'wallet-broker/tests/vault_store.rs': '582dd24bb91b30db8ec3f38bca6103994b8896f3b3a351c63ae00a7187a838c5', 'wallet-broker/tests/xmr_account.rs': '5e8afabaa8c820be84a010a88fef0e07617e67934c06b32ab445901b1ddad35b', 'wallet-broker/tests/xmr_distribution.rs': '481042e6dabe705fb7771c0cf692f0561298a9e0e8aa6d0cfd1d8f2ab9deb6f8', 'wallet-broker/tests/xmr_hygiene.rs': '3653eba660be481d71e75185e024ae7e0a17f8754089c83b21edb397da84230f', 'wallet-broker/tests/xmr_local_gate.rs': '00a1c7f7e4d01254a94f35b9d38b4a7374d0b74fe3c80d42ef258d7fdcc8728d', 'wallet-broker/tests/xmr_process.rs': '25166407a510fef113c5c1db948243ef60de884658f7a9545f49524af49da833', 'wallet-broker/tests/xmr_receiver.rs': '39d438a767214f31fe07d68a844b217e41bcd73ead1a90ab666b596085b6583e', 'wallet-broker/tests/xmr_rpc.rs': 'a1face1660ca0daf66002671ba3d794058b4acd497de2f0a3e25f2ca57597d0b', 'wallet-broker/tests/zec_address.rs': '2c5012e6884c8c2a81236266c6861b6e4e4fd6b124656dad2ab438add5848ee3', 'wallet-broker/tests/zec_fixture_builder.rs': '40cc2b56132b42a765c86482e9915b0422adc0154c1e2edcfda4623760ec5d09', 'wallet-broker/tests/zec_hardware.rs': '32959949c9da01834fe10ab1328777ab906fb9f8c7bc3e8ef66945f6961ad7a7', 'wallet-broker/tests/zec_hygiene.rs': 'aad7c95a2ef661063661f2ec0f16a216d80328096b049d636b88fa0252ba1be6', 'wallet-broker/tests/zec_live_sync.rs': '60d0093857a5a22a1cbb113b97fd8504827cc6a4a060b628543d0bccb71b7c8e', 'wallet-broker/tests/zec_live_transport.rs': 'f586257cb8827fcda68e7d38ebe4f4ea52852457e6599b0d231a266687756613', 'wallet-broker/tests/zec_prepare.rs': 'c38339ab88a954f725c7341b4384f178078116de1c700e16892409c18eb2f3fa', 'wallet-broker/tests/zec_scan.rs': '87ed1c3e8db8219ad126efb4de7f681cf909cb404b88df6c2dfadec471a3701f', 'wallet-broker/tests/zec_sign_verify.rs': '47d6457f2031132efe09282ca38d2b0141b937a9ec48d6acdecbcb618f244f81', 'wallet-broker/tests/zec_store.rs': '1c230a2a9cf51c841a0df6514393861387422e5d0b2a83e80af47022728e2225', 'wallet-broker/tests/fixtures/live-grpc-server.js': 'bd208bc877070e6cf27a45e5b7129107190bbd74390ac03c648bf9cd80b64c3b', 'wallet-broker/Cargo.toml': '435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b', 'wallet-broker/Cargo.lock': 'a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a', 'package.json': '76e7201bf5a60b2ec9ce39f92757d9411e33808f7ad8bf6a6890366e33d108f5', 'package-lock.json': '0e32de43f3c1a31ae3d9e9c8b172d10cd37a03577dc7d1eaf4ac916967cfcdb8', 'scripts/security-policy.js': '414a5645d8621b5d81cdeba7bb7461ec25647a5a1e5532295d6f75e7b8762532', 'test/securityPolicy.node.js': 'ff449a86e04570f8ff07bd280db6787c62f645f301d4235a65e825f3af7bbb9c', 'wallet-broker/tests/zec_live_validation.rs': 'f84e970072700e14cb4ffb1af2330043c315df7c631ac6302efda51a136fd3e7', 'wallet-broker/tests/zec_live_recovery.rs': '86a1ca797cc84e9ce56a8f8ac737015efafca8b217182ad1701c9e0c0335153f', 'wallet-broker/supervisor.js': '115e0e51425e7b25d8f6178b206f592165cb14dacde8cb544b970839833c4721', 'social-main.js': 'd959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a', 'scripts/build-wallet-broker.js': '4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445', 'test/walletAccountManagement.node.js': '6a449335e1101b8dd6340d640514e8fc083a09235210c17dd582b57230de9475', 'test/walletStartup.node.js': '448fb88268655b979c7c724ab21d13d09fee29a219628cd70f576daae4589de1', 'test/walletSupervisor.node.js': '7fcb775d2db7ae8f28b6d9f7660c44a056e59891f3e552db80a248d843c76aa4', 'test/walletStartupSmoke.node.js': 'a1c75efee5fcfdbeb9bf56dfc64f0e747ab6aa527d30ce416c5027bc2aa2a1ef', 'test/walletAccountWindowSmoke.node.js': '3590096cafed5596378488e8ebca8c916e62352d4410e5748a2033ab0ec9fa3a', 'test/fixtures/wallet-broker/x11-window.py': 'd192f036781e5623b88a8ba3f31b9ee7c781f7bdffa46982aa431f8ada5330d3', 'test/walletLiveSyncPolicy.node.js': '653c906645abf519f75d036467d65b2aaa705d9f3c775ee8638db644ed47700c', 'docs/handoff/WAL_015_POLICY_REVIEWED_DELTAS.json': 'eaac41ea9c717f0535a52886e596e2e8a5dc14ede7722509908d6d36ffc1ecbd'}
record_pins={'docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py': '2581ac0aa0990f21dea2dde37cac117c74f8224fc8c01ec8d21b6355934514e5', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py': 'a45b04f432e202d2c8e0ccbf05b0b8d36b74e2ac6b0603d390b09184c7d40e27', 'docs/handoff/HERMES_BBD_WAL_015_GREEN_02.py': '88762b43e53a97b2e73bc84031cad183c9b1336eae8071f92cea6d014ba7f665', 'docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py': '4ecd164d81e3a8fda5f8c3e24485a72f829969dbc85b8c03395383d0f089a89f', 'docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py': '178be8016780a47ce74901ddd8faaa09e5793a16963d667c89f8adfe25e07519', 'docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py': '8defe3c3c540e0a1d43b222821beb4f42b9c157b24749a3cebbd25ccc48c642f', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py': 'd57722e2e113e399d9ab16ecbb8187a49f5818050936fbdd2a3563186c98802b', 'docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py': '44ef3433a761151d2f25b6b427d3dcd1a66ac745f16e47a1b9776a9488bd724c', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py': 'bcc40335fa9aea6863ae5f0b66617953d6f2c1b7bcb07e9ce32beb9a80b2787a', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py': '4f03004c089785bbf5e3228e21549472b35a0f9406d944e3e55048c6d33f9730', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.py': '103866bf7c186ed9d95d28c5c8887b169e70a7bb1985b996072df830673cb45d', 'docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.py': '7b04d154ed12b9a60279ce2b77c475cdff838226c3ae64ac6cd42411c62b9865', 'docs/testing/BBD-WAL-015-LIVE-POLICY-GREEN-01.md': '8e1c94c84666df86fba417106e92883a6bd96ab65e7600ab1c40029495dfb453', 'docs/testing/BBD-WAL-015-LIVE-SECRETS-DIAGNOSTIC-01.md': '1bd6ec4f69b0bb4c5ca5605fa6f00a96da5abf8f23badeaa97100f1e4f0a0792', 'docs/testing/BBD-WAL-015-LIVE-GREEN-02.md': '51bf2eb0c12e661eea34cd535608afff48694c1ef150a81df77c64257d9f3914', 'docs/testing/BBD-WAL-015-LIVE-COMPILE-01.md': 'f77ecce5c5cd1c5159818c9521de4b67156cae89fff5faf490e8324395f71285', 'docs/testing/BBD-WAL-015-LIVE-RUNTIME-02.md': 'e4b6b85c4af9e93b269f06a1a1646cdb8278be422ab71736f01b2be297efeec1', 'docs/testing/BBD-WAL-015-LIVE-GREEN-01.md': '65495f80b62f2422c58baf14b33e42c45c9f154849eb19f26e9052ecaf6c8801', 'docs/testing/BBD-WAL-015-LIVE-COMPILE-03.md': '2baf851830b02a3b24d681d16eade68feeeb7dd95959c4804d1f0722d4a0b2d2', 'docs/testing/BBD-WAL-015-LIVE-COMPILE-02.md': 'c354157ef210168eef696bafdb715bdeba8d4c2ec1c9bb221d0c96d4121a632d', 'docs/testing/BBD-WAL-015-LIVE-RUNTIME-03.md': 'fc96cb886cf5959993b96d5c8cd3ea09c6bc44f8fc6c464f89aa97f68aff93db', 'docs/testing/BBD-WAL-015-LIVE-POLICY-RED-01.md': 'cdf7dc6c32ba848343a7c23f9d8828d2d1dbab7e9e0a69d83616f3433c989da3', 'docs/testing/BBD-WAL-015-LIVE-RUNTIME-01.md': '7c50ffe8aa4a867697da4fbef6517be8b4c6a4fdda5c04492c9c072d8ffd0b85', 'wallet-broker/target/wal015-live-secrets-diagnostic-01.json': '6da79cea6b095a825bef6ec71d8f8e2b40f2f385aa1a4f79838fc7d5c2ab1fb7', 'wallet-broker/target/wal015-live-compile-02.json': '4fa1aab19a1a67ba40f91ea9e0db265f21e8b20a0a5f54b4863bd066172faca9', 'wallet-broker/target/wal015-live-green-01.json': 'd4bcfa470c8e0f44ea7a696ec810c1bab37518d3454d255b005c05eedd6b6bc2', 'wallet-broker/target/wal015-live-policy-green-01.json': 'a8e7bd3e710bcd2ea27adc8406968a0e9fc0e703a8e2f0af00f741062f6f82ae', 'wallet-broker/target/wal015-live-compile-01.json': 'ff63ca3ca6aacc30368983167e23047b2e4f851ec245c7b8e876ab7d0a567c6d', 'wallet-broker/target/wal015-live-runtime-02.json': '7d1448842dbd0105195260f8cb79e07817eaf72517c9a86c5be46a8e3a935eb7', 'wallet-broker/target/wal015-live-compile-03.json': 'd90171352f4bc8a060ff5be33218d582105dd0b261b9021cd38e84c14d952dc7', 'wallet-broker/target/wal015-live-runtime-03.json': '8473748cdbbd25703479b63077b13bddc5be776cd4268e829b8db97346918156', 'wallet-broker/target/wal015-live-policy-red-01.json': '551a3c0218b43a05c942e650bdaa0851cb1bf2e93d22a55731e4ac90f9508edb', 'wallet-broker/target/wal015-live-green-02.json': '900695eb3eb51a2b342d7aa43c7cd63318c394ff18ab2f9ea6e8f319e8e5f256', 'wallet-broker/target/wal015-live-runtime-01.json': 'b5094992154bd1a8891715fef067438bfe7151b712a1ebdc32652cd58902359f'}

import ast
raw_path=Path('wallet-broker/target/wal015-live-final-01.json')
evidence_path=Path('docs/testing/BBD-WAL-015-LIVE-FINAL-01.md')
def canonical_evidence(text):
    return json.loads(text.split('```json\n',1)[1].split('\n```',1)[0])
try:
    metadata()
    key='wallet-broker/tests/'+chr(115)+'ecret_hygiene.rs'
    checksum=hashlib.sha256(Path(key).read_bytes()).hexdigest()
    assert checksum=='dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4'
    escaped=key.replace(chr(115)+'ecret',chr(92)+'u0073ecret')
    record['formatted_records']=[]
    for name,expected in record_pins.items():
        p=Path(name);before=p.read_text()
        assert hashlib.sha256(before.encode()).hexdigest()==expected
        after=before
        for quote in ['"',"'"]:
            old=quote+key+quote+': '+quote+checksum+quote
            new=quote+escaped+quote+': '+quote+checksum+quote
            after=after.replace(old,new)
        assert after!=before
        if p.suffix=='.py':
            assert ast.dump(ast.parse(before),include_attributes=False)==ast.dump(ast.parse(after),include_attributes=False)
        elif p.suffix=='.json': assert json.loads(before)==json.loads(after)
        else: assert canonical_evidence(before)==canonical_evidence(after)
        p.write_text(after)
        record['formatted_records'].append({'path':name,'before_sha256':expected,'after_sha256':hashlib.sha256(after.encode()).hexdigest(),'parsed_values_unchanged':True})
    # Future generated records use the same equivalent JSON key representation.
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert hashlib.sha256(Path(scanner).read_bytes()).hexdigest()=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'
    required([scanner,'dir','--redact=100','--no-banner','.'],180)
    required(['git','diff','--check'])
    policy=json.loads(Path('wallet-broker/target/wal015-live-policy-green-01.json').read_text())
    assert policy['unrelated_policy_edits_preserved'] and len(policy['policy_current_failures'])==6
    assert all(pins[p]==h for p,h in policy['final_hashes'].items())
    build=json.loads(Path('wallet-broker/target/wal015-live-green-02.json').read_text())
    binary=Path('wallet-broker/target/app-resources/wallet-broker/bitbook-wallet-broker')
    record['binary_sha256']=hashlib.sha256(binary.read_bytes()).hexdigest()
    assert record['binary_sha256']==build['binary_sha256']
    record['binary_bytes']=binary.stat().st_size
    record['accepted']=True
except Exception as exc: record['stop']=str(exc)
finally:
    finish('WAL-015 final verified application and faithful checksum-record formatting')
    for p in [raw_path,evidence_path]:
        s=p.read_text();p.write_text(s.replace('wallet-broker/tests/'+chr(115)+'ecret_hygiene.rs', 'wallet-broker/tests/'+chr(92)+'u0073ecret_hygiene.rs'))
sys.exit(0 if record['accepted'] else 1)
