# WAL-011 executable integration 01

Actor: Hermes, integration only. Reviewer publication follows 85b2820d; one
CURRENT-only launch commit may follow. Source actors and all test/build execution
are closed. Read active CURRENT prefix and this handoff. Do not reload history.

Accepted validation: seven stages in the saved Resume-01 gate, with the mutant test
failing as intended and all nine restored-runtime groups passing. No rerun needed.
The source/green evidence identities are pinned below via the reviewed raw record.

Only source/test integration paths:
- wallet-broker/src/main.rs
- wallet-broker/src/runtime.rs
- test/walletBrokerRuntime.node.js

Only evidence integration paths:
- docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md (regenerate from exact saved tool output)
- docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-RESUME-01.md (unchanged)

The rejected green-01 draft, WAL-009 evidence and four pending npm/policy files stay
untracked/unstaged or modified as found. Do not edit them. No binaries/raw target
artifacts, CURRENT, ticket or other paths may be committed by this actor.

Submit this short launcher exactly once from the repository root using terminal
background=true, notify_on_complete=true. Wait only on that process, at most 60
seconds per wait. Do not retype the driver, run independent preflight commands,
write evidence yourself or rerun after an error. Git push is the only authorized
network operation, using the existing remote without force. The driver generates
current runtime metadata from one exact environment key and its database row.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_INTEGRATION_01.md").read_text(); code=s.split("```python\n# WAL011_INTEGRATION_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_INTEGRATION_01_DRIVER","exec"))'
```

The driver verifies all eight source hashes and accepted green evidence, reads only
the original red session's exact command/result/version and three metadata fields,
regenerates the corrected red record, stages exactly five paths, checks staged
whitespace/path scope, commits and pushes. It records actual command outputs to
wallet-broker/target/wal011-executable-integration-01.json. No post-push commands.
On any failure, stop without retry/repair or unstaging; report actual driver output.

```python
# WAL011_INTEGRATION_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
report_path = Path('wallet-broker/target/wal011-executable-integration-01.json')
assert not report_path.exists(), 'integration record exists; no replay'
report = {'commands': [], 'success': False}
paths = ['wallet-broker/src/main.rs', 'wallet-broker/src/runtime.rs', 'test/walletBrokerRuntime.node.js', 'docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md', 'docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-RESUME-01.md']
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def command(argv, timeout=60):
    p = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout)
    row = {'argv': argv, 'exit': p.returncode, 'output': p.stdout}
    report['commands'].append(row)
    print(json.dumps(row), flush=True)
    assert p.returncode == 0, 'command failed; stop'
    return p.stdout
try:
    report['version'] = command(['hermes', '--version']).strip()
    report['head'] = command(['git', 'rev-parse', 'HEAD']).strip()
    report['initial_status'] = command(['git', 'status', '--short'])
    assert not command(['git', 'diff', '--cached', '--name-only']).strip(), 'index must start empty'
    raw_path = Path('wallet-broker/target/wal011-executable-green-resume-01.json')
    assert sha(raw_path) == '7994b2db6cac49a1e03c99533828617d9f4204f6e2dce9132c5b9f3bfe9ed404', 'accepted raw record changed'
    raw = json.loads(raw_path.read_text())
    assert raw['success'] and raw['driver_exit'] == 0
    for path, digest in raw['final_hashes'].items():
        assert sha(path) == digest, 'source identity mismatch: '+path
    assert sha(paths[4]) == 'ed7b7b3c437c21e027805c627b9e447927da5ca5c95c5fb30f0fa6891e1b451e', 'green evidence changed'
    assert sha(paths[3]) == '2f6e74fdcdbdf5c1018e0c00addc9dbc611d77d73f78de7be5d584d55ec2bac5', 'red draft changed'
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'current session ID absent; do not discover'
    red_sid = '20260909_164019_e64925'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        current = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?',(sid,)).fetchone()
        red_meta = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?',(red_sid,)).fetchone()
        assert current and all(current) and red_meta and all(red_meta), 'session metadata absent'
        def saved(mid, column):
            assert column in ('content','tool_calls')
            row = db.execute('SELECT '+column+' FROM messages WHERE session_id=? AND id=?',(red_sid,mid)).fetchone()
            assert row and row[0], 'saved red input absent'
            return json.loads(row[0])
        red_result = saved(80263, 'content')
        red_calls = saved(80262, 'tool_calls')
        red_version_baseline = saved(80253, 'content')
    report['session'] = dict(zip(('id','model','provider'),current))
    red_command = json.loads(red_calls[0]['function']['arguments'])['command']
    assert red_command == 'node test/walletBrokerRuntime.node.js 2>&1'
    assert red_result['exit_code'] == 1
    lines = red_result['output'].splitlines()
    assert sum(line.startswith('not ok ') for line in lines) == 9
    assert sum(line.startswith('ok ') for line in lines) == 0
    assert sum(line == 'Error: native wallet broker executable is missing' for line in lines) == 9
    corrected = {'session': dict(zip(('id','model','provider'),red_meta)), 'version_and_baseline_tool_result': red_version_baseline, 'actual_command': red_command, 'result': red_result, 'counts': {'ok': 0, 'not_ok': 9}, 'frozen_input_hashes': raw['input_hashes']}
    # Only the three inputs below existed in the red command's scope.
    corrected['frozen_input_hashes'] = {p: raw['input_hashes'][p] for p in ('test/walletBrokerRuntime.node.js','wallet-broker/supervisor.js','wallet-broker/protocol.js')}
    normalized = json.dumps(corrected, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    note = ('# WAL-011 executable expected red — corrected evidence\n\n'
            'Regenerated during integration from original session messages 80253, 80262 and 80263. The original draft omitted or changed stack frames and denied observed deviations. This record supersedes that draft without rerunning the test.\n\n'
            'Reviewer accepted the actual exit-1/nine-failure missing-executable result. Actual command included 2>&1. The actor also performed forbidden session-environment discovery, extra Git/history/source reads and a binary existence check, and used execute_code instead of the prescribed metadata command. See the collected review in the executable-red handoff. No credential values are reproduced. The missing-binary branch did not exercise child cleanup; no independent resource validation is claimed.\n\n')
    Path(paths[3]).write_text(note+'```json\n'+normalized+'\n```\n')
    report['integrated_path_hashes'] = {p: sha(p) for p in paths}
    report['line_counts'] = {p: len(Path(p).read_bytes().splitlines()) for p in paths}
    for path, digest in raw['final_hashes'].items():
        assert sha(path) == digest, 'source changed before staging'
    command(['git', 'add', '--']+paths)
    staged = command(['git', 'diff', '--cached', '--name-only']).splitlines()
    assert sorted(staged) == sorted(paths), 'unexpected staged paths; stop'
    command(['git', 'diff', '--cached', '--check'])
    command(['git', 'diff', '--cached', '--stat'])
    command(['git', 'commit', '-m', 'Add validated degraded Rust wallet broker executable'])
    command(['git', 'push'], timeout=120)
    report['success'] = True
except Exception as exc:
    report['stop'] = str(exc)
finally:
    report_path.write_text(json.dumps(report, indent=2)+'\n')
    print('INTEGRATION_RECORD='+str(report_path), flush=True)
    print('INTEGRATION_SUCCESS='+str(report['success']), flush=True)
sys.exit(0 if report['success'] else 1)
```


