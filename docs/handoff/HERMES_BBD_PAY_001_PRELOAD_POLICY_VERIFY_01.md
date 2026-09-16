# Hermes — PAY preload policy verification 01

CLOSED: correction accepted in [review 19](../testing/BBD-PAY-001-MESSAGES-REVIEW-19.md).
Next authority: [Hermes publication preparation](HERMES_BBD_PAY_001_PUBLICATION_PREP_01.md).
Instructions below are historical. No unchanged rerun.
Actor: locally installed Hermes Agent, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[source review 18](../testing/BBD-PAY-001-MESSAGES-REVIEW-18.md).
Sole execution authority. Old handoffs are closed; no actor launched.

## Scope

HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, empty index. Require the 21 review-05
pins with the driver override from review 15 and the two policy/test pins in review 18.
Preserve all unrelated dirty work. No checkout source/test edits, dependencies, policies,
Git mutation, UI/Node application suites, npm audit, external scans, native builds,
user-process restarts or live daemon/wallet operations. monerod availability stays recorded.

Writable report: docs/testing/BBD-PAY-001-HERMES-PRELOAD-POLICY-VERIFY-01.md.
New ignored directory: dist/pay001-policy-verify01/. Check disk type/free space first;
require disk-backed storage. Save the exact executor below as executor.py in that new
directory, then execute it once from bb-desktop with `python3 dist/pay001-policy-verify01/executor.py`.
If other capture files already exist, inspect/report; do not overwrite/rerun. Python is
capture orchestration; it invokes only the fixed commands below. No actor-authored tests.

The executor records actual timestamps, exact timeout argv, observed wrapper statuses,
log hashes, before/after source/Git identities and isolated-copy identities. Keep all
files, including on failure. Never replace metadata with handwritten estimates or future
results. If execution tooling fails, stop and document the observed failure/missing files;
do not run the underlying commands separately or reconstruct records afterward.

## Execution order and interpretation

1. Two source syntax checks, then the existing boundary group and two PAY groups: 3/3 green.
2. Copy only checker, test suite and unchanged preload to an isolated three-file tree.
   Remove only the two reviewed channel-list lines from the COPY, requiring the exact
   old policy hash. Run the new positive group: expected exit 1 at list comparison.
   Separately run the copied checker on the copied real preload: expected exit 1 with
   dynamic or unlisted IPC invoke. Do not mistake missing files/loader errors for red.
3. Restore the copy byte-for-byte, run all three focused groups there: 3/3 green.
4. Run the real repository checker and full policy suite once. Their 0/1 exits are
   recorded for diagnosis. Expected remaining red is inventory-related, not PAY IPC.

The full suite has 94 groups; expected inherited result is 90 pass / 4 fail. Compare
failure names AND root causes against review 17 and the older extractor review. A new
failure, syntax failure, unexpected falsification result or drift blocks acceptance.
No repair/rerun authority. The copy is always restored; no real source is mutated.

## Exact capture executor

Save this block verbatim; it is orchestration around the already-authored tests and the
reviewed two-line isolated falsification. Do not execute it in the reviewer role.

