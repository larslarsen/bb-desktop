# WAL-011 executable focused validation 01

Actor: Hermes, execution/evidence only; no integration or source repair.
Protected parent: reviewer publication following 5118d7a6; one CURRENT-only launch
commit may follow. Read AGENTS.md, TESTING.md, active CURRENT prefix, ticket, this
handoff and the final source-acceptance section only. No historical reload.

Resume the already known session 20260909_164019_e64925. Record `hermes --version`,
`git rev-parse HEAD` and `git status --short`. Use this literal metadata query once:

```bash
python3 - <<'WAL011_KNOWN_SESSION'
from pathlib import Path
import json, sqlite3
with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as con:
    rows = con.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?', ('20260909_164019_e64925',)).fetchall()
assert len(rows)==1 and all(rows[0]), 'known session metadata missing'
print(json.dumps(dict(zip(('session_id','model','provider'), rows[0]))))
WAL011_KNOWN_SESSION
```

No ID rediscovery, environment/proc/config/auth/status dumps, history commands or
other-session queries. If this invocation reports a different session ID, or the
query fails, record the mismatch and stop before execution. Do not search or retry.

## Exact validation command

Run the following command once, unchanged, from the repository root. The existing
wallet-broker/target is ext4 and stores all substantial build artifacts and saved
output. The command validates eight source identities, runs pinned formatting only
on the binary/module, builds offline, lints only the binary, compiles a temporary
session-check mutant, proves the wrong-session test rejects it, restores the exact
formatted source and rebuilds, then runs the full nine-group runtime suite.

Formatter normalization of main.rs and runtime.rs is explicitly authorized here;
no other source mutation except the automatically restored exact mutant is allowed.
The accepted test remains frozen. The raw JSON retains every stage's complete
output and actual exit code. Never rerun the command after a stop. Tool waiting
for the same running command is allowed; starting another gate is not.

