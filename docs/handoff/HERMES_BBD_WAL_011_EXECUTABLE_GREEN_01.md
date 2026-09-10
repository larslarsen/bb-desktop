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
