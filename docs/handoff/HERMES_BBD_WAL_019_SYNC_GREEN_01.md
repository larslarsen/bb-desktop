# Hermes — WAL-019 green, falsification and local wallet refresh 01

Actor: Hermes, free Nous Portal model, manually relayed by owner. Read AGENTS.md,
TESTING.md, docs/engineering/HERMES_JR_DEV_ROUTING.md, CURRENT_TASK.md,
tickets/BBD-WAL-019.md, production source review 01 and expected-red review 01.
Grok's source is accepted. All previous actor handoffs are closed.

## Baseline and scope

Require HEAD `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e` and these ten input pins.
Preserve all unrelated dirty source, governance, policy work and prior captures.

| Path | Lines | SHA-256 |
| --- | ---: | --- |
| `wallet-broker/src/account_ui.rs` | 1332 | `09d5c5cbcb7867a06574ebb99426f4331e4d789bd4ec29c1c47122f3566c1f6a` |
| `wallet-broker/tests/account_native_ui.rs` | 2654 | `e7ac08d26fad3eb0302c49b8983b62cb28a6aab5bad657b142c2e84d59a11446` |
| `wallet-broker/Cargo.toml` | 126 | `435907f00d7819bdf7dbe9ff9e367522f7f6eb8b6019dbde9e58212bbaea041b` |
| `wallet-broker/Cargo.lock` | 5771 | `a5d53b729433a06f60edcef51e608629c7365326d06fef2bbac8f2b23f048c2a` |
| `wallet-broker/src/accounts.rs` | 1006 | `8941884f837453bf71357f53341fae2c1334b8d10d017680053f9f8a44fb415d` |
| `wallet-broker/src/zec/live.rs` | 1238 | `17be2ab62cdda883940d301d9701ed14c507ff351c4356d46a523a045c3f149f` |
| `wallet-broker/src/zec/live_transport.rs` | 214 | `a9f3b50e2694746ec6f443d7a35ded4c035baab4b6003946ac29d1d88e09b90e` |
| `scripts/build-wallet-broker.js` | 152 | `4427ef3e713ec77c45daefe8d37b457ca8dc2c8529b30214dbf65628f43a1445` |
| `wallet-broker/launch-config.js` | 140 | `68adf92e2d543e5d51353477b1944c35d4189c5590ac581d3289f1bd72898af8` |
| `social-main.js` | 299 | `d959e6adffc2579fa3f2f447f60a165f2b6bcdc279afde2f6ddaa91bc9d9aa4a` |

Authorized writes:

- New capture directory `wallet-broker/target/wal019-sync-green-01` and existing
  disk-backed Cargo target/cache outputs.
- The exact temporary one-block source mutation below, followed by exact-byte
  restoration even if its test fails unexpectedly. No other source/test edit.
- Development wallet artifacts produced by the existing build script under
  `wallet-broker/target/app-resources`, including binary and manifest. Do not launch
  or restart Electron, the wallet process or the daemon.
- New report `docs/testing/BBD-WAL-019-SYNC-GREEN-01.md` and correction of
  `docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md` as specified below.

New capture/report must not preexist. No dependencies/network fetches, user wallet
data, broader wallet/proving suites, policy-source changes, other actors, Git,
commits, pushes, release packaging, funds or daemon rebuild. Preserve inherited
release blockers; this command sequence does not claim a clean release.

## Exact single-shot driver

Run once from bb-desktop. It captures raw output and actual exits. Do not rewrite
the driver, split out commands, retry after failure or restart on a yielded tool
wait. Keep polling the same session. Each long command is bounded by GNU timeout;
never manually kill the runner or bypass restoration. If interrupted, preserve the
capture and report the source state before any further action.