```bash
python3 - <<'WAL011_EXECUTABLE_GATE'
from pathlib import Path
import hashlib, json, subprocess, sys
root = Path.cwd()
record = {'stages': [], 'success': False}
expected = {
 'wallet-broker/src/main.rs': '19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d',
 'wallet-broker/src/runtime.rs': 'c579bf54af9e17abc2eb49b42c9a944cd6ec2b6be1e5d900963564f27ecd86e9',
 'test/walletBrokerRuntime.node.js': 'a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e',
 'wallet-broker/supervisor.js': '1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8',
 'wallet-broker/protocol.js': '79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4',
 'wallet-broker/Cargo.toml': '73e5e585eb2fd1ca867d66962460cfa010443886a4b59d8a33556ef2a4ff3503',
 'wallet-broker/Cargo.lock': 'b960bc39d9bd32319a59b0dea66817ef926754ad46340a8d5e9fc07522b28b71',
 'wallet-broker/src/lib.rs': '08dd09d23a8c18cdb9a50968ade153a2118b60132f2b7b66a36c6913596de925',
}
def hashes():
 return {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in expected}
def stage(name, argv, timeout=1200, want=0):
 try:
  result = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=timeout)
  entry = {'name': name, 'argv': argv, 'exit': result.returncode, 'output': result.stdout}
 except subprocess.TimeoutExpired as exc:
  output = exc.stdout or b''
  if isinstance(output, bytes): output = output.decode('utf8', errors='replace')
  entry = {'name': name, 'argv': argv, 'exit': None, 'output': output, 'timeout': True}
 record['stages'].append(entry)
 print(name + ': exit=' + str(entry['exit']), flush=True)
 assert entry['exit'] == want, name + ': unexpected result; stop'
 return entry['output']
rustup = str(Path.home()/'.cargo/bin/rustup')
build = [rustup, 'run', '1.98.0', 'cargo', 'build', '--manifest-path', 'wallet-broker/Cargo.toml', '--locked', '--offline', '--no-default-features', '--bin', 'bitbook-wallet-broker']
runtime = Path('wallet-broker/src/runtime.rs')
raw_path = Path('wallet-broker/target/wal011-executable-green-01.json')
original = None
try:
 record['input_hashes'] = hashes()
 assert record['input_hashes'] == expected, 'source identity mismatch'
 assert raw_path.parent.is_dir(), 'existing target directory missing'
 assert not raw_path.exists(), 'gate record already exists; no replay'
 stage('format', [rustup, 'run', '1.98.0', 'rustfmt', '--edition', '2024', 'wallet-broker/src/main.rs'])
 record['formatted_hashes'] = hashes()
 assert all(record['formatted_hashes'][p]==h for p,h in expected.items() if p not in ('wallet-broker/src/main.rs','wallet-broker/src/runtime.rs')), 'formatter changed frozen input'
 stage('build', build)
 stage('clippy', [rustup, 'run', '1.98.0', 'cargo', 'clippy', '--manifest-path', 'wallet-broker/Cargo.toml', '--locked', '--offline', '--no-default-features', '--bin', 'bitbook-wallet-broker', '--', '-D', 'warnings'])
 original = runtime.read_bytes()
 needle = b'|| session != session_id'
 assert original.count(needle)==1, 'unique session check missing'
 try:
  runtime.write_bytes(original.replace(needle, b'|| (session != session_id && false)'))
  stage('mutant-build', build)
  js = """const tests = require('./test/walletBrokerRuntime.node.js').tests;
const selected = tests.filter(t => t.name === 'direct: wrong session, sequence, kind, and duplicate ids terminate promptly');
if (selected.length !== 1) throw new Error('missing unique session test');
selected[0].fn().then(() => { console.error('UNEXPECTED MUTANT PASS'); process.exitCode=0; }, error => { console.error(error.stack || error); process.exitCode=1; });"""
  output = stage('session-falsification', ['node', '-e', js], timeout=20, want=1)
  assert 'wrong session did not terminate promptly' in output, 'mutant failed for another cause'
 finally:
  runtime.write_bytes(original)
  assert runtime.read_bytes()==original, 'source restoration failed'
  record['restored_runtime_sha256'] = hashlib.sha256(original).hexdigest()
  stage('restored-build', build)
 output = stage('runtime-green', ['node', 'test/walletBrokerRuntime.node.js'], timeout=60)
 lines = output.splitlines()
 assert sum(line.startswith('ok ') for line in lines)==9 and not any(line.startswith('not ok ') for line in lines), 'unexpected runtime counts'
 record['success'] = True
except Exception as exc:
 record['stop'] = str(exc)
finally:
 if original is not None and runtime.read_bytes()!=original:
  runtime.write_bytes(original)
  record['success'] = False
  record['stop'] = 'unexpected late source drift restored; binary validation invalid'
 record['final_hashes'] = hashes()
 if 'formatted_hashes' in record and record['final_hashes'] != record['formatted_hashes']:
  record['success'] = False
  record['stop'] = 'final source identities differ from formatted baseline'
 if not raw_path.exists():
  raw_path.write_text(json.dumps(record, indent=2)+'\n')
 print('GATE_RECORD=' + str(raw_path), flush=True)
 print('GATE_SUCCESS=' + str(record['success']), flush=True)
sys.exit(0 if record['success'] else 1)
WAL011_EXECUTABLE_GATE
```

## Evidence and stop

After the command, read its one saved JSON record, run `git status --short` once,
and write only docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md. Generate its stage
results programmatically from the JSON: preserve exact argv, exits, full outputs,
hashes and stop reason; sanitize local absolute path prefixes. Include version,
known actual session/provider/model, source-review and observed HEAD. Record skipped
stages as unrun. Do not reconstruct stack traces or infer success from stage names.
If the gate stops, report the first unexpected outcome and stop without repair.

No extra commands, gate retries, independent suites, crypto proof replay, package
policy/audit/scanners, network tools, Git mutation, actors or broad discovery.
The previous red evidence remains frozen for correction during later integration.
This task does not claim app startup, native/account composition, custody operation,
packaging or release acceptance. Stop after the single validation record.

## Collected stop and Resume 01 — 2026-09-09

Outer 99713 returned exit 0 on owner done. Actual session was
20260909_172157_d12849, nous / poolside/laguna-s-2.1:free. Resuming the prior session
created this new session without a parent_session_id link. The reviewer's assumption
that --resume retained the old ID was wrong. The actor queried the old row and
reported that identity instead of stopping on the active-session mismatch.

