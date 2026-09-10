# WAL-011 Electron quit focused validation 01

One driver invocation. Actual results, metadata, complete stage output and restoration identities follow. This is controlled normal-quit/main-handler/preload evidence, not real app startup or release acceptance. No integration is authorized.

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
      "output": "743552a0c163ac8fcb8da9e240ccb6b5c96d0a21\n"
    },
    {
      "argv": [
        "git",
        "status",
        "--short"
      ],
      "exit": 0,
      "output": " M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md\n"
    }
  ],
  "stages": [
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 0,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nok after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit\nok before ready, fulfilled no-child shutdown allows one resumed quit without windows\nok shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills\nok rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE\nok synchronous shutdown throw is contained and keeps quit blocked with one fixed error box\nok rejected shutdown plus throwing error box stays contained without resumed quit\nok window-all-closed uses the host platform branch and the same before-quit gate\nBitBook electron security tests passed (30).\n",
      "name": "electron-green",
      "counts": {
        "ok": 30,
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
        "-e",
        "const assert = require('assert'); const name = 'after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit'; const selected = require('./test/electronSecurity.node.js').tests.filter(t => t.name === name); assert.strictEqual(selected.length, 1); selected[0].fn().then(() => { console.log(JSON.stringify({unexpectedPass:true,name})); process.exitCode=2; }, error => { console.log(JSON.stringify({name,code:error.code,actual:error.actual,expected:error.expected,message:error.message,stack:error.stack})); process.exitCode=1; });"
      ],
      "exit": 1,
      "output": "{\"name\":\"after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit\",\"code\":\"ERR_ASSERTION\",\"actual\":1,\"expected\":0,\"message\":\"Expected values to be strictly equal:\\n\\n1 !== 0\\n\",\"stack\":\"AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:\\n\\n1 !== 0\\n\\n    at Object.fn (<repo>/test/electronSecurity.node.js:1419:12)\\n    at [eval]:1:295\\n    at runScriptInThisContext (node:internal/vm:209:10)\\n    at node:internal/process/execution:446:12\\n    at [eval]-wrapper:6:24\\n    at runScriptInContext (node:internal/process/execution:444:60)\\n    at evalFunction (node:internal/process/execution:279:30)\\n    at evalTypeScript (node:internal/process/execution:291:3)\\n    at node:internal/main/eval_string:74:3\"}\n",
      "name": "early-quit-falsification",
      "counts": {
        "ok": 0,
        "not_ok": 0
      }
    },
    {
      "argv": [
        "node",
        "test/electronSecurity.node.js"
      ],
      "exit": 0,
      "output": "ok app.enableSandbox is invoked before the window is created\nok BrowserWindow explicitly sets the fail-closed webPreferences\nok only the repository social/index.html is loaded\nok package.json keeps the maintained social-main.js entry point\nok renderer navigation is denied\nok renderer redirects are denied\nok webview attachment is denied\nok new-window creation is denied and shell.openExternal is unreachable\nok permission request handler denies every permission\nok permission check handler denies every permission\nok only the explicit local wallet preload and exact ipcMain bridge are introduced\nok CSP keeps self-only script/style and denies objects, frames, base, and forms\nok maintained source has no HTML injection, eval, or javascript: sinks\nok wallet preload retains exactly six frozen methods and no Electron confirmation action\nok wallet IPC registers only the exact renderer channel allowlist\nok wallet IPC rejects non-main frames, non-local origins, malformed shapes, and oversize input\nok wallet IPC valid calls map once to fixed supervisor methods with cloned parameters\nok wallet IPC delayed supervisor replies stay pending then clone fulfilled values for every channel\nok wallet IPC delayed supervisor rejections propagate original UNAVAILABLE and TIMEOUT errors\nok wallet IPC synchronous dispatch failure throws immediately\nok wallet snapshot subscription targets only the maintained main frame with sanitized cloned data\nok wallet Electron boundary exposes no confirmation, unlock, backup, sign, or broadcast surface\nok wallet reference contract is maintained source and retains an offline inert boundary\nok after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit\nok before ready, fulfilled no-child shutdown allows one resumed quit without windows\nok shutdown-emitted nested before-quit stays prevented until the original shutdown fulfills\nok rejected shutdown keeps quit blocked and shows one fixed error box for TIMEOUT and UNAVAILABLE\nok synchronous shutdown throw is contained and keeps quit blocked with one fixed error box\nok rejected shutdown plus throwing error box stays contained without resumed quit\nok window-all-closed uses the host platform branch and the same before-quit gate\nBitBook electron security tests passed (30).\n",
      "name": "electron-restored-green",
      "counts": {
        "ok": 30,
        "not_ok": 0
      }
    }
  ],
  "success": true,
  "version": "Hermes Agent v0.18.2 (2026.7.7.2) \u00b7 upstream 8e85b276 \u00b7 local 10b6d1a9 (+1 carried commit)\nInstall directory: <home>/.hermes/hermes-agent\nInstall method: git\nPython: 3.11.15\nOpenAI SDK: 2.24.0",
  "head": "743552a0c163ac8fcb8da9e240ccb6b5c96d0a21",
  "initial_status": "M package-lock.json\n M package.json\n M scripts/security-policy.js\n M social-main.js\n M test/electronSecurity.node.js\n M test/securityPolicy.node.js\n?? docs/testing/BBD-WAL-009-POLICY-RED-AND-NATIVE-LINT-01.md\n?? docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md\n?? docs/testing/BBD-WAL-011-EXECUTABLE-GREEN-01.md",
  "session": {
    "id": "20260909_212608_cf8d10",
    "model": "poolside/laguna-s-2.1:free",
    "provider": "nous"
  },
  "input_hashes": {
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md": "229c1a97b501dcf0a1f60865245e76c644520776500b06f3c3ff5d0ddedf167b"
  },
  "mutant_sha256": "12896f8cd1e2fa3ed089028c845633adcfe311b14d31370334577b11071256f6",
  "restored_sha256": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
  "falsification": {
    "name": "after ready, initial before-quit is prevented until deferred shutdown fulfills and resumes quit",
    "code": "ERR_ASSERTION",
    "actual": 1,
    "expected": 0,
    "message": "Expected values to be strictly equal:\n\n1 !== 0\n",
    "stack": "AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:\n\n1 !== 0\n\n    at Object.fn (<repo>/test/electronSecurity.node.js:1419:12)\n    at [eval]:1:295\n    at runScriptInThisContext (node:internal/vm:209:10)\n    at node:internal/process/execution:446:12\n    at [eval]-wrapper:6:24\n    at runScriptInContext (node:internal/process/execution:444:60)\n    at evalFunction (node:internal/process/execution:279:30)\n    at evalTypeScript (node:internal/process/execution:291:3)\n    at node:internal/main/eval_string:74:3"
  },
  "final_hashes": {
    "social-main.js": "c7687b52ee45b17c3063cc2b39071be3d4e613a9d1e86558f2a7bbc1567db0a3",
    "test/electronSecurity.node.js": "7f759f810762ccdfa5e29d69dc3bf2fa1b1502d2c416ca2b324442b2544ad18c",
    "wallet-broker/supervisor.js": "c1410bfc5cd4c205ad014a42154c9d9d9714794b134a5acab7ed499934cad51a",
    "wallet-broker/protocol.js": "79b0ac8bdd1dc6f4d54793dd1137ae72172688412eefbbe853f1cc421be630f4",
    "test/walletPreload.node.js": "60b151344a01776e1d7f38238f69534426883503f9d627466bac7acbf4dc4f9e",
    "wallet-preload.js": "3e6a18acf88dd5be4e6a88f326d6ace7a8071066480d9a70a2e8f89df035a1df",
    "wallet-pay/model.js": "acf07238366f3e28253be8c9208fbb27e13e9c2687d374d1a6ae87e0d173fa5e",
    "test/fixtures/wallet-pay/snapshots-v1.json": "bd51c24e003eb63f84fd99ca9573e765f28c3b7bc3ccbd924b14ba34afe05252",
    "docs/testing/BBD-WAL-011-ELECTRON-QUIT-RED-01.md": "229c1a97b501dcf0a1f60865245e76c644520776500b06f3c3ff5d0ddedf167b"
  }
}
```
