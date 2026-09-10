# WAL-011 Electron normal quit integration 01

Actor: Hermes, integration only. Source and all test/build execution are closed.
Protected parent: reviewer publication following 743552a0c163ac8fcb8da9e240ccb6b5c96d0a21; one CURRENT-only launch
commit may follow. Read CURRENT lines 1–40 only and this handoff. No history reload.

Accepted validation: Electron 30/0, preload 6/0, selected early-quit falsification
rejected as expected at line 1419, restored Electron 30/0. Exact restored bytes and evidence are accepted; no replay or repair.

Only integration paths:

- social-main.js
- test/electronSecurity.node.js
- docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md
- docs/testing/BBD-WAL-011-ELECTRON-QUIT-GREEN-01.md

No source or evidence rewriting. Preserve four pending npm/policy files, WAL-009
evidence and rejected EXECUTABLE-GREEN-01 draft. Do not stage CURRENT, ticket, other
docs, raw target artifacts or binaries. All unrelated files stay as found.

Submit the short launcher exactly once via terminal background=true,
notify_on_complete=true from the repository root. Wait on that same process only,
at most 60 seconds per wait. No independent commands, preflight, discovery,
transcription, repairs or retry. Git push is the only network operation authorized,
using the existing remote without force. Stop immediately after driver completion.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_ELECTRON_QUIT_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL011_ELECTRON_QUIT_INTEGRATION_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_ELECTRON_QUIT_INTEGRATION_01_DRIVER","exec"))'
```

The driver checks accepted validation and nine input identities plus green evidence,
records exact current session metadata, requires an empty index, stages four paths,
checks staged scope/whitespace, commits and pushes. Raw record goes to the existing
disk-backed target directory. No post-push commands. If any step fails, stop without
unstaging, retrying or repairing and report the actual failure. No credentials or
config discovery; only the exact session key and database row are authorized.

```python
# WAL011_ELECTRON_QUIT_INTEGRATION_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
report_path = Path('wallet-broker/target/wal011-electron-quit-integration-01.json')
assert not report_path.exists(), 'record exists; no replay'
record = {'commands': [], 'success': False}
paths = ['social-main.js', 'test/electronSecurity.node.js', 'docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md', 'docs/testing/BBD-WAL-011-ELECTRON-QUIT-GREEN-01.md']
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
    raw_path = Path('wallet-broker/target/wal011-electron-quit-green-01.json')
    assert sha(raw_path) == 'd242b83d099d0bc006705aa4c2593a8843cf9470168273c900c282f56da8fceb', 'accepted raw validation changed'
    raw = json.loads(raw_path.read_text())
    assert raw['success'] and len(raw['stages']) == 4, 'accepted validation absent'
    for path, digest in raw['final_hashes'].items():
        assert sha(path) == digest, 'input identity mismatch: '+path
    assert sha(paths[3]) == '6bbe4e88de9980f3856b7ffea53df210fe8650ac935618fe3ddfbe5f5e6d8b0e', 'accepted green evidence changed'
    record['integrated_hashes'] = {p: sha(p) for p in paths}
    command(['git', 'add', '--']+paths)
    staged = command(['git', 'diff', '--cached', '--name-only']).splitlines()
    assert sorted(staged) == sorted(paths), 'unexpected staged paths; stop'
    command(['git', 'diff', '--cached', '--check'])
    command(['git', 'diff', '--cached', '--stat'])
    command(['git', 'commit', '-m', 'Wait for wallet broker shutdown before normal Electron quit'])
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

Accept integration at 21e2e4d546adb06321e820ea77a52147fa47a71a. Read-only reviewer
Git inspection confirmed HEAD and origin/master both identify this commit and
exactly the four authorized paths changed: 1042 insertions/five deletions.
Both committed and working bytes match every accepted integration hash:

| Path | SHA-256 |
| --- | --- |
| social-main.js | c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3 |
| test/electronSecurity.node.js | 7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c |
| docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md | 229c1a97b501dcf0a1f60865245e76c644520776500b06f3c3ff5d0ddedf167b |
| docs/testing/BBD-WAL-011-ELECTRON-QUIT-GREEN-01.md | 6bbe4e88de9980f3856b7ffea53df210fe8650ac935618fe3ddfbe5f5e6d8b0e |

Outer 28190 collected on owner done, exit 0. Actual Hermes session
20260909_213252_c3ca6f, nous / poolside/laguna-s-2.1:free. Hermes v0.18.2
(2026.7.7.2), upstream 8e85b276/local 10b6d1a9, Python 3.11.15. Saved integration
record reports success=true, pre-integration HEAD
5286dabe645cdd1e8d82c28428e94a543c82419d, empty initial index, exact staging,
clean staged whitespace, commit and push all exit 0. Exact-session tool inventory
80377–80385 shows bounded document reads, the exact launcher with repository
workdir, one process wait and final report. No unrequested commands, source or
evidence edits, validation replay, extra Git mutations or post-push commands appear.

Accepted behavior: normal quit waits for supervisor completion once, permits the
approved reentrant quit, and keeps quit blocked after failure while attempting
one fixed native error box. Throwing dialogs are contained. Existing sandbox,
window and asynchronous wallet IPC behavior remains. Validation is 30 Electron
groups plus six preload groups, successful early-quit falsification and restored
30-group Electron green; all eight new authored cases completed.

This completes normal-quit composition, not broker startup or native wallet
availability. Next reviewer scope must resolve reviewed packaging-pin production,
platform artifact inventory, private user-data setup, startup failure/status
behavior, startup-versus-quit ordering and real application composition tests.
No runtime self-pinning or arbitrary executable selection is authorized.

The four unrelated npm/policy files remain modified; WAL-009 evidence and rejected
EXECUTABLE-GREEN-01 draft remain untracked. No actor or result is pending. Source,
execution and integration are closed until the next reviewer contract. Reviewer
ran no tests/builds; package/security failures are not waived and no release
acceptance is claimed.
