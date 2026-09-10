# WAL-015 redacted scanner diagnosis
Execute ONLY `python3 docs/handoff/HERMES_BBD_WAL_015_SECRETS_DIAGNOSTIC_01.py` once
in bb-desktop, readexactdriver, terminal background=true notify_on_complete=true,
process.wait<=60 until EXIT. No source edits/dependencies/Git/userdata/cleanup.
POLICYGREEN01 exact33newchecks and6inheritedratchet passed; scannerreported59findings
without locations. Run pinnedscanner only with redact100 JSONreport, record ONLY
rule/path/line/fingerprint/description for reviewer triage. Never outputsecrets/matches.
No suppression or acceptance yet. Exactraw/evidence/report paths in driver.
