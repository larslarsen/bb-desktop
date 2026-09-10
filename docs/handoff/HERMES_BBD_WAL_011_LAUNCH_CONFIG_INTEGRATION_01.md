# WAL-011 launch configuration integration 01

Actor: Hermes, integration only. Source and acceptance execution closed.
Parent: reviewer publication following 34e622d2 plus CURRENT-only launch record.
Read this handoff and CURRENT active prefix; no history reload.
Accepted validation: nine resolver groups, 13 supervisor groups, two detected
falsifications, exact restoration and final nine-group green. No replay.

Only integrated paths:
- wallet-broker/launch-config.js
- test/walletBrokerLaunchConfig.node.js
- docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md
- docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md
- docs/testing/BBD-WAL-011-LAUNCH-CONFIG-GREEN-01.md

No source/evidence rewriting or other staging. Preserve four pending npm/policy
files, WAL-009 evidence and rejected EXECUTABLE-GREEN-01 draft byte-for-byte.
No CURRENT, ticket, other docs, binaries or raw target artifacts in the commit.
Submit the exact extraction launcher once using terminal background=true,
notify_on_complete=true and the exact absolute repository workdir in the prompt.
No cd prefix, independent commands, repairs, retries, transcription, additional
actors, credentials/config discovery. Wait on the same process at most 60 seconds
per wait and report output, then stop. Reviewer collects without owner done.

Driver checks accepted raw/evidence and 17 frozen input hashes, actual version,
exact session row, HEAD/status and empty index. Stage exactly five paths, verify
scope and whitespace, commit and push existing remote without force. No commands
after push. Raw record only: wallet-broker/target/wal011-launch-config-integration-01.json
(existing ext4 target). Stop on any failure without unstaging or repairs.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_LAUNCH_CONFIG_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL011_LAUNCH_CONFIG_INTEGRATION_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_LAUNCH_CONFIG_INTEGRATION_01_DRIVER","exec"))'
```

```python
# WAL011_LAUNCH_CONFIG_INTEGRATION_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
report_path = Path('wallet-broker/target/wal011-launch-config-integration-01.json')
assert not report_path.exists(), 'record exists; no replay'
record = {'commands': [], 'success': False}
paths = ['wallet-broker/launch-config.js', 'test/walletBrokerLaunchConfig.node.js', 'docs/testing/BBD-WAL-011-LAUNCH-CONFIG-RED-01.md', 'docs/testing/BBD-WAL-011-LAUNCH-CONFIG-REFLECTION-RED-01.md', 'docs/testing/BBD-WAL-011-LAUNCH-CONFIG-GREEN-01.md']
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
    raw_path = Path('wallet-broker/target/wal011-launch-config-green-01.json')
    assert sha(raw_path) == 'd6131b28088999e7e74194995e2d04a9ae5f6b909a92364891525791f0de8421', 'accepted raw validation changed'
    raw = json.loads(raw_path.read_text())
    assert raw['green'] and len([c for c in raw['commands'] if c['argv'][0] == 'node']) == 5, 'accepted validation absent'
    for path, digest in raw['final_hashes'].items():
        assert sha(path) == digest, 'input identity mismatch: '+path
    assert sha(paths[4]) == 'ef19df852f51cfe2d113f584d0f083941eb75b3b1aeb45d6db8037e4c507dea8', 'accepted green evidence changed'
    record['integrated_hashes'] = {p: sha(p) for p in paths}
    command(['git', 'add', '--']+paths)
    staged = command(['git', 'diff', '--cached', '--name-only']).splitlines()
    assert sorted(staged) == sorted(paths), 'unexpected staged paths; stop'
    command(['git', 'diff', '--cached', '--check'])
    command(['git', 'diff', '--cached', '--stat'])
    command(['git', 'commit', '-m', 'Resolve pinned packaged wallet broker launch configuration'])
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
