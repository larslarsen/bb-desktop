from pathlib import Path
import hashlib,json
spec=json.loads(Path('docs/handoff/WAL018_DIAG02.json').read_text())
source=Path('wallet-broker/examples/wal018_diagnostic.rs')
assert hashlib.sha256(source.read_bytes()).hexdigest()==spec['pins'][str(source)]
assert hashlib.sha256(Path('wallet-broker/src/zec/test_support.rs').read_bytes()).hexdigest()=='de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791'
for literal in [
 'wallet-broker/target/wal018-diagnostic/root/network/account/live.sqlite3',
 'wallet-broker/target/wal018-diagnostic/root/network/account/live.sqlite3-journal',
 'wallet-broker/target/wal018-diagnostic/root/network/account/live.sqlite3-wal',
 'wallet-broker/target/wal018-diagnostic/root/network/account/live.sqlite3-shm']:
 p=Path(literal)
 assert not p.is_symlink()
 if p.exists():
  assert p.is_file()
  p.unlink()
for literal in ['wallet-broker/target/wal018-diagnostic/root/network/account','wallet-broker/target/wal018-diagnostic/root/network','wallet-broker/target/wal018-diagnostic/root','wallet-broker/target/wal018-diagnostic']:
 Path(literal).rmdir()
source.unlink()
Path('wallet-broker/examples').rmdir()
for literal in [
 'wallet-broker/target/debug/examples/wal018_diagnostic.d',
 'wallet-broker/target/debug/examples/wal018_diagnostic-01334394f842ce60.d',
 'wallet-broker/target/debug/examples/wal018_diagnostic',
 'wallet-broker/target/debug/examples/wal018_diagnostic-01334394f842ce60']:
 p=Path(literal)
 assert p.is_file() and not p.is_symlink()
 p.unlink()
print(json.dumps({'private_clone_removed':True,'temporary_source_removed':True,'production_helper_restored':True}))