Saved command 80276 was refused by the terminal tool because foreground timeout
1800 exceeds its 600-second maximum; it did not execute. Submission 80278 launched
the gate once in background. Result 80281 shows exit 1 before any stage. Both
submitted copies replace the lockfile pin's `...59b0dea...` with `...59b0dfa...`.
This is an actor transcription error, not source drift or an error in the published
pin. Reviewer statically extracted all eight published pins and verified every one
against source. The raw JSON's input/final identities match; stages is empty.

Reject the report's "unchanged command" claim. Its alleged full-raw JSON also
contains the wrong lock hash, while the real raw artifact contains the correct one.
The hand-written report misspells Hermes as Hermitian, poolside as poolsime and the
database path as .hermus. Actual version output 80274 is Hermes Agent v0.18.2
(2026.7.7.2), upstream 8e85b276, local 10b6d1a9. The wrapper/redirection on that
version query was not prescribed. No formatter, build, lint, mutant or runtime test
ran, and there was no source mutation or restoration. Retain the old draft/raw
record as a rejected historical stop; do not rewrite or delete either in this task.

Resume 01 is authorized from the reviewer publication following 5122eee0, allowing
one CURRENT-only launch commit. Source pins and gate semantics are unchanged.
Use a fresh Hermes invocation. The driver below reads only the single exact
HERMES_SESSION_ID environment key and its corresponding database row. This narrow
read is now explicitly authorized, replacing the stale-ID requirement above.
No environment enumeration, echo/printenv, proc/config/auth discovery or alternative
metadata lookup is authorized. Version, HEAD/status, metadata and evidence are
generated by the driver, not copied by the actor.

Only execution command: submit this short launcher exactly once using terminal
background=true and notify_on_complete=true. Wait on that same process in intervals
no longer than 60 seconds. Do not copy, edit or retype the embedded gate or driver.

```bash
python3 -c 'from pathlib import Path; s=Path("docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_GREEN_01.md").read_text(); code=s.split("```python\n# WAL011_RESUME_01_DRIVER\n",1)[1].split("\n```",1)[0]; exec(compile(code,"WAL011_RESUME_01_DRIVER","exec"))'
```

The driver authorizes the same pinned formatter normalization and restored mutant
as the original gate. It changes only the raw/evidence output filenames. Its
preflight and postflight commands and single-key metadata read are included in this
authorization. Do not independently repeat them. It writes
wallet-broker/target/wal011-executable-green-resume-01.json and
docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-RESUME-01.md, then stops. No Git mutation,
source repair, gate replay or other evidence write. If it fails, report its actual
stop; no further execution. This remains validation only, not integration.

```python
# WAL011_RESUME_01_DRIVER
from pathlib import Path
import hashlib, json, os, sqlite3, subprocess, sys
doc = Path('docs/handoff/HERMES_BBD_WAL_011_EXECUTABLE_GREEN_01.md').read_text()
gate = doc.split("python3 - <<'WAL011_EXECUTABLE_GATE'\n", 1)[1].split('\nWAL011_EXECUTABLE_GATE', 1)[0]
assert hashlib.sha256(gate.encode()).hexdigest() == '4ca3bb863c7983eaa3daf612af67d1f132cd7b4f5e1af7494eee586d82b416d4', 'published gate text changed'
raw = Path('wallet-broker/target/wal011-executable-green-resume-01.json')
evidence = Path('docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-RESUME-01.md')
assert raw.parent.is_dir() and not raw.exists() and not evidence.exists(), 'resume output exists or target missing; stop'
old = 'wallet-broker/target/wal011-executable-green-01.json'
assert gate.count(old) == 1
gate = gate.replace(old, str(raw))
metadata = {'preflight': []}
result = {'stages': [], 'success': False}
exit_code = 1
def observe(argv):
    p = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=30)
    row = {'argv': argv, 'exit': p.returncode, 'output': p.stdout}
    metadata['preflight'].append(row)
    assert p.returncode == 0, 'preflight command failed'
    return p.stdout
