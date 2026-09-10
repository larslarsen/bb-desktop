# WAL-011 awaitable shutdown integration 01

Actor: Hermes, integration only. Source and all test/build execution are closed.
Protected parent: reviewer publication following 4ec2cf35; one CURRENT-only launch
commit may follow. Read CURRENT lines 1–40 only and this handoff. No history reload.

Accepted validation: shutdown 6/0, transport 7/0, supervisor 13/0, selected
premature-completion failure as expected and restored shutdown 6/0. Exact restored
bytes and evidence are accepted; no replay or repair.

Only integration paths:

- wallet-broker/supervisor.js
- test/walletSupervisorShutdown.node.js
- test/fixtures/wallet-broker/shutdown-child.js
- docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md
- docs/testing/BBD-WAL-011-SHUTDOWN-GREEN-01.md

No source or evidence rewriting. Preserve four pending npm/policy files, WAL-009
evidence and rejected EXECUTABLE-GREEN-01 draft. Do not stage CURRENT, ticket, other
docs, raw target artifacts or binaries. All unrelated files stay as found.

Submit the short launcher exactly once, without a cd prefix, via terminal background=true,
notify_on_complete=true with workdir set to the repository root. Wait on that same process only,
at most 60 seconds per wait. No independent commands, preflight, discovery,
transcription, repairs or retry. Git push is the only network operation authorized,
using the existing remote without force. Stop immediately after driver completion.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_SHUTDOWN_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL011_SHUTDOWN_INTEGRATION_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_SHUTDOWN_INTEGRATION_01_DRIVER","exec"))'
```

The driver checks accepted validation and eight input identities plus green evidence,
records exact current session metadata, requires an empty index, stages five paths,
checks staged scope/whitespace, commits and pushes. Raw record goes to the existing
disk-backed target directory. No post-push commands. If any step fails, stop without
unstaging, retrying or repairing and report the actual failure. No credentials or
config discovery; only the exact session key and database row are authorized.

```python
# WAL011_SHUTDOWN_INTEGRATION_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
report_path = Path('wallet-broker/target/wal011-shutdown-integration-01.json')
assert not report_path.exists(), 'record exists; no replay'
record = {'commands': [], 'success': False}
paths = ['wallet-broker/supervisor.js', 'test/walletSupervisorShutdown.node.js', 'test/fixtures/wallet-broker/shutdown-child.js', 'docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md', 'docs/testing/BBD-WAL-011-SHUTDOWN-GREEN-01.md']
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
    raw_path = Path('wallet-broker/target/wal011-shutdown-green-01.json')
    assert sha(raw_path) == 'bda8e5a9a421a115c426ec431117ad61b9c691767027921b9b393c8fea5b4177', 'accepted raw validation changed'
    raw = json.loads(raw_path.read_text())
    assert raw['success'] and len(raw['stages']) == 5, 'accepted validation absent'
    for path, digest in raw['final_hashes'].items():
        assert sha(path) == digest, 'input identity mismatch: '+path
    assert sha(paths[4]) == 'a34373da4315d49f95953d996e3a573b468513a512af19abfb2e80137b838248', 'accepted green evidence changed'
    record['integrated_hashes'] = {p: sha(p) for p in paths}
    command(['git', 'add', '--']+paths)
    staged = command(['git', 'diff', '--cached', '--name-only']).splitlines()
    assert sorted(staged) == sorted(paths), 'unexpected staged paths; stop'
    command(['git', 'diff', '--cached', '--check'])
    command(['git', 'diff', '--cached', '--stat'])
    command(['git', 'commit', '-m', 'Add awaitable wallet broker shutdown with bounded completion'])
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
