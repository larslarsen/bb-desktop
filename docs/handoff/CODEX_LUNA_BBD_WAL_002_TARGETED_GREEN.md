# Codex Luna Handoff — BBD-WAL-002 Targeted Green

You are **Jr Dev — Codex Luna**. This is the complete durable handoff; ephemeral chat is
not authoritative.

Repository: `/home/lars/OpenBazaar/bb-desktop`

Read `AGENTS.md`, `TESTING.md`, `docs/engineering/DEVELOPMENT_ROLES.md`,
`tickets/BBD-WAL-002.md`, `docs/handoff/CURRENT_TASK.md`, all WAL-002 red evidence and
production corrections, and the current test/production/wiring source completely.

Reviewer XHigh accepts the source for targeted green at these exact hashes:

- `wallet-contract/canonical.js` `32750959ac41d87e8f598d4c215893c35fd5cc011e05686147273aa34b102761`
- `wallet-contract/framing.js` `805246cad2cc500bdd819e672e816514cc809f8bb44ab86260dab3e9ed682a0d`
- `wallet-contract/model.js` `08c6528cc29271bb4b939c2de890cb53558f94c2c32d1b74e1a5e91735069d53`
- `wallet-contract/state-machine.js` `ea070804111d28d336de6fc5371c2837838669e8a989608551067a4a944927ac`
- `wallet-contract/fakes.js` `3f3ad73a9b051831406ae37dd8db7325340c4763000abf09d03eb885c62c6b74`
- `wallet-contract/index.js` `5f97043133522cd2b27f5a45b76d7526c4176704736e0da0c5331ada2f065edc`
- `package.json` `dd991b9aea4c98c5dc668631cb5dba5b950daa439de28e005ae4a57b1ae6d35a`
- `.github/workflows/social.yml` `53652bae4f0fd2e2d4f871d6b9c94305ea7df4c7474cf30857fe8b3780bf7fa5`
- `.github/workflows/security.yml` `515f894a11f94aaacbae86615c638fcdfdf8794c26cc46e1fb3bc957fa49bb7a`
- `scripts/security-policy.js` `9e6d6a14251df8740378e5ccf62e611cc26e5f22a04c878276c07fa89ff0751b`
- fixture `08905d2b082fa6370211ebd494130d62e3696db0f8314636f3685ba5887f929f`
- wallet test `43830b1caec19904d23b400974c77c1edbebe32b4927b2f31ee4279611a46dbf`
- Electron test `3c18292790642ec3430b3ca1717657603da1a4e41c46b6c7812184dfb978939a`
- policy test `79b9286be5a679ba40baa5171417ab13bd19f0a2d8a56029398db8b083713204`
- unchanged lockfile `7d347693664abe2639dfd1cd04f5f7e8757adb2dd71098b0a23b49aedade5413`

Verify `HEAD == origin/master`, record the full authorization baseline, status, all
hashes, and `git diff --check`. Then run exactly these foreground commands in order:

```text
node test/walletContract.node.js
node test/electronSecurity.node.js
node test/securityPolicy.node.js
```

All must exit 0. Expected totals are 48 wallet, 14 Electron-security, and 54
security-policy tests. Stop immediately on any failure, canary exposure, resource leak,
hash drift, or unexpected worktree change.

Do not create evidence yet. Do not edit, stage, commit, or push anything. Do not run npm,
build, policy main, audit, falsifications, packaging, SBOM, scanners, Electron, network,
wallet, daemon, hardware, or devices. No background execution, root, `sudo`, `/tmp`,
deletion, cleanup, `rm`, globs, unresolved paths, or environment-variable targets.

Stop after the three commands and report exact exits/pass totals, protected hashes,
baseline/final status, canary absence, and confirmation that nothing changed. Reviewer
XHigh must accept targeted green before broader acceptance/falsification/Git.
