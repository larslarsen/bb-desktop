from pathlib import Path
import hashlib,json,subprocess
spec=json.loads(Path('docs/handoff/WAL018_DIAG02.json').read_text())
p=Path('wallet-broker/src/zec/test_support.rs')
assert hashlib.sha256(p.read_bytes()).hexdigest()==spec['pins'][str(p)]
original=subprocess.run(['git','show','acf53cf26c41992f198bdb6a4cde5fb04ccffb8b:wallet-broker/src/zec/test_support.rs'],check=True,stdout=subprocess.PIPE).stdout
assert hashlib.sha256(original).hexdigest()=='de157d4a78c4c452e0e19122200997e6676c82099ddb25bfebe84781470f0791'
p.write_bytes(original)
print('temporary_helper_restored=true')
