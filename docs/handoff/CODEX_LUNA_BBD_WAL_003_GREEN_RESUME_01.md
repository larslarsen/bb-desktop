# Codex Luna Handoff — BBD-WAL-003 Local Green Resume 01

You are **Jr Dev — Codex Luna**, using `gpt-5.6-luna`. This is the complete durable
resume prompt together with the original green handoff; ephemeral chat is not
authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Governance ancestor: `e04db82d`

Read and follow `CODEX_LUNA_BBD_WAL_003_GREEN.md` except for the updated source hashes
below. The prior attempt stopped on command one at 10/11 protocol because numeric
`child_pid` was coerced by a regex. Corrections 02–03 added explicit primitive-string
guards across the defect family and closed direct session-hash coercion. No other command
ran and no path was integrated.

Reviewer-accepted uncommitted production paths now are:

- `wallet-broker/protocol.js` — 340 lines — `79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4`
- `wallet-broker/supervisor.js` — 398 lines — `773a4815e9ca89752b28fcdbaaf19dcc347e648e4226314f7da78241f23d5520`
- `wallet-preload.js` — 68 lines — `3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df`
- `social-main.js` — 235 lines — `ef3c12eb00fe5ea990399bc8f4821d5574aa7bc79c353554e42851d8407e8397`
- `package.json` — 36 lines — `2f1e2e6d221baf676dbdf0436d7c595f5976dad765234948da0d632250d8c47e`
- `.github/workflows/social.yml` — 140 lines — `4308b94dec1d0ed9575332a812f0f2b320af89b37d77461b61df1cabc3c324d3`
- `.github/workflows/security.yml` — 52 lines — `dd3edfcd4b40c6d41130f836a66c525480560584aa5dac72cc6a4a65ffe21e82`
- `scripts/security-policy.js` — 1,849 lines — `e9bdb4f927defee883e02eca2fb5a2ad6d263c518b12147e66ca92802c6a5e31`

Verify these are the only dirty paths, `HEAD == origin/master`, and governance ancestor
`e04db82d` is in current history. Rerun the original green handoff from its first targeted
command. All original expected counts, broader commands, stop conditions, evidence path,
allowed documentation updates, exact staging/commit/push instructions, and restrictions
remain unchanged. The evidence must include the stopped first attempt and this successful
resume if green.
