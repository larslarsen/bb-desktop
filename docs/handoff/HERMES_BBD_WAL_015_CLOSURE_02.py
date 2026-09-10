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
    assert record['head'] == '105a2e36af03d26dbc15a3d052901d836b2a63b5', 'HEAD drift'
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


pins={'docs/architecture/BBD-WAL-001-REVIEW.md': '718639af629320fde351fd404f76c86d1eb57305b2ad91ab71e767eb0b1575da', 'docs/architecture/BBD-WAL-015-LIVE-SYNC-REVIEW.md': '640ca932845ede0f3315596d2c87ea8496e96b6c433ef3330fefbf674b9de89f', 'docs/handoff/CURRENT_TASK.md': '3e3de23ac04b4996daedbc4c873f6de445f3d2c4b8deb805680868b4cdd7e83f', 'tickets/BBD-WAL-015.md': 'a29a9d0e82d7cf80279ee11081022f45f1b6797b1ea7faa3e3fe824f4d404cdb', 'docs/handoff/HERMES_BBD_WAL_015_AUDIT_01.md': 'e761609d404380e1fa8921c8bac78c3dda357d06d716e027e21f1a74c64e2662', 'docs/handoff/HERMES_BBD_WAL_015_BUILD_01.md': 'd3454c7a93ef542656484d8b74741bf9a6feeb3ff77de25e7f8fa0ff5b0943a2', 'docs/handoff/HERMES_BBD_WAL_015_BUILD_01.py': '178be8016780a47ce74901ddd8faaa09e5793a16963d667c89f8adfe25e07519', 'docs/handoff/HERMES_BBD_WAL_015_CLOSURE_01.md': '30198484e3076bbee15d9b5174693801d5ae00ea1f7302aa62be27769e437734', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.md': 'aed487fa1df317115ed76cc79f9134fce12378083374f980e59ac7f33ae5accd', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_01.py': '2581ac0aa0990f21dea2dde37cac117c74f8224fc8c01ec8d21b6355934514e5', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.md': '9b9a2e5f8e892c79c2b9fe2fc1a5c14294893074b3bb1915adc3a73eda8343d5', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_02.py': 'd57722e2e113e399d9ab16ecbb8187a49f5818050936fbdd2a3563186c98802b', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.md': 'e4400f11cf092546a4034f19e71ebdaa1214d9cc094d4caaa20f5454a072497f', 'docs/handoff/HERMES_BBD_WAL_015_COMPILE_03.py': 'bcc40335fa9aea6863ae5f0b66617953d6f2c1b7bcb07e9ce32beb9a80b2787a', 'docs/handoff/HERMES_BBD_WAL_015_DEPS_01.md': '232f18fdb4aec7b44e3e0272b2b8d5341bf1a102e1455835aeca29dbf9ec74f8', 'docs/handoff/HERMES_BBD_WAL_015_FINAL_01.md': '40fc962144739578c51bddbc143e128c4496bd379e5156dd23f2d23252632518', 'docs/handoff/HERMES_BBD_WAL_015_FINAL_01.py': 'df0a11fa6436e8ea14683a3ad957a5567ec4233999f184751befa0c721b594bc', 'docs/handoff/HERMES_BBD_WAL_015_FINAL_02.md': '302394d445c05616c18cb7f3f9ce84a92ba4691a39fdd2837fbe9d1871603f17', 'docs/handoff/HERMES_BBD_WAL_015_FINAL_02.py': '6ef36a44e25e4c967c7f8c96c0b93b8e509ed902290137bd47213de5b969e757', 'docs/handoff/HERMES_BBD_WAL_015_GREEN_01.md': '121c26a530adc0d385dfcd921ed05026d2dc7a47ce44be4554478b71074c133c', 'docs/handoff/HERMES_BBD_WAL_015_GREEN_01.py': '44ef3433a761151d2f25b6b427d3dcd1a66ac745f16e47a1b9776a9488bd724c', 'docs/handoff/HERMES_BBD_WAL_015_GREEN_02.md': 'ac4ca058b925bdb7630cb0798826fa3e9f8dd46ae79e7f419dd0e407117c1ba1', 'docs/handoff/HERMES_BBD_WAL_015_GREEN_02.py': '88762b43e53a97b2e73bc84031cad183c9b1336eae8071f92cea6d014ba7f665', 'docs/handoff/HERMES_BBD_WAL_015_INTEGRATION_01.md': 'b6934299e40baa0f54b2cb5d99f2a0c7cdad3881c2efde5e2afe5dbd4a570fb5', 'docs/handoff/HERMES_BBD_WAL_015_INTEGRATION_01.py': '404c4058f85f0a870e7446860db8950406292227e2cec485d02531f1818955ed', 'docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.md': '122d10adf7539717d2b1180612147bca2b77ff88ff42916424ba8513624b6d60', 'docs/handoff/HERMES_BBD_WAL_015_LOCK_EDGE_01.py': 'e6495c15b2c6c29be68d471b9ca795a42a1e9a45bd7a944ed0e301e2a80184c9', 'docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.md': 'a838d68a1132ae633e0ae124deed895f537c5850dc116fd509ab8fff26ea731b', 'docs/handoff/HERMES_BBD_WAL_015_POLICY_GREEN_01.py': '7b04d154ed12b9a60279ce2b77c475cdff838226c3ae64ac6cd42411c62b9865', 'docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.md': '6c4a568e53cb628a828043c181f2759f3be5534bb12b80c1611368090d6f9ec6', 'docs/handoff/HERMES_BBD_WAL_015_POLICY_RED_01.py': '4ecd164d81e3a8fda5f8c3e24485a72f829969dbc85b8c03395383d0f089a89f', 'docs/handoff/HERMES_BBD_WAL_015_RED_01.md': '3729ea80feff68bcc9d5ffa98f6e214305e64dff213bc595c77ca1a4f09820ba', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.md': '9bc316a1ab9c7a8ded4956d3330275ce1b2f09cae297890e1a75ee93cc017010', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_01.py': 'a45b04f432e202d2c8e0ccbf05b0b8d36b74e2ac6b0603d390b09184c7d40e27', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.md': '729d29a920b11fdea3bce1faadb6e16c929851d7b7dbbd1a7593dd5523511487', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_02.py': '4f03004c089785bbf5e3228e21549472b35a0f9406d944e3e55048c6d33f9730', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.md': '773a8099ef00e3c810b7ac48855dd4d657f334965249115fc81d10b241eed0e4', 'docs/handoff/HERMES_BBD_WAL_015_RUNTIME_03.py': '103866bf7c186ed9d95d28c5c8887b169e70a7bb1985b996072df830673cb45d', 'docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.md': 'adfd396a185406f19345a51582c0bf1a6f06e60eb810e376939c21271f9f5abd', 'docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py': '8defe3c3c540e0a1d43b222821beb4f42b9c157b24749a3cebbd25ccc48c642f', 'docs/handoff/SOL_BBD_WAL_015_CORE_01.md': 'd37cc54a3d7509732d34bf786acdd26ad937be6eab553c4127ac878b9b4a6c17', 'docs/handoff/SOL_BBD_WAL_015_HELPERS_01.md': 'f6f850ebbc4d0b9b4c09586aff2d08117f1f6e57d514ff833277e79d1b689da9', 'docs/handoff/SOL_BBD_WAL_015_LINT_01.md': '753460174676a15d8de9f8da3861c4763b13e927f2a1f9cce3de5272cdca5e26', 'docs/handoff/SOL_BBD_WAL_015_POLICY_01.md': '440e386169e71735fc0dc299ec6abf083306f0a4fc4aaae9d7666dca11c14116', 'docs/handoff/SOL_BBD_WAL_015_PRODUCTION_01.md': '0c05947e69990b684b1697d0a7adf12b6f9c3991c4d429684ec621087e39d8b7', 'docs/handoff/SOL_BBD_WAL_015_RECOVERY_TEST_01.md': 'd0d5b802f5ccf8abc72d51e0c76e7a669f6d2d1f5771669ca21824d8d9d2bf62', 'docs/handoff/SOL_BBD_WAL_015_REORG_CORRECTION_01.md': '5628ddd9db92362d6bbd39793feccf04267bd5210a7aa79068416252d13048c6', 'docs/handoff/SOL_BBD_WAL_015_RUNTIME_CORRECTIONS_01.md': '8f34c83148944eb5464c9d49826e26a1a940a4f259e6f05c1e0521fc36b74151', 'docs/handoff/SOL_BBD_WAL_015_TESTS_01.md': '9ff231b911c61b153ac67c09507831efebb482da6a89f058f3a86da82cfbdece', 'docs/handoff/SOL_BBD_WAL_015_TRANSPORT_TESTS_01.md': '03b2d8a84221938617f1e5b90c405ed13f4a6abc6ba1a96fefb61544ac21be9c', 'docs/handoff/SOL_BBD_WAL_015_VALIDATION_TESTS_01.md': 'dcf3175185f9a2734405a4410c2ace879f0d71e1039b99e1dc8f1dc87d9f8440', 'docs/handoff/WAL_015_POLICY_REVIEWED_DELTAS.json': 'eaac41ea9c717f0535a52886e596e2e8a5dc14ede7722509908d6d36ffc1ecbd'}
public_file_pins={'wallet-broker/tests/secret_hygiene.rs': 'dcebe361c7061b06b9ec6bb6fbea88ccb208069f7360ae14e2d7c6ca83b433a4', 'docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py': '8defe3c3c540e0a1d43b222821beb4f42b9c157b24749a3cebbd25ccc48c642f', 'docs/testing/BBD-WAL-015-LIVE-SECRETS-DIAGNOSTIC-01.md': '1bd6ec4f69b0bb4c5ca5605fa6f00a96da5abf8f23badeaa97100f1e4f0a0792', 'wallet-broker/target/wal015-live-secrets-diagnostic-01.json': '6da79cea6b095a825bef6ec71d8f8e2b40f2f385aa1a4f79838fc7d5c2ab1fb7'}

