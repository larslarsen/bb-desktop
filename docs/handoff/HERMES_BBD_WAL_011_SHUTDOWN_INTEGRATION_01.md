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


## Collected integration acceptance — 2026-09-09

Accept the five-path shutdown integration at
1e45d6d2bf78df6b998cc2062604f23dd1252061. Read-only reviewer Git inspection confirms
HEAD and origin/master identify that commit. Exactly five authorized files changed,
1256 insertions and ten deletions: supervisor, new shutdown test, eight-line fixture
wrapper and two evidence records. Every committed and working-file hash matches
the accepted integration record. No source repair or test replay is required.

Outer 68128 collected on owner done with exit 0. Actual session
20260909_200042_2359ec, provider nous, model poolside/laguna-s-2.1:free.
Hermes v0.18.2 (2026.7.7.2), upstream 8e85b276/local 10b6d1a9, Python 3.11.15.
Raw wallet-broker/target/wal011-shutdown-integration-01.json records success=true,
pre-integration HEAD 35d554b859acf0e64994abfa2470c0fa81a35a8b, empty initial index,
exact staging, clean whitespace, successful commit/push and all commands exit 0.

Exact-session tool inventory (80351–80358) contains bounded CURRENT/handoff reads,
one exact extraction launcher with repository workdir, one process wait and final
report. No additional commands, source/evidence edits, tests/builds, discovery or
post-push commands appear. The report's `--diff --cached --name-only` is a wording
typo; the saved driver command is correctly `git diff --cached --name-only`.

Accepted committed SHA-256 identities:

| Path | SHA-256 |
| --- | --- |
| wallet-broker/supervisor.js | c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a |
| test/walletSupervisorShutdown.node.js | 69eb8bcbc2d169e00cb52a7bff73e1b5f39d7890d24d79383f63e77b3994b5c7 |
| test/fixtures/wallet-broker/shutdown-child.js | 0d8fbfa8179338a8fc05e6c41f4cbbf0ef652c4ccadd73db8876591a51f96dcb |
| docs/testing/BBD-WAL-011-SHUTDOWN-RED-01.md | 32e6d2b69e05e95def6eaac0ccce6d4ff606b535fc362376934716c028b443ae |
| docs/testing/BBD-WAL-011-SHUTDOWN-GREEN-01.md | a34373da4315d49f95953d996e3a573b468513a512af19abfb2e80137b838248 |

The shared shutdown Promise now permits waiting for observed child close, or
reports a fixed TIMEOUT when closure remains unconfirmed. Existing quit/cancel
behavior and signal escalation are retained. Accepted validation covers six shutdown,
seven transport and 13 supervisor groups, premature-completion falsification and
restored six-group green. Real child cases use Node fixtures; native descendants,
forced OS termination and Electron application lifecycle are not accepted by this
result. The main process does not yet invoke shutdown or start a pinned broker.

Four unrelated npm/policy files remain modified; WAL-009 evidence and rejected
EXECUTABLE-GREEN-01 draft remain untracked. All actor authorizations are closed.
Next is reviewer scoping of Electron startup/quit composition, independent package
pin provenance and truthful unavailable behavior. No new implementation is yet
authorized. Reviewer executed no tests/builds. BBD-WAL-011 remains incomplete.
