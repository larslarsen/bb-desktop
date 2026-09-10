# WAL-011 async IPC focused validation 01

One driver invocation. Actual results, metadata, complete stage output and restoration identities follow. This is controlled main-handler/preload evidence, not real app startup or release acceptance. No integration is authorized.

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
      "output": "d5096342fb923bded8a3de8565b82c1a83cb1725\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n"
    }
  ],
  "stages": [
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 0,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nBitBook electron security tests passed (23).\n",
      "name": "electron-green",
      "counts": {
        "ok": 23,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/walletPreload.node.js"
      ],
      "exit": 0,
      "output": "ok preload: one frozen bitbookWallet object exposes exactly six frozen own functions\nok preload: each callable API uses only its fixed channel and page supplies no method string\nok preload: arguments and results are structured clones immune to caller mutation\nok preload: invalid callback types register no listener\nok preload: subscription strips event objects, clones values, and unsubscribe is bounded\nok preload: hostile callbacks cannot retain Electron events or widen the bridge\nBitBook wallet preload tests passed (6).\n",
      "name": "preload-green",
      "counts": {
        "ok": 6,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 1,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nnot ok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nAssertionError [ERR_ASSERTION]: wallet:snapshot:get did not return a thenable\n    at <repo>/test/electronSecurity.node.js:833:14\n    at run (<repo>/test/electronSecurity.node.js:1200:13)\nnot ok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nAssertionError [ERR_ASSERTION]: wallet:snapshot:get did not return a thenable\n    at <repo>/test/electronSecurity.node.js:975:14\n    at run (<repo>/test/electronSecurity.node.js:1200:13)\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\n2 electron security test(s) failed\n",
      "name": "settlement-falsification",
      "counts": {
        "ok": 21,
        "not_ok": 2
      }
    },
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 0,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nBitBook electron security tests passed (23).\n",
      "name": "electron-restored-green",
      "counts": {
        "ok": 23,
        "not_ok": 0
      }
    }
  ],
  "success": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "d5096342fb923bded8a3de8565b82c1a83cb1725",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md",
  "session": {
    "id": "20260909_181901_686e38",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4",
    "test/electronSecurity.node.js": "df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md": "fa17e0060ca058560fde00309d6b4c5265c0493ee11c9e327cb150a6ef602435"
  },
  "mutant_sha256": "b67a6ba8187776f675714cb0ea26934d4ecbc809df5df72d3c738ab4bddea4df",
  "restored_sha256": "2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4",
  "final_hashes": {
    "social-main.js": "2449b0b190a9ad079639e4d4aca628470d93749bdf92200cc796a1ed33fa1aa4",
    "test/electronSecurity.node.js": "df0aab1686f872fbc2e77ab106f0003ad1fbe76f7c74bda9c00756f53bc0e4a3",
    "wallet-broker/supervisor.js": "1ac92493cb70cd4ea5e1884f4b96424c7fbc89126308299e7221f2b0ba1a8ff8",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "docs/testing/BBD-WAL-011-ASYNC-IPC-RED-01.md": "fa17e0060ca058560fde00309d6b4c5265c0493ee11c9e327cb150a6ef602435"
  }
}
```