# CLOSURE01 self-hash argument was incorrect; guard stopped before metadata/commands.
# Reviewer corrects the launcher and retains the failed no-side-effect raw record.
pins['docs/handoff/HERMES_BBD_WAL_015_CLOSURE_01.py']='7924e6bc910e05b0dca71b4a26e132392658d7c787235cfca735ce94853bddc7'
raw_path=Path('wallet-broker/target/wal015-governance-closure-02.json')
evidence_path=Path('wallet-broker/target/wal015-governance-closure-02.md')
try:
    assert len(sys.argv)==2 and len(sys.argv[1])==64
    assert hashlib.sha256(Path(__file__).read_bytes()).hexdigest()==sys.argv[1]
    pins['docs/handoff/HERMES_BBD_WAL_015_CLOSURE_02.py']=sys.argv[1]
    metadata()
    assert required(['git','diff','--cached','--name-only'])==''
    assert required(['git','branch','--show-current'])=='master'
    required(['git','diff','--check'])
    required(['git','add','--']+list(pins))
    assert set(required(['git','diff','--cached','--name-only']).splitlines())==set(pins)
    required(['git','commit','-m','Close WAL-015 live balance synchronization review'])
    record['governance_commit']=required(['git','rev-parse','HEAD'])
    for p,h in pins.items():
        assert hashlib.sha256(subprocess.run(['git','show','HEAD:'+p],check=True,stdout=subprocess.PIPE,timeout=60).stdout).hexdigest()==h
    assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in public_file_pins.items())
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert hashlib.sha256(Path(scanner).read_bytes()).hexdigest()=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'
    report=Path('wallet-broker/target/wal015-gitleaks-closure-triage-01.json')
    fd=os.open(report,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600);os.close(fd)
    scan=command([scanner,'git','--redact=0','--no-banner','--report-format','json','--report-path',str(report),'.'],240)
    assert scan['exit'] in (0,1)
    findings=json.loads(report.read_text())
    assert all(f['RuleID']=='generic-api-key' and f['File'].startswith(('docs/handoff/HERMES_BBD_WAL_015_','docs/testing/BBD-WAL-015-')) and f['Secret'] in set(public_file_pins.values()) for f in findings), 'unexpected scan finding; no values printed'
    record['public_checksum_false_positives']=len(findings)
    record['actual_secret_findings']=0
    for f in findings:f['Secret']='REDACTED';f['Match']='REDACTED'
    report.write_text(json.dumps(findings,indent=2)+'\n')
    required(['git','push','origin','master'],120)
    assert required(['git','ls-remote','origin','refs/heads/master'],120).split()[0]==record['governance_commit']
    record['final_status']=required(['git','status','--short'])
    record['accepted']=True
except Exception as exc:record['stop']=str(exc)
finally:finish('WAL-015 reviewer governance publication')
sys.exit(0 if record['accepted'] else 1)
