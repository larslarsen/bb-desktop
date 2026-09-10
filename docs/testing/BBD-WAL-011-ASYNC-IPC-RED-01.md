# WAL-011 async IPC expected-red evidence 01

One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; later rows of each failing delayed group remain unexecuted. No integration or validation replay is authorized.

```json
{
  "commands": [
    {
      "argv": [
        "hermes",
        "--version"
      ],
      "exit": 0,
      "output": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0\n"
    },
    {
      "argv": [
        "git",
        "rev-parse",
        "HEAD"
      ],
      "exit": 0,
      "output": "34c297b0ff35feff2f0d4350bbd1addd82faa384\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n"
    },
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 1,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nnot ok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nAssertionError [ERR_ASSERTION]: wallet:snapshot:get did not return a thenable\n    at <repo>/test/electronSecurity.node.js:833:14\n    at run (<repo>/test/electronSecurity.node.js:1200:13)\nnot ok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nAssertionError [ERR_ASSERTION]: wallet:snapshot:get did not return a thenable\n    at <repo>/test/electronSecurity.node.js:975:14\n    at run (<repo>/test/electronSecurity.node.js:1200:13)\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\n2 electron security test(s) failed\n"
    }
  ],
  "expected_red": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "34c297b0ff35feff2f0d4350bbd1addd82faa384",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md",
  "session": {
    "id": "20260909_181137_0cfb89",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "test/electronSecurity.node.js": "df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3",
    "social-main.js": "b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4"
  },
  "final_hashes": {
    "test/electronSecurity.node.js": "df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3",
    "social-main.js": "b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4"
  },
  "counts": {
    "ok": 21,
    "not_ok": 2
  }
}
```
