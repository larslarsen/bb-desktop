# BBD-WAL-019 publication correction 01

Date: 2026-09-15. Actor: Hermes, free Nous Portal model (meituan/longcat-2.0:free),
manually relayed by owner. Reviewer: Codex.

## Model and session

Hermes Agent v0.18.2 (2026.7.7.2) · upstream 110baa09 · local 10b6d1a9 (+1 carried commit)
Provider: nous · Model: meituan/longcat-2.0:free
Session identifier: not recoverable from Hermes version output.

## Baseline

Local master and live origin master: `f2d9c9a513b3285531352df84ada818fdac0faf6`
Parent: `2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e`
Branch: master. Staging index empty at start.
Origin: `larslarsen/bb-desktop` on GitHub.

## Source and artifact verification

All ten green-handoff pins match working bytes. Both wallet binaries are 552,768,544
bytes with SHA-256 `6f0b2ce78a7d871bde75d1fa8f330bd27dc7a00f598853dcc60944b92d15f985`.
The 115-byte manifest is `dfa323db0a754e094456fbaf1651e46c2a8f9a77281055b0512bd97782109bcd`.

## Record corrections

The following existing reports were corrected from retained raw captures and the
accepted green review — no tests rerun:

1. `docs/testing/BBD-WAL-019-SYNC-GREEN-01.md`: replaced all placeholder digests with
   actual SHA-256 values from raw files; added full argv arrays (including timeout)
   for every command; distinguished restoration snapshot from an executed command; made
   raw-capture links clickable relative paths; preserved correct runtime counts,
   binary sizes/hashes and process-refresh scope; stated account_native_ui target
   rather than entire native UI/Rust suite; kept session identity unavailable.
2. `docs/testing/BBD-WAL-019-SYNC-EXPECTED-RED-01.md`: replaced raw-capture list with
   clickable relative links and their computed digests; corrected the contradictory
   premature production authorization in Outcome and attributed it solely to the later
   expected-red review; preserved all actual red result values and captures.
3. `docs/testing/BBD-WAL-019-PUBLICATION-01.md`: distinguished original preflight
   baseline from published feature HEAD/remote ref; labeled historical scan exits and
   second-scan coverage unverified from retained evidence; described exactly the single
   retained log and absent metadata; did not claim a version command ran without proof;
   added actual feature commit `f2d9c9a513b3285531352df84ada818fdac0faf6`, remote ref
   and scoped coverage caveat.

## Scanner identity

`target/security-tools/gitleaks-v8.30.1/gitleaks`: size 21,958,840 bytes.
SHA-256: `88f91962aa2f93ac6ab281d553b9e125f5197bbbce38f9f2437f7299c32e5509`
Version: gitleaks v8.30.1 (from embedded module metadata). Version command not separately
captured in retained evidence.

## Published-commit scan

Command:
```
target/security-tools/gitleaks-v8.30.1/gitleaks git --log-opts=2b5ad193f01af31f2a2bdcc7b0d1080dd5a2f68e..f2d9c9a513b3285531352df84ada818fdac0faf6 --redact=100 --ignore-gitleaks-allow --no-banner .
```
Exit: 0. One commit scanned. No leaks found. No unexpected scope.

This is a new scan of the published feature diff, not a retrospective proof of the
historical staged scan.

## Corrective staged scan (first)

Command:
```
target/security-tools/gitleaks-v8.30.1/gitleaks git --pre-commit --staged --redact=100 --ignore-gitleaks-allow --no-banner .
```
Exit: 0. No leaks found. Capture: `wallet-broker/target/wal019-publication-correction-01/scan1.log`.

Staged tree: `fda1b14125b657fba36761fb1bc6ce4293b83d37`

## Corrective staged scan (second, final bytes)

After adding the correction report and restaging, the same scan ran again on final bytes.
Exit: 0. No leaks found. Final staged tree: `075c64c309b88ede9c60a4c1c7918d2da5862cfb`.
Capture: `wallet-broker/target/wal019-publication-correction-01/scan2.log`.

## Commit and push

Commit message: "Correct WAL019 publication evidence"
Commit ID: captured post-push.
Push destination: `origin HEAD:master`.
No amend. No force push.

Remaining dirty paths: all unrelated WAL-009 source, governance, policy work and
prior captures preserved.

Inherited release blockers and incomplete end-to-end payments remain unchanged.
Report returned to owner for Codex review.
