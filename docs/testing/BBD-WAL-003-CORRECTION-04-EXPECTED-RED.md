# BBD-WAL-003 Correction 04 Expected-Red Evidence

Timestamp: 2026-08-30T16:18:10-0700 (PDT)
Governance baseline: `HEAD == origin/master == 2172b4fbe05acd15fc20885f93d9e692334fbcef`

Corrected test verified unchanged: `test/securityPolicy.node.js`, 1,574 lines,
SHA-256 `1414a32cb114b1467c9d39bbcbf02228aa185857b0ccc304cd4356be9a02507b`.

## Authorized command

`node test/securityPolicy.node.js` — exit status 1. All 58 registered tests ran:
53 passed and exactly these five failed, each because the current package/workflow/
policy production wiring is absent:

1. `wallet contract package command and maintained-source policy are exact and fail closed` — inherited package command remains the old three-suite command.
2. `wallet broker boundary package scripts and syntax checks are exact` — wallet-broker script is absent.
3. `wallet broker and preload paths are required on every routine workflow trigger` — wallet-broker workflow path is absent.
4. `routine CI executes the named wallet broker suite and rejects omission` — CI omits `npm run test:wallet-broker`.
5. `wallet boundary source policy allows only reviewed built-ins and forbids listeners, shell, and generic IPC` — wallet-boundary source policy implementation is absent.

No canary appeared in output. No production path ran or changed, and no out-of-scope
action occurred. The corrected test was not edited.