try:
    metadata['version'] = observe(['hermes', '--version']).strip()
    metadata['head'] = observe(['git', 'rev-parse', 'HEAD']).strip()
    metadata['initial_status'] = observe(['git', 'status', '--short'])
    sid = os.environ.get('HERMES_SESSION_ID')
    assert sid, 'current session ID missing; no rediscovery authorized'
    with sqlite3.connect((Path.home()/'.hermes'/'state.db').as_uri()+'?mode=ro', uri=True) as db:
        rows = db.execute('SELECT id,model,billing_provider FROM sessions WHERE id=?', (sid,)).fetchall()
    assert len(rows) == 1 and all(rows[0]), 'current session metadata unavailable'
    metadata['session'] = dict(zip(('id','model','provider'), rows[0]))
    try:
        exec(compile(gate, 'WAL011_EXECUTABLE_GATE_RESUME_01', 'exec'), {'__name__': '__main__'})
    except SystemExit as stopped:
        exit_code = stopped.code if isinstance(stopped.code, int) else 1
    assert raw.exists(), 'gate did not produce its raw record'
    result = json.loads(raw.read_text())
    assert bool(result.get('success')) == (exit_code == 0), 'gate exit/record mismatch'
except Exception as exc:
    if raw.exists():
        result = json.loads(raw.read_text())
    result['success'] = False
    result['driver_stop'] = str(exc)
    exit_code = 1
finally:
    try:
        metadata['final_status'] = observe(['git', 'status', '--short'])
    except Exception as exc:
        result['success'] = False
        result['driver_stop'] = str(exc)
        exit_code = 1
    result['metadata'] = metadata
    result['driver_exit'] = exit_code
    raw.write_text(json.dumps(result, indent=2)+'\n')
    normalized = json.dumps(result, indent=2).replace(str(Path.cwd()), '<repo>').replace(str(Path.home()), '<home>')
    title = '# WAL-011 executable validation Resume 01\n\n'
    description = 'Generated from the actual driver/gate result. Earlier green-01 is a rejected zero-stage stop. This record does not claim integration or app/release acceptance.\n\n'
    evidence.write_text(title+description+'```json\n'+normalized+'\n```\n')
    print('RESUME_EVIDENCE='+str(evidence), flush=True)
    print('RESUME_SUCCESS='+str(result['success']), flush=True)
sys.exit(exit_code)
```

## Resume 01 acceptance — 2026-09-09

Accept the executed runtime boundary. Outer 92827 collected on owner done, exit 0.
Actual session 20260909_173905_d15995, nous / poolside/laguna-s-2.1:free; Hermes
v0.18.2 (2026.7.7.2), upstream 8e85b276, local 10b6d1a9. Exact-session tool inventory
shows one handoff read, one unchanged short launcher (80291), one process wait and
completion (80294), with no other actor commands or edits. The actor read the full
handoff instead of only the requested section; execution remained bounded.

Format, initial build, warning-denied Clippy, mutant build and restored build all
exit 0. The wrong-session mutant exits the selected test with 1 and exactly
`wrong session did not terminate promptly`; source restoration matches the formatted
runtime hash before rebuilding. All nine full runtime groups pass, zero failures.
This is actual compiled-Rust transport, degraded status and rejection behavior;
it is not app startup, native UI, connected accounts, custody or release acceptance.

Final measured inputs:
- main.rs: 7 lines, 19b1651d88d85e5d968597eb26eeb62f76a11af56eaa311d60a55d3ef282736d
- runtime.rs: 695 lines, 968194aadd609f54ef28a11a090fcb1ca840cfc851da127456dd5b356437305b
- runtime test: 947 lines, a49c3e0c49ff664997f78222d979e20603d9a2e9985735c6aa07c5c02308f39e
- all other frozen inputs retain their accepted identities.

Raw JSON SHA-256:
7994b2db6cac49a1e03c99533828617d9f4204f6e2dce9132c5b9f3bfe9ed404.
The 203-line green-resume evidence has SHA-256
ed7b7b3c437c21e027805c627b9e447927da5ca5c95c5fb30f0fa6891e1b451e
and exactly equals the raw JSON after the specified path normalization. Reviewer
verified the actual session row and eight final source identities independently.
No gates were rerun during review. The formatted source and green evidence are
accepted for the exact integration handoff. The older red draft requires the
already reviewed saved-output correction during that integration; the rejected
green-01 draft stays untracked. No other work is authorized.