```bash
python3 - <<'PY'
import datetime, hashlib, json, pathlib, re, subprocess

root = pathlib.Path.cwd()
handoff = root / 'docs/handoff/HERMES_BBD_WAL_019_SYNC_GREEN_01.md'
capture = root / 'wallet-broker/target/wal019-sync-green-01'
report = root / 'docs/testing/BBD-WAL-019-SYNC-GREEN-01.md'
target = root / 'wallet-broker/target'
source = root / 'wallet-broker/src/account_ui.rs'
rows = re.findall(r'\| `([^`]+)` \| (\d+) \| `([a-f0-9]{64})`', handoff.read_text())
assert len(rows) == 10 and len({row[0] for row in rows}) == 10
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
assert before['matches'], 'Baseline mismatch; no gates launched'
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
    return code, data.decode(errors='replace')

def expect_tests(name, seconds, extra, passed, failed, filtered):
    argv = ['timeout', '--signal=TERM', '--kill-after=10s', str(seconds) + 's',
        'rustup', 'run', '1.98.0', 'cargo', 'test', '--manifest-path',
        'wallet-broker/Cargo.toml', '--locked', '--offline', '--no-default-features',
        '--features', 'native-ui', '--test', 'account_native_ui'] + extra
    code, output = run(name, argv)
    counts = re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out', output)
    assert code == (101 if failed else 0), (name, code)
    assert counts == [(str(passed), str(failed), '0', '0', str(filtered))], (name, counts)
    return output

original = source.read_bytes()
try:
    assert run('00-hermes-version', ['timeout', '30s', 'hermes', '--version'])[0] == 0
    assert run('01-rust-version', ['timeout', '30s', 'rustup', 'run', '1.98.0', 'rustc', '--version'])[0] == 0
    assert run('02-format', ['timeout', '60s', 'rustup', 'run', '1.98.0', 'rustfmt',
        '--check', '--edition', '2024', 'wallet-broker/src/account_ui.rs',
        'wallet-broker/tests/account_native_ui.rs'])[0] == 0
    expect_tests('03-focused-green', 600, ['wal019_', '--', '--nocapture'], 6, 0, 16)
    assert snapshot()['matches'], 'Inputs changed before falsification'
    anchor = b'''        if sync_clicked && let Some((account_id, _)) = selected.as_ref() {
            self.begin_sync(account_id.clone());
            return;
        }'''
    replacement = anchor.replace(b'self.begin_sync(', b'self.open_sync_scene(')
    assert original.count(anchor) == 1, 'Falsification anchor mismatch'
    (capture / 'source-before-falsification.rs').write_bytes(original)
    mutated = original.replace(anchor, replacement, 1)
    save('04-mutation.json', dict(original_sha256=hashlib.sha256(original).hexdigest(),
         mutated_sha256=hashlib.sha256(mutated).hexdigest(), method='List Sync navigates without start_sync dispatch'))
    try:
        source.write_bytes(mutated)
        output = expect_tests('05-falsification-red', 600,
            ['wal019_list_server_disclosure_and_one_click_starts_at_both_sizes', '--', '--exact', '--nocapture'],
            0, 1, 21)
        assert 'left: []' in output and 'assertion `left == right` failed' in output, 'Wrong falsification failure'
    finally:
        source.write_bytes(original)
        assert source.read_bytes() == original
        save('06-restoration.json', snapshot())
    assert snapshot()['matches'], 'Restoration/input mismatch'
    expect_tests('07-restored-green', 600, ['wal019_', '--', '--nocapture'], 6, 0, 16)
    expect_tests('08-native-ui', 900, ['--', '--nocapture'], 22, 0, 0)
    assert snapshot()['matches'], 'Inputs changed before local build'
    assert run('09-local-build', ['timeout', '--signal=TERM', '--kill-after=10s',
        '600s', 'node', 'scripts/build-wallet-broker.js'])[0] == 0
    binary = target / 'app-resources/wallet-broker/bitbook-wallet-broker'
    manifest = target / 'app-resources/wallet-broker/manifest.json'
    debug = target / 'debug/bitbook-wallet-broker'
    for path in [binary, manifest, debug]:
        assert path.is_file() and not path.is_symlink(), str(path)
    info = json.loads(manifest.read_text())
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    assert info == dict(v=1, platform='linux', arch='x64', sha256=digest), info
    assert hashlib.sha256(debug.read_bytes()).hexdigest() == digest
    save('10-local-artifacts.json', dict(time_utc=stamp(), binary=str(binary.relative_to(root)),
        binary_sha256=digest, debug_sha256=digest, manifest=info,
        manifest_sha256=hashlib.sha256(manifest.read_bytes()).hexdigest(),
        process_restarted=False))
finally:
    after = snapshot()
    save('postflight.json', after)
    assert after['matches'], 'Frozen input drift; report and stop'
PY
```

## Evidence and stop conditions

Every gate must have its own retained actual exit/output. A failed gate stops all
later work except exact restoration and postflight. Compilation failures, timeouts
or count mismatches never count as the intended falsification. The temporary mutant
must fail because its start-call list is empty, not because controls disappeared.
Final green must run on the restored production hash. Report any failure honestly;
do not edit source to repair it or launch a new run.

In the new green report record actual Hermes version/provider/model/session (mark
unrecoverable session identifiers unavailable), all ten before/after pins, exact
commands/exits/counts, mutation and restoration hashes, observed falsification
cause, artifact hashes and relative clickable raw-capture links. Explain whether
the local wallet was rebuilt/staged and explicitly distinguish that from a process
restart. Do not claim the daemon binary was updated or payments completed.

Correct the existing expected-red report using only its retained raw captures and
docs/testing/BBD-WAL-019-EXPECTED-RED-REVIEW-01.md: actual upstream `110baa09`, Rust
`1.98.0 (88d9e12ae 2026-08-18)`, missing account-list Server before any sync start,
and no premature production authorization. Add raw hashes/relative links; replace
the absolute PATH detail with a general description. Record the original red
session identifier only if recoverable from your execution records, otherwise
explicitly mark it unavailable. Do not replace it with the current green session.
Preserve original command results, source pins and raw captures; this is a prose
correction, not another red run. Return report paths and stop for reviewer acceptance.
Git/publication and any further work require a subsequent exact-path handoff.
