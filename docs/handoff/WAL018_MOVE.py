from pathlib import Path
import os
os.umask(0o077)
base=Path('wallet-broker/target/wal018-diagnostic')
for suffix in ['root','root/network','root/network/account']:
 (base/suffix).mkdir(mode=0o700)
source=base/'viewing-copy.sqlite3'
target=base/'root/network/account/live.sqlite3'
assert source.is_file() and not source.is_symlink() and not target.exists()
source.rename(target)
print('private_clone_prepared=true')