## Collected integration acceptance — 2026-09-09

Decision: accept the five-path Rust executable integration at
31a6e54095a0b5519f8ce0b03833ac8862ddf32f. Reviewer read-only Git inspection confirms
HEAD and origin/master both identify that commit. All five files are newly tracked;
the actor's final phrase “+3 new, +2 modified” was inaccurate. The raw Git result
correctly records five created files and 1887 insertions. No source correction or
validation replay is required. This accepts the degraded executable stage, not
Electron startup, usable wallets or release readiness.

Outer 93493 was collected on owner done with exit 0. Exact runtime session
20260909_174555_2c6762 is nous / poolside/laguna-s-2.1:free, Hermes v0.18.2
(2026.7.7.2), upstream 8e85b276/local 10b6d1a9, Python 3.11.15. The saved
wallet-broker/target/wal011-executable-integration-01.json records success=true,
pre-integration HEAD b49fb5a22961537d7259be3f47f8e3ff6f197297, empty index,
exact five-path staging, successful whitespace check, commit and push, all exit 0.
The reviewer checked the exact session/message inventory: initial CURRENT read and
handoff read, one extraction launcher, one process wait, then final report. The
initial terminal command unnecessarily read all CURRENT history and added a missing-
file fallback. Record that read-scope deviation; no extra execution, edits, tests,
builds, Git mutations or post-push commands appear in the audited tool inventory.

Committed SHA-256 identities independently match the raw integration record:

| Path | SHA-256 |
| --- | --- |
| wallet-broker/src/main.rs | 19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d |
| wallet-broker/src/runtime.rs | 968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b |
| test/walletBrokerRuntime.node.js | a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e |
| docs/testing/BBD-WAL-011-EXECUTABLE-RED-01.md | 9a1087d5fd2c8dbb014563eabba12765c85a111c77862774cb6c66b8c7d608b4 |
| docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-RESUME-01.md | ed7b7b3c437c21e027805c627b9e447927da5ca5c95c5fb30f0fa6891e1b451e |

The corrected red record's result equals original message 80263 after explicit
repository/home path normalization. Green evidence retains the accepted seven-stage
raw record, nine passing runtime groups and session-check falsification. Unrelated
npm/policy changes, WAL-009 evidence and rejected GREEN-01 draft remain pending.
Hermes integration authorization is closed. The reviewer executed no tests/builds.
