# Hermes — PAY publication preparation 01

CLOSED: candidate scan accepted in [review 20](../testing/BBD-PAY-001-MESSAGES-REVIEW-20.md).
Next authority: [publication and closeout](HERMES_BBD_PAY_001_PUBLICATION_01.md).
Instructions below are historical; no unchanged preparation rerun.
Actor: locally installed Hermes, owner-relayed. Reviewer: Codex.
Read AGENTS.md, TESTING.md, CURRENT_TASK, HERMES_JR_DEV_ROUTING.md and
[review 19](../testing/BBD-PAY-001-MESSAGES-REVIEW-19.md).
The reviewed feature/boundary checks are complete. This task captures the final candidate
bytes and their secret-scan result; it does not publish or waive inherited release blockers.

## Scope

Require HEAD `7a31c41cb29692a94acf1f24adb379f3a829d237`, master, empty index, and the
21 frozen review-05 inputs with review-15/18 overrides. Preserve unrelated dirty work.
Only report docs/testing/BBD-PAY-001-HERMES-PUBLICATION-PREP-01.md and new ignored
captures under dist/pay001-publication-prep01/ are writable. No source/doc edits beyond
that report, Git mutation, test/UI/audit rerun, build, dependency installation, live node,
wallet or user-process restart. Owner-reported monerod sync stays a planning input.

The exact candidate paths are [this literal list](BBD_PAY_001_PUBLICATION_PATHS_01.txt).
Copy no other files. In particular, no dirty Rust, agent-role documents, wallet tickets,
private profiles, credentials or generated binaries. The 17 source/input files include
the complete tested package and policy baseline for the reasons in review 19. This is
prospective source publication, not a claim of a fully green repository/release.

## Run once with automatic capture

Check disk type/free space. Create the new directory and save the exact Python block
below as executor.py there; execute `python3 dist/pay001-publication-prep01/executor.py`
from bb-desktop. If other capture files exist already, inspect/report rather than rerun.
No independent scanner invocation. Preserve the exact executor and all metadata/logs,
including failures. Correct only the declared provider/model strings if the real active
session differs; record the actual values. Stop on any other incompatibility.

The executor verifies the installed Gitleaks 8.30.1 identity, makes matching original/copy
manifests, records exact command argv/time/exit/log hashes and requires zero findings.
It checks the same bytes afterward. No metadata reconstruction or broad scan fallback.

```python
import datetime, hashlib, json, pathlib, re, shutil, subprocess
root = pathlib.Path.cwd()
cap = root / 'dist/pay001-publication-prep01'
assert root.name == 'bb-desktop' and cap.resolve() == cap and cap.is_dir()
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

list_path = root / 'docs/handoff/BBD_PAY_001_PUBLICATION_PATHS_01.txt'
paths = list_path.read_text().splitlines()
assert paths == sorted(set(paths)) and paths
snapshot = cap / 'snapshot'
scanner = root / 'target/security-tools/gitleaks-v8.30.1/gitleaks'
assert scanner.is_file() and not scanner.is_symlink()
assert scanner.stat().st_size == 21958840
assert digest(scanner) == '88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509'

def inventory(base):
    rows = []
    for rel in paths:
        pure = pathlib.PurePosixPath(rel)
        assert not pure.is_absolute() and '..' not in pure.parts
        p = base / rel
        assert p.is_file() and p.resolve() == p and not p.is_symlink(), rel
        rows.append(dict(path=rel, bytes=p.stat().st_size, sha256=digest(p)))
    return rows

before = gitfacts()
save('git-before.json', before)
assert before['head'].strip() == '7a31c41cb29692a94acf1f24adb379f3a829d237' and not before['index']
try:
    inputs = manifest()
    save('source-before.json', inputs)
    assert all(row['matches'] for row in inputs)
    save('actor-declared.json', dict(provider='nous', model='meituan/longcat-2.0:free', provenance='executor-declared'))
    save('scanner.json', dict(bytes=scanner.stat().st_size, sha256=digest(scanner)))
    save('path-list.json', dict(path=str(list_path.relative_to(root)), sha256=digest(list_path), count=len(paths)))
    source_rows = inventory(root)
    save('candidate-source.json', source_rows)
    for rel in paths:
        dest = snapshot / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / rel, dest)
    copied_rows = inventory(snapshot)
    save('candidate-copy.json', copied_rows)
    assert copied_rows == source_rows
    assert sorted(str(p.relative_to(snapshot)) for p in snapshot.rglob('*') if p.is_file()) == paths
    run('00-hermes-version', ['hermes', '--version'], 30, {0})
    version, _ = run('01-gitleaks-version', [str(scanner), 'version'], 30, {0})
    assert version.strip() == '8.30.1'
    run('02-candidate-scan', [str(scanner), 'dir', '--redact=100', '--no-banner', '--report-format', 'json', '--report-path', str(cap / 'gitleaks.json'), str(snapshot)], 180, {0})
    assert json.loads((cap / 'gitleaks.json').read_text()) == []
    source_after = inventory(root)
    copy_after = inventory(snapshot)
    save('candidate-source-after.json', source_after)
    save('candidate-copy-after.json', copy_after)
    assert source_after == source_rows == copy_after
    save('result.json', dict(candidate_paths=len(paths), candidate_manifest_sha256=digest(cap / 'candidate-copy.json'), findings=0, publication_authorized=False))
finally:
    final_inputs = manifest()
    save('source-after.json', final_inputs)
    after = gitfacts()
    save('git-after.json', after)
    assert all(row['matches'] for row in final_inputs)
    assert after == before, 'Git baseline/status changed during execution'
```

## Report and stop

Link the real captures, exact commands/exits, actor/tool identities, path count,
candidate manifest SHA-256 and findings. State which checks were NOT RUN on any early
failure. No secrets in prose; the scanner redacts values. No suppressions/source repairs.
Do not modify captured candidate files or listed governance after copying.

The new actor report is outside this candidate and must be covered in the eventual
publication closeout; do not claim it was scanned here. Reviewer will inspect candidate
identity and give the final exact-path Git authority. No commit, push or release is
authorized by this handoff. Earlier evidence limitations remain explicit; no passing
suite is repeated for documentation. Return the report pointer.
