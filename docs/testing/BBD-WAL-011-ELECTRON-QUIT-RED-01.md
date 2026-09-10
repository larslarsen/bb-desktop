# WAL-011 Electron quit expected-red evidence 01

One driver invocation; actual results below. expected_red=true means the reviewed failure was observed, not a green suite. Production was not changed; the seven new lifecycle bodies and second rejection row remain unexecuted on the absent-handler baseline. No integration or validation replay is authorized.

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
      "output": "384ba2398337540d4085e6a2a2a0b444c2d9f365\n"
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
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nnot ok after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1415:5\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\nnot ok before ready, fulfilled no-child shutdown allows one resumed quit without windows\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1499:5\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\nnot ok shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1564:5\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\nnot ok rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1648:7\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\nnot ok synchronous shutdown throw is contained and keeps quit blocked with one fixed error box\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1734:5\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\nnot ok rejected shutdown plus throwing error box stays contained without resumed quit\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1839:5\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\nnot ok window-all-closed uses the host platform branch and the same before-quit gate\nAssertionError [ERR_ASSERTION]: before-quit handler is missing\n    at requireAppHandlers (<repo>/test/electronSecurity.node.js:520:10)\n    at <repo>/test/electronSecurity.node.js:1925:5\n    at run (<repo>/test/electronSecurity.node.js:2003:13)\n7 electron security test(s) failed\n"
    }
  ],
  "expected_red": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "384ba2398337540d4085e6a2a2a0b444c2d9f365",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md",
  "session": {
    "id": "20260909_205604_2ac4dd",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "social-main.js": "2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e"
  },
  "final_hashes": {
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "social-main.js": "2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e"
  },
  "counts": {
    "ok": 23,
    "not_ok": 7
  }
}
```
