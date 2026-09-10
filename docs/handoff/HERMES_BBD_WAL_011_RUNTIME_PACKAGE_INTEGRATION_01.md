# WAL-011 runtime package integration 01

Actor: Hermes, integration only. Source and all acceptance execution closed.
Parent: reviewer publication following fbaf8045 plus CURRENT-only launch record.
Read this handoff and CURRENT active prefix; no history or other discovery.
Seven package groups/44 cases, nine resolver and 30 Electron groups, falsification,
restoration, npm audit/Gitleaks and actual Linux package inventory proof accepted.
Two policy command failures remain identical to baseline and block release.

Only integration paths:
- scripts/stage-wallet-runtime.js
- scripts/build-deb.sh
- scripts/build-macos.sh
- scripts/build-windows.ps1
- test/walletRuntimePackage.node.js
- docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md
- docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-GREEN-01.md

No rewriting, other staging, test/build/scan replay or artifact changes. Preserve
all unrelated pending files and the retained package/backup/staging/extraction.
Do not stage CURRENT, ticket, other docs, binaries or raw target artifacts.

Submit exact extraction launcher once via terminal background=true,
notify_on_complete=true and exact absolute repository workdir in prompt. No cd,
independent commands, transcription, repairs, retry, additional actors or config
access. Wait on same process at most 60 seconds per wait, report and stop without
post-push commands. Reviewer collects directly without owner done.

Driver checks accepted evidence/raw, 23 frozen input hashes and both .deb hashes;
records actual version, exact current session row, HEAD/status and empty index;
stages exactly seven files, checks scope/whitespace, commits and pushes existing
remote without force. Push is the only network operation. Stop on any failure.
Raw record only: wallet-broker/target/wal011-runtime-package-integration-01.json.
Existing target ext4. No source repairs, cleanup, release publication or waiver.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_RUNTIME_PACKAGE_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL011_RUNTIME_PACKAGE_INTEGRATION_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_RUNTIME_PACKAGE_INTEGRATION_01_DRIVER","exec"))'
```

```python
# WAL011_RUNTIME_PACKAGE_INTEGRATION_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
report_path = Path('wallet-broker/target/wal011-runtime-package-integration-01.json')
assert not report_path.exists(), 'record exists; no replay'
record = {'commands': [], 'success': False}
paths = ['scripts/stage-wallet-runtime.js', 'scripts/build-deb.sh', 'scripts/build-macos.sh', 'scripts/build-windows.ps1', 'test/walletRuntimePackage.node.js', 'docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-RED-01.md', 'docs/testing/BBD-WAL-011-RUNTIME-PACKAGE-GREEN-01.md']
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def command(argv, timeout=60):
    p = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout)
    row = {'argv': argv, 'exit': p.returncode, 'output': p.stdout}
    record['commands'].append(row)
    print(json.dumps(row), flush=True)
    assert p.returncode == 0, 'command failed; stop'
    return p.stdout
try:
    record['version'] = command(['hermes', '--version']).strip()
    record['head'] = command(['git', 'rev-parse', 'HEAD']).strip()
    record['initial_status'] = command(['git', 'status', '--short'])
    assert not command(['git', 'diff', '--cached', '--name-only']).strip(), 'index must start empty'
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'session key absent; no discovery'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        row = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?', (sid,)).fetchone()
    assert row and all(row), 'session metadata absent; no discovery'
    record['session'] = dict(zip(('id', 'model', 'provider'), row))
    raw_path = Path('wallet-broker/target/wal011-runtime-package-green-01.json')
    assert sha(raw_path) == '85e704fd61c64f0b01db2990cbbbd75ae651def3ff0a8b747672b2e9dc668d9e', 'accepted raw validation changed'
    raw = json.loads(raw_path.read_text())
    assert raw['component_green'] and not raw['release_green'] and len(raw['policy_comparison']) == 2, 'accepted validation absent'
    for path, digest in raw['final_hashes'].items():
        assert sha(path) == digest, 'input identity mismatch: '+path
    assert sha(paths[6]) == 'e56fcc0ef2bbb27c5557800a8b5b44b2c185afa8760d5798c90c51cdc9718d29', 'accepted green evidence changed'
    for key in ['package', 'preserved_old_package']:
        assert sha(raw[key]['path']) == raw[key]['sha256'], 'accepted artifact changed: '+key
    record['integrated_hashes'] = {p: sha(p) for p in paths}
    command(['git', 'add', '--']+paths)
    staged = command(['git', 'diff', '--cached', '--name-only']).splitlines()
    assert sorted(staged) == sorted(paths), 'unexpected staged paths; stop'
    command(['git', 'diff', '--cached', '--check'])
    command(['git', 'diff', '--cached', '--stat'])
    command(['git', 'commit', '-m', 'Include wallet runtime modules in desktop packages'])
    command(['git', 'push'], timeout=120)
    record['success'] = True
except Exception as exc:
    record['stop'] = str(exc)
finally:
    report_path.write_text(json.dumps(record, indent=2)+'\n')
    print('INTEGRATION_SUCCESS='+str(record['success']), flush=True)
    print('INTEGRATION_RECORD='+str(report_path), flush=True)
sys.exit(0 if record['success'] else 1)
```