```python
import datetime, hashlib, json, pathlib, re, shutil, subprocess

root = pathlib.Path.cwd()
cap = root / 'dist/pay001-policy-verify01'
assert root.name == 'bb-desktop'
assert cap.resolve() == cap and cap.is_dir()
assert {p.name for p in cap.iterdir()} == {'executor.py'}
assert subprocess.check_output(['stat', '-f', '-c', '%T', str(cap)], text=True).strip() not in ('tmpfs', 'ramfs')

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def save(name, value):
    (cap / name).write_text(json.dumps(value, indent=2) + '\n')

def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def gitfacts():
    return {key: subprocess.check_output(argv, cwd=root, text=True) for key, argv in [
        ('head', ['git', 'rev-parse', 'HEAD']),
        ('index', ['git', 'diff', '--cached', '--name-only']),
        ('status', ['git', 'status', '--short'])]}

pins = {p: (h, int(n)) for p, h, n in re.findall(
    r'^\| ([\w./-]+) \| ([0-9a-f]{64}) \| (\d+) \|',
    (root / 'docs/testing/BBD-PAY-001-MESSAGES-REVIEW-05.md').read_text(), re.M)}
pins.update({
    'test/paymentInbox.electron.js': ('f09c2dcc214b4fb97699d77eecf7407391cdcff5d1785cf850a1d59455abaf54', 1599),
    'scripts/security-policy.js': ('c5431ee340564af26f0d3d2f5f19a3aa72869a45d9b3b0d64353a7da266732cb', 2808),
    'test/securityPolicy.node.js': ('29e4e213c803ca7792fae51cff99aaa61f96abdf43830157fda50c89292799fd', 3922)})
assert len(pins) == 21

def manifest():
    rows = []
    for rel, (expected, lines) in sorted(pins.items()):
        p = root / rel
        assert p.is_file() and not p.is_symlink()
        rows.append(dict(path=rel, sha256=digest(p), lines=len(p.read_bytes().splitlines()),
                         matches=digest(p) == expected and len(p.read_bytes().splitlines()) == lines))
    return rows

def run(label, command, limit, expected, cwd=root):
    argv = ['timeout', '--signal=TERM', '--kill-after=5s', str(limit) + 's'] + command
    out, err = cap / (label + '.stdout.log'), cap / (label + '.stderr.log')
    meta = dict(argv=argv, cwd=str(cwd), environment={}, started_utc=now(), ended_utc=None,
                wrapper_exit_code=None, signal=None, timeout_status=None, child_exit_code=None)
    save(label + '.metadata.json', meta)
    try:
        with out.open('xb') as stdout, err.open('xb') as stderr:
            result = subprocess.run(argv, cwd=cwd, stdout=stdout, stderr=stderr)
        meta.update(wrapper_exit_code=result.returncode,
                    signal=-result.returncode if result.returncode < 0 else None,
                    timeout_status='timeout-or-forced-termination' if result.returncode in (124, 137) or result.returncode < 0 else 'completed')
    except Exception as error:
        meta['executor_error'] = repr(error)
        raise
    finally:
        meta['ended_utc'] = now()
        for kind, p in [('stdout', out), ('stderr', err)]:
            if p.exists():
                meta[kind + '_path'] = str(p.relative_to(root))
                meta[kind + '_sha256'] = digest(p)
        save(label + '.metadata.json', meta)
    assert result.returncode in expected, (label, result.returncode)
    return out.read_text(), err.read_text()

names = [
    'wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC',
    'PAY-001 preload policy accepts the exact seven channels and real bridge',
    'PAY-001 preload policy rejects missing duplicate computed and unlisted channels']

def focused(selected):
    return r"""const assert = require('assert');
const { tests } = require('./test/securityPolicy.node.js');
const names = NAMES;
assert.strictEqual(tests.length, 94);
let failed = 0;
for (const name of names) {
  const found = tests.filter(test => test.name === name);
  assert.strictEqual(found.length, 1, name);
  try { found[0].fn(); process.stdout.write('ok ' + name + '\n'); }
  catch (error) { failed++; process.stderr.write('not ok ' + name + '\n' + error.stack + '\n'); }
}
process.stdout.write((names.length - failed) + ' passed / ' + failed + ' failed\n');
process.exitCode = failed ? 1 : 0;
""".replace('NAMES', json.dumps(selected))

copy = cap / 'isolated'
copy_paths = ['scripts/security-policy.js', 'test/securityPolicy.node.js', 'wallet-preload.js']
def copy_manifest():
    return [dict(path=rel, sha256=digest(copy / rel)) for rel in copy_paths]

before = gitfacts()
save('git-before.json', before)
assert before['head'].strip() == '7a31c41cb29692a94acf1f24adb379f3a829d237' and not before['index']
original_policy = None
try:
    inputs = manifest()
    save('source-before.json', inputs)
    assert all(row['matches'] for row in inputs)
    save('actor-declared.json', dict(provider='nous', model='meituan/longcat-2.0:free',
                                   provenance='executor-declared; verify against actual active session'))
    run('00-hermes-version', ['hermes', '--version'], 30, {0})
    run('01-node-version', ['node', '--version'], 30, {0})
    run('02-policy-syntax', ['node', '--check', 'scripts/security-policy.js'], 30, {0})
    run('03-test-syntax', ['node', '--check', 'test/securityPolicy.node.js'], 30, {0})
    run('04-focused-green', ['node', '-e', focused(names)], 60, {0})
    for rel in copy_paths:
        dest = copy / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / rel, dest)
        assert digest(dest) == digest(root / rel)
    save('copy-before.json', copy_manifest())
    copied_policy = copy / 'scripts/security-policy.js'
    original_policy = copied_policy.read_bytes()
    needle = b"  'payment:inbox:get',\n  'payment:inbox:connect',\n"
    assert original_policy.count(needle) == 1
    copied_policy.write_bytes(original_policy.replace(needle, b'', 1))
    assert digest(copied_policy) == '0e971da11175c1abc4f081a9f03ac603df9d30e449f2b707dee90e6c41813e8d'
    save('copy-falsified.json', copy_manifest())
    run('05-falsified-positive', ['node', '-e', focused([names[1]])], 60, {1}, copy)
    direct = "const fs=require('fs'); const p=require('./scripts/security-policy.js'); p.checkWalletBoundarySource(fs.readFileSync('wallet-preload.js','utf8'),'wallet-preload.js');"
    _, error_text = run('06-falsified-real-preload', ['node', '-e', direct], 60, {1}, copy)
    assert 'dynamic or unlisted IPC invoke' in error_text
    copied_policy.write_bytes(original_policy)
    save('copy-restored.json', copy_manifest())
    assert all(digest(copy / rel) == digest(root / rel) for rel in copy_paths)
    run('07-restored-focused', ['node', '-e', focused(names)], 60, {0}, copy)
    run('08-repository-policy', ['node', 'scripts/security-policy.js'], 120, {0, 1})
    run('09-policy-suite', ['node', 'test/securityPolicy.node.js'], 180, {0, 1})
finally:
    if original_policy is not None:
        (copy / 'scripts/security-policy.js').write_bytes(original_policy)
        save('copy-final.json', copy_manifest())
    final_inputs = manifest()
    save('source-after.json', final_inputs)
    after = gitfacts()
    save('git-after.json', after)
    assert all(row['matches'] for row in final_inputs)
    assert after == before, 'Git baseline/status changed during execution'
```

The actor declaration in this block reflects the last reported routing. If actual active
provider/model differs, correct only those two declaration strings before execution and
record the actual values/source in the report. No other executor alteration is authorized;
stop on incompatibility. Report observed counts/reasons and all artifact paths/hashes.
If the executor stops early, later checks are NOT RUN, not inferred passes. Keep old
execution gaps explicit; do not manufacture missing old records. Reviewer accepts the
results and controls final security/publication scope after this report.
