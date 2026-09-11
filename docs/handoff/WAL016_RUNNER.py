from pathlib import Path
import os, sys, json, hashlib, sqlite3, subprocess, signal
spec=json.loads(Path(sys.argv[1]).read_text())
record={'accepted':False,'commands':[]}
pins=dict(spec['pins'])
raw=Path(spec['raw']); evidence=Path(spec['evidence'])
def digest(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def run(argv,timeout=180,env=None):
    child=subprocess.Popen(argv,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,env=env,start_new_session=True)
    timed_out=False
    try:out,_=child.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out=True;os.killpg(child.pid,signal.SIGTERM)
        try:out,_=child.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid,signal.SIGKILL);out,_=child.communicate(timeout=5)
    row={'argv':argv,'exit':child.returncode,'output':out,'timeout':timed_out}
    record['commands'].append(row)
    print(json.dumps({'argv':argv,'exit':child.returncode,'tail':out[-1600:]}),flush=True)
    return row
def required(argv,timeout=180,env=None):
    row=run(argv,timeout,env);assert row['exit']==0 and not row['timeout'],str(argv)
    return row['output'].strip()
def scan(mode):
    scanner='target/security-tools/gitleaks-v8.30.1/gitleaks'
    assert digest(scanner)=='88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'
    public_files=[
        'wallet-broker/tests/secret_hygiene.rs',
        'docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py',
        'docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.md',
        'docs/testing/BBD-WAL-015-LIVE-SECRETS-DIAGNOSTIC-01.md',
        'wallet-broker/target/wal015-live-secrets-diagnostic-01.json']
    allowed={digest(p) for p in public_files}
    report=raw.with_name(raw.stem+'-gitleaks-'+mode+'.json')
    fd=os.open(report,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600);os.close(fd)
    row=run([scanner,mode,'--redact=0','--no-banner','--report-format','json','--report-path',str(report),'.'],240)
    assert row['exit'] in (0,1) and not row['timeout'],'scanner error'
    findings=json.loads(report.read_text())
    assert all(f['RuleID']=='generic-api-key' and f['File'].startswith(('docs/handoff/HERMES_BBD_WAL_015_','docs/testing/BBD-WAL-015-','wallet-broker/target/wal015-')) and f['Secret'] in allowed for f in findings),'unexpected scan finding; captured values remain private'
    record.setdefault('scans',[]).append({'mode':mode,'exit':row['exit'],'public_checksum_false_positives':len(findings),'actual_credentials':0,'classification':'docs/architecture/BBD-WAL-015-FINAL-SCAN-REVIEW.md'})
    for finding in findings:finding['Secret']='REDACTED';finding['Match']='REDACTED'
    report.write_text(json.dumps(findings,indent=2)+'\n')
