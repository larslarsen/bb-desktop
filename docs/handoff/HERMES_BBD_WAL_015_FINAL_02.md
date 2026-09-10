# WAL-015 exact public-checksum triage
Execute ONLY `python3 docs/handoff/HERMES_BBD_WAL_015_FINAL_02.py` once in bb-desktop.
Readexactdriver, terminal background=true notify_on_complete=true, process.wait<=60
until EXIT. No source/test/Git/dependency/userdata changes. Revert ONLY previously
recorded equivalent serialization changes to their exact original file hashes.
The scanner still recognized the escaped filename; do not weaken its rules/config.
Reviewer authorizes actual-value triage: private0600localJSON report, no verboseoutput,
never print capturedvalues. Accept ONLY generic-api-key findings whose capturedvalue
exactlyequals freshlycomputed SHA256 of the public unchanged hygiene test file AND
path lies within exactWAL015 generatedrecord prefixes. Any other finding failsclosed.
After proving valuespublic, redact capturedSecret/Match fields in private report.
Record rule/path/line/boolean only. No ignore/config/allowlist/source weakening.
This is a reviewer classification of a non-secret checksum, not a clean scanner exit.
Source/binary/diffcheck preserve prior111Rust/61JSgroups and6inheritedpolicyfailures.
Reviewer owns exception rationale and removal condition in WAL015 review below.
