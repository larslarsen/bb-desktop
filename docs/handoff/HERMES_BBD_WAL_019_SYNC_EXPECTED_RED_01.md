# Hermes — WAL-019 Sync expected red 01

Actor: Hermes, using a free Nous Portal model, manually relayed by the owner.
Read AGENTS.md, TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md,
CURRENT_TASK.md, tickets/BBD-WAL-019.md and the accepted source review
docs/testing/BBD-WAL-019-TEST-SOURCE-REVIEW-02.md.

This authorizes one formal expected-red capture, not source changes or Git.
Do not reintegrate or reformat the already-present test drop. No earlier executor
handoff is active. Record actual Hermes version, resolved provider/model and session
identifier; do not substitute the historical adoption model for the active model.

## Frozen inputs and writable outputs

Require HEAD `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` and all seven pins below.
Preserve unrelated unpublished WAL-009, npm/policy, governance and other work.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/account_ui.rs` | 1281 | `cbfaf77b02ea725422426f2b1276e738308e4a110654e56fa05f2008070b6e0e` |
| `wallet-broker/tests/account_native_ui.rs` | 2654 | `e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446` |
| `wallet-broker/Cargo.toml` | 126 | `435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b` |
| `wallet-broker/Cargo.lock` | 5771 | `a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a` |
| `wallet-broker/src/accounts.rs` | 1006 | `8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d` |
| `wallet-broker/src/zec/live.rs` | 1238 | `17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f` |
| `wallet-broker/src/zec/live_transport.rs` | 214 | `a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e` |

New capture directory: `wallet-broker/target/wal019-sync-expected-red-01`.
New report: `docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md`.
Those outputs must not preexist. Existing disk-backed Cargo target/cache writes
are permitted. No source, lockfile, other evidence or control-plane edits. Do not
access user wallet data, call public services, fetch dependencies, build a release,
run broader suites, launch other actors, commit or push.

## Exact capture command

Run this once from bb-desktop. The driver checks pins and disk type, captures actual
version/command outputs and exits, stops on formatter failure, and runs only the
six selected tests. The GNU timeout process owns test termination. Do not detach,
restart or manually kill the runner because a tool wait yields; keep polling the
same tool session. If the driver fails, preserve its partial capture and report the
actual failure; no retry or alternate command is authorized by this handoff.

```bash
python3 - <<'PY'
import datetime, hashlib, json, pathlib, re, subprocess

root = pathlib.Path.cwd()
handoff = root / 'docs/handoff/HERMES_BBD_WAL_019_SYNC_EXPECTED_RED_01.md'
capture = root / 'wallet-broker/target/wal019-sync-expected-red-01'
report = root / 'docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md'
target = root / 'wallet-broker/target'
rows = re.findall(r'\| `(wallet-broker/[^`]+)` \| (\d+) \| `([a-f0-9]{64})`', handoff.read_text())
assert len(rows) == 7 and len({r[0] for r in rows}) == 7
assert not capture.exists() and not capture.is_symlink() and not report.exists()
assert target.is_dir() and target.resolve() == target
fs_type = subprocess.check_output(['stat', '-f', '-c', '%T', str(target)], text=True).strip()
assert fs_type not in ('tmpfs', 'ramfs'), fs_type

def stamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def snapshot():
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    status = subprocess.check_output(['git', 'status', '--short'], text=True)
    items = []
    for path, lines, digest in rows:
        data = (root / path).read_bytes()
        actual = hashlib.sha256(data).hexdigest()
        count = data.count(b'\n')
        items.append(dict(path=path, lines=count, sha256=actual,
                          matches=(actual == digest and count == int(lines))))
    return dict(time_utc=stamp(), head=head, status=status, inputs=items,
                matches=(head == '2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e'
                         and all(item['matches'] for item in items)))

def save(name, value):
    with (capture / name).open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')

before = snapshot()
assert before['matches'], 'Frozen input mismatch; no commands launched'
capture.mkdir()
save('preflight.json', dict(snapshot=before, filesystem_type=fs_type))

def run(name, argv):
    started = stamp()
    log = capture / (name + '.log')
    with log.open('xb') as output:
        process = subprocess.Popen(argv, cwd=root, stdout=output, stderr=subprocess.STDOUT)
        print('Started', name, 'PID', process.pid, flush=True)
        while True:
            try:
                code = process.wait(timeout=30)
                break
            except subprocess.TimeoutExpired:
                print('Still running', name, 'PID', process.pid, flush=True)
    data = log.read_bytes()
    save(name + '.json', dict(argv=argv, pid=process.pid, start_utc=started,
         end_utc=stamp(), exit_code=code, log_sha256=hashlib.sha256(data).hexdigest()))
    print('Completed', name, 'exit', code, flush=True)
    return code

try:
    assert run('00-hermes-version', ['timeout', '30s', 'hermes', '--version']) == 0
    assert run('01-rust-version', ['timeout', '30s', 'rustup', 'run', '1.98.0', 'rustc', '--version']) == 0
    assert run('02-format', ['timeout', '60s', 'rustup', 'run', '1.98.0', 'rustfmt',
        '--check', '--edition', '2024', 'wallet-broker/tests/account_native_ui.rs']) == 0, 'Formatter failure is not expected red'
    run('03-focused-red', ['timeout', '--signal=TERM', '--kill-after=10s', '600s',
        'rustup', 'run', '1.98.0', 'cargo', 'test', '--manifest-path',
        'wallet-broker/Cargo.toml', '--locked', '--offline', '--no-default-features',
        '--features', 'native-ui', '--test', 'account_native_ui', 'wal019_', '--', '--nocapture'])
finally:
    after = snapshot()
    save('postflight.json', after)
    assert after['matches'], 'Frozen inputs changed during capture'
PY
```

## Interpret the actual output and stop

Read retained logs and JSON metadata. Expected outcome: formatter exit 0; the
focused Cargo invocation compiles, selects exactly six `wal019_` tests and exits
101 with six assertion failures on missing list Server/Sync/Balance UI (normally
`exact label must be painted`). The existing production list has no Server/Sync
controls. The sixteen other UI tests should be filtered out, not counted as passes.
Do not infer an exercised post-start assertion when a test failed earlier while
locating a missing widget. Compilation errors, missing tooling, timeout, zero
selected tests, a different failure cause or unexpected green require reviewer
attention; never relabel them as the intended red. Driver completion alone is
not test acceptance.

Write only the named new report with actual provider/model/session, version output,
before/after pins, exact commands and exits, selected/passed/failed/ignored/filtered
counts, each selected test name and actual failure location/cause, and relative
links plus hashes for raw captures. Include anomalies and missing evidence
explicitly. Do not reproduce local absolute paths in publishable prose. Retained
ignored raw logs may include tool-generated paths. Return the report path and stop
for Codex review. No production authorization follows automatically from red.
