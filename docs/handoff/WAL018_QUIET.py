from pathlib import Path
import time,json
expected={str(Path(p).resolve()) for p in ['wallet-broker/target/debug/examples/wal018_diagnostic','wallet-broker/target/debug/examples/wal018_diagnostic-01334394f842ce60']}
def active():
 count=0
 for entry in Path('/proc').iterdir():
  if not entry.name.isdigit():continue
  try:
   target=(entry/'exe').readlink()
   if str(target).removesuffix(' (deleted)') in expected:count+=1
  except (FileNotFoundError,PermissionError,ProcessLookupError):pass
 return count
for attempt in range(30):
 if active()==0:break
 time.sleep(2)
assert active()==0,'owned diagnostic still active; do not clean up'
print(json.dumps({'owned_diagnostic_processes':0}))
