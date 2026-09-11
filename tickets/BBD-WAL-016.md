# WAL-016 — Sync survives automatic wallet locking

Status: REVIEW ACCEPTED; verified correction ready for exact integration. Baseline4cae91cb59635213ceecbae075c482ad8949aab2.
User reports every sync returns to locked account list before completion. Code confirms
15-minute SessionManager deadline triggers manager cancellation and UI scene invalidation.
Reviewer corrects the WAL015 coupling; owner need not select implementation details.

Keep the existing authorization deadline and seed wiping unchanged. An explicitly
started viewing-only worker may continue through ordinary idle expiration. Do not
retain spending material, refresh deadlines, start work while locked, or relax native
origin/account/job/network binding. Manual Lock, hide/quit/drop, Back, explicit Cancel,
account removal/replacement, and clock failure still cancel/join and clear private data.

Keep the current native Sync screen visible across idle expiration. Show the lock
state, scanned/target progress and terminal "Sync complete" clearly. Never paint or
return amounts while locked, including the frame that crosses the deadline. Manager
status redacts a returned copy (amounts only) while retaining the actual completed
snapshot for reauthentication; callbacks must not mutate a shared snapshot into a
permanently redacted result. Unlock on the Sync screen uses existing masked input and
native port, reveals the same result/job without starting another connection, and keeps
progress visible. Wrong/empty passphrase keeps balances hidden; Back/hide/quit wipe any
password. Locked users may cancel an existing job, but cannot start/retry until unlocked.
Public progress/status survives, private balance/receiver data does not.

Tests first: existing wallet-broker/tests/account_management.rs and account_native_ui.rs.
Add wal016 regressions for real manager/fake clock expiration with worker still alive,
locked native actions denied, completion/status redaction and unlock revealing samejob;
actual UI pointer/paint progression, locked terminal status, inline unlock and badpassword,
cancel/Back/hide. Update only contradictory WAL015 idle-expiry expectations; keep manual
lock/drop/identity/hide assertions. Prefer existing helpers; if needed a narrow controlled
source factory in zec/test_support.rs may supply an empty valid testnet scan through the
real engine to prove manager terminal status. No fabricated engine counters/results.

Production after RED: accounts.rs, account_ui.rs; narrowly test_support.rs if required.
No session.rs timeout/event change, scanner/transport/vault schema/dependency/policy edits.
Principal Sol gpt-5.6-sol High is retained: prior Grok stopped without usable drop;
this concurrency/private-state correction needs the established senior source actor.
Source actor authors only; no execution/formatters/Git/evidence. Hermes executes/integrates.

RED: rustup run1.98.0 cargo test --manifest-path wallet-broker/Cargo.toml --locked --offline
--no-default-features --features native-ui --test account_management wal016_
and same with --test account_native_ui wal016_. Expected runtime cancellation/scene reset
or missing narrowly proposed test-source seam. GREEN full account_management,
account_native_ui,vault_session; productionClippy lib/bin with native-ui -Dwarnings.
Falsify removal of returned-copy balance redaction or idle-continuation path; require
meaningful runtime failure, restore exact bytes and rerun affected regression.
Rebuild existing staged broker via node scripts/build-wallet-broker.js with trusted cargo
PATH; native window smoke Xvfb and actualdisplay using owned empty profile only.
No user account/profile reads/writes or installer work. Existing npm/policyWIP preserved.
No dependency change: reuse prior audit. Maintain six existing policy failures, pinned
Gitleaks scan with prior five-public-checksum triage only; actual secret/newpolicyfinding
blocks. Exact commands/pins in execution handoff. Keep evidence concise to avoid repeating
unrelated hashmaps. Acceptance completes rebuilt user-facing fix and pushed source/evidence.

Reviewer acceptance: Hermes RED reproduced the scene reset; GREEN passed20manager,
15native-UI and13session tests plus productionClippy. Returned-copy redaction
falsification failed at the actual locked amount assertion (Some(0) versus None), exact
restore passed. Native rebuild and Xvfb/actual-display window checks passed. Six inherited
policy failures unchanged; pinned directory scanner exit1 classified352 exact public
WAL015 checksum matches, no credentials. No user profile/dependency/policyWIP changes.
Exact source/evidence publication is authorized by docs/handoff/WAL016_INTEGRATE.json.