try:
    assert not raw.exists() and not evidence.exists(),'artifact already exists'
    sid=os.environ['HERMES_SESSION_ID']
    with sqlite3.connect((Path.home()/'.hermes/state.db').as_uri()+'?mode=ro',uri=True) as db:
        row=db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?',(sid,)).fetchone()
    assert row and all(row)
    record['session']=dict(zip(('id','model','provider'),row))
    record['version']=required(['hermes','--version'])
    record['head']=required(['git','rev-parse','HEAD'])
    assert record['head']==spec['head'],'HEAD drift'
    record['input_hashes']={p:digest(p) for p in pins}
    assert record['input_hashes']==pins,'input drift'
    for prerequisite in spec.get('requires_records',[]):
        previous=json.loads(Path(prerequisite).read_text())
        assert previous['accepted'],'unaccepted prerequisite '+prerequisite
    record['formatted']=[]
    for p in spec.get('format',[]):
        result=subprocess.run([str(Path.home()/'.cargo/bin/rustup'),'run','1.98.0','rustfmt','--edition','2024','--emit','stdout'],input=Path(p).read_bytes(),stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=60)
        assert result.returncode==0 and result.stdout,result.stderr.decode()
        Path(p).write_bytes(result.stdout);pins[p]=digest(p);record['formatted'].append(p)
    for cmd in spec['commands']:
        argv=[a.replace('<rustup>',str(Path.home()/'.cargo/bin/rustup')) for a in cmd['argv']]
        env=None
        if cmd.get('cargo_path'):
            env=os.environ.copy();env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','')
        original=None
        if 'falsify' in cmd:
            mutation=cmd['falsify'];target=Path(mutation['path']);original=target.read_bytes()
            before=mutation['before'].encode();after=mutation['after'].encode()
            assert original.count(before)==1,'mutation occurrence mismatch'
            target.write_bytes(original.replace(before,after))
        try:row=run(argv,cmd.get('timeout',180),env)
        finally:
            if original is not None:
                target.write_bytes(original)
                assert digest(target)==pins[str(target)],'falsification restoration mismatch'
                record['falsification_restored']=True
        assert row['exit']==cmd.get('exit',0) and not row['timeout'],'command exit mismatch'
        for needle in cmd.get('contains',[]):assert needle in row['output'],'missing expected outcome '+needle
        for needle in cmd.get('absent',[]):assert needle not in row['output'],'unexpected outcome '+needle
        if cmd.get('policy_baseline'):
            baseline=json.loads(Path(cmd['policy_baseline']).read_text())['policy_current_failures']
            failures=[line for line in row['output'].splitlines() if line.startswith('not ok')]
            assert failures==baseline,'policy regression'
            record['inherited_policy_failures']=failures
    if spec.get('artifacts'):
        record['artifacts']={p:{'sha256':digest(p),'bytes':Path(p).stat().st_size} for p in spec['artifacts']}
    for mode in spec.get('scans',[]):scan(mode)
    if 'integrate' in spec:
        integration=spec['integrate'];paths=integration['paths']
        assert required(['git','diff','--cached','--name-only'])=='','index not empty'
        assert required(['git','branch','--show-current'])=='master'
        assert required(['git','remote','get-url','origin'])=='https://github.com/larslarsen/bb-desktop.git'
        assert {p:digest(p) for p in pins}==pins,'pre-integration drift'
        record['integrated_paths']={p:{'sha256':digest(p),'lines':len(Path(p).read_text().splitlines())} for p in paths}
        required(['git','diff','--check'])
        required(['git','add','--']+paths)
        assert set(required(['git','diff','--cached','--name-only']).splitlines())==set(paths),'staged scope mismatch'
        required(['git','diff','--cached','--check'])
        required(['git','commit','-m',integration['message']])
        record['source_commit']=required(['git','rev-parse','HEAD'])
        for p in paths:
            blob=subprocess.run(['git','show','HEAD:'+p],stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=60)
            assert blob.returncode==0 and hashlib.sha256(blob.stdout).hexdigest()==record['integrated_paths'][p]['sha256'],'committed blob mismatch'
        scan('git')
        required(['git','push','origin','master'],120)
        assert required(['git','ls-remote','origin','refs/heads/master'],120).split()[0]==record['source_commit']
        record['source_pushed']=True
    record['accepted']=True
except Exception as error:record['stop']=str(error)
finally:
    record['final_hashes']={p:digest(p) for p in pins}
    if record['final_hashes']!=pins:record['accepted']=False;record['stop']='final input drift'
    raw.write_text(json.dumps(record,indent=2)+'\n')
    normalized=json.dumps(record,indent=2).replace(str(Path.cwd()),'<repo>').replace(str(Path.home()),'<home>')
    evidence.write_text('# '+spec['title']+'\n\n```json\n'+normalized+'\n```\n')
    print('ACCEPTED='+str(record['accepted']),flush=True)
if record['accepted'] and 'integrate' in spec:
    try:
        required(['git','add','--',str(evidence)])
        assert required(['git','diff','--cached','--name-only'])==str(evidence)
        required(['git','commit','-m','Record WAL016 sync correction verification'])
        record['evidence_commit']=required(['git','rev-parse','HEAD'])
        required(['git','push','origin','master'],120)
        assert required(['git','ls-remote','origin','refs/heads/master'],120).split()[0]==record['evidence_commit']
        record['final_status']=required(['git','status','--short'])
    except Exception as error:record['accepted']=False;record['stop']=str(error)
    raw.write_text(json.dumps(record,indent=2)+'\n')
    print('INTEGRATION_ACCEPTED='+str(record['accepted']),flush=True)
sys.exit(0 if record['accepted'] else 1)
