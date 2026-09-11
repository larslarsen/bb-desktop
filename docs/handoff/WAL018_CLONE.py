from pathlib import Path
import os,sqlite3,json
os.umask(0o077)
root=Path.home()/'.config/BitBook/wallet-broker/zec-testnet'
sources=[]
for account in root.iterdir():
    if account.is_symlink() or not account.is_dir():continue
    candidate=account/'live.sqlite3'
    if candidate.is_file() and not candidate.is_symlink():sources.append(candidate)
assert len(sources)==1,'ambiguous viewing cache'
directory=Path('wallet-broker/target/wal018-diagnostic')
directory.mkdir(mode=0o700)
destination=directory/'viewing-copy.sqlite3'
fd=os.open(destination,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600);os.close(fd)
with sqlite3.connect(sources[0].as_uri()+'?mode=ro',uri=True,timeout=2) as source:
    with sqlite3.connect(destination) as target:
        source.backup(target,pages=256,sleep=0.05)
        height=target.execute('SELECT committed_height FROM ext_bitbook_live_state WHERE singleton=1').fetchone()
        assert height==(4308219,),'checkpoint changed; diagnostic range no longer applicable'
print(json.dumps({'private_viewing_clone_created':True,'committed_height':4308219,'source_opened_readonly':True}))
